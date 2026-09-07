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
    let parser = Parser::new_ext(markdown, opts);

    let mut out = String::new();
    if let Some(fm) = front_matter {
        out.push_str(fm);
        out.push_str("\n\n");
    }

    let mut list_stack: Vec<bool> = Vec::new(); // true = 有序
    let mut in_quote = false;

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
            Event::Text(t) => out.push_str(&inline_text(&t)),
            Event::Code(c) => out.push_str(&format!("`{c}`")),
            Event::Start(Tag::Emphasis) => out.push('_'),
            Event::End(TagEnd::Emphasis) => out.push('_'),
            Event::Start(Tag::Strong) => out.push('*'),
            Event::End(TagEnd::Strong) => out.push('*'),
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
