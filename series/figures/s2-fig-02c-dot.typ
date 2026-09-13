// s2-fig-02c-dot.typ — 数量积：投影的语言
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.4, 6.4, -0.4, 4.2, x-step: 1, y-step: 1, tick-labels: false)
  let o = (0.6, 0.5)
  let a = (4.6, 0.8)
  let b = (3.0, 3.0)
  vector(o, a, paint: math-blue, thickness: 1.4pt, label: $bold(a)$, label-off: (0.1, -0.4))
  vector(o, b, paint: phys-orange, thickness: 1.4pt, label: $bold(b)$, label-off: (0.05, 0.2))
  // b 在 a 上的投影
  let dot = (o.at(0) + (3.0 - 0.6) * 0.72 + 0.6 * 0.28, o.at(1) + (3.0 - 0.5) * 0.28)
  let proj = (o.at(0) + 3.05, o.at(1) + 0.4)
  line((b.at(0), b.at(1)), proj, stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  circle(proj, radius: 0.06, fill: gray-line, stroke: none)
  vector(o, proj, paint: green-ok, thickness: 1.2pt, label: [投影], label-off: (0.3, -0.4), label-size: 8pt)
  content((0.8, 4.0), text(size: 8.5pt, fill: ink, align(center, [物理同款：$W = bold(F) dot bold(s)$，#linebreak()功 = 力在位移方向的投影 × 位移])))
})
