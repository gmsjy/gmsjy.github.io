//! 编译入口：扫描 → 缓存判定 → Typst 编译 → HTML 提取 → 公式编号注入。


use std::path::{Path, PathBuf};
use std::sync::Arc;

use rayon::prelude::*;
use typst::diag::{SourceDiagnostic, Warned};
use typst_html::{html, HtmlDocument, HtmlOptions};

use crate::config::Config;
use crate::content::{self, DocumentMeta};
use crate::ir::{CompiledDoc, extract_math_items};
use crate::world::{SharedAssets, TypallWorld};
use crate::cache::CompileCache;
use crate::cache::content_hash;


/// 扫描 + 并行编译全部文章与独立页面（`build` 与 `publish` 共用的编译入口）。
pub(crate) struct CompiledDocuments {
    pub posts: Vec<CompiledDoc>,
    pub pages: Vec<CompiledDoc>,
    /// 源文件清单（供调用方做缓存孤儿清理）。
    pub post_files: Vec<PathBuf>,
    pub page_files: Vec<PathBuf>,
    pub cache: CompileCache,
}

pub(crate) fn compile_documents(
    root: &Path,
    config: &Config,
) -> anyhow::Result<CompiledDocuments> {
    let shared = Arc::new(SharedAssets::load(root)?);
    let post_files = content::scan_typ_files(&root.join("posts"));
    let page_files = content::scan_typ_files(&root.join("pages"));
    ensure_project_root(root)?;

    let preamble = math_preamble(config, root);
    // 编号前缀由 HTML 后处理注入（不在 preamble 内），必须纳入缓存上下文。
    let context = format!("{preamble}[equation-prefix:{}]", config.build.math.equation_prefix);
    let cache = CompileCache::new(root, &context);
    let compile_one = |path: &PathBuf| {
        compile_doc(root, path, shared.clone(), &preamble, &config.build.math.renderer, config, Some(&cache))
    };

    // 并行编译，收集所有错误一起报告（避免短路吞掉后续文章的错误）
    let post_results: Vec<anyhow::Result<CompiledDoc>> = post_files.par_iter().map(compile_one).collect();
    let page_results: Vec<anyhow::Result<CompiledDoc>> = page_files.par_iter().map(compile_one).collect();

    let mut posts: Vec<CompiledDoc> = Vec::new();
    let mut pages: Vec<CompiledDoc> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for r in post_results {
        match r {
            Ok(d) => posts.push(d),
            Err(e) => errors.push(format!("{e:#}")),
        }
    }
    for r in page_results {
        match r {
            Ok(d) => pages.push(d),
            Err(e) => errors.push(format!("{e:#}")),
        }
    }
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("❌ {e}");
        }
        anyhow::bail!("{} 篇文章/页面编译失败", errors.len());
    }
    Ok(CompiledDocuments { posts, pages, post_files, page_files, cache })
}

/// 草稿与定时发布过滤（posts / pages 共用，站点构建与平台发布口径一致）：
/// `include_drafts` 为假时剔除 `draft = true` 以及日期在未来（严格大于今天）的文档。
pub(crate) fn filter_unpublished(docs: &mut Vec<CompiledDoc>, include_drafts: bool) {
    if include_drafts {
        return;
    }
    docs.retain(|p| !p.meta.draft);
    // 定时发布：未来日期的文章暂不发布
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    docs.retain(|p| match &p.meta.date {
        Some(d) if !d.is_empty() => d.as_str() <= today.as_str(),
        _ => true,
    });
}


/// 返回空 `math_style` 避免冗余注入。
pub(crate) fn compile_doc(
    root: &Path,
    path: &Path,
    shared: Arc<SharedAssets>,
    preamble: &str,
    renderer: &str,
    config: &Config,
    cache: Option<&CompileCache>,
) -> anyhow::Result<CompiledDoc> {
    let rel = content::rel_path(root, path);
    let source_text = std::fs::read_to_string(path)?;
    let meta = DocumentMeta::from_map(&content::extract_meta(&source_text));
    let slug = content::slug_of(&rel);

    // 增量：命中缓存则跳过昂贵的 Typst 编译，直接复用正文 HTML 与 MathML 样式。
    let source_hash = content_hash(&source_text);
    if let Some((body_html, math_style)) = cache.and_then(|c| c.load(&rel, &source_hash)) {
        let math_style = if renderer == "svg" {
            String::new()
        } else {
            math_style
        };
        return Ok(CompiledDoc {
            slug,
            meta,
            body_html,
            math_style,
            // 公式源在语法层提取（<1ms），缓存命中也照常填充，无需入缓存。
            math: extract_math_items(&source_text),
        });
    }

    let world = TypallWorld::new(root, &rel, shared, preamble)?;
    let Warned { output, warnings } = typst::compile::<HtmlDocument>(&world);

    for w in &warnings {
        // 跳过无源位置的引擎级警告（如 html export 提示），避免批量构建刷屏。
        if w.span.is_detached() {
            continue;
        }
        eprintln!("警告 {}", format_diag(&world, w));
    }

    let doc = output.map_err(|errs| {
        let msgs: Vec<String> = errs
            .iter()
            .map(|e| format!("  - {}", format_diag(&world, e)))
            .collect();
        anyhow::anyhow!("编译失败 [{rel}]:\n{}", msgs.join("\n"))
    })?;

    let html_str = html(&doc, &HtmlOptions { pretty: true }).map_err(|errs| {
        let msgs: Vec<String> = errs
            .iter()
            .map(|e| format!("  - {}", format_diag(&world, e)))
            .collect();
        anyhow::anyhow!("HTML 编码失败 [{rel}]:\n{}", msgs.join("\n"))
    })?;

    let (body_html, math_style) = split_body_and_style(&html_str);
    // HTML 后处理：注入块级公式编号（typst 0.15 svg renderer 不渲染原生
    // numbering，typst#5512）。编号在 HTML 层生成，不污染 typst→HTML 主干。
    let body_html = inject_equation_numbers(
        &body_html,
        config.build.math.number_equations,
        &config.build.math.equation_prefix,
    );
    let math_style = if renderer == "svg" {
        String::new()
    } else {
        math_style
    };

    // 写缓存（记录依赖文件，供下次失效判断）
    if let Some(cache) = cache {
        let deps = world.deps();
        cache.store(&rel, &source_hash, &deps, &body_html, &math_style)?;
    }

    Ok(CompiledDoc {
        slug,
        meta,
        body_html,
        math_style,
        math: extract_math_items(&source_text),
    })
}

/// 格式化单条诊断，附带 `文件:行:列` 位置（行列为 1-based）。
pub(crate) fn format_diag(world: &TypallWorld, diag: &SourceDiagnostic) -> String {
    match world.location(diag) {
        Some((path, line, col)) => format!("{path}:{line}:{col}: {}", diag.message),
        // Detached 或无位置的诊断：退化为纯消息，无路径前缀。
        None => format!("?: {}", diag.message),
    }
}

/// 从 Typst 生成的完整 HTML 中提取 body 内容与 MathML 样式块。
pub(crate) fn split_body_and_style(html_str: &str) -> (String, String) {
    use scraper::{Html, Selector};

    let doc = Html::parse_document(html_str);
    let body_sel = Selector::parse("body").unwrap();
    let style_sel = Selector::parse("style").unwrap();

    let body = doc
        .select(&body_sel)
        .next()
        .map(|b| b.inner_html())
        .unwrap_or_default();
    let style: String = doc.select(&style_sel).map(|s| s.html()).collect();
    (body, style)
}

/// 给 body HTML 中的块级公式按文档顺序注入编号 span。
///
/// typst 0.15 svg renderer 不渲染原生 numbering（typst/typst#5512：编号
/// 字符处理异常，不渲染/豆腐块）。此处在 HTML 后处理层注入
/// `<span class="eq-num">(prefix N)</span>`——字符走 text 字体，站点与
/// 平台渲染均稳定。架构上属于"基于 html 调整"，不污染 typst→HTML 主干。
///
/// block 公式容器 `<div data-equation="block">` 内只有 svg（typst show rule
/// 保证），用第一个 `</div>` 定位闭合，在闭合前插入编号 span。
pub(crate) fn inject_equation_numbers(body_html: &str, number_equations: bool, prefix: &str) -> String {
    if !number_equations || !body_html.contains("data-equation=\"block\"") {
        return body_html.to_string();
    }
    let marker = "<div data-equation=\"block\"";
    let mut out = String::with_capacity(body_html.len() + 256);
    let mut rest = body_html;
    let mut n = 0u32;
    while let Some(start) = rest.find(marker) {
        // 开标签结束 >
        let Some(tag_end_rel) = rest[start..].find('>') else { break };
        let tag_end = start + tag_end_rel;
        // block 公式 div 内只有 <svg>…</svg>，第一个 </div> 即闭合
        let inner = &rest[tag_end + 1..];
        let Some(close_rel) = inner.find("</div>") else { break };
        let close = tag_end + 1 + close_rel;
        n += 1;
        let label = if prefix.is_empty() {
            format!("<span class=\"eq-num\">({n})</span>")
        } else {
            format!("<span class=\"eq-num\">{prefix} ({n})</span>")
        };
        out.push_str(&rest[..close]); // <div …><svg>…</svg>
        out.push_str(&label);
        out.push_str("</div>");
        rest = &rest[close + 6..]; // 跳过 </div>
    }
    out.push_str(rest);
    out
}


/// 根据配置生成注入到文章开头的 Typst 代码（语言、公式编号、物理宏等）。
pub(crate) fn math_preamble(config: &Config, root: &Path) -> String {
    let mut s = String::new();

    // 语言（影响引用 supplement、日期等本地化显示）
    let lang = typst_lang(&config.site.language);
    s.push_str(&format!("#set text(lang: \"{lang}\")\n"));

    // 公式编号：typst 0.15 html export 在 renderer=svg 下，原生 numbering
    // 字符处理异常（typst/typst#5512：编号不渲染/豆腐块）。用 (..) => none
    // 让 typst 不画编号（counter 仍步进供 @ref 引用），编号改由 HTML 后处理
    // 层按 body.html 里 block 公式出现顺序注入 eq-num span——字符走 text
    // 字体，站点与平台渲染均稳定。架构上属于"基于 html 调整"，不污染
    // typst→HTML 主干。
    if config.build.math.number_equations {
        s.push_str("#set math.equation(numbering: (..) => none)\n");
    }

    // 公式渲染策略：svg → 注入 show rule，公式走 html.frame 布局输出内联 SVG
    // （typst_svg 矢量渲染，像素级一致；mathml 为默认，浏览器原生 MathML）。
    // 块级公式直接 frame（display:block）；行内公式需 box 包裹保持 inline，
    // 否则会被 typst-html 的 FrameElem 处理强制 block 化导致文本断行。
    // 外层 html.elem 加 data-equation 标记：Markdown 等非站点目标需要把
    // 正文中的公式渲染物按序配对到 IR 公式源（cetz 插图也是 <svg>，无标记
    // 则无法区分）。包裹层无样式，站点排版行为不变。
    //
    // ⚠ 关键：show rule 必须用 `context { if target() == "html" }` 守卫。
    // `html.frame(cetz.canvas(...))` 内部 target 是 "paged"（html.frame 用
    // Typst 布局引擎渲染 SVG），若公式 show rule 在此处也套 html.elem +
    // html.frame，会导致 cetz 的 `content($P$)` 等文字被吞掉（defs/symbol/use
    // 全丢），图像变成空白。详见 typst 论坛 4139 的 state/target 方案。
    if config.build.math.renderer == "svg" {
        s.push_str(
            "#show math.equation.where(block: true): it => context { if target() == \"html\" { html.elem(\"div\", attrs: (\"data-equation\": \"block\"), html.frame(it)) } else { it } }\n\
             #show math.equation.where(block: false): it => context { if target() == \"html\" { html.elem(\"span\", attrs: (\"data-equation\": \"inline\"), box(html.frame(it))) } else { it } }\n",
        );
    }

    // 自动加载物理宏（规格书 §6.4）：若 assets/macros.typ 存在则注入 import
    let macros_path = root.join("assets").join("macros.typ");
    if macros_path.is_file() {
        s.push_str("#import \"/assets/macros.typ\": *\n");
    }

    s
}

/// 把 `zh-CN` / `en-US` 映射为 Typst 的 ISO 语言码。
pub(crate) fn typst_lang(lang: &str) -> &str {
    if lang.is_empty() {
        "en"
    } else {
        lang.split('-').next().unwrap_or("en")
    }
}

/// 语法检查：编译所有 `.typ` 文件（不生成输出）。
/// `strict` 为真时，若存在构建产物则额外做死链检查。
/// 防呆：配置缺失时会静默回退默认值（见 `config::load`），在错误目录启动会
/// 得到一个「0 篇文章」的空结果且毫无报错——比失败更难排查。既无
/// typall.toml 也无 posts/ 的目录几乎必然不是项目根，直接给出人话错误。
pub(crate) fn ensure_project_root(root: &Path) -> anyhow::Result<()> {
    if !root.join("typall.toml").exists() && !root.join("posts").exists() {
        anyhow::bail!(
            "在 {} 未找到 typall.toml 或 posts/——请先 cd 到项目根目录（含 typall.toml 与 posts/ 的目录），或用 --dir 指定项目根",
            root.display()
        );
    }
    Ok(())
}
