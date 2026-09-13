// s1-fig-14b-geometry.typ — 数形结合：代数条件与图像互相翻译
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let f(x) = x * x - 2 * x

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-1.6, 3.9, -3.7, 3.9, x-step: 1, y-step: 1, show-grid: true)
  curve(f, -1.4, 3.4, yclip: (-3.7, 3.9), stroke: 1.4pt + math-blue)
  line((-1.4, -3), (3.4, -3), stroke: 1.2pt + phys-orange)
  content((-1.5, 3.3), text(size: 8.5pt, fill: math-blue, [$y = x^2 - 2x$]))
  content((3.1, -2.75), text(size: 8.5pt, fill: phys-orange, [$y = -3$]), anchor: "west")
  content((1.05, 1.0), text(size: 8.5pt, fill: ink, align(center, [代数语言：#linebreak()解 $x^2 - 2x = -3$，#linebreak()判别式 $Delta = 4 - 12 < 0$，无解])))
  content((1.05, -1.75), text(size: 8.5pt, fill: warn-red, weight: "bold", align(center, [图像语言：直线与抛物线#linebreak()没有交点——一眼可见])))
})
