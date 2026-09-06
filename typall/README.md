# Typall

一个用 **Rust** 编写的、**完全自包含**的静态博客生成器，深度集成 **Typst** 排版引擎。以「配置驱动」和「一键发布」为核心。

## 特性

- **单二进制**：内嵌 Typst 编译器，无需安装 Typst CLI、Node.js、Python 等任何外部依赖
- **Typst 写作**：`.typ` 是唯一的内容格式，充分发挥其排版与数学公式能力
- **数学公式 → SVG / MathML**：SVG（推荐）输出内联矢量并完美渲染公式编号；切 MathML 可改 `typall.toml`
- **配置驱动**：一个 `typall.toml` 管理站点、主题、构建、部署
- **极速构建**：100 篇文章（含公式）构建仅 **0.3 秒**
- **一键发布**：内置 Git / 本地复制 / Netlify / Vercel 部署
- **真增量编译**：内容哈希 + 依赖追踪，仅重编变化文件，改一篇只编一篇
- **Live Reload**：`serve` 实时预览——SSE 推送，构建错误直接在浏览器浮层展示

## 快速开始

```bash
# 1. 创建新项目
typall init my-blog
cd my-blog

# 2. 写文章（posts/ 下 .typ 文件）

# 3. 构建
typall build

# 4. 本地预览（Live Reload）
typall serve --open

# 5. 发布
typall deploy
```

分步说明（以一个真实的写作循环串起来）：

1. **建站**：`init` 生成 `typall.toml`、`posts/` 示例文章与目录骨架。已装好可直接 `typall serve --open` 看到站点；
2. **写作**：`typall new 二次函数` 生成 `posts/2026-09-06-二次函数.typ`，编辑器打开写正文——
   元数据用顶部 `#let` 定义（title/date/tags/draft/series…，见「写作指南」），公式直接用 Typst 数学语法；
3. **预览**：`serve` 启动后保存即重建刷新；构建错误显示在浏览器底部浮层；草稿也会出现在预览里；
4. **核对**：`typall status` 看站点概况，`typall list` 看每篇文章的状态（已发布/草稿/定时）；
5. **上线**：`typall build --strict` 通过后 `typall deploy`。首次部署前先到「部署指南」选好策略、配好 `site.url`。

## 命令

所有子命令都支持全局参数 `--dir`（短写 `-C`）指定项目根目录，放在子命令前或后均可，从此不必受「必须在项目目录里启动」的限制：

```bash
typall --dir ~/blogs/physics serve     # 在任何目录启动指定项目的服务
typall build -C ~/blogs/physics        # 等价：参数放在子命令后
```

| 命令 | 说明 |
|------|------|
| `typall init [name]` | 创建项目脚手架（含示例文章、配置） |
| `typall new <post>` | 在 `posts/` 下生成新文章模板 |
| `typall build` | 构建站点到 `public/`（`--strict` 死链检查、`--drafts` 含草稿） |
| `typall serve` | 开发服务器 + Live Reload（`--port` 端口、`--open` 开浏览器、`--host 0.0.0.0` 开放局域网并打印手机扫码二维码） |
| `typall deploy` | 按配置部署（`--target` 临时覆盖、`--dry-run` 模拟） |
| `typall publish --to markdown` | 将文章导出为 Markdown（单篇 → 平台，见「发布（publish）」） |
| `typall check` | 语法检查（不生成输出） |
| `typall list` | 文章清单：状态（已发布/草稿/定时）、日期、标题、slug |
| `typall status` | 站点概况：文章统计、主题、site.url、上次构建、发布账本 |
| `typall mv <old> <new>` | 重命名 slug 并自动写入 `aliases`（旧链接构建后自动跳转） |
| `typall ci github` | 生成 GitHub Pages 工作流（push 部署 + 每日定时构建实现定时发布） |
| `typall pdf` | 导出文章为 PDF（原生 paged 渲染，`--slug` 单篇、`--output` 目录、`--drafts` 含草稿） |
| `typall theme package` | 打包 `themes/<name>/` 为 `themes-<name>.zip`（`--name` 指定，默认取配置） |
| `typall theme install <source>` | 从本地 zip 路径或 http(s) URL 安装主题到 `themes/` |
| `typall clean` | 删除构建产物与缓存 |
| `typall version` | 显示版本 |

> 项目根以 `typall.toml` + `posts/` 为标志：启动目录两者皆无（又未用 `--dir` 指定）时，`build` / `serve` / `check` / `publish` / `pdf` 会直接报错提示，而不是静默生成一个空站点。

## 配置

项目根目录 `typall.toml`（也支持 `.yaml`）：

```toml
[site]
title = "物理与数学博客"
description = "探索自然律动的文字"
url = "https://physics.blog"   # 用于生成 RSS / sitemap
author = "Dr. Typst"
language = "zh-CN"

[build]
output_dir = "public"
strict = false
posts_per_page = 20            # 首页每页文章数（分页），默认 20

[build.math]
number_equations = true        # 公式编号（见「已知限制」）
equation_prefix = "公式"

[theme]
name = "default"               # 或 "my-theme"（对应 themes/my-theme/）

[theme.params]                 # 主题参数，在主题模板中用 {{params.xxx}} 引用
accent_color = "#007acc"

[deploy]
strategy = "git"               # git | copy | netlify | vercel

[deploy.git]
repo = "git@github.com:user/blog.git"
branch = "gh-pages"
commit_message = "Update blog at {timestamp}"
auto_push = true

[deploy.copy]
target_dir = "/var/www/blog"

[deploy.netlify]               # strategy = "netlify"
site_id = "your-netlify-site-api-id"
auth_token = "nfp_xxx"         # 或环境变量 NETLIFY_AUTH_TOKEN

[deploy.vercel]                # strategy = "vercel"
project_id = "prj_xxx"         # 可选；留空则自动创建新项目
auth_token = "xxx"             # 或环境变量 VERCEL_TOKEN
```

`--target netlify|vercel` 可临时覆盖配置中的 strategy。部署前执行 `pre_deploy`、成功后执行 `post_deploy` 钩子（shell 命令，`--dry-run` 均跳过）。token 建议用环境变量（`NETLIFY_AUTH_TOKEN` / `VERCEL_TOKEN`）避免写入配置文件：

```bash
NETLIFY_AUTH_TOKEN=nfp_xxx typall deploy --target netlify
typall deploy --target netlify --dry-run   # 仅打印将要调用的 API 与参数
```

> 以上是配置项参考；**按平台走完部署的完整步骤**（GitHub Pages 三种姿势、token 环境变量、
> 生产检查清单、定时发布联动）见下文[「部署指南（deploy）」](#部署指南deploy)。

## 发布（publish）

`deploy` 是**整站 → 主机**；`publish` 是**单篇 → 平台**（公众号 / 知乎 / 掘金等），从同一份文档 IR 按目标重新变换，不复用站点 HTML。当前可用目标：

| 目标 | 说明 |
|------|------|
| `markdown` | 导出 Markdown + YAML frontmatter，到 `<out_dir>/markdown/<slug>.md` |
| `wechat` | 生成微信公众号富文本 HTML 片段（内联样式 + SVG/公式栅格化为 PNG）到 `<out_dir>/wechat/<slug>.html` |

```bash
typall publish                                  # 全部文章 → publish/markdown/（--to 省略默认 markdown）
typall publish --to wechat                      # → publish/wechat/（公众号富文本）
typall publish --slug math-trig                 # 只发布一篇（slug 可写 posts/math-trig 或 math-trig）
typall publish --output out/md                  # 自定义导出目录
typall publish --drafts                         # 包含草稿
typall publish --status                         # 查看发布状态库（.typall/publish.json）
typall publish --force                          # 内容未变也强制重新发布
```

### 微信目标说明

`--to wechat` 生成可直接粘贴到公众号编辑器的富文本 HTML 片段（**纯本地导出，全程不触网**）：

- **SVG/公式栅格化为 PNG**（`resvg` 2x 高清渲染，PNG 以 data URI 内嵌）：公众号保存层会剥离 `<svg>` 的 `xlink:href` 与 `<symbol>` 的 `id`，使 typst 公式字形（`use`→`symbol` 引用结构）失效变空白；因此公式与 cetz 插图统一栅格化为 PNG 位图、以 `<img>` 引用，块级公式的 `eq-num` 编号 span 在栅格化时单独保留；
- **行内公式**与文字基线对齐（`vertical-align:middle`）；**块级公式 / 插图**外层居中段落；
- **站内锚点链接**（如"式 1"）降级为纯文本；
- 全套内联样式（标题字号 / 行高 / 表格边框 / 代码块底色等）保证不同平台渲染一致；
- 当前 `[build.math] renderer` 必须为 `svg`（mathml 会被拒绝）。

**发布到公众号的流程**：`typall serve` 打开文章页，点「复制到公众号」浮动按钮
（自动做 use/symbol 展开、图片绝对化、排版样式内联），直接粘贴进公众号编辑器。
粘贴保存时编辑器会把外链图片**自动转存**为微信图床文件，因此无需 API 上传素材——
早期版本的草稿 API 推送（`--push`、素材转传、`typall.wechat.toml` 凭据、
token/媒体缓存）已整体移除。

- frontmatter 含 `title / date / updated / tags / categories / draft / excerpt`，并标注 `math: typst`；
- **公式**输出 Typst 数学源码（`$..$` / `$$..$$`）；LaTeX 转换是知乎等目标的后续规划；
- **插图**（cetz 等内联 SVG）保留为原生 HTML 块，不支持的平台会剥离；
- **图片资产**：正文里的 data URI 图片自动提取为内容寻址文件 `assets/<指纹>.<ext>`（相同内容去重），`src` 重写为相对路径；
- **幂等发布**：每次发布记录内容指纹，未变的文档自动跳过（`--force` 强刷），适合 CI 增量发布；
- 导出目录自动清理孤儿文件（`assets/` 内容寻址目录除外）；默认输出根由 `[publish] out_dir`（默认 `publish`）控制。

## 部署指南（deploy）

`typall build` 产出的是纯静态文件（`public/`，零 JS、自带 gzip/brotli 预压缩副本），任何静态托管都能直接服务。四种部署策略按需选择，都支持 `--dry-run` 模拟执行。

### 部署前检查清单

| 检查项 | 命令/位置 | 说明 |
|--------|----------|------|
| `site.url` 已配置 | `typall status` | RSS / sitemap / canonical / og 分享卡全部依赖它；未配置时这些产物自动省略 |
| 死链检查通过 | `typall build --strict` | 有死链会构建失败并列出明细 |
| 定时文章核对 | `typall list` | 未来日期的文章不在本次产物中，确认它们确实该等 |
| 草稿未泄漏 | `typall status` | 正式构建永远不含 `draft = true`，无需手动排查 |

### 方式一：GitHub Pages（推荐）

一条命令生成现成的工作流，push 即部署：

```bash
typall ci github          # 写入 .github/workflows/deploy.yml
git add .github && git commit -m "ci: add deploy workflow" && git push
```

然后到仓库 **Settings → Pages → Source 选 "GitHub Actions"**，完成。此后每次 push 到 main 自动构建部署。

这份工作流还内置**每日定时构建**（cron）——「定时发布」因此全自动：文章 `date` 写未来日期，到点由 cron 触发的构建自动上线，无需人在电脑前。若不想要定时构建，删掉工作流里的 `schedule:` 段即可。

> 二进制获取方式在工作流里以注释给出两种选择（crates.io `cargo install` / 从 release 下载），按自己的分发渠道改一行。

### 方式二：git 推送到静态仓

适合已有 Pages 仓库、或想把产物推到独立分支（`gh-pages`）的场景：

```toml
[deploy]
strategy = "git"

[deploy.git]
repo = "git@github.com:user/blog.git"
branch = "gh-pages"
commit_message = "Update blog at {timestamp}"   # 支持 {timestamp} 占位
auto_push = true                                 # false 则只 commit 不 push
```

```bash
typall deploy                    # 构建 + commit（+ push）
typall deploy --dry-run          # 只打印将执行的 git 操作
```

`pre_deploy` / `post_deploy` 钩子（`[deploy]` 下的 shell 命令）可插入自定义步骤（如通知、CDN 刷新），`--dry-run` 时一并跳过。

### 方式三：Netlify / Vercel

适合要 CDN、预览部署、自定义域名的用户。token 一律建议放环境变量，别写进配置文件：

```toml
[deploy]
strategy = "netlify"             # 或 "vercel"

[deploy.netlify]
site_id = "your-site-api-id"     # Netlify 后台 Site settings → API ID

[deploy.vercel]
project_id = "prj_xxx"           # 可选；留空自动创建新项目
```

```bash
export NETLIFY_AUTH_TOKEN=nfp_xxx        # 或 VERCEL_TOKEN=xxx
typall deploy
typall deploy --target vercel            # 临时改道另一平台（覆盖配置）
typall deploy --dry-run                  # 打印将要调用的 API 与参数
```

### 方式四：自有服务器（copy）

nginx / caddy 直接服务静态目录：

```toml
[deploy]
strategy = "copy"

[deploy.copy]
target_dir = "/var/www/blog"
```

```bash
typall deploy && sudo systemctl reload nginx
```

利用预压缩产物开满优化：nginx 开 `gzip_static on;`（有 `.br` 时配 `brotli_static`），静态文件直接返回预压缩副本，省去实时压缩开销。

### 部署 ≠ 发布：两件事各管各的

| 命令 | 管什么 | 产物 |
|------|--------|------|
| `typall deploy` | **整站上线**：全站静态文件到托管平台 | `public/` → 服务器 |
| `typall publish --to markdown` | **单篇分发**：文章导出成 Markdown（含图片内容寻址、幂等指纹） | `publish/markdown/` |
| `typall serve` + 复制按钮 | **公众号发文**：预览页一键复制富文本，粘贴进公众号编辑器 | 剪贴板 |

博客作者的典型节奏：`serve` 写作 → `build --strict` → `deploy` 上线 → 需要时 `publish` 导出分发。

### 分享图（cover）

front-matter 可显式指定文章的社交分享图（og:image 第一优先级）：

```typst
#let cover = "assets/images/cover.png"
```

不指定时依次回退：正文首图 → 自动生成的分享卡（见「SEO」）。

### 定时发布的部署联动

定时发布 = 文章 `date` 写未来日期 + **到点有人重新构建**。三条实现路径：

1. `typall ci github` 的工作流自带每日 cron——GitHub 用户零配置；
2. 服务器 crontab：`0 6 * * * cd /path/to/blog && typall build && typall deploy`；
3. 本机 `typall serve` 常驻时任何保存都会触发重建（适合边写边等，不适合无人值守）。

用 `typall status` 随时确认有几篇定时文章在排队、`typall list` 看它们的具体日期。

## 写作指南

### 文章元数据

每篇文章顶部用 `#let` 定义元数据（**仅支持字面量**）：

```typst
#let title = "量子力学入门"
#let date = "2026-09-01"
#let tags = ("物理", "量子")
#let draft = false    # true 则 build 默认忽略
```

支持的字段：`title`、`date`、`tags`、`categories`、`draft`、`excerpt`、`updated`、`series`、`series_weight`，以及任意自定义字段。

`tags`、`categories`、`date`、`series` 会自动生成集合页面：

| 集合页 | URL | 说明 |
|--------|-----|------|
| 标签云 | `/tags/` | 列出所有标签（按文章数排序） |
| 标签详情 | `/tags/<标签>/` | 该标签下的所有文章（最新在前） |
| 分类列表 | `/categories/` | 列出所有分类 |
| 分类详情 | `/categories/<分类>/` | 该分类下的所有文章 |
| 日期归档 | `/archive/` | 按年月分组归档 |
| 专栏列表 | `/series/` | 列出所有专栏（按文章数排序） |
| 专栏详情 | `/series/<专栏名>/` | 该专栏全部文章，按**课程顺序**排列 |

### 专栏（series）

专栏用于**教材式连载**（如高中数学/物理系列）：

```typst
#let series = "高中物理"      # 同名文章组入同一专栏
#let series_weight = 3        # 课序，越小越靠前；缺省排最后
```

- 专栏详情按 `series_weight` 升序 → `date` 升序排列（**课程顺序**，与标签页「最新在前」相反）；
- 专栏内每篇文章底部显示**专栏导航**：专栏首页链接 + `2 / 17` 位置 + 上一篇/下一篇；
- 站点导航栏自动出现「专栏」入口（无专栏文章时不出现）；
- 专栏名会 slug 化为 URL 路径；**两个专栏名的 slug 冲突时构建直接报错**（否则专栏页会静默互相覆盖）。

标签/分类名会自动「slug 化」为 URL 路径（中文保留，空格→连字符），例如标签 `hello world` → `/tags/hello-world/`。

### 摘要与列表预览

首页、集合页、归档页的列表项会显示文章摘要，两种方式：

1. **手动指定**（推荐，精确控制）：在元数据中写 `#let excerpt = "一句话简介"`。
2. **自动截取**：未指定时自动取正文前约 200 字，剥离标签后作为摘要。

### 定时发布

`date` 为**未来日期**的文章在 `build` 时自动过滤（不会出现在站点中），到期后再构建即自动上线。适合"周六自动发"这类场景——提前写好文章、填好未来日期，到点重新 `typall build` 即可。本地调试可用 `typall build --drafts` 强制包含。

**自动上线靠定时构建**：静态站不会自己醒来。三种方式任选：

1. `typall ci github` 生成的工作流已内置**每日定时构建**（cron），推到 GitHub 即全自动；
2. 服务器 cron：`0 6 * * * cd /path/to/blog && typall build && typall deploy`；
3. `typall serve` 常驻时的任何文件保存也会顺带重建（不精确到点，仅适合作者本机）。

用 `typall status` / `typall list` 可随时确认有几篇定时文章在排队。

### slug 变更与旧链接（aliases）

改 slug 后旧链接会 404——用 `typall mv` 一步到位：

```bash
typall mv posts/old-name posts/new-name
# 等价于：重命名文件 + 在 front-matter 写入 #let aliases = ("posts/old-name",)
```

构建时为每条别名生成跳转页（meta refresh + canonical 指向新地址），旧链接不再 404；从 front-matter 删掉别名后，跳转页随孤儿清理自动消失。也可手写：

```typst
#let aliases = ("old-slug", "another-old")
```

### 最后更新

元数据支持 `#let updated = "2026-09-02"` 标记修订时间：

- 文章页显示「更新于 2026-09-02」；
- sitemap 的 `<lastmod>` 优先取 `updated`，未填写时回退 `date`。

### 目录（TOC）

文章页自动生成目录：正文中的 `= 一级`（h2）与 `== 二级`（h3）标题会注入锚点并聚合为「目录」折叠块（`<details>`）。长文默认展开，样式可用主题 CSS 定制（`.toc` / `.toc-list`）。

### 分页

首页按 `[build] posts_per_page`（默认 20）分页，生成 `/page/2/`、`/page/3/`… 并在页脚显示分页导航。

### 数学公式

Typst 原生数学语法，注意几个**易错点**：

```typst
行内公式 $E = m c^2$。

块级公式（前后加空格）：

$ integral_(-oo)^(oo) e^(-x^2) dif x = sqrt(pi) $

$ mat(1, 2; 3, 4) $
```

> ⚠️ 常见错误写法 → 正确写法：
> - `dx` → `dif x`（微分）
> - `diff` → `dif`（`(dif f)/(dif x)`）
> - `hbar` → `ℏ`（直接输入 Unicode，或自定义宏）

### 渲染策略（MathML / SVG）

在 `typall.toml` 切换 `[build.math].renderer`：

```toml
[build.math]
# renderer = "mathml"  # 浏览器原生 MathML，零客户端 JS ⚠️ 见下方"已知问题"
renderer = "svg"       # 默认推荐：像素级一致 + 公式编号完整渲染
```

**两种策略对比**：

| 策略 | 输出 | 优点 | 代价 / 风险 | 适用 |
|------|------|------|------|------|
| **`svg`（默认推荐）** | 内联 `<svg>` 矢量 | **像素级一致**（与 PDF 同源排版）、**公式编号 `(1)` 完整渲染**、不依赖浏览器公式渲染差异、行内公式排版稳定 | 产物略大；公式无法文本复制（图片式） | **推荐作为默认**（当前示例配置） |
| `mathml` | `<math>` 元素 | 零 JS、文本可选可复制、辅助阅读器友好、产物最小 | ⚠️ 块级公式 `<mfrac>` 在 Chromium 系（含 Edge）上**有显示错乱 bug**（分子分母被压成竖列）；CSS 兜底无效 | 不推荐在 v0.3.x 使用 |

SVG 模式由 Typst 的 `html.frame` 实现——所有公式（含行内）在构建期布局成矢量并内联进 HTML。
**完全离线、零客户端 JS**（与 MathML 模式保持一致），行内公式与块级公式自动区分处理（行内 `box` 包裹保持 inline 排版）。

### 公式内汉字（CJK 字体）

Typst 数学模式的字体回退链**不含 CJK 字体**，公式里的汉字（如 `$ vec(F_"合") $` 的下标「合」）需要 CJK 字体才能渲染。Typall 仓库**自带 [Noto Sans CJK SC](https://github.com/notofonts/noto-cjk)（OFL 开源许可，`assets/fonts/` 下）**，公式汉字开箱即用，无需任何配置。

若自行替换字体文件，注意**清缓存全量重编**：

```bash
rm -rf .typall/cache && typall build
```

> 缓存 key 只含源码哈希，字体变化不会自动触发失效，必须手动清缓存。数学回退在字族耗尽后会扫描全字体库自动命中 CJK 字形，无需在文章里写 show rule。请勿放入版权字体（如 SimHei/微软雅黑）对外分发。

### 交叉引用与跳转

文章内可用 `@` 自动交叉引用与 `#link` 手动链接，二者都会渲染为**可点击的站内锚点**（HTML `<a href="#...">`，点击跳转到对应元素）。

**编号前提**：`@` 引用显示"类型 + 编号"，被引用对象必须有编号：

| 引用对象 | 编号来源 |
|---|---|
| 公式 | 引擎自动编号（`[build.math]`，默认 `公式 (1)`），开箱可用 |
| 标题（章/节） | 文章内自行开启：`#set heading(numbering: "1.1")` |
| 图 | 文章内自行开启：`#set figure(numbering: "1")` |

> 未开启编号就 `@` 引用会直接**编译失败**：`cannot reference heading without numbering`。

```typst
#set heading(numbering: "1.1")    // 引用标题前必须开启编号
#set figure(numbering: "1")       // 引用图前必须开启编号

= 引言 <intro>                    // 给标题打标签
== 实验设计 <sec1>

#figure(image("assets/cover.png"), caption: [封面]) <fig1>   // 给图打标签（图片路径须真实存在）
$ E = m c^2 $ <eq:einstein>                           // 给公式打标签

见 @intro、@sec1、@fig1、@eq:einstein。  // 自动显示编号（含语言化前缀）
见 @intro[引言]。                       // 自定义显示文字
请参考 #link(<sec1>)[这个实验小节]。    // 手动链接到锚点
```

`@` 与 `#link(<标签>)` 指向同一锚点，区别仅在显示文本：

- `@intro`：自动填充编号 + 类型前缀；
- `@intro[自定义文字]`：自定义文字 + 编号；
- `#link(<sec1>)[任意文字]`：完全手动。

**自定义引用样式**（`#show ref` 规则实测可用）：

```typst
#show ref: it => {
  let el = it.element
  if el != none and el.func() == heading {
    return link(it.target)[§#el.numbering]
  }
  it   // 其余引用（公式/图）保持默认
}
```

**站外 / 站内路径链接**用 `#link("url")[文字]`：

```typst
#link("/posts/quantum/")[这篇文章]   // ✅ 站内相对路径
#link("https://example.com")[外链]   // ✅ 外部链接
[这篇文章](/posts/quantum/)          // ❌ Markdown 风格相对路径会输出为纯文本
```

**⚠️ 与 Typst 桌面版（PDF 语境）的差异**——HTML 导出没有"页"的概念，因此：

- `#ref(<intro>, form: "page")` **不支持**：编译报错 `cannot reference without page numbering`；
- `#link(<锚点>)` 的锚点必须是**当前文章内**的标签；跨文章跳转用站内路径链接（见上例）。


### 物理宏

可在 `assets/macros.typ` 定义常用符号（可选）。该文件存在时构建会**自动注入** import，无需在文章里手写。

### Typst 包与插图（CeTZ）

Typall 内嵌的 Typst 编译器支持从官方源下载第三方包（`@preview`），首次使用自动下载并缓存到
`.typall/packages/`，之后离线可用。文章里直接写 `#import "@preview/<name>:<version>"` 即可。

最常用的绘图库是 **CeTZ**（受 TikZ 启发）。由于 Typall 输出 HTML，而 Typst 的原生矢量图形
（`line`/`curve`/`circle` 等）不会被 HTML 导出渲染，需要用项目自带的 `fig` 包装器把插图包一层：
站点构建时 CeTZ 内容以**内联 SVG** 的形式嵌入页面，VS Code 预览与 PDF 导出则由 Typst 原生矢量渲染——
一份源码三种环境通用：

```typst
#import "@preview/cetz:0.5.2"
#import "../assets/preview.typ": fig, eq-numbering

#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

#fig(cetz.canvas(length: 4cm, {
  import cetz.draw: *
  line((0, 0), (3, 2), mark: (end: ">"))
  circle((0, 0), radius: 0.06, fill: black)
  content((3, 2), $ P $, anchor: "west")
}))
```

> 上例中的两条 `show` 规则让笔记**不依赖生成器也能独立预览**（VS Code / 裸 `typst compile`）：
> 块级公式获得 `(1)` 编号使 `@eq:` 引用可用，行内公式与画布内标签不参与编号。
> `preview.typ` 在 HTML 导出时经 `html.frame` 渲染（该分支运行时才求值），分页环境下
> 无需 `--features html` 即可编译——这也是 VS Code 里 Tinymist 预览能直接工作的原因。

CeTZ 自带 WebAssembly 内核（贝塞尔曲线求极值、布尔运算、树布局等）也能正常加载，**无需额外配置**；
其依赖的其他包（如 `oxifmt`）会一并自动下载。

更丰富的示例见项目内的 `posts/cetz-demo.typ`（坐标系、单位圆三角函数、树布局），以及
[CeTZ 官方手册](https://cetz-package.github.io/docs) 和 [示例画廊](https://janosh.github.io/diagrams)。

### RSS / Sitemap

构建时自动生成：

- `public/atom.xml`：Atom 订阅源，**包含文章全文**（`<content type="html">`），带 `<link rel="self">` 自引用；文章数不限，按日期倒序。
- `public/sitemap.xml`：站点地图，含 `<lastmod>`（文章/页面日期）。

需要在配置里填 `[site] url`，否则这两个文件不会生成。

### 站内搜索

构建时自动生成 `public/search.json`（全站文章纯文本索引）与 `/search/` 页面。搜索页使用**零依赖内联 JS**（不引入外部库），在标题 / 标签 / 摘要 / 正文全文中做即时检索。默认主题导航栏含"搜索"入口。

### SEO

配置 `[site] url` 后，构建会自动为每个页面生成：

- `<link rel="canonical">` + OG / Twitter Card 标签（文章页 `og:type="article"`，其余 `website`）；
- 文章页 `og:description` / 页面描述取自摘要（无摘要时回退站点描述）；
- **文章页 `og:image` 自动生成**：优先级为 front-matter `#let cover = "…"` → 正文首图 →
  站点令牌配色自动渲染的 1200×630 分享卡（标题 + 站点名 + 日期，写入 `og/<slug>.png`），
  Twitter Card 自动升级 `summary_large_image`——分享到微信/Twitter 时有像样的缩略图。
  强调色可用 `[theme.params] accent_color = "#…"` 定制；
- **JSON-LD 结构化数据**：文章页输出 BlogPosting schema（headline/author/datePublished/…），供搜索引擎富摘要；
- `public/robots.txt`（含 `Sitemap:` 指引）与 `public/404.html`（供 GitHub Pages / Netlify 等静态托管使用）。

未配置 `site.url` 时 SEO 标签整块省略（避免残缺标签），robots.txt 与 404.html 仍会生成。

### 预压缩（gzip + brotli）

构建会为输出目录中的文本文件（html / css / js / json / xml / txt / svg）自动生成 `.gz` 与 `.br` 副本（内容无收益的文件跳过对应格式），供部署层直接服务；`serve` 会按 `Accept-Encoding` 优先返回 `.br`、其次 `.gz`（注入了 Live Reload 脚本的 HTML 无法复用磁盘预压缩产物，实时 gzip）：

```nginx
# nginx 开启 gzip_static 后优先返回 .gz，无 .gz 时回退实时压缩
gzip_static on;
```

增量构建时产物做了**内容感知**处理：源文件内容未变则不重写（保持 mtime），`.gz` / `.br` 不旧于源即跳过重压，避免每次构建重复压缩全部产物。

### PDF 导出

`typall pdf` 把文章编译为 PDF（Typst 原生 paged 渲染，字体内嵌）：

```bash
typall pdf                                  # 全部已发布文章 → pdf/
typall pdf --slug posts/math-trig           # 只导出一篇（slug 可写 posts/math-trig 或 math-trig）
typall pdf --output out/pdf                 # 自定义输出目录
typall pdf --drafts                         # 包含草稿（与 `--to build --drafts` 语义一致）
```

- **公式编号与交叉引用**：PDF 走原生 paged 渲染，公式编号 `(1)` 与 `@eq:xxx` 引用（"式 1"）由引擎正确呈现。站点 HTML 因 typst#5512 需用 `numbering: (..) => none` 抑制编号、改由后处理注入 eq-num；PDF **不能**沿用该抑制（否则 `@ref` 引用会报 `cannot reference equation without numbering`），故使用独立 preamble；
- **字体**：除内置字体集 + `assets/fonts/` 外，额外加载系统字体目录兜底（PDF 内嵌字体、天然机器相关，不影响站点构建的确定性字体集）；
- **输出结构**：`pdf/posts/<slug>.pdf` 与 `pdf/pages/<slug>.pdf`（`--output` 自定义根目录时同理）。

## 自定义主题

除了内置默认主题（「墨理」杂志编辑风），你可以直接启用仓库自带的四套**参考主题**（`themes/` 下，设计令牌与验收见 [docs/THEME-GALLERY.md](../docs/THEME-GALLERY.md)）：

```toml
[theme]
name = "obsidian"   # 曜石：暗色极客风，等宽标题 + 霓虹青 + 代码高亮重配
# name = "paper"    # 纸砚：学术纸感，衬线正文 + booktabs 三线表 + 朱砂点睛
# name = "minimal"  # 留白：极简黑白，零圆角 + post_list partial 接管的双栏网格列表
# name = "citrus"   # 柑橘：暖色文艺，米纸底 + 橘果强调 + 楷体标题圆角卡片
```

你也可以通过 `themes/<name>/` 目录定义自己的主题，包含四类文件（均可选，缺失用内置默认）：

```
themes/my-theme/
├── style.css       # 普通 CSS，定义全局样式（缺失用内置默认）
├── template.html   # HTML 骨架模板（缺失用内置默认）
├── partials/       # 结构片段模板（列表项/分页/文章元信息/专栏导航等 9 个，缺失用内置默认）
└── static/         # 静态资源，复制到输出根目录
```

在 `typall.toml` 里启用：

```toml
[theme]
name = "my-theme"

[theme.params]
accent_color = "#ff5500"
```

**模板占位符**：`template.html` 与 `style.css` 中可用以下占位符：

| 占位符 | 含义 |
|--------|------|
| `{{site_title}}` | 站点标题 |
| `{{site_description}}` | 站点描述 |
| `{{language}}` | 语言（如 `zh-CN`） |
| `{{title}}` | 当前页标题（首页为空） |
| `{{full_title}}` | `标题 · 站点标题` 组合 |
| `{{body}}` | 页面正文 HTML |
| `{{math_style}}` | Typst 生成的 MathML 样式块 |
| `{{nav_extra}}` | 额外导航链接 |
| `{{seo_head}}` | SEO head 块（canonical + OG/Twitter，站点 URL 未配置时为空） |
| `{{analytics}}` | 统计/分析 HTML（`[site] analytics` 配置，原样注入；未配置为空） |
| `{{params.xxx}}` | 主题参数（支持嵌套，如 `{{params.font.family}}`） |

示例 `template.html`（自定义 footer）：

```html
<!DOCTYPE html>
<html lang="{{language}}">
<head>
<meta charset="utf-8">
<title>{{full_title}}</title>
<link rel="stylesheet" href="/assets/style.css">
{{math_style}}
</head>
<body>
<main>{{body}}</main>
<footer style="background: {{params.accent_color}};">{{site_title}}</footer>
</body>
</html>
```

> 注意：`{{body}}`、`{{math_style}}`、`{{nav_extra}}`、`{{seo_head}}`、`{{analytics}}`、`{{params.*}}` 原样注入（不转义），其余文本类占位符会做 HTML 转义。

> 上表是历史扁平写法，目前与命名空间形式（`{{site.title}}` / `{{page.*}}` / `{{post.*}}`）**双轨并存**。完整的占位符全集、`posts` 集合、9 个可覆盖 partial 与 `{% for %}` / `{% if %}` 模板能力，见 **[主题规格 docs/THEME-SPEC.md](../docs/THEME-SPEC.md)**（主题作者只依赖该契约）。

### 主题打包与分发

自定义主题可打包成 zip 分享或通过 URL 安装：

```bash
# 打包 themes/my-theme/ → themes-my-theme.zip（zip 内顶层目录为主题名）
typall theme package --name my-theme

# 本地安装（zip 顶层目录会被自动剥离，文件直接落入 themes/<name>/）
typall theme install themes-my-theme.zip

# 远程安装（URL 末段作主题名）
typall theme install https://example.com/themes-my-theme.zip

# 安装后在 typall.toml 启用
[theme]
name = "my-theme"
```

安装时会校验路径穿越（拒绝 `..`）与主题完整性（至少包含 `style.css` 或 `template.html` 之一），非法包直接拒绝并清理。

### 图片懒加载

正文中的图片自动获得优化（无需手动处理）：

- **首图保留即时加载**（利于 LCP 首屏指标），后续图片自动加 `loading="lazy"`；
- 图片缺失 `alt` 时，自动用 `src` 文件名兜底（如 `images/logo.png` → `logo`）；data URI 内联图回退为 `image`。

在 Typst 中引用图片：

```typst
#image("../assets/images/cover.png")
```

### 评论系统（giscus / utterances）

内置默认主题不挂评论，但仓库自带 **示例主题 `themes/comments-demo/`**——基于 [giscus](https://giscus.app/zh-CN)（GitHub Discussions 驱动）演示如何在文章页底部挂载评论系统。**纯前端注入**，构建期无依赖、零后端。

启用步骤：

```toml
# typall.toml
[theme]
name = "comments-demo"

[theme.params]
giscus_repo        = "owner/repo"      # 必填：你的 GitHub 仓库
giscus_repo_id     = "R_kgDOxxxxx"     # 必填：giscus.app 配置页面获取
giscus_category    = "General"         # 可选，默认 General
giscus_category_id = "DIC_kwDOxxxxx"   # 可选
giscus_mapping     = "pathname"        # 可选，默认 pathname
giscus_lang        = "zh-CN"           # 可选，默认 en
```

> 拿到 `repo_id` / `category_id`：访问 https://giscus.app/zh-CN 填仓库、生成配置，把页面给出的 `data-repo-id` / `data-category-id` 复制过来。

切换为更简单的 [utterances](https://utteranc.es)（基于 GitHub Issues，配置更少）——编辑 `themes/comments-demo/template.html`，把 `<script src="https://giscus.app/client.js" ...>` 整段替换为：

```html
<script src="https://utteranc.es/client.js"
        repo="owner/repo"
        issue-term="pathname"
        label="comment"
        theme="github-light"
        crossorigin="anonymous" async></script>
```

> 💡 **仅文章页显示**：模板引擎支持 `{% if %}` 条件（tera）。评论区块默认出现在所有页面，如需只在文章页挂载，在主题 `template.html` 里用 `{% if page.og_type == "article" %} … {% endif %}` 包起来即可（语法见 [docs/THEME-SPEC.md](../docs/THEME-SPEC.md)）。

## 目录结构

```
my-blog/
├── typall.toml
├── posts/          # 文章（.typ）
├── pages/          # 独立页面（.typ）
├── assets/         # 静态资源（图片、字体、宏）
├── themes/         # 自定义主题
├── .typall/        # 编译缓存与构建清单（可丢弃，`typall clean` 清除）
└── public/         # 构建输出（可丢弃）
```

## 已知限制

Typall 基于 Typst 的 **HTML 导出**，该功能目前仍是实验性（Typst 官方明确「不用于生产」）。已知影响：

- **MathML 渲染器在 Chromium 系（含 Edge）有 `<mfrac>` 显示错乱 bug**：块级公式分子分母会被压成竖列（一行只显示一列字符）。
  CSS 兜底（min-height、line-height 等）无效——根因是浏览器原生 MathML 块级 mfrac 的高度计算忽略 CSS。
  **解决：** 在 `typall.toml` 设 `[build.math].renderer = "svg"`，走 `typst_svg::svg_in_html` 输出内联矢量 SVG，无此 bug，
  且额外正确渲染公式编号 `(1)`。**SVG 是当前推荐默认。**
- **公式编号文本**：在 `mathml` 模式下不渲染（typst-html 上游限制）；`svg` 模式无此问题。
- **不输出主题 CSS**：Typst 只输出语义化 HTML + MathML 样式，主题外观由 Typall 的 `style.css` 负责。

因此 Typall 锁定 `typst = "0.15.1"` 版本，升级 Typst 时需回归测试。

## 性能

实测（release 版，99 篇测试文章、每篇 5 个公式 + 代码块）：

| 场景 | 耗时 |
|------|------|
| 首次构建（全量编译 99 篇） | 0.3 秒 |
| 增量构建（99 篇全命中缓存） | 0.06 秒 |
| 改一篇（其余 98 篇命中） | 仅重编 1 篇 |
| 空项目 | 0.02 秒（不含进程启动） |
| 单篇 HTML 体积 | 5.9 KB |

> 以上为 `--release` 实测（99 篇压测项目，12 核 Windows）。增量编译通过「源文件哈希 + 编译上下文（preamble）哈希 + import 依赖哈希」三级判定，只重编变化的文件；
> 缓存存放在 `.typall/cache/`，`typall clean` 可清除。文章页/独立页渲染用 rayon 并行化（写盘串行）；
> 写盘与 `.gz` 预压缩均做内容感知增量（内容未变不重写、不重压），二次构建开销接近纯 IO。

## 实时编译路线图（Live Compile）

`serve` 的实时编译闭环已升级为 **SSE 推送**（早期 800ms 轮询 `/__version` 方案已移除）。

### 现状

- **监听**：notify 递归监听 `posts/ pages/ assets/ themes/`、`typall.toml` 与
  `.typall/packages/`（serve 运行中新增 `#import "@preview/…"` 首次拉包后自动重建）
  （`src/serve.rs`）。事件做 300ms 尾沿去抖（等编辑器连续保存稳定后才重建，
  避免「改一次永不触发」），并做**事件类型感知过滤**：编辑器临时文件
  （`.#*` 锁、`#*#` 备份、`*~`、`.swp/.swx` swap）、读访问（`Access` 事件，
  索引/杀毒扫描）、rename「旧名」事件（编辑器原子写的前半段）均不触发重建——
  Windows 原子写实测恰好触发一次重建。
- **重建**：去抖后调用 `build::build()`（每次重建前重读 `typall.toml`，
  运行中切换主题等配置即时生效），内部走三级哈希增量缓存——单篇改动只重编该篇，
  其余命中缓存，重建耗时接近首次构建的零头。
- **推送**：`/__events` SSE（tokio watch 通道）把构建状态 `{version, error}` 推给浏览器：
  版本变化 → **软刷新**——fetch 当前页新 HTML，仅替换 `main#content` 与 `<title>`，
  **滚动位置保留**；顶栏结构变化（主题/模板改了）或 fetch 拿不到页面
  （写盘窗口，先重试一次）→ 回退整页刷新；构建失败 → 页面底部**错误浮层**
  （追加不覆盖、可关闭，monospace 展示完整错误）。
  watch 在订阅时立即产出当前值——构建失败后新打开的页面也能看到浮层；
  **初始构建失败不再中止 serve**，服务器照常启动，修复后任一文件保存自动恢复。
- 脚本仅 serve 时注入响应，不写盘，`build` 产物保持零 JS。

已知限制：初始构建失败时几乎无产物，直接访问的 URL 会 404（无注入脚本），浮层只对
已存在的旧产物页面生效——此时以终端输出为准。

### 后续演进（未做）

**「更细粒度增量」暂不做**：构建期增量缓存已覆盖编译开销的大头（单篇改动仅重编 1 篇），
继续下钻（world 层复用 typst `Source`、跨次 serve 常驻编译状态）工程量大而边际收益小。
主题/模板改动触发全站重渲染是合理语义，保留。

## 技术栈

Rust · Typst 0.15（内嵌编译）· rayon（并行）· axum（dev server）· notify（文件监听）· scraper（HTML 解析）· clap（CLI）

## 开发

```bash
cargo build --release    # 构建
cargo test               # 测试
cargo run -- build       # 调试运行
```

---

*Typall 处于早期开发阶段，接口可能变动。*
