// s3-fig-15-data.typ — 线性回归与相关系数
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 5.9, -0.3, 4.4, x-step: 1, y-step: 1, x-label: $x$, y-label: $y$, tick-labels: false)
  // 强正相关
  let pts = ((0.8, 0.9), (1.6, 1.5), (2.4, 2.1), (3.2, 2.8), (4.0, 3.4), (4.8, 3.9))
  for p in pts { circle(p, radius: 0.07, fill: math-blue, stroke: none) }
  fit-line(0.77, 0.3, 0.3, 5.3, paint: warn-red, thickness: 1.2pt, dash: "solid")
  content((3.2, 1.35), text(size: 8.5pt, fill: warn-red, weight: "bold", [回归线]))
  content((1.6, 3.7), text(size: 8.5pt, fill: ink, weight: "bold", [$r approx 0.99$：强正相关]))
  content((3.0, 0.9), text(size: 8.5pt, fill: gray-line, align(center, [机器学习的第一课：所谓「训练」，#linebreak()就是找这条最优的线])))
})
