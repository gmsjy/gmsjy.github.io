# 复制排版主题（Copy Theme）实现与验证报告

> 日期：2026-09-17 · 分支：`main` · 环境：Windows 10 x64，`cargo test`（debug）、`typall serve --port 8931` + 内置浏览器实测
> 截图证据：[docs/copy-theme-report/](copy-theme-report/)（8 张，与本文各节对应）

---

## 一、结论摘要

| 事项 | 结论 |
|---|---|
| 复制排版主题（一期，5 套内置） | ✅ 已实现并验证（`typall serve` 预览页浮动栏下拉切换，所见即所得） |
| 「复制到公众号 / 知乎」按主题出稿 | ✅ 两按钮共用主题化管线，出稿内容逐项校验通过 |
| 自定义主题（`copythemes/*.toml`） | ✅ 已实现并验证：创建/修改随重建热加载，坏文件容错 |
| 测试 | ✅ `cargo test` 227 单测 + 2 e2e 全绿，零警告 |
| 截图验证 | ✅ 站点样式 + 5 内置 + 自定义共 8 个状态存档；**过程中发现并修复 2 个暗色主题真问题**（见 §四） |

---

## 二、实现架构

```
copythemes/*.toml ──┐
                    ├─→ copy_theme::all_themes(root) ─→ JSON（`<` 转义防 </script> 逃逸）
内置 5 套（Rust）───┘         │
                              ↓
serve.rs: ServeState.copy_themes (Arc<RwLock<String>>)
   · watch 线程每次重建后刷新（copythemes/ 已入监听清单）
   · HTML 响应注入时读取最新值 → COPY_SCRIPT 的 __COPY_THEMES_JSON__ 占位符
                              ↓
浏览器端 COPY_SCRIPT：
   · 下拉选择（含「站点样式」档）→ applyPreview() 把逐元素内联样式直接套到正文（所见即所得）
   · 选择存 localStorage；SSE 软刷新换入新正文后经 typall:content-refresh 事件自动重套
   · 复制按钮：永远从干净 DOM 副本出稿（strip 站内导航 → use/symbol 展开 → 图片绝对化 → 按主题内联）
```

- 主题数据 = 「槽位 → 内联 CSS 字符串」：`article` / 标签名 / `codeInline` / `codeInPre` / `svg` / `strong` / `eqBlock` / `eqSvg` / `eqNum`；
- 自定义 TOML 两层写法：`[palette]` 色板字段覆盖（回落默认墨理色板）+ `[styles]` 逐槽位覆写（高级）；
- 「站点样式」档位下复制按默认墨理蓝出稿，与历史写死行为一致（回归锚点单测锁定色值）。

## 三、完整测试

### 3.1 单元测试（227 通过 + 2 e2e，零警告）

新增 10 个单测：内置主题形状/顺序/id 唯一、槽位齐全性（12 套主题 × 23 槽位）、
墨理蓝色值与历史版本回归锚点、注入 JSON 合法性、`</script>` 转义、
自定义 TOML 解析合并、坏文件/ID 冲突/非法文件名跳过、目录缺失降级、
注入管线透传自定义主题 JSON。

### 3.2 内置主题截图验证（[copy-theme-report/](copy-theme-report/)）

| 状态 | 断言要点 | 截图 |
|---|---|---|
| 站点样式（基线） | article 无内联样式，下拉含「站点样式 + 5 内置 + 自定义」 | [01-site-default.png](copy-theme-report/01-site-default.png) |
| 墨理蓝（默认） | 正文 `#20222a`、链接 `#0b6fc0` | [02-moli.png](copy-theme-report/02-moli.png) |
| 纸砚衬线 | 衬线字体、h1 居中、朱砂 `#8c1f28` 链接 | [03-paper.png](copy-theme-report/03-paper.png) |
| 柑橘暖阳 | h2 橘色左条、楷体标题、`#e2691e` 链接 | [04-citrus.png](copy-theme-report/04-citrus.png) |
| 极简黑白 | 零圆角、黑色引用条 2px、链接下划线 | [05-minimal.png](copy-theme-report/05-minimal.png) |
| 曜石夜（暗色） | 深底 `#131722` 浅字、加粗/公式/引用全部可读 | [06-obsidian.png](copy-theme-report/06-obsidian.png) |

辅助验证：刷新页面后 localStorage 选择自动恢复并重套主题（纸砚衬线、曜石夜两次实测）；
块级公式在所有主题下均为「公式居中 + 编号右对齐独立行」的公众号兼容布局。

> 注：01–05 摄于 `svg`/`strong` 槽位修复（§四）之前；该修复对浅色主题视觉中性
> （公式与加粗色从「继承站点墨色」变为「显式主题正文色」，二者在浅色下几乎相同），无需重摄。

### 3.3 复制出稿校验（桩替换 `clipboard.write` 捕获实际出稿 HTML）

| 检查项 | 结果 |
|---|---|
| 管线无异常执行（捕获到 426KB HTML） | ✅ |
| 站内导航剥离（toc / meta / prev / next） | ✅ |
| use/symbol 完全展开（无 `<use` / `<defs`；仅余惰性 `xmlns:xlink` 命名空间声明） | ✅ |
| 图片绝对化、块级公式编号右对齐 | ✅ |
| 主题样式在稿（所选主题的字体/色值/h2 结构） | ✅ |
| 自动化环境剪贴板权限拒绝显示「❌ 复制失败」 | 环境现象，非管线问题（桩捕获证明出稿成功） |

## 四、截图验证中发现并修复的问题（暗色主题）

1. **行内公式黑块**：typst 部分公式字形的 `<use>` **不带 fill 属性**，继承站点 CSS
   直接加在 svg 上的 `fill:#000`（样式表规则优先级高于祖先继承，`currentColor` 救不了）。
   **修复**：新增 `svg` 槽位 `color:{fg};fill:{fg};`——内联样式必赢样式表；
   cetz 彩色插图用显式 fill，不受影响。
2. **加粗文字黑块**：站点 CSS 对 `strong` 直接上墨色，主题未覆盖时暗色底上不可见。
   **修复**：新增 `strong` 槽位 `color:{fg};`（极简黑白保留其纯黑覆写）。

两处修复后曜石夜复测：加粗、行内公式、块级公式全部清晰（[06-obsidian.png](copy-theme-report/06-obsidian.png)）。
配套单测：槽位清单加入 `svg` / `strong`，缺失即 fail。

## 五、自定义主题验证（`copythemes/academy.toml`，示例随仓库提供）

### 5.1 创建 → 热加载
serve 运行中新建 `academy.toml` → watch 触发增量重建（6.06s）→ 整页刷新后下拉出现
「学院青」→ 选中即所见即所得：h1 居中墨青、链接青绿 `#0f766e`、引用青条浅青底。
截图：[07-custom-academy.png](copy-theme-report/07-custom-academy.png)

### 5.2 修改 → 热加载
改为紫色调（`accent #7c3aed`、`label 学院青·紫`、`h2_left_bar = true`）→ 增量重建 →
刷新后：下拉标签更新、h2 紫色左条、链接紫色，localStorage 的 `academy` 选择继续生效。
截图：[08-custom-hot-reload.png](copy-theme-report/08-custom-hot-reload.png)

### 5.3 复制出稿
公众号按钮出稿 430KB HTML：紫色强调在稿、h2 左条结构在稿、h1 居中在稿、
站内导航已剥离、无 use/defs、公式编号右对齐——全部通过。

### 5.4 坏文件容错
写入非法 TOML（`[palette` 未闭合）→ 重建成功、页面 200、终端告警
`⚠️ 自定义复制主题 …解析失败，已跳过: TOML parse error at line 2, column 9`、
主题清单不受影响；删除坏文件后清单恢复。ID 冲突（内置 `moli.toml`）与
非法文件名（`bad name!.toml`）的跳过逻辑由单测覆盖。

## 六、遗留事项与建议

- 主题定义修改后需**整页刷新**才进下拉（软刷新只换正文，不换注入的 THEMES 常量）——与站点主题变更语义一致，符合直觉；已写入 README。
- `publish --to wechat`（Rust 端 PNG 栅格化管线）尚未接主题，两套出稿风格体系暂独立；如需统一可作为后续迭代。
- 自定义 `dark = true` 仅为语义标记；公众号暗色出稿的实际观感建议发布前用真实编辑器预览一次。
- 浏览器自动化中剪贴板 API 被拒属环境限制，真实浏览器（HTTPS/localhost + 用户手势）正常。

---

## 七、主题机制 CSS 化（同日追加迭代）

> 需求：**通过不同的 CSS 来实现主题的变化**。落地后"一套主题 = 一份 CSS"。

### 7.1 实现

- 新增**极简 CSS 解析器**（`copy_theme.rs`）：去注释 → 引号感知地切声明（`font-family:"a;b"`、`url(http://…)` 不误切）→ 逗号选择器组拆分 → 选择器映射到正文槽位 → 与默认墨理基底合并（只写想改的元素即得完整主题）。
- 选择器→槽位映射：标签直映射（`h1`–`h4`/`p`/`blockquote`/`pre`/`ul`/`ol`/`li`/`table`/`th`/`td`/`hr`/`a`/`img`/`svg`），特殊槽位 `article`/`code`→行内/`pre code`→块内/`b`/`strong`、`em`/`i`/`del`/`s`/`.eq-num`/`[data-equation="block"]`；**类名/伪类/@规则静默忽略**（粘贴目标不认）。
- **内置四套主题全部改写为 CSS 常量**（`PAPER_CSS`/`CITRUS_CSS`/`MINIMAL_CSS`/`OBSIDIAN_CSS`），兼作可照抄的主题模板；墨理蓝即默认基底。每套配移植锚点单测锁色值，防 CSS 化走样。
- 自定义：`copythemes/<id>.css`（推荐），文件头可选 `/* typall-copy-theme: label="…" dark */` 指令；`.toml` 令牌格式保留兼容。热加载、坏文件容错、`</script>` 转义机制不变。
- JS 应用列表增加 `em`/`del` 槽位。

### 7.2 验证（`cargo test` 232 单测 + 2 e2e 全绿）

| 项 | 结果 | 证据 |
|---|---|---|
| CSS 解析器（注释/引号分号/url 冒号/逗号组/@规则嵌套跳过/未知选择器丢弃） | ✅ | 单测 5 项 |
| 内置 CSS 移植锚点（四套 × 5 关键值） | ✅ | 单测 |
| 自定义 CSS 装载（指令 label/dark、注释内规则不生效、回落基底） | ✅ | 单测 + 浏览器 |
| academy.css（学院青·紫）应用：紫色 h2 左条/居中 h1/淡紫引用/正文墨青 | ✅ | [09-custom-css-theme.png](copy-theme-report/09-custom-css-theme.png) |
| **serve 运行中新建 ocean.css（深海蓝）→ 热加载 → 下拉出现并可用** | ✅ | [10-custom-css-ocean.png](copy-theme-report/10-custom-css-ocean.png) |
| CSS 主题复制出稿（蓝色下边线 h2/加粗墨蓝在稿、剥离导航、无 use/defs、公式编号右对齐） | ✅ | 桩捕获 429KB 逐项断言 |

已知边角：@规则体内嵌套块按大括号深度整体跳过；大括号不配对的残段丢弃。解析器按"主题 CSS 都是扁平元素规则"的约定实现，不支持后代选择器组合（`pre code` 除外，已内建映射）。

— 完 —
