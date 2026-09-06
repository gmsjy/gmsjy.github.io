#let title = "集合与常用逻辑用语：数学的普通话"
#let date = "2026-09-02"
#let tags = ("数学", "集合与逻辑", "高中")
#let series = "高中数学"
#let series_weight = 1
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 集合：把对象装进"袋子"

*集合*是数学中最基础的概念：把一些确定的、彼此不同的对象汇集在一起，就构成一个集合。构成集合的对象叫*元素*。它有三个基本特征，是判断"是不是集合"的试金石：

- *确定性*：任一对象要么属于该集合，要么不属于，不能模棱两可。"高个子的人"不是集合（多高算高？），"身高超过 1.8 m 的人"是集合；
- *互异性*：集合中的元素互不相同，$\{1, 2\}$ 与 $\{1, 2, 2\}$ 表示同一个集合；
- *无序性*：集合与元素的排列顺序无关，$\{1, 2\} = \{2, 1\}$。

常用数集用花体字母记（都是"确定的"集合）：

#table(
  columns: (auto, auto, auto, auto),
  inset: 6pt,
  [*记号*], [*含义*], [*记号*], [*含义*],
  [$NN$], [自然数集（含 0）], [$ZZ$], [整数集],
  [$QQ$], [有理数集], [$RR$], [实数集],
  [$NN^"*"$], [正整数集], [$emptyset$], [空集（不含任何元素）],
)

== 表示方法

- *列举法*：把元素一一列出，如 $\{1, 2, 3\}$、$\{x, y\}$；
- *描述法*：写清元素的共同特征，如 $\{x in RR | x^2 < 2\}$、$\{(x, y) | y = x^2\}$——竖线前写"元素长什么样"，竖线后写"满足什么条件"。

> 易错点：描述法中竖线两侧内容别写反，集合元素是"谁"要看清。$\{x | x^2 < 2\}$ 是数集，$\{(x, y) | y = x^2\}$ 是点集——高考常在这里设陷阱。

= 集合间的基本关系

== 子集与真子集

若集合 $A$ 的*每一个*元素都属于 $B$，则 $A$ 是 $B$ 的*子集*，记 $A subset.eq B$。若 $A subset.eq B$ 且 $B$ 中至少有一个元素不属于 $A$，则 $A$ 是 $B$ 的*真子集*，记 $A subset B$。

*空集是任何集合的子集，是任何非空集合的真子集*——这是最容易漏掉的考点，讨论子集个数时千万别忘了 $emptyset$。

*子集个数公式*：$n$ 个元素的集合有 $2^n$ 个子集、$2^n - 1$ 个真子集。

> 记忆：每个元素"选或不选"独立决策，$n$ 个元素就是 $2 dot 2 dot ... dot 2 = 2^n$ 种子集。含元素的"选法"就对应真子集，去掉"全选"那一种。

== 集合相等

$ A = B $ 当且仅当 $A subset.eq B$ 且 $B subset.eq A$——互相包含即相等。证明两个集合相等，标准写法就是"先证 $A subset.eq B$，再证 $B subset.eq A$"。

= 集合的基本运算

#fig(cetz.canvas(length: 0.9cm, {
  // 韦恩图：全集 U 中两个相交集合 A、B
  rect((0, -0.3), (7, 3.6), stroke: rgb("#9aa2ad"))
  content((0.45, 3.2), $ U $, size: 9pt)
  circle((2.5, 1.5), radius: 1.35, stroke: rgb("#0b6fc0"))
  circle((4.5, 1.5), radius: 1.35, stroke: rgb("#d64541"))
  content((1.55, 2.4), $ A $, size: 9pt)
  content((5.45, 2.4), $ B $, size: 9pt)
  content((3.5, 1.5), $ A inter B $, size: 8pt)
  content((3.5, -0.75), "两圈的公共部分是交集，覆盖的全部是并集", size: 8pt)
}))

== 交集、并集与补集

$ A inter B = {x | x in A "且" x in B}, quad A union B = {x | x in A "或" x in B} $ <eq:inter-union>

补集运算需要先明确*全集* $U$：

$ complement_A U B = {x in U | x in B} $ <eq:complement>

#table(
  columns: (auto, 1fr, 1fr),
  inset: 6pt,
  [*运算*], [*文字记忆*], [*图形直觉*],
  [交集], ["公共部分", 越交越少], [两个圈重叠的区域],
  [并集], ["合在一起", 越并越多], [两个圈覆盖的全部区域],
  [补集], [全集里"挖掉"它], [大矩形去掉 $B$ 剩下的部分],
)

> 口诀：*"交"取公共，"并"取全部，"补"是挖掉*。$A inter B subset.eq A subset.eq A union B$，子集关系链条可帮助快速判断选项。

== 运算律与常用结论

$ A union A = A, quad A inter A = A, quad A inter emptyset = emptyset, quad A union emptyset = A $

$ A inter complement_A U A = emptyset, quad A union complement_A U A = U $ <eq:law>

以及著名的*摩根律*（德摩根对偶）：$(A union B)^c = A^c inter B^c$，"并的补等于补的交"。

> 含参集合问题标准流程：① 化简集合（解不等式）；② 画数轴/韦恩图；③ 列端点不等式（*注意端点能否取等*——单独验证）；④ 别忘了讨论 $B = emptyset$ 的情形。

= 充分条件与必要条件

若 $p arrow.r.double q$（$p$ 成立则 $q$ 必成立），则 $p$ 是 $q$ 的*充分条件*，$q$ 是 $p$ 的*必要条件*。四类判断全部由此派生：

#table(
  columns: (auto, 1fr),
  inset: 6pt,
  [*关系*], [*判定*],
  [$p arrow.r.double q$ 且 $q cancel(arrow.r.double) p$], [$p$ 是 $q$ 的充分不必要条件],
  [$q arrow.r.double p$ 且 $p cancel(arrow.r.double) q$], [$p$ 是 $q$ 的必要不充分条件],
  [$p arrow.r.double q$ 且 $q arrow.r.double p$], [互为充要条件],
  [两者都不推出], [既不充分也不必要],
)

> 记忆技巧：*"小范围是大范围的充分条件"*。若 $p$ 对应的集合是 $q$ 对应集合的真子集，则 $p arrow.r.double q$ 成立——$x = 1$ 推出 $x^2 = 1$，但反过来不行，所以"$x = 1$"是"$x^2 = 1$"的充分不必要条件。把命题翻译成集合包含关系，判断立刻变简单。

= 全称量词与存在量词

- *全称量词*"所有、任意、每一个"，记 $forall$：$forall x in M, p(x)$；
- *存在量词*"存在、至少有一个"，记 $exists$：$exists x in M, p(x)$。

*命题的否定*是高频考点，规则一句话：*量词换、结论否*。

$ overline(forall x in M", " p(x)) = exists x in M, overline(p(x)), quad overline(exists x in M", " p(x)) = forall x in M, overline(p(x)) $ <eq:negation>

> 易错点：① 否命题与命题的否定不是一回事——否命题要同时否定条件和结论，命题的否定只否结论；② "任意…都大于 0"的否定是"存在一个…不大于 0"（即 $<= 0$），不是"都小于 0"。"都大于"的反面是"至少有一个不大于"，注意边界值。
