# 「复制到公众号 / 知乎」方案设计文档

> 状态：**阶段 1 已完成并提交**（2026-09-03，commit `fd4e18c`）；知乎按老板指示**延后**
> 决策：老板拍板三点 —— ① 公式用 SVG（use/symbol 展开保留矢量） ② 展开放**浏览器端 JS** ③ 保留 `publish --to markdown` 纯文件导出
> 前置结论：**放弃「平台直接发布」（`publish --to wechat --push`），改为预览模式下的「复制按钮」+ 手动粘贴**
>
> **执行进度**（详见 §6 分阶段）：
> - ✅ **阶段 1**：`serve` 注入「复制到公众号」浮动按钮 → JS use/symbol 展开 → 双格式剪贴板。已实现、resvg 端到端验证非空白、已提交 `fd4e18c`。
> - ⏸️ **阶段 2（知乎按钮）**：**老板拍板缓一缓**。卡点：渲染 DOM 只含 SVG、不含 Typst 源码，浏览器 JS 无法还原 LaTeX（知乎 `$...$` 需 LaTeX 非 Typst 语法）。候选路径待定（转 SVG 图片 / 构建期注入源码转 LaTeX / HTML 富文本）。
> - ✅ **阶段 3**（移除 `publish --to wechat --push` 触网链路）：**已完成**（2026-09-04）。`src/wechat_api.rs` 整体删除（草稿 API、token/媒体缓存、素材转传、`--push`、`typall.wechat.example.toml`）；`publish --to wechat` 保留为**纯本地富文本导出**（公式/插图栅格化 PNG data URI）。历史缓存 `.typall/wechat_token.json` / `wechat_media.json` 成为死文件，`typall clean` 会顺手删除。

---

## 0. 结论摘要（TL;DR）

| 议题 | 结论 |
|---|---|
| 方向可行性 | ✅ 可行且更务实。微信/知乎官方 API 通道脆弱，复制粘贴是保真度最高的入口 |
| 公式处理 | ✅ **SVG + use/symbol 展开**：把 `<defs>`+`<symbol id>`+`<use xlink:href>` 展开成纯 `<path>`，删光 id/defs。**展开放浏览器端 JS**（老板定），复制动作触发时实时处理 |
| 按钮形态 | 浮动侧边栏按钮（复制到公众号 / 复制到知乎） |
| 技术边界 | 预览模式（`serve`）注入按钮，发布模式（`build`）保持零 JS |
| 知乎 | 当前代码零实现；复制纯文本 Markdown（复用 html_to_markdown） |
| `publish --to markdown` | **保留**（纯文件导出、不触网），仅砍 `--to wechat --push` API 触网发布 |

---

## 1. 背景与动机

### 1.1 上一轮撞的墙（真实教训）

上一轮尝试「微信 API 直接发布」时遇到两个硬约束，导致 ROI 极低：

1. **`draft/add` content 硬上限 2 万字符**：内联 SVG 一条公式约 5KB，一篇 74 公式的文章高达 40 万字符，是上限的 20 倍，根本无法推送。
2. **API 静默剥离 SVG 属性**：`<use>` 的 `xlink:href`、`<symbol>` 的 `id` 被剥离，typst 公式字形靠 `use→symbol` 引用机制渲染，属性被剥后公式渲染成空白。

### 1.2 为何「复制粘贴」是更优路径

- 微信/知乎编辑器的**富文本粘贴**是最稳定、保真度最高的内容入口（不像 API 有字符上限和属性清洗）。
- 业界成熟做法（Mdnice、Markdown Here、doocs/md、135 编辑器）都是「本地排版 → 复制 → 粘贴」。
- 知乎当前代码里**完全没有实现**（仅注释提及「阶段 3」），放弃 API 发布零成本。

---

## 2. 关键结论：公式「作为 SVG」可行，但必须做 use/symbol 展开

### 2.1 证据链

微信公众号对 SVG 的处理存在**三层限制**（公开资料 + 上一轮 API 实测交叉印证）：

| 层 | 行为 | 对 typst 公式的影响 |
|---|---|---|
| 编辑器层 | 过滤 `<script>`、JS、外部资源 | 无直接破坏 |
| **保存层** | **删除所有 `id`、删除 `<defs>` 组** | **致命**：`<use xlink:href="#id">` + `<symbol id="id">` 引用断裂 |
| 客户端渲染层 | `<img>` 内 SVG 失去 CSS 继承、`currentColor` 断裂 | 颜色/样式丢失 |

**多条独立来源的踩坑实录一致确认**：公众号编辑器「不能有 `id`」「不能有 `<defs>`」：

- 博客园 tadshi：*「微信的编辑器似乎不允许定义 id 属性」「有的公式生成的 defs 元素，公众号平台很神奇地会把组元素以及之外的所有组全部删掉，导致公式直接变白纸」* → 解法就是**手动把 use 展开、删 defs**。
- CSDN qq_37588752：*「标签里不能有 id，不能有 defs 标签」*。

而 typst 公式 SVG 的渲染机制恰恰是：

```html
<defs>
  <symbol id="gA0FB579...">
    <path d="M0 0 ..."/>
  </symbol>
</defs>
...
<use xlink:href="#gA0FB579..." fill="#000000" x="0" y="0"/>
```

保存层删除 `id`/`defs` 后，`use` 找不到 `symbol`，公式字形全部消失 → **空白**。

### 2.2 应对方案：use/symbol 展开（保留矢量，复用已有代码）

**结论：公式可以保留 SVG 矢量形态，只需在复制前做「use/symbol 展开」预处理**——把 `<defs>`+`<symbol id>`+`<use xlink:href>` 结构内联展开成纯 `<path>`，删除所有 `id`/`defs`，`use` 上的 `x`/`y` 平移转成外层 `<g transform="translate(...)">`，`fill` 等展示属性继承到 `path`。

这正是上一轮在 `src/wechat.rs` 里**已经实现并通过单测**的 `expand_svg_use_refs` 函数。当时废弃它是因为「API 推送有 2 万字符上限、展开后体积爆炸」，但**复制粘贴没有字符上限**，这个缺点在复制场景里不存在。

| 方案 | 做法 | 公式效果 | 代价 |
|---|---|---|---|
| ❌ 直接 SVG 原样复制 | 无处理 | 空白（id/defs 被删） | 无 |
| ✅ **use/symbol 展开** | 展开成纯 path，删 id/defs | **矢量保留、正常显示** | 少量字符串处理（代码现成） |
| ⚠️ 栅格化 PNG | 转图片 | 正常但变位图 | 丧失矢量、需图片上传 |

> **决策：采用 use/symbol 展开，且展开放浏览器端 JS**（老板定，2026-09-03）。理由：展开依赖当前页面的 `<svg>` 结构，浏览器端复制时实时处理最贴合「复制动作触发」语义；Rust 端 `expand_svg_use_refs` 曾因 API 字符上限废弃且已被栅格化替换删除，无需恢复为 Rust 实现，改为在注入的 JS 里实现等价的展开逻辑。栅格化（`svg_to_png`）仅作为「展开后仍有残缺字形」时的兜底（不启用，保留代码引用）。

---

## 3. 架构设计

### 3.1 两态边界（干净复用现状）

| 模式 | 命令 | 行为 | 按钮/脚本 |
|---|---|---|---|
| 预览 | `typall serve` | 构建 + Live Reload（脚本注入内存，不落盘） | ✅ 注入复制按钮 + 剪贴板脚本 |
| 发布 | `typall build` + `deploy` | 生成静态产物，零 JS | ❌ 无按钮 |

现有 `serve.rs` 的 `inject_reload` 已经在「HTML 响应时注入脚本」——**同一个注入点**追加复制按钮脚本即可，天然满足「预览有、发布无」。

### 3.2 复制内容格式（双格式剪贴板）

浏览器 `navigator.clipboard.write()` 支持 `ClipboardItem` 同时携带多种 MIME。复制按钮点按即触发，浏览器端 JS **实时**抓取 `<article>` DOM 并处理：

```
复制到公众号：
  text/html  → 富文本 HTML（公式/插图 SVG 实时 use/symbol 展开成纯 path，图片转绝对 URL）
  text/plain → 纯文本（公众号编辑器忽略，作为降级）

复制到知乎：
  text/plain → Markdown（复用 html_to_markdown）
  text/html  → 可选，知乎富文本支持有限
```

> **use/symbol 展开发生在浏览器端复制动作触发时**（老板定），用一个注入的纯 JS 函数遍历 `<article>` 内所有 `<svg>`：
> 1. 收集 `<defs><symbol id="X">INNER</symbol></defs>` → 删除整个 `<defs>`
> 2. 把每个 `<use xlink:href="#X">` 替换为对应 symbol 的 INNER 内容（`<path>`），并把 `use` 上的 `fill` 等展示属性继承到 path、`x`/`y` 平移转成外层 `<g transform="translate(...)">`
> 3. 展开后页面内不再有 `id`/`defs`/`xlink` 依赖，微信保存层无 `id` 可删 → 公式矢量正常显示
> 4. 用 cloneNode 操作克隆 DOM 副本处理，**不污染页面原 DOM**

### 3.3 按钮形态：浮动侧边栏

- 文章页右侧浮动竖排按钮组：`复制到公众号` / `复制到知乎`
- 跟随滚动，不遮挡正文
- 点击后：写入剪贴板 → 按钮短暂显示「✅ 已复制」
- 移动端：改为顶部固定按钮组（小屏无侧边空间）

### 3.4 图片/公式的引用处理

| 元素 | 站点产物形态 | 复制时转换 |
|---|---|---|
| 公式（block/inline） | 内联 `<svg>`（含 defs/symbol/use） | **use/symbol 展开成纯 path**，删 id/defs（复用 `expand_svg_use_refs`） |
| cetz 插图 | 内联 `<svg>`（含 defs/symbol/use） | 同上 |
| 普通图片 | `<img src="assets/x.png">`（相对路径） | 转绝对 URL `<img src="{site.url}/assets/x.png">` |

> 公式/插图 SVG **不转图片、不依赖 site.url**（use 展开后自包含）。仅普通图片需要绝对 URL，依赖 `site.url` 配置正确。

---

## 4. 改动清单（预估，含浏览器 JS 决策）

| # | 模块 | 改动 | 说明 |
|---|---|---|---|
| 1 | 前端脚本（serve 注入） | **use/symbol 展开 + 图片绝对 URL 替换** | 纯 JS 遍历 `<article>` 内 `<svg>` 展开成纯 path，删 id/defs/xlink |
| 2 | `src/serve.rs` | `inject_reload` 旁新增按钮+脚本注入 | 仅 serve 时注入浮动「复制到公众号」按钮与剪贴板 JS；`build` 产物零 JS |
| 3 | 前端脚本 | 剪贴板双格式写入（text/html + text/plain） | 复制动作触发时实时抓取 DOM 处理，克隆副本不污染原 DOM |
| 4 | 知乎 | 复制 text/plain Markdown（复用 html_to_markdown） | 阶段 2 |
| 5 | `src/main.rs` | 移除 `publish --to wechat --push` 触网链路；**保留 `publish --to markdown`** | 纯文件导出、不触网 |
| 6 | `README.md` / 文档 | 更新使用说明 | 说明复制流程、`site.url` 前提 |

---

## 5. 风险与待验证项

| # | 风险 | 等级 | 应对 |
|---|---|---|---|
| 1 | 公众号保存层删 ID/defs（公式空白） | 已确认 | **use/symbol 展开**（浏览器端 JS，删 id/defs，纯 path 内联），绕开引用机制 |
| 2 | JS 端展开逻辑正确性 | 中 | 用真实 typst 公式 SVG 结构单测验证展开结果（对照站点产物） |
| 3 | 剪贴板 API 需 HTTPS / 用户授权 | 低 | `localhost` 例外可用；降级到 `document.execCommand` |
| 4 | 知乎富文本粘贴对 HTML 支持有限 | 中 | 知乎走 text/plain Markdown，避开 HTML |
| 5 | 展开后 SVG 体积增大 | 低 | 复制场景无字符上限，不影响 |

---

## 6. 分阶段落地建议（浏览器 JS 决策下）

- **阶段 1（最小闭环）** ✅ 已完成（commit `fd4e18c`）：`serve` 注入「复制到公众号」浮动按钮 → JS 抓取 `<article>` → use/symbol 展开 → 双格式剪贴板。端到端用 resvg 0.45 栅格化「展开后」的 5 个真实公式 SVG，全部 NON-BLANK（主公式 274×267，56.8% alpha>0），证明展开后纯 path 结构可被标准 SVG 渲染器正常绘制 → 微信删 id/defs 不再影响，且粘贴无字符上限。**待老板实测公众号粘贴效果**。
- **阶段 2** ⏸️ 延后（老板拍板）。「复制到知乎」按钮。**已知卡点**：渲染出的 DOM 只含公式 SVG、**不含 Typst 源码**，浏览器端 JS 无法像 Rust `publish.rs` 那样用 `CompiledDoc.math[].source` 还原 LaTeX；而知乎 `$...$` 要求 LaTeX 语法（Typst ≠ LaTeX）。候选路径（择一，需老板定）：
  - A. 公式 SVG → `data:` 图片内嵌进 Markdown（`![](data:image/svg+xml...)`），不动 Rust、改动最小；
  - B. 改 `build.rs` 把每公式 Typst 源码注入 `data-*` 属性 → 浏览器端 Typst→LaTeX 转换器（复杂、易错、覆盖率低）；
  - C. 知乎也复制 HTML 富文本（与公众号同款 use/symbol 展开），需实测知乎兼容性。
- **阶段 3** ✅ 已完成（2026-09-04）：移除 `publish --to wechat --push` 触网发布链路——`wechat_api.rs` 整删（约 1000 行：草稿 API、图片素材上传、token/媒体缓存、`--push` 参数、发布状态 `pushed_hash`/`remote_id` 字段），`typall.wechat.example.toml` 删除，dev-dep `wiremock` 移除。**保留 `publish --to wechat` 本地富文本导出**（栅格化 PNG data URI，粘贴时编辑器自动转存外链图床，无需 API 上传）与 `publish --to markdown`（纯文件导出）。旧 `publish.json` 含已删字段的记录可正常读取（serde 忽略未知键，下次保存时消失）。
- **阶段 4** ⏳ 未开始：完善（移动端按钮、`site.url` 校验、错误提示、展开兜底栅格化）。

---

## 7. 决策记录（已确认）

1. **公式采用 use/symbol 展开（保留 SVG 矢量）** ✅ —— 比栅格化更优（矢量保真），微信保存层无 id 可删。
2. **展开放浏览器端 JS**（复制动作触发时实时处理） ✅ —— 克隆 DOM 副本，不污染页面；`build` 产物保持零 JS。
3. **保留 `publish --to markdown`**（纯文件导出、不触网）✅ —— API 触网链路（`--push` / 素材转传 / 凭据）已全删（阶段 3 完成）；`publish --to wechat` 退化为本地导出。
