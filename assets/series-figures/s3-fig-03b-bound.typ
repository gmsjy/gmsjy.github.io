// s3-fig-03b-bound.typ — 放缩与夹逼：给求和套上下界
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  axes(-0.3, 5.6, -0.3, 4.4, x-step: 1, y-step: 1, tick-labels: false)
  // 上界曲线 1/x 下界曲线 1/(x+1) 之间的柱子
  for i in range(5) {
    let x = i
    rect((x + 0.55, 1 / (x + 2)), (x + 0.95, 1 / (x + 1)), fill: soft-orange, stroke: (paint: phys-orange, thickness: 0.7pt))
  }
  curve(x => 1 / (x + 0.75), 0.45, 5.3, stroke: 1.2pt + math-blue)
  content((3.1, 3.3), text(size: 8.5pt, fill: ink, weight: "bold", [调和级数与积分之间的夹逼]))
  content((2.8, -0.1), text(size: 8.5pt, fill: gray-line, align(center, [数列求和证不等式的图形本质：#linebreak()把每一项变成一条矩形，再跟已知曲线的面积比大小])))
})
