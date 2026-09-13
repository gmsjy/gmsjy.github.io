// s2-fig-01b-derivative-graph.typ — 原函数与导函数上下联动（f 与 f'）
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let f(x) = x * x * x - 3 * x
#let df(x) = 3 * x * x - 3

#let fig-a = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-2.6, 2.6, -3.5, 3.6, x-step: 1, y-step: 2)
  // 同步虚线：极值点横坐标
  line((-1, -3.5), (-1, 3.6), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  line((1, -3.5), (1, 3.6), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  curve(f, -2.35, 2.35, yclip: (-3.5, 3.6), stroke: 1.4pt + math-blue)
  circle((-1, 2), radius: 0.08, fill: ink, stroke: none)
  circle((1, -2), radius: 0.08, fill: ink, stroke: none)
  content((-0.75, 2.35), text(size: 8pt, fill: ink, [极大值 $2$]))
  content((0.75, -2.62), text(size: 8pt, fill: ink, [极小值 $-2$]))
  content((-2.45, 3.15), text(size: 8.5pt, fill: math-blue, [$f(x) = x^3 - 3x$]))
})

#let fig-b = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  // 导数符号着色（垫底）
  rect((-2.6, -4.3), (-1, 0), fill: soft-green, stroke: none)
  rect((-1, -4.3), (1, 0), fill: soft-red, stroke: none)
  rect((1, -4.3), (2.6, 0), fill: soft-green, stroke: none)
  axes(-2.6, 2.6, -4.3, 4.2, x-step: 1, y-step: 2)
  line((-1, -4.3), (-1, 4.2), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  line((1, -4.3), (1, 4.2), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  curve(df, -2.35, 2.35, yclip: (-4.3, 4.2), stroke: 1.4pt + phys-orange)
  circle((-1, 0), radius: 0.08, fill: ink, stroke: none)
  circle((1, 0), radius: 0.08, fill: ink, stroke: none)
  content((-1, -0.5), text(size: 8pt, fill: ink, [$x = -1$]))
  content((1, -0.5), text(size: 8pt, fill: ink, [$x = 1$]))
  content((-2.1, 1.5), text(size: 8pt, fill: green-ok, [$f'(x) > 0$]))
  content((2.1, 1.5), text(size: 8pt, fill: green-ok, [$f'(x) > 0$]))
  content((0, -3.5), text(size: 8pt, fill: warn-red, [$f'(x) < 0$]))
  content((-2.45, 3.6), text(size: 8.5pt, fill: phys-orange, [$f'(x) = 3x^2 - 3$]))
})

#let fig = grid(columns: 1, row-gutter: 4pt, fig-a, fig-b)
