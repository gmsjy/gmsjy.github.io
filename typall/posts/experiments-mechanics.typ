#let title = "力学实验：从打点计时器到单摆"
#let date = "2026-09-05"
#let tags = ("物理", "实验", "力学", "高中")
#let series = "高中物理实验"
#let series_weight = 1
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

实验是物理学的地基。力学三个必做实验——弹簧弹力、加速度与力质量的关系、单摆测重力加速度——分别教会你三件事：*图像法处理数据*、*控制变量法*、*间接测量*。这三个方法比任何一条结论都重要。

= 探究弹簧弹力与形变量的关系

把弹簧悬挂起来，下端挂砝码。平衡时砝码重力等于弹力：$F = m g$。逐次加砝码，记录弹簧伸长量 $x$（用刻度尺读出"挂 $m$ 个砝码时的总长度"再减去原长）。

#fig(cetz.canvas(length: 0.95cm, {
  // 装置：支架 + 弹簧 + 砝码 + 标尺
  line((-1.8, 4.2), (3.2, 4.2), stroke: (paint: black, thickness: 2pt))
  line((-1.5, 4.2), (-1.5, 0.2), stroke: (paint: black, thickness: 2pt))
  line((-1.5, 0.2), (-1.1, 0.2), stroke: black)
  // 标尺
  rect((2.2, 0.2), (2.7, 4.2), stroke: black)
  for i in range(9) {
    line((2.2, 0.6 + i * 0.45), (2.45, 0.6 + i * 0.45), stroke: black)
  }
  content((2.95, 4.0), [刻度尺], anchor: "west", size: 8pt)
  // 弹簧（折线）+ 指针 + 砝码
  let sx = 0.6
  line((sx, 4.2), (sx + 0.25, 3.9), (sx - 0.25, 3.6), (sx + 0.25, 3.3), (sx - 0.25, 3.0), (sx + 0.25, 2.7), (sx - 0.25, 2.4), (sx, 2.1), stroke: black)
  line((sx, 2.1), (sx, 1.5), stroke: black)
  rect((sx - 0.5, 0.9), (sx + 0.5, 1.5), stroke: black, fill: rgb("#e6e2d6"))
  content((sx, 1.2), $ m $, size: 9pt)
  // 指针
  line((sx + 0.5, 1.5), (2.2, 1.5), stroke: (paint: rgb("#d64541"), dash: "dashed"))
  content((3.4, 2.1), [逐次加砝码，读出弹簧末端位置], anchor: "west", size: 8pt)
}))

理论预期是*胡克定律*：弹力与形变量成正比，

$ F = k x $ <eq:hooke>

数据处理用*图像法*：以 $x$ 为横轴、$F$ 为纵轴描点，若各点落在一条过原点的直线上，比例关系成立，且*斜率就是劲度系数 $k$*。图像法的妙处：个别偏离太远的点一眼可见（舍去），不必逐个数据反复验算。

#fig(cetz.canvas(length: 0.95cm, {
  line((-0.3, 0), (4.6, 0), mark: (end: ">"), stroke: black)
  line((0, -0.3), (0, 3.4), mark: (end: ">"), stroke: black)
  content((4.8, 0), $ x \/ "cm" $, size: 9pt)
  content((0.4, 3.5), $ F \/ "N" $, size: 9pt)
  content((0.35, -0.35), [0], size: 8pt)
  for i in range(6) {
    let x = 0.6 + i * 0.6
    circle((x, 0.35 * i + 0.1), radius: 0.05, fill: rgb("#0b6fc0"), stroke: none)
  }
  line((0, 0.05), (4.2, 2.9), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  content((3.7, 3.1), [斜率 = $k$], size: 9pt)
}))

= 探究加速度与力、质量的关系

研究 $a$、$F$、$m$ 三个量的关系要用*控制变量法*：先保 $m$ 不变探究 $a$–$F$，再保 $F$ 不变探究 $a$–$m$。

装置：小车放在长木板上，系着细绳绕过定滑轮挂沙桶；小车后拖纸带，打点计时器以 50 Hz 打点——*纸带上的点距就是加速度的原始记录*（逐差法求 $a$）。

#fig(cetz.canvas(length: 0.95cm, {
  // 打点计时器 + 小车 + 滑轮 + 沙桶
  line((-0.5, 1.2), (7.0, 1.2), stroke: (paint: black, thickness: 2pt))
  rect((-0.2, 1.3), (0.9, 1.9), stroke: black, fill: rgb("#e6e2d6"))
  content((0.35, 2.15), [打点计时器], size: 8pt)
  rect((1.2, 1.35), (3.0, 1.95), stroke: black, fill: rgb("#0b6fc0").lighten(85%))
  circle((1.6, 1.25), radius: 0.15, stroke: black)
  circle((2.6, 1.25), radius: 0.15, stroke: black)
  content((2.1, 2.2), [小车 $m$], size: 8.5pt)
  // 纸带
  line((-1.6, 1.55), (1.2, 1.55), stroke: black)
  for i in range(12) {
    circle((-1.45 + i * 0.22, 1.55), radius: 0.025, fill: black, stroke: none)
  }
  content((-0.5, 1.15), [纸带：点距变大 → 加速], size: 7.5pt)
  // 滑轮与沙桶
  circle((7.0, 1.85), radius: 0.22, stroke: black)
  line((3.0, 1.65), (6.78, 1.9), stroke: black)
  line((7.22, 1.7), (7.22, 0.5), stroke: black)
  rect((6.85, 0), (7.6, 0.5), stroke: black, fill: rgb("#e6e2d6"))
  content((7.9, 0.3), [沙桶 $m prime$], anchor: "west", size: 8.5pt)
  content((7.5, 2.4), [定滑轮], size: 8pt)
}))

两个关键步骤决定成败：

+ *平衡摩擦力*：垫高木板无绳一端，轻推小车，纸带点距均匀即说明重力沿斜面分量抵消了摩擦力。这样绳的拉力才等于小车受到的合力。
+ *近似条件*：只有当沙和沙桶质量 $m prime$ *远小于*小车质量 $m$ 时，绳的拉力才近似等于 $m prime g$。

结论呈现为两条图线：$m$ 不变时 $a$–$F$ 是过原点直线（斜率 $= 1 \/ m$）；$F$ 不变时 $a$–$1 \/ m$ 是过原点直线。*画 $a$–$1 \/ m$ 而不是 $a$–$m$*，正是图像法的又一次运用——把"反比"变成可检验的"正比"。

= 用单摆测重力加速度

单摆的周期公式（摆角小于 $5 degree$ 时成立）：

$ T = 2 pi sqrt(L \/ g) quad ==> quad g = (4 pi^2 L) / T^2 $ <eq:pendulum>

摆长 $L$ 是*悬点到球心*的距离（线长 + 球半径），不是线长。为减小计时误差，不测单个周期，而是用秒表测 *30–50 个全振动*的总时间再除以次数。

#fig(cetz.canvas(length: 0.95cm, {
  // 铁架台 + 摆线 + 摆球 + 弧线
  line((-1.6, 4.3), (2.4, 4.3), stroke: (paint: black, thickness: 2pt))
  line((2.0, 4.3), (2.0, 0.3), stroke: (paint: black, thickness: 2pt))
  line((2.0, 4.3), (2.0, 4.3), stroke: black)
  circle((2.0, 4.3), radius: 0.06, fill: black, stroke: none)
  content((1.65, 4.05), [悬点 O], anchor: "east", size: 8pt)
  line((2.0, 4.3), (2.0, 1.3), stroke: black)
  circle((2.0, 1.1), radius: 0.22, stroke: black, fill: rgb("#e6e2d6"))
  arc((2.0, 4.3), radius: 3.0, start: 260deg, stop: 280deg, mark: (end: ">"), stroke: rgb("#d64541"))
  content((-0.7, 1.5), [摆角 $theta < 5 degree$], size: 8pt)
  line((2.0, 4.3), (0.4, 1.25), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  content((-0.15, 2.9), $ L $, size: 9pt)
  content((0.15, 1.0), [摆球], size: 8pt)
  line((2.0, 4.3), (2.0, 0.9), stroke: (paint: rgb("#0b6fc0"), thickness: 1.1pt))
  content((2.1, 0.75), [线长 + 球半径 = 摆长], anchor: "west", size: 8pt)
}))

一组示例数据（$g$ 的测量值与本地标准值 $9.79$ 相比偏差在偶然误差内）：

#table(
  columns: 5,
  align: center,
  table.header([*次数*], [*摆长 $L$ / m*], [*50$T$ 时间 / s*], [*周期 $T$ / s*], [g / (m/s²)]),
  [1], [1.000], [99.5], [1.99], [9.96],
  [2], [1.000], [99.8], [1.996], [9.90],
  [3], [0.800], [89.2], [1.784], [9.91],
  [4], [0.600], [77.4], [1.548], [9.90],
)

*误差要点*：摆角必须小（大了周期公式失效）；计时从*最低点*启动（此处速度最大，位置判断最准）；摆线选细轻不易拉伸的。若图像法处理，作 $T^2$–$L$ 图线，斜率 $= 4 pi^2 \/ g$，可进一步平滑偶然误差。

= 小结

- 弹簧实验：图像法处理数据，$F$–$x$ 直线斜率即 $k$（胡克定律 $F = k x$）；
- $a$–$F$–$m$ 实验：控制变量法 + 平衡摩擦力 + $m prime minus m$ 近似条件；画 $a$–$1 \/ m$ 图线；
- 单摆实验：间接测量 $g = 4 pi^2 L \/ T^2$，摆角小、测多周期、摆长算到球心；
- 三个实验合起来就是实验题的三大方法论：图像化数据、控制变量、间接测量。
