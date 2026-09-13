# 演进实测报告：LaTeX 转换器覆盖率 + 真实博客迁移（import）

> 日期：2026-09-09 · 范围：昨日梳理报告的演进建议 ①（Typst→LaTeX 转换器覆盖率扩展）与
> ②（import 真实迁移实测）· 结论：**两项完成，另修复 2 个生产级 bug；真实语料迁移成功率 67/68（98.5%）**。
> 验证：196 单测 + 2 e2e 全绿，clippy 零警告，浏览器实测截图见 `_shots/2026-09-09-*.png`。

---

## 1. 演进项①：Typst→LaTeX 转换器覆盖率扩展（知乎发布链路）

### 1.1 方法：全量公式探针扫描

以临时探针扫描内置 49 篇文章（2953 条数学公式），对每条跑
`typst_math_to_latex` 并统计转换后残留的 Typst 构造（函数调用形态、裸词、引号文本等）。
探针用完即删，结论沉淀为 15 个回归单测。

### 1.2 探出的两个生产 bug（均已修复）

| bug | 根因 | 影响 | 修复 |
|---|---|---|---|
| **CJK 公式 panic** | `rewrite_bare_symbol` 按字节步进切片，多字节字符中间切片越界 | 任何含中文的公式（如 `$vec(F_"合")$`）执行 `publish --to zhihu` 直接崩溃 | 按字符边界步进（`char_indices` 语义），补 CJK 回归测试 |
| **`=>` 破坏为垃圾** | `\cmd` 无边界字符串替换：`arrow.r` 先于 `arrow.r.double` 匹配，`\arrow.r.double` 被替换成 `\rightarrow.double` | 19 处 `=>` 产物损坏（`\Rightarrow` 变乱码） | 点号长名前移 + 裸词匹配拒绝前导 `\` 与后随 `.` |

### 1.3 新增映射（按扫描频次数据驱动）

- **重音/矢量**（高频）：`vec()`→`\vec`、`arrow()`→`\overrightarrow`、`hat/bar/tilde/dot/dot.double/overline/underline` 各归其位；
- **引号文本**（127 条）：`"…"` → `\text{…}`；引号附标 `_"合"` → `_{\text{合}}`；
- **函数名正体**：裸词 `sin/cos/tan/log/ln/exp/max/min/deg` 等直排为 `\sin` 等（残留在数百次量级）；
- **间距**：`quad` → `\quad`（120 次）；度数符号 `°` → `^{\circ}`；
- **集合/逻辑/无穷**：`oo`、`infinity`→`\infty`，`RR/ZZ/NN/QQ/CC/AA/EE/FF/PP/HH`→`\mathbb{X}`，`subset.eq`→`\subseteq`、`inter/union/emptyset/parallel/forall/exists/bot/top/complement/prop` 等；
- **定界与结构**：`lr()`→`\left(…\right)`、`norm()`、`binom()`、`floor/ceil`、`root(n,x)`→`\sqrt[n]{x}`、`mat(…;…)`（含 `delim:` 参数 → pmatrix/bmatrix/vmatrix/Bmatrix/matrix）、`upright`→`\mathrm`、`cancel`；
- **边界加固**：裸词替换拒绝前导 `\`（保护函数产物 `\vec{…}`）与后随 `.`（保护 `dot` vs `dot.double`），函数名回溯纳入 `.`，未知函数不再整组跳过（组内嵌套已知函数可继续翻译），重写轮数 8→64（长公式几十个调用的统计类文章实测需要）。

### 1.4 覆盖率对比（2953 条公式）

| 指标 | 修复前 | 修复后 |
|---|---|---|
| 残留 Typst 函数调用 | 318+（arrow×119、vec×91、overline×55、hat×22…） | **3**（`calc.round`/`t.at` 等 Typst 代码调用，数学层不可译，原样保留属预期） |
| 引号文本未转换 | 127 | **0**（全部 → `\text{…}`） |
| `=>` 产物损坏 | 19 | **0** |
| CJK 公式 | **进程崩溃** | 正常输出 |

知乎产物抽查（`publish --to zhihu`，`interactions-equilibrium`）：
行内公式输出 `<span class="ztext-math">$ \vec{F_{\text{合}}} = m \vec{a} $</span>`，
块级公式输出 `$$ … $$` 独立段，公式均为干净 LaTeX。

## 2. 演进项②：真实博客迁移实测（import）

### 2.1 语料（真实开源仓库，非构造样本）

| 语料 | 来源 | 规模 | 特征 |
|---|---|---|---|
| **al-folio** | github.com/alshedivat/al-folio `_posts/`（2k+ star 的 jekyll 学术博客主题的演示内容） | 33 篇英文 | YAML front-matter 齐全（内联数组 tags/categories）、专门 math/tables/code 演示文、LaTeX 公式、脚注式引用、HTML 块 |
| **mqyqingfeng/Blog** | github.com/mqyqingfeng/Blog `articles/`（知名中文 JS 博客） | 35 篇中文 | 真实中文正文、大量代码块、引用块、`[object Number]` 类文本、GitHub 远程图片 |

选型过程说明：colah 博客为纯 HTML 文章、godweiyang 仓库只有渲染产物，均不适用；最终双语料兼顾「front-matter+数学」与「中文+代码密度」。

### 2.2 实测发现并修复的缺口

| # | 缺口 | 影响 | 修复 |
|---|---|---|---|
| 1 | **`#quote[block][..]` 不是合法 Typst**（`block` 是命名参数），且引用结束后无换行 | mqy 29/35 篇编译失败——最大单一致命项 | 输出 `#quote(block: true)[…]\n\n` |
| 2 | **`[` `]` 未转义** | `[object Number]` 之类文本被当 Typst 内容块定界符 → `unclosed delimiter` | `inline_text` 转义集扩充 `[` `]` |
| 3 | **远程图片**（https://…）传给 `image()` | typst 无法加载网络地址，mqy 10 篇失败 | 远程图 → 注释 + `#link` 占位（提示手动下载） |
| 4 | **代码块内容被标记转义**（`_` → `\_`）且嵌套围栏破坏 raw 结构 | al-folio vega 示例文 `unclosed raw text` | 代码块内容原样缓冲输出；围栏长度按内容自适应（含 ``` 时用 ````） |
| 5 | **LaTeX 数学重写管线缺失**：`\frac{a}{b}` 花括号形态、`\begin{pmatrix}` 矩阵环境、`mc^2` 多字母变量 | 数学文章公式大量不可编译 | 新管线：环境标签感知的字母拆分 → 命令映射（100+ 条，含 `\left/\right` 剥离、`\to`、`\mathbb→bb` 等）→ 矩阵/cases 环境重写 → 花括号参数重写（`frac{a}{b}`→`frac(a,b)`、`sqrt[3]{x}`→`sqrt(3,x)`，嵌套多轮收敛） |
| 6 | **YAML front-matter**：内联数组 `tags: [a, b]` 解析出脏值 `[a`；categories/aliases/updated 不支持；值含 `"` 不转义 | Jekyll/Hexo 博客标配写法全踩 | 内联数组兼容、新增 categories/aliases/updated/description→excerpt 映射、引号转义 |
| 7 | **脚注/任务列表/删除线/HTML 块静默丢弃** | 内容丢失 | 脚注定义内联到引用处 `#footnote[…]`；任务列表 ☑/☐；`~~x~~`→`#strike[x]`；块级 HTML 保留为 Typst 注释 |

### 2.3 最终结果

| 语料 | 导入 | 构建成功 | 失败说明 |
|---|---|---|---|
| al-folio | 33/33 | **32/33（97%）** | 唯一失败 `distill` 篇是**美元符号用法演示**（故意展示裸 `$`，对任何 `$`-数学导入器都是病态输入：prose 被误配对为公式） |
| mqy | 35/35 | **35/35（100%）** | — |
| **合计** | **68/68** | **67/68（98.5%）** | |

转换质量抽查（al-folio math 篇）：front-matter 五键全对（含内联数组和 categories），
`$$E=mc^2$$` → `$ E = m c^2 $`（多字母拆分），`\langle/\rangle`、`\|x\|` 正确；
中文文章引用块、代码高亮、TOC 全部正常（截图实证）。

### 2.4 截图

| 截图 | 内容 |
|---|---|
| `_shots/2026-09-09-import-mqy-home.png` | mqy 中文博客迁移站点首页（列表+摘要） |
| `_shots/2026-09-09-import-article.png` | 迁移文章页：`#quote` 引用块、TOC、代码高亮、serve 复制按钮 |
| `_shots/2026-09-09-zhihu-latex.png` | 知乎产物浏览器预览：标题层级 + 行内 LaTeX 公式 |

## 3. 验证与回归

- `cargo test`：**196 单测 + 2 e2e 全部通过**（新增 15 个 LaTeX 转换 + 7 个 import 回归用例）；
- `cargo clippy`：**0 警告**；
- 全程未提交 commit，工作区改动：`typall/src/latex.rs`、`typall/src/import_md.rs`、`typall/src/main.rs`、`docs/`、README（昨日对齐遗留）、`_shots/`。

## 4. 已知局限与后续建议

1. **`$` 病态输入**（distill 篇）：作者在正文里演示裸 `$` 用法时，任何基于 `$…$` 的 Markdown 数学解析都会误配对。可在 import 文档标注「建议源文用 `$$` 或转义」。
2. **行内 HTML 标签仅保留文本**（`<b>x</b>` → `x`）；`\label`/`\eqref` 等引用类命令、`align` 对齐符 `&` 未翻译。
3. **LaTeX 转换器剩余 3 条残留**均为 Typst 代码调用（`calc.round` 等），不可数学翻译，维持原样。
4. 建议下一步：对真实目标博客做一次**线上迁移演练**（用户自己的博客），把 import 的 slug/图片路径策略再校准一轮；知乎真机粘贴实测仍未闭环（见 COPY-TO-PLATFORM-DESIGN.md 待实测项）。
