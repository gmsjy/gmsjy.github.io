#let title = "磁场：看不见却处处有力的场"
#let date = "2026-09-04"
#let tags = ("物理", "电磁学", "磁场", "高中")
#let series = "高中物理"
#let series_weight = 12
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 磁场与磁感应强度

*磁感应强度* $B$ 描述磁场强弱，单位特斯拉 $T$。通电导线在磁场中受力 $F = B I L$（$B$ 与 $I$ 垂直），由此定义：

$ B = F/(I L) $ <eq:magdef>

磁感线是描述磁场的假想曲线：*外部从 N 极指向 S 极*，内部从 S 极回到 N 极，是闭合曲线。疏密表示强弱。

> 易错点：$B$ 是磁场的固有属性，与放不放导线、通不通电无关（就像电场强度 $E$ 与放不放试探电荷无关）。$B = F/(I L)$ 是*定义式*不是*决定式*。

= 安培力：磁场对电流的作用力

通电导线在磁场中受*安培力*：

$ F = B I L sin theta $ <eq:ampere>

$theta$ 是 $B$ 与 $I$ 的夹角（垂直时 $F = B I L$）。方向用*左手定则*：伸开左手，让磁感线穿过掌心，四指指向电流方向，大拇指所指即安培力方向。

> 安培力方向总*垂直于* $B$ 与 $I$ 所在的平面（$F$ 垂直 $B$ 且垂直 $I$）。判断时"掌心迎磁感线、四指顺电流、拇指指方向"——左手定则管"力"，别和右手定则（管"电流/磁场"）混用。

= 洛伦兹力：磁场对运动电荷的作用力

运动电荷在磁场中受*洛伦兹力*：

$ f = q v B sin theta $ <eq:lorentz>

方向同样用*左手定则*（四指指*正电荷运动方向*，负电荷相反）。洛伦兹力的关键性质：*始终与速度垂直，永不做功*——它只改变速度方向，不改变速度大小。

> 三个特殊情形：① $v parallel B$（$theta = 0$ 或 $180 degree$）：不受力，做匀速直线运动；② $v bot B$：做*匀速圆周运动*；③ $v$ 与 $B$ 有夹角：做等距螺旋运动（速度分解为平行与垂直分量）。

= 带电粒子在匀强磁场中的圆周运动

洛伦兹力提供向心力 $q v B = m v^2 / r$，得：

#fig(cetz.canvas(length: 0.9cm, {
  // 匀强磁场中的圆周运动：洛伦兹力指向圆心
  circle((3.5, 2.0), radius: 1.5, stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  circle((3.5, 3.5), radius: 0.08, fill: rgb("#d64541"), stroke: none)
  line((3.65, 3.5), (4.85, 3.5), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  line((3.5, 3.35), (3.5, 2.2), mark: (end: ">"), stroke: (paint: rgb("#3c8a4d"), thickness: 1.3pt))
  content((5.05, 3.55), $ v $, size: 9pt)
  content((3.8, 2.7), $ F $, size: 9pt)
  content((5.3, 2.35), "洛伦兹力指向圆心", size: 8pt)
  for p in ((1.2, 1.0), (2.3, 0.35), (4.7, 0.35), (5.8, 1.0), (1.2, 3.5), (5.8, 3.5)) {
    content(p, $ times $, size: 10pt)
  }
  content((3.5, -0.5), "× 表示磁场 B 垂直纸面向里", size: 8pt)
}))

$ r = (m v)/(q B), quad T = (2 pi m)/(q B) $ <eq:circle>

> 惊人结论：*周期 $T$ 与速度 $v$ 无关*！$v$ 越大半径越大，但转一圈的时间不变。这是回旋加速器能反复加速粒子的理论基础。

== 找圆心定半径的几何套路

1. 洛伦兹力方向即圆心方向（垂直速度向内）；
2. 由几何关系定圆心（两速度垂线交点、或弦的中垂线与速度垂线交点）；
3. 用三角函数或勾股定理求半径，代入 @eq:circle。

= 两大应用：质谱仪与回旋加速器

#table(
  columns: (auto, 1fr),
  inset: 6pt,
  [*装置*], [*原理与结论*],
  [质谱仪], [速度选择器 + 偏转磁场。$q v B = m v^2 / r$，测 $r$ 反推荷质比 $q/m = v/(B r)$，分离不同质量同位素],
  [回旋加速器], [交变电场加速 + 磁场回旋。粒子最大动能 $E_(k,max) = (q^2 B^2 R^2)/(2 m)$（$R$ 为最大半径），与加速电压无关],
  [速度选择器], [$E$ 与 $B$ 正交，$q E = q v B$ 时粒子直线通过，$v = E/B$ 与粒子电荷、质量无关],
)

> 速度选择器只能选"速度"，不能选"电荷/质量"——因为 $q E = q v B$ 中 $q$ 约掉了。回旋加速器"最大动能由 $R$ 决定、交变电压只决定加速次数"是高频考点。
