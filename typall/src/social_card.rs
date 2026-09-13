//! 社交分享卡（og:image）自动生成。
//!
//! 文章无首图时，用站点令牌配色渲染一张 1200×630 的分享卡 PNG
//! （标题 + 站点名 + 日期），供微信/Twitter 等平台转发时展示缩略图。
//! 直接手绘 SVG 后走 resvg 栅格化（复用 [`crate::wechat`] 的渲染栈），
//! 不经 Typst 编译器，单张成本毫秒级。

/// 分享卡逻辑尺寸（px）——各平台 og:image 的通用比例 1.91:1。
const W: f32 = 1200.0;
const H: f32 = 630.0;
/// 栅格化倍率（2x 高清）。
const SCALE: f32 = 2.0;

/// 转义 SVG 文本中的保留字符。
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
      .replace('<', "&lt;")
      .replace('>', "&gt;")
      .replace('"', "&quot;")
}

/// CJK 感知的标题换行：按显示宽度近似折行（CJK 记 2、半角记 1），
/// 最多 `max_lines` 行，溢出末行以「…」结尾。返回每行文本。
fn wrap_title(title: &str, max_units_per_line: usize, max_lines: usize) -> Vec<String> {
    let width = |c: char| if c.is_ascii() { 1 } else { 2 };
    let mut lines = Vec::new();
    let mut cur = String::new();
    let mut cur_w = 0usize;
    for c in title.chars() {
        let cw = width(c);
        if cur_w + cw > max_units_per_line {
            lines.push(std::mem::take(&mut cur));
            cur_w = 0;
            if lines.len() == max_lines {
                // 已达行数上限：末行截断加省略号（末行至少保留一半宽度再截）。
                let last = lines.last_mut().unwrap();
                let mut units = 0usize;
                let mut end = last.char_indices().count();
                for (i, ch) in last.char_indices() {
                    units += width(ch);
                    if units > max_units_per_line.saturating_sub(2) {
                        end = i;
                        break;
                    }
                }
                let mut truncated: String = last.chars().take(end).collect();
                truncated.push('…');
                *last = truncated;
                return lines;
            }
        }
        cur_w += cw;
        cur.push(c);
    }
    if !cur.is_empty() || lines.is_empty() {
        lines.push(cur);
    }
    lines
}

/// 渲染分享卡 PNG 字节。
///
/// `accent`：主题强调色（如 `#0b6fc0`），用于左侧色条与站点名。
pub(crate) fn render_card_png(
    title: &str,
    site_title: &str,
    date: &str,
    accent: &str,
) -> anyhow::Result<Vec<u8>> {
    let lines = wrap_title(title, 26, 3);
    let font = "font-family=\"PingFang SC, Microsoft YaHei, Noto Sans CJK SC, Source Han Sans SC, sans-serif\"";
    // 标题行从视觉中心起排：行高 74，块高按行数收缩，整体垂直居中偏上。
    let line_h = 74.0_f32;
    let block_h = lines.len() as f32 * line_h;
    let title_y0 = H / 2.0 - block_h / 2.0 + 26.0;

    let mut title_tspans = String::new();
    for (i, line) in lines.iter().enumerate() {
        title_tspans.push_str(&format!(
            "<tspan x=\"96\" y=\"{}\">{}</tspan>",
            title_y0 + i as f32 * line_h,
            esc(line)
        ));
    }

    // 注意：SVG 含 `fill="#…"`，必须用 `r##` 定界（`"#` 会提前终止 `r#`）。
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">
  <rect width="{W}" height="{H}" fill="#faf9f6"/>
  <rect x="0" y="0" width="14" height="{H}" fill="{accent}"/>
  <text x="96" y="128" font-size="30" font-weight="600" fill="{accent}" {font}>{site}</text>
  <text x="96" y="{title_y}" font-size="58" font-weight="700" fill="#20222a" {font}>{tspans}</text>
  <line x1="96" y1="{H}-150" x2="{W}-96" y2="{H}-150" stroke="#e6e2d6" stroke-width="2"/>
  <text x="96" y="{H}-100" font-size="26" fill="#686a72" {font}>{date}</text>
</svg>"##,
        site = esc(site_title),
        tspans = title_tspans,
        title_y = title_y0,
        date = esc(date),
        font = font,
        accent = esc(accent),
    );

    // usvg 默认 fontdb 为空库，`<text>` 无法匹配任何字体会渲染成空白——
    // 必须加载系统字体，并把默认字体族指到中文字体（标题含 CJK）。
    let mut fontdb = resvg::usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    let options = resvg::usvg::Options {
        fontdb: std::sync::Arc::new(fontdb),
        font_family: "Microsoft YaHei".into(),
        ..Default::default()
    };
    let tree = resvg::usvg::Tree::from_str(&svg, &options)
        .map_err(|e| anyhow::anyhow!("分享卡 SVG 解析失败: {e}"))?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new((W * SCALE) as u32, (H * SCALE) as u32)
        .ok_or_else(|| anyhow::anyhow!("分享卡位图分配失败"))?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(SCALE, SCALE),
        &mut pixmap.as_mut(),
    );
    pixmap
        .encode_png()
        .map_err(|e| anyhow::anyhow!("分享卡 PNG 编码失败: {e}"))
}

/// 从正文 HTML 提取首图路径（`<img src="…">`），供 og:image 优先使用。
pub(crate) fn first_image_path(body_html: &str) -> Option<String> {
    let idx = body_html.find("<img")?;
    let rest = &body_html[idx..];
    // `src="` 必须落在属性名位置（前一字符是空白）：`data-src="` 里的
    // `src="` 子串不构成匹配，否则懒加载占位属性会抢先被当成首图。
    let mut from = 0usize;
    let src_pos = loop {
        let rel = rest[from..].find("src=\"")? + from;
        let before = rest.as_bytes().get(rel.wrapping_sub(1));
        if before.is_some_and(|b| b.is_ascii_whitespace()) {
            break rel + 5;
        }
        from = rel + 5;
    };
    let tail = &rest[src_pos..];
    let end = tail.find('"')?;
    let src = &tail[..end];
    if src.is_empty() {
        return None;
    }
    Some(src.to_string())
}

/// 把正文首图的相对路径归一化为站点绝对路径（以 `/` 开头）。
///
/// 文章渲染物里的图片相对路径以文章目录（`/{slug 所在目录}/`）为基准
/// （typst `image("../assets/x.png")` 语义）；绝对 URL 原样返回；
/// 站点根相对（`/assets/x.png`）与协议相对（`//cdn.x/i.png`）按各自语义拼接。
pub(crate) fn absolutize_og_image(slug: &str, src: &str, site_url: &str) -> Option<String> {
    let site_url = site_url.trim_end_matches('/');
    if src.starts_with("http://") || src.starts_with("https://") || src.starts_with("data:") {
        return Some(src.to_string());
    }
    // 协议相对：补 https（站点根相对的 `/` 前缀判断必须在 `//` 之后）
    if let Some(rest) = src.strip_prefix("//") {
        return Some(format!("https://{rest}"));
    }
    // 站点根相对：直接挂到域名下（不能按文章目录拼接，否则 404）
    if src.starts_with('/') {
        return Some(format!("{site_url}{src}"));
    }
    // slug 形如 `posts/foo` 或 `foo`；文章页 URL 为 `/{slug}/`。
    let base = format!("/{}/", slug.trim_end_matches('/'));
    // 简易路径归一化：按段处理 `.`/`..`。
    let joined = format!("{base}{src}");
    let mut stack: Vec<&str> = Vec::new();
    for seg in joined.split('/') {
        match seg {
            "." | "" => {}
            ".." => {
                stack.pop();
            }
            s => stack.push(s),
        }
    }
    Some(format!("{site_url}/{}", stack.join("/")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_png_has_magic_and_dimensions() {
        let png = render_card_png("测试标题", "站点", "2026-09-05", "#0b6fc0").unwrap();
        assert_eq!(&png[..4], &[0x89, b'P', b'N', b'G']);
        // 2x：应为 2400×1260
        let w = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
        let h = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
        assert_eq!((w, h), (2400, 1260));
    }

    #[test]
    fn long_title_wraps_and_truncates() {
        let lines = wrap_title(&"字".repeat(100), 26, 3);
        assert_eq!(lines.len(), 3);
        assert!(lines[2].ends_with('…'));
        assert!(lines.iter().all(|l| l.chars().count() <= 26));
        let short = wrap_title("短标题", 26, 3);
        assert_eq!(short, vec!["短标题"]);
    }

    #[test]
    fn first_image_extracts_src() {
        let html = r#"<p>前文</p><img src="../assets/a.png"><p>后</p>"#;
        assert_eq!(first_image_path(html).as_deref(), Some("../assets/a.png"));
        assert_eq!(first_image_path("<p>无图</p>"), None);
    }

    #[test]
    fn first_image_skips_data_src_attribute() {
        // 回归：`data-src="` 内的 `src="` 子串曾被误当首图属性
        let html = r#"<img data-src="lazy.png" src="real.png">"#;
        assert_eq!(first_image_path(html).as_deref(), Some("real.png"));
    }

    #[test]
    fn absolutize_handles_relative_and_absolute() {
        let url = "https://blog.example.com";
        // 标准 URL 解析语义：文章页 /posts/foo/ 基准下 `../assets/a.png`
        // → /posts/assets/a.png（与浏览器行为一致）
        assert_eq!(
            absolutize_og_image("posts/foo", "../assets/a.png", url).as_deref(),
            Some("https://blog.example.com/posts/assets/a.png")
        );
        assert_eq!(
            absolutize_og_image("posts/foo", "cover.png", url).as_deref(),
            Some("https://blog.example.com/posts/foo/cover.png")
        );
        assert_eq!(
            absolutize_og_image("posts/foo", "https://cdn.x/i.png", url).as_deref(),
            Some("https://cdn.x/i.png")
        );
    }

    #[test]
    fn absolutize_handles_root_and_protocol_relative() {
        let url = "https://blog.example.com";
        // 站点根相对：挂到域名下，而非按文章目录拼接（曾产出 /posts/foo/assets/… 404）
        assert_eq!(
            absolutize_og_image("posts/foo", "/assets/cover.png", url).as_deref(),
            Some("https://blog.example.com/assets/cover.png")
        );
        // 协议相对：补 https
        assert_eq!(
            absolutize_og_image("posts/foo", "//cdn.x/i.png", url).as_deref(),
            Some("https://cdn.x/i.png")
        );
    }
}
