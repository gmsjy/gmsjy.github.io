//! 「复制排版主题」：`serve` 预览页复制到公众号 / 知乎时，正文内联样式的可选风格。
//!
//! **主题的变化 = 不同的 CSS**。与站点主题（`themes/`，控制博客自身外观）解耦——
//! 公众号编辑器只认浅色内联 style，粘贴时会剥类名与外链 CSS，因此主题引擎把
//! CSS 规则逐元素转成内联 style，而不是下发样式表。
//!
//! 主题来源两层：
//! 1. **内置预设**：五套 CSS 定义（墨理蓝为基底 / 纸砚衬线 / 柑橘暖阳 / 极简黑白 /
//!    曜石夜），色板取自对应站点主题，CSS 本身就是可照抄的主题模板；
//! 2. **自定义**：项目根 `copythemes/<id>.css`（推荐）或 `.toml`（令牌式），
//!    文件名即主题 id；serve 运行中新增/修改即随重建热加载。
//!
//! CSS 写法 = 普通元素选择器 + 声明，只写想改的槽位，其余回落默认墨理基底：
//!
//! ```css
//! /* typall-copy-theme: label="学院青" dark=false */
//! h2 { border-left: 4px solid #7c3aed; padding-left: 12px; color: #2b2350; }
//! a { color: #7c3aed; }
//! ```
//!
//! 选择器 → 槽位映射（[`slot_for_selector`]）：标签名直映射（`h1`/`p`/
//! `blockquote`/…），另有 `article`（正文基底）、`code`（行内 code）、
//! `pre code`（pre 内 code）、`svg`（公式/插图底色）、`b`/`strong`、`em`/`i`、
//! `del`/`s`、`.eq-num`（公式编号）、`[data-equation="block"]`（块级公式容器）。
//! 未知选择器（类名/伪类/@规则）静默忽略——粘贴目标不认它们，写了也没用。
//!
//! 数据以 JSON 注入预览页（`serve.rs` 的 `COPY_SCRIPT` 消费）：
//! `[{ "id", "label", "dark", "styles": { "h2": "…", … } }]`；`styles` 值为
//! 完整内联 CSS 字符串。typst 公式 SVG 用 `fill="currentColor"`，但站点 CSS
//! 对 svg 有直接 fill/color 规则，优先级高于祖先继承，故 `svg` 槽位显式
//! 上正文色；cetz 彩色插图用显式 fill，不受影响。

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use serde_json::{json, Map, Value};

/// 首个主题是默认主题：`复制到公众号/知乎` 在「站点样式」档位下按它出稿
/// （与历史行为一致——旧版写死的就是墨理配色）。
pub const DEFAULT_ID: &str = "moli";

/// 自定义主题目录（项目根相对）。
pub const CUSTOM_DIR: &str = "copythemes";

/// 内置主题（顺序即下拉框顺序，首个为默认）。除墨理蓝（=默认基底本身）外，
/// 其余四套均为 CSS 定义——一份 CSS 就是一套主题，可直接照抄改写。
pub fn builtin() -> Vec<Value> {
    vec![
        moli(),
        theme_from_css("paper", "纸砚衬线", false, PAPER_CSS),
        theme_from_css("citrus", "柑橘暖阳", false, CITRUS_CSS),
        theme_from_css("minimal", "极简黑白", false, MINIMAL_CSS),
        theme_from_css("obsidian", "曜石夜（暗色）", true, OBSIDIAN_CSS),
    ]
}

/// 内置 + 自定义（`<root>/copythemes/*.{css,toml}`）合并后的完整主题清单。
/// 自定义追加在内置之后；非法文件 / id 冲突跳过并告警（不中断 serve）。
pub fn all_themes(root: &Path) -> Vec<Value> {
    let mut themes = builtin();
    let mut seen: Vec<String> = themes
        .iter()
        .filter_map(|t| t["id"].as_str().map(String::from))
        .collect();
    let dir = root.join(CUSTOM_DIR);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return themes; // 目录不存在 = 无自定义，静默
    };
    let mut files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            matches!(
                p.extension().and_then(|x| x.to_str()),
                Some("css") | Some("toml")
            )
        })
        .collect();
    files.sort();
    for path in files {
        let Some(stem) = path.file_stem().and_then(|x| x.to_str()) else {
            continue;
        };
        // id 进下拉框 value 与 localStorage，限定安全字符集
        let valid_id = !stem.is_empty()
            && stem != "__site__"
            && stem
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if !valid_id {
            eprintln!("⚠️ 跳过自定义复制主题 `{}`：文件名需为字母/数字/-/_", path.display());
            continue;
        }
        if seen.iter().any(|s| s == stem) {
            eprintln!(
                "⚠️ 跳过自定义复制主题 `{stem}`（{}）：id 与已有主题冲突",
                path.display()
            );
            continue;
        }
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("⚠️ 自定义复制主题 `{}` 读取失败: {e}", path.display());
                continue;
            }
        };
        let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        let rendered = if ext == "css" {
            let (label, dark) = css_theme_meta(&text);
            Ok(theme_from_css(
                stem,
                &label.unwrap_or_else(|| stem.to_string()),
                dark,
                &text,
            ))
        } else {
            match toml::from_str::<CustomThemeFile>(&text) {
                Ok(f) => Ok(theme(
                    stem,
                    &f.label.clone().unwrap_or_else(|| stem.to_string()),
                    f.dark.unwrap_or(false),
                    {
                        let mut styles = base_styles(&f.merged_palette());
                        for (k, v) in &f.styles {
                            styles.insert(k.clone(), Value::String(v.clone()));
                        }
                        styles
                    },
                )),
                Err(e) => Err(e.to_string()),
            }
        };
        match rendered {
            Ok(v) => {
                seen.push(stem.to_string());
                themes.push(v);
            }
            Err(e) => {
                eprintln!("⚠️ 自定义复制主题 `{}` 解析失败，已跳过: {e}", path.display());
            }
        }
    }
    themes
}

/// 注入用 JSON（内置 + 自定义）。`<` 统一转义为 `\u003c`：JSON 会内嵌进
/// `<script>` 标签，自定义主题文件里若出现 `</script>` 不能提前闭合标签。
pub fn themes_json_for(root: &Path) -> String {
    sanitize(serde_json::to_string(&all_themes(root)).expect("copy themes serialize"))
}

/// `<` → `\u003c`（合法 JSON 转义；JSON 语法字符不含 `<`，只影响字符串值）。
fn sanitize(json: String) -> String {
    json.replace('<', "\\u003c")
}

// ────────────────────────── 极简 CSS 解析 ──────────────────────────

/// 引号感知地按 `sep` 切分（引号内的分隔符不切，反斜杠转义不结束引号）。
fn split_top(s: &str, sep: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for c in s.chars() {
        if escaped {
            cur.push(c);
            escaped = false;
            continue;
        }
        match c {
            '\\' if quote.is_some() => {
                cur.push(c);
                escaped = true;
            }
            '\'' | '"' if quote.is_none() => {
                quote = Some(c);
                cur.push(c);
            }
            '\'' | '"' if quote == Some(c) => {
                quote = None;
                cur.push(c);
            }
            c if quote.is_some() => cur.push(c),
            c if c == sep => {
                parts.push(std::mem::take(&mut cur));
            }
            c => cur.push(c),
        }
    }
    parts.push(cur);
    parts
}

/// 解析「选择器组 { 声明 }」规则。声明不支持嵌套；@规则体可含嵌套块，
/// 按大括号深度整体跳过。声明值里的分号/冒号在引号内不切分
/// （如 `font-family:"a;b"`、`url(http://…)`）。
fn parse_css_rules(css: &str) -> Vec<(String, String)> {
    // 去块注释（字符串里的 /* 极罕见，主题 CSS 不处理该边角）
    let mut no_comments = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(pos) = rest.find("/*") {
        no_comments.push_str(&rest[..pos]);
        rest = match rest[pos + 2..].find("*/") {
            Some(end) => &rest[pos + 2 + end + 2..],
            None => "",
        };
    }
    no_comments.push_str(rest);

    let s = no_comments;
    let bytes = s.as_bytes();
    let mut rules = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        // 下一个块起始 `{`
        let open = match bytes[i..].iter().position(|&b| b == b'{') {
            Some(p) => i + p,
            None => break,
        };
        let selector = s[i..open].trim();
        // 本块收尾 `}`：@规则体可嵌套块，按深度匹配
        let (body, next) = {
            let mut depth = 1usize;
            let mut j = open + 1;
            while j < bytes.len() && depth > 0 {
                match bytes[j] {
                    b'{' => depth += 1,
                    b'}' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            if depth != 0 {
                break; // 大括号不配对，丢弃残段
            }
            (s[open + 1..j - 1].to_string(), j)
        };
        i = next;
        if selector.starts_with('@') || selector.is_empty() {
            continue; // @media 等整块忽略
        }
        let decls: String = split_top(&body, ';')
            .iter()
            .filter_map(|d| {
                let d = d.trim();
                if d.is_empty() {
                    return None;
                }
                let kv = split_top(d, ':');
                let prop = kv[0].trim();
                let value = kv[1..].join(":").trim().to_string();
                if prop.is_empty() || value.is_empty() {
                    return None;
                }
                Some(format!("{prop}:{value}"))
            })
            .collect::<Vec<_>>()
            .join(";");
        if decls.is_empty() {
            continue;
        }
        for sel in split_top(selector, ',') {
            rules.push((sel.trim().to_lowercase(), format!("{decls};")));
        }
    }
    rules
}

/// 选择器 → styles 槽位。未知选择器返回 None（静默忽略）。
fn slot_for_selector(sel: &str) -> Option<&'static str> {
    match sel {
        "article" => Some("article"),
        "h1" => Some("h1"),
        "h2" => Some("h2"),
        "h3" => Some("h3"),
        "h4" => Some("h4"),
        "p" => Some("p"),
        "blockquote" => Some("blockquote"),
        "pre" => Some("pre"),
        "ul" => Some("ul"),
        "ol" => Some("ol"),
        "li" => Some("li"),
        "table" => Some("table"),
        "th" => Some("th"),
        "td" => Some("td"),
        "hr" => Some("hr"),
        "a" => Some("a"),
        "img" => Some("img"),
        "strong" | "b" => Some("strong"),
        "em" | "i" => Some("em"),
        "del" | "s" => Some("del"),
        "svg" => Some("svg"),
        "code" => Some("codeInline"),
        "pre code" => Some("codeInPre"),
        ".eq-num" => Some("eqNum"),
        "[data-equation=\"block\"]"
        | "div[data-equation=\"block\"]"
        | "div[data-equation=block]" => Some("eqBlock"),
        "[data-equation=\"block\"] svg" | "div[data-equation=\"block\"] svg" => Some("eqSvg"),
        _ => None,
    }
}

/// 以默认墨理基底起步，把 CSS 规则按槽位覆盖上去，返回完整主题。
/// 只写想改元素的 CSS 也能得到齐全槽位（缺的回落基底）。
fn theme_from_css(id: &str, label: &str, dark: bool, css: &str) -> Value {
    let mut styles = base_styles(&default_palette());
    for (sel, decls) in parse_css_rules(css) {
        if let Some(slot) = slot_for_selector(&sel) {
            styles.insert(slot.into(), Value::String(decls));
        }
    }
    theme(id, label, dark, styles)
}

/// 解析文件头可选的元信息注释：`/* typall-copy-theme: label="名字" dark */`。
/// label 缺省用文件名；`dark` 裸写或 `dark=true` 均为真。
fn css_theme_meta(css: &str) -> (Option<String>, bool) {
    let Some(pos) = css.find("typall-copy-theme:") else {
        return (None, false);
    };
    let rest = &css[pos + "typall-copy-theme:".len()..];
    let scope = &rest[..rest.find("*/").unwrap_or(rest.len())];
    let mut label = None;
    let mut dark = false;
    for tok in split_top(scope, ' ') {
        let tok = tok.trim();
        if tok.is_empty() {
            continue;
        }
        match tok.split_once('=') {
            Some(("label", v)) => {
                label = Some(v.trim().trim_matches('"').to_string());
            }
            Some(("dark", v)) => dark = v.trim() == "true",
            Some(_) => {}
            None if tok == "dark" => dark = true,
            None => {}
        }
    }
    (label, dark)
}

// ────────────────────────── 色板与基底（默认 = 墨理蓝） ──────────────────────────

#[derive(Deserialize, Default)]
#[serde(default)]
struct CustomThemeFile {
    /// 下拉框显示名；缺省用文件名（即 id）。
    label: Option<String>,
    /// 暗色主题标记（无样式作用，仅语义；暗色出稿自带深底）。
    dark: Option<bool>,
    /// 色板：在默认墨理色板上按字段覆盖，再展开成全套槽位样式。
    palette: PaletteToml,
    /// 逐槽位覆写（高级）：直接给出内联 CSS，优先级高于 palette 展开结果。
    styles: BTreeMap<String, String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct PaletteToml {
    fg: Option<String>,
    muted: Option<String>,
    accent: Option<String>,
    border: Option<String>,
    code_bg: Option<String>,
    code_ink: Option<String>,
    quote_bar: Option<String>,
    quote_bg: Option<String>,
    th_bg: Option<String>,
    body_font: Option<String>,
    head_font: Option<String>,
    mono_font: Option<String>,
    radius: Option<String>,
    radius_s: Option<String>,
    h2_left_bar: Option<bool>,
    h1_align: Option<String>,
    quote_bar_w: Option<String>,
}

impl CustomThemeFile {
    fn merged_palette(&self) -> PaletteOwned {
        fn set(dst: &mut String, src: &Option<String>) {
            if let Some(v) = src {
                *dst = v.clone();
            }
        }
        let mut p = default_palette();
        let o = &self.palette;
        set(&mut p.fg, &o.fg);
        set(&mut p.muted, &o.muted);
        set(&mut p.accent, &o.accent);
        set(&mut p.border, &o.border);
        set(&mut p.code_bg, &o.code_bg);
        set(&mut p.code_ink, &o.code_ink);
        set(&mut p.quote_bar, &o.quote_bar);
        set(&mut p.quote_bg, &o.quote_bg);
        set(&mut p.th_bg, &o.th_bg);
        set(&mut p.body_font, &o.body_font);
        set(&mut p.head_font, &o.head_font);
        set(&mut p.mono_font, &o.mono_font);
        set(&mut p.radius, &o.radius);
        set(&mut p.radius_s, &o.radius_s);
        set(&mut p.h1_align, &o.h1_align);
        set(&mut p.quote_bar_w, &o.quote_bar_w);
        if let Some(v) = o.h2_left_bar {
            p.h2_left_bar = v;
        }
        p
    }
}

/// 色板与字体（owned，供默认基底与 TOML 令牌主题共用）。
#[derive(Clone)]
struct PaletteOwned {
    fg: String,          // 正文
    muted: String,       // 次级文本（引用文字 / 公式编号）
    accent: String,      // 强调色（链接）
    border: String,      // 边框 / 分隔线
    code_bg: String,     // 代码块与行内 code 衬底
    code_ink: String,    // 代码文字色
    quote_bar: String,   // 引用左侧竖条
    quote_bg: String,    // 引用衬底
    th_bg: String,       // 表头衬底（`transparent` = 不上底色）
    body_font: String,   // 正文字体栈
    head_font: String,   // 标题字体栈
    mono_font: String,   // 等宽字体栈
    radius: String,      // 大圆角（pre / img）
    radius_s: String,    // 小圆角（行内 code）
    h2_left_bar: bool,   // h2 处理：左侧色条（true）或下边线（false）
    h1_align: String,    // h1 附加片段（如 `text-align:center;`，空 = 不居中）
    quote_bar_w: String, // 引用竖条宽度
}

const SANS: &str = "-apple-system,\"PingFang SC\",\"Microsoft YaHei\",\"Helvetica Neue\",sans-serif;";
const MONO: &str = "\"SFMono-Regular\",Consolas,\"DejaVu Sans Mono\",monospace;";

/// 默认色板 = 墨理蓝。
fn default_palette() -> PaletteOwned {
    PaletteOwned {
        fg: "#20222a".into(),
        muted: "#686a72".into(),
        accent: "#0b6fc0".into(),
        border: "#e6e2d6".into(),
        code_bg: "#f3f1ec".into(),
        code_ink: "#20222a".into(),
        quote_bar: "#3c5a49".into(),
        quote_bg: "#eef2ec".into(),
        th_bg: "transparent".into(),
        body_font: SANS.into(),
        head_font: SANS.into(),
        mono_font: MONO.into(),
        radius: "8px".into(),
        radius_s: "4px".into(),
        h2_left_bar: false,
        h1_align: String::new(),
        quote_bar_w: "3px".into(),
    }
}

/// 由色板生成全套槽位（默认基底）。结构照抄 `serve.rs` 复制脚本验证过的
/// 公众号兼容写法（px 字号、无类名依赖、合并式内联 style）。
fn base_styles(p: &PaletteOwned) -> Map<String, Value> {
    let mut s = Map::new();
    let mut ins = |k: &str, v: String| {
        s.insert(k.into(), Value::String(v));
    };

    ins("article", format!("{f}font-size:16px;line-height:1.8;color:{fg};", f = p.body_font, fg = p.fg));
    ins("h1", format!("font-size:28px;font-weight:bold;line-height:1.3;margin:34px 0 16px;{hf}{align}color:{fg};", hf = p.head_font, align = p.h1_align, fg = p.fg));
    if p.h2_left_bar {
        ins("h2", format!("font-size:22px;font-weight:bold;line-height:1.3;margin:34px 0 14px;padding-left:12px;border-left:4px solid {accent};{hf}color:{fg};", accent = p.accent, hf = p.head_font, fg = p.fg));
    } else {
        ins("h2", format!("font-size:22px;font-weight:bold;line-height:1.3;margin:34px 0 14px;padding-bottom:6px;border-bottom:1px solid {b};{hf}color:{fg};", b = p.border, hf = p.head_font, fg = p.fg));
    }
    ins("h3", format!("font-size:19px;font-weight:bold;line-height:1.3;margin:26px 0 10px;{hf}color:{fg};", hf = p.head_font, fg = p.fg));
    ins("h4", format!("font-size:17px;font-weight:bold;line-height:1.3;margin:26px 0 10px;{hf}color:{fg};", hf = p.head_font, fg = p.fg));
    ins("p", format!("font-size:16px;line-height:1.8;margin:14px 0;{f}color:{fg};", f = p.body_font, fg = p.fg));
    ins("blockquote", format!("font-size:16px;line-height:1.8;margin:18px 0;padding:8px 18px;border-left:{w} solid {bar};background:{bg};color:{muted};{f};", w = p.quote_bar_w, bar = p.quote_bar, bg = p.quote_bg, muted = p.muted, f = p.body_font));
    ins("pre", format!("background:{cb};padding:14px 16px;border-radius:{r};overflow-x:auto;font-size:14px;line-height:1.6;color:{ci};{m};", cb = p.code_bg, r = p.radius, ci = p.code_ink, m = p.mono_font));
    ins("ul", format!("margin:14px 0;padding-left:26px;line-height:1.8;{f}color:{fg};", f = p.body_font, fg = p.fg));
    ins("ol", format!("margin:14px 0;padding-left:26px;line-height:1.8;{f}color:{fg};", f = p.body_font, fg = p.fg));
    ins("li", "margin:4px 0;".into());
    ins("table", "border-collapse:collapse;width:100%;margin:16px 0;".into());
    ins("th", format!("border:1px solid {b};background:{tb};padding:8px 12px;text-align:left;font-size:15px;line-height:1.6;{f}color:{fg};", b = p.border, tb = p.th_bg, f = p.body_font, fg = p.fg));
    ins("td", format!("border:1px solid {b};padding:8px 12px;text-align:left;font-size:15px;line-height:1.6;{f}color:{fg};", b = p.border, f = p.body_font, fg = p.fg));
    ins("hr", format!("border:none;border-top:1px solid {b};margin:26px 0;", b = p.border));
    ins("a", format!("color:{accent};", accent = p.accent));
    // strong：站点 CSS 常给加粗文字直接上色，主题必须覆盖，否则暗色主题下不可见
    ins("strong", format!("color:{fg};", fg = p.fg));
    ins("img", format!("max-width:100%;height:auto;border-radius:{r};", r = p.radius));
    ins("codeInline", format!("background:{cb};color:{ci};padding:1px 6px;border-radius:{rs};{m}font-size:0.9em;", cb = p.code_bg, ci = p.code_ink, rs = p.radius_s, m = p.mono_font));
    ins("codeInPre", format!("{m}font-size:0.9em;", m = p.mono_font));
    // 公式 SVG：站点 CSS 对 svg 有直接 fill/color 规则，优先级高于祖先继承——
    // svg 必须显式拿到正文色与填充色（否则暗色主题下无 fill 属性的字形继承黑色）。
    // cetz 彩色插图用显式 fill，不受此影响。
    ins("svg", format!("color:{fg};fill:{fg};", fg = p.fg));
    // 块级公式：预览用 flex/绝对定位，公众号不支持 → 居中 + 编号单独一行靠右
    ins("eqBlock", "text-align:center;margin:18px 0;".into());
    ins("eqSvg", "vertical-align:middle;".into());
    ins("eqNum", format!("display:block;text-align:right;margin:5px 0 0;font-size:15px;color:{muted};{f}", muted = p.muted, f = p.body_font));
    s
}

fn theme(id: &str, label: &str, dark: bool, styles: Map<String, Value>) -> Value {
    json!({ "id": id, "label": label, "dark": dark, "styles": Value::Object(styles) })
}

/// 墨理蓝（默认）：即默认基底本身，色值与历史写死版本一致。
fn moli() -> Value {
    theme(DEFAULT_ID, "墨理蓝（默认）", false, base_styles(&default_palette()))
}

// ────────────────────────── 内置主题（CSS 定义，可照抄作模板） ──────────────────────────

/// 纸砚衬线：学术纸感（色板取自站点主题 paper）——衬线正文、朱砂强调、居中大标题、零圆角。
const PAPER_CSS: &str = r#"/* 纸砚衬线 —— 学术纸感：衬线正文 + 朱砂点睛 + 居中大标题 */
article { font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; font-size:16px; line-height:1.8; color:#1c1c1c; }
h1 { font-size:28px; font-weight:bold; line-height:1.3; margin:34px 0 16px; text-align:center; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; color:#1c1c1c; }
h2 { font-size:22px; font-weight:bold; line-height:1.3; margin:34px 0 14px; padding-bottom:6px; border-bottom:1px solid #e3e1dc; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; color:#1c1c1c; }
h3 { font-size:19px; font-weight:bold; line-height:1.3; margin:26px 0 10px; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; color:#1c1c1c; }
h4 { font-size:17px; font-weight:bold; line-height:1.3; margin:26px 0 10px; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; color:#1c1c1c; }
p { font-size:16px; line-height:1.8; margin:14px 0; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; color:#1c1c1c; }
blockquote { font-size:16px; line-height:1.8; margin:18px 0; padding:8px 18px; border-left:3px solid #34506b; background:#eef2f6; color:#565656; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; }
pre { background:#f7f6f3; padding:14px 16px; border-radius:0; overflow-x:auto; font-size:14px; line-height:1.6; color:#3a3a3a; font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace; }
ul, ol { margin:14px 0; padding-left:26px; line-height:1.8; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; color:#1c1c1c; }
th { border:1px solid #e3e1dc; background:#f7f6f3; padding:8px 12px; text-align:left; font-size:15px; line-height:1.6; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; color:#1c1c1c; }
td { border:1px solid #e3e1dc; padding:8px 12px; text-align:left; font-size:15px; line-height:1.6; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; color:#1c1c1c; }
hr { border:none; border-top:1px solid #e3e1dc; margin:26px 0; }
a { color:#8c1f28; }
img { max-width:100%; height:auto; border-radius:0; }
strong { color:#1c1c1c; }
code { background:#f7f6f3; color:#3a3a3a; padding:1px 6px; border-radius:0; font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace; font-size:0.9em; }
svg { color:#1c1c1c; fill:#1c1c1c; }
.eq-num { display:block; text-align:right; margin:5px 0 0; font-size:15px; color:#565656; font-family:Georgia,"Noto Serif SC","Source Han Serif SC","SimSun",serif; }
"#;

/// 柑橘暖阳：暖色文艺（色板取自站点主题 citrus）——楷体标题、橘色左条、大圆角。
const CITRUS_CSS: &str = r#"/* 柑橘暖阳 —— 米纸暖褐 + 楷体标题 + 橘色左条 + 大圆角 */
article { font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; font-size:16px; line-height:1.8; color:#40342a; }
h1 { font-size:28px; font-weight:bold; line-height:1.3; margin:34px 0 16px; font-family:"Kaiti SC","STKaiti","KaiTi","FangSong",serif; color:#40342a; }
h2 { font-size:22px; font-weight:bold; line-height:1.3; margin:34px 0 14px; padding-left:12px; border-left:4px solid #e2691e; font-family:"Kaiti SC","STKaiti","KaiTi","FangSong",serif; color:#40342a; }
h3 { font-size:19px; font-weight:bold; line-height:1.3; margin:26px 0 10px; font-family:"Kaiti SC","STKaiti","KaiTi","FangSong",serif; color:#40342a; }
h4 { font-size:17px; font-weight:bold; line-height:1.3; margin:26px 0 10px; font-family:"Kaiti SC","STKaiti","KaiTi","FangSong",serif; color:#40342a; }
p { font-size:16px; line-height:1.8; margin:14px 0; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#40342a; }
blockquote { font-size:16px; line-height:1.8; margin:18px 0; padding:8px 18px; border-left:3px solid #7a8450; background:#f2f3e3; color:#83715e; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; }
pre { background:#f6edda; padding:14px 16px; border-radius:10px; overflow-x:auto; font-size:14px; line-height:1.6; color:#4d4234; font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace; }
ul, ol { margin:14px 0; padding-left:26px; line-height:1.8; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#40342a; }
th { border:1px solid #ecdfc8; background:#f5ead6; padding:8px 12px; text-align:left; font-size:15px; line-height:1.6; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#40342a; }
td { border:1px solid #ecdfc8; padding:8px 12px; text-align:left; font-size:15px; line-height:1.6; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#40342a; }
hr { border:none; border-top:1px solid #ecdfc8; margin:26px 0; }
a { color:#e2691e; }
img { max-width:100%; height:auto; border-radius:10px; }
strong { color:#40342a; }
code { background:#f6edda; color:#4d4234; padding:1px 6px; border-radius:4px; font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace; font-size:0.9em; }
svg { color:#40342a; fill:#40342a; }
.eq-num { display:block; text-align:right; margin:5px 0 0; font-size:15px; color:#83715e; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; }
"#;

/// 极简黑白：零圆角单色（色板取自站点主题 minimal）——无装饰、黑色引用条、下划线链接。
const MINIMAL_CSS: &str = r#"/* 极简黑白 —— 零圆角单色：黑色引用条 + 下划线链接 */
article { font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; font-size:16px; line-height:1.8; color:#1a1a1a; }
h1 { font-size:28px; font-weight:bold; line-height:1.3; margin:34px 0 16px; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#1a1a1a; }
h2 { font-size:22px; font-weight:bold; line-height:1.3; margin:34px 0 14px; padding-bottom:6px; border-bottom:1px solid #e6e6e6; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#1a1a1a; }
h3 { font-size:19px; font-weight:bold; line-height:1.3; margin:26px 0 10px; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#1a1a1a; }
h4 { font-size:17px; font-weight:bold; line-height:1.3; margin:26px 0 10px; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#1a1a1a; }
p { font-size:16px; line-height:1.8; margin:14px 0; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#1a1a1a; }
blockquote { font-size:16px; line-height:1.8; margin:18px 0; padding:8px 18px; border-left:2px solid #000000; background:transparent; color:#666666; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; }
pre { background:#f0f0f0; padding:14px 16px; border-radius:0; overflow-x:auto; font-size:14px; line-height:1.6; color:#1a1a1a; font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace; }
ul, ol { margin:14px 0; padding-left:26px; line-height:1.8; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#1a1a1a; }
th { border:1px solid #e6e6e6; background:#f0f0f0; padding:8px 12px; text-align:left; font-size:15px; line-height:1.6; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#1a1a1a; }
td { border:1px solid #e6e6e6; padding:8px 12px; text-align:left; font-size:15px; line-height:1.6; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#1a1a1a; }
hr { border:none; border-top:1px solid #e6e6e6; margin:26px 0; }
a { color:#1a1a1a; text-decoration:underline; }
img { max-width:100%; height:auto; border-radius:0; }
strong { color:#000000; }
code { background:#f0f0f0; color:#1a1a1a; padding:1px 6px; border-radius:0; font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace; font-size:0.9em; }
svg { color:#1a1a1a; fill:#1a1a1a; }
.eq-num { display:block; text-align:right; margin:5px 0 0; font-size:15px; color:#666666; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; }
"#;

/// 曜石夜（暗色）：暗色极客风（色板取自站点主题 obsidian）——深底浅字、霓虹青强调。
/// 公众号粘贴后呈深色块；svg 槽位显式浅色 fill，公式无黑块问题。
const OBSIDIAN_CSS: &str = r#"/* 曜石夜 —— 蓝黑深底 + 浅灰正文 + 霓虹青强调（暗色） */
article { font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; font-size:16px; line-height:1.8; color:#d3d8e3; background:#131722; padding:24px 20px; border-radius:8px; }
h1 { font-size:28px; font-weight:bold; line-height:1.3; margin:34px 0 16px; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#f3f5fa; }
h2 { font-size:22px; font-weight:bold; line-height:1.3; margin:34px 0 14px; padding-bottom:6px; border-bottom:1px solid #1f2635; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#f3f5fa; }
h3 { font-size:19px; font-weight:bold; line-height:1.3; margin:26px 0 10px; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#f3f5fa; }
h4 { font-size:17px; font-weight:bold; line-height:1.3; margin:26px 0 10px; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#f3f5fa; }
p { font-size:16px; line-height:1.8; margin:14px 0; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#d3d8e3; }
blockquote { font-size:16px; line-height:1.8; margin:18px 0; padding:8px 18px; border-left:3px solid #b4a4f4; background:#232038; color:#8a93a8; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; }
pre { background:#0a0d14; padding:14px 16px; border-radius:8px; overflow-x:auto; font-size:14px; line-height:1.6; color:#c8d3e0; font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace; }
ul, ol { margin:14px 0; padding-left:26px; line-height:1.8; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#d3d8e3; }
th { border:1px solid #2c3547; background:#1a2030; padding:8px 12px; text-align:left; font-size:15px; line-height:1.6; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#d3d8e3; }
td { border:1px solid #2c3547; padding:8px 12px; text-align:left; font-size:15px; line-height:1.6; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; color:#d3d8e3; }
hr { border:none; border-top:1px solid #2c3547; margin:26px 0; }
a { color:#39c5cf; }
img { max-width:100%; height:auto; border-radius:8px; }
strong { color:#d3d8e3; }
code { background:#0a0d14; color:#c8d3e0; padding:1px 6px; border-radius:4px; font-family:"SFMono-Regular",Consolas,"DejaVu Sans Mono",monospace; font-size:0.9em; }
svg { color:#d3d8e3; fill:#d3d8e3; }
.eq-num { display:block; text-align:right; margin:5px 0 0; font-size:15px; color:#8a93a8; font-family:-apple-system,"PingFang SC","Microsoft YaHei","Helvetica Neue",sans-serif; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_themes_shape_and_order() {
        let themes = builtin();
        assert_eq!(themes.len(), 5);
        let ids: Vec<&str> = themes.iter().map(|t| t["id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["moli", "paper", "citrus", "minimal", "obsidian"]);
        assert_eq!(themes[0]["id"].as_str().unwrap(), DEFAULT_ID);
        let mut uniq = ids.clone();
        uniq.sort_unstable();
        uniq.dedup();
        assert_eq!(uniq.len(), ids.len(), "主题 id 必须唯一");
        for t in &themes {
            assert!(!t["label"].as_str().unwrap().is_empty());
            assert_eq!(t["dark"].as_bool().unwrap(), t["id"].as_str().unwrap() == "obsidian");
        }
    }

    /// 复制脚本依赖的槽位必须齐全，缺一个对应元素就裸奔进公众号。
    #[test]
    fn every_theme_covers_all_slots() {
        const SLOTS: &[&str] = &[
            "article", "h1", "h2", "h3", "h4", "p", "blockquote", "pre", "ul", "ol", "li",
            "table", "th", "td", "hr", "a", "img", "strong", "codeInline", "codeInPre", "svg",
            "eqBlock", "eqSvg", "eqNum",
        ];
        for t in builtin() {
            let styles = t["styles"].as_object().unwrap();
            for slot in SLOTS {
                let v = styles
                    .get(*slot)
                    .unwrap_or_else(|| panic!("{:?} 缺槽位 {slot}", t["id"]));
                assert!(v.as_str().unwrap().contains(':'), "{:?} 槽位 {slot} 不是 CSS", t["id"]);
            }
        }
    }

    /// 墨理蓝的正文/强调色值必须与历史写死版本一致（行为兼容回归锚点）。
    #[test]
    fn moli_matches_legacy_hardcoded_colors() {
        let moli = &builtin()[0];
        let styles = moli["styles"].as_object().unwrap();
        assert!(styles["article"].as_str().unwrap().contains("#20222a"));
        assert!(styles["a"].as_str().unwrap().contains("#0b6fc0"));
        assert!(styles["blockquote"].as_str().unwrap().contains("#3c5a49"));
        assert!(styles["pre"].as_str().unwrap().contains("#f3f1ec"));
    }

    /// 内置四套 CSS 定义的移植锚点：每套抽关键色值/结构，防止 CSS 化时走样。
    #[test]
    fn builtin_css_ports_keep_signature_values() {
        let by_id: BTreeMap<String, Value> = builtin()
            .into_iter()
            .map(|t| (t["id"].as_str().unwrap().to_string(), t))
            .collect();
        let styles = |id: &str| by_id[id]["styles"].as_object().unwrap();

        let paper = styles("paper");
        assert!(paper["article"].as_str().unwrap().contains("Noto Serif SC"));
        assert!(paper["a"].as_str().unwrap().contains("#8c1f28"));
        assert!(paper["h1"].as_str().unwrap().contains("text-align:center"));
        assert!(paper["h2"].as_str().unwrap().contains("border-bottom:1px solid #e3e1dc"));
        assert!(paper["codeInline"].as_str().unwrap().contains("#f7f6f3"));

        let citrus = styles("citrus");
        assert!(citrus["h2"].as_str().unwrap().contains("border-left:4px solid #e2691e"));
        assert!(citrus["h3"].as_str().unwrap().contains("KaiTi"));
        assert!(citrus["a"].as_str().unwrap().contains("#e2691e"));
        assert!(citrus["img"].as_str().unwrap().contains("border-radius:10px"));
        assert!(citrus["blockquote"].as_str().unwrap().contains("#7a8450"));

        let minimal = styles("minimal");
        assert!(minimal["a"].as_str().unwrap().contains("text-decoration:underline"));
        assert!(minimal["blockquote"].as_str().unwrap().contains("border-left:2px solid #000000"));
        assert!(minimal["pre"].as_str().unwrap().contains("border-radius:0"));
        assert!(minimal["strong"].as_str().unwrap().contains("#000000"));
        assert!(minimal["th"].as_str().unwrap().contains("background:#f0f0f0"));

        let obsidian = styles("obsidian");
        assert!(obsidian["article"].as_str().unwrap().contains("background:#131722"));
        assert!(obsidian["a"].as_str().unwrap().contains("#39c5cf"));
        assert!(obsidian["svg"].as_str().unwrap().contains("fill:#d3d8e3"));
        assert!(obsidian["blockquote"].as_str().unwrap().contains("#b4a4f4"));
        assert!(obsidian["h2"].as_str().unwrap().contains("color:#f3f5fa"));
    }

    #[test]
    fn themes_json_is_valid_json_array() {
        let parsed: Value =
            serde_json::from_str(&themes_json_for(Path::new("/nonexistent"))).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), builtin().len());
    }

    /// 注入 JSON 内嵌 `<script>`：值里的 `</script>` 必须被转义，防提前闭合标签。
    #[test]
    fn json_escapes_lt_to_neutralize_script_close() {
        let raw = serde_json::to_string(&builtin()).unwrap();
        let escaped = sanitize(raw);
        let parsed: Value = serde_json::from_str(&escaped).unwrap();
        assert_eq!(parsed.as_array().unwrap().len(), builtin().len(), "转义后仍是等价 JSON");
        assert!(!escaped.contains('<'), "sanitize 后不应残留裸 <");
    }

    // ────────────── CSS 解析器 ──────────────

    #[test]
    fn css_parser_handles_comments_quotes_groups_and_at_rules() {
        let rules = parse_css_rules(
            r#"/* 注释 h2 { color: red } */
            @media (max-width: 640px) { p { color: red; } }
            h2, a { color: #7c3aed; margin: 0 auto; }
            blockquote { font-family: "a;b"; background: url(http://x/y); color: #111 }
            "#,
        );
        let get = |sel: &str| {
            rules
                .iter()
                .find(|(s, _)| s == sel)
                .map(|(_, d)| d.as_str())
                .unwrap_or_else(|| panic!("缺 {sel} 规则"))
        };
        // @规则与注释中的规则整体忽略
        assert!(!rules.iter().any(|(s, _)| s.contains("@media")));
        assert!(!rules.iter().any(|(s, _)| s.contains("注释")));
        // 逗号选择器组拆开、声明去空白并规范分号
        assert_eq!(get("h2"), "color:#7c3aed;margin:0 auto;");
        assert_eq!(get("a"), "color:#7c3aed;margin:0 auto;");
        // 引号内的分号/冒号不切分
        assert!(get("blockquote").contains("font-family:\"a;b\";"));
        assert!(get("blockquote").contains("background:url(http://x/y);"));
        // 无尾分号的最后一条声明也收
        assert!(get("blockquote").contains("color:#111;"));
    }

    #[test]
    fn css_unknown_selectors_are_dropped() {
        let before = base_styles(&default_palette()).len();
        let theme = theme_from_css(
            "t",
            "t",
            false,
            "h2 { color:#123456; } a:hover { color:#000; } .foo { color:#111; }",
        );
        let styles = theme["styles"].as_object().unwrap();
        assert!(styles["h2"].as_str().unwrap().contains("#123456"));
        // a:hover / .foo 不产生槽位；a 槽位保持基底墨理
        assert!(styles["a"].as_str().unwrap().contains("#0b6fc0"));
        assert_eq!(styles.len(), before, "槽位集合不变，仅覆写");
    }

    #[test]
    fn css_meta_directive_parses_label_and_dark() {
        let (label, dark) = css_theme_meta(r#"/* typall-copy-theme: label="学院青" dark */ h2{}"#);
        assert_eq!(label.as_deref(), Some("学院青"));
        assert!(dark);
        let (label, dark) = css_theme_meta("/* typall-copy-theme: dark=true */ p{}");
        assert_eq!(label, None);
        assert!(dark);
        let (label, dark) = css_theme_meta("h2 { color: red }");
        assert_eq!(label, None);
        assert!(!dark);
    }

    // ────────────── 自定义主题装载 ──────────────

    /// 自定义 TOML 令牌主题（保留格式）：palette 覆盖 + styles 覆写。
    #[test]
    fn custom_theme_toml_parses_and_merges() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join(CUSTOM_DIR);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("academy.toml"),
            r##"
label = "学院青"
[palette]
accent = "#0f766e"
h2_left_bar = true
[styles]
h1 = 'font-size:30px;text-align:center;color:#12343b;'
"##,
        )
        .unwrap();
        let themes = all_themes(tmp.path());
        assert_eq!(themes.len(), 6, "5 内置 + 1 自定义");
        let custom = &themes[5];
        assert_eq!(custom["id"].as_str().unwrap(), "academy");
        assert_eq!(custom["label"].as_str().unwrap(), "学院青");
        assert_eq!(custom["dark"].as_bool().unwrap(), false, "dark 缺省 false");
        let styles = custom["styles"].as_object().unwrap();
        assert!(styles["a"].as_str().unwrap().contains("#0f766e"));
        assert!(styles["h2"].as_str().unwrap().contains("border-left:4px solid #0f766e"));
        assert!(styles["h1"].as_str().unwrap().contains("font-size:30px"));
        assert!(styles["p"].as_str().unwrap().contains("#20222a"), "未覆盖槽位回落基底");
    }

    /// 自定义 CSS 主题：文件头指令 + 槽位映射 + 其余回落基底。
    #[test]
    fn custom_css_theme_loads_with_directive() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join(CUSTOM_DIR);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("ocean.css"),
            r#"/* typall-copy-theme: label="深海蓝" */
            /* 注释里的 h2 { color: red } 不生效 */
            h2 { padding-left:12px; border-left:4px solid #2563eb; color:#0f2a5c; }
            a { color:#2563eb; }
            "#,
        )
        .unwrap();
        let themes = all_themes(tmp.path());
        assert_eq!(themes.len(), 6);
        let ocean = &themes[5];
        assert_eq!(ocean["id"].as_str().unwrap(), "ocean");
        assert_eq!(ocean["label"].as_str().unwrap(), "深海蓝");
        assert_eq!(ocean["dark"].as_bool().unwrap(), false);
        let styles = ocean["styles"].as_object().unwrap();
        assert!(styles["h2"].as_str().unwrap().contains("border-left:4px solid #2563eb"));
        assert!(styles["a"].as_str().unwrap().contains("#2563eb"));
        // 注释里的规则未生效；未覆盖槽位回落墨理基底
        assert!(styles["p"].as_str().unwrap().contains("#20222a"));
        assert!(styles["article"].as_str().unwrap().contains("#20222a"));
    }

    /// 非法文件（坏 TOML / id 冲突 / 非法文件名 / dark=true）逐项验证。
    #[test]
    fn custom_theme_rejects_bad_files_and_conflicts() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join(CUSTOM_DIR);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("broken.toml"), "not [valid toml").unwrap();
        std::fs::write(dir.join("moli.toml"), "label = \"冒牌\"").unwrap();
        std::fs::write(dir.join("bad name!.css"), "h2 { color: red }").unwrap();
        std::fs::write(dir.join("ok.toml"), "label = \"正常\"\ndark = true").unwrap();
        let themes = all_themes(tmp.path());
        let ids: Vec<&str> = themes.iter().map(|t| t["id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["moli", "paper", "citrus", "minimal", "obsidian", "ok"]);
        assert_eq!(themes[5]["dark"].as_bool().unwrap(), true);
        let tmp2 = tempfile::tempdir().unwrap();
        let dir2 = tmp2.path().join(CUSTOM_DIR);
        std::fs::create_dir_all(&dir2).unwrap();
        std::fs::write(dir2.join("noname.toml"), "dark = false").unwrap();
        let themes2 = all_themes(tmp2.path());
        assert_eq!(themes2[5]["label"].as_str().unwrap(), "noname");
    }

    /// 目录不存在时 all_themes 与内置一致（零自定义场景不报错）。
    #[test]
    fn missing_custom_dir_yields_builtin_only() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(all_themes(tmp.path()).len(), builtin().len());
    }
}
