# Typall 完整测试校验报告

**日期**：2026-09-06 · **平台**：Windows 10 x64 · **版本**：typall 0.1.0（内嵌 Typst 0.15.1）· **代码规模**：22 个 Rust 模块 / 11,012 行

本轮为架构治理（模块拆分、e2e 安全网、注册表化）与 VS Code 预览修复后的**全链路回归校验**，
覆盖 8 个验证维度，过程中发现并修复 2 个缺陷（详见 §3）。

---

## 1. 测试矩阵总览

| # | 维度 | 内容 | 结果 |
|---|------|------|------|
| 1 | 单元测试 | 缓存/转义/部署/搜索/兼容层等 170 个用例 | ✅ 170 通过 / 0 失败 |
| 2 | e2e 黄金基线 | 真实 CLI 构建夹具站点，55 文件内容哈希比对 + 幂等 + 草稿过滤 + 错误目录守卫 | ✅ 2 用例通过 |
| 3 | 严格构建 | `build --strict`：42 篇文章 + 1 独立页面，死链检查 | ✅ 通过（44 缓存命中 / 0 重编译） |
| 4 | VS Code 场景 | 全部 43 个 `.typ` 裸编译（无 `--features html`，等价 Tinymist 预览） | ✅ 43 / 43 通过 |
| 5 | PDF 导出 | `typall pdf` 全量（原生分页渲染） | ✅ 43 个 PDF（120s） |
| 6 | 发布 · Markdown | `publish --to markdown` 全量 | ✅ 27 新建 + 16 更新 |
| 7 | 发布 · 公众号 | `publish --to wechat` 全量（resvg 栅格化 + 内联样式） | ✅ 26 新建 + 15 更新 + 2 未变 |
| 8 | 部署演练 | `deploy --dry-run`（git → gmsjy.github.io / main） | ✅ 计划输出正确 |
| 9 | 运行时实测 | serve 起站后首页/文章/搜索/标签/归档 6 页 HTTP 状态 + 浏览器交互 | ✅ 全部 200 |

## 2. 运行时验证截图

**首页**（文章列表 + 标签药丸）：

![首页](../_shots/verify-home.png)

**数学·插图页**（空间向量：CeTZ 坐标系插图 + 内联 SVG 公式 + 目录）：

![插图页](../_shots/verify-math-figure.png)

**搜索交互**（关键词「圆周运动」全文检索，命中物理/数学多篇）：

![搜索](../_shots/verify-search.png)

### 主题 × 页面矩阵（20 张，上一轮验收存档）

| 主题 | 首页 | 数学 | 物理 | 搜索 |
|------|------|------|------|------|
| default 墨理 | [home](../_shots/theme-test-default-home.png) | [math](../_shots/theme-test-default-math.png) | [physics](../_shots/theme-test-default-physics.png) | [search](../_shots/theme-test-default-search.png) |
| citrus 柑橘 | [home](../_shots/theme-test-citrus-home.png) | [math](../_shots/theme-test-citrus-math.png) | [physics](../_shots/theme-test-citrus-physics.png) | [search](../_shots/theme-test-citrus-search.png) |
| minimal 留白 | [home](../_shots/theme-test-minimal-home.png) | [math](../_shots/theme-test-minimal-math.png) | [physics](../_shots/theme-test-minimal-physics.png) | [search](../_shots/theme-test-minimal-search.png) |
| obsidian 曜石 | [home](../_shots/theme-test-obsidian-home.png) | [math](../_shots/theme-test-obsidian-math.png) | [physics](../_shots/theme-test-obsidian-physics.png) | [search](../_shots/theme-test-obsidian-search.png) |
| paper 纸砚 | [home](../_shots/theme-test-paper-home.png) | [math](../_shots/theme-test-paper-math.png) | [physics](../_shots/theme-test-paper-physics.png) | [search](../_shots/theme-test-paper-search.png) |

架构总览：[docs/CODE-MAP.svg](CODE-MAP.svg) · 兼容假设清单：[docs/COMPAT.md](COMPAT.md)

## 3. 本轮发现并修复的缺陷

### 3.1 Markdown 全量发布 CJK 字节切片 panic（严重 · 已修复）

`publish --to markdown` 全量发布时在 `find_matching_close` 崩溃：
`start byte index 40394 is not a char boundary; it is inside '公'`。
根因是手写标签扫描器**逐字节步进后直接切片**，落在多字节字符中间必然 panic。

修复：非字符边界位置跳过继续扫描（标签序列为 ASCII，语义不变）；
新增回归测试 `find_matching_close_walks_multibyte_safely`（含嵌套配对断言）。
修复后全量发布成功：27 新建 + 16 更新。

### 3.2 分享卡重复栅格化拖慢增量构建（性能 · 已修复）

每篇无图文章**每次构建**都重跑 resvg 栅格化生成 og:image（42 卡 × 字体加载 ≈ 30s），
即使产物完全不变。修复：输入指纹（标题/站点名/日期/主题色）哈希比对，未变化直接复用
既有 PNG 并向写盘器登记（避免孤儿清理误删）。

| 指标 | 修复前 | 修复后 |
|------|--------|--------|
| 增量构建「渲染+写盘」 | 28–30 s | **0.53 s** |
| 增量构建总耗时 | ~32 s | **~2.5 s** |

> 该缺陷的第一版修复曾引入新回归（跳过栅格化后卡片被孤儿清理误删），
> 由本轮新增的 **e2e 幂等断言当场抓获**——安全网机制首次实战即兑现价值。

## 4. 已知限制（如实记录，非本轮回归）

- 公众号「复制到公众号」按钮样式硬编码墨理主题色值（画廊文档已记录的功能耦合）；
- MathML 渲染器在 Chromium 系浏览器存在 `<mfrac>` 已知缺陷（默认渲染器为 SVG，不受影响）；
- 物理单位在数学环境中为斜体（如 `300 J`）——既定内容风格决策，保持现状；
- `deploy` 的 git 推送依赖本机凭证；远程仓库 `gmsjy.github.io` 尚未创建，实际推送待用户建仓后执行。

## 5. 结论

8 个维度全部通过。架构治理（模块拆分 + e2e 安全网 + 注册表化 + 兼容层收拢）完成后，
全链路——**笔记 → 站点 / VS Code 预览 / PDF / Markdown / 公众号 / 部署演练**——均可复现，
且增量构建性能提升一个数量级。发现并修复 2 个缺陷（1 严重崩溃 + 1 性能），均有回归测试覆盖。
