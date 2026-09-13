// s2-fig-05b-diagonal.typ — 正方体的对角线向量：用坐标算角度与距离
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let pr(p) = (p.at(0) + 0.5 * p.at(2), p.at(1) + 0.3 * p.at(2))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  let v = ((0,0,0), (2,0,0), (2,2,0), (0,2,0), (0,0,2), (2,0,2), (2,2,2), (0,2,2))
  let w = v.map(pr)
  line(w.at(4), w.at(5), w.at(6), w.at(7), close: true, stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  line(w.at(0), w.at(3), w.at(2), close: true, stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  line(w.at(0), w.at(7), w.at(6), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  line(w.at(0), w.at(1), w.at(2), close: true, stroke: 1.1pt + ink)
  line(w.at(1), w.at(5), stroke: 1.1pt + ink)
  line(w.at(2), w.at(6), stroke: 1.1pt + ink)
  line(w.at(4), w.at(5), stroke: 1.1pt + ink)
  // 体对角线 D(0,0,0) → H(2,2,2)
  arrow(w.at(0), w.at(6), paint: warn-red, thickness: 1.5pt, label: $bold("DB")$, label-off: (0.3, 0.1), label-size: 8.5pt)
  // 面对角线
  arrow(w.at(0), w.at(2), paint: math-blue, thickness: 1.2pt, label: $bold("DB'")$, label-off: (0.15, -0.35), label-size: 8pt)
  content((w.at(6).at(0) + 0.2, w.at(6).at(1) + 0.25), text(size: 9pt, fill: ink, weight: "bold", [$B$]))
  content((w.at(0).at(0) - 0.25, w.at(0).at(1) - 0.2), text(size: 9pt, fill: ink, weight: "bold", [$D$]))
  content((6.9, 1.2), text(size: 8.5pt, fill: ink, align(center, [建系后：$bold("DB") = (2, 2, 2)$#linebreak()$|bold("DB")| = 2 sqrt(3)$，与面所成角#linebreak()一步数量积搞定])))
})
