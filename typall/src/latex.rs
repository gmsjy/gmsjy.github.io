//! Typst 数学 → LaTeX 转换器（知乎等目标：公式以 LaTeX 形态输出）。
//!
//! 渐进式覆盖：先处理高频结构（分数/根号/上下标/极限求和积分/希腊字母/
//! 常用函数与箭头），未覆盖的语法片段**原样保留**并可在产物中检索 `\\` 手工处理。
//!
//! 设计为字符串层的规则转换而非完整 AST 解析——数学源码来自
//! [`crate::ir::MathItem`]（typst 语法层提取），结构相对规整，规则转换
//! 的覆盖率已满足常见教学文章；遇到复杂结构时保真优先于强行翻译。

/// Typst 数学命令 → LaTeX 命令（同名直译部分在 SYMBOLS 之外单独列）。
///
/// **点号名必须排在对应前缀名之前**（如 `arrow.r.double` 先于 `arrow.r`）：
/// 无边界检查的 `\name` 字符串替换按字典序就近匹配，前缀先替换会把
/// `\arrow.r.double` 破坏成 `\rightarrow.double`。
const COMMAND_MAP: &[(&str, &str)] = &[
    // --- 点号（长名）在前 ---
    ("arrow.r.double", "Rightarrow"),
    ("arrow.l.double", "Leftarrow"),
    ("arrow.t.r", "uparrow"),
    ("arrow.t.l", "downarrow"),
    ("dot.double", "ddot"),
    ("dot.op", "cdot"),
    ("subset.eq", "subseteq"),
    ("superset.eq", "supseteq"),
    ("in.not", "notin"),
    ("angle.l", "langle"),
    ("angle.r", "rangle"),
    ("plus.circle", "oplus"),
    ("times.circle", "otimes"),
    // --- 单名 ---
    ("times", "times"),
    ("div", "div"),
    ("pm", "pm"),
    ("approx", "approx"),
    ("neq", "neq"),
    ("leq", "leq"),
    ("geq", "geq"),
    ("infty", "infty"),
    ("partial", "partial"),
    ("sum", "sum"),
    ("product", "prod"),
    ("integral", "int"),
    ("lim", "lim"),
    ("alpha", "alpha"),
    ("beta", "beta"),
    ("gamma", "gamma"),
    ("delta", "delta"),
    ("epsilon", "epsilon"),
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
    ("Phi", "Phi"),
    ("Psi", "Psi"),
    ("Omega", "Omega"),
    ("degree", "degree"),
    ("arrow.r", "rightarrow"),
    ("arrow.l", "leftarrow"),
    ("dots", "ldots"),
    ("plus.minus", "pm"),
    // --- 集合 / 逻辑 / 关系 ---
    ("emptyset", "emptyset"),
    ("parallel", "parallel"),
    ("subset", "subset"),
    ("superset", "supset"),
    ("inter", "cap"),
    ("union", "cup"),
    ("in", "in"),
    ("exists", "exists"),
    ("forall", "forall"),
    ("bot", "bot"),
    ("top", "top"),
    ("equiv", "equiv"),
    ("sim", "sim"),
    ("propto", "propto"),
    // --- 无穷 / 双字母黑板体 / 杂项 ---
    ("oo", "infty"),
    ("infinity", "infty"),
    ("RR", "mathbb{R}"),
    ("ZZ", "mathbb{Z}"),
    ("NN", "mathbb{N}"),
    ("QQ", "mathbb{Q}"),
    ("CC", "mathbb{C}"),
    ("AA", "mathbb{A}"),
    ("EE", "mathbb{E}"),
    ("FF", "mathbb{F}"),
    ("PP", "mathbb{P}"),
    ("HH", "mathbb{H}"),
    ("prop", "propto"),
    ("complement", "complement"),
    ("Re", "Re"),
    ("Im", "Im"),
    ("prime", "prime"),
    ("space", " "),
    ("quad", "quad"),
    // --- 常见函数名（裸词直排 → LaTeX 正体函数名）---
    ("sin", "sin"),
    ("cos", "cos"),
    ("tan", "tan"),
    ("cot", "cot"),
    ("sec", "sec"),
    ("csc", "csc"),
    ("log", "log"),
    ("ln", "ln"),
    ("exp", "exp"),
    ("max", "max"),
    ("min", "min"),
    ("deg", "deg"),
    // 裸 `dot` 是点乘符号（call 形态 dot(x) 是顶点重音，在 rewrite_functions 处理）
    ("dot", "cdot"),
];

/// 单字符运算/关系符映射（Typst 数学里直接写 `<=` 等）。
const CHAR_MAP: &[(&str, &str)] = &[("<=", "\\leq"), (">=", "\\geq"), ("!=", "\\neq"), ("->", "\\rightarrow"), ("=>", "\\Rightarrow")];

/// 把 Typst 数学源码转换为 LaTeX。
///
/// 转换顺序：函数调用（frac/sqrt/bold/…）→ 附标 → 命令/字符映射。
/// 无法识别的部分原样保留（LaTeX 对拉丁字母/数字/空格天然兼容）。
pub fn typst_math_to_latex(src: &str) -> String {
    let mut out = src.to_string();

    // 1. 带参数函数调用：frac(a, b) / sqrt(x) / vec(v) / mat(...) 等
    //    每轮重写最左一个已知函数，长公式可能含几十个调用，上限放宽到 64 轮
    for _ in 0..64 {
        let before = out.len();
        out = rewrite_functions(&out);
        if out.len() == before {
            break;
        }
    }

    // 2. 附标：`_(...)` `^(...)` → `_{...}` `^{...}`；单字符 `_x` `^x` → `_{x}` `^{x}`
    out = rewrite_attachments(&out);

    // 3. 符号命令映射（仅裸词独立出现时替换；带反斜杠的已是合法 LaTeX，
    //    边界检查会跳过，避免破坏函数重写产生的 \dot{x} 等命令）
    for (cmd, latex) in COMMAND_MAP {
        out = rewrite_bare_symbol(&out, cmd, &format!("\\{}", latex));
    }

    // 4. 单字符运算符
    for (from, to) in CHAR_MAP {
        out = out.replace(from, to);
    }

    // 5. 引号文本 → \text{…}（Typst 数学里的 "…" 是正体文本）
    out = rewrite_quoted_text(&out);

    // 6. 度数符号（posts 里常直接写 °）与微分记号
    out = out.replace('°', "^{\\circ}");
    out = out.replace("dif", "d");
    out
}

/// Typst 数学引号文本 `"…"` → LaTeX `\text{…}`。引号不配对（奇数个）时原样保留。
fn rewrite_quoted_text(s: &str) -> String {
    let parts: Vec<&str> = s.split('"').collect();
    if parts.len() < 3 || !(parts.len() - 1).is_multiple_of(2) {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len() + 8 * (parts.len() / 2));
    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 0 {
            out.push_str(part);
        } else {
            out.push_str("\\text{");
            out.push_str(part);
            out.push('}');
        }
    }
    out
}

/// Typst 裸符号名（如 `alpha`）→ LaTeX `\alpha`。
/// 仅翻译作为独立词出现的：前一个字节不是 ASCII 字母/反斜杠（避免破坏
/// 函数重写产生的 `\vec{…}` 等命令），后一个字节不是 ASCII 字母/点号
/// （避免 `dot` 误匹配 `dot.double`、`arrow.r` 误匹配 `arrow.r.double`）。
/// 按字符边界步进（公式常含 CJK 字符，按字节切片会在多字节字符中间 panic）。
fn rewrite_bare_symbol(s: &str, name: &str, latex: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < s.len() {
        if s[i..].starts_with(name) {
            let before_ok =
                i == 0 || !bytes[i - 1].is_ascii_alphabetic() && bytes[i - 1] != b'\\';
            let after = i + name.len();
            let after_ok =
                after >= s.len() || !bytes[after].is_ascii_alphabetic() && bytes[after] != b'.';
            if before_ok && after_ok {
                out.push_str(latex);
                i = after;
                continue;
            }
        }
        let ch = s[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

/// 重写函数调用形态：`fn(args)` → LaTeX（参数逐个递归转换，逗号分组）。
fn rewrite_functions(s: &str) -> String {
    // 找最内层的一个 `name(`：从左到右扫描
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'(' {
            // 回溯函数名（点号名如 dot.double 也算函数名）
            let mut name_start = i;
            while name_start > 0
                && (bytes[name_start - 1].is_ascii_alphanumeric()
                    || bytes[name_start - 1] == b'_'
                    || bytes[name_start - 1] == b'.')
            {
                name_start -= 1;
            }
            let name = &s[name_start..i];
            // 找配对闭括号
            if let Some(close) = find_matching_paren(s, i) {
                let args_src = &s[i + 1..close];
                // 只翻译已知函数；未知函数保留原样（含括号）
                let known = [
                    "frac", "sqrt", "bold", "cases", "abs", "underbrace", "overbrace", "vec",
                    "hat", "tilde", "bar", "overline", "underline", "dot", "dot.double", "arrow",
                    "root", "lr", "norm", "binom", "upright", "cancel", "mat", "floor", "ceil",
                ];
                if !known.contains(&name) {
                    // 未知名（含空名/纯代码调用）：不跳过整组——组内可能嵌套
                    // 已知函数（如 (x - overline(x))^{2}），继续向右扫描。
                    i += 1;
                    continue;
                }
                let converted_args = convert_args(args_src);
                let body = match name {
                    "frac" if converted_args.contains(',') => {
                        let parts: Vec<String> =
                            converted_args.split(',').map(|a| format!("{{{}}}", a.trim())).collect();
                        format!("\\frac{}", parts.join(""))
                    }
                    "frac" => format!("\\frac{{{}}}", converted_args),
                    "sqrt" => format!("\\sqrt{{{}}}", converted_args),
                    // root(n, x) → \sqrt[n]{x}；root(x) → \sqrt{x}
                    "root" => {
                        let parts: Vec<&str> = converted_args.split(", ").collect();
                        if parts.len() == 2 {
                            format!("\\sqrt[{}]{{{}}}", parts[0], parts[1])
                        } else {
                            format!("\\sqrt{{{}}}", converted_args)
                        }
                    }
                    "binom" if converted_args.contains(", ") => {
                        let parts: Vec<&str> = converted_args.split(", ").collect();
                        format!("\\binom{{{}}}{{{}}}", parts[0], parts[1])
                    }
                    "mat" => render_mat(args_src),
                    "cases" => format!(
                        "\\begin{{cases}} {} \\end{{cases}}",
                        converted_args.replace(" , ", " \\\\ ").replace(',', " \\\\ ")
                    ),
                    "abs" => format!("\\left|{}\\right|", converted_args),
                    "norm" => format!("\\left\\|{}\\right\\|", converted_args),
                    "lr" => format!("\\left({}\\right)", converted_args),
                    "floor" => format!("\\lfloor {} \\rfloor", converted_args),
                    "ceil" => format!("\\lceil {} \\rceil", converted_args),
                    // underbrace(x, 说明) → \underbrace{x}_{说明}
                    "underbrace" if converted_args.contains(", ") => {
                        let parts: Vec<&str> = converted_args.split(", ").collect();
                        format!("\\underbrace{{{}}}_{{{}}}", parts[0], parts[1])
                    }
                    "overbrace" if converted_args.contains(", ") => {
                        let parts: Vec<&str> = converted_args.split(", ").collect();
                        format!("\\overbrace{{{}}}^{{{}}}", parts[0], parts[1])
                    }
                    _ => {
                        let latex_name = match name {
                            "bold" => "mathbf",
                            "upright" => "mathrm",
                            "dot.double" => "ddot",
                            "arrow" => "overrightarrow",
                            other => other,
                        };
                        format!("\\{}{{{}}}", latex_name, converted_args)
                    }
                };
                let mut out = String::with_capacity(s.len());
                out.push_str(&s[..name_start]);
                out.push_str(&body);
                out.push_str(&s[close + 1..]);
                return out;
            }
        }
        i += 1;
    }
    s.to_string()
}

/// Typst `mat(...)`：`;` 分行、`,` 分列，`delim:` 参数选环境
/// （默认/`(` → pmatrix，`[` → bmatrix，`|` → vmatrix，`{` → Bmatrix，none → matrix）。
fn render_mat(raw_args: &str) -> String {
    let mut env = "pmatrix";
    let mut body_src = raw_args.trim();
    if let Some(rest) = body_src.strip_prefix("delim:")
        && let Some(comma) = rest.find(',')
    {
        let d = rest[..comma].trim();
        env = if d.contains('[') {
            "bmatrix"
        } else if d.contains('|') {
            "vmatrix"
        } else if d.contains('{') {
            "Bmatrix"
        } else if d.contains("none") {
            "matrix"
        } else {
            "pmatrix"
        };
        body_src = rest[comma + 1..].trim();
    }
    let converted = typst_math_to_latex(body_src);
    let rows: Vec<String> = converted
        .split(';')
        .map(|row| {
            row.split(',')
                .map(|c| c.trim())
                .filter(|c| !c.is_empty())
                .collect::<Vec<_>>()
                .join(" & ")
        })
        .collect();
    format!("\\begin{{{}}} {} \\end{{{}}}", env, rows.join(" \\\\ "), env)
}

/// 找配对闭括号位置（嵌套计数，跳过字符串）。
fn find_matching_paren(s: &str, open: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    for (i, &b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// 函数参数串递归转换 + 逗号两侧规整为 ", "。
fn convert_args(args: &str) -> String {
    let converted = typst_math_to_latex(args);
    // 参数内顶层逗号统一为 ", "（嵌套函数的逗号已在其自身转换中消化）
    let mut parts: Vec<String> = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in converted.chars() {
        match c {
            '(' | '{' | '[' => {
                depth += 1;
                cur.push(c);
            }
            ')' | '}' | ']' => {
                depth -= 1;
                cur.push(c);
            }
            ',' if depth == 0 => {
                parts.push(cur.trim().to_string());
                cur = String::new();
            }
            _ => cur.push(c),
        }
    }
    parts.push(cur.trim().to_string());
    parts.join(", ")
}

/// 附标重写：`_(…)`→`_{…}`、`^(…)`→`^{…}`、`_x`/`^x`（单字符）→ `_{x}`/`^{x}`。
fn rewrite_attachments(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        out.push(c);
        if (c == '_' || c == '^') && i + 1 < chars.len() {
            let next = chars[i + 1];
            if next == '(' {
                // 括号组 → 花括号组
                if let Some(close_rel) = find_matching_paren_char(&chars, i + 1) {
                    let inner: String = chars[i + 2..close_rel].iter().collect();
                    out.push('{');
                    out.push_str(&rewrite_attachments(&inner));
                    out.push('}');
                    i = close_rel + 1;
                    continue;
                }
            } else if next == '"' {
                // 引号文本作附标：_"合" → _{\text{合}}（`_` 已在循环顶部输出）
                if let Some(close_rel) = chars[i + 2..].iter().position(|&c| c == '"') {
                    let inner: String = chars[i + 2..i + 2 + close_rel].iter().collect();
                    out.push_str("{\\text{");
                    out.push_str(&inner);
                    out.push_str("}}");
                    i = i + 2 + close_rel + 1;
                    continue;
                }
            } else if next != '{' && next != '_' && next != '^' {
                // 单字符附标
                out.push('{');
                out.push(next);
                out.push('}');
                i += 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn find_matching_paren_char(chars: &[char], open: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (i, &c) in chars.iter().enumerate().skip(open) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols_map_to_latex() {
        assert_eq!(typst_math_to_latex("alpha beta gamma"), "\\alpha \\beta \\gamma");
        assert_eq!(typst_math_to_latex("E = m c^2"), "E = m c^{2}");
        // 词内不误伤
        assert_eq!(typst_math_to_latex("alphabet"), "alphabet");
    }

    #[test]
    fn frac_sqrt_and_operators() {
        assert_eq!(typst_math_to_latex("frac(a, b)"), "\\frac{a}{b}");
        assert_eq!(typst_math_to_latex("sqrt(x + 1)"), "\\sqrt{x + 1}");
        assert_eq!(typst_math_to_latex("frac(a, b) + sqrt(c)"), "\\frac{a}{b} + \\sqrt{c}");
    }

    #[test]
    fn nested_functions() {
        assert_eq!(
            typst_math_to_latex("frac(1, sqrt(2))"),
            "\\frac{1}{\\sqrt{2}}"
        );
    }

    #[test]
    fn attachments() {
        assert_eq!(typst_math_to_latex("x^2"), "x^{2}");
        assert_eq!(typst_math_to_latex("a_i"), "a_{i}");
        assert_eq!(typst_math_to_latex("sum_(i=1)^n"), "\\sum_{i=1}^{n}");
        // 组合：求和符号与上下标
        assert!(typst_math_to_latex("sum_(i=1)^n x_i").contains("\\sum"));
    }

    #[test]
    fn operators_and_derivatives() {
        assert!(typst_math_to_latex("x <= 1").contains("\\leq"));
        assert!(typst_math_to_latex("a -> b").contains("\\rightarrow"));
        assert!(typst_math_to_latex("integral_0^1 x dif x").contains("\\int"));
    }

    #[test]
    fn bold_and_real_world() {
        assert_eq!(typst_math_to_latex("bold(v)"), "\\mathbf{v}");
        // 真实案例：μ 子寿命公式
        let src = "Delta t = gamma Delta tau";
        assert!(typst_math_to_latex(src).contains("\\Delta"));
    }

    #[test]
    fn accents_and_overlines() {
        assert_eq!(typst_math_to_latex("vec(a)"), "\\vec{a}");
        assert_eq!(typst_math_to_latex("hat(x)"), "\\hat{x}");
        assert_eq!(typst_math_to_latex("tilde(t)"), "\\tilde{t}");
        assert_eq!(typst_math_to_latex("bar(x)"), "\\bar{x}");
        assert_eq!(typst_math_to_latex("overline(z)"), "\\overline{z}");
        assert_eq!(typst_math_to_latex("underline(x)"), "\\underline{x}");
        // dot：call 形态是顶点重音，dot.double 是二阶
        assert_eq!(typst_math_to_latex("dot(x)"), "\\dot{x}");
        assert_eq!(typst_math_to_latex("dot.double(x)"), "\\ddot{x}");
    }

    #[test]
    fn vector_arrow_root_and_delims() {
        assert_eq!(typst_math_to_latex("arrow(AB)"), "\\overrightarrow{AB}");
        assert_eq!(typst_math_to_latex("root(n, a)"), "\\sqrt[n]{a}");
        assert_eq!(typst_math_to_latex("root(a)"), "\\sqrt{a}");
        assert_eq!(typst_math_to_latex("lr(a + b)"), "\\left(a + b\\right)");
        assert_eq!(typst_math_to_latex("norm(v)"), "\\left\\|v\\right\\|");
        assert_eq!(typst_math_to_latex("binom(n, k)"), "\\binom{n}{k}");
        assert_eq!(typst_math_to_latex("floor(x)"), "\\lfloor x \\rfloor");
        assert_eq!(typst_math_to_latex("upright(kg)"), "\\mathrm{kg}");
    }

    #[test]
    fn matrices() {
        assert_eq!(
            typst_math_to_latex("mat(1, 2; 3, 4)"),
            "\\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\end{pmatrix}"
        );
        assert!(typst_math_to_latex("mat(delim: \"[\", 1; 2)").contains("bmatrix"));
        assert!(typst_math_to_latex("mat(delim: none, 1)").contains("\\begin{matrix}"));
    }

    #[test]
    fn quoted_text_to_text() {
        assert_eq!(typst_math_to_latex("n \"为偶数\""), "n \\text{为偶数}");
        assert_eq!(typst_math_to_latex("f(x) = 0 \"，其中\" x > 0"), "f(x) = 0 \\text{，其中} x > 0");
        // 引号不配对时原样保留
        assert_eq!(typst_math_to_latex("a \"b"), "a \"b");
    }

    #[test]
    fn cjk_math_does_not_panic() {
        // 回归：多字节字符曾被按字节切片导致 panic（publish --to zhihu 崩溃）
        let _ = typst_math_to_latex("vec(F)_合 = m vec(a)");
        let _ = typst_math_to_latex("设 alpha = 1");
        // 引号文本附标（真实 posts 写法 vec(F_"合")）→ \text{} 下标
        assert_eq!(
            typst_math_to_latex("vec(F_\"合\") = m vec(a)"),
            "\\vec{F_{\\text{合}}} = m \\vec{a}"
        );
    }

    #[test]
    fn double_arrow_not_corrupted() {
        // 回归：arrow.r 曾先于 arrow.r.double 匹配，把 => 破坏成 \rightarrow.double
        assert_eq!(typst_math_to_latex("A => B"), "A \\Rightarrow B");
        assert_eq!(typst_math_to_latex("arrow.r.double"), "\\Rightarrow");
        assert_eq!(typst_math_to_latex("arrow.l.double"), "\\Leftarrow");
        assert_eq!(typst_math_to_latex("a => b >= c"), "a \\Rightarrow b \\geq c");
    }

    #[test]
    fn sets_logic_and_infinity() {
        assert_eq!(typst_math_to_latex("oo"), "\\infty");
        assert_eq!(typst_math_to_latex("x -> oo"), "x \\rightarrow \\infty");
        assert_eq!(typst_math_to_latex("RR"), "\\mathbb{R}");
        assert_eq!(typst_math_to_latex("x in NN"), "x \\in \\mathbb{N}");
        assert_eq!(typst_math_to_latex("A subset.eq B"), "A \\subseteq B");
        assert_eq!(typst_math_to_latex("A union B"), "A \\cup B");
        assert_eq!(typst_math_to_latex("A inter B"), "A \\cap B");
        assert_eq!(typst_math_to_latex("emptyset"), "\\emptyset");
        assert_eq!(typst_math_to_latex("forall x exists y"), "\\forall x \\exists y");
    }

    #[test]
    fn function_names_upright_and_spacing() {
        assert_eq!(typst_math_to_latex("sin theta"), "\\sin \\theta");
        assert_eq!(typst_math_to_latex("sin(x)"), "\\sin(x)");
        assert_eq!(typst_math_to_latex("log_a b"), "\\log_{a} b");
        assert_eq!(typst_math_to_latex("a quad b"), "a \\quad b");
        assert_eq!(typst_math_to_latex("max_(x)"), "\\max_{x}");
        // 裸 dot 是点乘
        assert_eq!(typst_math_to_latex("a dot b"), "a \\cdot b");
        // 函数产物不被裸词二次处理
        assert_eq!(typst_math_to_latex("vec(x)"), "\\vec{x}");
        assert_eq!(typst_math_to_latex("dot(x)"), "\\dot{x}");
    }

}

