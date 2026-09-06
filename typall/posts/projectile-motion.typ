#let title = "抛体运动：把一条弯线拆成两条直线"
#let date = "2026-09-02"
#let tags = ("物理", "力学", "抛体运动", "高中")
#let series = "高中物理"
#let series_weight = 4
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

直线运动的公式 $x = v t$、$v = v_0 + a t$ 之所以好用，是因为速度方向从来不变。可自然界到处是"弯着走"的运动——踢飞的足球、扔出的铅球、屋檐滴下的水珠。速度方向时刻在变，怎么办？物理学家的答案是两个字：*分解*。把一条弯线拆成两条互相垂直的直线，分别套用直线公式，最后再合成。这一篇就把"拆"字练透。

= 曲线运动：为什么"弯"了

== 速度方向沿轨迹切线

质点做曲线运动时，任一时刻的速度方向都沿轨迹在该点的*切线方向*，且指向它前进的一侧。所以"速度方向时刻改变"是曲线运动的固有属性——哪怕速度大小不变（如匀速圆周），只要有方向变化，就有加速度。

== 做曲线运动的条件

物体做曲线运动的条件是：*合外力的方向与速度方向不共线*。此时轨迹弯向合外力一侧——力把速度的方向"掰弯"了。最典型的例子：物体被水平抛出后只受竖直向下的重力，$v_0$ 水平、$vec(F)$ 竖直，轨迹就向下弯成抛物线：

#fig(cetz.canvas({
  // 水平基线（浅灰细线）
  line((0.15, 0), (11.4, 0), stroke: rgb(0, 0, 0, 25%))
  // 轨迹：从 (0.9, 0) 抛出的抛物线，y = -0.0464 u^2
  let pts = range(0, 37).map(i => {
    let u = i * 10.0 / 36
    (0.9 + u, -0.0464 * u * u)
  })
  for i in range(0, pts.len() - 1) {
    line(pts.at(i), pts.at(i + 1), stroke: rgb("#d64541"))
  }
  // 初速度 v0：水平向右
  line((0.9, 0), (3.0, 0), stroke: black, mark: (end: ">"))
  content((3.2, 0.18), $ vec(v)_0 $, anchor: "west")
  // 合外力 F：竖直向下
  line((0.9, 0), (0.9, -2.2), stroke: rgb("#2f6fd0"), mark: (end: ">"))
  content((1.2, -1.15), $ vec(F) $, anchor: "west")
  // 抛出点
  circle((0.9, 0), radius: 0.1, fill: black, stroke: none)
  content((0.85, 0.3), $ O $, anchor: "east")
}))

> 看箭头：$vec(F)$ 与 $vec(v)_0$ 不在一条直线上，轨迹就被"拉"向 $vec(F)$ 一侧。合力与速度夹角越大，弯曲越明显。

= 运动的合成与分解

== 合运动与分运动

若物体同时参与两个运动，它实际表现出的运动叫*合运动*，参与的那两个叫*分运动*。位移、速度、加速度都是矢量，合矢量与分矢量满足*平行四边形定则*：

$ vec(v)_"合" = vec(v)_1 + vec(v)_2 $ <eq:vel-add>

分运动之间满足两条重要性质：
- *独立性*：一个分运动的存在与快慢，不影响另一个分运动；
- *等时性*：合运动与各分运动经历的时间完全相同。

== 小船渡河：两个分运动怎么"叠"

河水向右冲（水速 $vec(v)_"水"$），船头垂直对岸开（船速 $vec(v)_"船"$）。船实际走的路线既不正对岸、也不顺流直下，而是斜着到达对岸的某点：

#fig(cetz.canvas({
  // 河岸（上、下）
  line((0.2, 0), (9.6, 0), stroke: rgb("#a1887f"), thickness: 2pt)
  line((0.2, -4.0), (9.6, -4.0), stroke: rgb("#a1887f"), thickness: 2pt)
  // 起点 A
  circle((1.8, 0), radius: 0.1, fill: black, stroke: none)
  content((1.8, 0.3), $ A $, anchor: "west")
  // 分速度：船速（垂直河岸向下）+ 水速（顺流向右）
  line((1.8, 0), (1.8, -4.0), stroke: rgb("#2f6fd0"), mark: (end: ">"))
  line((1.8, 0), (4.4, 0), stroke: rgb("#8e7cc3"), mark: (end: ">"))
  content((2.0, -2.3), $ vec(v)_"船" $, anchor: "west")
  content((3.0, 0.32), $ vec(v)_"水" $, anchor: "west")
  // 平行四边形补齐（浅灰细线）
  line((1.8, -4.0), (4.4, -4.0), stroke: rgb(0, 0, 0, 30%))
  line((4.4, 0), (4.4, -4.0), stroke: rgb(0, 0, 0, 30%))
  // 合速度（红色）——实际航线，直达对岸 A'
  line((1.8, 0), (4.4, -4.0), stroke: rgb("#d64541"), mark: (end: ">"))
  content((4.6, -2.3), $ vec(v)_"合" $, anchor: "west")
  // 到岸点
  circle((4.4, -4.0), radius: 0.1, fill: black, stroke: none)
  content((4.55, -4.25), $ A' $, anchor: "west")
  content((0.4, -2.2), $ d $)
}))

渡河时间只由"垂直河岸的分速度"决定：$t = d / v_"船"$——水流再急，也*拖慢不了*渡河时间，只是把船冲偏、改变靠岸点 $A'$ 的位置。这就是等时性与独立性的直观体现。

= 平抛运动：最经典的"一拆二"

== 定义与分解

*平抛运动*：初速度水平、且只受重力的抛体运动。重力竖直向下、初速度水平，两者恰好垂直——天然适合沿 *水平（x）* 和 *竖直（y）* 两个方向分解：

- 水平方向：不受力，做*匀速直线运动*：$v_x = v_0$，$x = v_0 t$；
- 竖直方向：初速度为零，做*自由落体运动*：$v_y = g t$，$y = 1/2 g t^2$。

任一时该质点位于 $P(x, y)$，这就是它的"位置报告"。

== 位移规律：竖直方向藏着 1 : 4 : 9

每隔相等的时间 $t$ 观察一次：水平位移 $v_0 t, 2 v_0 t, 3 v_0 t$ 均匀递增（匀速的脚印）；竖直位移之比却是

$ 1/2 g t^2 : 1/2 g (2t)^2 : 1/2 g (3t)^2 = 1 : 4 : 9 $

这就证明$1 : 4 : 9$ 正是初速度为零的匀加速直线运动的位移比——它有力地证明：*平抛的竖直分运动就是自由落体*。

#fig(cetz.canvas({
  // 坐标系：x 向右、y 向下为正
  line((0, 0), (11.6, 0), stroke: rgb(0, 0, 0, 45%), mark: (end: ">"))
  line((0, 0), (0, -5.2), stroke: rgb(0, 0, 0, 45%), mark: (end: ">"))
  content((11.7, 0.25), $ x $)
  content((0.28, -5.3), $ y $)
  content((0.25, 0.3), $ O $)
  // 平抛轨迹：x = 3t, y = -0.5 t^2 （t 为"时间单位"）
  let pts = range(0, 33).map(i => {
    let t = i * 3.1 / 32
    (3 * t, -0.5 * t * t)
  })
  for i in range(0, pts.len() - 1) {
    line(pts.at(i), pts.at(i + 1), stroke: rgb("#3a6ea5"))
  }
  // 三个等时位置：水平 3/6/9，竖直 -0.5/-2/-4.5
  let marks = ((3, -0.5), (6, -2.0), (9, -4.5))
  for p in marks {
    // 辅助线：到 x 轴、到 y 轴（浅灰，水平线止于 x=-0.35 避免压住标注）
    line(p, (p.at(0), 0), stroke: rgb(0, 0, 0, 28%))
    line(p, (-0.35, p.at(1)), stroke: rgb(0, 0, 0, 28%))
    circle(p, radius: 0.12, fill: black, stroke: none)
  }
  // 竖直位移标注（1 : 4 : 9）
  content((-1.6, -0.5), $ (g t^2)/2 $, anchor: "east")
  content((-1.6, -2.0), $ 2 g t^2 $, anchor: "east")
  content((-1.6, -4.5), $ (9 g t^2)/2 $, anchor: "east")
  // 水平位移标注
  content((3, 0.32), $ v_0 t $, anchor: "center")
  content((6, 0.32), $ 2 v_0 t $, anchor: "center")
  content((9, 0.32), $ 3 v_0 t $, anchor: "center")
  // 抛点初速度 v0
  line((0, -0.18), (2.5, -0.18), stroke: black, mark: (end: ">"))
  content((2.65, -0.4), $ v_0 $, anchor: "west")
}))

从抛出点看，水平方向三个脚印等距（匀速），竖直方向按 $1 : 4 : 9$ 拉大（匀加速）。把这两幅"脚印图"拼起来，就是整条抛物线。

== 速度规律：合速度沿切线，方向不断"低头"

任意时刻水平分速度不变 $v_x = v_0$，竖直分速度 $v_y = g t$ 随时间均匀增大。合成后：

$ v = sqrt(v_0^2 + (g t)^2) $ <eq:proj-v>

$v$ 的方向沿轨迹切线，且越来越"低头"（与水平方向的夹角越来越大）。设 $v$ 与水平方向夹角为 $theta$（速度偏角），则：

$ tan theta = (v_y)/(v_x) = (g t)/(v_0) $ <eq:proj-theta>

#fig(cetz.canvas({
  // 顶部水平基线（x 方向，抛出点 O 在其上）
  line((0.0, 0), (11.2, 0), stroke: rgb(0, 0, 0, 35%))
  // 平抛轨迹：O=(0.6,0)，y = -K (x-0.6)^2
  let K = 0.0597
  let pts = range(0, 37).map(i => {
    let u = i * 9.0 / 36
    (0.6 + u, -K * u * u)
  })
  for i in range(0, pts.len() - 1) {
    line(pts.at(i), pts.at(i + 1), stroke: rgb("#3a6ea5"))
  }
  // 抛出点 O 与位移 s = O->P
  circle((0.6, 0), radius: 0.1, fill: black, stroke: none)
  content((0.55, 0.3), $ O $, anchor: "west")
  let P = (7.2, -2.6)
  line((0.6, 0), P, stroke: rgb("#b36b00"), mark: (end: ">"))
  content((3.6, -1.35), $ s $, anchor: "west")
  circle(P, radius: 0.12, fill: black, stroke: none)
  content((7.4, -2.75), $ P $, anchor: "west")
  // 过 P 的水平参考线（左侧浅灰）
  line((0.0, -2.6), (7.2, -2.6), stroke: rgb(0, 0, 0, 28%))
  // 速度分解：vx 水平、vy 竖直、v 为对角线（沿切线）
  let vx-e = (8.8, -2.6)
  let vy-e = (7.2, -3.9)
  line(P, vx-e, stroke: black, mark: (end: ">"))
  line(P, vy-e, stroke: rgb("#2e8540"), mark: (end: ">"))
  line(P, (8.8, -3.9), stroke: rgb("#d64541"), mark: (end: ">"))
  // 补成平行四边形（浅灰）
  line(vx-e, (8.8, -3.9), stroke: rgb(0, 0, 0, 25%))
  line(vy-e, (8.8, -3.9), stroke: rgb(0, 0, 0, 25%))
  // 标注
  content((9.0, -2.52), $ v_x $, anchor: "west")
  content((7.45, -3.28), $ v_y $, anchor: "west")
  content((9.0, -3.7), $ vec(v) $, anchor: "west")
  // 位移偏角 alpha（在 O 处）
  let a-pts = range(0, 15).map(i => {
    let a = 21.5 * i / 14
    (0.6 + 1.0 * calc.cos(a * 1deg), -1.0 * calc.sin(a * 1deg))
  })
  for i in range(0, a-pts.len() - 1) {
    line(a-pts.at(i), a-pts.at(i + 1), stroke: rgb("#b36b00"))
  }
  content((1.55, -0.42), $ alpha $)
  // 速度偏角 theta（在 P 处，水平线下方顺时针）
  let t-pts = range(0, 15).map(i => {
    let a = 38.2 * i / 14
    (7.2 + 0.7 * calc.cos(a * 1deg), -2.6 - 0.7 * calc.sin(a * 1deg))
  })
  for i in range(0, t-pts.len() - 1) {
    line(t-pts.at(i), t-pts.at(i + 1), stroke: rgb("#d64541"))
  }
  content((8.0, -2.92), $ theta $)
}))

位移也有个偏角：设抛出点到 $P$ 的位移 $s$ 与水平方向夹角为 $alpha$（位移偏角），由 $x = v_0 t$、$y = 1/2 g t^2$：

$ tan alpha = y/x = (g t)/(2 v_0) $ <eq:proj-alpha>

把 $tan theta = (g t)/(v_0)$ 与上式联立，消去 $(g t)/(v_0)$，立刻得到平抛运动最重要的角度关系：

$ tan theta = 2 tan alpha $ <eq:proj-tan>

= 两个常考推论

- *推论一*：速度 $vec(v)$ 的反向延长线，恰好交水平位移 $x$ 于中点。由 $tan theta = 2 tan alpha$ 与几何关系立即可得——小题里画个图就能"秒杀"。
- *推论二*：落地时间只由下落高度决定，与初速度无关。从高度 $h$ 平抛，落地时间与水平射程分别为：

$ t = sqrt((2 h)/g), quad x_"最大" = v_0 sqrt((2 h)/g) $ <eq:proj-range>

也就是说：在同一高度、同一 $v_0$ 平抛和自由落体的两个小球，会*同时*着地（竖直分运动相同），只是平抛的那颗飞得更远。

> 例题：从高 $h = 20 "m"$ 的平台以 $v_0 = 10 "m/s"$ 水平抛出一个小球（取 $g = 10 "m/s"^2$），求落地时间与落地速度。
>
> 解：落地时间只看竖直方向——自由落体 $t = sqrt((2 h)/g) = sqrt(40/10) = 2 "s"$。此时 $v_y = g t = 20 "m/s"$，故 $v = sqrt(v_0^2 + v_y^2) = sqrt(100 + 400) approx 22.4 "m/s"$；速度偏角 $tan theta = v_y/v_0 = 2$，约 $63.4°$。

= 小结

把曲线运动拆成两个直线运动，是贯穿抛体问题的总纲：

#table(
  columns: (auto, 1fr, 1fr, 1fr),
  inset: 6pt,
  [*分运动*], [*性质*], [*速度*], [*位移*],
  [水平], [匀速直线], [$v_x = v_0$], [$x = v_0 t$],
  [竖直], [自由落体], [$v_y = g t$], [$y = 1/2 g t^2$],
)

- 合速度沿切线：$v = sqrt(v_0^2 + (g t)^2)$；
- 角度关系：$tan theta = 2 tan alpha$（$theta$ 速度偏角、$alpha$ 位移偏角）；
- 落地时间 $t = sqrt((2 h)/g)$ 只取决于高度，射程 $x = v_0 sqrt((2 h)/g)$。

> 易错点：① 平抛时间由*高度*决定，与 $v_0$ 无关，别一看到"飞得远"就以为时间长；② 算偏角时先分清是速度偏角 $theta$ 还是位移偏角 $alpha$，两者差着 2 倍关系；③ 速度方向是轨迹的*切线*，不是位移方向。
