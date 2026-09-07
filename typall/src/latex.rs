//! Typst 数学 → LaTeX 转换器（知乎等目标：公式以 LaTeX 形态输出）。
//!
//! 渐进式覆盖：先处理高频结构（分数/根号/上下标/极限求和积分/希腊字母/
//! 常用函数与箭头），未覆盖的语法片段**原样保留**并可在产物中检索 `\\` 手工处理。
//!
//! 设计为字符串层的规则转换而非完整 AST 解析——数学源码来自
//! [`crate::ir::MathItem`]（typst 语法层提取），结构相对规整，规则转换
//! 的覆盖率已满足常见教学文章；遇到复杂结构时保真优先于强行翻译。

/// Typst 数学命令 → LaTeX 命令（同名直译部分在 SYMBOLS 之外单独列）。
const COMMAND_MAP: &[(&str, &str)] = &[
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
    ("arrow.r.double", "Rightarrow"),
    ("arrow.l.double", "Leftarrow"),
    ("arrow.t.r", "uparrow"),
    ("dots", "ldots"),
    ("plus.minus", "pm"),
];

/// 单字符运算/关系符映射（Typst 数学里直接写 `<=` 等）。
const CHAR_MAP: &[(&str, &str)] = &[("<=", "\\leq"), (">=", "\\geq"), ("!=", "\\neq"), ("->", "\\rightarrow"), ("=>", "\\Rightarrow")];

/// 把 Typst 数学源码转换为 LaTeX。
///
/// 转换顺序：函数调用（frac/sqrt/bold/…）→ 附标 → 命令/字符映射。
/// 无法识别的部分原样保留（LaTeX 对拉丁字母/数字/空格天然兼容）。
pub fn typst_math_to_latex(src: &str) -> String {
    let mut out = src.to_string();

    // 1. 带参数函数调用：frac(a, b) / sqrt(x) / bold(x) / cases(...) 等
    for round in 0..8 {
        let before = out.len();
        out = rewrite_functions(&out);
        if out.len() == before {
            break;
        }
        let _ = round;
    }

    // 2. 附标：`_(...)` `^(...)` → `_{...}` `^{...}`；单字符 `_x` `^x` → `_{x}` `^{x}`
    out = rewrite_attachments(&out);

    // 3. 符号命令映射（\frac 等已由函数重写产生的不再处理）
    for (cmd, latex) in COMMAND_MAP {
        out = out.replace(&format!("\\{}", cmd), &format!("\\{}", latex));
        // typst 数学里裸写 alpha（无反斜杠）也常见
        out = rewrite_bare_symbol(&out, cmd, &format!("\\{}", latex));
    }

    // 4. 单字符运算符
    for (from, to) in CHAR_MAP {
        out = out.replace(from, to);
    }

    // 5. dif x → dx（Typst 微分记号）
    out = out.replace("dif", "d");
    out
}

/// Typst 裸符号名（如 `alpha`）→ LaTeX `\alpha`。
/// 仅翻译作为独立词出现的（前后不是字母），避免误伤 `alphabet` 这类词。
fn rewrite_bare_symbol(s: &str, name: &str, latex: &str) -> String {
    let needle = name.to_string();
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < s.len() {
        if s[i..].starts_with(&needle) {
            let before_ok = i == 0 || !bytes[i - 1].is_ascii_alphabetic();
            let after = i + needle.len();
            let after_ok = after >= s.len() || !bytes[after].is_ascii_alphabetic();
            if before_ok && after_ok {
                out.push_str(latex);
                i = after;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

/// 重写函数调用形态：`fn(args)` → `\fn{args}`（参数逐个递归转换，逗号分组）。
fn rewrite_functions(s: &str) -> String {
    // 找最内层的一个 `name(`：从左到右扫描
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'(' {
            // 回溯函数名
            let mut name_start = i;
            while name_start > 0
                && (bytes[name_start - 1].is_ascii_alphanumeric() || bytes[name_start - 1] == b'_')
            {
                name_start -= 1;
            }
            let name = &s[name_start..i];
            // 找配对闭括号
            if let Some(close) = find_matching_paren(s, i) {
                let args_src = &s[i + 1..close];
                // 只翻译已知函数；未知函数保留原样（含括号）
                let known = ["frac", "sqrt", "bold", "cases", "abs", "underbrace", "overbrace"];
                if !known.contains(&name) {
                    i = close + 1;
                    continue;
                }
                let converted_args = convert_args(args_src);
                let latex_name = match name {
                    "frac" => "frac",
                    "sqrt" => "sqrt",
                    "bold" => "mathbf",
                    "abs" => "left|",
                    "cases" => "begin{cases}",
                    _ => name,
                };
                let body = match name {
                    "frac" if converted_args.contains(',') => {
                        let parts: Vec<String> =
                            converted_args.split(',').map(|a| format!("{{{}}}", a.trim())).collect();
                        format!("\\{}{}", latex_name, parts.join(""))
                    }
                    "frac" => format!("\\frac{{{}}}", converted_args),
                    "sqrt" => format!("\\sqrt{{{}}}", converted_args),
                    "bold" => format!("\\mathbf{{{}}}", converted_args),
                    "abs" => format!("\\left|{}\\right|", converted_args),
                    "cases" => format!(
                        "\\begin{{cases}} {} \\end{{cases}}",
                        converted_args.replace(" , ", " \\\\ ").replace(',', " \\\\ ")
                    ),
                    _ => format!("\\{}{{{}}}", latex_name, converted_args),
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
                let bytes: Vec<char> = chars.clone();
                let _ = bytes;
                if let Some(close_rel) = find_matching_paren_char(&chars, i + 1) {
                    let inner: String = chars[i + 2..close_rel].iter().collect();
                    out.push('{');
                    out.push_str(&rewrite_attachments(&inner));
                    out.push('}');
                    i = close_rel + 1;
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
}
