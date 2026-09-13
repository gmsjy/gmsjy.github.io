// s3-fig-02-conic-final.typ — 焦点弦：r1 + r2 = 2a 的椭圆语言
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  conic(kind: "ellipse", center: (3.4, 2.8), a: 2.6, b: 1.7, stroke: 1.4pt + math-blue)
  // 焦点
  circle((1.7, 2.8), radius: 0.08, fill: warn-red, stroke: none)
  circle((5.1, 2.8), radius: 0.08, fill: warn-red, stroke: none)
  content((1.7, 2.25), text(size: 8pt, fill: warn-red, weight: "bold", [$F_1$]))
  content((5.1, 2.25), text(size: 8pt, fill: warn-red, weight: "bold", [$F_2$]))
  // 过 F2 的焦点弦
  circle((4.4, 3.9), radius: 0.07, fill: ink, stroke: none)
  circle((4.65, 1.6), radius: 0.07, fill: ink, stroke: none)
  line((4.4, 3.9), (4.65, 1.6), stroke: 1.1pt + phys-orange)
  content((4.85, 3.95), text(size: 8pt, fill: ink, weight: "bold", [$A$]), anchor: "west")
  content((4.85, 1.5), text(size: 8pt, fill: ink, weight: "bold", [$B$]), anchor: "west")
  content((5.15, 3.9), text(size: 7.8pt, fill: gray-line, align(center, [|AF_2| + |BF_2| + |AB| = 4a])), anchor: "west")
  content((3.4, 0.5), text(size: 8.5pt, fill: ink, align(center, [焦点弦问题的一半结论，#linebreak()直接来自定义「距离和」])))
})
