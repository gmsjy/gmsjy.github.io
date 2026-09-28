# 写作指南（AUTHORING）

> 面向在本仓库写作与改稿的所有人（及 AI 会话）。工具链约定见仓库根 `AGENTS.md`。

## 一篇文章的最小闭环

```bash
typall new --series 讲义 "细胞的结构"   # 生成 posts/2026-XX-XX-细胞的结构.typ
typall live posts/2026-XX-XX-细胞的结构  # 实时模式：保存即毫秒级刷新浏览器
# …编辑保存，浏览器即时反馈…
typall build                             # 完整构建验证
```

不用 `--series` 时得到最简骨架（自动带公式编号 show 规则）；`--series <名>`
使用 `_templates/<名>.typ` 作骨架，`{{title}}` / `{{date}}` / `{{series}}`
占位符就地替换。本站现成模板：`_templates/lecture.typ`（讲义）。

## Frontmatter（`#let` 赋值风格）

| 字段 | 必填 | 说明 |
|------|------|------|
| `title` | ✅ | 标题；也是页面 `<title>` 与 og 标题 |
| `date` | ✅ | `YYYY-MM-DD`；**未来日期 = 定时发布**（CI 每日构建到点自动上线） |
| `tags` | 建议 | `(标签一, 标签二)` |
| `series` | 系列文必填 | 系列名，集合页按它聚合 |
| `series_weight` | 系列文必填 | 系列内排序权重 |
| `draft` | — | `true` 时默认构建跳过（`--drafts` 才包含） |

## 公式

- 语法即 Typst math（`$x^2$` 行内、`$ ... $` 块级）；
- **块级公式编号**靠模板里那两条 `#show math.equation.where(...)` 规则——勿删；
  构建期 HTML 由后处理注入编号，PDF/VS Code 预览走原生 `(1)` 编号（`preview.typ`
  双模式兼容层处理了两端差异）；
- 需要引用编号用 `@eq:label` 语法（分页目标下可用）。

## 插图

**铁律：禁止裸 `rgb(...)`。** 颜色一律用 `series-styles.typ` 的色板常量，
保证全站图文一致（正文里 `#import "../assets/series-kit.typ": *` 即可获得全部图元与色板）。

按优先级选图的方式：

1. **figkit 模板**（`#import "../assets/figkit.typ": *`）——六类高频图一行成图：
   `plot`（函数曲线）、`flow`（流程框图）、`cycle`（转化三角）、`card`（全景卡）、
   `number-line`(数轴区间)、`raw` + 零件（beaker/electrode/salt-bridge 装置图）。
   参数「宽容默认」，只填关心的；完整用法与示例见站内《figkit 插图模板库》一文，
   或 serve 模式下的 `/__gallery` 速查页；
2. **成品图**（`assets/series-figures/`，151 张）——`/__gallery` 页点击复制
   `#import` 行，正文用 `#figure-block("编号", [题注], fig-别名)` 调用；
3. **位图**（照片/截图）——放 `assets/images/`（serve 的 `/__gallery` 支持
   拖拽/粘贴上传并给出片段），正文 `#image("../assets/images/<名>")`；
4. **特殊图**——figkit `raw()` 空画布 + CeTZ 原语 + series-kit 图元自由组装。

## 排版习惯

- 中文与英文/数字之间留空格；全角标点；
- 每讲开头给一行导读（`#text(size: 8.5pt, fill: gray-line)[…]` 风格，见已有讲义）；
- 小标题层级 `=` / `==` / `===`，不要跳级；
- 引用/强调用 Typst 语义标记（`*粗体*`、`_斜体_`），不手写 HTML。

## 复制到公众号 / 知乎

`typall serve` 打开文章页：浮动栏可切换**复制排版主题**（5 套内置 +
`copythemes/` 自定义，所见即所得），「复制到公众号」「复制到知乎」按当前主题
出稿。与站点主题解耦，`build` 产物零 JS 不受影响。细节见 README「微信目标说明」。

## 发布前检查

- `typall check`（或 `cargo run -- build`）无错误；
- frontmatter 齐全（系列文有 `series` + `series_weight`）；
- 文内链接与图片路径可解析（`build --strict` 做死链检查）；
- 需要社交分享图的确认 `og:image`（frontmatter 显式指定优先）。
