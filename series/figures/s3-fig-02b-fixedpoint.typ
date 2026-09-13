// s3-fig-02b-fixedpoint.typ — 定点定值：直线束扫过定点
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.4, 7.4, -0.4, 5.4, x-step: 1, y-step: 1, tick-labels: false)
  conic(kind: "ellipse", center: (3.6, 2.6), a: 2.4, b: 1.7, stroke: 1.3pt + math-blue)
  // 定点
  circle((3.6, 4.3), radius: 0.1, fill: warn-red, stroke: none)
  content((3.9, 4.45), text(size: 8.5pt, fill: warn-red, weight: "bold", [定点 $P$]))
  // 直线束
  for dy in (-1.5, -0.75, 0.75, 1.5) {
    line((3.6, 4.3), (3.6 + dy, 0.2), stroke: (paint: gray-line, thickness: 0.7pt))
  }
  line((3.6, 4.3), (1.6, 0.2), stroke: 1.1pt + phys-orange)
  line((3.6, 4.3), (5.6, 0.2), stroke: 1.1pt + phys-orange)
  content((1.15, 4.9), text(size: 8.5pt, fill: ink, align(center, [过定点的直线束，#linebreak()与曲线交于 $A$、$B$])))
  content((3.6, -0.15), text(size: 8.5pt, fill: warn-red, weight: "bold", align(center, [问：$k_("OA") dot k_("OB")$、$x_1 x_2 + y_1 y_2$ 这类「积和式」是否与直线无关？#linebreak()——这就是「定值」的判定方式])))
})
