# AGENTS.md — AI 会话工作规范

本仓库是一个 Typst 驱动的静态博客生成器（Rust），同时**本身就是一个站点**：`typall/` 目录既是生成器源码，也是站点的文章与配置根。

## 仓库布局

- `typall/src/` — 生成器 Rust 源码（axum serve、typst 编译、主题、部署等模块）
- `typall/posts/` — 站点文章（Typst 源，`#let` 风格 frontmatter）
- `typall/pages/` — 独立页面；`typall/themes/` — 站点主题；`typall/_templates/` — `typall new --series` 系列模板
- `typall/assets/` — `preview.typ`（fig/eq-numbering 兼容层）、`figkit.typ`（插图模板库）、`series-styles.typ` / `series-kit.typ`（图元与配色）、`series-figures/`（151 张成品图）、`images/`（位图）、`copythemes/`（复制排版主题示例）
- `docs/` — 设计文档与验收报告（`AUTHORING.md` 是写作规范）

## 常用命令

```bash
cd typall
cargo test                 # 全部单元 + 集成测试（提交前必须全绿）
cargo clippy               # 无警告为准
cargo run -- build         # 构建站点（验证文章可编译）
cargo run -- serve --open  # 开发预览
```

## 写作规范（要点，详见 docs/AUTHORING.md）

- 新文章用 `typall new <名>`（或 `--series 讲义`）生成，不手写骨架；
- frontmatter 是 `#let` 赋值：`title / date / tags / series / series_weight / draft`；
- 块级公式编号靠两条 show 规则（模板自带，勿删）；行内公式不编号；
- 插图**禁止裸 rgb()**：一律用 `series-styles.typ` 的色板常量（`math-blue`、`phys-orange`、`soft-*` 等）；
- 常用图优先 figkit 六类模板（plot/flow/cycle/card/number-line/raw）；成品图从 `assets/series-figures/` import；
- 位图放 `assets/images/`，正文用 `#image("../assets/images/<名>")`；
- 日期写真实日期；未来日期 = 定时发布（到点由 CI 定时构建自动上线）。

## 部署（不要弄错）

- **push 到 `main` → GitHub Actions 自动构建并发布 GitHub Pages**（`.github/workflows/deploy.yml`，Pages 为 GitHub Actions 模式）——这是唯一上线通道；
- `typall deploy`（git 策略）的产物**只落 `built-site` 分支**（`typall.toml` 已配置），**严禁**把部署目标指回 `main`——曾把构建产物推上 main 清空过远端源码；
- `main` 分支永远只放源码，不放构建产物。

## 改动后的验证清单

1. `cargo test` 全绿、`cargo clippy` 无警告；
2. 若动了文章/模板/主题：`cargo run -- build` 能完整构建；
3. 若动了 serve 注入脚本：`cargo run -- serve` 起服务后实际开页面确认（浮层/复制栏/素材库均在 `</body>` 前注入，`build` 产物必须零 JS）。
