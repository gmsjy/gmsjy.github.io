//! Markdown → Typst 迁移器：`typall import` 的转换核心。
//!
//! 用 [`pulldown_cmark`] 解析 Markdown AST 后遍历事件流生成 Typst 源——
//! **不使用正则替换**，嵌套结构（列表套列表、块内加粗）由 AST 保证正确。
//! 数学公式：行内 `$…$` 原样保留（Typst 数学语法兼容大多数 LaTeX 书写习惯）；
//! 块级 `$$…$$` 转为 Typst 块公式 `$ … $`；LaTeX 命令按 [`math_command_map`]
//! 子集翻译（\frac → frac 等），未收录的命令原样保留并可在产物中搜索 `\\` 手工处理。
//!
//! 支持：表格 / 删除线 `#strike[..]` / 任务列表（☑/☐ 前缀）/ 脚注
//! （定义内联到引用处 `#footnote[..]`）/ 块级 HTML（保留为 Typst 注释，
//! 便于手工处理）。局限（渐进补全）：行内 HTML 标签剥离仅保留文本；
//! 定义列表未支持。

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::collections::HashMap;

/// LaTeX 命令 → Typst 数学函数/符号 映射表（高频子集，按需追加）。
/// 带参数的命令（frac/sqrt/bold…）在此映射名字，括号形态由
/// [`rewrite_math_calls`] 把 `frac{a}{b}` 重写为 `frac(a, b)`。
const MATH_COMMANDS: &[(&str, &str)] = &[
    ("frac", "frac"),
    ("dfrac", "frac"),
    ("tfrac", "frac"),
    ("binom", "binom"),
    ("sqrt", "sqrt"),
    ("vec", "vec"),
    ("hat", "hat"),
    ("bar", "bar"),
    ("tilde", "tilde"),
    ("dot", "dot"),
    ("ddot", "dot.double"),
    ("overline", "overline"),
    ("underline", "underline"),
    ("bold", "bold"),
    ("mathbf", "bold"),
    ("mathrm", "upright"),
    ("mathbb", "bb"),
    ("mathcal", "cal"),
    ("mathfrak", "frak"),
    ("times", "times"),
    ("div", "div"),
    ("pm", "plus.minus"),
    ("mp", "plus.minus"),
    ("cdot", "dot.op"),
    ("ast", "plus.dot"),
    ("leq", "<="),
    ("geq", ">="),
    ("neq", "!="),
    ("approx", "approx"),
    ("equiv", "equiv"),
    ("sim", "sim"),
    ("propto", "prop"),
    ("infty", "infinity"),
    ("partial", "diff"),
    ("nabla", "nabla"),
    ("rightarrow", "arrow.r"),
    ("to", "arrow.r"),
    ("longrightarrow", "arrow.r"),
    ("leftarrow", "arrow.l"),
    ("Leftarrow", "arrow.l.double"),
    ("Rightarrow", "arrow.r.double"),
    ("Leftrightarrow", "arrow.l.r.double"),
    ("sum", "sum"),
    ("prod", "product"),
    ("int", "integral"),
    ("iint", "integral.double"),
    ("oint", "integral.cont"),
    ("lim", "lim"),
    ("max", "max"),
    ("min", "min"),
    ("log", "log"),
    ("ln", "ln"),
    ("lg", "log"),
    ("exp", "exp"),
    ("sin", "sin"),
    ("cos", "cos"),
    ("tan", "tan"),
    ("cot", "cot"),
    ("sec", "sec"),
    ("csc", "csc"),
    ("arcsin", "arcsin"),
    ("arccos", "arccos"),
    ("arctan", "arctan"),
    ("deg", "deg"),
    ("alpha", "alpha"),
    ("beta", "beta"),
    ("gamma", "gamma"),
    ("delta", "delta"),
    ("epsilon", "epsilon"),
    ("varepsilon", "epsilon.alt"),
    ("zeta", "zeta"),
    ("eta", "eta"),
    ("theta", "theta"),
    ("iota", "iota"),
    ("kappa", "kappa"),
    ("lambda", "lambda"),
    ("mu", "mu"),
    ("nu", "nu"),
    ("xi", "xi"),
    ("pi", "pi"),
    ("rho", "rho"),
    ("sigma", "sigma"),
    ("tau", "tau"),
    ("upsilon", "upsilon"),
    ("phi", "phi"),
    ("varphi", "phi.alt"),
    ("chi", "chi"),
    ("psi", "psi"),
    ("omega", "omega"),
    ("Gamma", "Gamma"),
    ("Delta", "Delta"),
    ("Theta", "Theta"),
    ("Lambda", "Lambda"),
    ("Xi", "Xi"),
    ("Pi", "Pi"),
    ("Sigma", "Sigma"),
    ("Upsilon", "Upsilon"),
    ("Phi", "Phi"),
    ("Psi", "Psi"),
    ("Omega", "Omega"),
    ("forall", "forall"),
    ("exists", "exists"),
    ("in", "in"),
    ("notin", "in.not"),
    ("subset", "subset"),
    ("subseteq", "subset.eq"),
    ("supset", "superset"),
    ("supseteq", "superset.eq"),
    ("cup", "union"),
    ("cap", "inter"),
    ("emptyset", "emptyset"),
    ("degree", "degree"),
    ("circ", "circle.big"),
    ("prime", "prime"),
    ("left", ""),
    ("right", ""),
    ("big", ""),
    ("Big", ""),
    ("bigl", ""),
    ("bigr", ""),
    ("Bigl", ""),
    ("Bigr", ""),
    ("displaystyle", ""),
    ("limits", ""),
    ("quad", "quad"),
    ("qquad", "wide"),
];

/// 这些词是 Typst/LaTeX 共识的函数名或下标词，多字母拆分时保留原样。
const KNOWN_MATH_WORDS: &[&str] = &[
    "sin", "cos", "tan", "cot", "sec", "csc", "sinh", "cosh", "tanh", "coth", "arcsin",
    "arccos", "arctan", "exp", "log", "ln", "lg", "lim", "sup", "inf", "max", "min", "mod",
    "gcd", "det", "deg", "dim", "arg", "Pr", "limsup", "liminf", "arcsec", "arccot",
];

/// 把 LaTeX 数学片段翻译为 Typst 数学片段。
///
/// 管线：多字母变量拆分（`mc^2` → `m c^2`，Typst 数学把多字母串当未知变量，
/// 拆分需在反斜杠命令仍在时进行且放过 `\begin{pmatrix}` 环境标签）→
/// 矩阵/行内环境（`\begin{pmatrix}…\end{pmatrix}` → `mat(...)`）→
/// 命令映射（`\alpha` → `alpha`）→ 花括号参数重写（`frac{a}{b}` → `frac(a, b)`）。
fn latex_to_typst_math(src: &str) -> String {
    let out = split_letter_runs(src);
    let out = rewrite_math_envs(&out);
    let out = translate_commands(&out);
    rewrite_math_calls(&out)
}

/// 把连续 2 个以上的拉丁字母串拆成单字母空格分隔（`mc` → `m c`），
/// 已知函数名（`sin` 等）、反斜杠命令（`\alpha`）与 `\begin{…}`/`\end{…}`
/// 环境标签（含花括号内名字）原样保留。
fn split_letter_runs(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let mut out = String::with_capacity(src.len() + 8);
    let mut i = 0usize;
    while i < chars.len() {
        // 环境标签整段保留（环境名被拆掉会让 rewrite_math_envs 认不出）
        if chars[i] == '\\'
            && (chars[i..].starts_with(&['\\', 'b', 'e', 'g', 'i', 'n'])
                || chars[i..].starts_with(&['\\', 'e', 'n', 'd']))
        {
            let mut j = i;
            while j < chars.len() && chars[j] != '}' {
                out.push(chars[j]);
                j += 1;
            }
            if j < chars.len() {
                out.push('}');
                j += 1;
            }
            i = j;
            continue;
        }
        if chars[i].is_ascii_alphabetic() {
            // 词首必须不在反斜杠命令内
            let word_start_ok = i == 0 || chars[i - 1] != '\\';
            let mut j = i;
            while j < chars.len() && chars[j].is_ascii_alphabetic() {
                j += 1;
            }
            let word: String = chars[i..j].iter().collect();
            if word_start_ok && word.len() > 1 && !KNOWN_MATH_WORDS.contains(&word.as_str()) {
                for (k, c) in chars[i..j].iter().enumerate() {
                    if k > 0 {
                        out.push(' ');
                    }
                    out.push(*c);
                }
            } else {
                out.push_str(&word);
            }
            i = j;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

fn translate_commands(src: &str) -> String {
    let mut out = src.to_string();
    // 按命令名长度降序替换：`String::replace` 无词边界检查，短名先命中会把
    // 长命令截断成无效残留（`\qquad` → `quadad`、`\subseteq` → `subseteq`、
    // `\bigl(` → `l(`），长名必须先处理。
    let mut ordered: Vec<&(&str, &str)> = MATH_COMMANDS.iter().collect();
    ordered.sort_by_key(|(cmd, _)| std::cmp::Reverse(cmd.len()));
    for (cmd, typst) in ordered {
        out = out.replace(&format!("\\{}", cmd), typst);
    }
    out
}

/// `\begin{env} … \end{env}` → Typst 调用（pmatrix/bmatrix/vmatrix/Bmatrix/
/// matrix → `mat(...)`，cases → `cases(...)`，aligned/align/cases* 去壳保留内容）。
/// 行按 `\\`、单元格按 `&` 分隔。逐个最内层处理直到无环境残留。
fn rewrite_math_envs(src: &str) -> String {
    let mut out = src.to_string();
    while let Some(begin) = out.find("\\begin{") {
        let Some(name_end) = out[begin + 7..].find('}') else { break };
        let name = out[begin + 7..begin + 7 + name_end].to_string();
        let open_tag = format!("\\begin{{{}}}", name);
        let end_tag = format!("\\end{{{}}}", name);
        let body_start = begin + open_tag.len();
        let Some(rel_end) = out[body_start..].find(&end_tag) else { break };
        let body = out[body_start..body_start + rel_end].to_string();
        let after = body_start + rel_end + end_tag.len();

        let replaced = match name.as_str() {
            "pmatrix" | "bmatrix" | "Bmatrix" | "vmatrix" | "matrix" | "smallmatrix" => {
                let delim = match name.as_str() {
                    "pmatrix" | "smallmatrix" => "\"(\"",
                    "bmatrix" => "\"[\"",
                    "Bmatrix" => "\"{\"",
                    "vmatrix" => "\"|\"",
                    _ => "none",
                };
                format!("mat(delim: {}, {})", delim, latex_rows_to_args(&body))
            }
            "cases" | "dcases" => format!("cases({})", latex_rows_to_args(&body)),
            // 未知环境同样去壳——保留原标记会让 find 死循环（内容不再变化）
            _ => body.to_string(),
        };
        out.replace_range(begin..after, &replaced);
    }
    out
}

/// LaTeX 矩阵体（`a & b \\ c & d`）→ Typst 参数（`a, b; c, d`）。
fn latex_rows_to_args(body: &str) -> String {
    body.split("\\\\")
        .map(|row| {
            row.split('&')
                .map(|c| c.trim())
                .filter(|c| !c.is_empty())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .filter(|row| !row.is_empty())
        .collect::<Vec<_>>()
        .join("; ")
}

/// 花括号参数 → 函数调用参数：`frac{a}{b}` → `frac(a, b)`、
/// `sqrt(n){x}` → `sqrt(n, x)`（根指数已带括号时）、`bold{x}` → `bold(x)`。
/// 反复从最内层重写直到无匹配（嵌套 `\frac{1}{\sqrt{2}}` 需多轮）。
fn rewrite_math_calls(src: &str) -> String {
    const CALL_CMDS: &[&str] = &[
        "frac", "binom", "sqrt", "vec", "hat", "bar", "tilde", "dot", "dot.double",
        "overline", "underline", "bold", "upright", "bb", "cal", "frak",
    ];
    let mut out = src.to_string();
    'outer: loop {
        // 根指数特例：sqrt[n]{x} → sqrt(n, x)
        if let Some(start) = out.find("sqrt[") {
            let rest = &out[start + 5..];
            if let Some(close) = rest.find(']')
                && rest[close + 1..].starts_with('{')
                && let Some((consumed, args)) = collect_brace_groups(&rest[close + 1..], 1)
            {
                let call =
                    format!("sqrt({}, {})", rest[..close].trim(), args.join(", "));
                out.replace_range(start..start + 5 + close + 1 + consumed, &call);
                continue 'outer;
            }
        }
        for cmd in CALL_CMDS {
            let needle = format!("{cmd}{{");
            if let Some(start) = out.find(&needle) {
                // 从命令名之后收集全部连续花括号组（首组以 `{` 开头）
                if let Some((consumed, args)) = collect_brace_groups(&out[start + cmd.len()..], 2) {
                    let call = format!("{cmd}({})", args.join(", "));
                    out.replace_range(start..start + cmd.len() + consumed, &call);
                    continue 'outer;
                }
            }
        }
        break;
    }
    out
}

/// 从 s 开头收集至多 max 组平衡的 `{…}`，返回 (消耗字节数, 各组内容)。
fn collect_brace_groups(s: &str, max: usize) -> Option<(usize, Vec<String>)> {
    let bytes = s.as_bytes();
    let mut groups = Vec::new();
    let mut i = 0usize;
    while groups.len() < max {
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'{' {
            break;
        }
        let mut depth = 0i32;
        let start = i + 1;
        let mut j = i;
        while j < bytes.len() {
            match bytes[j] {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            j += 1;
        }
        if depth != 0 {
            return None;
        }
        groups.push(s[start..j].to_string());
        i = j + 1;
    }
    if groups.is_empty() {
        None
    } else {
        Some((i, groups))
    }
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
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_TASKLISTS);
    let events: Vec<Event<'_>> = Parser::new_ext(markdown, opts).collect();

    // 脚注定义预扫：Typst 没有独立脚注定义区，把定义内容内联到引用处
    let mut footnotes: HashMap<String, String> = HashMap::new();
    let mut i = 0usize;
    while i < events.len() {
        if let Some(Event::Start(Tag::FootnoteDefinition(label))) = events.get(i) {
            let mut j = i + 1;
            while j < events.len() && !matches!(events[j], Event::End(TagEnd::FootnoteDefinition)) {
                j += 1;
            }
            let body = render_events(&events[i + 1..j], &HashMap::new());
            footnotes.insert(label.to_string(), body.trim().to_string());
            i = j + 1;
        } else {
            i += 1;
        }
    }

    let mut out = String::new();
    if let Some(fm) = front_matter {
        out.push_str(fm);
        out.push_str("\n\n");
    }
    out.push_str(&render_events(&events, &footnotes));
    out
}

/// 遍历事件流生成 Typst 源；`footnotes` 为脚注定义表（label → Typst 行内内容）。
fn render_events(events: &[Event<'_>], footnotes: &HashMap<String, String>) -> String {
    let mut out = String::new();

    let mut list_stack: Vec<bool> = Vec::new(); // true = 有序
    let mut in_quote = false;
    // 代码块状态：内容缓冲原样输出（不做标记转义），围栏长度随内容适配
    // （Markdown 嵌套围栏里的 ``` 行原样保留，外层用 ```` 包裹才不被截断）
    let mut in_code = false;
    let mut code_lang = String::new();
    let mut code_buf = String::new();
    // 表格状态：Some(列数) 表示正在收集表格；cell_buf 为当前单元格内容
    let mut table_cols: Option<usize> = None;
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut cell_buf = String::new();

    for event in events {
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
                if in_code {
                    code_buf.push_str(t);
                } else if table_cols.is_some() {
                    cell_buf.push_str(&inline_text(t));
                } else {
                    out.push_str(&inline_text(t));
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
            Event::Start(Tag::Strikethrough) => {
                if table_cols.is_some() { cell_buf.push_str("#strike["); } else { out.push_str("#strike["); }
            }
            Event::End(TagEnd::Strikethrough) => {
                if table_cols.is_some() { cell_buf.push(']'); } else { out.push(']'); }
            }
            Event::TaskListMarker(checked) => {
                out.push_str(if *checked { "☑ " } else { "☐ " });
            }
            Event::FootnoteReference(label) => {
                let body = footnotes.get(label.as_ref()).map(String::as_str).unwrap_or("");
                out.push_str(&format!("#footnote[{body}]"));
            }
            Event::Start(Tag::FootnoteDefinition(_)) | Event::End(TagEnd::FootnoteDefinition) => {
                // 定义已内联到引用处，原文位置跳过
            }
            Event::Html(html) => {
                // 块级 HTML 无法表达为 Typst，保留原文为注释便于手工处理
                for line in html.lines() {
                    out.push_str("// ");
                    out.push_str(line);
                    out.push('\n');
                }
                out.push('\n');
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
                // block 是 quote 的命名参数；先前的 `#quote[block][..]` 不是合法 Typst
                out.push_str("#quote(block: true)[");
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                in_quote = false;
                out.push_str("]\n\n");
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
                in_code = true;
                code_buf.clear();
                code_lang = match &kind {
                    CodeBlockKind::Fenced(l) => l.to_string(),
                    _ => String::new(),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code = false;
                // 围栏至少 3 个反引号；内容含连续反引号时加长以免被截断
                let mut max_run = 0usize;
                let mut run = 0usize;
                for c in code_buf.chars() {
                    if c == '`' {
                        run += 1;
                        max_run = max_run.max(run);
                    } else {
                        run = 0;
                    }
                }
                let fence = "`".repeat((max_run + 1).max(3));
                out.push_str(&fence);
                if !code_lang.is_empty() {
                    out.push_str(&code_lang);
                }
                out.push('\n');
                out.push_str(&code_buf);
                if !code_buf.ends_with('\n') {
                    out.push('\n');
                }
                out.push_str(&fence);
                out.push_str("\n\n");
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                out.push_str("#link(");
                out.push_str(&serde_json::to_string(dest_url.as_ref()).unwrap_or_default());
                out.push_str(")[");
            }
            Event::End(TagEnd::Link) => out.push(']'),
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                let url = dest_url.as_ref();
                // 远程图片：typst image() 只读本地文件，改为注释 + 链接占位
                if url.starts_with("http://") || url.starts_with("https://") {
                    out.push_str(&format!(
                        "// 远程图片（需手动下载到本地后改为 image()）：{url}\n#link(\"{url}\")[查看图片]\n\n"
                    ));
                } else {
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
            }
            Event::InlineMath(m) => {
                out.push('$');
                out.push_str(&latex_to_typst_math(m));
                out.push('$');
            }
            Event::DisplayMath(m) => {
                out.push_str("$ ");
                out.push_str(&latex_to_typst_math(m));
                out.push_str(" $\n\n");
            }
            _ => {}
        }
    }
    out
}

/// 行内文本处理：转义 Typst 标记敏感字符（防 Markdown 内容被误当标记）。
fn inline_text(t: &str) -> String {
    // 保守转义：防止源文中的裸 * _ # $ @ [ ] 破坏 Typst 标记结构
    // （[ ] 是内容块定界符——真实博客里的 `[object Number]` 之类文本必炸编译）
    let mut out = String::with_capacity(t.len());
    for c in t.chars() {
        match c {
            '*' | '_' | '#' | '$' | '@' | '[' | ']' => {
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
        assert!(out.contains("#quote(block: true)[引用内容"), "实际: {out}");
        assert!(out.contains("$E = m c^2$"), "实际: {out}");
        assert!(out.contains("#line(length: 30%)"));
    }

    #[test]
    fn latex_prefix_commands_not_truncated() {
        // 回归：短名先于长名替换会把长命令截断成无效残留
        assert_eq!(latex_to_typst_math("\\qquad x"), "wide x");
        assert_eq!(latex_to_typst_math("A \\subseteq B"), "A subset.eq B");
        assert_eq!(latex_to_typst_math("A \\supseteq B"), "A superset.eq B");
        assert_eq!(latex_to_typst_math("\\bigl(x\\bigr)"), "(x)");
    }

    #[test]
    fn latex_math_command_mapping() {
        assert_eq!(latex_to_typst_math("\\alpha"), "alpha");
        assert_eq!(latex_to_typst_math("\\times"), "times");
        assert_eq!(latex_to_typst_math("\\frac{1}{2}"), "frac(1, 2)");
        assert_eq!(latex_to_typst_math("x \\leq 1"), "x <= 1");
        assert!(latex_to_typst_math("\\unknowncmd").contains("unknowncmd"));
    }

    #[test]
    fn latex_frac_and_nested_calls() {
        // 回归：真实博客全量实测发现 \frac{a}{b} 花括号形态必须重写为函数调用
        assert_eq!(latex_to_typst_math("\\frac{a}{b}"), "frac(a, b)");
        assert_eq!(latex_to_typst_math("\\frac{1}{\\sqrt{2}}"), "frac(1, sqrt(2))");
        assert_eq!(latex_to_typst_math("\\sqrt[3]{x}"), "sqrt(3, x)");
        assert_eq!(latex_to_typst_math("\\mathbf{v}"), "bold(v)");
    }

    #[test]
    fn latex_matrix_envs() {
        assert_eq!(
            latex_to_typst_math("\\begin{pmatrix} a & b \\\\ c & d \\end{pmatrix}"),
            "mat(delim: \"(\", a, b; c, d)"
        );
        assert!(latex_to_typst_math("\\begin{cases} 1 \\\\ 2 \\end{cases}").starts_with("cases("));
        // equation 环境去壳保留内容
        assert!(latex_to_typst_math("\\begin{equation} x \\end{equation}").contains("x"));
    }

    #[test]
    fn latex_multiletter_variables_split() {
        // 回归：Typst 数学把多字母串当未知变量（$E = mc^2$ 会编译失败）
        assert_eq!(latex_to_typst_math("mc^2"), "m c^2");
        // 已知函数名与反斜杠命令不拆
        assert_eq!(latex_to_typst_math("\\sin x + \\log y"), "sin x + log y");
        assert_eq!(latex_to_typst_math("arcsin z"), "arcsin z");
    }

    #[test]
    fn inline_text_escapes_markup_chars() {
        assert_eq!(inline_text("a*b#c$d"), "a\\*b\\#c\\$d");
        // 回归：[ ] 是 Typst 内容块定界符（mqy 语料里的 [object Number] 文本）
        assert_eq!(inline_text("[object Number]"), "\\[object Number\\]");
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
        let md2 = "| *强调* | 文本 |\n|---|---|\n";
        let out2 = convert(md2, None);
        assert!(out2.contains("[_强调_]"), "实际: {}", out2.trim());
    }

    #[test]
    fn footnotes_inline_into_reference() {
        let md = "正文有脚注[^1]。\n\n[^1]: 这是脚注内容，含 **加粗**。\n";
        let out = convert(md, None);
        assert!(out.contains("#footnote[这是脚注内容，含 *加粗*。]"), "实际: {out}");
        // 定义区原文位置不再出现
        assert!(!out.contains("[^1]:"));
    }

    #[test]
    fn task_list_markers() {
        let md = "- [x] 已完成\n- [ ] 待办\n";
        let out = convert(md, None);
        assert!(out.contains("☑ 已完成"), "实际: {out}");
        assert!(out.contains("☐ 待办"), "实际: {out}");
    }

    #[test]
    fn strikethrough_and_block_html() {
        let md = "~~删除线~~ 文本\n\n<div class=\"ad\">\nHTML 块\n</div>\n";
        let out = convert(md, None);
        assert!(out.contains("#strike[删除线]"), "实际: {out}");
        assert!(out.contains("// <div class=\"ad\">"), "实际: {out}");
        assert!(out.contains("// HTML 块"), "实际: {out}");
    }
}
