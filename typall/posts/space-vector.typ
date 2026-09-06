#let title = "空间向量与立体几何：给空间装上坐标"
#let date = "2026-09-05"
#let tags = ("数学", "立体几何", "向量", "高中")
#let series = "高中数学"
#let series_weight = 12
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

上一课立体几何初步用「纯几何法」证平行与垂直——辅助线要靠灵感。这一课给空间装上直角坐标系，让向量把「找角、算距离」变成*机械化计算*：建系、求坐标、套公式，三步走完。

= 空间直角坐标系与空间向量

== 建系：把立体图形数字化

在空间取一点 $O$ 作原点，三条*两两垂直*的数轴 $x$、$y$、$z$ 构成空间直角坐标系。长方体与正方体是天然的坐标系：选一个顶点作原点，三条互相垂直的棱作坐标轴，所有顶点的坐标立刻写出。

#fig(cetz.canvas(length: 0.85cm, {
  // 单位正方体 + 坐标轴：O 为原点，三条棱为三轴
  // 底面 O(0,0) A(3,0) C(3,3) B(-1.2,0.9)（斜二测感觉的投影）
  let off = (1.5, 1.1) // 深度方向
  // 底面与顶面
  rect((0, 0), (3, 3), stroke: rgb("#9aa2ad"))
  rect((off.at(0), off.at(1)), (off.at(0) + 3, off.at(1) + 3), stroke: rgb("#c4cad2"))
  line((0, 0), (off.at(0), off.at(1)), stroke: rgb("#9aa2ad"))
  line((3, 0), (off.at(0) + 3, off.at(1)), stroke: rgb("#9aa2ad"))
  line((0, 3), (off.at(0), off.at(1) + 3), stroke: rgb("#9aa2ad"))
  line((3, 3), (off.at(0) + 3, off.at(1) + 3), stroke: rgb("#9aa2ad"))
  // 坐标轴（加粗）：x 沿棱、y 沿深度、z 竖直
  line((0, 0), (4.2, 0), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.4pt))
  line((0, 0), (2.7, 1.98), mark: (end: ">"), stroke: (paint: rgb("#0b6fc0"), thickness: 1.4pt))
  line((0, 0), (0, 4.2), mark: (end: ">"), stroke: (paint: rgb("#3c8a4d"), thickness: 1.4pt))
  content((4.35, 0), $ x $, size: 9pt)
  content((2.85, 2.15), $ y $, size: 9pt)
  content((0.3, 4.1), $ z $, size: 9pt)
  content((-0.25, -0.25), $ O $, size: 9pt)
  content((3.25, -0.3), $ A(3, 0, 0) $, size: 7.5pt)
  content((off.at(0) + 0.15, off.at(1) + 0.45), $ B(0, 3, 0) $, size: 7.5pt)
  content((0.35, 2.7), $ D(0, 0, 3) $, size: 7.5pt)
}))

坐标读法口诀：*沿 $x$ 走、沿 $y$ 走、再沿 $z$ 升*。向量 $vec(A B) = (a_1, a_2, a_3)$ 就是终点坐标减起点坐标——与平面向量完全一致，只是多了一个分量。

== 空间向量的运算表

设 $vec(a) = (a_1, a_2, a_3)$，$vec(b) = (b_1, b_2, b_3)$：

#table(
  columns: (auto, auto),
  inset: 6pt,
  stroke: 0.5pt + rgb("#c4cad2"),
  [*运算*], [*坐标公式*],
  $ vec(a) +- vec(b) $, $ (a_1 +- b_1, a_2 +- b_2, a_3 +- b_3) $,
  $ lambda vec(a) $, $ (lambda a_1, lambda a_2, lambda a_3) $,
  $ vec(a) dot vec(b) $, $ a_1 b_1 + a_2 b_2 + a_3 b_3 $,
  $ |vec(a)| $, $ sqrt(a_1^2 + a_2^2 + a_3^2) $,
  $ cos theta $, $ (vec(a) dot vec(b)) / (|vec(a)| |vec(b)|) $,
)

两个*向量级判据*（证明题直接用）：

- *平行*：$vec(a) = lambda vec(b)$，即对应分量成比例；
- *垂直*：$vec(a) dot vec(b) = 0$，即 $a_1 b_1 + a_2 b_2 + a_3 b_3 = 0$。

> 记忆点：数量积为 0 是空间中证*线线垂直*最快的路——两条相交或异面直线的方向向量一算便知，完全不需要找辅助线。

= 直线的方向向量与平面的法向量

== 两个「钥匙向量」

- *直线的方向向量*：与直线平行的非零向量。直线上两点的坐标差即为一个现成的方向向量；
- *平面的法向量*：与平面*垂直*的非零向量，记作 $vec(n) = (x, y, z)$。平面的平行、垂直、角度、距离，全都通过法向量转述。

== 求法向量的标准流程

设 $vec(n) dot vec(a) = 0$ 且 $vec(n) dot vec(b) = 0$（$vec(a)$、$vec(b)$ 是平面内两个不共线向量），联立后*任取一个非零分量*，解出其余分量。三步：

+ 写出平面内两个向量 $vec(a)$、$vec(b)$ 的坐标；
+ 列方程组 $cases(x_1 a_1 + x_2 a_2 + x_3 a_3 = 0, x_1 b_1 + x_2 b_2 + x_3 b_3 = 0,)$；
+ 赋值一个分量（如 $z = 1$），解出 $x$、$y$。

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：正方体 $"ABCD"-A_1 B_1 C_1 D_1$ 中，$A B = 2$，求平面 $A B_1 C$ 的一个法向量。

建系 $D(0,0,0)$、$A(2,0,0)$、$B(2,2,0)$、$C(0,2,0)$、$B_1(2,2,2)$。平面内取 $vec(A C) = (-2, 2, 0)$、$vec(A B_1) = (0, 2, 2)$。

设 $vec(n) = (x, y, z)$：$-2x + 2y = 0$，$2y + 2z = 0$。取 $z = 1$ 得 $y = -1$，$x = -1$。

$vec(n) = (-1, -1, 1)$（或任何非零倍数）。*法向量不唯一，成倍数都算对*。
]

= 三种角与一种距离：全部套公式

== 空间角对照表

设直线方向向量为 $vec(a)$、$vec(b)$，平面法向量为 $vec(n_1)$、$vec(n_2)$：

#table(
  columns: (auto, auto, auto),
  inset: 6pt,
  stroke: 0.5pt + rgb("#c4cad2"),
  [*对象*], [*公式*], [*范围与修正*],
  [异面直线所成角], $ cos theta = |vec(a) dot vec(b)| / (|vec(a)| |vec(b)|) $, [$theta in (0, pi\/2]$，取绝对值],
  [直线与平面所成角], $ sin theta = |vec(a) dot vec(n)| / (|vec(a)| |vec(n)|) $, [*用 $sin$*！结果取正],
  [二面角], $ cos theta = (vec(n_1) dot vec(n_2)) / (|vec(n_1)| |vec(n_2)|) $, [需结合图形判断同侧异侧，取 $theta$ 或 $pi - theta$],
)

最容易失分的是*线面角*：定义是直线与它在平面内的*射影*的夹角，恰好等于方向向量与法向量夹角的*余角*——所以公式里是 $sin theta$ 而不是 $cos theta$。二面角则*必须看图*：两个法向量同指向二面角内部时取补角，否则取本身。

== 点面距：一个投影公式

点 $P$ 到平面 $alpha$ 的距离，等于 $vec(PP_0)$（$P_0$ 为平面内*任意*一点）在法向量上的投影长度：

$ d = (|vec(PP_0) dot vec(n)|) / (|vec(n)|) $ <eq:dist>

公式的美妙之处：$P_0$ 取平面内*随便哪个顶点*都行——选坐标最简单的那个。

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：承上例的正方体，求点 $D$ 到平面 $A B_1 C$ 的距离。

平面 $A B_1 C$ 过 $A(2,0,0)$，取 $P_0 = A$，$vec(A D) = (-2, 0, 0)$（注意从平面内一点指向 $D$）。由上例 $vec(n) = (-1, -1, 1)$，$|vec(n)| = sqrt(3)$。

$d = (|(-2)(-1) + 0 + 0|) / sqrt(3) = 2 \/ sqrt(3) = (2 sqrt(3)) \/ 3$。

检验：正方体中心到该面的距离应更小，量级 $sqrt(3) approx 1.73$ 合理（$2 \/ sqrt(3) approx 1.15$）。
]

= 空间向量法 vs 几何法

- *先问自己：好不好建系？*长方体、正方体、有现成三条互相垂直棱的图形，直接建系走公式；圆柱圆锥组合体、垂直关系隐蔽的图形，几何法可能更快；
- *向量法不证存在，只算大小*：平行与垂直的*判定*（证明题）用向量判据 $vec(a) = lambda vec(b)$、$vec(a) dot vec(b) = 0$ 同样好写；
- *异面直线距离*（选学）：转化为点面距——过一条直线作平面平行于另一条直线；
- *动态问题*（选填压轴）：向量坐标让「动点」变成参数 $(t, t_0, 0)$，距离平方化为二次函数求最值。

= 解题方法论

- *建系三优先*：原点选在*对称中心或线段中点*（坐标最简）、坐标轴选*现成垂直棱*、先写出*全部顶点*坐标再动笔；
- *每个向量都回溯到坐标差*：$vec(A B) = B - A$，直接抄顶点坐标相减，不现场想象方向；
- *法向量必须回代检验*：算完拿 $vec(n) dot vec(a)$ 与 $vec(n) dot vec(b)$ 各验一次为零，一步出错全题报废；
- *角的范围先写后算*：线面角 $sin$、其余 $cos$，锐角钝角看图定——公式算完，回到图形里「长什么样」做最终裁决；
- *点面距选 $P_0$ 的自由*：在平面内挑坐标为零最多的点，投影公式分子立刻变简单。

一句话收束：立体几何初步回答「为什么平行、为什么垂直」，这一课回答「*等于多少*」——坐标与法向量把灵感问题变成了流水线问题，几何的直观配上代数的精度，空间从此*可算*。
