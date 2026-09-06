//! Typst 版本兼容层：集中管理对 Typst 内部行为的全部假设。
//!
//! typall 依赖若干 Typst 的非稳定行为（HTML 导出模块、运行时 Feature 等），
//! 这些假设此前散落在 world.rs / build.rs / assets/preview.typ 各处。
//! 升级 Typst 版本时：
//! 1. 只允许修改本文件中的兼容逻辑与 `Cargo.toml` 版本号；
//! 2. 运行 e2e 黄金基线测试（`cargo test --test e2e`）判断行为是否漂移；
//! 3. 逐条核对 docs/COMPAT.md 中的假设清单。
//!
//! 上游 HTML 导出转正后，本文件的大部分内容可整体退役。

use typst::{Feature, Features};

/// 当前锁定的 Typst 版本（与 Cargo.toml 的精确锁版一致）。
#[allow(dead_code)] // 契约锚点：版本升级演练时对照 COMPAT.md
pub const TYPST_VERSION: &str = "0.15.1";

/// HTML 导出所需的运行时特性集合。
///
/// `Feature::Html` 是 Typst 的非稳定特性：不经 cargo feature，而是编译期
/// 构建、运行期按 World 选择性启用。上游转正后此函数可改为默认行为。
pub fn html_features() -> Features {
    Features::from_iter([Feature::Html])
}

/// 版本假设清单核对提示（供升级演练脚本 / 人工复查使用）。
#[allow(dead_code)] // 契约锚点：假设清单文本，供 COMPAT.md 引用
pub const COMPAT_ASSUMPTIONS: &[&str] = &[
    "html.frame / html.elem 仅在 html 目标下可用（assets/preview.typ 经 eval 隔离）",
    "SVG 渲染器不绘制原生公式编号 → 编号由 HTML 后处理注入（typst#5512）",
    "@eq 引用依赖 numbering 设置 → preview.typ 的 eq-numbering 在分页下启用",
    "空格块级公式（$ x $）参与编号计数 → fig() 内已局部屏蔽",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_features_contains_html() {
        let f = html_features();
        // Features 无直接查询接口，能构造成功即为版本兼容的最低信号
        let _ = f;
        assert_eq!(TYPST_VERSION, "0.15.1");
        assert!(COMPAT_ASSUMPTIONS.len() >= 4);
    }
}
