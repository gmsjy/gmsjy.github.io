// s1-fig-09c-frame.typ — 参考系：同一运动，两种描述
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 站台参考系
  line((0.5, 2.9), (6.6, 2.9), stroke: 1pt + ink)
  rect((1.2, 3.05), (4.2, 4.0), fill: soft-blue, stroke: ink)
  circle((1.7, 2.9), radius: 0.18, fill: soft-gray, stroke: ink)
  circle((3.7, 2.9), radius: 0.18, fill: soft-gray, stroke: ink)
  content((2.7, 3.5), text(size: 8.5pt, fill: ink, [乘客（静止坐着）]))
  vector((4.3, 3.5), (5.5, 3.5), paint: warn-red, thickness: 1.2pt, label: [站台看：车向右动], label-off: (0.1, 0.25), label-size: 7.8pt)
  content((3.4, 2.35), text(size: 8.5pt, fill: gray-line, [参考系 = 站台]))
  // 车厢参考系
  line((7.6, 2.9), (13.7, 2.9), stroke: 1pt + ink)
  rect((8.3, 3.05), (11.3, 4.0), fill: soft-orange, stroke: ink)
  circle((8.8, 2.9), radius: 0.18, fill: soft-gray, stroke: ink)
  circle((10.8, 2.9), radius: 0.18, fill: soft-gray, stroke: ink)
  content((9.8, 3.5), text(size: 8.5pt, fill: ink, [乘客（还是静止）]))
  vector((11.4, 3.5), (12.6, 3.5), paint: math-blue, thickness: 1.2pt, label: [车内看：站台向左退], label-off: (0.05, 0.25), label-size: 7.8pt)
  content((10.5, 2.35), text(size: 8.5pt, fill: gray-line, [参考系 = 车厢]))
  content((7.1, 0.9), text(size: 9pt, fill: math-blue, weight: "bold", align(center, [运动的描述是相对的：先选参考系，再谈运动。#linebreak()高中的「相对运动」问题，都从这一句长出来])))
})
