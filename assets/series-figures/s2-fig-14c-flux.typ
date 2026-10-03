// s2-fig-14c-flux.typ — 磁通量变化率决定电动势：Φ–t 图的斜率
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  axes(-0.3, 6.4, -0.3, 3.9, x-step: 1, y-step: 1, x-label: $t$, y-label: $Phi$, tick-labels: false)
  // Φ 先增后平：斜率 = E
  curve(x => 3 * (1 - calc.exp(-x / 1.4)), 0, 6.0, stroke: 1.5pt + math-blue)
  // 切线
  line((1.1, 1.15), (3.1, 3.1), stroke: 1.2pt + phys-orange)
  content((3.3, 3.2), text(size: 8.5pt, fill: phys-orange, weight: "bold", [切线斜率 = $E$]), anchor: "west")
  // 对比：平坦段
  content((5.35, 1.5), text(size: 8.5pt, fill: gray-line, align(center, [曲线变平#linebreak()$Delta Phi / Delta t -> 0$])))
  vector((4.6, 2.6), (4.6, 2.95), paint: warn-red, thickness: 1pt, scale: 0.5)
  vector((4.6, 2.6), (4.6, 2.25), paint: warn-red, thickness: 1pt, scale: 0.5)
  content((7.0, -0.15), text(size: 8.5pt, fill: gray-line, align(center, [感应电动势不看「磁通有多少」，只看「磁通变得多快」——$E = n dot (Delta Phi) / (Delta t)$])))
})
