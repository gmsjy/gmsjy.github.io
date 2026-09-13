// s3-fig-06c-energyflow.typ — 电磁感应的能量账本
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((1.8, 2.6), [外力做功#linebreak()（或动能减少）], w: 3.0, h: 1.2, fill: soft-orange, stroke: phys-orange, text-size: 8.5pt)
  node((7.0, 2.6), [电能#linebreak()$E = B L v$], w: 3.0, h: 1.2, fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
  node((12.2, 2.6), [焦耳热#linebreak()$Q = I^2 R t$], w: 3.2, h: 1.2, fill: soft-gray, stroke: gray-line, text-size: 8.5pt)
  vector((3.4, 2.6), (5.4, 2.6), paint: phys-orange, thickness: 1.6pt, label: [电磁感应], label-off: (0, 0.3), label-size: 8pt)
  vector((8.6, 2.6), (10.5, 2.6), paint: math-blue, thickness: 1.6pt, label: [电流做功], label-off: (0, 0.3), label-size: 8pt)
  content((7.0, 0.9), text(size: 9.5pt, fill: ink, weight: "bold", align(center, [安培力做负功 = 产生电能：$W_"安" = -Q$])))
  content((7.0, -0.1), text(size: 8.5pt, fill: gray-line, align(center, [所有电磁感应压轴的能量问，都是这本账的变形：#linebreak()外力功 − 动能变化 = 回路焦耳热（乘以棒数、段数照旧成立）])))
})
