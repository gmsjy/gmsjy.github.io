#let title = "圆锥曲线：椭圆、双曲线与抛物线"
#let date = "2026-09-04"
#let tags = ("数学", "解析几何", "圆锥曲线", "高中")
#let series = "高中数学"
#let series_weight = 14
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 圆锥曲线的统一定义

椭圆、双曲线、抛物线都由"平面截圆锥"而来，可统一为：到定点（焦点）的距离与到定直线（准线）的距离之比为常数 $e$（离心率）的点的轨迹。

- $0 < e < 1$：椭圆；
- $e = 1$：抛物线；
- $e > 1$：双曲线。

= 椭圆

*定义*：到两定点 $F_1$、$F_2$ 的距离之和为常数 $2a$（$2a > 2c$，$2c$ 为焦距）的点的轨迹。

*标准方程*（焦点在 $x$ 轴）：

$ x^2/a^2 + y^2/b^2 = 1 quad (a > b > 0) $ <eq:ellipse>

其中 $a^2 = b^2 + c^2$。焦点坐标 $(plus.minus c, 0)$，离心率：

$ e = c/a = sqrt(1 - b^2/a^2) quad (0 < e < 1) $ <eq:ellipse-e>

#fig(cetz.canvas(length: 0.9cm, {
  // 椭圆的定义图：到两焦点距离之和恒为 2a
  let pts = range(0, 41, step: 1).map(k => {
    let t = k / 40.0 * 2 * calc.pi
    (3.5 + 2.8 * calc.cos(t), 1.7 + 1.7 * calc.sin(t))
  })
  line(..pts, stroke: rgb("#9aa2ad"))
  circle((1.28, 1.7), radius: 0.07, fill: rgb("#0b6fc0"), stroke: none)
  circle((5.72, 1.7), radius: 0.07, fill: rgb("#0b6fc0"), stroke: none)
  circle((5.01, 3.13), radius: 0.07, fill: rgb("#d64541"), stroke: none)
  line((1.28, 1.7), (5.01, 3.13), stroke: (paint: rgb("#0b6fc0"), thickness: 1.2pt))
  line((5.72, 1.7), (5.01, 3.13), stroke: (paint: rgb("#d64541"), thickness: 1.2pt))
  content((0.95, 1.2), $ F_1 $, size: 9pt)
  content((5.95, 1.2), $ F_2 $, size: 9pt)
  content((5.35, 3.35), $ P $, size: 9pt)
  content((3.5, 0.35), $ abs(P F_1) + abs(P F_2) = 2 a quad ("常数") $, size: 8pt)
}))

> 记忆：椭圆里"*长轴 $2a$、短轴 $2b$、焦距 $2c$*"，$a$ 最大、$a^2 = b^2 + c^2$（像直角三角形的斜边）。离心率越大，椭圆越扁；越接近 $0$ 越接近圆。

= 双曲线

*定义*：到两定点 $F_1$、$F_2$ 的距离之差的*绝对值*为常数 $2a$（$2a < 2c$）的点的轨迹。

*标准方程*（焦点在 $x$ 轴）：

$ x^2/a^2 - y^2/b^2 = 1 quad (a > 0, b > 0) $ <eq:hyperbola>

其中 $c^2 = a^2 + b^2$。焦点坐标 $(plus.minus c, 0)$，离心率：

$ e = c/a > 1 $ <eq:hyperbola-e>

*渐近线*：双曲线无限逼近的两条直线：

$ y = plus.minus b/a x $ <eq:asymptote>

> 记忆：双曲线里"*$c$ 最大*，$c^2 = a^2 + b^2$"（与椭圆相反，椭圆 $a^2 = b^2 + c^2$）。渐近线是双曲线的"招牌"，写标准方程时 $a$、$b$ 就在渐近线斜率里。

== 等轴双曲线

$a = b$ 时，方程为 $x^2 - y^2 = a^2$，渐近线 $y = plus.minus x$（互相垂直），离心率 $e = sqrt(2)$。

= 抛物线

*定义*：到定点 $F$（焦点）与到定直线（准线）距离相等的点的轨迹。标准方程（开口向右）：

$ y^2 = 2 p x quad (p > 0) $ <eq:parabola>

焦点 $F(p/2, 0)$，准线 $x = -p/2$。四个开口方向对应四种标准方程：

#table(
  columns: (auto, auto, auto, auto),
  inset: 6pt,
  [*开口*], [*方程*], [*焦点*], [*准线*],
  [向右], [$y^2 = 2 p x$], [$(p/2, 0)$], [$x = -p/2$],
  [向左], [$y^2 = -2 p x$], [$(-p/2, 0)$], [$x = p/2$],
  [向上], [$x^2 = 2 p y$], [$(0, p/2)$], [$y = -p/2$],
  [向下], [$x^2 = -2 p y$], [$(0, -p/2)$], [$y = p/2$],
)

> 记忆：抛物线"*一次项定轴、符号定开口*"。方程中哪一项是一次，焦点就在哪条轴上；一次项系数为正，开口朝正方向。焦半径（抛物线上点到焦点的距离）等于该点到准线的距离，这是抛物线最重要的几何性质。

= 直线与圆锥曲线的位置关系

设直线 $y = k x + m$，与圆锥曲线联立消元，得关于 $x$（或 $y$）的一元二次方程，判别式 $Delta$ 判定：

- $Delta > 0$：相交（两个交点）；
- $Delta = 0$：相切（一个交点）；
- $Delta < 0$：相离（无交点）。

> 弦长公式：$|A B| = sqrt(1 + k^2) |x_1 - x_2| = sqrt(1 + k^2) dot sqrt(Delta)/|a|$。韦达定理（$x_1 + x_2$、$x_1 x_2$）是联立后的标准动作，"设而不求"是圆锥曲线大题的灵魂。

= 解题套路

1. 由条件定标准方程（先定位焦点在哪个轴，再求 $a$、$b$、$p$）；
2. 联立直线与曲线，消元得一元二次方程；
3. 用韦达定理表达 $x_1 + x_2$、$x_1 x_2$；
4. 把题目条件（垂直、中点、定值、面积）翻译成 $x_1$、$x_2$ 的式子，代韦达结论求解。

> 易错点：双曲线与直线联立时可能消元后二次项系数为零（直线与渐近线平行），此时只有一个交点，要单独讨论；椭圆、双曲线都要检验"判别式与定义域"。
