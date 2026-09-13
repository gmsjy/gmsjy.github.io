// s3-fig-16b-param.typ — 参数化绘图：一个滑块换一族曲线
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  axes(-0.3, 6.4, -0.3, 4.4, x-step: 1, y-step: 1, x-label: $x$, y-label: $y$, tick-labels: false)
  // 曲线族 y = a x^2
  for (a, c) in ((0.2, gray-line), (0.45, math-blue), (0.75, phys-orange)) {
    curve(x => a * x * x, -2.4, 2.4, yclip: (0, 4.2), stroke: 1.2pt + c)
    content((2.45, 4.2 - 0.0), text(size: 7.8pt, fill: c, weight: "bold", align(right, [$a = #str(a)$])), anchor: "east")
    break
  }
  // 手动画三条标注
  curve(x => 0.2 * x * x, -2.4, 2.4, stroke: (paint: gray-line, thickness: 1pt))
  curve(x => 0.45 * x * x, -2.4, 2.4, yclip: (0, 4.2), stroke: (paint: math-blue, thickness: 1.1pt))
  curve(x => 0.75 * x * x, -2.4, 2.4, yclip: (0, 4.2), stroke: (paint: phys-orange, thickness: 1.2pt))
  content((0.35, 3.6), text(size: 8pt, fill: phys-orange, [大 $a$：更陡]))
  content((-2.3, 1.1), text(size: 8pt, fill: gray-line, [小 $a$：更平]), anchor: "west")
  content((3.2, 3.9), text(size: 8.5pt, fill: ink, align(center, [参数化交互讲义：#linebreak()「滑块」改一个数，全图重画，#linebreak()适合课堂投影现场演示])))
})
