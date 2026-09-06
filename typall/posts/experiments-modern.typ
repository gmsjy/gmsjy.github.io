#let title = "热学·光学·近代实验：给看不见的量一把尺"
#let date = "2026-09-05"
#let tags = ("物理", "实验", "热学", "光学", "高中")
#let series = "高中物理实验"
#let series_weight = 3
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

分子的大小、玻璃的折射率、光的波长——这些量小到或抽象到无法直接用尺量。三个经典实验的共同智慧：*把看不见的量放大成看得见的量*。

= 油膜法估测分子的大小

思路朴素到惊人：让一滴已知体积的油酸酒精溶液滴到水面上，油酸铺开成*单分子层*的油膜（近似视为圆形）。膜厚就是分子直径：

$ d = V / S $ <eq:oil>

$V$ 是纯油酸的体积（一滴溶液的体积 × 浓度稀释比），$S$ 用透明方格纸数出来的油膜面积（数格子的规则：不足半格舍去、超过半格算一格）。

#fig(cetz.canvas(length: 0.95cm, {
  // 水盘 + 油膜圆 + 局部放大：单分子层
  rect((0, 0.4), (4.6, 2.8), stroke: black, fill: rgb("#0b6fc0").lighten(88%))
  content((0.35, 2.45), [水盘], size: 8pt)
  circle((2.3, 1.6), radius: 1.0, stroke: (paint: rgb("#d64541"), thickness: 1.4pt))
  content((2.3, 1.6), [油膜 S], size: 8.5pt)
  content((3.85, 0.65), [撒痱子粉显边界], size: 7.5pt)
  // 放大：单分子层立方
  line((5.2, 2.9), (4.0, 1.9), stroke: (paint: rgb("#9aa2ad"), thickness: 0.8pt))
  rect((5.4, 1.2), (8.4, 2.9), stroke: black, fill: rgb("#e6e2d6"))
  line((5.4, 2.45), (8.4, 2.45), stroke: (paint: rgb("#d64541"), dash: "dashed"))
  content((6.9, 2.65), [单分子层，厚 $d$], size: 8.5pt)
  for i in range(6) {
    line((6.0 + i * 0.45, 1.2), (6.0 + i * 0.45, 2.45), stroke: (paint: rgb("#9aa2ad"), thickness: 0.7pt))
  }
  content((6.9, 0.9), [放大看：分子一个挨一个立着], size: 8pt)
}))

误差主要有三处：一滴溶液的体积偏大（用容量瓶浓度稀释 + 累积法测几十滴取平均）、油膜形状不规则（数格子误差）、油酸没有完全铺开（水盘要洗净、痱子粉薄而匀）。算出来的 $d$ 在 $10^(-10) "m"$ 量级——与分子直径的真实数量级一致，这就是"用宏观的尺量微观的世界"。

= 测玻璃的折射率

一块平行玻璃砖 + 几枚大头针就能测出折射率。原理是折射定律 $n = sin theta_1 \/ sin theta_2$：$theta_1$ 是*空气中*的入射角，$theta_2$ 是玻璃中的折射角。

#fig(cetz.canvas(length: 1.0cm, {
  // 玻璃砖 + 入射/折射光线 + 法线
  rect((1, 0.4), (6, 2.8), stroke: black, fill: rgb("#0b6fc0").lighten(90%))
  content((5.6, 0.65), [玻璃砖], size: 8.5pt)
  line((3.5, 0.4), (3.5, 2.8), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  content((3.65, 2.55), [法线], size: 8pt)
  line((1.2, 3.4), (3.5, 2.8), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  line((3.5, 2.8), (3.5, 0.4), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  line((3.5, 0.4), (5.8, -0.2), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  arc((3.5, 2.8), radius: 0.9, start: 180deg, stop: 260deg, stroke: rgb("#0b6fc0"))
  content((2.6, 2.35), $ theta_1 $, size: 9pt)
  arc((3.5, 2.8), radius: 0.7, start: 270deg, stop: 318deg, stroke: rgb("#3c8a4d"))
  content((3.75, 1.7), $ theta_2 $, size: 9pt)
  // 大头针
  circle((1.2, 3.4), radius: 0.05, fill: black, stroke: none)
  circle((2.2, 3.1), radius: 0.05, fill: black, stroke: none)
  content((0.4, 3.7), [大头针 $P_1 P_2$ 定入射方向], size: 8pt)
}))

插针法操作：在纸上画好玻璃砖边界，一侧插两枚针 $P_1 P_2$ 定入射光线，透过玻璃砖观察并插 $P_3 P_4$ 挡住它们的像，连线即出射光——入射点与出射点一连，折射光线就"显影"了。量出 $theta_1$、$theta_2$（或用直尺量线段长换算正弦比，避免用量角器），多组数据求平均。示例：

#table(
  columns: 4,
  align: center,
  table.header([*次数*], [*入射角 $theta_1$*], [*折射角 $theta_2$*], [*$n$*]),
  [1], [30°], [19°], [1.53],
  [2], [45°], [28°], [1.51],
  [3], [60°], [35°], [1.51],
)

= 双缝干涉测光的波长

光的波长比分子还小，但干涉条纹的*间距*是毫米级的——把"$lambda$"放大成"$Delta x$"：

$ lambda = (d Delta x) / L $ <eq:double-slit>

$d$ 是双缝间距（出厂刻好），$L$ 是双缝到屏的距离（米尺测），$Delta x$ 是相邻亮条纹中心间距（*测 $n$ 条条纹的总宽度再除以 $(n-1)$*，累积法again）。

#fig(cetz.canvas(length: 0.95cm, {
  // 光源 + 单缝 + 双缝 + 屏 + 条纹
  line((0, 1.7), (1.4, 1.7), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.2pt))
  content((0.6, 2.0), [激光], size: 8.5pt)
  rect((1.4, 1.3), (1.7, 2.1), stroke: black, fill: rgb("#e6e2d6"))
  content((1.55, 2.35), [单缝], size: 8pt)
  rect((3.0, 1.1), (3.3, 2.3), stroke: black, fill: rgb("#e6e2d6"))
  line((3.15, 1.1), (3.15, 1.55), stroke: white)
  line((3.15, 1.85), (3.15, 2.3), stroke: white)
  content((3.15, 2.55), [双缝 $d$], size: 8pt)
  line((3.15, 1.7), (3.15, 1.7), stroke: black)
  for i in range(3) {
    line((3.2 + i * 0.35, 1.7), (3.5 + i * 0.35, 1.35 - i * 0.15), stroke: (paint: rgb("#9aa2ad"), thickness: 0.7pt))
    line((3.2 + i * 0.35, 1.7), (3.5 + i * 0.35, 2.05 + i * 0.15), stroke: (paint: rgb("#9aa2ad"), thickness: 0.7pt))
  }
  content((4.6, 2.7), [L], size: 9pt)
  line((4.55, 2.5), (4.55, 1.7), stroke: black)
  line((4.55, 2.5), (4.65, 2.5), stroke: black)
  line((4.55, 1.7), (4.65, 1.7), stroke: black)
  // 屏上的条纹
  for i in range(7) {
    line((4.75, 0.6 + i * 0.42), (4.75, 0.82 + i * 0.42), stroke: (paint: rgb("#d64541"), thickness: 1.2pt))
  }
  content((4.75, 3.0), [屏：亮暗相间、等间距], size: 8pt)
}))

条纹是等间距的红黑相间——间距 $Delta x$ 用测量头（螺旋测微器式的游标）读出。这个实验的历史分量极重：1801 年托马斯·杨用它第一次证明了光是波，"干涉条纹"至今仍是波动性的铁证。

= 小结

- 油膜法：$d = V \/ S$，用宏观面积量微观直径，数量级 $10^(-10) "m"$；
- 折射率：插针法定光路，$n = sin theta_1 \/ sin theta_2$，多组取平均；
- 双缝干涉：$lambda = d Delta x \/ L$，把波长放大成条纹间距，累积法测 $Delta x$；
- 三个实验共享同一思想：*当量小到测不了，就为它造一个放大器*——油膜是放大器，角度是放大器，条纹间距也是放大器。
