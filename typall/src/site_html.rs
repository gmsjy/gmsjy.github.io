//! 站点 HTML 目标层：[`crate::ir::CompiledDoc`] 的第一个消费者。
//!
//! 这里集中了**只属于站点 HTML** 的正文变换（扩展余地核查 · 阶段 0 移位）：
//! 未来新增发布目标（Markdown / 公众号 / 知乎…）时各自实现变换，
//! 不得复用/污染本模块的逻辑，编译管线也与此无关。
//!
//! 职责：
//! 1. heading 锚点注入 + TOC 提取；
//! 2. 代码高亮颜色变量化（主题可覆盖 `--tok-*`）；
//! 3. 图片懒加载 + alt 兜底。

use crate::theme;

/// 正文 HTML 后处理：
///
/// 1. **TOC 锚点**：给 `<h2>/<h3>` 注入 `id`（slug 化标题），返回目录 HTML（嵌套列表）；
/// 2. **代码高亮定制**：把 Typst 输出的内联色 `style="color: #hex"` 改写为
///    `style="color: var(--tok-<hex>, #hex)"` —— 默认色不变（fallback），
///    用户可在自定义主题 CSS 中覆盖 `--tok-<hex>` 变量换配色。
///
/// 返回 `(处理后的正文, TOC HTML)`。
pub(crate) fn postprocess_body(body_html: &str) -> (String, String) {
    // --- 1. heading 锚点 + TOC ---
    let mut headings: Vec<(u8, String, String)> = Vec::new(); // (level, id, text)
    let mut out = String::with_capacity(body_html.len());
    let mut rest = body_html;
    while let Some(start) = rest.find("<h") {
        // 定位 <h2 / <h3 开标签
        let Some(tag_end) = rest[start..].find('>') else { break };
        let tag = &rest[start..start + tag_end];
        let level = tag[2..3].parse::<u8>().unwrap_or(0);
        if !(2..=3).contains(&level) {
            out.push_str(&rest[..start + tag_end + 1]);
            rest = &rest[start + tag_end + 1..];
            continue;
        }
        // 提取标题文本（到 </hN> 结束）
        let closer = format!("</h{level}>");
        let Some(text_end) = rest[start + tag_end + 1..].find(&closer) else { break };
        let text_start = start + tag_end + 1;
        let text_end = text_start + text_end;
        let raw = rest[text_start..text_end].trim();
        // 锚点与 TOC 基于纯文本：标题里的 *强调* 会输出 <em>、行内公式输出
        // 整段 MathML，直接 slugify/展示会把标签残骸吞进 id（如 `-em-斜率-em-`）。
        let text = strip_html(raw);
        let id = slugify(&text);

        // 写入开标签（带 id）+ 原始标题内容 + 闭标签（正文展示保留原排版）
        out.push_str(&rest[..start]);
        out.push_str(&format!("<h{level} id=\"{}\">", theme::escape(&id)));
        out.push_str(&rest[text_start..text_end]);
        out.push_str(&closer);
        headings.push((level, id, text));
        rest = &rest[text_end + closer.len()..];
    }
    out.push_str(rest);

    // --- 2. 代码高亮颜色变量化 ---
    let mut out2 = String::with_capacity(out.len() + 64);
    let mut rest = out.as_str();
    while let Some(start) = rest.find("style=\"color: #") {
        out2.push_str(&rest[..start]);
        let after = &rest[start + "style=\"color: #".len()..];
        let hex_len = after.chars().take(6).take_while(|c| c.is_ascii_hexdigit()).count();
        let hex = &after[..hex_len];
        out2.push_str(&format!("style=\"color: var(--tok-{hex}, #{hex})\""));
        rest = &after[hex_len..];
    }
    out2.push_str(rest);

    // --- 2.5 引用提升：typst 0.15 默认 HTML 输出不识别 quote 元素，把 `> ` 块
    // 当文本渲染成 `<p>&gt; ...</p>`。这里把段首为 `&gt; ` 的段落提升为
    // `<blockquote><p>...</p></blockquote>`，让主题的编辑风引用样式（墨绿边条
    // + 大号装饰引号）真正生效。算法：按 `<p>...</p>` 切片；对内部文本以
    // `&gt; `（或仅 `&gt;`）开头的段落，连续多段合并到同一个 blockquote。
    let mut out_quote = String::with_capacity(out2.len() + 64);
    let mut rest = out2.as_str();
    loop {
        let p_start = match rest.find("<p>") {
            Some(i) => i,
            None => {
                out_quote.push_str(rest);
                break;
            }
        };
        out_quote.push_str(&rest[..p_start]);
        rest = &rest[p_start..];
        // 处理一个 quote 块或单个普通段；循环回到顶层继续。
        let p_open = "<p>";
        let p_close = "</p>";
        debug_assert!(rest.starts_with(p_open));
        // 探查本段是否 `&gt;` 起头（用 strip_prefix 而非字节切片，避免中文
        // 多字节字符切在非 char 边界 panic）。
        let body_after_open = rest.strip_prefix(p_open).unwrap_or("");
        let is_quote = body_after_open.starts_with("&gt;");
        if is_quote {
            out_quote.push_str("<blockquote>");
            loop {
                // 找本段 </p>
                let close_pos = match rest.find(p_close) {
                    Some(i) => i,
                    None => {
                        // 异常：未闭合，原样落表后退出
                        out_quote.push_str(rest);
                        break;
                    }
                };
                let para_full = &rest[..close_pos + p_close.len()];
                // 剥开标签取内文
                let inner = &para_full[p_open.len()..para_full.len() - p_close.len()];
                // 剥 `&gt;` 后所有前导空格（typst `> ` 与 `>  内容` 都兼容）
                let stripped = inner
                    .strip_prefix("&gt;")
                    .map(|s| s.trim_start_matches(' '))
                    .unwrap_or(inner);
                out_quote.push_str("<p>");
                out_quote.push_str(stripped);
                out_quote.push_str(p_close);
                rest = &rest[close_pos + p_close.len()..];
                // 跳过段间空白（typst 输出段落间通常无空白）
                let spaces = rest.chars().take_while(|c| c.is_whitespace()).count();
                rest = &rest[spaces..];
                // 探查下一段是否仍为 quote
                let next_after_open = rest.strip_prefix(p_open).unwrap_or("");
                let next_is_quote = next_after_open.starts_with("&gt;");
                if !next_is_quote || !rest.starts_with(p_open) {
                    break;
                }
            }
            out_quote.push_str("</blockquote>");
        } else {
            // 普通段落：原样拷贝直到 </p>
            let close_pos = match rest.find(p_close) {
                Some(i) => i,
                None => {
                    out_quote.push_str(rest);
                    break;
                }
            };
            out_quote.push_str(&rest[..close_pos + p_close.len()]);
            rest = &rest[close_pos + p_close.len()..];
        }
    }

    // --- 3. 图片懒加载 + alt 兜底（首图保留即时加载以利于 LCP；alt 缺失用文件名）---
    let mut out3 = String::with_capacity(out_quote.len() + 64);
    let mut rest = out_quote.as_str();
    let mut img_index = 0usize;
    while let Some(start) = rest.find("<img") {
        out3.push_str(&rest[..start]);
        // 找开标签结束（跳过引号内的字符，兼容 alt="a > b"）
        let mut tag_end = 0usize;
        let mut in_quote = false;
        for (i, c) in rest[start..].char_indices() {
            if c == '"' {
                in_quote = !in_quote;
            }
            if c == '>' && !in_quote {
                tag_end = i;
                break;
            }
        }
        if tag_end == 0 {
            // 未闭合：原样保留并终止
            out3.push_str(&rest[start..]);
            rest = "";
            break;
        }
        let tag = &rest[start..start + tag_end + 1];
        img_index += 1;
        let mut new_tag = tag.to_string();

        if img_index > 1 && !new_tag.contains("loading=") {
            new_tag = insert_attr(&new_tag, r#"loading="lazy""#);
        }
        if !new_tag.contains("alt=") {
            let alt = extract_alt_from_src(&new_tag);
            new_tag = insert_attr(&new_tag, &format!("alt=\"{}\"", theme::escape(&alt)));
        }
        out3.push_str(&new_tag);
        rest = &rest[start + tag_end + 1..];
    }
    out3.push_str(rest);

    // --- 组装 TOC ---
    // 只产出 `<li>` 列表：`<ul>`/`<nav>` 外壳由主题 `partials/toc.html` 决定
    // （规格书 §4.3，内置默认见 `theme::DEFAULT_TOC`）。
    let mut items = String::new();
    for (level, id, text) in &headings {
        let cls = if *level == 2 { "toc-l2" } else { "toc-l3" };
        items.push_str(&format!(
            r##"<li class="{cls}"><a href="#{id}">{}</a></li>"##,
            theme::escape(text)
        ));
    }

    (out3, items)
}

/// 从标题片段中剥离 HTML 标签并解码实体，仅保留可见文本。
///
/// 标题里的 `*强调*` 会被 Typst 输出为 `<em>…</em>`、行内公式输出为整段
/// MathML；锚点 slug 与 TOC 文案都基于还原后的纯文本，避免标签残骸混入。
/// 也供 publish 层生成摘要复用（`&nbsp;` 解码为普通空格）。
pub(crate) fn strip_html(html: &str) -> String {
    // 1) 剥离标签（跳过 `<...>`，含 MathML 自闭合标签）
    let mut text = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    // 2) 单遍解码实体
    decode_entities(&text)
}

/// 单遍解码常见命名/数字 HTML 实体（不级联：`&amp;lt;` 应还原为 `&lt;` 而非 `<`）。
///
/// 站点正文 HTML 中的字面 `&`/`<`/`>` 已被转义为实体；任何从正文 HTML 再提取
/// 纯文本的出口（摘要、搜索）都必须经此还原，否则下游转义一次后会显示成
/// `&amp;`（双重转义）。与 publish 层 [`strip_html`] 共用同一实现。
pub(crate) fn decode_entities(text: &str) -> String {
    let entities: [(&str, char); 7] = [
        ("&amp;", '&'),
        ("&lt;", '<'),
        ("&gt;", '>'),
        ("&quot;", '"'),
        ("&apos;", '\''),
        ("&#39;", '\''),
        ("&nbsp;", ' '),
    ];
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let tail = &rest[amp..];
        let mut decoded = None;
        for (name, ch) in entities {
            if tail.starts_with(name) {
                decoded = Some((name, ch));
                break;
            }
        }
        match decoded {
            Some((name, ch)) => {
                out.push(ch);
                rest = &tail[name.len()..];
            }
            None => {
                out.push('&');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// 在 HTML 开标签的尾部 `>`（或 `/>`）之前插入属性，自动补齐空格分隔。
fn insert_attr(tag: &str, attr: &str) -> String {
    let end = tag.trim_end();
    let base = if end.ends_with("/>") {
        end.len() - 2
    } else {
        end.len() - 1
    };
    let before = &tag[..base];
    let sep = if before.ends_with(' ') || before.ends_with('\t') {
        ""
    } else {
        " "
    };
    format!("{before}{sep}{attr}{}", &tag[base..])
}

/// 从 `<img src="...">` 提取文件名作 alt 兜底（如 `images/logo.png` → `logo`）。
/// data URI（内联 base64）无文件名，统一回退 `image`。
fn extract_alt_from_src(tag: &str) -> String {
    let Some(src_start) = tag.find("src=\"") else {
        return "image".to_string();
    };
    let after = &tag[src_start + 5..];
    let Some(src_end) = after.find('"') else {
        return "image".to_string();
    };
    let src = &after[..src_end];
    if src.starts_with("data:") {
        return "image".to_string();
    }
    let file = src.rsplit(['/', '\\']).next().unwrap_or("image");
    let stem = file.split('.').next().unwrap_or(file);
    if stem.is_empty() {
        "image".to_string()
    } else {
        stem.to_string()
    }
}

/// 把标签/分类名/标题转为 URL 安全的路径段（保留中文/字母/数字，空格等 → 连字符）。
pub(crate) fn slugify(name: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for c in name.trim().chars() {
        if c.is_alphanumeric() || c == '_' {
            out.push(c);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    let s = out.trim_matches('-').to_string();
    if s.is_empty() {
        "untitled".to_string()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_keeps_chinese_and_ascii() {
        assert_eq!(slugify("物理"), "物理");
        assert_eq!(slugify("hello world"), "hello-world");
        assert_eq!(slugify("  Rust  编程 "), "Rust-编程");
    }

    #[test]
    fn slugify_empty_falls_back_to_untitled() {
        assert_eq!(slugify(""), "untitled");
        assert_eq!(slugify("!!!"), "untitled");
    }

    #[test]
    fn postprocess_adds_toc_and_heading_ids() {
        let body = "<p>前言</p><h2>第一节</h2><p>正文</p><h3>子小节</h3>";
        let (out, toc) = postprocess_body(body);
        assert!(out.contains("<h2 id=\"第一节\">第一节</h2>"));
        assert!(out.contains("<h3 id=\"子小节\">子小节</h3>"));
        // P1 起 TOC 只产出 `<li>` 列表，`<ul class="toc-list">` 移入主题 partial
        assert!(toc.starts_with("<li class=\"toc-l2\""));
        assert!(toc.contains("href=\"#第一节\""));
        assert!(toc.contains("toc-l2"));
        assert!(toc.contains("toc-l3"));
    }

    #[test]
    fn postprocess_skips_toc_when_no_headings() {
        let (out, toc) = postprocess_body("<p>无标题</p>");
        assert_eq!(out, "<p>无标题</p>");
        assert_eq!(toc, "");
    }

    #[test]
    fn postprocess_strips_markup_from_headings() {
        // 标题含 *强调*（<em>）与行内公式（MathML）：id 与 TOC 只应出现纯文本
        let body = r#"<h2><em>重点</em>公式 <math><mi>x</mi><mo>=</mo><mn>1</mn></math></h2>"#;
        let (out, toc) = postprocess_body(body);
        // 正文标题保留原排版，仅注入纯文本派生 id（空格与 = 均折叠为连字符）
        assert!(out.contains(r#"<h2 id="重点公式-x-1"><em>重点</em>公式 <math>"#));
        // TOC 锚点与文案均为纯文本，无 <em>/<math> 残骸
        assert!(toc.contains("href=\"#重点公式-x-1\""));
        assert!(toc.contains(">重点公式 x=1</a>"));
        assert!(!toc.contains("<em>") && !toc.contains("<math"));
    }

    #[test]
    fn strip_html_removes_tags_and_decodes_entities_once() {
        assert_eq!(strip_html("<em>斜率</em>"), "斜率");
        assert_eq!(strip_html("<em>A</em> &lt; B"), "A < B");
        // 不级联解码：&amp;lt; 应还原为 &lt; 而非 <
        assert_eq!(strip_html("&amp;lt;"), "&lt;");
        // MathML 自闭合标签与标签间文本
        assert_eq!(
            strip_html(r#"<math><mi>x</mi><mspace width="1em"/><mo>=</mo></math>"#),
            "x="
        );
        assert_eq!(strip_html(""), "");
    }

    #[test]
    fn postprocess_tokenizes_inline_colors() {
        let body = r#"<pre><code data-lang="rust"><span style="color: #d73948">fn</span> main()</code></pre>"#;
        let (out, _) = postprocess_body(body);
        assert!(out.contains("color: var(--tok-d73948, #d73948)"));
        // 非代码文本不受影响
        assert!(out.contains("main()"));
    }

    #[test]
    fn postprocess_lazy_loads_images_but_not_first() {
        let body = r#"<img src="cover.png"><img src="a.png" alt="已有"><img src="b/c.png">"#;
        let (out, _) = postprocess_body(body);
        // 首图不加 lazy（LCP），但仍补 alt
        assert!(out.contains(r#"<img src="cover.png" alt="cover">"#));
        assert!(!out.contains("cover.png\" loading"));
        // 后两张加 lazy；已有 alt 保留，缺失 alt 用文件名兜底
        assert!(out.contains(r#"<img src="a.png" alt="已有" loading="lazy">"#));
        assert!(out.contains(r#"<img src="b/c.png" loading="lazy" alt="c">"#));
    }

    #[test]
    fn postprocess_adds_alt_fallback_to_images() {
        let body = r#"<img src="images/logo.png">"#;
        let (out, _) = postprocess_body(body);
        assert!(out.contains(r#"alt="logo""#));
    }

    #[test]
    fn extract_alt_from_src_handles_paths() {
        assert_eq!(extract_alt_from_src(r#"<img src="images/logo.png">"#), "logo");
        assert_eq!(extract_alt_from_src(r#"<img src="a/b/c">"#), "c");
        assert_eq!(extract_alt_from_src("<img>"), "image");
    }

    #[test]
    fn extract_alt_from_src_data_uri_falls_back() {
        assert_eq!(
            extract_alt_from_src(r#"<img src="data:image/png;base64,iVBORw0KGgo=">"#),
            "image"
        );
    }

    // --- 引用提升测试（typst 0.15 quote 元素降级补救） ---

    #[test]
    fn postprocess_promotes_single_gt_paragraph_to_blockquote() {
        // typst `> 单段引用` 渲染成 `<p>&gt; 单段引用</p>`：应提升为 blockquote。
        let (out, _) = postprocess_body("<p>&gt; 单段引用</p>");
        assert!(out.contains("<blockquote>"));
        assert!(out.contains("<p>单段引用</p>"));
        assert!(out.contains("</blockquote>"));
        assert!(!out.contains("&gt;"));
    }

    #[test]
    fn postprocess_merges_consecutive_gt_paragraphs_into_one_blockquote() {
        // 跨段连续的 `>` 引文应合并到同一个 blockquote。
        let body = "<p>&gt; 第一句</p><p>&gt; 第二句</p>";
        let (out, _) = postprocess_body(body);
        assert!(out.contains("<blockquote><p>第一句</p><p>第二句</p></blockquote>"));
        assert!(!out.contains("&gt;"));
    }

    #[test]
    fn postprocess_blockquote_does_not_swallow_neighbour_paragraphs() {
        // 普通段落夹在中间不应被吞入 blockquote。
        let body = "<p>普通</p><p>&gt; 引用</p><p>普通2</p>";
        let (out, _) = postprocess_body(body);
        assert!(out.contains("<p>普通</p>"));
        assert!(out.contains("<p>普通2</p>"));
        assert!(out.contains("<blockquote><p>引用</p></blockquote>"));
    }

    #[test]
    fn postprocess_blockquote_handles_gt_with_leading_space() {
        // typst 渲染时 `> ` 后可能带空格，需兼容。
        let (out, _) = postprocess_body("<p>&gt;  带空格</p>");
        assert!(out.contains("<blockquote><p>带空格</p></blockquote>"));
    }
}
