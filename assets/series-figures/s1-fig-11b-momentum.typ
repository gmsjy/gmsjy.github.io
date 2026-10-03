// s1-fig-11b-momentum.typ — 碰撞前后：动量矢量图
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 碰前
  content((2.6, 3.9), text(size: 9pt, fill: ink, weight: "bold", [碰前]))
  circle((1.2, 2.2), radius: 0.42, fill: soft-blue, stroke: math-blue)
  content((1.2, 2.2), text(size: 7.5pt, fill: math-blue, [$m_1$]))
  vector((1.7, 2.2), (3.3, 2.2), paint: math-blue, thickness: 1.4pt, label: $v_1$, label-off: (0, 0.26))
  circle((4.4, 2.2), radius: 0.5, fill: soft-orange, stroke: phys-orange)
  content((4.4, 2.2), text(size: 7.5pt, fill: phys-orange, [$m_2$]))
  content((4.4, 1.35), text(size: 7.8pt, fill: gray-line, [静止]))
  // 碰后
  content((11.2, 3.9), text(size: 9pt, fill: ink, weight: "bold", [碰后]))
  circle((9.6, 2.2), radius: 0.42, fill: soft-blue, stroke: math-blue)
  content((9.6, 2.2), text(size: 7.5pt, fill: math-blue, [$m_1$]))
  vector((9.05, 2.2), (8.05, 2.2), paint: math-blue, thickness: 1.1pt, label: $v'_1$, label-off: (-0.15, 0.26))
  circle((11.6, 2.2), radius: 0.5, fill: soft-orange, stroke: phys-orange)
  content((11.6, 2.2), text(size: 7.5pt, fill: phys-orange, [$m_2$]))
  vector((12.15, 2.2), (13.35, 2.2), paint: phys-orange, thickness: 1.4pt, label: $v'_2$, label-off: (0.05, 0.26))
  // 守恒式
  content((7.0, 0.7), text(size: 9.5pt, fill: warn-red, weight: "bold", align(center, [$m_1 v_1 = m_1 v'_1 + m_2 v'_2$])))
  content((7.0, -0.15), text(size: 8.5pt, fill: gray-line, [碰前总动量 = 碰后总动量：矢量式，方向也要算]))
})
