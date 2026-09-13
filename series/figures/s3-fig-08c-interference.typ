// s3-fig-08c-interference.typ — 双源干涉：加强与减弱的同心弧
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  let s1 = (3.4, 2.6)
  let s2 = (8.0, 2.6)
  for r in (0.65, 1.3, 1.95, 2.5) {
    circle(s1, radius: r, stroke: (paint: math-blue, thickness: 0.7pt))
    circle(s2, radius: r, stroke: (paint: phys-orange, thickness: 0.7pt))
  }
  circle(s1, radius: 0.1, fill: math-blue, stroke: none)
  content((s1.at(0), s1.at(1) - 0.42), text(size: 8.5pt, fill: math-blue, weight: "bold", [$S_1$]))
  circle(s2, radius: 0.1, fill: phys-orange, stroke: none)
  content((s2.at(0), s2.at(1) - 0.42), text(size: 8.5pt, fill: phys-orange, weight: "bold", [$S_2$]))
  content((5.7, 5.2), text(size: 9pt, fill: ink, weight: "bold", align(center, [波峰遇波峰 → 加强；波峰遇波谷 → 减弱])))
  content((5.7, -0.75), text(size: 8.5pt, fill: gray-line, align(center, [到两源距离差为 $n lambda$ 的点始终加强，为 $(n + 1/2) lambda$ 的点始终减弱])))
})
