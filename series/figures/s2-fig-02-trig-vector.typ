// s2-fig-02-trig-vector.typ — 单位圆定义三角函数
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  axes(-1.5, 3.9, -1.5, 1.9, x-step: 1, y-step: 1)
  circle((0, 0), radius: 1, stroke: 1.2pt + math-blue)
  let th = 52deg
  let p = (calc.cos(th), calc.sin(th))
  line((0, 0), p, stroke: 1.3pt + ink)
  line(p, (p.at(0), 0), stroke: (paint: phys-orange, thickness: 1.1pt, dash: "dashed"))
  line(p, (0, p.at(1)), stroke: (paint: green-ok, thickness: 1.1pt, dash: "dashed"))
  circle(p, radius: 0.07, fill: warn-red, stroke: none)
  content((p.at(0) + 0.12, p.at(1) + 0.18), text(size: 8.5pt, fill: ink, [$(cos theta, sin theta)$]))
  content((0.55, 0.32), text(size: 8.5pt, fill: ink, [$theta$]))
  content((p.at(0) / 2 + 0.05, p.at(1) / 2 - 0.18), text(size: 8pt, fill: gray-line, [斜边 = 1]))
  content((-0.35, p.at(1) / 2), text(size: 8pt, fill: green-ok, [cos]), anchor: "east")
  content((p.at(0) + 0.16, p.at(1) / 2), text(size: 8pt, fill: phys-orange, [sin]), anchor: "west")
  content((0.4, -1.05), text(size: 8.5pt, fill: gray-line, align(center, [半径 = 1：角离开「三角形」成为实数，#linebreak()正弦线、余弦线随时可读])))
})
