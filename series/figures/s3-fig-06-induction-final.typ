// s3-fig-06-induction-final.typ — 双杆模型：轨道上的两根导体棒
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  // 导轨
  line((1.0, 0.6), (9.4, 0.6), stroke: 1.1pt + ink)
  line((1.0, 3.0), (9.4, 3.0), stroke: 1.1pt + ink)
  // 左杆（静止/初速）与右杆
  line((3.2, 0.6), (3.2, 3.0), stroke: (paint: ink, thickness: 2.2pt))
  content((3.2, -0.05), text(size: 8pt, fill: math-blue, weight: "bold", [$m_1$ 棒 1]))
  line((6.6, 0.6), (6.6, 3.0), stroke: (paint: ink, thickness: 2.2pt))
  content((6.6, -0.05), text(size: 8pt, fill: phys-orange, weight: "bold", [$m_2$ 棒 2]))
  // 棒2初速
  vector((6.7, 2.55), (7.85, 2.55), paint: phys-orange, thickness: 1.2pt, label: $v_0$, label-off: (0.05, 0.2), label-size: 8.5pt)
  // 电流方向
  vector((6.6, 1.6), (6.6, 0.95), paint: warn-red, thickness: 1pt, label: $I$, label-off: (0.22, -0.05), label-size: 8pt)
  // 磁场符号
  b-into((2.0, 2.3))
  b-into((4.9, 1.3))
  b-into((8.4, 2.0))
  content((9.8, 2.6), text(size: 8.5pt, fill: gray-line, [$B$ 向里]), anchor: "west")
  content((5.2, 3.6), text(size: 8.5pt, fill: ink, align(center, [棒 2 切割磁感线 → 回路电流 → 棒 1 受安培力起步、棒 2 受阻减速])))
})
