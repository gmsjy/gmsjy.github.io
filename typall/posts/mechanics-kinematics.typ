#let title = "匀变速直线运动：核心公式与图像"
#let date = "2026-09-02"
#let tags = ("物理", "力学", "高中")
#let series = "高中物理"
#let series_weight = 1
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 为什么要研究"匀变速"

自然界中最常见的变速运动，是加速度 $a$ 保持不变的直线运动——自由落体、刹车滑行、匀加速启动都属于这一类。它的价值在于：*加速度恒定 → 公式封闭可解*，不需要微积分就能精确预言每一时刻的位置与速度。

三个核心概念先厘清：

- *位移* $x$：从初位置指向末位置的有向线段，是矢量；
- *速度* $v$：描述位置变化快慢，$v = (Delta x)/(Delta t)$；
- *加速度* $a$：描述速度变化快慢，$a = (Delta v)/(Delta t)$，单位 $m/s^2$。

= 三大基本公式

设初速度为 $v_0$、末速度为 $v$、加速度 $a$、位移 $x$、时间 $t$。加速度恒定意味着 $v$-$t$ 图像是一条直线，斜率即 $a$。

== 速度公式

由加速度定义直接积分（或理解为 $v$-$t$ 图斜率）：

$ v = v_0 + a t $ <eq:vt>

== 位移公式

$v$-$t$ 图像与时间轴围成的梯形面积等于位移：

$ x = v_0 t + (1)/(2) a t^2 $ <eq:xt>

== 速度—位移公式

@eq:vt 与 @eq:xt 中消去时间 $t$，得到不含时间的第三公式：

$ v^2 - v_0^2 = 2 a x $ <eq:vx>

> 记忆口诀：*知三求一*——五个物理量（$v_0, v, a, t, x$）中已知任意三个，必能由 @eq:vt、@eq:xt、@eq:vx 解出其余两个。审题第一步永远是"圈出已知量、标出待求量"。

= 重要推论（秒杀选择题）

== 平均速度等于中间时刻速度

匀变速直线运动中，某段时间内的平均速度 $bar(v)$，等于该段时间*中间时刻*的瞬时速度：

$ bar(v) = (v_0 + v)/(2) = v_(t/2) = x/t $ <eq:avg>

== 连续相等时间内的位移差恒定

取连续相等的时间间隔 $T$，相邻两段位移之差为一恒量——这是"逐差法"测加速度的理论依据：

$ Delta x = x_2 - x_1 = x_3 - x_2 = ... = a T^2 $ <eq:dx>

#table(
  columns: (auto, 1fr, 1fr),
  inset: 6pt,
  [*推论*], [*公式*], [*适用场景*],
  [平均速度], [$bar(v) = (v_0 + v)/2$], [已知 $v_0$、$v$ 求位移],
  [中间时刻速度], [$v_(t/2) = bar(v)$], [纸带打点计时器实验],
  [位移差恒定], [$Delta x = a T^2$], [逐差法求加速度],
  [初速度为零的比例], [$x_1 : x_2 : x_3 = 1 : 3 : 5$], [第 $1,2,3$ 个 $T$ 内位移比],
)

= 两类经典运动

== 自由落体

初速度为 0、加速度为 $g approx 9.8 m/s^2$ 的匀加速直线运动。@eq:xt 与 @eq:vx 退化为：

$ h = (1)/(2) g t^2, quad v^2 = 2 g h $ <eq:fall>

== 竖直上抛

初速度竖直向上。上升段是匀减速、下降段是自由落体，*全程对称*——可用全程法直接套三大公式（取向上为正，$a = -g$）。往返时间：

$ t_("总") = (2 v_0)/g $

> 易错点：竖直上抛到最高点速度为零但*加速度不为零*（仍为 $g$）。"速度为零"与"加速度为零"是两回事，受力不变加速度就不变。

= 图像法速览

#fig(cetz.canvas(length: 0.9cm, {
  // v-t 图：匀加速 → 匀速 → 匀减速，面积即位移
  line((-0.2, 0), (7, 0), mark: (end: ">"), stroke: black)
  line((0, -0.2), (0, 3.4), mark: (end: ">"), stroke: black)
  content((6.85, 0.4), $ t $, size: 9pt)
  content((0.4, 3.25), $ v $, size: 9pt)
  line((0.5, 0.3), (2, 2.8), stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  line((2, 2.8), (4.8, 2.8), stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  line((4.8, 2.8), (6.3, 0.3), stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  line((2, 2.8), (2, 0), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  line((4.8, 2.8), (4.8, 0), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  content((1.0, 2.1), "匀加速", size: 8pt)
  content((3.4, 3.15), "匀速", size: 8pt)
  content((5.55, 2.1), "匀减速", size: 8pt)
  content((3.5, -0.7), "图线与 t 轴围成的面积 = 位移", size: 8pt)
}))

#table(
  columns: (auto, 1fr, 1fr),
  inset: 6pt,
  [*图像*], [*斜率含义*], [*面积含义*],
  [$x$-$t$ 图], [速度 $v$], [无],
  [$v$-$t$ 图], [加速度 $a$], [位移 $x$（正负代数和）],
  [$a$-$t$ 图], [无], [速度变化量 $Delta v$],
)

> 口诀：*"斜率看纵比横，面积看横积纵"*。$v$-$t$ 图是最重要的图像——一条直线就能读出全部运动信息。
