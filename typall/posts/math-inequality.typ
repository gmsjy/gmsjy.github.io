#let title = "等式性质与不等式性质：从比大小到基本不等式"
#let date = "2026-09-02"
#let tags = ("数学", "不等式", "高中")
#let series = "高中数学"
#let series_weight = 2
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

不等式是高中数学里*工具性最强*的内容——求值域、证最值、讨论单调性都要用到它。这一篇先把"比大小"的地基打牢，再拿下高考最爱考的基本不等式与一元二次不等式。

= 不等式的性质（地基）

设 $a, b, c in RR$，不等式的核心性质都从"数轴上的大小"来：

#table(
  columns: (auto, 1fr),
  inset: 6pt,
  [*性质*], [*内容*],
  [传递性], [$a > b, b > c arrow.r.double a > c$],
  [可加性], [$a > b arrow.r.double a + c > b + c$（同加同减不变向）],
  [可乘性], [$a > b, c > 0 arrow.r.double a c > b c$；$a > b, c < 0 arrow.r.double a c < b c$（乘负数要变号！）],
  [同向可加], [$a > b, c > d arrow.r.double a + c > b + d$],
  [同正可乘], [$a > b > 0, c > d > 0 arrow.r.double a c > b d$],
  [乘方/开方], [$a > b > 0 arrow.r.double a^n > b^n, root(n, a) > root(n, b)$（$n in NN^"*"$）],
  [倒数], [$a > b > 0 arrow.r.double 1/a < 1/b$（正数同号取倒数变号）],
)

> 易错点：乘负数必须变号，这是不等式运算与等式最大的不同。判断"能否推出"的选择题，逐条对照上表，拿不准就取特殊值验证（如 $a = 2, b = 1, c = -1$）。

= 基本不等式（高考最值工具）

对于*正数* $a, b$，有：

$ (a + b)/(2) >= sqrt(a b) $ <eq:am-gm>

当且仅当 $a = b$ 时取等号。左边叫*算术平均数*，右边叫*几何平均数*——"算术平均不小于几何平均"。

#fig(cetz.canvas(length: 0.9cm, {
  // 均值不等式的半圆解释：半径 OC >= 弦高 CD
  let pts = range(0, 41, step: 1).map(k => {
    let t = k / 40.0 * calc.pi
    (3 + 3 * calc.cos(t), 3 * calc.sin(t))
  })
  line(..pts, stroke: rgb("#9aa2ad"))
  line((0, 0), (6, 0), stroke: black)
  // C 把直径分成 a = 2 与 b = 4，CD 垂直直径交圆于 D
  line((2, 0), (2, 2.83), mark: (end: ">"), stroke: (paint: rgb("#3c8a4d"), thickness: 1.3pt))
  line((3, 0), (2, 2.83), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  circle((3, 0), radius: 0.07, fill: black, stroke: none)
  circle((2, 2.83), radius: 0.07, fill: rgb("#3c8a4d"), stroke: none)
  content((1, -0.45), $ a $, size: 9pt)
  content((4, -0.45), $ b $, size: 9pt)
  content((3.3, -0.3), $ O $, size: 9pt)
  content((1.6, 3.05), $ D $, size: 9pt)
  content((2.35, 1.5), $ "CD" = sqrt(a b) $, anchor: "west", size: 8pt)
  content((1.35, 0.85), $ "OC" = (a + b) / 2 $, anchor: "east", size: 8pt)
  content((3, -1.35), "半圆里一眼看懂：半径不短于弦高", size: 8pt)
}))

== 使用三原则：一正、二定、三相等

1. *一正*：$a, b$ 必须都是正数（负数不适用，先取相反数或变形）；
2. *二定*：和 $a + b$ 为定值时，积 $a b$ 有最大值；积 $a b$ 为定值时，和 $a + b$ 有最小值——必须出现"定值"才能用；
3. *三相等*：验证等号能否取到，即 $a = b$ 是否在允许范围内。取不到等号时，最值要另找方法（如对勾函数、单调性）。

#table(
  columns: (auto, 1fr, 1fr),
  inset: 6pt,
  [*题型*], [*处理套路*], [*示例*],
  [和定求积最大], [直接套用，$a = b$ 时最大], [$x > 0$，$x + 2/x$ 求最值],
  [积定求和最小], [拆配出定值], [$x > 1$，$x + 1/(x-1)$ 求最值],
  ["1"的代换], [乘以 $1$ 的等价式再展开], [已知 $1/a + 1/b = 1$，求 $a + b$ 最值],
  [分母有变量], [先换元或凑分母], [求 $(x^2 + 3)/(x)$ 最小值],
)

> "1 的代换"是高频技巧：已知 $x, y > 0$ 且 $2/x + 1/y = 1$，求 $x + 2y$ 的最小值。做法：$(x + 2y)(2/x + 1/y)$ 展开得 $4 + x/y + (4y)/x >= 4 + 4 = 8$，当 $x = 2y$ 即 $x = 4, y = 2$ 时取等。

= 一元二次不等式（高考必考）

== 三个"二次"的关系

一元二次不等式 $a x^2 + b x + c > 0$（$a > 0$）的解法，本质是看二次函数图像在 $x$ 轴上方还是下方。设对应方程 $a x^2 + b x + c = 0$ 的两根为 $x_1 < x_2$，判别式 $Delta = b^2 - 4 a c$：

#table(
  columns: (auto, 1fr, 1fr),
  inset: 6pt,
  [$Delta$ 情形], [$a x^2 + b x + c > 0$ 解集], [$a x^2 + b x + c < 0$ 解集],
  [$Delta > 0$], [两根之外：$x < x_1$ 或 $x > x_2$], [两根之间：$x_1 < x < x_2$],
  [$Delta = 0$], [$\{x | x != x_1\}$], [空集 $emptyset$],
  [$Delta < 0$], [全体实数 $RR$], [空集 $emptyset$],
)

> 记忆口诀：*"大于取两边，小于取中间"*（前提是 $a > 0$ 且有两根）。若 $a < 0$，先把两边同乘 $-1$ 化为 $a > 0$ 再解——别硬记符号。

== 含参讨论的固定流程

解含参一元二次不等式，按以下顺序分类：

1. 看*二次项系数*是否为 0（是一次还是二次）；
2. 二次时求判别式 $Delta$，讨论与 0 的关系（两根存在性）；
3. 有根时比较*两根大小*（尤其含参时 $x_1$ 与 $x_2$ 谁大取决于参数）；
4. 最后套"大于取两边、小于取中间"。

$ a x^2 + b x + c > 0 $ 对 $x in RR$ 恒成立 $<=> a > 0$ 且 $Delta < 0$（二次）；$a = 0$ 时退化为一次恒成立问题，需单独讨论。

= 分式不等式与绝对值不等式（延伸）

分式不等式 $(f(x))/(g(x)) >= 0$ 不能直接"两边乘分母"（不知道正负），要移项通分转化为整式不等式：

$ (f(x))/(g(x)) >= 0 <=> f(x) g(x) >= 0, quad g(x) != 0 $

> 易错点：分式不等式转整式后，分母 $g(x) != 0$ 这个约束*必须保留*——否则会把使分母为零的"假根"混进解集。带等号时尤其要小心。

绝对值不等式 $|x| < a <=> -a < x < a$（$a > 0$），$|x| > a <=> x < -a$ 或 $x > a$——"小于夹中间，大于分两边"，与一元二次不等式口诀同构。
