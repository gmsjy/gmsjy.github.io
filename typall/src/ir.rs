//! 文档中间表示（IR）：Typst 编译产物与各发布目标之间的唯一桥梁。
//!
//! 设计意图（扩展余地核查 · 阶段 0）：
//! - 编译管线只负责产出 [`CompiledDoc`]，**不做任何目标特定的变换**；
//! - 每个发布目标（站点 HTML / Markdown / 微信公众号 / 知乎 / …）都是 IR 的
//!   一个消费者，从同一份 IR 按目标重变换（站点 HTML 是第一个消费者，
//!   见 [`crate::site_html`]）；
//! - 未来扩展目标时**新增消费者**，而不是在编译管线里加分支。
//!
//! 注意：`body_html` 仍是整块字符串（由 typst-html 导出），标题/TOC/摘要等
//! 语义仍由消费者从 HTML 反推。公式语义已先行入 IR：[`MathItem`] 在语法层
//! 提取全部公式源码（不经编译，缓存命中零成本），与 `body_html` 中公式
//! 渲染物按序一一对应。后续待扩展（Markdown/知乎目标深化时按需推进）：
//! 公式编号、结构化 TOC/标题层级、插图（cetz）节点、块级内容模型。

use crate::content::DocumentMeta;
use typst::syntax::parse;
use typst::syntax::ast::AstNode;

/// 一个公式条目（Typst 数学源码）。
///
/// 由编译管线在**语法层**捕获（不经 Typst 编译器，缓存命中时同样零成本提取），
/// 供 Markdown / 知乎等目标把正文中的公式渲染物还原为文本公式。
/// 注意：这是 Typst 数学语法而非 LaTeX；LaTeX 转换是各发布目标的
/// per-target 关注点（阶段 3）。
#[derive(Debug, Clone, PartialEq)]
pub struct MathItem {
    /// 公式源码（不含 `$` 定界符，如 `x^2 + 1`）。
    pub source: String,
    /// 是否块级公式（`$ ... $`）。
    pub block: bool,
}

/// 从 Typst 源码提取全部公式的源码（文档顺序）。
///
/// 用 typst-syntax 完整解析（能正确处理注释、字符串、转义），递归遍历
/// 语法树收集所有 `Equation` 节点。与编译产物中公式的出现顺序一致。
///
/// 限制：`cetz.canvas(...)` 等把内容吃进画布自行渲染的调用，其内部的
/// 标注公式（如 `$x_"中文"$`）会随画布成为单个 SVG，**不会**在正文里
/// 产出独立的公式渲染物——收集时跳过这类调用体，保证与渲染物数量
/// 配对。其余代码动态生成公式的场景由消费方做数量校验兜底。
pub fn extract_math_items(source_text: &str) -> Vec<MathItem> {
    fn walk(node: &typst::syntax::SyntaxNode, out: &mut Vec<MathItem>) {
        // 画布式库（cetz.canvas 等）的调用体不产出正文公式渲染物，整体跳过。
        if let Some(call) = node.cast::<typst::syntax::ast::FuncCall>()
            && call
                .callee()
                .to_untyped()
                .full_text()
                .ends_with("canvas")
        {
            return;
        }
        if let Some(eq) = node.cast::<typst::syntax::ast::Equation>() {
            // full_text 含 `$..$` 定界符；去掉首尾定界符与空白得纯公式源。
            let text = eq.to_untyped().full_text();
            let inner = text
                .strip_prefix('$')
                .and_then(|s| s.strip_suffix('$'))
                .unwrap_or(text.as_str())
                .trim()
                .to_string();
            out.push(MathItem {
                source: inner,
                block: eq.block(),
            });
            // 公式内部不嵌套公式，直接返回，避免重复收集。
            return;
        }
        for child in node.children() {
            walk(child, out);
        }
    }

    let root = parse(source_text);
    let mut out = Vec::new();
    walk(&root, &mut out);
    out
}

/// 一篇编译完成的文档（发布 IR）。
pub struct CompiledDoc {
    /// 输出 slug，如 `posts/quantum`。
    pub slug: String,
    pub meta: DocumentMeta,
    /// 正文 HTML（不含页面骨架）。目标无关：站点消费者会再做
    /// 锚点/TOC/懒加载等目标特定后处理（见 `site_html::postprocess_body`）。
    pub body_html: String,
    /// Typst 生成的 MathML 样式块。
    pub math_style: String,
    /// 正文公式的 Typst 源码（文档顺序），与 `body_html` 中公式渲染物
    /// 按序一一对应（配对数量由消费方校验）。见 [`MathItem`]。
    pub math: Vec<MathItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_math_collects_inline_and_block_in_order() {
        let src = r#"#let title = "t"
开头 $x^2 + 1$ 行内公式。

$ sum_(i=1)^n i = (n(n+1))/2 $

结尾 $E = m c^2$ 完。
"#;
        let items = extract_math_items(src);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].source, "x^2 + 1");
        assert!(!items[0].block);
        assert_eq!(items[1].source, "sum_(i=1)^n i = (n(n+1))/2");
        assert!(items[1].block);
        assert_eq!(items[2].source, "E = m c^2");
        assert!(!items[2].block);
    }

    #[test]
    fn extract_math_ignores_comments_and_strings() {
        // 注释里的 $ 不算公式；字符串里同理。
        let src = r#"// $fake = 1$
#let s = "$not-math$"
真公式 $a < b$。
"#;
        let items = extract_math_items(src);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "a < b");
    }

    #[test]
    fn extract_math_empty_source() {
        assert!(extract_math_items("= 标题\n\n普通段落。").is_empty());
    }

    #[test]
    fn extract_math_skips_canvas_bodies() {
        // cetz 画布内的标注公式随画布渲染成单个 SVG，不产出正文公式渲染物。
        let src = r#"#import "@preview/cetz:0.5.2"

正文公式 $a^2 + b^2 = c^2$。

#cetz.canvas(length: 2cm, {
  content((0, 0), $x_"最大"$)
  line((0, 0), (1, 1))
})

结尾 $E = m c^2$。
"#;
        let items = extract_math_items(src);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].source, "a^2 + b^2 = c^2");
        assert_eq!(items[1].source, "E = m c^2");
    }

    #[test]
    fn extract_math_handles_multiline_block() {
        let src = "$\n integral_0^1 x dif x = 1/2 \n$";
        let items = extract_math_items(src);
        assert_eq!(items.len(), 1);
        assert!(items[0].block);
        assert_eq!(items[0].source, "integral_0^1 x dif x = 1/2");
    }
}
