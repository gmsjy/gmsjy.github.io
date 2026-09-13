// s2-fig-08-kinematics.typ — x-t / v-t / a-t 三图联动 + 追及问题
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

// 同一运动：a = 0.2 m/s²，v = 0.2t，x = 0.1t²
#let panel-x-t = cetz.canvas(length: 0.9cm, {
  import cetz.draw: line, content
  axes(0, 10.6, 0, 6.6, x-step: 2, y-step: 2, x-label: $t/"s"$, y-label: $x/"m"$)
  line((4, 0), (4, 6.6), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  line((8, 0), (8, 6.6), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  curve(t => 0.1 * t * t, 0, 8.05, stroke: 1.3pt + math-blue)
  content((9.3, 6.0), text(size: 8.5pt, fill: math-blue, weight: "bold", [$x$–$t$]))
})

#let panel-v-t = cetz.canvas(length: 0.9cm, {
  import cetz.draw: line, content
  axes(0, 10.6, 0, 2.8, x-step: 2, y-step: 1, x-label: $t/"s"$, y-label: $v/("m/s")$)
  line((4, 0), (4, 2.8), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  line((8, 0), (8, 2.8), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  curve(t => 0.2 * t, 0, 10.0, stroke: 1.3pt + phys-orange)
  content((9.0, 2.35), text(size: 8.5pt, fill: phys-orange, weight: "bold", [$v$–$t$]))
})

#let panel-a-t = cetz.canvas(length: 0.9cm, {
  import cetz.draw: line, content
  axes(0, 10.6, 0, 0.85, x-step: 2, y-step: 1, tick-labels: true, x-label: $t/"s"$, y-label: $a/("m/s"^2)$)
  line((4, 0), (4, 0.85), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  line((8, 0), (8, 0.85), stroke: (paint: gray-line.lighten(30%), thickness: 0.5pt, dash: "dashed"))
  curve(t => 0.2, 0, 10.0, stroke: 1.3pt + green-ok)
  content((9.0, 0.62), text(size: 8.5pt, fill: green-ok, weight: "bold", [$a$–$t$]))
})

// 追及：A 匀速 v=3，B 由静止匀加速 a=0.6，初始相距 7.5 m
#let fig-chase = cetz.canvas(length: 0.9cm, {
  import cetz.draw: line, content, circle, rect
  axes(0, 8.6, 0, 4.4, x-step: 2, y-step: 1, x-label: $t/"s"$, y-label: $v/("m/s")$)
  // 相遇前 A、B 图线之间的面积 = 初始距离
  let band = ((0, 3),) + range(31).map(i => { let t = 5 - 5 * i / 30; (t, 0.6 * t) })
  line(..band, close: true, fill: soft-orange, stroke: none)
  line((0, 3), (8.2, 3), stroke: 1.3pt + math-blue)
  curve(t => 0.6 * t, 0, 7.0, stroke: 1.3pt + phys-orange)
  circle((5, 3), radius: 0.07, fill: ink, stroke: none)
  line((5, 0), (5, 3), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  content((5, -0.4), text(size: 8pt, fill: ink, weight: "bold", [$t_追$]))
  content((6.6, 3.25), text(size: 8pt, fill: math-blue, [$v_A$ (匀速)]))
  content((1.35, 2.6), text(size: 8pt, fill: phys-orange, [$v_B = a t$]))
  content((1.5, 2.6), text(size: 7.8pt, fill: phys-orange, [阴影面积 = 初始距离]))
  content((1.5, 2.15), text(size: 7.8pt, fill: gray-line, [$s_0 = (1/2) dot v_A dot t_追$]))
})

#let fig = grid(rows: 4, row-gutter: 10pt, panel-x-t, panel-v-t, panel-a-t, fig-chase)
