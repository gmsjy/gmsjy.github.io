// s2-fig-05-solid.typ — 空间坐标系：立体的样子，代数的心脏
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

// 斜二测投影：P(x, y, z) → 屏幕 (x + 0.5z, y + 0.3z)
#let pr(p) = (p.at(0) + 0.5 * p.at(2), p.at(1) + 0.3 * p.at(2))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  let o = pr((0, 0, 0))
  let ux = pr((3.2, 0, 0))
  let uy = pr((0, 3.0, 0))
  let uz = pr((0, 0, 3.0))
  arrow(o, ux, paint: math-blue, thickness: 1.2pt, label: $x$, label-off: (0.1, -0.3))
  arrow(o, uy, paint: phys-orange, thickness: 1.2pt, label: $y$, label-off: (-0.35, 0.15))
  arrow(o, uz, paint: green-ok, thickness: 1.2pt, label: $z$, label-off: (0.15, 0.2))
  // 单位正方体
  let v = ((0,0,0), (2,0,0), (2,2,0), (0,2,0), (0,0,2), (2,0,2), (2,2,2), (0,2,2))
  let w = v.map(pr)
  // 后面与底面（淡）
  line(w.at(4), w.at(5), w.at(6), w.at(7), close: true, stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  line(w.at(0), w.at(3), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  line(w.at(3), w.at(2), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  line(w.at(0), w.at(7), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  line(w.at(7), w.at(6), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  // 前面棱（实）
  line(w.at(0), w.at(1), w.at(2), close: true, stroke: 1.1pt + ink)
  line(w.at(1), w.at(5), stroke: 1.1pt + ink)
  line(w.at(2), w.at(6), stroke: 1.1pt + ink)
  line(w.at(4), w.at(5), stroke: 1.1pt + ink)
  content((2.0, -0.5), text(size: 8.5pt, fill: gray-line, align(center, [空间向量 = 三个坐标的有序组，#linebreak()加减、数量积与平面完全同理])))
})
