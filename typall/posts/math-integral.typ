#let title = "定积分初步：把无限个小矩形加起来"
#let date = "2026-09-05"
#let tags = ("数学", "定积分", "微积分", "高中")
#let series = "高中数学"
#let series_weight = 18
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 一个"算不出面积"的问题

抛物线 $y = x^2$ 与 $x$ 轴、直线 $x = 1$ 围出一块"曲边梯形"。矩形的面积 = 底 × 高，可这里的高*处处在变*——小学到高中的所有面积公式在此集体失效。

牛顿与莱布尼茨的思路是化归：把曲的当成直的算，用*无限*逼近消灭误差。把区间 $[0, 1]$ 切成 $n$ 等份，每一小段上用矩形近似代替曲边：

#fig(cetz.canvas(length: 1.0cm, {
  // y = x² on [0,2]，8 矩形逼近
  line((-0.3, 0), (4.6, 0), mark: (end: ">"), stroke: black)
  line((0, -0.3), (0, 3.6), mark: (end: ">"), stroke: black)
  content((4.8, 0), $ x $, size: 9pt)
  content((0.3, 3.7), $ y $, size: 9pt)
  content((0.35, -0.35), "0", size: 8pt)
  content((4.0, 0.35), "2", size: 8pt)
  for i in range(8) {
    let x = i * 2 / 8
    let h = x * x * 0.8
    rect((x * 2, 0), (x * 2 + 0.5, h), stroke: rgb("#9aa2ad"), fill: rgb("#0b6fc0").lighten(85%))
  }
  let pts = range(0, 41, step: 1).map(k => {
    let x = 2 * k / 40
    (x * 2, x * x * 0.8)
  })
  line(..pts, stroke: (paint: rgb("#d64541"), thickness: 1.6pt))
  content((3.9, 3.5), $ y = x^2 $, size: 9pt, stroke: white)
  content((2.6, 1.0), "矩形和 ≈ 曲边梯形面积", size: 8pt)
}))

分割越细，矩形面积和与真实面积只差"一个可以任意小的量"。当 $n -> infinity$，这个和就*定义*了面积：

$ integral_0^1 x^2 dif x = lim_(n -> infinity) sum_(i=1)^n (i/n)^2 dot 1/n = lim_(n -> infinity) (n+1)(2n+1)/(6n^2) = 1/3 $ <eq:riemann>

这个极限就叫*定积分*。注意我们并没有"发明"什么新公式——只是把加法做到了无穷多份。极限思想又一次把"算不出来"变成了"逼近到精确"。

= 定积分的记号与含义

一般地，函数 $f(x)$ 在区间 $[a, b]$ 上的定积分写作

$ integral_a^b f(x) dif x $ <eq:def>

读作"$f(x)$ 从 $a$ 到 $b$ 的定积分"。几何上，$f(x) >= 0$ 时它就是曲线下方的面积；$f(x)$ 有正有负时，$x$ 轴上方的面积记正、下方记负。

微分与积分是同一枚硬币的两面：导数研究"变化率"，积分把变化率*累积*回来。速度对时间积分得位移，力对位移积分得功——积分就是"无限细分再求和"的通用语言。

= 牛顿–莱布尼茨公式：一步登天

按定义算极限太苦。牛顿与莱布尼茨发现了一条惊人捷径：若 $F'(x) = f(x)$（$F$ 是 $f$ 的一个原函数），则

$ integral_a^b f(x) dif x = F(b) - F(a) $ <eq:newton-leibniz>

积分问题瞬间退化成"找原函数"——而找原函数正是*求导的逆运算*，上一章的导数表倒着背即可。比如 $F(x) = x^3/3$ 满足 $F'(x) = x^2$，立刻得到 $integral_0^1 x^2 dif x = 1^3/3 - 0 = 1/3$，与黎曼和的极限殊途同归。

#fig(cetz.canvas(length: 1.0cm, {
  // v–t 图：面积 = 位移（先匀加速后匀速）
  line((-0.3, 0), (5.6, 0), mark: (end: ">"), stroke: black)
  line((0, -0.3), (0, 3.3), mark: (end: ">"), stroke: black)
  content((5.8, 0), $ t $, size: 9pt)
  content((0.35, 3.4), $ v $, size: 9pt)
  content((0.35, -0.35), "0", size: 8pt)
  content((1.9, -0.35), "1", size: 8pt)
  content((4.2, -0.35), "3", size: 8pt)
  line((0, 0), (2, 2.4), stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  line((2, 2.4), (4.4, 2.4), stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  line((2, 0), (2, 2.4), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  rect((0, 0), (2, 2.4), stroke: none, fill: rgb("#0b6fc0").lighten(88%))
  rect((2, 0), (4.4, 2.4), stroke: none, fill: rgb("#3c8a4d").lighten(85%))
  content((1.0, 1.0), "∫v dt", size: 9pt)
  content((3.2, 1.25), "∫v dt", size: 9pt)
  content((3.6, 2.85), "两块面积 = 总位移", size: 8.5pt)
}))

比如 $v = 2t$（$t$ 从 0 到 3，单位 m/s）：位移 $= integral_0^3 2t dif t = 9 "m"$，正是 $v$–$t$ 图线下的三角形面积 $1/2 times 3 times 6$。物理课的"面积法求位移"，本质就是一次定积分。

= 一点展望

高中阶段只需掌握：定积分的*思想*（分割—近似—求和—取极限）、简单幂函数与线性函数的积分、牛顿–莱布尼茨公式的使用。到了大学，这套语言会生长成整座微积分大厦——面积、体积、做功、概率、期望，凡"连续量的累积"，皆用积分书写。

= 小结

- 定积分 $integral_a^b f(x) dif x$ 是"分割、近似、求和、取极限"的产物，几何上是曲线下的（代数）面积；
- 牛顿–莱布尼茨公式 $integral_a^b f dif x = F(b) - F(a)$ 把积分化为找原函数，与导数互为逆运算；
- 积分是"变化率的累积"：速度累积成位移，力累积成功；
- 曲边面积、变力做功……凡是"不规则量的总和"，都是定积分的用武之地。
