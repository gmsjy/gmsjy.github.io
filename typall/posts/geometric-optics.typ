#let title = "几何光学：光的直线、折射与全反射"
#let date = "2026-09-04"
#let tags = ("物理", "光学", "高中")
#let series = "高中物理"
#let series_weight = 16
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

光是什么？物理光学（下一课）会给出波动性的答案。但在很多场合，光表现得像一支支*笔直飞行的箭*——用「光线」+ 几何作图就能解释镜中成像、筷子变弯、光纤导光。这一课就是这个简化模型：*几何光学*。

= 光的直线传播与反射

光在*同种均匀介质*中沿直线传播——影子、日食、小孔成像都是证据。遇到界面时部分光折回原介质，即*反射*：

- 反射角*等于*入射角；
- 反射光线与入射光线分居法线两侧，三线共面；
- 光路*可逆*。

平面镜成像的特点一句话：*等大、正立（上下不倒）、左右反、虚像*，像与物关于镜面对称。

#fig(cetz.canvas(length: 0.85cm, {
  // 平面镜成像：物 S 与像 S' 关于镜面对称，两条反射线的反向延长线交于 S'
  line((0, 0), (8, 0), stroke: (paint: black, thickness: 1.5pt))
  // 镜面短斜线（表示镜背）
  for x in range(4, 76,step: 8) {
    line((x / 10.0, 0), (x / 10.0 - 0.22, -0.22), stroke: rgb("#888"))
  }
  // 物 S(2, 2)，像 S'(6, 2)（镜面在 x=4）
  circle((2, 2), radius: 0.09, fill: rgb("#d64541"), stroke: none)
  content((2, 2.35), $ S $)
  circle((6, 2), radius: 0.09, fill: none, stroke: rgb("#d64541"))
  content((6, 2.35), $ S' $)
  // 入射线 S -> 镜上点 P(4, 1)；反射线 P -> 眼睛 E(7, 1)
  line((2, 2), (4, 1), mark: (end: ">"), stroke: rgb("#d64541"))
  line((4, 1), (7, 1), mark: (end: ">"), stroke: rgb("#d64541"))
  // 反射线的反向延长线 P -> S'（虚线）
  line((4, 1), (6, 2), stroke: (paint: rgb("#d64541"), dash: "dashed"))
  // 法线
  line((4, 1), (4, 2.2), stroke: (paint: rgb("#0b6fc0"), dash: "dashed"))
  content((4.15, 2.1), "法线", anchor: "south-west", size: 7.5pt)
}))

进入眼睛的其实是反射线 $P E$，但视觉沿直线「倒推」，于是觉得光来自 $S'$——这就是*虚像*的含义：不是真实光线会聚，而是*反向延长线*会聚。

= 折射定律：光为什么「弯腰」

光穿过界面进入另一种介质时传播方向偏折，*斯涅尔定律*：

$ n_1 sin theta_1 = n_2 sin theta_2 $ <eq:snell>

折射率 $n = c / v$（$c$ 真空光速，$v$ 介质中光速），恒有 $n >= 1$。光从*光疏*介质（$n$ 小）进入*光密*介质（$n$ 大），折射角*小于*入射角——光线向法线「弯腰」；反向则远离法线。

#fig(cetz.canvas(length: 0.85cm, {
  // 折射光路：上空气 n1=1，下玻璃 n2=1.5
  rect((0, -2.4), (8, 0), fill: rgb("#e8eef4"), stroke: none)
  content((7.3, -2.1), $ n_2 $, size: 9pt)
  content((7.3, 1.7), $ n_1 $, size: 9pt)
  line((0, 0), (8, 0), stroke: black)
  // 界面点 (4, 0)；入射 55°，折射约 33°
  line((4 - 2.1, 2.1 * 1.43), (4, 0), mark: (end: ">"), stroke: rgb("#d64541"))
  line((4, 0), (4 + 1.6, -2.4), mark: (end: ">"), stroke: rgb("#d64541"))
  line((4, 0), (4, 2.2), stroke: (paint: rgb("#0b6fc0"), dash: "dashed"))
  line((4, 0), (4, -2.2), stroke: (paint: rgb("#0b6fc0"), dash: "dashed"))
  content((2.2, 2.2), $ theta_1 $, size: 9pt)
  content((4.55, -1.7), $ theta_2 $, size: 9pt)
}))

> 记忆点：$sin theta$ *小*的那一侧折射率*大*。判断介质疏密不用背方向——比一比两边角度即可。光路可逆在折射中同样成立。

== 折射的生活注脚

- *筷子变弯*：水上部分反射的光与水下部分折射的光来自不同方向；
- *池水「变浅」*：池底的光折射后远离法线进入眼睛，看到的虚像位置比实际浅——所以「看着能站的地方」未必真能站；
- *海市蜃楼*：空气密度不均匀导致连续折射。

= 全反射：光被「锁」在介质里

光从光密介质射向光疏介质（如水 → 空气）时，折射角大于入射角。入射角增大到某个值时折射角达到 $90 deg$——再增大，折射光*消失*，全部能量反射回来。这就是*全反射*，临界角 $C$ 满足：

$ sin C = 1 / n quad ("从介质射向真空"\/"空气") $ <eq:crit>

#fig(cetz.canvas(length: 0.8cm, {
  // 全反射三态：折射光线角度随入射角增大，最后消失
  line((0, 0), (9, 0), stroke: black)
  line((4.5, -3), (4.5, 1), stroke: (paint: rgb("#0b6fc0"), dash: "dashed"))
  content((4.5, 1.2), "法线", size: 7.5pt, stroke: none)
  // 三束入射（从左下往界面），折射越来越平，第三束只有反射
  line((2.2, -2.6), (4.5, 0), stroke: rgb("#d64541"))
  line((4.5, 0), (5.15, 2.7), mark: (end: ">"), stroke: rgb("#d64541"))
  line((3.4, -2.6), (4.5, 0), stroke: rgb("#3c8a4d"))
  line((4.5, 0), (6.4, 2.1), mark: (end: ">"), stroke: rgb("#3c8a4d"))
  line((4.2, -2.6), (4.5, 0), stroke: rgb("#7a4bbd"))
  line((4.5, 0), (2.9, 2.35), mark: (end: ">"), stroke: rgb("#7a4bbd"))
  content((5.3, 2.55), $ theta_1 < C $, anchor: "west", size: 8pt)
  content((6.6, 1.95), $ theta_1 > C: "全反射" $, anchor: "west", size: 8pt)
  content((7.6, -0.35), "光密介质", anchor: "east", size: 8pt)
}))

发生全反射必须*同时*满足两个条件：光从*光密*射向*光疏*介质，且入射角*大于等于*临界角。

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：水的折射率 $n = 4\/3$，求从水中射向空气的临界角。

$sin C = 1\/n = 3\/4 = 0.75$，故 $C approx 48.6 deg$。水下光源以大于 $48.6 deg$ 的入射角（相对法线）射向水面时，光完全反射回水中——从水下看，水面像一面发亮的「天花板」，只有正上方一个透光亮斑（斯涅尔窗）。
]

== 全反射的应用：光纤

光在细如发丝的玻璃纤维内连续发生全反射，被「锁」在里面沿弯曲路径传输——现代通信的骨架。内窥镜、光纤传感同理。光纤能导光的关键正是*内芯折射率大于外层包层*，保证全反射条件成立。

= 解题方法论

- *作图三要素*：先画法线，再定角度（反射等角、折射用 @eq:snell），最后检查「三线共面、分居两侧」。
- *几何光学题八成是三角形*：入射角/折射角往往要转化到直角三角形里算边长——画完整光路再动笔。
- *全反射题先验证条件*：先确认「光密 → 光疏」，再用 @eq:crit 求临界角与入射角比较；顺序反了容易硬套公式。
- *视深问题*：正上方观察时 $h' = h \/ n$（视深 = 实深 ÷ 折射率），只对近轴视线近似成立。

一句话收束：几何光学的全部内容就是一句话——*光走「最快」的路*（费马原理的影子）；反射与折射，只是这条路径在不同介质界面处的两种表现。
