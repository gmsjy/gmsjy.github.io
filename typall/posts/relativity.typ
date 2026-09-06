#let title = "狭义相对论初步：当速度逼近光速"
#let date = "2026-09-05"
#let tags = ("物理", "近代物理", "相对论", "高中")
#let series = "高中物理"
#let series_weight = 20
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 两条基本假设

1905 年，爱因斯坦从两个看似无害的假设出发，重建了整个时空观：

+ *相对性原理*：物理规律在一切惯性参考系中形式相同——没有一个惯性系是"特殊的"，你在匀速行驶的火车里做力学实验，结果与地面上没有任何区别。
+ *光速不变原理*：真空中的光速对任何惯性系都是同一个值 $c$，与光源和观察者的运动无关。

第二条假设是"离经叛道"的：按照经典的速度叠加，你以 $0.9 c$ 追一束光，光相对你应当只有 $0.1 c$。但迈克耳孙-莫雷实验等大量事实说：不，它仍然是 $c$。要让"光速对谁都一样"成立，就必须让"时间"和"长度"不再是绝对的。

= 时间膨胀：运动的钟走得慢

狭义相对论最著名的推论：*运动的时钟走得慢*。若一个参考系相对观察者以速度 $v$ 运动，则其中发生的过程在观察者看来变慢为原来的 $gamma$ 倍：

$ Delta t = gamma Delta tau, quad gamma = 1 / sqrt(1 - v^2 / c^2) $ <eq:gamma>

$Delta tau$ 是与钟*相对静止*的观察者测得的时间（固有时）。$gamma$ 叫*洛伦兹因子*，速度越大它膨胀得越猛——$v = 0.5 c$ 时 $gamma approx 1.15$，$v = 0.9 c$ 时 $gamma approx 2.29$，而 $0.99 c$ 时高达 $approx 7.1$：

#fig(cetz.canvas(length: 0.85cm, {
  // 横轴 v/c，纵轴 γ（截断到 8）
  line((-0.3, 0), (6.4, 0), mark: (end: ">"), stroke: black)
  line((0, -0.3), (0, 3.6), mark: (end: ">"), stroke: black)
  content((6.6, 0), $ v\/c $, size: 9pt)
  content((0.35, 3.7), $ gamma $, size: 9pt)
  content((0.35, -0.35), "0", size: 8pt)
  let pts = range(0, 100, step: 1).map(k => {
    let v = 0.995 * k / 100
    let g = 1 / calc.sqrt(1 - v * v)
    (v * 6, calc.min(g * 0.45, 3.35))
  })
  line(..pts, stroke: (paint: rgb("#d64541"), thickness: 1.4pt))
  // 标注点 0.5c / 0.9c / 0.99c
  for pair in ((0.5, 3.0), (0.9, 5.4), (0.99, 5.94)) {
    let v = pair.at(0)
    let x = pair.at(1)
    let g = 1 / calc.sqrt(1 - v * v) * 0.45
    circle((x, g), radius: 0.05, fill: rgb("#0b6fc0"), stroke: none)
    content((x + 0.15, g + 0.28), $ #v c arrow gamma approx #calc.round(g / 0.45, digits: 2) $, size: 8pt)
  }
  line((0, 0.45), (6, 0.45), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  content((5.6, 0.62), $ gamma = 1 $, size: 8pt)
}))

这不是数学游戏。宇宙射线中的 $mu$ 子在地面参考系里寿命只有约 $2.2 mu s$，即便以 $0.99 c$ 飞行也只能走 660 米，本该到不了地面——但正因为它"运动得快"，我们测得它的寿命膨胀了约 7 倍，足够穿越大气层抵达山脚。加速器里高速粒子的寿命、GPS 卫星时钟的校准，都精确验证了这一效应。

= 长度收缩：运动的尺子变短

与时间膨胀对偶的是空间效应：沿运动方向*运动的尺子变短*（长度收缩）：

$ L = L_0 / gamma = L_0 sqrt(1 - v^2 / c^2) $ <eq:length>

$L_0$ 是尺子*静止时*测得的长度（固有长度）。收缩只发生在运动方向上，垂直方向不受影响。

#fig(cetz.canvas(length: 0.9cm, {
  // 静止尺 L0 与运动尺 L = L0/γ（取 γ=2）对比
  content((0.3, 3.2), [静止（固有长度 $L_0$）], anchor: "west", size: 9pt)
  line((0.5, 2.6), (5.5, 2.6), stroke: (paint: black, thickness: 2pt))
  for x in range(0, 11) {
    line((0.5 + x * 0.5, 2.45), (0.5 + x * 0.5, 2.75), stroke: black)
  }
  line((0.5, 2.2), (5.5, 2.2), mark: (start: "|", end: "|"), stroke: black)
  content((3, 1.85), $ L_0 $, size: 9pt)
  content((0.3, 1.2), [运动（$gamma = 2$ 时 $L = L_0 \/ 2$）], anchor: "west", size: 9pt)
  line((0.5, 0.6), (3.0, 0.6), stroke: (paint: rgb("#d64541"), thickness: 2pt))
  for x in range(0, 11) {
    line((0.5 + x * 0.25, 0.45), (0.5 + x * 0.25, 0.75), stroke: rgb("#d64541"))
  }
  line((0.5, 0.2), (3.0, 0.2), mark: (start: "|", end: "|"), stroke: rgb("#d64541"))
  content((1.75, -0.15), $ L $, size: 9pt, stroke: rgb("#d64541"))
}))

注意"收缩"是*测量效应*而不是视觉压缩：它说的是"同时"测量两端得到的坐标差变小。谁在运动，谁的时间与长度就由谁来重新定义——没有谁是"真的"变短，两个参考系彼此看对方都是收缩的，这正是相对论名字的由来。

= 质能方程：$E = m c^2$

相对论还揭示：质量本身就是一种能量的度量。

$ E = m c^2 $ <eq:emc>

由于 $c^2$ 是一个巨大的数（$9 times 10^16 "m"^2\/"s"^2$），*很小*的质量对应*巨大*的能量：1 克物质完全转化为能量约 $9 times 10^13 "J"$，相当于 2 万吨 TNT。核武器、核电站、太阳燃烧的光辉，都是质量转化为能量的现实例证——这也正是后面原子核物理中"质量亏损"的根源。

高速运动的粒子还有"运动质量随速度增大"的效应，使得任何有质量的粒子都无法被加速到光速：越接近光速，$gamma$ 越大，加速所需能量趋于无穷。光速因此成为宇宙中的"限速牌"。

= 时空图一瞥

物理学家喜欢用*时空图*整理相对论：横轴是空间 $x$，纵轴是时间 $c t$。从原点出发的光以 $45 degree$ 的直线前进，构成一个"光锥"：

#fig(cetz.canvas(length: 0.95cm, {
  line((-2.4, 0), (2.4, 0), mark: (end: ">"), stroke: black)
  line((0, -2.4), (0, 2.4), mark: (end: ">"), stroke: black)
  content((2.6, 0), $ x $, size: 9pt)
  content((0.3, 2.5), $ c t $, size: 9pt)
  // 光锥：±45°
  line((-2.1, -2.1), (2.1, 2.1), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  line((-2.1, 2.1), (2.1, -2.1), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  content((1.75, 2.1), "光", size: 8pt, stroke: white)
  content((-1.9, 2.05), "光", size: 8pt, stroke: white)
  content((0, 1.5), "未来（类时）", size: 9pt)
  content((0, -1.6), "过去（类时）", size: 9pt)
  content((-1.7, 0.3), "类空", size: 9pt)
  content((1.7, 0.3), "类空", size: 9pt)
  content((1.1, -1.2), "“此时此地”的你", size: 8pt)
}))

任何有质量物体的世界线都必须待在光锥内部（速度小于 $c$）；"类空"区域的事件与你没有任何因果联系——除非光锥被打破，因果律本身就会崩塌。这张图是后面学习广义相对论、黑洞与宇宙学时最基础的画布。

= 小结

- 相对性原理 + 光速不变 → 时间与长度是*相对的*：$Delta t = gamma Delta tau$，$L = L_0 \/ gamma$；
- $gamma = 1 \/ sqrt(1 - v^2 \/ c^2)$，日常速度下 $gamma approx 1$，牛顿力学照常适用——相对论不是推翻牛顿，而是给出它的适用边界；
- $E = m c^2$：质量与能量可以相互转化，核能即来源于此；
- 光速是信息与物质的极限速度，时空图上的光锥划定了因果的边界。
