// s3-fig-04b-decision.typ — 期望决策树：答题顺序的期望收益
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((7.0, 4.8), [剩余 10 分钟，先攻哪道压轴？], w: 6.6, h: 0.9, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 9.5pt)
  node((3.2, 3.0), [先做第 21 题], w: 3.0, h: 0.9, fill: soft-blue, stroke: math-blue, text-size: 9pt)
  node((10.8, 3.0), [先做第 22 题], w: 3.0, h: 0.9, fill: soft-blue, stroke: math-blue, text-size: 9pt)
  let rows = (
    (x: 2.4, opts: (([12 分#linebreak()P = 0.6], soft-green, green-ok), ([4 分#linebreak()P = 0.4], soft-orange, phys-orange))),
    (x: 11.6, opts: (([15 分#linebreak()P = 0.3], soft-green, green-ok), ([2 分#linebreak()P = 0.7], soft-orange, phys-orange))),
  )
  for grp in rows {
    for k in (0, 1) {
      let o = grp.opts.at(k)
      node((grp.x + (k * 3.4 - 1.7), 1.15), o.at(0), w: 2.5, h: 1.15, fill: o.at(1), stroke: o.at(2), text-size: 8pt)
    }
  }
  arrow((4.75, 3.0), (5.35, 2.6), paint: gray-line, thickness: 1pt)
  arrow((2.9, 2.55), (2.4, 1.85), paint: gray-line, thickness: 1pt, label: [0.6], label-off: (-0.45, 0), label-size: 7.5pt)
  arrow((3.9, 2.55), (4.4, 1.85), paint: gray-line, thickness: 1pt, label: [0.4], label-off: (0.45, 0), label-size: 7.5pt)
  arrow((9.25, 3.0), (9.85, 2.6), paint: gray-line, thickness: 1pt)
  arrow((10.5, 2.55), (9.9, 1.85), paint: gray-line, thickness: 1pt, label: [0.3], label-off: (-0.45, 0), label-size: 7.5pt)
  arrow((11.9, 2.55), (13.3, 1.85), paint: gray-line, thickness: 1pt, label: [0.7], label-off: (0.45, 0), label-size: 7.5pt)
  content((7.0, -0.55), text(size: 9.5pt, fill: warn-red, weight: "bold", align(center, [期望收益：$0.6 dot 12 + 0.4 dot 4 = 8.8$ ＞ $0.3 dot 15 + 0.7 dot 2 = 5.9$ —— 先攻第 21 题])))
})
