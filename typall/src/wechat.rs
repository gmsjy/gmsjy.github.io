//! 微信目标 renderer：把 IR 正文变换为可直接粘贴到公众号编辑器的富文本 HTML 片段。
//!
//! 管线只有一步：`wechat_body` — DOM 遍历，注入内联样式，SVG/公式栅格化为
//! PNG data URI。图片/公式以 data URI 留在产物里（复制按钮流程下，公众号
//! 编辑器粘贴时自动转存图片），本目标**完全不触网**。

use anyhow::bail;
use base64::Engine as _;
use scraper::Node;

use crate::ir::CompiledDoc;
use crate::publish::{PublishCtx, Publisher, RenderedDoc};

pub struct WechatPublisher;

impl Publisher for WechatPublisher {
    fn name(&self) -> &'static str {
        "wechat"
    }

    fn rel_path(&self, doc: &CompiledDoc) -> String {
        format!("{}.html", doc.slug)
    }

    fn render(
        &self,
        doc: &CompiledDoc,
        body_html: &str,
        ctx: &PublishCtx<'_>,
    ) -> anyhow::Result<RenderedDoc> {
        if ctx.config.build.math.renderer != "svg" {
            bail!(
                "微信目标要求 [build.math] renderer = \"svg\"（MathML 公式无法转为位图）；\
                 在 typall.toml 设置后重新构建"
            );
        }
        // 内联样式 + 公式/插图栅格化
        let content = wechat_body(body_html)?;
        Ok(RenderedDoc {
            rel_path: self.rel_path(doc),
            content,
        })
    }
}

type TreeNode<'a> = ego_tree::NodeRef<'a, scraper::node::Node>;

/// 正文 → 内联样式 HTML 片段。
fn wechat_body(body_html: &str) -> anyhow::Result<String> {
    let dom = scraper::Html::parse_fragment(body_html);
    let mut out = String::with_capacity(body_html.len() * 2);
    for child in dom.tree.root().children() {
        out.push_str(&render_node(child)?);
    }
    Ok(out)
}

/// 渲染单个节点为内联样式 HTML。
fn render_node(node: TreeNode<'_>) -> anyhow::Result<String> {
    let Some(el) = node.value().as_element() else {
        return Ok(match node.value() {
            // scraper 已解码实体（&amp; → &），输出时需重新转义，
            // 否则裸 & / < 进入最终 HTML（公众号服务端校验不可控）。
            Node::Text(t) => text_escape(t),
            _ => String::new(),
        });
    };
    let name = el.name();

    // 公式 / 插图：SVG → PNG 位图（公众号不支持 `<svg>`，会静默剥离
    // `xlink:href`/`id` 使 symbol/use 引用失效，公式渲染成空白）。
    // 因此统一栅格化为 PNG data URI。
    if name == "svg" {
        // 裸 svg（cetz 插图等，不在 data-equation 包裹层内）= 居中插图。
        return svg_img(node, true);
    }
    if name == "div" && el.attr("data-equation") == Some("block") {
        // block 公式：svg → PNG + 编号 span（编号需保留，栅格化不能吞掉）。
        return svg_img_with_eq_num(node);
    }
    if name == "span" && el.attr("data-equation") == Some("inline") {
        // inline 公式：svg → PNG，行内垂直居中。
        return svg_img(node, false);
    }
    // 公式编号 span（HTML 后处理层注入；公众号不加载 theme.css，
    // 必须 inline style 才能在编辑器里正确显示在公式右上角）。
    if name == "span"
        && el.attr("class")
            .is_some_and(|c| c.split_whitespace().any(|x| x == "eq-num"))
    {
        let inner = render_children(node)?;
        return Ok(format!(
            "<span class=\"eq-num\" style=\"position:absolute;right:0;top:50%;transform:translateY(-50%);font-size:0.95em;color:#666;white-space:nowrap;\">{inner}</span>"
        ));
    }

    // 脚本/样式内容不是正文：公众号产物里输出会把源码当可见文字展示
    // （与 markdown 目标 inline() 的跳过行为对齐）。
    if name == "script" || name == "style" {
        return Ok(String::new());
    }

    // 常规元素：注入内联样式
    let style: &str = match name {
        "h1" => "font-size:22px;font-weight:bold;margin:30px 0 14px;color:#1a1a1a;",
        "h2" => "font-size:20px;font-weight:bold;margin:28px 0 14px;color:#1a1a1a;",
        "h3" => "font-size:17px;font-weight:bold;margin:22px 0 10px;color:#1a1a1a;",
        "h4" | "h5" | "h6" => "font-size:15px;font-weight:bold;margin:18px 0 8px;color:#1a1a1a;",
        "p" => "font-size:15px;line-height:1.75;margin:10px 0;letter-spacing:0.5px;color:#3f3f3f;text-align:justify;",
        "blockquote" => "border-left:3px solid #d0d0d0;padding:4px 14px;margin:14px 0;color:#888;background:#f7f7f7;font-size:14px;line-height:1.7;",
        "pre" => "background:#f6f8fa;padding:12px;border-radius:4px;overflow-x:auto;margin:14px 0;font-family:Menlo,Consolas,monospace;font-size:13px;line-height:1.5;",
        "code" => "background:#f0f0f0;padding:2px 4px;border-radius:3px;font-family:Menlo,Consolas,monospace;font-size:13px;",
        "table" => "width:100%;border-collapse:collapse;margin:14px 0;font-size:14px;",
        // tr/thead/tbody 不带样式但必须保留标签：此前落入兜底分支被剥掉，
        // 公众号编辑器按行渲染失败，整张表格塌成一行。
        "tr" | "thead" | "tbody" => "",
        "td" | "th" => "border:1px solid #e0e0e0;padding:6px 10px;text-align:left;",
        "section" => "",
        "mark" => "background:#fff3b0;color:inherit;padding:0 2px;",
        "ul" | "ol" => "padding-left:24px;margin:10px 0;",
        "li" => "font-size:15px;line-height:1.75;margin:4px 0;color:#3f3f3f;",
        "strong" => "font-weight:bold;color:#1a1a1a;",
        "em" | "i" => "font-style:italic;",
        "del" | "s" => "text-decoration:line-through;",
        "u" => "text-decoration:underline;",
        "a" => "color:#576b95;text-decoration:none;",
        "img" => "max-width:100%;",
        "figure" => "margin:14px 0;text-align:center;",
        "figcaption" => "font-size:13px;color:#999;margin-top:6px;",
        "hr" => "border:none;border-top:1px solid #e0e0e0;margin:20px 0;",
        _ => return render_children(node),
    };

    let style = if name == "code" && in_pre(node) {
        "font-family:Menlo,Consolas,monospace;font-size:13px;"
    } else {
        style
    };

    // 站内锚点链接降级纯文本；外链保留。
    if name == "a"
        && let Some(href) = el.attr("href")
        && href.starts_with('#')
    {
        return render_children(node);
    }

    let mut extra = String::new();
    if name == "a"
        && let Some(href) = el.attr("href")
    {
        extra.push_str(&format!(" href=\"{}\"", attr_escape(href)));
    }

    // 空 style（tr/thead/tbody/section 等仅保结构的标签）不输出 style 属性
    let style_attr = if style.is_empty() {
        String::new()
    } else {
        format!(" style=\"{style}\"")
    };

    Ok(format!(
        "<{name}{style_attr}{extra}>{}</{name}>",
        render_children(node)?
    ))
}

fn attr_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// HTML 文本节点转义（scraper 解码过实体，输出时重新转义）。
fn text_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn in_pre(node: TreeNode<'_>) -> bool {
    let mut cur = node.parent();
    while let Some(p) = cur {
        if p.value().as_element().is_some_and(|e| e.name() == "pre") {
            return true;
        }
        cur = p.parent();
    }
    false
}

fn render_children(node: TreeNode<'_>) -> anyhow::Result<String> {
    let mut out = String::new();
    for child in node.children() {
        out.push_str(&render_node(child)?);
    }
    Ok(out)
}

/// 位图放大倍数：typst 输出 pt 尺寸，2x 保证高分屏清晰。
const SCALE: f32 = 2.0;

/// 公众号正文图片显示宽度上限（px）。
const WECHAT_MAX_WIDTH: f32 = 578.0;

/// SVG 位图像素上限：宽高来自作者可控的 SVG `width`/`height`，
/// 病态尺寸（如 1e6×1e6）会直接按 w×h×4 字节分配位图导致 OOM。
/// 超限按报错处理（栅格化无意义），不做有损缩放。
const SVG_MAX_PIXELS: u64 = 16_000_000;

/// 公众号显示尺寸：超宽时宽高等比缩小。
///
/// 只 clamp 宽不缩高会把超宽公式/插图压扁（`max-width:100%` 只兜住极端，
/// 显式 height 仍会把比例钉死），必须两维同乘缩放系数。
fn display_size(w: f32, h: f32) -> (f32, f32) {
    let scale = (WECHAT_MAX_WIDTH / w).min(1.0);
    (w * scale, h * scale)
}

/// 渲染 `<svg>` 为 PNG data URI，返回（data uri, 显示宽 px, 显示高 px）。
///
/// 公众号编辑器会静默剥离 `<svg>` 的 `xlink:href`/`id`，使 typst 公式/
/// cetz 插图赖以渲染的 symbol/use 引用机制失效（公式变空白）。因此统一
/// 栅格化为 PNG 位图（resvg 纯 Rust 离线渲染，2x 高清），正文以 `<img>`
/// data URI 引用，粘贴时由编辑器自动转存图片。
fn svg_to_png(svg: &str) -> anyhow::Result<(String, f32, f32)> {
    let tree = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default())
        .map_err(|e| anyhow::anyhow!("SVG 解析失败: {e}"))?;
    let size = tree.size();
    let w = ((size.width() * SCALE).ceil() as u32).max(1);
    let h = ((size.height() * SCALE).ceil() as u32).max(1);
    // 位图内存 = w×h×4 字节，尺寸来自作者可控的 SVG 标签，必须设上限防 OOM
    if u64::from(w) * u64::from(h) > SVG_MAX_PIXELS {
        anyhow::bail!(
            "SVG 尺寸过大（{w}×{h} 像素，上限 {SVG_MAX_PIXELS}），拒绝栅格化：请检查 SVG 的 width/height"
        );
    }
    let mut pixmap = resvg::tiny_skia::Pixmap::new(w, h)
        .ok_or_else(|| anyhow::anyhow!("位图分配失败 {w}x{h}"))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(SCALE, SCALE),
        &mut pixmap.as_mut(),
    );
    let png = pixmap
        .encode_png()
        .map_err(|e| anyhow::anyhow!("PNG 编码失败: {e}"))?;
    Ok((
        format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(png)
        ),
        size.width(),
        size.height(),
    ))
}

/// svg（或其包裹层）→ PNG data URI 的 `<img>`。
///
/// `centered`：块级公式与 cetz 插图居中包裹；行内公式垂直居中。
/// 显示尺寸取 usvg 换算后的 px（typst 以 pt 标注 SVG 尺寸），位图按 2x 渲染。
fn svg_img(node: TreeNode<'_>, centered: bool) -> anyhow::Result<String> {
    // 包裹层节点（div/span）先找内部 <svg>；本身是 svg 则直接用。
    let svg_node = if node.value().as_element().is_some_and(|e| e.name() == "svg") {
        node
    } else {
        node.descendants()
            .find(|n| n.value().as_element().is_some_and(|e| e.name() == "svg"))
            .ok_or_else(|| anyhow::anyhow!("data-equation 包裹层内未找到 <svg>"))?
    };
    let svg_html = crate::publish::element_html(svg_node);
    let (data_uri, w, h) = svg_to_png(&svg_html)?;
    let (dw, dh) = display_size(w, h);
    let display = format!("width:{dw:.0}px;height:{dh:.0}px;max-width:100%;");
    if centered {
        Ok(format!(
            "<p style=\"text-align:center;margin:14px 0;\"><img src=\"{data_uri}\" style=\"{display}\" alt=\"公式或插图\"></p>"
        ))
    } else {
        Ok(format!(
            "<img src=\"{data_uri}\" style=\"{display}vertical-align:middle;\" alt=\"公式\">"
        ))
    }
}

/// block 公式（`<div data-equation="block">`）→ svg 栅格化 PNG + 编号 span。
///
/// 编号 span（`class="eq-num"`，含「公式 (N)」文本）是 HTML 后处理层注入的，
/// 栅格化只作用于内部 <svg>，编号必须原样保留并仍定位到公式右侧。
fn svg_img_with_eq_num(node: TreeNode<'_>) -> anyhow::Result<String> {
    let svg_node = node
        .descendants()
        .find(|n| n.value().as_element().is_some_and(|e| e.name() == "svg"))
        .ok_or_else(|| anyhow::anyhow!("block 公式 div 内未找到 <svg>"))?;
    let svg_html = crate::publish::element_html(svg_node);
    let (data_uri, w, h) = svg_to_png(&svg_html)?;
    let (dw, dh) = display_size(w, h);
    let display = format!("width:{dw:.0}px;height:{dh:.0}px;max-width:100%;");
    // 提取编号 span 的纯文本（「公式 (1)」），栅格化后单独保留。
    let eq_num = node
        .descendants()
        .find(|n| {
            n.value().as_element().is_some_and(|e| {
                e.attr("class").is_some_and(|c| c.split_whitespace().any(|x| x == "eq-num"))
            })
        })
        .map(|n| {
            n.descendants()
                .filter_map(|d| d.value().as_text())
                .map(|t| t.to_string())
                .collect::<String>()
                .trim()
                .to_string()
        })
        .unwrap_or_default();

    let img = format!("<img src=\"{data_uri}\" style=\"{display}\" alt=\"公式\">");
    let num = if eq_num.is_empty() {
        String::new()
    } else {
        format!(
            "<span class=\"eq-num\" style=\"position:absolute;right:0;top:50%;transform:translateY(-50%);font-size:0.95em;color:#666;white-space:nowrap;\">{eq_num}</span>"
        )
    };
    Ok(format!(
        "<div style=\"position:relative;text-align:center;margin:14px 0;\">{img}{num}</div>"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::content::DocumentMeta;
    use crate::ir::MathItem;
    use std::path::Path;

    fn ctx(_root: &Path) -> PublishCtx<'_> {
        PublishCtx {
            config: leak_config(),
        }
    }

    fn leak_config() -> &'static Config {
        Box::leak(Box::new(Config::default()))
    }

    fn leak_path(s: &str) -> &'static std::path::Path {
        Box::leak(Box::new(std::path::PathBuf::from(s)))
    }

    fn test_doc() -> CompiledDoc {
        CompiledDoc {
            slug: "posts/test".into(),
            meta: DocumentMeta {
                title: "T".into(),
                date: Some("2026-09-02".into()),
                ..Default::default()
            },
            body_html: String::new(),
            math_style: String::new(),
            math: Vec::<MathItem>::new(),
        }
    }

    const TINY_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\">\
        <rect width=\"10\" height=\"10\" fill=\"#e33\"/></svg>";

    // ── DOM 遍历 + 内联样式 + SVG→PNG ─────────────────────────────

    #[test]
    fn inline_svg_rasterizes_to_png() {
        let html = format!("<p>前 <span data-equation=\"inline\">{}</span> 后</p>", TINY_SVG);
        let out = wechat_body(&html).unwrap();
        // 公众号不支持 <svg>（剥离 xlink:href/id 使公式空白）→ 栅格化为 PNG
        assert!(!out.contains("<svg"), "svg 不应原样保留: {out}");
        assert!(out.contains("data:image/png;base64,"), "应栅格化为 PNG: {out}");
        assert!(out.contains("vertical-align:middle"), "行内公式应垂直居中");
    }

    #[test]
    fn requires_svg_renderer() {
        let doc = test_doc();
        let r = WechatPublisher.render(&doc, "<p>x</p>", &ctx(leak_path("/")));
        let msg = match r {
            Err(e) => e.to_string(),
            Ok(_) => panic!("MathML 渲染器下应当拒绝微信目标"),
        };
        assert!(msg.contains("renderer = \"svg\""));
    }

    #[test]
    fn inline_styles_and_anchor_stripping() {
        let html = "<h2>标题</h2>\
            <p>正文 <strong>强调</strong> <a href=\"https://a.b\">外链</a> \
            <a href=\"#loc-1\">式 1</a></p>";
        let out = wechat_body(html).unwrap();
        assert!(out.contains("<h2 style=\"font-size:20px"));
        assert!(out.contains("<strong style=\"font-weight:bold"));
        assert!(out.contains(
            "<a style=\"color:#576b95;text-decoration:none;\" href=\"https://a.b\">外链</a>"
        ));
        // 锚点链接（公式引用 #loc-1）→ 纯文本降级；外链保留。
        assert!(!out.contains("href=\"#loc-1\""));
        assert!(out.contains("式 1"));
    }

    #[test]
    fn equations_and_cetz_rasterize_to_png() {
        let html = format!(
            "<p>前 <span data-equation=\"inline\">{}</span> 后</p>\
             <div data-equation=\"block\" id=\"loc-1\"><span class=\"eq-num\">公式 (1)</span>{}</div>\
             <div>{}</div>",
            TINY_SVG, TINY_SVG, TINY_SVG
        );
        let out = wechat_body(&html).unwrap();
        // 3 个 svg 全部栅格化为 PNG，不保留 <svg>
        assert_eq!(out.matches("<svg").count(), 0);
        assert_eq!(out.matches("data:image/png;base64,").count(), 3);
        // inline 公式垂直居中
        assert!(out.contains("vertical-align:middle"));
        // block 公式 + cetz 居中
        assert_eq!(out.matches("text-align:center").count(), 2);
        // block 公式编号 span 保留（不因栅格化丢失）
        assert!(out.contains("公式 (1)"));
    }

    #[test]
    fn code_block_and_table_styled() {
        let html = "<pre><code>fn f() {}</code></pre>\
            <table><tr><td><strong>a</strong></td><td>b</td></tr></table>";
        let out = wechat_body(html).unwrap();
        assert!(out.contains("background:#f6f8fa"));
        assert!(out.contains("border-collapse:collapse"));
        assert!(out.contains("border:1px solid #e0e0e0"));
    }

    // ── P2 修复的回归测试 ──────────────────────────────────────────

    /// 斜体 / 删除线 / 下划线：Typst 的 *斜体* 输出 <em>，此前无样式规则被透明展开。
    #[test]
    fn italic_del_underline_styled() {
        let html = "<p><em>斜</em> <del>删</del> <u>下划</u> <i>斜2</i></p>";
        let out = wechat_body(html).unwrap();
        assert!(out.contains("<em style=\"font-style:italic;\">"));
        assert!(out.contains("<i style=\"font-style:italic;\">"));
        assert!(out.contains("<del style=\"text-decoration:line-through;\">"));
        assert!(out.contains("<u style=\"text-decoration:underline;\">"));
    }

    /// 文本节点实体重新转义：正文里的裸 & 和 < 不应漏进最终 HTML。
    #[test]
    fn text_nodes_reescaped() {
        let html = "<p>a &amp; b &lt; c</p>";
        let out = wechat_body(html).unwrap();
        assert!(out.contains("a &amp; b &lt; c"), "应重新转义: {out}");
        assert!(!out.contains("a & b"));
    }

    // ── SVG → PNG 栅格化（公众号不支持 <svg> 的修复） ──

    /// 简单 SVG 栅格化为 PNG data URI，返回显示尺寸（pt → px）。
    #[test]
    fn svg_rasterizes_to_png_data_uri() {
        let (uri, w, h) = svg_to_png(TINY_SVG).unwrap();
        assert!(uri.starts_with("data:image/png;base64,"), "应为 PNG data URI: {uri}");
        assert!(uri.len() > "data:image/png;base64,".len(), "base64 内容非空");
        assert!(w > 0.0 && h > 0.0, "应返回正显示尺寸: {w}x{h}");
    }

    /// block 公式栅格化后仍保留编号 span「公式 (N)」。
    #[test]
    fn block_equation_keeps_eq_num_after_rasterize() {
        let html = format!(
            "<div data-equation=\"block\"><span class=\"eq-num\">公式 (3)</span>{}</div>",
            TINY_SVG
        );
        let out = wechat_body(&html).unwrap();
        assert!(!out.contains("<svg"), "svg 应栅格化: {out}");
        assert!(out.contains("data:image/png;base64,"), "应含 PNG: {out}");
        assert!(out.contains("公式 (3)"), "编号应保留: {out}");
        assert!(out.contains("eq-num"), "编号 span 应保留: {out}");
    }

    #[test]
    fn display_size_scales_proportionally() {
        // 未超宽：原样
        assert_eq!(display_size(400.0, 100.0), (400.0, 100.0));
        // 超宽：宽高等比缩小，不把图压扁
        let (w, h) = display_size(1156.0, 200.0);
        assert!((w - 578.0).abs() < 0.01 && (h - 100.0).abs() < 0.01, "got {w}x{h}");
    }

    #[test]
    fn table_rows_keep_tags_without_style() {
        let html = "<table><tr><td>a</td><td>b</td></tr></table>";
        let out = wechat_body(html).unwrap();
        assert!(out.contains("<tr>"), "tr 标签应保留: {out}");
        assert!(!out.contains("<tr style="), "空 style 不应输出属性: {out}");
        assert!(out.contains("<td style="), "td 应带内联样式: {out}");
    }
}
