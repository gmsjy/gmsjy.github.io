// s1-fig-13b-vt.typ — 从纸带到 v-t 图：逐段速度 + 拟合直线
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.4, 4.9, -0.4, 3.9, x-step: 1, y-step: 1, x-label: $t/"s"$, y-label: $v/("m/s")$)
  // 各段速度点
  let pts = ((0.4, 0.4), (1.0, 0.9), (1.9, 1.6), (2.9, 2.4), (3.9, 3.3))
  for p in pts { circle(p, radius: 0.07, fill: warn-red, stroke: none) }
  // 拟合直线
  line((0, 0.05), (4.4, 3.7), stroke: 1.2pt + math-blue)
  content((3.3, 2.1), text(size: 8.5pt, fill: math-blue, weight: "bold", [拟合直线]))
  content((1.3, 3.4), text(size: 8.5pt, fill: gray-line, align(center, [点没落在一条线上？#linebreak()画一条「两侧点数差不多」的直线])))
  content((4.5, 0.6), text(size: 8.5pt, fill: ink, align(center, [斜率 =#linebreak()加速度 $a$])), anchor: "west")
})
