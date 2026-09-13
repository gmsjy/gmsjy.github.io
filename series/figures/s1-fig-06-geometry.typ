// s1-fig-06-geometry.typ — 三角形三大辅助线：一题多线
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  let a = (1.0, 0.4)
  let b = (6.4, 0.4)
  let c = (4.2, 3.5)
  line(a, b, c, close: true, stroke: 1.2pt + ink)
  // 中线（B 边中点 → C）
  line(((a.at(0) + b.at(0)) / 2, a.at(1)), c, stroke: (paint: math-blue, thickness: 1pt, dash: "dashed"))
  content(((a.at(0) + b.at(0)) / 2 + 0.15, 0.12), text(size: 8pt, fill: math-blue, [中线]))
  // 高（C → AB 垂线）
  line(c, (c.at(0), a.at(1)), stroke: (paint: phys-orange, thickness: 1pt, dash: "dashed"))
  rect((c.at(0), a.at(1)), (c.at(0) + 0.22, a.at(1) + 0.22), stroke: 0.6pt + phys-orange)
  content((c.at(0) + 0.3, 1.8), text(size: 8pt, fill: phys-orange, [高]), anchor: "west")
  // 角平分线（A 角）
  line(a, (5.6, 2.05), stroke: (paint: green-ok, thickness: 1pt, dash: "dashed"))
  content((3.4, 1.75), text(size: 8pt, fill: green-ok, [角平分线]))
  // 标签
  content((0.75, 0.4), text(size: 9pt, fill: ink, weight: "bold", [$A$]))
  content((6.65, 0.4), text(size: 9pt, fill: ink, weight: "bold", [$B$]))
  content((4.2, 3.8), text(size: 9pt, fill: ink, weight: "bold", [$C$]))
  content((3.7, -0.35), text(size: 8.5pt, fill: gray-line, [三种「基本件」：解一半题靠它们自动出现]))
})
