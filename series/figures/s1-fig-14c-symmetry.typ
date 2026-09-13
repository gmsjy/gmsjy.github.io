// s1-fig-14c-symmetry.typ — 对称与守恒：镜像与能量跷跷板
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 左：镜像对称
  line((3.4, 0.6), (3.4, 4.6), stroke: (paint: gray-line, thickness: 0.9pt, dash: "dashed"))
  content((3.4, 5.0), text(size: 8.5pt, fill: gray-line, [对称轴]))
  circle((2.2, 2.6), radius: 0.55, fill: soft-blue, stroke: math-blue)
  content((2.2, 2.6), text(size: 8pt, fill: math-blue, weight: "bold", [$f(-x)$]))
  circle((4.6, 2.6), radius: 0.55, fill: soft-orange, stroke: phys-orange)
  content((4.6, 2.6), text(size: 8pt, fill: phys-orange, weight: "bold", [$f(x)$]))
  line((2.75, 2.6), (4.05, 2.6), stroke: 0.7pt + gray-line, mark: (end: ">>", fill: gray-line, scale: 0.45))
  content((3.4, 1.2), text(size: 8.5pt, fill: ink, align(center, [对称出现 →#linebreak()只需要研究一半])))
  // 右：单摆能量跷跷板
  let px = 10.4
  line((px, 4.9), (px, 4.5), stroke: 1pt + ink)
  line((px, 4.5), (px - 1.7, 2.2), stroke: 0.9pt + ink)
  line((px, 4.5), (px + 1.7, 2.2), stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  circle((px - 1.7, 2.2), radius: 0.24, fill: math-blue, stroke: none)
  circle((px, 2.0), radius: 0.24, fill: phys-orange, stroke: none)
  line((px - 1.7, 1.7), (px + 1.7, 1.7), stroke: 0.6pt + gray-line, dash: "dashed")
  // 能量条
  rect((px - 2.6, 2.3), (px - 2.2, 3.3), fill: phys-orange, stroke: none)
  rect((px - 2.6, 2.3), (px - 2.2, 2.5), fill: math-blue, stroke: none)
  rect((px + 2.2, 2.3), (px + 2.6, 2.5), fill: phys-orange, stroke: none)
  rect((px + 2.2, 2.5), (px + 2.6, 3.3), fill: math-blue, stroke: none)
  content((px - 3.0, 3.75), text(size: 7.5pt, fill: ink, [最高点]), anchor: "east")
  content((px + 3.0, 3.75), text(size: 7.5pt, fill: ink, [最高点]), anchor: "west")
  content((px - 3.0, 1.95), text(size: 7.5pt, fill: ink, [最低点]), anchor: "east")
  content((px, 0.7), text(size: 8.5pt, fill: ink, align(center, [能量在「势」与「动」之间来回倒，#linebreak()总量条始终一样高])))
})
