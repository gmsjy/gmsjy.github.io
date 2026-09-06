# Typall

一个用 **Rust** 编写、**完全自包含**的静态博客生成器，深度集成 **Typst 0.15** 排版引擎。`.typ` 是唯一的内容格式：同一份文档 IR，一次编译产出站点 HTML、Markdown、微信公众号富文本与 PDF。

## 特性

- **单二进制**：内嵌 Typst 编译器，无需 Typst CLI / Node.js / Python 等任何外部依赖
- **数学公式**：内联 SVG（本仓库默认，公式编号完整渲染）或 MathML（引擎默认），完全离线、零客户端 JS
- **手绘插图**：全部数理文章各配一幅 CeTZ 矢量示意图（受力分析、光路、函数曲线、轨道图……），随构建生成内联 SVG，支持暗色主题
- **真增量编译**：源码 + preamble + import 依赖三级哈希缓存——99 篇文章全量 0.3s、增量 0.06s、改一篇只编一篇
- **开箱即用的博客能力**：站内搜索 / SEO（canonical + OG）/ RSS 全文 / sitemap / 分页 / TOC / 标签分类归档 / 专栏（series 教材连载）/ gzip + brotli 预压缩 / Lazy Load
- **主题系统**：CSS 层 + 骨架模板层解耦，9 个 partial 片段 + tera 循环条件，不改主程序即可换结构与外观
- **多目标发布**：`publish` 单篇导出 Markdown / 公众号富文本（本地导出 + 「复制到公众号」按钮粘贴）；`pdf` 原生排版导出；`deploy` 整站部署（git / copy / netlify / vercel）

## 快速开始

```bash
# 构建二进制
cargo build --release --manifest-path typall/Cargo.toml

# 本仓库自带一个内容项目（posts/ 下 47 篇数理文章——「高中数学」24 篇、「高中物理」20 篇、「高中物理实验」3 篇，每篇配 CeTZ 手绘插图——另有「教学工具」专栏的 AiGGB 使用教程），可直接体验
cd typall
../target/release/typall build
../target/release/typall serve --open      # 浏览器打开 http://localhost:8080
```

从零建站：

```bash
typall init my-blog
cd my-blog
typall new my-first-post    # 生成 posts/<日期>-my-first-post.typ
typall build && typall serve --open
```

## 五分钟教程

**① 建站**：`typall init my-blog` 生成脚手架（配置 + 示例文章 + 目录结构）。

**② 写作**：`typall new 二次函数` 在 `posts/` 下生成今天的文章模板，用任意编辑器打开：

```typst
#let title = "二次函数的图像"
#let date = "2026-09-06"          // 未来日期 = 定时发布
#let tags = ("数学",)
#let draft = false                // true 则 build 默认跳过

= 二次函数的图像

开口方向由 $a$ 的符号决定：$a > 0$ 时开口向上。

#figure(cetz.canvas(length: 1cm, { /* CeTZ 手绘插图 */ }))
```

**③ 实时预览**：`typall serve --open` 启动开发服务器（默认 <http://localhost:8080>）。
保存文件即自动重建并刷新浏览器；写错了，错误全文直接显示在浏览器底部浮层，不用切终端。
`serve` 会额外显示草稿——预览所见即所有内容，正式发布不含。

**④ 构建与自检**：`typall build` 产出纯静态站到 `public/`（零 JS、含预压缩）。
`typall status` / `typall list` 随时查看草稿、定时与发布概况；上线前建议 `typall build --strict` 做死链检查。

**5. 发布**：见下方部署速览与[完整部署指南](typall/README.md#部署指南deploy)。

日常高频操作速查：

| 场景 | 命令 |
|------|------|
| 改 slug 且旧链接不 404 | `typall mv posts/old posts/new` |
| 看有哪些草稿/定时文章 | `typall list` |
| 文章到点自动上线 | `typall ci github`（每日定时构建） |
| 公众号发文 | `typall serve` 后点页面「复制到公众号」按钮 |

## 部署速览

| 策略 | 适用 | 关键配置 |
|------|------|----------|
| **GitHub Pages**（推荐） | 个人博客首选，免费 + CI 自动化 | `typall ci github` 一条命令生成工作流 |
| git 推送 | 已有 Pages 仓库 / 自管静态仓 | `[deploy.git]` repo + branch |
| Netlify / Vercel | 免运维 CDN，PR 预览 | `[deploy.netlify]` / `[deploy.vercel]` + 环境变量存 token |
| copy | 自有服务器（nginx/caddy） | `[deploy.copy]` target_dir |

生产环境三件事别忘了：配 `site.url`（SEO / RSS / og 分享卡全靠它）、`build --strict` 查死链、
`typall status` 核对定时文章。完整步骤、token 环境变量与钩子见[部署指南](typall/README.md#部署指南deploy)。

## 仓库结构

```
typall/    生成器源码（Rust crate）+ 自带内容项目 + 完整使用文档（typall/README.md）
docs/      设计文档
```

## 文档

| 文档 | 内容 |
|------|------|
| [typall/README.md](typall/README.md) | **完整使用文档**：命令、配置、写作指南、公式、主题、发布、部署指南、性能 |
| [docs/SPEC-v1.1.md](docs/SPEC-v1.1.md) | 产品规格说明书（v1.1，经技术验证修订） |
| [docs/THEME-SPEC.md](docs/THEME-SPEC.md) | 主题系统规格（唯一权威文档：占位符 / partial / class 契约与稳定性承诺） |
| [docs/THEME-GALLERY.md](docs/THEME-GALLERY.md) | 四套参考主题（曜石/纸砚/留白/柑橘）设计令牌与踩坑记录 |
| [docs/COPY-TO-PLATFORM-DESIGN.md](docs/COPY-TO-PLATFORM-DESIGN.md) | 「复制到公众号 / 知乎」方案设计与进度 |
| [docs/COMPAT.md](docs/COMPAT.md) | Typst 版本升级兼容假设清单（升级演练手册） |
| [docs/TEST-REPORT.md](docs/TEST-REPORT.md) | 全链路回归测试报告 |

## 本地文件（不入库）

| 路径 | 说明 |
|------|------|
| `typall/.typall/` `typall/public/` 等 | 编译缓存与构建产物，`typall clean` 可清除重建 |

> CJK 字体（Noto Sans CJK SC，OFL 许可）已随仓库内置在 `typall/assets/fonts/`，公式汉字开箱即用。

## 开发

```bash
cd typall
cargo test           # 全量测试
cargo clippy         # 保持零警告
```

## License

Apache-2.0（见 [LICENSE](LICENSE)）
