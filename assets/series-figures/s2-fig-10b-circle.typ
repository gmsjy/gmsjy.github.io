// s2-fig-10b-circle.typ — 竖直平面圆周：最高点与最低点的受力
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  circle((3.0, 3.0), radius: 2.0, stroke: 1.2pt + gray-line)
  // 最高点
  circle((3.0, 5.0), radius: 0.12, fill: ink, stroke: none)
  vector((3.0, 5.0), (3.0, 4.15), paint: warn-red, thickness: 1.2pt, label: $m g$, label-off: (0.15, -0.1), label-size: 8pt)
  vector((3.0, 5.0), (3.0, 3.6), paint: math-blue, thickness: 1.2pt, label: $F_N >= 0$, label-off: (0.15, -0.35), label-size: 8pt)
  vector((3.35, 5.0), (4.35, 5.0), paint: phys-orange, thickness: 1.1pt, label: $v$, label-off: (0.05, 0.18), label-size: 8pt)
  content((5.7, 5.2), text(size: 8.5pt, fill: ink, align(center, [最高点：$m g + F_N = m v^2 / r$#linebreak()临界：$v = sqrt(g r)$（绳模型不失速下限）])))
  // 最低点
  circle((3.0, 1.0), radius: 0.12, fill: ink, stroke: none)
  vector((3.0, 1.0), (3.0, 1.85), paint: math-blue, thickness: 1.2pt, label: $F_N - m g = m v^2 / r$, label-off: (0.2, 0.0), label-size: 8pt)
  vector((2.65, 1.0), (1.65, 1.0), paint: phys-orange, thickness: 1.1pt, label: $v$, label-off: (-0.3, 0.18), label-size: 8pt)
  content((7.0, 1.0), text(size: 8.5pt, fill: gray-line, [最低点最容易断]))
})
