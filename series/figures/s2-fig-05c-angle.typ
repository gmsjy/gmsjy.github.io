// s2-fig-05c-angle.typ — 线面角：斜线、射影与法向量
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let pr(p) = (p.at(0) + 0.45 * p.at(2), p.at(1) + 0.28 * p.at(2))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 平面（平行四边形）
  let a = pr((0, 0, 0))
  let b = pr((4.5, 0, 0))
  let c = pr((4.5, 0, 3.2))
  let d = pr((0, 0, 3.2))
  line(a, b, c, d, close: true, fill: soft-blue.lighten(50%), stroke: 1pt + math-blue)
  content((4.9, 1.35), text(size: 8.5pt, fill: math-blue, weight: "bold", [平面 $alpha$]), anchor: "west")
  // 斜线
  let top = pr((1.2, 2.8, 0.6))
  let foot = pr((2.8, 0, 1.6))
  arrow(top, foot, paint: warn-red, thickness: 1.4pt, label: [斜线], label-off: (-0.4, 0.2), label-size: 8pt)
  circle(foot, radius: 0.06, fill: warn-red, stroke: none)
  // 垂线与射影
  line(top, pr((1.2, 0, 0.6)), stroke: (paint: gray-line, thickness: 0.9pt, dash: "dashed"))
  arrow(foot, pr((1.2, 0, 0.6)), paint: phys-orange, thickness: 1.1pt, label: [射影], label-off: (-0.1, -0.35), label-size: 8pt)
  content((top.at(0) - 0.3, top.at(1) + 0.2), text(size: 8pt, fill: gray-line, [垂线]))
  // 线面角
  content((3.6, 2.35), text(size: 8.5pt, fill: warn-red, weight: "bold", [$theta$]))
  content((5.9, 3.4), text(size: 8.5pt, fill: ink, align(center, [高中升级：用*法向量*算线面角#linebreak()$sin theta = |bold(n) dot bold(l)| / (|bold(n)||bold(l)|)$，#linebreak()证明题从此变成计算题])))
})
