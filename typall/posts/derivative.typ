#let title = "导数及其应用：用极限的眼光看变化"
#let date = "2026-09-04"
#let tags = ("数学", "导数", "高中")
#let series = "高中数学"
#let series_weight = 15
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 导数的概念

函数 $y = f(x)$ 在 $x_0$ 处的*导数*，是平均变化率 $((f(x_0 + Delta x) - f(x_0))/(Delta x))$ 当 $Delta x -> 0$ 时的极限：

$ f'(x_0) = lim_(Delta x -> 0) (f(x_0 + Delta x) - f(x_0))/(Delta x) $ <eq:deriv>

导数的*几何意义*：$f'(x_0)$ 是曲线 $y = f(x)$ 在点 $(x_0, f(x_0))$ 处切线的斜率。物理意义：位移对时间求导是速度，速度对时间求导是加速度——导数就是"瞬时变化率"。

#fig(cetz.canvas(length: 0.9cm, {
  // 抛物线在点 P 处的切线：斜率即导数
  line((-0.3, 0), (6.4, 0), mark: (end: ">"), stroke: black)
  line((0, -0.3), (0, 3.3), mark: (end: ">"), stroke: black)
  content((6.3, 0.4), $ x $, size: 9pt)
  content((0.4, 3.2), $ y $, size: 9pt)
  let pts = range(0, 41, step: 1).map(k => {
    let x = k / 40.0 * 6
    (x, 0.4 * (x - 2.8) * (x - 2.8) + 0.4)
  })
  line(..pts, stroke: (paint: rgb("#d64541"), thickness: 1.4pt))
  // P 在 x = 1.6 处：y = 0.4 * 1.44 + 0.4 = 0.976，斜率 0.8(x-2.8) = -0.96
  circle((1.6, 0.98), radius: 0.08, fill: rgb("#0b6fc0"), stroke: none)
  line((0.3, 2.23), (2.6, 0.02), stroke: (paint: rgb("#0b6fc0"), thickness: 1.2pt))
  content((1.25, 1.35), $ P $, size: 9pt)
  content((1.05, 1.75), "切线", size: 8pt)
  content((3.15, 2.7), "切线斜率 = 该点的导数", size: 8pt)
}))

= 基本初等函数的导数公式

#table(
  columns: (auto, auto, auto, auto),
  inset: 6pt,
  [*函数*], [*导数*], [*函数*], [*导数*],
  [$c$], [$0$], [$x^n$], [$n x^(n-1)$],
  [$sin x$], [$cos x$], [$cos x$], [$-sin x$],
  [$e^x$], [$e^x$], [$a^x$], [$a^x ln a$],
  [$ln x$], [$1/x$], [$log_a x$], [$1/(x ln a)$],
)

> 记忆：幂函数"指数搬下来、指数减一"；正弦余弦"一求导就互换、还带符号循环"（$sin -> cos -> -sin -> -cos$）；指数函数求导"自己乘 $ln$ 底数"（$e^x$ 是"自己"最省心的特例）。

= 求导法则

- 和差：$(u plus.minus v)' = u' plus.minus v'$；
- 积：$(u v)' = u' v + u v'$；
- 商：$(u/v)' = (u' v - u v')/(v^2)$（$v != 0$）；
- 复合函数：$[f(g(x))]' = f'(g(x)) dot g'(x)$——"由外向内，逐层求导再相乘"。

> 复合函数求导是高考最爱挖的坑：先对"外层"求导、括号里的整体不变，再乘"内层"的导数。例：$(sin 2x)' = cos 2x dot 2 = 2 cos 2x$，漏掉 $2$ 就错了。

= 导数的应用

== 切线与法线

曲线在 $(x_0, y_0)$ 处的切线方程：

$ y - f(x_0) = f'(x_0)(x - x_0) $ <eq:tangent>

> 易错点：求切线先判断*切点是否已知*。若已知切点在曲线上，直接代入 @eq:tangent；若只知道切线过某一点（未必是切点），要设切点、用斜率相等列方程求解。二者常被混为一谈。

== 单调性

在区间内：$f'(x) > 0 => f(x)$ 单调递增；$f'(x) < 0 => f(x)$ 单调递减。求单调区间的步骤：求导 → 令 $f'(x) = 0$ 找"临界点" → 列表判断各区间符号。

== 极值与最值

*极值点*：导函数在 $x_0$ 左右两侧*变号*（左正右负为极大值点，左负右正为极小值点）。*最值*：在闭区间 $[a, b]$ 上，比较所有极值与端点值 $f(a)$、$f(b)$，最大者为最大值、最小者为最小值。

> 关键区分：$f'(x_0) = 0$ 是 $x_0$ 为极值点的*必要不充分条件*——例如 $f(x) = x^3$ 在 $x = 0$ 处导数为零但无极值（两侧同号）。判断极值要看"导数是否变号"，不是看"导数是否为零"。

= 典型应用：恒成立与最值

- *求参数范围*："$f(x) >= 0$ 恒成立" 等价于 "$f(x)_min >= 0$"；
- *证明不等式*：构造 $F(x) = f(x) - g(x)$，证 $F(x)_min >= 0$；
- *实际优化*：利润最大、用料最省，设变量列函数，求导找最值。

> 思路总纲：导数题的核心动作只有两个——"求导、判断符号"。符号由导函数的零点划分，必要时对导函数再求导（二阶导）判断原导函数的单调性，从而确定符号。
