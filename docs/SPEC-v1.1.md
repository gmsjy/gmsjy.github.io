# Typall 完整规格说明书

**版本** 1.1
**状态** 设计阶段（经技术验证修订）
**作者** 项目发起人

> **v1.1 修订说明**：本版根据 2026-09-01 技术验证（Spike）结果修订，落实 4 处修正——
> ① 主题样式由「Typst 生成 CSS」改为「普通 CSS 文件」；② KaTeX 渲染降级为可选插件；
> ③ 修正附录 B 样例的 Typst 语法错误；④ 明确元数据仅支持字面量。
> 另锁定 Typst 版本为 0.15.1，并补充技术选型结论（见第 13 节）。

---

## 1. 项目概述

### 1.1 项目名称
**Typall** – 一个用 Rust 编写的、完全自包含的静态博客生成器，深度集成 Typst 排版引擎，以 **配置驱动** 和 **一键发布** 为核心设计理念。

### 1.2 项目目标
- 提供 **单二进制** 工具链，无需安装任何外部依赖（Typst 编译器、Node.js、Python 等）。
- 使用 **Typst** 作为唯一的内容编写格式，充分发挥其强大的排版能力，尤其针对 **数学物理公式** 进行优化。
- 通过 **单一配置文件**（`typall.toml`）管理站点、主题、构建选项和部署目标。
- 将 **发布管理** 作为一等公民，内置多种部署策略（Git、Netlify、Vercel、本地复制）。
- 生成 **纯静态 HTML/CSS**，零客户端 JavaScript，保证极致的加载速度与安全性。
- 支持 **增量编译** 与 **实时预览**，提升写作体验。

---

## 2. 核心功能模块

### 2.1 内容管理
- **文章格式**：所有内容以 `.typ` 文件存放于 `posts/` 目录。
- **元数据**：通过 Typst 的 `#let` 指令定义，例如：
  ```typst
  #let title = "量子力学入门"
  #let date = "2026-09-01"
  #let tags = ("物理", "量子")
  #let draft = false
  ```
  元数据支持：标题、日期（支持时间戳）、标签、分类、草稿状态、自定义字段。
  > **限制（v1.1 明确）**：元数据在语法层提取，**仅支持字面量**（字符串/布尔/整数/浮点/数组）。非字面量字段（如 `#let date = datetime.today()`）在 v0.x 中暂不支持，未来通过探针编译回退。
- **内容解析**：元数据通过语法层（AST）提取，正文编译一次生成 HTML。**元数据提取不产生编译开销**，整体满足"每篇仅编译一次"。
- **集合页面**：自动生成按标签、分类、日期归档的索引页。
- **独立页面**：在 `pages/` 目录下放置 `.typ` 文件，生成与文章同级的独立页面（如 `about.typ` → `/about/index.html`）。
- **草稿管理**：标记 `draft = true` 的文章在 `build` 时默认忽略（可通过 `--drafts` 包含）。

### 2.2 配置系统
- **主配置文件**：项目根目录下的 `typall.toml`（支持 `.yaml` 格式）。
- **配置结构**（详见第 4 节）：
  - `[site]`：站点全局元数据（标题、描述、URL、作者、语言等）。
  - `[build]`：构建选项（输出目录、严格模式、代码高亮主题等）。
  - `[build.math]`：数学公式渲染策略（MathML / SVG），以及编号、前缀配置。
  - `[theme]`：主题选择（内置默认或自定义路径）。
  - `[deploy]`：部署目标配置（Git / Netlify / Vercel / Copy）。
- **环境变量覆盖**：常用配置项可通过 `TP_` 前缀的环境变量覆盖（当前实现覆盖 9 项：`TP_SITE_TITLE`、`TP_SITE_DESCRIPTION`、`TP_SITE_URL`、`TP_SITE_AUTHOR`、`TP_SITE_LANGUAGE`、`TP_BUILD_OUTPUT_DIR`、`TP_BUILD_STRICT`、`TP_BUILD_DRAFTS`、`TP_THEME_NAME`），便于 CI/CD。其余配置项暂不支持环境变量覆盖。

### 2.3 主题系统（v1.1 修正）
- **内置默认主题**：提供明/暗模式，响应式设计，干净优雅。
- **自定义主题**：通过 `themes/<name>/` 目录定义，包含：
  - `template.html`：控制 HTML 骨架结构，使用 `{{占位符}}` 模板引擎（`{{body}}`、`{{site_title}}`、`{{params.xxx}}` 等，详见 README）。
  - `style.css`：**普通 CSS 文件**，定义全局样式与主题外观。
  - `static/`：主题静态资源（图片、字体、额外 CSS/JS）。
- **关键修正（v1.1）**：Typst 0.15 **不导出 CSS**（仅输出 MathML 排版所需的少量样式）。因此主题外观由 `style.css` 负责，Typst 只负责语义化 HTML 结构，二者职责分离。HTML 骨架亦由 `template.html` 占位符引擎承担（原 `template.typ` + `html.elem` 方案经实测改为此方案，见 v1.2 修订说明）。
- **主题参数**：可在配置中传递参数（如 `[theme.params] accent_color = "#007acc"`），模板内通过 `{{params.xxx}}` 访问。
- **打包分发**：主题可压缩为 `.zip` 并通过 URL 安装（v0.3 规划）。

### 2.4 发布管理（集成部署）
- **构建**：`typall build` 生成完整的静态站点到 `public/` 目录（可配置）。
- **部署命令**：`typall deploy` 根据 `[deploy]` 配置执行部署。
- **支持策略**：
  - **Git**：自动提交并推送 `public/` 内容到指定远程仓库的分支（如 `gh-pages`）。
  - **Netlify**：通过 API Token 触发部署（需要 `site_id` 和 `token`）。
  - **Vercel**：类似 Netlify，通过 API 触发。
  - **Copy**：将 `public/` 复制到指定本地目录（用于手动上传或 rsync）。
- **部署钩子**：支持部署前/后执行自定义脚本（`pre_deploy`、`post_deploy`）。
- **回滚**：Git 部署自动打标签（如 `deploy-YYYYMMDD-HHMMSS`），方便回滚。

### 2.5 开发服务
- **命令**：`typall serve [--port]`
- **功能**：
  - 启动 HTTP 服务器（默认 `localhost:8080`）。
  - 监听项目文件变化，自动增量重建并刷新浏览器（Live Reload）。
  - 显示编译错误信息（行号、上下文）。

### 2.6 辅助工具
- **`typall init [name]`**：创建新项目脚手架（包含示例文章、配置、主题）。
- **`typall new <post>`**：在 `posts/` 下生成新文章，自动填充当前日期和标题。
- **`typall clean`**：删除所有构建产物（`public/` 和缓存目录）。
- **`typall check`**：仅进行语法检查和链接检查（不生成输出）。

---

## 3. 技术架构

### 3.1 技术选型（v1.1 更新）

| 组件 | 技术 | 说明 |
|------|------|------|
| **语言** | Rust 2021 Edition | 高性能、内存安全、生态丰富 |
| **排版引擎** | `typst` = 0.15.1（锁定） | **以 Rust 库形式内嵌**，编译 `.typ` 并导出 HTML/MathML；HTML 导出为实验性功能，故锁版本 |
| **HTML 导出** | `typst-html` 0.15.1 | 运行时启用 `Feature::Html`，无需 cargo feature |
| **嵌入字体** | `typst-assets`（`features=["fonts"]`） | 离线可用；注意 `fonts` 非默认 feature |
| **配置解析** | `serde` + `toml` / `yaml` | 灵活配置格式支持 |
| **CLI** | `clap` (derive) | 命令解析 |
| **文件监听** | `notify` | 跨平台文件变化检测 |
| **HTTP 服务器** | `axum` 或 `hyper` | 轻量异步 Web 服务 |
| **Git 操作** | `git2` | Rust 绑定 libgit2，用于 Git 部署 |
| **HTTP 客户端** | `reqwest` | 调用 Netlify/Vercel API |
| **并发处理** | `rayon` | 并行编译多个 `.typ` 文件 |
| **HTML 解析** | `scraper` | 用于死链检查的链接提取 |

### 3.2 数据流与工作流程
```
┌─────────────────┐
│ 用户编写 .typ   │
└────────┬────────┘
         ▼
┌─────────────────┐
│ typall build    │
└────────┬────────┘
         ▼
┌─────────────────────────────────────────┐
│ 加载配置 typall.toml                    │
│ 扫描 posts/ 和 pages/ 下所有 .typ       │
│ 语法层提取元数据（AST，<1ms）           │
└────────┬────────────────────────────────┘
         ▼
┌─────────────────────────────────────────┐
│ 对每个 .typ 并行编译（rayon）：         │
│  - 生成 HTML 正文                       │
│  - 数学公式渲染为 MathML                │
└────────┬────────────────────────────────┘
         ▼
┌─────────────────────────────────────────┐
│ 组装站点：                              │
│  - 应用主题（template.html + style.css） │
│  - 索引页 (按时间、标签、分类)          │
│  - 独立页面                             │
│  - RSS/Atom 订阅源                      │
│  - sitemap.xml                          │
└────────┬────────────────────────────────┘
         ▼
┌─────────────────────────────────────────┐
│ 写入 public/ 目录，保持目录结构         │
│ 复制静态资源 (assets/ 中的 CSS/图片)    │
└────────┬────────────────────────────────┘
         ▼ (可选)
┌─────────────────┐
│ typall deploy   │
│ 根据配置推送    │
└─────────────────┘
```

### 3.3 性能优化
- **增量编译**：通过内容哈希缓存，仅重建变化文件。
- **并行处理**：使用 `rayon` 充分利用多核 CPU。
- **内存缓存**：预加载主题模板和宏文件，避免重复 I/O。
- **压缩输出**：自动压缩 HTML/CSS（gzip 或 brotli）以减小传输体积。

---

## 4. 配置详解

`typall.toml` 完整示例：

```toml
[site]
title = "物理与数学博客"
description = "探索自然律动的文字"
url = "https://physics.blog"
author = "Dr. Typst"
language = "zh-CN"
# 可选：版权信息、头像、社交链接等

[build]
output_dir = "public"
strict = false                # 是否严格模式（死链检查）
highlight_theme = "github-dark"  # 代码高亮主题（内置多个）
drafts = false                # 构建时是否包含草稿

[build.math]
renderer = "mathml"           # mathml | svg
number_equations = true
equation_prefix = "公式"      # 显示为 "公式 (1)"

[theme]
name = "default"              # 或自定义路径 path = "themes/my-theme"
[theme.params]                # 主题自定义参数
accent_color = "#007acc"
font_family = "Latin Modern"

[deploy]
strategy = "git"              # git | netlify | vercel | copy

[deploy.git]
repo = "git@github.com:user/blog.git"
branch = "gh-pages"
commit_message = "Update blog at {timestamp}"
auto_push = true

[deploy.netlify]              # 可选
site_id = "abc123"
token = "env:NETLIFY_TOKEN"   # 从环境变量读取

[deploy.copy]                 # 本地复制
target_dir = "/var/www/blog"
```

---

## 5. 命令行接口（CLI）

所有命令统一前缀为 `typall`。

| 命令 | 参数 | 描述 |
|------|------|------|
| `typall init [name]` | `[name]` 可选项目目录名 | 创建新项目（含示例配置与文章） |
| `typall new <post>` | `<post>` 文章文件名（不含扩展名） | 在 `posts/` 下生成新文章模板，自动填入日期 |
| `typall build` | `--strict` 启用死链检查；<br>`--drafts` 包含草稿；<br>`--output <dir>` 自定义输出目录 | 构建完整站点 |
| `typall serve` | `--port <number>` 监听端口，默认 8080；<br>`--open` 自动打开浏览器 | 启动开发服务器，支持 Live Reload |
| `typall deploy` | `--target <strategy>` 临时覆盖部署策略；<br>`--dry-run` 仅模拟不实际执行 | 按配置部署站点 |
| `typall clean` | 无 | 删除 `public/` 和缓存 |
| `typall check` | `--strict` 严格检查 | 语法与链接检查，不生成输出 |
| `typall version` | 无 | 显示版本信息 |

---

## 6. 数学物理公式支持（核心特色）

Typall 原生支持 Typst 的所有数学语法，并针对公式排版提供以下增强：

### 6.1 Typst 数学语法支持
- **行内公式**：`$E = mc^2$`
- **独立显示公式**：`$ lim_(n->oo) (1 + 1/n)^n = e $`
- **物理单位**：`$ 3.0 "m/s"^2 $`
- **矩阵/方程组**：`$ mat(1,2; 3,4) $`
- **希腊字母、微积分符号**：`partial`、`nabla`、`integral` 等。
- **微分记号**：`dif x`（如 `$ integral_(-oo)^(oo) f(x) dif x $`）

### 6.2 渲染策略（可配置，v1.1 修正）

| 策略 | 特点 | 适用场景 |
|------|------|----------|
| **MathML**（默认） | 零 JavaScript，浏览器原生支持，加载快 | 现代浏览器，极致性能 |
| **SVG** | 输出内联 SVG（`html.frame`），像素级精确，完全离线 | 公式数量少，追求绝对一致性 |

> **v1.1 修正**：原 KaTeX 策略依赖客户端 JS，与"零客户端 JS"目标矛盾，已降级为未来插件系统的可选组件（不在 v0.x 范围）。

### 6.3 公式编号与引用
- **自动编号**：独立公式（`$ ... $` 块级）自动编号，可通过 `number_equations = true` 开启。
- **交叉引用**：在文章中可通过 `@eq:maxwell` 标记并引用（需在构建时解析）。

### 6.4 物理宏支持
用户可在 `assets/macros.typ` 中定义常用物理常量与算子，并在文章中导入：
```typst
#import "/assets/macros.typ": *
#let hbar = ℏ
$ hbar = (h)/(2 pi) $   // 使用自定义宏
```

### 6.5 Typst 第三方包与插图（v0.4 新增）

内嵌的 Typst 编译器支持解析 `@preview` 命名空间的第三方包：首次使用自动从
`packages.typst.org` 下载并解压到 `.typall/packages/<namespace>/<name>/<version>/`，此后离线可用；
传递依赖（包引用的其他包）一并自动下载。`World` 实现通过 `VirtualRoot::Package(spec)` 把包内文件
映射到缓存目录。

配套的 WebAssembly 插件（Typst `plugin()`）经同一 `file()` 通道加载，**无需额外 `PluginWorld` 实现**
（Typst 0.15 的 wasmi 运行时随编译器内置）。因此像 **CeTZ**（受 TikZ 启发的矢量绘图库，含 WASM 内核
与 `oxifmt` 依赖）可直接在文章中编写插图：

```typst
#import "@preview/cetz:0.5.2"

#html.frame(cetz.canvas(length: 4cm, {
  import cetz.draw: *
  line((0, 0), (3, 2), mark: (end: ">"))
  circle((0, 0), radius: 0.06, fill: black)
}))
```

> **HTML 输出说明**：Typst 0.15 的 HTML 导出不渲染原生矢量图形（`line`/`curve`/`circle` 等），
> 故插图需用 `html.frame` 包一层，令其以**内联 SVG** 输出（与公式 SVG 渲染同源）。

---

## 7. 发布与部署集成

### 7.1 Git 部署流程
1. 检查当前目录是否为 Git 仓库（否则初始化）。
2. 克隆远程仓库到临时目录（或使用 worktree）。
3. 将 `public/` 内容复制到目标分支（如 `gh-pages`）。
4. 自动提交并推送，提交信息可自定义（支持 `{timestamp}` 变量）。
5. 打标签以便回滚。

### 7.2 Netlify / Vercel 部署
- 通过 `site_id` 和 API Token 触发部署。
- Token 建议通过环境变量注入（配置中使用 `env:VAR_NAME`）。

### 7.3 本地复制
- 将 `public/` 复制到指定本地目录，适合配合 rsync 或 FTP 使用。

### 7.4 部署钩子
- 支持 `pre_deploy` 和 `post_deploy` 脚本（Shell 或可执行文件），在部署前后执行。

---

## 8. 安全性

- **敏感信息**：所有 Token、密码等通过环境变量注入，不允许硬编码在配置文件中。
- **路径遍历**：所有文件操作均通过 `VirtualPath::realize()` 映射（官方函数自带逃逸防护），并限制在项目目录内。
- **链接检查**：严格模式下检查所有输出链接，避免用户暴露内部路径。
- **内容安全**：生成的 HTML 自动转义，防止 XSS。

---

## 9. 性能目标

- **构建速度**：100 篇文章（含平均 10 个公式）的站点，首次构建 < 2 秒，增量构建 < 0.5 秒。
- **内存占用**：< 100 MB。
- **输出体积**：每篇文章 HTML 压缩后 < 20 KB（不含图片）。
- **并发处理**：充分利用 CPU 核心数。

> 技术验证实测（debug 单篇）：编译 ~457ms、元数据提取 ~0.5ms、World 初始化 ~30ms。100 篇目标需 release + rayon 并行实测。

---

## 10. 路线图（版本规划）

| 版本 | 里程碑 |
|------|--------|
| **v0.1** | 基础构建、开发服务器、默认主题、Git 部署、MathML 支持 |
| **v0.2** | 主题自定义、增量编译、死链检查、Sitemap 与 RSS、公式编号与引用 |
| **v0.3** | Netlify/Vercel 部署、多语言站点、SVG 渲染 |
| **v1.0** | 稳定 API、插件系统（WASM）、PDF 导出、CMS 集成预览、KaTeX 插件 |

> **v1.1 调整**：KaTeX 支持后移到 v1.0 插件系统；公式编号与引用从 v0.3 提前到 v0.2。

---

## 11. 开发与贡献指南

- **代码仓库**：[待定]
- **构建**：`cargo build --release`
- **测试**：`cargo test`（包含单元测试和集成测试）。
- **文档**：通过 `cargo doc` 生成 API 文档，并提供用户手册（`docs/`）。

---

## 12. 附录

### A. 项目目录结构示例
```
my-blog/
├── typall.toml
├── posts/
│   ├── 2026-09-01-quantum.typ
│   └── 2026-09-02-relativity.typ
├── pages/
│   └── about.typ
├── assets/
│   ├── images/
│   ├── fonts/
│   └── macros.typ
├── themes/
│   └── default/        (内置主题，可覆盖)
└── public/             (构建输出)
```

### B. 典型文章模板（v1.1 修正语法）
```typst
#let title = "量子力学的波函数"
#let date = "2026-09-01"
#let tags = ("物理", "量子")
#let draft = false

= 波函数的统计诠释

Born 规则指出，波函数 $Psi(x,t)$ 的模方给出概率密度：

$ P(x,t) = |Psi(x,t)|^2 $

归一化条件：

$ integral_(-oo)^(oo) |Psi(x,t)|^2 dif x = 1 $
```

> **修正说明**：原 v1.0 样例中的 `dx`、`diff`、`hbar` 均为非法 Typst 记号，已分别修正为 `dif x`、`dif`、`ℏ`。

---

## 13. 技术验证结论（v1.1 新增）

2026-09-01 Spike 实测结论（详见 `SPIKE-REPORT.md`）：

| 结论 | 说明 |
|------|------|
| Rust 库集成可行 | `typst::compile::<HtmlDocument>` 一步到位，无需外部 CLI |
| MathML 渲染完美 | 积分/分数/矩阵/重音全部转语义化 `<math>`，零 JS |
| 元数据 AST 提取 | <1ms，满足"仅编译一次" |
| CSS 需自建 | Typst 仅输出 MathML 样式，主题 CSS 独立维护 |
| 主要风险 | Typst HTML 导出标记实验性，需锁版本 0.15.1 |

**现成参考**：typage（Rust SSG，但 shell out 外部 CLI、禁生产）。Typall 的单二进制是核心差异化优势。

---

**本规格书作为 Typall 项目的设计蓝图，后续开发将严格遵循其定义。如有修改，需经项目维护者讨论并更新文档。**
