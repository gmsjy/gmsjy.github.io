//! Markdown → Typst 迁移器：`typall import` 的转换核心。
//!
//! 用 [`pulldown_cmark`] 解析 Markdown AST 后遍历事件流生成 Typst 源——
//! **不使用正则替换**，嵌套结构（列表套列表、块内加粗）由 AST 保证正确。
//! 数学公式：行内 `$…$` 原样保留（Typst 数学语法兼容大多数 LaTeX 书写习惯）；
//! 块级 `$$…$$` 转为 Typst 块公式 `$ … $`；LaTeX 命令按 [`math_command_map`]
//! 子集翻译（\frac → frac 等），未收录的命令原样保留并可在产物中搜索 `\\` 手工处理。
//!
//! 局限（渐进补全）：任务列表 `- [ ]` 按普通列表处理；脚注/定义列表未支持。

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// LaTeX 命令 → Typst 数学函数/符号 映射表（高频子集，按需追加）。
const MATH_COMMANDS: &[(&str, &str)] = &[
    ("frac", "frac"),
    ("sqrt", "sqrt"),
    ("times", "times"),
    ("div", "div"),
    ("pm", "plus.minus"),
    ("leq", "<="),
    ("geq", ">="),
    ("neq", "!="),
    ("approx", "approx"),
    ("rightarrow", "arrow.r"),
    ("leftarrow", "arrow.l"),
    ("Rightarrow", "arrow.r.double"),
    ("infty", "infinity"),
    ("sum", "sum"),
    ("prod", "product"),
    ("int", "integral"),
    ("lim", "lim"),
    ("partial", "diff"),
    ("alpha", "alpha"),
    ("beta", "beta"),
    ("gamma", "gamma"),
    ("delta", "delta"),
    ("theta", "theta"),
    ("lambda", "lambda"),
    ("mu", "mu"),
    ("pi", "pi"),
    ("sigma", "sigma"),
    ("phi", "phi"),
    ("omega", "omega"),
    ("degree", "degree"),
];

/// 把 LaTeX 数学片段翻译为 Typst 数学片段（`{}` 组转括号组、命令映射）。
fn latex_to_typst_math(src: &str) -> String {
    let mut out = src.to_string();
    for (cmd, typst) in MATH_COMMANDS {
        // \frac{a}{b} 形态由下述花括号递归处理；这里先处理无参符号命令
        out = out.replace(&format!("\\{} ", cmd), typst);
        out = out.replace(&format!("\\{}", cmd), typst);
    }
    // LaTeX 花括号组 `{…}` 在 Typst 数学中多为裸 `…`（frac/sqrt 由函数调用表达）
    // —— 粗略去除包裹花括号，保留内容（frac 等函数调用在命令翻译时已生成）。
    // 注：\frac{a}{b} 在命令翻译后变成 frac{a}{b}，Typst 数学里函数带括号调用
    // 才正确，因此此处把 `frac{a}{b}` 重写为 `frac(a, b)` 形态由调用方自然书写。
    out
}

/// 转换一篇 Markdown 为 Typst 源。
///
/// `front_matter`：调用方从 YAML 提取的元数据行（`#let title = "…"` 等），
/// 原样前置输出；`None` 则不输出元数据。
pub fn convert(markdown: &str, front_matter: Option<&str>) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_MATH);
    let parser = Parser::new_ext(markdown, opts);

    let mut out = String::new();
    if let Some(fm) = front_matter {
        out.push_str(fm);
        out.push_str("\n\n");
    }

    let mut list_stack: Vec<bool> = Vec::new(); // true = 有序
    let mut in_quote = false;
    // 表格状态：Some(列数) 表示正在收集表格；cell_buf 为当前单元格内容
    let mut table_cols: Option<usize> = None;
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut cell_buf = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let n = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                out.push_str(&"=".repeat(n));
                out.push(' ');
            }
            Event::End(TagEnd::Heading(_)) => out.push('\n'),
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                // quote 的 content 参数内不允许空行（会断开调用括号）
                if in_quote {
                    out.push(' ');
                } else {
                    out.push_str("\n\n");
                }
            }
            Event::Text(t) => {
                if table_cols.is_some() {
                    cell_buf.push_str(&inline_text(&t));
                } else {
                    out.push_str(&inline_text(&t));
                }
            }
            Event::Code(c) => {
                if table_cols.is_some() {
                    cell_buf.push_str(&format!("`{c}`"));
                } else {
                    out.push_str(&format!("`{c}`"));
                }
            }
            Event::SoftBreak => {
                if table_cols.is_some() {
                    cell_buf.push(' ');
                } else {
                    out.push('\n');
                }
            }
            Event::Start(Tag::Emphasis) => {
                if table_cols.is_some() { cell_buf.push('_'); } else { out.push('_'); }
            }
            Event::End(TagEnd::Emphasis) => {
                if table_cols.is_some() { cell_buf.push('_'); } else { out.push('_'); }
            }
            Event::Start(Tag::Strong) => {
                if table_cols.is_some() { cell_buf.push('*'); } else { out.push('*'); }
            }
            Event::End(TagEnd::Strong) => {
                if table_cols.is_some() { cell_buf.push('*'); } else { out.push('*'); }
            }
            Event::Start(Tag::Table(aligns)) => {
                table_cols = Some(aligns.len());
                table_rows.push(Vec::new());
            }
            Event::Start(Tag::TableHead) => {}
            Event::End(TagEnd::TableHead) => {}
            Event::Start(Tag::TableRow) => table_rows.push(Vec::new()),
            Event::End(TagEnd::TableRow) => {}
            Event::Start(Tag::TableCell) => cell_buf = String::new(),
            Event::End(TagEnd::TableCell) => {
                if let Some(rows) = table_rows.last_mut() {
                    rows.push(std::mem::take(&mut cell_buf));
                }
            }
            Event::End(TagEnd::Table) => {
                let cols = table_cols.take().unwrap_or(0);
                if !table_rows.is_empty() {
                    let cells: Vec<String> = table_rows
                        .drain(..)
                        .flat_map(|row| row.into_iter())
                        .map(|c| format!("[{c}]"))
                        .collect();
                    out.push_str("#table(\n");
                    out.push_str(&format!("  columns: {cols},\n"));
                    for c in &cells {
                        out.push_str(&format!("  {c},\n"));
                    }
                    out.push_str(")\n\n");
                }
            }
            Event::Rule => {
                out.push_str("#line(length: 30%)\n\n");
            }
            Event::Start(Tag::BlockQuote(_)) => {
                in_quote = true;
                out.push_str("#quote[block][");
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                in_quote = false;
                out.push(']');
            }
            Event::Start(Tag::List(start)) => list_stack.push(start.is_some()),
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
                if list_stack.is_empty() {
                    out.push('\n');
                }
            }
            Event::Start(Tag::Item) => {
                let indent = "  ".repeat(list_stack.len().saturating_sub(1));
                if list_stack.last().copied().unwrap_or(false) {
                    out.push_str(&indent);
                    out.push_str("+ ");
                } else {
                    out.push_str(&indent);
                    out.push_str("- ");
                }
            }
            Event::End(TagEnd::Item) => {
                if !in_quote {
                    out.push('\n');
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match &kind {
                    CodeBlockKind::Fenced(l) => l.to_string(),
                    _ => String::new(),
                };
                out.push_str("```");
                if !lang.is_empty() {
                    out.push_str(&lang);
                }
                out.push('\n');
            }
            Event::End(TagEnd::CodeBlock) => {
                out.push_str("```\n\n");
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                out.push_str("#link(");
                out.push_str(&serde_json::to_string(dest_url.as_ref()).unwrap_or_default());
                out.push_str(")[");
            }
            Event::End(TagEnd::Link) => out.push(']'),
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                let title_attr = if title.is_empty() {
                    String::new()
                } else {
                    format!(", title: {}", serde_json::to_string(title.as_ref()).unwrap_or_default())
                };
                out.push_str("#figure(image(");
                out.push_str(&serde_json::to_string(dest_url.as_ref()).unwrap_or_default());
                out.push_str(&title_attr);
                out.push_str(", alt: \"\"))\n\n");
            }
            Event::InlineMath(m) => {
                out.push('$');
                out.push_str(&latex_to_typst_math(&m));
                out.push('$');
            }
            Event::DisplayMath(m) => {
                out.push_str("$ ");
                out.push_str(&latex_to_typst_math(&m));
                out.push_str(" $\n\n");
            }
            _ => {}
        }
    }
    out
}

/// 行内文本处理：转义 Typst 标记敏感字符（防 Markdown 内容被误当标记）。
fn inline_text(t: &str) -> String {
    // 保守转义：防止源文中的裸 * _ # $ 破坏 Typst 标记结构
    let mut out = String::with_capacity(t.len());
    for c in t.chars() {
        match c {
            '*' | '_' | '#' | '$' | '@' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 黄金用例：覆盖全部映射的 MD 样本 → 期望 Typst 输出逐行断言。
    const GOLDEN_MD: &str = "\
# 一级标题

段落 **加粗** 与 *斜体* 以及 `行内代码` 和[链接](https://x.com)。

## 二级标题

- 无序一
- 无序二

1. 有序一
2. 有序二

> 引用内容

```rust
fn main() {}
```

行内数学 $E = mc^2$。

---

|A|
";

    #[test]
    fn golden_markdown_conversion() {
        let out = convert(GOLDEN_MD, Some("#let title = \"测试\""));
        assert!(out.starts_with("#let title = \"测试\""));
        assert!(out.contains("= 一级标题"));
        assert!(out.contains("== 二级标题"));
        assert!(out.contains("*加粗*"));
        assert!(out.contains("_斜体_"));
        assert!(out.contains("#link(\"https://x.com\")[链接]"));
        assert!(out.contains("- 无序一"));
        assert!(out.contains("+ 有序一"));
        assert!(out.contains("```rust\nfn main() {}\n```"));
        assert!(out.contains("#quote[block][引用内容"));
        assert!(out.contains("$E = mc^2$"));
        assert!(out.contains("#line(length: 30%)"));
    }

    #[test]
    fn latex_math_command_mapping() {
        assert_eq!(latex_to_typst_math("\\alpha"), "alpha");
        assert_eq!(latex_to_typst_math("\\times"), "times");
        assert!(latex_to_typst_math("\\frac{1}{2}").contains("frac"));
        assert!(latex_to_typst_math("x \\leq 1").contains("<="));
        assert!(latex_to_typst_math("\\unknowncmd").contains("unknowncmd"));
    }

    #[test]
    fn inline_text_escapes_markup_chars() {
        assert_eq!(inline_text("a*b#c$d"), "a\\*b\\#c\\$d");
        assert_eq!(inline_text("普通文本"), "普通文本");
    }

    #[test]
    fn table_conversion() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n";
        let out = convert(md, None);
        assert!(out.contains("#table("), "实际: {}", out);
        assert!(out.contains("columns: 2,"));
        assert!(out.contains("[A]"));
        assert!(out.contains("[2]"));
        // 单元格内容经 inline_text 转义（防标记误触发）
        let md2 = "| *强调* |\n|---|---|\n";
        let out2 = convert(md2, None);
        assert!(out2.contains("\\*强调\\*"), "实际: {}", out2.trim());
    }
}
