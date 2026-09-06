#let title = "电磁感应：变化的磁场生出电流"
#let date = "2026-09-04"
#let tags = ("物理", "电磁学", "电磁感应", "高中")
#let series = "高中物理"
#let series_weight = 13
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 磁通量：穿过面积的"磁力线流量"

*磁通量* $Phi = B S cos theta$，其中 $theta$ 是磁感应强度方向与平面法线方向的夹角（$B$ 垂直于平面时 $Phi = B S$）。单位是韦伯 $W b$。

产生感应电流的*条件*是"穿过闭合回路的磁通量发生变化"。抓关键词"*变化*"——磁通量大不一定有感应电流，*变化*才有。导体棒切割磁感线、磁铁插入拔出线圈、回路面积变化，都是让 $Phi$ 变。

> 易错点：$Phi$、$Delta Phi$、$(Delta Phi)/(Delta t)$ 三个量含义不同。$Phi$ 大，$(Delta Phi)/(Delta t)$ 可能为零（磁通量恒定时无感应电动势）。感应电动势的大小由*变化率* $(Delta Phi)/(Delta t)$ 决定，不是由 $Phi$ 决定。

= 法拉第电磁感应定律

感应电动势的大小正比于磁通量的变化率——

#fig(cetz.canvas(length: 0.9cm, {
  // 导轨 + 导体棒：棒右移切割磁感线，回路出现感应电流
  line((0.4, 0.7), (6.6, 0.7), stroke: (paint: black, thickness: 1.2pt))
  line((0.4, 2.5), (6.6, 2.5), stroke: (paint: black, thickness: 1.2pt))
  line((5.2, 0.4), (5.2, 2.8), stroke: (paint: rgb("#d64541"), thickness: 2pt))
  line((5.55, 1.6), (6.5, 1.6), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.2pt))
  content((6.6, 1.9), $ v $, size: 9pt)
  for p in ((1.3, 1.6), (2.5, 1.6), (3.7, 1.6)) {
    content(p, $ times $, size: 10pt)
  }
  line((4.4, 2.5), (3.2, 2.5), mark: (end: ">"), stroke: (paint: rgb("#0b6fc0"), thickness: 1.3pt))
  line((1.4, 0.7), (2.6, 0.7), mark: (end: ">"), stroke: (paint: rgb("#0b6fc0"), thickness: 1.3pt))
  content((3.8, 2.85), "感应电流 I", size: 8pt)
  content((3.5, -0.5), "× 为向里的磁场：棒右移使磁通量增大，感应电流反抗这一增大", size: 8pt)
}))

$ E = n (Delta Phi)/(Delta t) $ <eq:faraday>

其中 $n$ 为线圈匝数。若磁通量变化由面积变化引起（导体棒切割），则：

$ E = B L v $ <eq:motional>

> 记忆：@eq:faraday 是"变化的磁通量"生电（一般情况），@eq:motional 是"导体切割磁感线"生电（$B$、$L$、$v$ 三者互相垂直时的特例，若速度与磁场有夹角要乘 $sin$）。

= 楞次定律：感应电流的方向总在"反抗"

*楞次定律*：感应电流产生的磁场，总是*阻碍*引起感应电流的磁通量的变化。口诀：

- *增反减同*：原磁通量增加时，感应电流的磁场与原磁场方向相反；减少时方向相同；
- *来拒去留*：磁铁靠近线圈时被"排斥"，远离时被"吸引"——感应效果总是反抗相对运动；
- *增缩减扩*：磁通量增加时回路有"收缩"趋势，减少时有"扩张"趋势。

> 判断感应电流方向的标准四步：① 看原磁场方向；② 看磁通量是增是减；③ 据"增反减同"定感应电流的磁场方向；④ 右手螺旋定则定感应电流方向。*先定"阻碍"再定"方向"，顺序不能乱。*

= 自感现象

线圈中电流变化时，线圈自身产生感应电动势阻碍电流变化——这就是*自感*。通电自感（电流"滞后"增大）、断电自感（灯泡"延迟"熄灭、可能闪亮一下）都是自感的表现。自感电动势：

$ E = L (Delta i)/(Delta t) $ <eq:self>

> 断电自感"闪一下"的条件：原线圈电流 $i_"线"$ 大于灯泡原电流 $i_"灯"$（即 $i_"线" > i_"灯"$ 才闪）。判断依据是"断电瞬间自感电流从线圈原电流值开始衰减"，而非从灯泡原电流。

= 电磁感应的能量观

感应电流流过电路要做功、发热，能量来源是"外界克服安培力做的功"。一句话：

$ "外力克服安培力做的功" = "回路产生的焦耳热" $ <eq:energy>

> 综合题套路：导体棒在磁场中运动 → 产生感应电动势 → 感应电流 → 受安培力 → 用牛顿定律或能量守恒求速度、热量。*"安培力做功 = 电能变化"*是电磁感应与能量综合的桥梁。
