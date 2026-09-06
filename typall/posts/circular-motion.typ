#let title = "匀速圆周运动：方向一直在变的匀速率运动"
#let date = "2026-09-03"
#let tags = ("物理", "力学", "圆周运动", "高中")
#let series = "高中物理"
#let series_weight = 5
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

直线运动和抛体运动解决的都是"走直线或走抛物线"的问题，速度方向都不绕圈。可生活里还有一种常见的运动——钟表指针的针尖、自行车轮的辐条、游乐场转盘上的座位——它们沿着*圆周*走，速度大小可以不变，但_方向每时每刻都在变_。这一篇讲清"匀速圆周运动"为什么明明叫"匀速"，却时刻有加速度。

= 匀速圆周运动：匀"速率"，变"方向"

== 线速度与角速度

物体沿圆周运动，弧长与时间的比值叫*线速度*，记作 $v$。匀速圆周运动的线速度*大小*不变，但方向沿圆周*切线*且不断改变。描述转动快慢还常用*角速度* $omega$，即单位时间转过的圆心角：

$ omega = v / r $ <eq:om>

线速度、角速度与周期的关系，是把弧长和圆心角分别除以时间得到的：

$ v = omega r, quad T = (2 pi) / omega $ <eq:vt>

> 记忆点：线速度负责"跑得多快"，角速度负责"转得多快"。半径越大，同样的角速度对应越大的线速度——像地球自转，赤道上的线速度远大于高纬度。

== 周期与转速

物体转一圈所需时间叫*周期* $T$，单位时间转的圈数叫*频率* $f$（也称转速）。三者满足：

$ f = 1 / T $ <eq:freq>

= 向心加速度：方向变化要付出"加速度"的代价

== 为什么匀速圆周也有加速度

加速度的定义是"速度的变化率"。速度是矢量，_方向变了也是变了_。匀速圆周运动速度方向时刻在变，所以*一定有加速度*。这个加速度始终指向圆心，叫*向心加速度*，记作 $a_n$：

#fig(cetz.canvas({
  // 一个逆时针转动的圆 + 某点的速度方向(切线)与向心加速度(指向圆心)
  let R = 4.5
  circle((0, 0), radius: R, stroke: rgb(0, 0, 0, 30%))
  circle((0, 0), radius: 0.08, fill: rgb("#555"), stroke: none)
  // 质点 P 在右上角（约 40°）
  let ang = 40 * (3.14159265358979 / 180.0)
  let px = R * calc.cos(ang)
  let py = R * calc.sin(ang)
  circle((px, py), radius: 0.12, fill: black, stroke: none)
  content((px + 0.4, py - 0.25), $ P $, anchor: "west")
  // 向心加速度 a_n：从 P 指向圆心
  let cx = 0.0
  let cy = 0.0
  let dx = cx - px
  let dy = cy - py
  let L = calc.sqrt(dx * dx + dy * dy)
  line((px, py), (px + dx / L * (R - 0.6), py + dy / L * (R - 0.6)), stroke: rgb("#d64541"), mark: (end: ">"))
  content((px + dx / L * (R / 2), py + dy / L * (R / 2) - 0.3), $ vec(a)_n $, anchor: "west")
  // 圆心 O
  content((-0.5, 0.25), $ O $, anchor: "west")
})) 

看红色箭头：$vec(a)_n$ 从质点 $P$ 指向圆心 $O$，与速度 $vec(v)$（沿切线）垂直。向心加速度_只改变速度方向，不改变速度大小_——这正是它叫"向心"而不叫"加速(增大速率)"的原因。

== 向心加速度的大小

由速度的矢量三角形可得，匀速圆周运动的向心加速度大小为：

$ a_n = v^2 / r = omega^2 r $ <eq:acc>

> 两种写法等价的桥梁是 $omega = v/r$。注意：$a_n = v^2/r$ 里 $v$ 是线速度、$r$ 是半径；不要把它误当一般的牛顿第二定律式用。

= 向心力：是什么把物体"拉"在圆周上

== 向心力不是独立的新力

物体能沿圆周而不沿切线飞出，说明有一个*指向圆心*的合力在起作用，这个合外力的向心分量叫*向心力* $F_n$。它_不是某种新种类的力_，而是"指向圆心的合外力"的别称——重力、弹力、摩擦力、电场力……任何力只要指向圆心，都可充当向心力。由牛顿第二定律：

$ F_n = m a_n = m v^2 / r = m omega^2 r $ <eq:cent>

== 常见情境里的向心力来源

- *汽车过拱桥最高点*：重力 $m vec(g)$ 与支持力 $vec(N)$ 的合力提供向心力，此时 $m g - N = m v^2/r$；
- *圆锥摆*：绳的拉力与重力的合力指向圆心（而非沿绳），提供向心力；
- *水平转盘上的物块*：静摩擦力指向圆心，提供向心力。

#fig(cetz.canvas({
  // 圆锥摆：绳绕竖直线转出圆锥面
  line((0, 0), (0, -5.5), stroke: rgb(0, 0, 0, 30%)) // 竖直悬线位置
  let L = 5.5
  let half = 32 * (3.14159265358979 / 180.0)
  let bx = L * calc.sin(half)
  let by = -L * calc.cos(half)
  // 绳：从悬点到摆球
  line((0, 0), (bx, by), stroke: black, mark: (start: ">"))
  circle((bx, by), radius: 0.4, fill: rgb("#e8e8e8"), stroke: black)
  content((bx + 0.3, by - 0.3), $ m $, anchor: "west")
  // 悬点
  circle((0, 0), radius: 0.1, fill: black, stroke: none)
  content((0.3, 0.2), $ A $, anchor: "west")
  // 摆角标注（示意）
  content((1.2, -0.7), $ theta $, anchor: "west")
}))

> 上图的圆锥摆里，摆球受重力与绳拉力，二者合力水平指向圆心——所以摆球在水平面内做匀速圆周运动，_绳的拉力并不指向圆心_（这是初学者最容易画错的地方）。

= 变速圆周运动（简介）

若速率也在变（如竖直平面内过最高点的小球），物体所受合外力不再恒指向圆心。此时可把合力分解为*切向分量*（改变速率）与*法向分量*（改变方向，即向心力）。竖直面内绳拉小球过最高点的临界条件是：

$ m g = m v^2 / r, quad v_"临界" = sqrt(g r) $ <eq:crit>

绳子恰好能提供的向心力最小为零（松弛临界），此时重力恰好充当全部向心力。

= 小结

- 匀速圆周运动 = _速率不变、方向时刻变_，因而*必有向心加速度*；
- 线量 $v$、角量 $omega$、周期 $T$ 靠 $v = omega r$ 与 $T = (2pi)/omega$ 连接；
- 向心加速度 $a_n = v^2/r = omega^2 r$，方向始终指向圆心，只改变方向不改变大小；
- 向心力是"指向圆心的合外力"，不是新力，$F_n = m v^2/r$；
- 变速圆周运动用切向、法向两个分量分开处理。
