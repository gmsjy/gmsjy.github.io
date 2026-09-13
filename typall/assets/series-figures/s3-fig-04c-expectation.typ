// s3-fig-04c-expectation.typ — 期望随回合收敛
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 7.4, -0.3, 4.4, x-step: 1, y-step: 1, x-label: [回合 $n$], y-label: [答对率 $P_n$])
  // 振荡收敛到 0.6
  let pts = ((0.5, 1.0), (1.4, 0.17), (2.3, 0.94), (3.2, 0.36), (4.1, 0.81), (5.0, 0.46), (5.9, 0.69), (6.8, 0.55))
  line(..pts, stroke: 1.5pt + math-blue)
  for p in pts { circle(p, radius: 0.06, fill: math-blue, stroke: none) }
  line((0, 2.4), (7.2, 2.4), stroke: (paint: warn-red, thickness: 1pt, dash: "dashed"))
  content((7.15, 2.65), text(size: 8.5pt, fill: warn-red, weight: "bold", [稳态 $P = 0.6$]), anchor: "west")
  content((3.5, -1.15), text(size: 8.5pt, fill: gray-line, align(center, [不管从答对还是答错出发，答对率都会振荡收敛到稳态——#linebreak()求稳态只需解二元方程组，不需要模拟一万回合])))
})
