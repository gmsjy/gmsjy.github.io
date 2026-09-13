// s2-fig-07b-regression.typ — 散点与回归线：从数据里读出关系
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let pts = ((0.7, 0.8, 0.2), (1.6, 1.3, 0.18), (2.5, 2.1, 0.22), (3.4, 2.6, 0.16), (4.3, 3.2, 0.24), (5.2, 3.9, 0.2))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 6.3, -0.3, 4.7, x-step: 1, y-step: 1, x-label: [广告费 $x$], y-label: [销量 $y$])
  scatter-error(pts, dot: math-blue, bar: gray-line)
  fit-line(0.68, 0.25, 0.2, 6.1, paint: warn-red, thickness: 1.2pt, dash: "solid")
  content((4.9, 1.35), text(size: 8.5pt, fill: warn-red, weight: "bold", [回归线 $hat(y) = b x + a$]))
  content((3.1, 0.5), text(size: 8.5pt, fill: gray-line, align(center, [回归线由最小二乘法确定：#linebreak()让所有点的竖直偏差平方和最小])))
})
