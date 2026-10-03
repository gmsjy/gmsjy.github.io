// s2-fig-06b-focus.typ — 抛物线的定义：到焦点 = 到准线
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  axes(-3.9, 4.4, -2.9, 3.9, x-step: 1, y-step: 1)
  conic(kind: "parabola", center: (0, 0), p: 1.0, ymax: 3.4, stroke: 1.4pt + math-blue)
  // 焦点 F(1, 0) 与准线 x = -1
  circle((1, 0), radius: 0.1, fill: warn-red, stroke: none)
  content((1.0, 0.42), text(size: 8.5pt, fill: warn-red, weight: "bold", [焦点 $F(1, 0)$]))
  line((-1, -2.6), (-1, 3.6), stroke: 1.2pt + phys-orange)
  content((-1, 3.9), text(size: 8.5pt, fill: phys-orange, weight: "bold", [准线 $x = -1$]))
  // 曲线上一点 P 及两组等长线段
  let py = 2.3
  let px = py * py / 2
  circle((px, py), radius: 0.08, fill: ink, stroke: none)
  content((px + 0.15, py + 0.25), text(size: 8.5pt, fill: ink, weight: "bold", [$P$]))
  line((px, py), (1, 0), stroke: (paint: warn-red, thickness: 1pt))
  line((px, py), (-1, py), stroke: (paint: warn-red, thickness: 1pt))
  content((px / 2 + 0.5, py / 2 + 0.15), text(size: 8pt, fill: warn-red, [$d$]))
  content((px / 2 - 0.5, py + 0.18), text(size: 8pt, fill: warn-red, [$d$]))
  content((0.2, -2.5), text(size: 8.5pt, fill: ink, align(center, [红色两条线段永远等长：#linebreak()这就是抛物线的定义本身])))
})
