// s2-fig-04c-compare.typ — 三组 (a, b) 的算术平均 vs 几何平均
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  let groups = ((1.8, [1, 9], 5.0, 3.0), (4.3, [2, 8], 5.0, 4.0), (6.8, [4, 6], 5.0, 4.9))
  axes(-0.4, 9.2, -0.4, 6.4, x-step: 2, y-step: 2, x-label: none, y-label: none, tick-labels: false)
  for g in groups {
    let x = g.at(0)
    let am = g.at(2)
    let gm = g.at(3)
    rect((x - 0.35, 0), (x, am), fill: soft-blue, stroke: 1pt + math-blue)
    rect((x + 0.05, 0), (x + 0.4, gm), fill: soft-orange, stroke: 1pt + phys-orange)
    content((x - 0.18, am + 0.2), text(size: 7.8pt, fill: math-blue, weight: "bold", fnum(am)))
    content((x + 0.22, gm + 0.2), text(size: 7.8pt, fill: phys-orange, weight: "bold", str(gm)))
    content((x, -0.5), text(size: 8.5pt, fill: ink, [a, b = #g.at(1)]))
  }
  content((0.9, 5.9), text(size: 8.5pt, fill: math-blue, [■ 算术平均]))
  content((3.6, 5.9), text(size: 8.5pt, fill: phys-orange, [■ 几何平均]))
  content((4.7, -1.35), text(size: 8.5pt, fill: gray-line, align(center, [a 与 b 差得越远，两个平均差距越大；a = b 时两柱等高（取等）])))
})
