// s2-fig-11-energy2.typ — 势能曲线：总能量线与「安全区」
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.4, 6.4, -0.4, 4.6, x-step: 1, y-step: 1, x-label: $x$, y-label: $E_p$, tick-labels: false)
  // 势能曲线（谷形）
  curve(x => 4.0 * (x - 3) * (x - 3) / 4.5 + 0.4, 0.9, 5.1, stroke: 1.4pt + math-blue)
  content((3.0, 0.75), text(size: 8pt, fill: math-blue, weight: "bold", [势能谷底 = 稳定平衡点]))
  // 总能量线
  line((0, 2.6), (6.2, 2.6), stroke: (paint: phys-orange, thickness: 1.2pt))
  content((5.3, 2.9), text(size: 8.5pt, fill: phys-orange, weight: "bold", [总能量线 $E$]))
  // 转向点
  circle((1.55, 2.6), radius: 0.08, fill: warn-red, stroke: none)
  circle((4.45, 2.6), radius: 0.08, fill: warn-red, stroke: none)
  content((1.35, 2.95), text(size: 8pt, fill: warn-red, weight: "bold", [转向点]))
  content((4.5, 2.95), text(size: 8pt, fill: warn-red, weight: "bold", [转向点]))
  content((6.55, 1.3), text(size: 8.5pt, fill: gray-line, align(center, [动能 = 竖直距离：#linebreak()谷底动能最大；#linebreak()碰到总能量线就得折返])), anchor: "west")
})
