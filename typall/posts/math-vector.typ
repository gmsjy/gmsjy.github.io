#let title = "平面向量：既有大小又有方向的数"
#let date = "2026-09-02"
#let tags = ("数学", "平面向量", "高中")
#let series = "高中数学"
#let series_weight = 7
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 平面向量的概念

*向量*是既有*大小*又有*方向*的量（本文用 $arrow(a)$ 表示向量 $a$）。与之相对，只有大小没有方向的量叫*标量*（如质量、温度）。位移、速度、力都是向量。

向量用带箭头的有向线段表示，线段的长度表示向量的大小，箭头的指向表示方向。向量的*模*（大小）记作 $abs(arrow(a))$ 或 $abs(a)$。几个特殊向量是概念题的常客：

- *零向量*：模为 $0$，记 $bold(0)$，方向任意；
- *单位向量*：模为 $1$ 的向量，与 $arrow(a)$ 同向的单位向量记 $arrow(a)/abs(arrow(a))$；
- *相等向量*：方向相同且模相等（与起点位置无关，可自由平移）；
- *相反向量*：与 $arrow(a)$ 方向相反、模相等的向量，记 $-arrow(a)$；
- *平行向量*（共线向量）：方向相同或相反的非零向量，规定零向量与任何向量平行。

> 易错点：① 向量不能比较大小，只有*模*能比较；② 相等向量必共线，共线向量不一定相等；③ 单位向量有无数个（方向各不相同），说"单位向量"必须指明方向；④ 零向量方向任意，是"与任何向量平行"这句话成立的前提。

= 向量的线性运算

== 加法与减法

*三角形法则*：$arrow(a) + arrow(b)$ 把 $arrow(b)$ 的起点放在 $arrow(a)$ 的终点，从 $arrow(a)$ 起点指向 $arrow(b)$ 终点。"首尾相连，连首尾"。

*平行四边形法则*：两个向量同起点作平行四边形，对角线就是和向量。三角形法则更通用，平行四边形法则只用于不共线的两个向量。

*减法*：$arrow(a) - arrow(b) = arrow(a) + (-arrow(b))$，作图口诀"*共起点、连终点、指向被减*"。

#fig(cetz.canvas(length: 0.9cm, {
  // 平行四边形法则：对角线是和，另一条对角线（方向）是差
  line((0.6, 0.6), (3.6, 1.2), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  line((0.6, 0.6), (1.4, 2.6), mark: (end: ">"), stroke: (paint: rgb("#0b6fc0"), thickness: 1.3pt))
  line((3.6, 1.2), (4.4, 3.2), stroke: rgb("#9aa2ad"))
  line((1.4, 2.6), (4.4, 3.2), stroke: rgb("#9aa2ad"))
  line((0.6, 0.6), (4.4, 3.2), mark: (end: ">"), stroke: (paint: rgb("#3c8a4d"), thickness: 1.3pt))
  line((1.4, 2.6), (3.6, 1.2), stroke: (paint: rgb("#7a4bbd"), dash: "dashed"))
  content((2.0, 0.65), $ arrow(a) $, size: 9pt)
  content((0.75, 1.85), $ arrow(b) $, size: 9pt)
  content((2.9, 2.3), $ arrow(a) + arrow(b) $, size: 9pt)
  content((3.35, 1.75), $ arrow(a) - arrow(b) $, size: 9pt)
  content((2.5, -0.1), "灰边补成平行四边形：绿对角线是和，紫对角线指向被减向量是差", size: 8pt)
}))

向量加法的模满足*三角不等式*：

$ abs(arrow(a)) - abs(arrow(b)) <= abs(arrow(a) + arrow(b)) <= abs(arrow(a)) + abs(arrow(b)) $ <eq:tri-ineq>

（等号在共线同向 / 反向时取到——判断"何时取等"是小题常考点。）

== 数乘向量与共线定理

实数 $lambda$ 与向量 $arrow(a)$ 的数乘 $lambda arrow(a)$：模变为 $abs(lambda) abs(arrow(a))$；$lambda > 0$ 同向、$lambda < 0$ 反向、$lambda = 0$ 得零向量。数乘满足结合律与分配律（与实数乘法形式一致，可放心"脱括号"）。

*共线定理*（向量平行与数乘的桥梁）：对非零向量 $arrow(b)$，$arrow(a) parallel arrow(b)$ 当且仅当存在唯一实数 $lambda$，使

$ arrow(a) = lambda arrow(b) $ <eq:collinear>

三点共线的判定：$A$、$B$、$C$ 共线 $<=>$ 存在 $t$ 使 $arrow(A C) = t arrow(A B)$（或 $arrow(O C) = (1-t) arrow(O A) + t arrow(O B)$）。后者是"定比分点"与向量中点的统一写法：$M$ 为 $A B$ 中点时 $arrow(O M) = 1/2 (arrow(O A) + arrow(O B))$。

= 平面向量的数量积

== 定义

两个非零向量 $arrow(a)$、$arrow(b)$ 的夹角为 $theta$（$0 <= theta <= pi$），数量积定义为：

$ arrow(a) dot arrow(b) = abs(arrow(a)) abs(arrow(b)) cos theta $ <eq:dot-def>

结果是一个*数*（标量），不是向量。$abs(arrow(a)) cos theta$ 叫 $arrow(a)$ 在 $arrow(b)$ 方向上的*投影*——"力做功 $W = F s cos theta$"就是数量积的物理原型。

数量积的核心结论要背熟：

- $arrow(a) dot arrow(a) = abs(arrow(a))^2$（求模的钥匙：$abs(arrow(a)) = sqrt(arrow(a) dot arrow(a))$）；
- $arrow(a) bot arrow(b) <=> arrow(a) dot arrow(b) = 0$（含零向量的约定情形）；
- $arrow(a) dot arrow(b) <= abs(arrow(a)) abs(arrow(b))$（柯西不等式的向量形式，等号当且仅当共线）。

运算律：交换律、数乘结合律、分配律都成立，但*消去律不成立*——$arrow(a) dot arrow(b) = arrow(a) dot arrow(c)$ 推不出 $arrow(b) = arrow(c)$（只能得到 $arrow(a) bot (arrow(b) - arrow(c))$）。数量积不满足结合律：$(arrow(a) dot arrow(b)) dot arrow(c)$ 无意义，因为点乘结果是数，不能再与向量点乘。

= 平面向量基本定理与坐标表示

== 基本定理

如果 $arrow(e)_1$、$arrow(e)_2$ 是同一平面内两个*不共线*向量，那么该平面内任一向量 $arrow(a)$ 都可以唯一表示为：

$ arrow(a) = lambda_1 arrow(e)_1 + lambda_2 arrow(e)_2 $ <eq:basic-theorem>

$arrow(e)_1$、$arrow(e)_2$ 叫*基底*，$lambda_1$、$lambda_2$ 是 $arrow(a)$ 在基底下的坐标。把基底取成互相垂直的单位向量 $arrow(i)$、$arrow(j)$（正交基底），就得到*直角坐标表示* $arrow(a) = (x, y)$——向量的坐标即终点坐标减起点坐标。

== 坐标运算总表

#table(
  columns: (auto, auto),
  inset: 6pt,
  [*运算*], [*坐标形式*（$arrow(a) = (x_1, y_1)$，$arrow(b) = (x_2, y_2)$）],
  [加法], [$arrow(a) + arrow(b) = (x_1 + x_2, y_1 + y_2)$],
  [减法], [$arrow(a) - arrow(b) = (x_1 - x_2, y_1 - y_2)$],
  [数乘], [$lambda arrow(a) = (lambda x_1, lambda y_1)$],
  [模], [$abs(arrow(a)) = sqrt(x_1^2 + y_1^2)$],
  [数量积], [$arrow(a) dot arrow(b) = x_1 x_2 + y_1 y_2$],
  [平行], [$arrow(a) parallel arrow(b) <=> x_1 y_2 - x_2 y_1 = 0$（交叉相乘相等）],
  [垂直], [$arrow(a) bot arrow(b) <=> x_1 x_2 + y_1 y_2 = 0$],
  [夹角], [$cos theta = (x_1 x_2 + y_1 y_2)/(sqrt(x_1^2 + y_1^2) sqrt(x_2^2 + y_2^2))$],
)

> 口诀：平行看"*叉积为零*"（$x_1 y_2 = x_2 y_1$），垂直看"*点积为零*"。别记反！两条式子长得像，但差一个"叉/点"。

坐标运算把几何问题变成纯代数问题：证明垂直就证点积为 $0$，求夹角就套余弦公式，求模就开根号——*建系是向量大题的第一动作*。

== 中点与距离

已知 $A(x_1, y_1)$、$B(x_2, y_2)$，中点 $M$ 坐标为 $(x_1 + x_2)/2, (y_1 + y_2)/2$，两点间距离 $abs(arrow(A B)) = sqrt((x_2 - x_1)^2 + (y_2 - y_1)^2)$——它就是模公式穿上坐标的外衣。

= 向量的应用：几何问题的代数化

- *证平行*：证对应向量共线（叉积为零）；
- *证垂直*：证对应向量点积为零；
- *证三点共线*：构造两个有公共起点的向量，证共线；
- *求夹角与最值*：夹角用余弦公式，最值常用 $abs(arrow(a))^2 = arrow(a) dot arrow(a)$ 把"模"化为"点积"再配方，或直接上三角不等式与柯西不等式。

> 例题感：$abs(arrow(a)) = abs(arrow(b)) = 1$，$arrow(a) bot arrow(b)$ 时求 $abs(arrow(a) + t arrow(b))$ 的最小值——两边平方得 $abs(arrow(a) + t arrow(b))^2 = 1 + t^2$，配方即得最小值 $1$（$t = 0$ 取到）。*平方化模、点积展开*是处理含参向量模的标准动作。
