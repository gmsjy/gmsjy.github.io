//! PDF 导出（`typall pdf`）：把单篇/全部文章编译为 PDF。
//!
//! 与站点构建（HTML 目标）的关键区别：
//! - **字体**：站点构建用确定性内置字体集（不扫系统，保证产物可复现）；PDF
//!   内嵌字体、天然与机器相关，因此额外加载系统字体目录兜底 CJK 等
//!   typst-assets 未内置的字体（[`world::system_font_dirs`]）。
//! - **公式编号**：站点 HTML 因 typst 0.15 `html export` 的编号字形回退 bug
//!   在 preamble 里用 `numbering: (..) => none` 抑制编号、改由 HTML 后处理层
//!   注入 `eq-num` span；PDF 走原生 paged 渲染，编号渲染正确，**不能**复用该
//!   抑制——否则 PDF 会丢公式编号。故此处使用独立的 `pdf_preamble`。
//! - **公式渲染**：paged 目标始终原生渲染公式，不受 `[build.math] renderer`
//!   （mathml/svg）影响，天然像素级一致。

use std::path::{Path, PathBuf};
use std::sync::Arc;

use typst::diag::Warned;
use typst_layout::PagedDocument;
use typst_pdf::{pdf, PdfOptions};

use crate::build::{format_diag, typst_lang};
use crate::config::Config;
use crate::content::{self, DocumentMeta};
use crate::world::{system_font_dirs, SharedAssets, TypallWorld};

/// 导出文章（可选按 slug 过滤）为 PDF。
///
/// - `slug`：精确匹配文章的 rel 路径 slug（如 `posts/quantum`），或省略全部导出。
/// - `output`：输出目录，默认 `<root>/pdf/`。
/// - `cli_include_drafts`：与 `[build] drafts` / `TP_BUILD_DRAFTS` 合并后
///   决定是否包含草稿与未来日期的文章（与 `build` 语义一致）。
pub fn export(
    root: &Path,
    config: &Config,
    slug: Option<&str>,
    output: Option<&str>,
    cli_include_drafts: bool,
) -> anyhow::Result<()> {
    let include_drafts = config.build.drafts || cli_include_drafts;
    let t_start = std::time::Instant::now();

    // PDF 内嵌字体、天然机器相关：额外加载系统字体目录兜底 CJK。
    let shared = Arc::new(SharedAssets::load_with_font_dirs(root, &system_font_dirs())?);
    let preamble = pdf_preamble(config, root);

    let out = output
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("pdf"));
    std::fs::create_dir_all(&out)?;

    let mut files: Vec<PathBuf> = content::scan_typ_files(&root.join("posts"));
    files.extend(content::scan_typ_files(&root.join("pages")));
    crate::build::ensure_project_root(root)?;

    // slug 过滤：匹配完整 slug（posts/xxx）、省略前缀的 slug（xxx）或文件名。
    if let Some(slug) = slug {
        let slug = slug.trim_matches('/');
        files.retain(|f| {
            let s = content::slug_of(&content::rel_path(root, f));
            s == slug
                || s.ends_with(&format!("/{slug}"))
                || f.file_stem().is_some_and(|st| st == slug)
        });
        if files.is_empty() {
            anyhow::bail!("未找到 slug = {slug} 的文章/页面");
        }
    }

    // 草稿过滤（与 build 一致：草稿 + 未来日期的文章不导出）。
    if !include_drafts {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        files.retain(|f| {
            let src = std::fs::read_to_string(f).unwrap_or_default();
            let meta = DocumentMeta::from_map(&content::extract_meta(&src));
            if meta.draft {
                return false;
            }
            match &meta.date {
                Some(d) if !d.is_empty() => d.as_str() <= today.as_str(),
                _ => true,
            }
        });
    }

    if files.is_empty() {
        anyhow::bail!("没有可导出的文章（无匹配文件，或全部为草稿）");
    }

    println!(
        "📄 导出 {} 个文件到 {}（字体：内置 + 系统目录兜底）",
        files.len(),
        out.display()
    );

    let mut ok = 0usize;
    let mut errors: Vec<String> = Vec::new();
    for f in &files {
        let rel = content::rel_path(root, f);
        let slug = content::slug_of(&rel);
        let world = TypallWorld::new(root, &rel, shared.clone(), &preamble)?;
        let Warned { output, warnings } = typst::compile::<PagedDocument>(&world);

        // 警告：跳过无源位置的引擎级警告（与 build 一致），避免刷屏。
        for w in &warnings {
            if w.span.is_detached() {
                continue;
            }
            eprintln!("警告 {}", format_diag(&world, w));
        }

        match output {
            Err(errs) => {
                let msgs: Vec<String> = errs
                    .iter()
                    .map(|e| format!("  - {}", format_diag(&world, e)))
                    .collect();
                errors.push(format!("编译失败 [{rel}]:\n{}", msgs.join("\n")));
            }
            Ok(paged) => match pdf(&paged, &PdfOptions::default()) {
                Err(errs) => {
                    let msgs: Vec<String> = errs
                        .iter()
                        .map(|e| format!("  - {}", format_diag(&world, e)))
                        .collect();
                    errors.push(format!("PDF 编码失败 [{rel}]:\n{}", msgs.join("\n")));
                }
                Ok(bytes) => {
                    let path = out.join(format!("{slug}.pdf"));
                    // slug 带 posts/pages 前缀 → 输出为 pdf/posts/xxx.pdf，
                    // 需要逐层确保父目录存在。
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&path, &bytes)?;
                    println!(
                        "✅ {}.pdf  ({} 页, {:.1} KB)",
                        slug,
                        paged.pages().len(),
                        bytes.len() as f64 / 1024.0
                    );
                    ok += 1;
                }
            },
        }
    }

    if !errors.is_empty() {
        for e in &errors {
            eprintln!("❌ {e}");
        }
        anyhow::bail!("{} 个文件导出失败", errors.len());
    }

    println!(
        "⏱ 导出完成：{} 个文件，总耗时 {:.2}s",
        ok,
        t_start.elapsed().as_secs_f64()
    );
    Ok(())
}

/// PDF 目标专用的 preamble：设置语言 + 公式编号 + 自动加载宏。
///
/// 刻意**不含**站点构建 preamble 里的 HTML 专属部分：
/// - `renderer="svg"` 的 show rule——仅对 `target() == "html"` 生效，对 PDF 无意义；
/// - 公式编号的 `(..) => none` 抑制——那是为绕开 typst 0.15 `html export`
///   的编号字形 bug（typst#5512）、由 HTML 后处理注入 eq-num 的妥协。PDF 走
///   原生 paged 渲染，编号与引用均正常，**必须用真实编号**：若沿用抑制，
///   块级公式会丢编号、且 `@ref` 引用会报 "cannot reference equation without
///   numbering"（这些文章源大量用 `@eq:xxx` 交叉引用）。
fn pdf_preamble(config: &Config, root: &Path) -> String {
    let mut s = String::new();
    let lang = typst_lang(&config.site.language);
    s.push_str(&format!("#set text(lang: \"{lang}\")\n"));

    // 公式编号：站点 eq-num 显示的 "公式 (N)" 前缀是 HTML 层注入的样式；
    // PDF 用原生 `(1)` 模式即可（引用渲染为 "式 (1)"，编号 `(1)` 与站点一致）。
    if config.build.math.number_equations {
        s.push_str("#set math.equation(numbering: \"(1)\")\n");
    }

    // 自动加载物理宏（规格书 §6.4）：若 assets/macros.typ 存在则注入 import
    let macros_path = root.join("assets").join("macros.typ");
    if macros_path.is_file() {
        s.push_str("#import \"/assets/macros.typ\": *\n");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_with(language: &str) -> Config {
        let mut c = Config::default();
        c.site.language = language.to_string();
        c
    }

    #[test]
    fn pdf_preamble_sets_language() {
        let p = pdf_preamble(&config_with("zh-CN"), Path::new("."));
        assert!(p.contains("#set text(lang: \"zh\")"), "preamble = {p:?}");
    }

    #[test]
    fn pdf_preamble_uses_real_equation_numbering() {
        // 站点构建为绕开 typst 0.15 html export 编号 bug 会注入
        // `numbering: (..) => none`；PDF 走原生 paged 渲染必须用真实编号，
        // 否则块级公式无编号、且 @ref 引用会报 "cannot reference equation
        // without numbering"。
        let mut c = config_with("en-US");
        c.build.math.number_equations = true;
        let p = pdf_preamble(&c, Path::new("."));
        assert!(
            p.contains("numbering: \"(1)\""),
            "PDF preamble 应启用真实编号: {p:?}"
        );
        assert!(
            !p.contains("(..) => none"),
            "PDF preamble 不应抑制编号: {p:?}"
        );
    }

    #[test]
    fn pdf_preamble_no_numbering_when_disabled() {
        let mut c = config_with("en-US");
        c.build.math.number_equations = false;
        let p = pdf_preamble(&c, Path::new("."));
        assert!(!p.contains("numbering"), "preamble = {p:?}");
    }

    #[test]
    fn pdf_preamble_imports_macros_when_present() {
        let dir = tempfile::tempdir().unwrap();
        let assets = dir.path().join("assets");
        std::fs::create_dir_all(&assets).unwrap();
        std::fs::write(assets.join("macros.typ"), "#let foo = 1\n").unwrap();
        let p = pdf_preamble(&Config::default(), dir.path());
        assert!(p.contains("#import \"/assets/macros.typ\": *"));
    }

    #[test]
    fn pdf_preamble_skips_macros_when_absent() {
        let dir = tempfile::tempdir().unwrap();
        let p = pdf_preamble(&Config::default(), dir.path());
        assert!(!p.contains("#import"));
    }
}
