// s1-fig-10c-newton3.typ — 牛顿第三定律：作用力与反作用力成对出现
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 两个冰面上的人互推
  line((0.4, 1.0), (13.6, 1.0), stroke: 1pt + gray-line)
  rect((2.0, 1.15), (3.6, 2.35), fill: soft-blue, stroke: ink)
  content((2.8, 1.75), text(size: 8.5pt, fill: math-blue, weight: "bold", [甲]))
  circle((2.8, 2.62), radius: 0.28, fill: soft-blue, stroke: ink)
  rect((10.4, 1.15), (12.0, 2.35), fill: soft-orange, stroke: ink)
  content((11.2, 1.75), text(size: 8.5pt, fill: phys-orange, weight: "bold", [乙]))
  circle((11.2, 2.62), radius: 0.28, fill: soft-orange, stroke: ink)
  // 作用力与反作用力（等大反向）
  vector((5.6, 1.75), (3.75, 1.75), paint: math-blue, thickness: 1.4pt, label: [甲推乙的力], label-off: (-0.4, 0.3), label-size: 7.8pt)
  vector((8.4, 1.75), (10.25, 1.75), paint: phys-orange, thickness: 1.4pt, label: [乙推甲的力], label-off: (0.1, 0.3), label-size: 7.8pt)
  content((7.0, 3.5), text(size: 9pt, fill: ink, weight: "bold", align(center, [等大、反向、共线、同性质、同时存在——#linebreak()但作用在*不同物体*上，永远不会抵消])))
  // 三要素对照
  content((7.0, 0.45), text(size: 8.5pt, fill: gray-line, align(center, [平衡力：作用在同一物体，可以抵消｜作用力与反作用力：分属两物体，各管各的])))
})
