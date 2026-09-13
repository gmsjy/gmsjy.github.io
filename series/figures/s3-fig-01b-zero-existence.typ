// s3-fig-01b-zero-existence.typ — 零点存在性定理的图像语言
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let f(x) = 0.25 * (x - 3) * (x - 3) * (x - 3) + 0.6 * (x - 3)

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  // 区间 [a, b] 高亮带
  rect((1.5, -2.5), (4.5, 2.5), fill: soft-green.lighten(40%), stroke: none)

  axes(-0.5, 5.6, -2.5, 2.6, x-step: 1, y-step: 1)
  curve(f, 0.9, 5.1, yclip: (-2.5, 2.6), stroke: 1.4pt + math-blue)

  // 端点 a、b
  line((1.5, -1.93), (1.5, 0), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  line((4.5, 0), (4.5, 1.93), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  circle((1.5, -1.93), radius: 0.08, fill: ink, stroke: none)
  circle((4.5, 1.93), radius: 0.08, fill: ink, stroke: none)
  content((1.5, -2.85), text(size: 8.5pt, fill: ink, weight: "bold", [$a$]))
  content((4.5, -0.38), text(size: 8.5pt, fill: ink, weight: "bold", [$b$]))
  content((0.62, -2.1), text(size: 8pt, fill: ink, [$f(a) < 0$]))
  content((5.05, 1.75), text(size: 8pt, fill: ink, [$f(b) > 0$]), anchor: "west")

  // 零点
  circle((3, 0), radius: 0.11, fill: warn-red, stroke: white)
  content((3.2, -0.85), text(size: 8.5pt, fill: warn-red, weight: "bold", [存在零点 $xi$]), anchor: "west")
})
