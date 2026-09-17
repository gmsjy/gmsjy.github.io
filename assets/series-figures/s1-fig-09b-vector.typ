// s1-fig-09b-vector.typ — 矢量合成：平行四边形与三角形法则
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  axes(-0.4, 6.8, -0.4, 4.6, x-step: 1, y-step: 1, tick-labels: false)
  let o = (1.0, 0.6)
  let a = (4.6, 0.9)
  let b = (2.2, 3.3)
  vector(o, a, paint: math-blue, thickness: 1.4pt, label: $F_1$, label-off: (0.1, -0.35))
  vector(o, b, paint: phys-orange, thickness: 1.4pt, label: $F_2$, label-off: (-0.4, 0.1))
  line(a, (a.at(0) + b.at(0) - o.at(0), a.at(1) + b.at(1) - o.at(1)), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  line(b, (a.at(0) + b.at(0) - o.at(0), a.at(1) + b.at(1) - o.at(1)), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  vector(o, (a.at(0) + b.at(0) - o.at(0), a.at(1) + b.at(1) - o.at(1)), paint: warn-red, thickness: 1.6pt, label: $F_"合"$, label-off: (0.18, 0.15))
  content((4.9, 4.15), text(size: 8.5pt, fill: ink, align(center, [平行四边形法则#linebreak()（共起点，对角线 = 合力）])))
  // 三角形法则（右）
  vector((5.6, 0.6), (9.2, 0.9), paint: math-blue, thickness: 1.4pt)
  vector((9.2, 0.9), (6.8, 3.3 - 0.55 + 0.6), paint: phys-orange, thickness: 1.4pt, label: $F_2$, label-off: (0.25, 0.0))
  vector((5.6, 0.6), (6.8, 3.35), paint: warn-red, thickness: 1.6pt, label: $F_"合"$, label-off: (-0.45, 0.15))
  content((8.9, 4.15), text(size: 8.5pt, fill: ink, align(center, [三角形法则#linebreak()（首尾相接）])))
  content((0.6, -0.05), text(size: 8pt, fill: gray-line, [标量直接相加，矢量要按方向合成——这是高中物理第一道新的门槛]))
})
