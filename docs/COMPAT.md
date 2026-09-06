# Typst 兼容假设清单（COMPAT.md）

typall 的 HTML 主干依赖若干 Typst 非稳定行为。**升级 Typst 版本前，逐条核对本清单**，
并执行升级演练：

```bash
# 升级演练三步
# 1. Cargo.toml 改 typst 版本号 + 只允许改动 src/typst_compat.rs
# 2. cargo test          # 单元 + e2e 黄金基线
# 3. 手工抽查一篇含公式/插图的页面
```

所有假设的实现集中在 `src/typst_compat.rs`（版本常量、html 特性启用）与
`assets/preview.typ`（fig / eq-numbering 双模式分支）。

## 假设清单

| # | 依赖的 Typst 行为 | typall 侧对策 | 验证方法 | 上游变化时 |
|---|------------------|--------------|---------|-----------|
| 1 | `html.frame` / `html.elem` 仅在 html 目标下存在 | `preview.typ` 用 `eval("html")` 运行时隔离；分页目标走原生渲染 | 裸编译探测 + e2e | 可简化为直接引用 |
| 2 | SVG 渲染器不绘制原生公式编号（typst#5512） | 文章 preamble 抑制编号，`build.rs` 后处理注入 `span.eq-num` | 页面 svg 数量基线 | 删除抑制 + 注入两层 |
| 3 | `@eq:` 引用要求 numbering 已设置 | `preview.typ` 的 `eq-numbering` 在分页下返回 `"(1)"` | 裸编译探测（17 篇含引用） | 无需变化 |
| 4 | `$ x $`（空格块级）参与编号计数 | `fig()` 内局部屏蔽画布内编号 | derivative 第一页目检 | 无需变化 |
| 5 | `Feature::Html` 运行时启用（非 cargo feature） | `typst_compat::html_features()` 集中构造 | cargo check / e2e | 转正后删除 |
| 6 | `html` 模块输出形状（div/span + data-equation 属性） | 后处理与 wechat/wechat 拷贝脚本按形状解析 | HTML 全文基线 | 调整解析规则 |

## 退役路线

上游 HTML 导出 stabilized 之时：

- 删除 `eq-numbering` 与编号抑制 → 公式编号回归 Typst 原生；
- 删除 `eval("html")` 分支 → `fig` 直连 `html.frame`；
- 42 篇文章头部的 3 行样板缩减为 1 行 import（或 0 行，若生成器注入可覆盖预览场景）；
- `typst_compat.rs` 收缩为纯版本常量。
