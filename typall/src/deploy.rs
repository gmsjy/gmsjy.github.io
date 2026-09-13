//! 部署：`copy`（本地复制）、`git`（推送静态站点到分支）、
//! `netlify` / `vercel`（官方 API 上传部署）。

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::build;
use crate::config::Config;

pub fn deploy(
    root: &Path,
    config: &Config,
    target_override: Option<&str>,
    dry_run: bool,
) -> anyhow::Result<()> {
    let strategy = target_override
        .map(String::from)
        .or_else(|| config.deploy.strategy.clone())
        .unwrap_or_else(|| "git".into());

    // 部署前钩子（规格书 §7.4，dry_run 不执行）
    if !dry_run && !config.deploy.pre_deploy.is_empty() {
        run_hook("pre_deploy", &config.deploy.pre_deploy)?;
    }

    let result = match strategy.as_str() {
        "git" => deploy_git(root, config, dry_run),
        "copy" => deploy_copy(root, config, dry_run),
        "netlify" => deploy_netlify(root, config, dry_run),
        "vercel" => deploy_vercel(root, config, dry_run),
        other => anyhow::bail!("未知部署策略: {other}（支持 git / copy / netlify / vercel）"),
    };

    // 部署后钩子（仅部署成功且非 dry_run 时执行）
    if result.is_ok() && !dry_run && !config.deploy.post_deploy.is_empty() {
        run_hook("post_deploy", &config.deploy.post_deploy)?;
    }

    result
}

/// 解析 API token：优先配置值，其次环境变量（token 直接写配置文件不安全）。
fn auth_token(config_val: &str, env_name: &str) -> anyhow::Result<String> {
    if !config_val.trim().is_empty() {
        return Ok(config_val.trim().to_string());
    }
    if let Ok(v) = std::env::var(env_name)
        && !v.trim().is_empty()
    {
        return Ok(v.trim().to_string());
    }
    anyhow::bail!(
        "未配置 API token：请填写对应配置字段或设置环境变量 {env_name}"
    )
}

/// 脱敏显示 token（仅 dry-run 打印用）。按 char 切片，多字节字符不会 panic。
fn mask_token(t: &str) -> String {
    let chars: Vec<char> = t.chars().collect();
    if chars.len() <= 8 {
        "******".to_string()
    } else {
        let head: String = chars[..4].iter().collect();
        let tail: String = chars[chars.len() - 4..].iter().collect();
        format!("{head}…{tail}")
    }
}

/// 递归收集目录下所有文件，返回（zip 内相对路径，磁盘路径）。
fn collect_files(dir: &Path) -> anyhow::Result<Vec<(String, PathBuf)>> {
    let mut out = Vec::new();
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(dir)?
            .to_string_lossy()
            .replace('\\', "/");
        out.push((rel, entry.path().to_path_buf()));
    }
    out.sort();
    Ok(out)
}

/// 将目录打包为 zip（deflate），返回（字节，文件数）。
fn zip_directory(dir: &Path) -> anyhow::Result<(Vec<u8>, usize)> {
    use zip::write::SimpleFileOptions;
    use zip::CompressionMethod;

    let files = collect_files(dir)?;
    let mut buf = Vec::new();
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
    let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    for (rel, path) in &files {
        writer.start_file(rel, opts)?;
        let data = std::fs::read(path)?;
        writer.write_all(&data)?;
    }
    writer.finish()?;
    Ok((buf, files.len()))
}

/// Netlify 部署：将 public 打包为 zip，经官方 Deploy API 上传（规格书 §7.2）。
fn deploy_netlify(root: &Path, config: &Config, dry_run: bool) -> anyhow::Result<()> {
    let cfg = &config.deploy.netlify;
    if cfg.site_id.trim().is_empty() {
        anyhow::bail!("未配置 deploy.netlify.site_id（Netlify 站点 API ID）");
    }
    let token = auth_token(&cfg.auth_token, "NETLIFY_AUTH_TOKEN")?;
    let url = format!(
        "https://api.netlify.com/api/v1/sites/{}/deploys",
        cfg.site_id.trim()
    );

    if dry_run {
        println!("[dry-run] Netlify 部署");
        println!("  API: POST {url}");
        println!("  Auth: Bearer {}（{} 字符）", mask_token(&token), token.len());
        println!("  内容: public/ 打包为 application/zip 上传");
        return Ok(());
    }

    ensure_built(root, config)?;
    let public = config.output_dir(root);
    println!("📦 打包 {} …", public.display());
    let (zip_bytes, n) = zip_directory(&public)?;
    println!("  zip {} 字节 / {} 个文件", zip_bytes.len(), n);

    let resp = ureq::post(&url)
        .set("Authorization", &format!("Bearer {token}"))
        .set("Content-Type", "application/zip")
        .send_bytes(&zip_bytes)
        .map_err(|e| anyhow::anyhow!("Netlify API 请求失败: {e}"))?;
    let status = resp.status();
    let body = resp.into_string().unwrap_or_default();
    if !(200..300).contains(&status) {
        anyhow::bail!("Netlify API 返回 {status}: {body}");
    }
    let v: serde_json::Value = serde_json::from_str(&body)?;
    let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("?");
    let state = v.get("state").and_then(|x| x.as_str()).unwrap_or("?");
    let site_url = v
        .get("ssl_url")
        .and_then(|x| x.as_str())
        .or_else(|| v.get("url").and_then(|x| x.as_str()))
        .unwrap_or("?");
    println!("✅ Netlify 部署已创建: deploy={id} state={state} → {site_url}");
    Ok(())
}

/// Vercel 部署：public 文件以 base64 内联，经 v13 Deployments API 上传（规格书 §7.2）。
fn deploy_vercel(root: &Path, config: &Config, dry_run: bool) -> anyhow::Result<()> {
    use base64::Engine;

    let cfg = &config.deploy.vercel;
    let token = auth_token(&cfg.auth_token, "VERCEL_TOKEN")?;
    let mut url = "https://api.vercel.com/v13/deployments".to_string();
    if !cfg.project_id.trim().is_empty() {
        url.push_str(&format!("?projectId={}", cfg.project_id.trim()));
    }

    if dry_run {
        println!("[dry-run] Vercel 部署");
        println!("  API: POST {url}");
        println!("  Auth: Bearer {}（{} 字符）", mask_token(&token), token.len());
        println!("  内容: public/ 全部文件 base64 内联上传");
        return Ok(());
    }

    ensure_built(root, config)?;
    let public = config.output_dir(root);
    let files = collect_files(&public)?;
    let mut payload_files = Vec::with_capacity(files.len());
    let mut total = 0usize;
    for (rel, path) in &files {
        let data = std::fs::read(path)?;
        total += data.len();
        let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
        payload_files.push(serde_json::json!({ "file": rel, "data": b64 }));
    }
    println!("📤 上传 {} 个文件 / {} 字节到 Vercel…", files.len(), total);

    let payload = serde_json::json!({
        "name": slugify_project(config.site.title.trim()),
        "files": payload_files,
        "projectSettings": { "framework": null },
    });
    let resp = ureq::post(&url)
        .set("Authorization", &format!("Bearer {token}"))
        .set("Content-Type", "application/json")
        .send_string(&payload.to_string())
        .map_err(|e| anyhow::anyhow!("Vercel API 请求失败: {e}"))?;
    let status = resp.status();
    let body = resp.into_string().unwrap_or_default();
    if !(200..300).contains(&status) {
        anyhow::bail!("Vercel API 返回 {status}: {body}");
    }
    let v: serde_json::Value = serde_json::from_str(&body)?;
    let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("?");
    let state = v.get("readyState").and_then(|x| x.as_str()).unwrap_or("?");
    let deploy_url = v
        .get("url")
        .and_then(|x| x.as_str())
        .map(|u| format!("https://{u}"))
        .unwrap_or_else(|| "?".into());
    println!("✅ Vercel 部署已创建: deployment={id} state={state} → {deploy_url}");
    Ok(())
}

/// Vercel 项目名：站点标题转小写连字符（非法字符剔除、连续分隔符合并）。
fn slugify_project(title: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            last_dash = false;
        } else if (c.is_whitespace() || c == '-' || c == '_') && !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    let out = out.trim_matches('-').to_string();
    if out.is_empty() {
        "typall-site".to_string()
    } else {
        out
    }
}

fn ensure_built(root: &Path, config: &Config) -> anyhow::Result<()> {
    let public = config.output_dir(root);
    if !public.exists() {
        println!("未发现构建产物，先执行构建…");
        build::build(root, config, true)?;
    }
    Ok(())
}

fn deploy_copy(root: &Path, config: &Config, dry_run: bool) -> anyhow::Result<()> {
    let target = &config.deploy.copy.target_dir;
    if target.is_empty() {
        anyhow::bail!("未配置 deploy.copy.target_dir");
    }
    let target = expand_tilde(Path::new(target));
    let public = config.output_dir(root);

    if dry_run {
        println!("[dry-run] 将复制 {} → {}", public.display(), target.display());
        return Ok(());
    }

    ensure_built(root, config)?;
    copy_dir_recursive(&public, &target)?;
    println!("✅ 已复制到 {}", target.display());
    Ok(())
}

fn deploy_git(root: &Path, config: &Config, dry_run: bool) -> anyhow::Result<()> {
    let public = config.output_dir(root);
    let git = &config.deploy.git;
    let branch = if git.branch.is_empty() {
        "gh-pages".to_string()
    } else {
        git.branch.clone()
    };
    let repo = &git.repo;

    if dry_run {
        let repo_desc = if repo.is_empty() { "(本地仓库)" } else { repo.as_str() };
        println!("[dry-run] git 部署: repo={repo_desc}, branch={branch}");
        return Ok(());
    }

    ensure_built(root, config)?;

    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
    let tmp = std::env::temp_dir().join(format!("typall-deploy-{stamp}"));

    // 1. 准备仓库
    if repo.is_empty() {
        std::fs::create_dir_all(&tmp)?;
        git_cmd(&tmp, &["init", "-b", &branch])?;
    } else {
        let cloned = Command::new("git")
            .arg("clone")
            .arg("--depth").arg("1")
            .arg("--branch").arg(&branch)
            .arg(repo)
            .arg(&tmp)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !cloned {
            // 分支可能尚不存在，克隆默认分支
            if tmp.exists() {
                std::fs::remove_dir_all(&tmp)?;
            }
            let ok = Command::new("git")
                .arg("clone")
                .arg("--depth").arg("1")
                .arg(repo)
                .arg(&tmp)
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if !ok {
                anyhow::bail!("无法克隆仓库 {repo}");
            }
        }
    }

    // 2. 清空临时目录（保留 .git）
    clear_keep_git(&tmp)?;

    // 3. 复制 public → 临时目录
    copy_dir_recursive(&public, &tmp)?;

    // 4. 提交
    git_cmd(&tmp, &["add", "-A"])?;
    let msg = commit_message(&git.commit_message);
    let has_changes = !git_output(&tmp, &["status", "--porcelain"])?.trim().is_empty();
    if has_changes {
        git_cmd(&tmp, &["commit", "-m", &msg])?;
    } else {
        println!("无内容变化，跳过提交");
    }

    // 5. 打标签（便于回滚）：仅在有新提交时——空部署反复打 tag 会向远端
    //    灌入指向同一提交的无意义标签
    let tag = format!("deploy-{stamp}");
    if has_changes {
        git_cmd(&tmp, &["tag", &tag])?;
    }

    // 6. 推送
    let pushed = !repo.is_empty() && git.auto_push;
    if pushed {
        if has_changes {
            git_cmd(&tmp, &["push", "origin", &format!("HEAD:{branch}")])?;
            git_cmd(&tmp, &["push", "origin", &tag])?;
            println!("✅ 已部署到远程 {branch}（标签 {tag}）");
        } else {
            println!("✅ 远程 {branch} 已是最新（无内容变化）");
        }
    } else if !repo.is_empty() {
        println!("⚠️ auto_push 已关闭，未推送。仓库保留在 {}", tmp.display());
    } else if has_changes {
        println!("✅ 已在本地提交（标签 {tag}），仓库保留在 {}（未配置远程仓库）", tmp.display());
    } else {
        println!("无内容变化，仓库保留在 {}（未配置远程仓库）", tmp.display());
    }

    // 7. 清理临时目录（失败不阻塞，因为是临时文件）。仅在完成推送后清理：
    //    auto_push 关闭 / 未配 repo 时，临时仓库（含提交与标签）就是交付物，
    //    删掉会让上面的提示变成谎言。
    if pushed {
        let _ = std::fs::remove_dir_all(&tmp);
    }

    Ok(())
}

fn git_cmd(dir: &Path, args: &[&str]) -> anyhow::Result<()> {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output()?;
    if !out.status.success() {
        anyhow::bail!(
            "git {:?} 失败: {}",
            args,
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

fn git_output(dir: &Path, args: &[&str]) -> anyhow::Result<String> {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output()?;
    if !out.status.success() {
        anyhow::bail!(
            "git {:?} 失败: {}",
            args,
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn commit_message(template: &str) -> String {
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
    if template.is_empty() {
        format!("Update blog at {stamp}")
    } else {
        template.replace("{timestamp}", &stamp)
    }
}

fn clear_keep_git(dir: &Path) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_name() == ".git" {
            continue;
        }
        let p = entry.path();
        if p.is_dir() {
            std::fs::remove_dir_all(&p)?;
        } else {
            std::fs::remove_file(&p)?;
        }
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn expand_tilde(p: &Path) -> PathBuf {
    if let Some(s) = p.to_str()
        && let Some(rest) = s.strip_prefix("~/")
            && let Some(home) = std::env::var_os("USERPROFILE")
                .or_else(|| std::env::var_os("HOME"))
            {
                return PathBuf::from(home).join(rest);
            }
    p.to_path_buf()
}

/// 执行部署钩子（shell 命令），失败则 bail（规格书 §7.4）。
fn run_hook(name: &str, cmd: &str) -> anyhow::Result<()> {
    println!("🪝 执行 {name} 钩子: {cmd}");
    #[cfg(target_os = "windows")]
    let status = std::process::Command::new("cmd").args(["/C", cmd]).status();
    #[cfg(not(target_os = "windows"))]
    let status = std::process::Command::new("sh").args(["-c", cmd]).status();
    let status = status?;
    if !status.success() {
        anyhow::bail!("{name} 钩子执行失败（退出码 {:?})", status.code());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_message_uses_template_with_timestamp() {
        let msg = commit_message("Deploy at {timestamp}");
        assert!(msg.starts_with("Deploy at "));
        assert!(msg.len() > "Deploy at ".len());
    }

    #[test]
    fn mask_token_handles_multibyte_and_short() {
        // 短 token（<=8 char）整段掩码
        assert_eq!(mask_token("abc"), "******");
        assert_eq!(mask_token("12345678"), "******");
        // 多字节 token：按 char 切片不 panic（9 个 char，取首 4 尾 4）
        assert_eq!(mask_token("密钥令牌测试超长值"), "密钥令牌…试超长值");
        assert_eq!(mask_token("ghp_abcdefghijklmnop"), "ghp_…mnop");
    }

    #[test]
    fn commit_message_default_includes_timestamp() {
        let msg = commit_message("");
        assert!(msg.starts_with("Update blog at "));
    }

    #[test]
    fn auth_token_prefers_config_value() {
        let got = auth_token("config-token", "NETLIFY_AUTH_TOKEN").unwrap();
        assert_eq!(got, "config-token");
    }

    #[test]
    fn auth_token_falls_back_to_env() {
        let _env = crate::config::ENV_LOCK.lock().unwrap();
        // 2024 edition 中 env 修改为 unsafe；用后即清理
        unsafe { std::env::set_var("NETLIFY_AUTH_TOKEN", "env-token") };
        let got = auth_token("", "NETLIFY_AUTH_TOKEN").unwrap();
        unsafe { std::env::remove_var("NETLIFY_AUTH_TOKEN") };
        assert_eq!(got, "env-token");
    }

    #[test]
    fn auth_token_errors_when_missing() {
        let _env = crate::config::ENV_LOCK.lock().unwrap();
        unsafe { std::env::remove_var("VERCEL_TOKEN") };
        let err = auth_token("", "VERCEL_TOKEN").unwrap_err();
        assert!(err.to_string().contains("VERCEL_TOKEN"));
    }

    #[test]
    fn mask_token_masks_short_and_long() {
        assert_eq!(mask_token("short"), "******");
        let m = mask_token("abcdefghijklmnop");
        assert!(m.starts_with("abcd"));
        assert!(m.ends_with("mnop"));
        assert!(!m.contains("efghijkl"));
    }

    #[test]
    fn zip_directory_packs_files() {
        let dir = std::env::temp_dir().join(format!("typall-zip-test-{}", std::process::id()));
        let sub = dir.join("sub");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(dir.join("index.html"), "<h1>hi</h1>").unwrap();
        std::fs::write(sub.join("a.css"), "body{}").unwrap();

        let (bytes, n) = zip_directory(&dir).unwrap();
        assert_eq!(n, 2);

        let reader = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(reader).unwrap();
        assert_eq!(archive.len(), 2);
        let mut names = vec![archive.by_index(0).unwrap().name().to_string()];
        names.push(archive.by_index(1).unwrap().name().to_string());
        names.sort();
        assert_eq!(names, vec!["index.html", "sub/a.css"]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn slugify_project_normalizes_title() {
        assert_eq!(slugify_project("My Blog!"), "my-blog");
        assert_eq!(slugify_project("中文站点"), "typall-site");
        assert_eq!(slugify_project("  A  B  "), "a-b");
    }
}
