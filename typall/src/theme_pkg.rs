//! 主题打包分发（规格书 §2.3）：
//!
//! - `package`：把 `themes/<name>/` 打包为 `themes-<name>.zip`（zip 内顶层目录为主题名）；
//! - `install`：从本地 zip 路径或 http(s) URL 安装主题到 `themes/`，
//!   自动识别顶层目录，校验包内路径安全与主题完整性。

use std::io::Write;
use std::path::{Path, PathBuf};

/// 递归收集目录下文件，返回（zip 内相对路径，磁盘路径）。
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

/// 打包主题为 zip，输出到项目根 `themes-<name>.zip`，返回输出路径。
pub fn package(root: &Path, name: &str) -> anyhow::Result<PathBuf> {
    let dir = root.join("themes").join(name);
    if !dir.is_dir() {
        anyhow::bail!("主题不存在: {}（请确认 themes/{name}/ 已创建）", dir.display());
    }
    if !dir.join("style.css").is_file() && !dir.join("template.html").is_file() {
        anyhow::bail!(
            "{} 不是有效主题（需包含 style.css 或 template.html）",
            dir.display()
        );
    }

    let files = collect_files(&dir)?;
    let mut buf = Vec::new();
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
    let opts = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (rel, path) in &files {
        writer.start_file(format!("{name}/{rel}"), opts)?;
        let data = std::fs::read(path)?;
        writer.write_all(&data)?;
    }
    writer.finish()?;

    let out_path = root.join(format!("themes-{name}.zip"));
    std::fs::write(&out_path, buf)?;
    println!("📦 {}（{} 个文件）", out_path.display(), files.len());
    Ok(out_path)
}

/// 从本地 zip 路径或 http(s) URL 安装主题，返回安装后的主题名。
pub fn install(root: &Path, source: &str) -> anyhow::Result<String> {
    let bytes: Vec<u8> = if source.starts_with("http://") || source.starts_with("https://") {
        println!("⬇️ 下载 {source} …");
        let resp = ureq::get(source)
            .call()
            .map_err(|e| anyhow::anyhow!("下载失败: {e}"))?;
        let status = resp.status();
        if !(200..300).contains(&status) {
            anyhow::bail!("下载返回 HTTP {status}");
        }
        use std::io::Read;
        let mut buf = Vec::new();
        resp.into_reader()
            .take(50 * 1024 * 1024) // 50MB 上限，防恶意大包
            .read_to_end(&mut buf)?;
        buf
    } else {
        let p = Path::new(source);
        if !p.is_file() {
            anyhow::bail!("找不到主题包: {}", p.display());
        }
        std::fs::read(p)?
    };

    install_from_bytes(root, &bytes, source)
}

fn install_from_bytes(root: &Path, bytes: &[u8], source: &str) -> anyhow::Result<String> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;

    // 1. 扫描：校验路径安全，识别顶层目录
    let mut tops: Vec<String> = Vec::new();
    for i in 0..archive.len() {
        let f = archive.by_index(i)?;
        let name = f.name().to_string();
        let segments: Vec<&str> = name.split('/').collect();
        // 绝对路径条目（`/x`）与中部空段（`a//b`）同样非法：`Path::join` 遇到
        // 带根的路径会丢弃基路径，解压会写出主题目录之外（zip-slip 变体）。
        let empty_mid = segments
            .iter()
            .enumerate()
            .any(|(i, s)| s.is_empty() && i + 1 != segments.len());
        if name.starts_with('/')
            || empty_mid
            || segments.iter().any(|s| *s == ".." || s.contains('\\') || s.contains(':'))
        {
            anyhow::bail!("主题包包含非法路径: {name}");
        }
        if let Some(first) = segments.first()
            && !first.is_empty()
            && !tops.contains(&first.to_string())
        {
            tops.push(first.to_string());
        }
    }
    // 单一共享顶层目录 → 解压时剥掉；多顶层/无顶层 → 保留原路径
    let strip_top: Option<String> = if tops.len() == 1 {
        Some(tops.remove(0))
    } else {
        None
    };

    let theme_name = match &strip_top {
        Some(t) => t.clone(),
        None => infer_name(source),
    };
    if theme_name.is_empty() || theme_name == "." || theme_name == "themes" {
        anyhow::bail!("无法从主题包推断主题名，请确保 zip 内含顶层目录或使用 <name>.zip 命名");
    }

    let dest = root.join("themes").join(&theme_name);
    if dest.exists() {
        anyhow::bail!("主题已存在: {}（如需覆盖请先手动删除）", dest.display());
    }

    // 2. 解压（带总体积上限：下载侧只限压缩包大小，Deflate 高压缩比的
    //    zip 炸弹在解压侧仍可膨胀数十 GB）
    std::fs::create_dir_all(&dest)?;
    const MAX_UNCOMPRESSED: u64 = 200 * 1024 * 1024;
    let mut total: u64 = 0;
    for i in 0..archive.len() {
        let mut f = archive.by_index(i)?;
        if f.is_dir() {
            continue;
        }
        let name = f.name().to_string();
        let rel = match &strip_top {
            Some(t) => name
                .strip_prefix(t)
                .and_then(|r| r.strip_prefix('/'))
                .unwrap_or(&name)
                .to_string(),
            None => name.clone(),
        };
        if rel.is_empty() {
            continue;
        }
        let out_path = dest.join(&rel);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut out = std::fs::File::create(&out_path)?;
        total += std::io::copy(&mut f, &mut out)?;
        if total > MAX_UNCOMPRESSED {
            let _ = std::fs::remove_dir_all(&dest);
            anyhow::bail!("主题包解压总体积超过 200MB 上限，已中止安装");
        }
    }

    // 3. 完整性校验：至少 style.css 或 template.html 之一
    if !dest.join("style.css").is_file() && !dest.join("template.html").is_file() {
        let _ = std::fs::remove_dir_all(&dest);
        anyhow::bail!("不是有效主题包（解压后未找到 style.css 或 template.html）");
    }

    Ok(theme_name)
}

/// 从来源推断主题名：URL 取末段，本地路径取文件名，去掉 `.zip` 后缀。
fn infer_name(source: &str) -> String {
    let trimmed = source.trim_end_matches(['/', '\\']);
    let base = match trimmed.rfind(['/', '\\']) {
        Some(idx) => &trimmed[idx + 1..],
        None => trimmed,
    };
    base.strip_suffix(".zip")
        .map(|s| s.to_string())
        .unwrap_or_else(|| base.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_zip(files: &[(&str, &str)]) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
        let opts = zip::write::SimpleFileOptions::default();
        for (name, content) in files {
            writer.start_file(*name, opts).unwrap();
            writer.write_all(content.as_bytes()).unwrap();
        }
        writer.finish().unwrap();
        buf
    }

    #[test]
    fn install_with_top_dir_strips_it() {
        let root = std::env::temp_dir().join(format!("typall-theme-test-{}", std::process::id()));
        let zip = make_zip(&[
            ("my-theme/style.css", "body{}"),
            ("my-theme/template.html", "<html>{{title}}</html>"),
            ("my-theme/static/logo.png", "png"),
        ]);
        let name = install_from_bytes(&root, &zip, "my-theme.zip").unwrap();
        assert_eq!(name, "my-theme");
        assert!(root.join("themes/my-theme/style.css").is_file());
        assert!(root.join("themes/my-theme/static/logo.png").is_file());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn install_flat_zip_uses_source_name() {
        let root = std::env::temp_dir().join(format!("typall-theme-test2-{}", std::process::id()));
        let zip = make_zip(&[("style.css", "body{}"), ("template.html", "<html/>")]);
        let name = install_from_bytes(&root, &zip, "flat-theme.zip").unwrap();
        assert_eq!(name, "flat-theme");
        assert!(root.join("themes/flat-theme/style.css").is_file());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn install_rejects_path_traversal() {
        let root = std::env::temp_dir().join(format!("typall-theme-test3-{}", std::process::id()));
        let zip = make_zip(&[("../evil/style.css", "body{}")]);
        assert!(install_from_bytes(&root, &zip, "evil.zip").is_err());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn install_rejects_absolute_and_double_slash_entries() {
        // 回归：`/evil.css` 的首段为空串，旧校验三项检查全放行；
        // `Path::join` 遇带根路径会丢弃基路径，解压写出主题目录之外。
        let root = std::env::temp_dir().join(format!("typall-theme-test5-{}", std::process::id()));
        let zip = make_zip(&[("/evil.css", "body{}")]);
        assert!(install_from_bytes(&root, &zip, "evil2.zip").is_err());
        let zip2 = make_zip(&[("a//style.css", "body{}")]);
        assert!(install_from_bytes(&root, &zip2, "evil3.zip").is_err());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn install_rejects_invalid_theme() {
        let root = std::env::temp_dir().join(format!("typall-theme-test4-{}", std::process::id()));
        let zip = make_zip(&[("junk/readme.txt", "hello")]);
        assert!(install_from_bytes(&root, &zip, "junk.zip").is_err());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn infer_name_strips_zip_suffix() {
        assert_eq!(infer_name("my-theme.zip"), "my-theme");
        assert_eq!(infer_name("https://example.com/t/themes.zip"), "themes");
        assert_eq!(infer_name("a/b/c.tgz"), "c.tgz");
    }
}
