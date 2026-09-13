// s1-fig-06c-coordinate.typ — 坐标系登场：几何条件变成代数条件
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  axes(-0.6, 6.4, -0.6, 4.6, x-step: 1, y-step: 1, show-grid: true)
  // 三角形放入坐标系
  let a = (0.6, 0.6)
  let b = (5.4, 0.6)
  let c = (3.2, 3.4)
  line(a, b, c, close: true, stroke: 1.2pt + ink)
  for p in ((a, [$A$]), (b, [$B$]), (c, [$C$])) {
    circle(p.at(0), radius: 0.07, fill: warn-red, stroke: none)
    content((p.at(0).at(0) + 0.22, p.at(0).at(1) + 0.2), text(size: 9pt, fill: ink, weight: "bold", p.at(1)))
  }
  // 中点 M 与中位线
  let m = ((a.at(0) + b.at(0)) / 2, a.at(1))
  line(m, c, stroke: (paint: math-blue, thickness: 1.1pt, dash: "dashed"))
  circle(m, radius: 0.06, fill: math-blue, stroke: none)
  content((m.at(0), 0.2), text(size: 8pt, fill: math-blue, [中点 $M(3, 0.6)$]))
  content((4.6, 2.5), text(size: 8pt, fill: math-blue, [中位线 = 坐标平均]))
  content((5.1, 3.9), text(size: 8.5pt, fill: gray-line, [$|A B| = sqrt((x_B - x_A)^2 + (y_B - y_A)^2)$]))
})
