//! 内容管理：扫描 `posts/`、`pages/`，提取元数据。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use typst::syntax::ast::{self, AstNode};
use typst::syntax::parse;

/// 元数据值（仅字面量）。
#[derive(Debug, Clone, PartialEq)]
pub enum MetaValue {
    Str(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Array(Vec<MetaValue>),
    None,
}

/// 从 Typst 源码顶层提取 `#let name = value` 形式的元数据。
/// 纯语法层解析，不编译，<1ms。
///
/// 只提取根 Markup 的直接子节点中的 let 绑定，不递归进入函数体或代码块，
/// 避免把 `#let f() = { let x = 1 }` 中的 `x` 误当元数据。
pub fn extract_meta(source: &str) -> HashMap<String, MetaValue> {
    let root = parse(source);
    let mut out = HashMap::new();
    for child in root.children() {
        if let Some(binding) = child.cast::<ast::LetBinding>()
            && let ast::LetBindingKind::Normal(ast::Pattern::Normal(expr)) = binding.kind() {
                let name = expr.to_untyped().full_text().to_string();
                if let Some(init) = binding.init() {
                    out.insert(name, to_value(&init));
                }
            }
    }
    out
}

fn to_value(expr: &ast::Expr) -> MetaValue {
    match expr {
        ast::Expr::Str(s) => MetaValue::Str(s.get().to_string()),
        ast::Expr::Bool(b) => MetaValue::Bool(b.get()),
        ast::Expr::Int(i) => MetaValue::Int(i.get()),
        ast::Expr::Float(f) => MetaValue::Float(f.get()),
        ast::Expr::None(_) => MetaValue::None,
        ast::Expr::Array(arr) => MetaValue::Array(
            arr.items()
                .filter_map(|item| match item {
                    ast::ArrayItem::Pos(expr) => Some(to_value(&expr)),
                    _ => None,
                })
                .collect(),
        ),
        _ => MetaValue::None,
    }
}

/// 一篇文章 / 页面的元数据视图。
#[derive(Debug, Clone, Default)]
pub struct DocumentMeta {
    pub title: String,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub draft: bool,
    /// 手动摘要（`#let excerpt = "..."`），缺省则从正文 HTML 自动截取。
    pub excerpt: Option<String>,
    /// 最后更新时间（`#let updated = "..."`），用于展示修订状态与 sitemap lastmod。
    pub updated: Option<String>,
    /// 专栏名（`#let series = "高中物理"`）。同名文章组成一个课程序列，
    /// 生成 `/series/` 索引与 `/series/<slug>/` 详情页。
    pub series: Option<String>,
    /// 专栏内排序权重（`#let series_weight = 3`），越小越靠前；缺省排最后。
    pub series_weight: Option<i64>,
    /// 旧地址别名（`#let aliases = ("old-slug",)`）：构建时为每条别名生成
    /// 跳转页（meta refresh），slug 变更后旧链接不 404。
    pub aliases: Vec<String>,
    /// 全部原始元数据（含自定义字段，供主题系统 `{{post.raw.<key>}}` 消费）。
    pub raw: HashMap<String, MetaValue>,
}

impl DocumentMeta {
    pub fn from_map(map: &HashMap<String, MetaValue>) -> Self {
        let mut meta = Self {
            raw: map.clone(),
            ..Self::default()
        };

        if let Some(MetaValue::Str(s)) = map.get("title") {
            meta.title = s.clone();
        }
        if let Some(MetaValue::Str(s)) = map.get("date") {
            meta.date = Some(s.clone());
        }
        if let Some(MetaValue::Bool(b)) = map.get("draft") {
            meta.draft = *b;
        }
        if let Some(MetaValue::Array(items)) = map.get("tags") {
            meta.tags = items
                .iter()
                .filter_map(|v| match v {
                    MetaValue::Str(s) => Some(s.clone()),
                    _ => None,
                })
                .collect();
        }
        if let Some(MetaValue::Array(items)) = map.get("categories") {
            meta.categories = items
                .iter()
                .filter_map(|v| match v {
                    MetaValue::Str(s) => Some(s.clone()),
                    _ => None,
                })
                .collect();
        }
        if let Some(MetaValue::Array(items)) = map.get("aliases") {
            meta.aliases = items
                .iter()
                .filter_map(|v| match v {
                    MetaValue::Str(s) => Some(s.trim_matches('/').to_string()),
                    _ => None,
                })
                .filter(|s| !s.is_empty())
                .collect();
        }
        if let Some(MetaValue::Str(s)) = map.get("excerpt") {
            meta.excerpt = Some(s.clone());
        }
        if let Some(MetaValue::Str(s)) = map.get("updated") {
            meta.updated = Some(s.clone());
        }
        if let Some(MetaValue::Str(s)) = map.get("series") {
            meta.series = Some(s.clone());
        }
        // weight 只认 Int；`#let series_weight = 1.5` 属作者笔误，不解析。
        if let Some(MetaValue::Int(i)) = map.get("series_weight") {
            meta.series_weight = Some(*i);
        }
        meta
    }

    /// 返回有效摘要：手动 `excerpt` 优先，否则从正文 HTML 自动截取。
    pub fn effective_excerpt(&self, body_html: &str) -> String {
        if let Some(s) = &self.excerpt {
            s.clone()
        } else {
            auto_excerpt(body_html, 200)
        }
    }
}

/// 从 HTML 中提取纯文本（去标签、解码实体、压缩空白）。供摘要与搜索索引共用。
///
/// 正文 HTML 里的字面 `&` 等已是实体形态，必须在此还原成纯文本：下游渲染层
/// （主题模板/搜索索引）会统一再转义一次，若不解码则页面显示 `&amp;`。
pub fn plain_text(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    let text = crate::site_html::decode_entities(&text);
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// 从 HTML 正文中截取纯文本摘要（去标签，在词边界截断）。
pub fn auto_excerpt(body_html: &str, max_len: usize) -> String {
    let text = plain_text(body_html);
    if text.len() <= max_len {
        return text;
    }
    let mut end = max_len;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    if let Some(last_space) = text[..end].rfind(' ') {
        format!("{}…", &text[..last_space])
    } else {
        format!("{}…", &text[..end])
    }
}

/// 扫描目录下所有 `.typ` 文件（返回绝对路径）。
pub fn scan_typ_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    for entry in walkdir::WalkDir::new(dir) {
        let Ok(entry) = entry else { continue };
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "typ") {
                out.push(path.to_path_buf());
            }
        }
    }
    out.sort();
    out
}

/// 把绝对路径转为相对根路径，统一使用 `/` 分隔符。
pub fn rel_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

/// 从文件路径推导输出 slug（不含扩展名）。
pub fn slug_of(rel: &str) -> String {
    rel.trim_end_matches(".typ").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aliases_parsed_and_normalized() {
        let src = r#"#let title = "t"
#let aliases = ("/old-path/", "second-old", )
正文。
"#;
        let meta = DocumentMeta::from_map(&extract_meta(src));
        assert_eq!(meta.aliases, vec!["old-path", "second-old"]);
    }

    #[test]
    fn aliases_absent_by_default() {
        let src = "#let title = \"t\"\n正文。\n";
        let meta = DocumentMeta::from_map(&extract_meta(src));
        assert!(meta.aliases.is_empty());
    }

    #[test]
    fn extract_meta_parses_common_fields() {
        let src = r#"#let title = "测试"
#let date = "2026-09-01"
#let tags = ("a", "b")
#let categories = ("c",)
#let draft = true
"#;
        let map = extract_meta(src);
        assert_eq!(map.get("title"), Some(&MetaValue::Str("测试".into())));
        assert_eq!(map.get("date"), Some(&MetaValue::Str("2026-09-01".into())));
        assert_eq!(map.get("draft"), Some(&MetaValue::Bool(true)));
        match map.get("tags") {
            Some(MetaValue::Array(items)) => assert_eq!(items.len(), 2),
            other => panic!("tags 应为数组，实际 {other:?}"),
        }
    }

    #[test]
    fn from_map_maps_typed_fields() {
        let mut map = HashMap::new();
        map.insert("title".to_string(), MetaValue::Str("标题".into()));
        map.insert("date".to_string(), MetaValue::Str("2026-09-01".into()));
        map.insert("draft".to_string(), MetaValue::Bool(true));
        map.insert(
            "tags".to_string(),
            MetaValue::Array(vec![MetaValue::Str("a".into()), MetaValue::Str("b".into())]),
        );
        let meta = DocumentMeta::from_map(&map);
        assert_eq!(meta.title, "标题");
        assert_eq!(meta.date, Some("2026-09-01".into()));
        assert!(meta.draft);
        assert_eq!(meta.tags, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn slug_of_strips_extension() {
        assert_eq!(slug_of("posts/foo.typ"), "posts/foo");
        assert_eq!(slug_of("pages/about.typ"), "pages/about");
    }

    #[test]
    fn rel_path_normalizes_separators() {
        let root = PathBuf::from("/tmp/blog");
        let path = PathBuf::from("/tmp/blog/posts/a.typ");
        assert_eq!(rel_path(&root, &path), "posts/a.typ");
    }

    #[test]
    fn auto_excerpt_strips_tags_and_truncates() {
        let html = "<p>Hello <strong>world</strong>. This is a test.</p>";
        // 去标签后 `world</strong>.` 连写为 `world.`（标签移除不留空格）
        assert_eq!(auto_excerpt(html, 100), "Hello world. This is a test.");
        let long = "<p>".to_string() + &"word ".repeat(100) + "</p>";
        let excerpt = auto_excerpt(&long, 50);
        assert!(excerpt.len() <= 54);
        assert!(excerpt.ends_with('…'));
    }

    #[test]
    fn auto_excerpt_handles_empty_and_short_input() {
        assert_eq!(auto_excerpt("", 200), "");
        assert_eq!(auto_excerpt("<p>Hi</p>", 200), "Hi");
    }

    #[test]
    fn plain_text_strips_tags_and_collapses_whitespace() {
        assert_eq!(plain_text("<p>Hello <strong>world</strong>.</p>"), "Hello world.");
        assert_eq!(plain_text("<div>a\n  b\t c</div>"), "a b c");
        assert_eq!(plain_text(""), "");
        assert_eq!(plain_text("no tags"), "no tags");
    }

    #[test]
    fn from_map_extracts_excerpt() {
        let mut map = HashMap::new();
        map.insert("excerpt".to_string(), MetaValue::Str("摘要内容".into()));
        let meta = DocumentMeta::from_map(&map);
        assert_eq!(meta.excerpt, Some("摘要内容".to_string()));
        assert_eq!(meta.effective_excerpt("<p>ignored</p>"), "摘要内容");
    }

    #[test]
    fn effective_excerpt_falls_back_to_auto() {
        let meta = DocumentMeta { excerpt: None, ..DocumentMeta::default() };
        assert_eq!(meta.effective_excerpt("<p>自动摘要</p>"), "自动摘要");
    }

    #[test]
    fn plain_text_decodes_entities_once() {
        // 正文 HTML 的字面 & 是实体形态：提取纯文本时必须还原，
        // 否则下游模板再转义一次会显示成 &amp;（双重转义）。
        assert_eq!(plain_text("<p>a &amp; b</p>"), "a & b");
        assert_eq!(plain_text("<p>A &lt; B</p>"), "A < B");
        // 不级联解码：&amp;lt; 还原为 &lt; 而非 <
        assert_eq!(plain_text("<p>&amp;lt;</p>"), "&lt;");
        // &nbsp; 参与空白归一化
        assert_eq!(plain_text("<p>x&nbsp;&nbsp;y</p>"), "x y");
    }

    #[test]
    fn series_meta_parses() {
        let src = r#"#let title = "t"
#let series = "高中物理"
#let series_weight = 3
"#;
        let meta = DocumentMeta::from_map(&extract_meta(src));
        assert_eq!(meta.series.as_deref(), Some("高中物理"));
        assert_eq!(meta.series_weight, Some(3));

        // 缺省 / 类型不符：series 缺省 None；Float weight 不解析。
        let none = DocumentMeta::from_map(&extract_meta("#let title = \"t\"\n"));
        assert_eq!(none.series, None);
        assert_eq!(none.series_weight, None);
        let float = DocumentMeta::from_map(&extract_meta("#let title = \"t\"\n#let series_weight = 1.5\n"));
        assert_eq!(float.series_weight, None);
    }
}
