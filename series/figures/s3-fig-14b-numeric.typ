// s3-fig-14b-numeric.typ — 数值解与解析解对比：步长的艺术
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 5.4, -0.3, 3.9, x-step: 1, y-step: 1, x-label: $t/"s"$, y-label: $v/("m/s")$, tick-labels: false)
  // 解析解（直线）
  line((0, 0), (4.8, 3.6), stroke: 1.4pt + math-blue)
  content((3.3, 3.5), text(size: 8.5pt, fill: math-blue, weight: "bold", [解析解（精确）]))
  // 欧拉法数值解（大步长，折线偏移）
  let pts = ((0, 0), (1.2, 0.75), (2.4, 1.95), (3.6, 3.15))
  line(..pts, stroke: (paint: warn-red, thickness: 1.2pt))
  for p in pts { circle(p, radius: 0.07, fill: warn-red, stroke: none) }
  content((3.05, 2.35), text(size: 8.5pt, fill: warn-red, weight: "bold", [数值解（$Delta t = 1.2$）]))
  content((2.6, -0.05), text(size: 8.5pt, fill: gray-line, align(center, [步长越大误差越大（本例中折线系统性偏低）；#linebreak()步长减半 → 误差大体减半——收敛性是数值方法的生命线])))
})
