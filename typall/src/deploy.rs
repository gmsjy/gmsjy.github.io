//! 部署：`copy`（本地复制）、`git`（推送静态站点到分支）、
//! `netlify` / `vercel`（官方 API 上传部署）。

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

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

// ---------------------------------------------------------------------------
// 定时/周期部署（[deploy.schedule]，serve 常驻期间后台执行）
// ---------------------------------------------------------------------------

/// 单个部署触发器。
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ScheduleTrigger {
    /// 固定周期（自上次部署完成起算）。
    Every(std::time::Duration),
    /// 每天本地时刻 HH:MM。
    DailyAt(u8, u8),
}

/// 校验 `[deploy.schedule]` 格式（Config::load 启动期调用，报人话错误）。
pub(crate) fn validate_schedule(
    schedule: &crate::config::DeploySchedule,
) -> anyhow::Result<()> {
    parse_schedule(schedule).map(|_| ())
}

/// 解析计划为触发器列表；两字段皆空 = 停用（空 Vec）。
pub(crate) fn parse_schedule(
    schedule: &crate::config::DeploySchedule,
) -> anyhow::Result<Vec<ScheduleTrigger>> {
    let mut triggers = Vec::new();
    if !schedule.every.trim().is_empty() {
        triggers.push(ScheduleTrigger::Every(parse_interval(&schedule.every)?));
    }
    for t in &schedule.daily {
        let (h, m) = parse_hhmm(t)?;
        triggers.push(ScheduleTrigger::DailyAt(h, m));
    }
    Ok(triggers)
}

/// 周期间隔：数字 + 单位（m/h/d），最小 1 分钟。
fn parse_interval(s: &str) -> anyhow::Result<std::time::Duration> {
    let raw = s.trim();
    let (digits, unit) = raw.split_at(raw.len().saturating_sub(1));
    let secs = match unit {
        "m" => 60u64,
        "h" => 3600,
        "d" => 86400,
        _ => anyhow::bail!("deploy.schedule.every = {s:?} 格式错误：应为 数字+单位（如 30m / 2h / 1d）"),
    };
    let n: u64 = digits
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("deploy.schedule.every = {s:?} 格式错误：应为 数字+单位（如 30m / 2h / 1d）"))?;
    let total = n
        .checked_mul(secs)
        .ok_or_else(|| anyhow::anyhow!("deploy.schedule.every = {s:?} 数值过大"))?;
    if total < 60 {
        anyhow::bail!("deploy.schedule.every = {s:?} 周期过短：最少 1 分钟");
    }
    Ok(std::time::Duration::from_secs(total))
}

/// 每日时刻 "HH:MM"（小时可 1-2 位，分钟 0-59）。
fn parse_hhmm(s: &str) -> anyhow::Result<(u8, u8)> {
    let (h, m) = s
        .trim()
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("deploy.schedule.daily = {s:?} 格式错误：应为 HH:MM（如 06:30 / 21:00）"))?;
    let h: u8 = h
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("deploy.schedule.daily = {s:?} 小时位格式错误"))?;
    let m: u8 = m
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("deploy.schedule.daily = {s:?} 分钟位格式错误"))?;
    if h > 23 {
        anyhow::bail!("deploy.schedule.daily = {s:?} 小时需在 00-23");
    }
    if m > 59 {
        anyhow::bail!("deploy.schedule.daily = {s:?} 分钟需在 00-59");
    }
    Ok((h, m))
}

/// 计划的人类可读描述（serve 启动横幅与触发日志共用）。
pub(crate) fn describe_schedule(triggers: &[ScheduleTrigger]) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut dailies: Vec<String> = Vec::new();
    for t in triggers {
        match t {
            ScheduleTrigger::Every(d) => parts.push(format!("每 {}", humanize_duration(*d))),
            ScheduleTrigger::DailyAt(h, m) => dailies.push(format!("{h:02}:{m:02}")),
        }
    }
    if !dailies.is_empty() {
        parts.push(format!("每天 {}", dailies.join("、")));
    }
    parts.join("；")
}

fn humanize_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs();
    if secs.is_multiple_of(86400) {
        format!("{}d", secs / 86400)
    } else if secs.is_multiple_of(3600) {
        format!("{}h", secs / 3600)
    } else {
        format!("{}m", secs / 60)
    }
}

/// 距下一次触发的时长（取所有触发器中最近的一个）。
pub(crate) fn next_wait(
    triggers: &[ScheduleTrigger],
    now: chrono::DateTime<chrono::Local>,
) -> std::time::Duration {
    let mut best: Option<chrono::Duration> = None;
    for t in triggers {
        let delta = match t {
            ScheduleTrigger::Every(d) => chrono::Duration::from_std(*d).unwrap_or_else(|_| chrono::Duration::minutes(1)),
            ScheduleTrigger::DailyAt(h, m) => {
                let today = now
                    .date_naive()
                    .and_hms_opt((*h).into(), (*m).into(), 0)
                    .unwrap_or_default();
                let target = if today <= now.naive_local() {
                    today + chrono::Duration::days(1)
                } else {
                    today
                };
                target - now.naive_local()
            }
        };
        if best.is_none_or(|b| delta < b) {
            best = Some(delta);
        }
    }
    best.and_then(|d| d.to_std().ok())
        .unwrap_or(std::time::Duration::from_secs(60))
}

/// serve 启动时挂载定时部署后台线程；未配置 `[deploy.schedule]` 返回 None。
///
/// 每次触发：重读配置（策略/仓库/计划的修改免重启）→ 全量构建（增量缓存，
/// 未变文章直接命中；定时发布的未来日期文章由这一步到点带上线）→ 按
/// `[deploy] strategy` 部署一次。构建或部署失败只告警不退出，等下次触发。
///
/// `build_lock`：与 serve 的文件监听重建/live 快速路径互斥（构建产物与
/// 孤儿清理不允许并发写），构建+部署全程持锁。
pub(crate) fn spawn_scheduled_deploy(
    root: PathBuf,
    config: Config,
    build_lock: Arc<std::sync::Mutex<()>>,
) -> Option<std::thread::JoinHandle<()>> {
    let triggers = parse_schedule(&config.deploy.schedule).ok()?;
    if triggers.is_empty() {
        return None;
    }
    println!("⏰ 定时部署已启动：{}（[deploy.schedule]，随 serve 常驻）", describe_schedule(&triggers));
    Some(std::thread::spawn(move || loop {
        std::thread::sleep(next_wait(&triggers, chrono::Local::now()));
        println!("⏰ 定时部署触发……");
        let config = match Config::load(&root) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("⚠️ 定时部署跳过（配置读取失败）：{e:#}");
                continue;
            }
        };
        let _guard = build_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Err(e) = build::build(&root, &config, false) {
            eprintln!("⚠️ 定时部署的构建失败，本次跳过部署：{e:#}");
            continue;
        }
        if let Err(e) = deploy(&root, &config, None, false) {
            eprintln!("⚠️ 定时部署失败（下个周期自动重试）：{e:#}");
        }
    }))
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
    // 一次序列化直接发送：payload 里是全站 base64，`to_string()` 再拷一份
    // 会让内存峰值翻倍。
    let body_bytes = serde_json::to_vec(&payload)?;
    let resp = ureq::post(&url)
        .set("Authorization", &format!("Bearer {token}"))
        .set("Content-Type", "application/json")
        .send_bytes(&body_bytes)
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

    // 目录名带 pid：同秒内的两次部署（如 serve 定时部署与手动 deploy 并行）
    // 不再互相覆盖临时仓库（对照 packages.rs 的下载目录惯例）。
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
    let tmp = std::env::temp_dir().join(format!("typall-deploy-{stamp}-{}", std::process::id()));

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
            git_push_retry(&tmp, &["push", "origin", &format!("HEAD:{branch}")])?;
            git_push_retry(&tmp, &["push", "origin", &tag])?;
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

/// 带退避重试的 git push：对端瞬时抖动（连接重置/超时/DNS 失败）很常见，
/// 一次失败就报错会中断整个部署。共尝试 3 次（间隔 3s/6s 递增）。
/// 注意：这只兜「抖动」；网络被持续阻断时 3 次同样失败——那种情况交给
/// 外层的定时重试（或用户恢复网络后重新 `typall deploy`）。
fn git_push_retry(dir: &Path, args: &[&str]) -> anyhow::Result<()> {
    const MAX_ATTEMPTS: u32 = 3;
    let mut last_err = None;
    for attempt in 1..=MAX_ATTEMPTS {
        match git_cmd(dir, args) {
            Ok(()) => return Ok(()),
            Err(e) => {
                last_err = Some(e);
                if attempt < MAX_ATTEMPTS {
                    let wait = 3u64 * u64::from(attempt);
                    eprintln!("⚠️ push 失败（第 {attempt}/{MAX_ATTEMPTS} 次），{wait}s 后自动重试…");
                    std::thread::sleep(std::time::Duration::from_secs(wait));
                }
            }
        }
    }
    Err(last_err.expect("至少尝试一次"))
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

    fn schedule(every: &str, daily: &[&str]) -> crate::config::DeploySchedule {
        crate::config::DeploySchedule {
            every: every.to_string(),
            daily: daily.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn parse_schedule_accepts_intervals_and_daily() {
        let t = parse_schedule(&schedule("30m", &["06:30", "21:00"])).unwrap();
        assert_eq!(
            t,
            vec![
                ScheduleTrigger::Every(std::time::Duration::from_secs(30 * 60)),
                ScheduleTrigger::DailyAt(6, 30),
                ScheduleTrigger::DailyAt(21, 0),
            ]
        );
        // 1 位小时 / 1 位分钟也接受
        let t2 = parse_schedule(&schedule("2h", &["6:05"])).unwrap();
        assert_eq!(t2[1], ScheduleTrigger::DailyAt(6, 5));
        // 空配置 = 停用
        assert!(parse_schedule(&schedule("", &[])).unwrap().is_empty());
        // 只配 daily 也行
        assert_eq!(
            parse_schedule(&schedule("", &["23:59"])).unwrap(),
            vec![ScheduleTrigger::DailyAt(23, 59)]
        );
    }

    #[test]
    fn parse_schedule_rejects_bad_input() {
        // 周期格式
        assert!(parse_schedule(&schedule("abc", &[])).is_err());
        assert!(parse_schedule(&schedule("30", &[])).is_err()); // 缺单位
        assert!(parse_schedule(&schedule("30x", &[])).is_err());
        assert!(parse_schedule(&schedule("0m", &[])).is_err()); // 小于 1 分钟
        // 时刻格式
        assert!(parse_schedule(&schedule("", &["24:00"])).is_err());
        assert!(parse_schedule(&schedule("", &["12:60"])).is_err());
        assert!(parse_schedule(&schedule("", &["1230"])).is_err());
        assert!(parse_schedule(&schedule("", &["aa:bb"])).is_err());
    }

    #[test]
    fn next_wait_picks_nearest_trigger() {
        use chrono::TimeZone;
        // 固定「现在」：2026-09-14 10:00 本地时间
        let now = chrono::Local
            .with_ymd_and_hms(2026, 9, 14, 10, 0, 0)
            .unwrap();
        // 周期：直接返回周期本身
        assert_eq!(
            next_wait(&[ScheduleTrigger::Every(std::time::Duration::from_secs(1800))], now),
            std::time::Duration::from_secs(1800)
        );
        // 今日 21:00 未到 → 11 小时
        assert_eq!(
            next_wait(&[ScheduleTrigger::DailyAt(21, 0)], now),
            std::time::Duration::from_secs(11 * 3600)
        );
        // 今日 09:00 已过 → 明天 09:00，即 23 小时
        assert_eq!(
            next_wait(&[ScheduleTrigger::DailyAt(9, 0)], now),
            std::time::Duration::from_secs(23 * 3600)
        );
        // 恰好等于当前时刻 → 算作已过，取明天
        assert_eq!(
            next_wait(&[ScheduleTrigger::DailyAt(10, 0)], now),
            std::time::Duration::from_secs(24 * 3600)
        );
        // 多触发器取最近（30m < 11h）
        assert_eq!(
            next_wait(
                &[
                    ScheduleTrigger::Every(std::time::Duration::from_secs(1800)),
                    ScheduleTrigger::DailyAt(21, 0),
                ],
                now
            ),
            std::time::Duration::from_secs(1800)
        );
    }

    #[test]
    fn describe_schedule_is_human_readable() {
        let t = parse_schedule(&schedule("30m", &["06:30", "21:00"])).unwrap();
        assert_eq!(describe_schedule(&t), "每 30m；每天 06:30、21:00");
        let only_daily = parse_schedule(&schedule("", &["09:05"])).unwrap();
        assert_eq!(describe_schedule(&only_daily), "每天 09:05");
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
