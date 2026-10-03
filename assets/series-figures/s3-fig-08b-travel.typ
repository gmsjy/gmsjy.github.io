// s3-fig-08b-travel.typ — 波形图：同一列波在两个时刻的快照
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 6.9, -1.9, 2.4, x-step: 1, y-step: 1, x-label: $x$, y-label: $y$, tick-labels: false)
  // t 时刻实线，t+Δt 时刻虚线（右移）
  curve(x => calc.sin(x), 0, 6.5, n: 70, stroke: 1.5pt + math-blue)
  curve(x => calc.sin(x - 0.9), 0.9, 6.5, n: 60, stroke: (paint: phys-orange, thickness: 1.3pt, dash: "dashed"))
  content((2.0, 1.6), text(size: 8.5pt, fill: math-blue, weight: "bold", [$t$ 时刻]))
  content((4.4, 1.3), text(size: 8.5pt, fill: phys-orange, weight: "bold", [$t + Delta t$]))
  arrow((2.6, -1.45), (3.8, -1.45), paint: green-ok, thickness: 1.3pt, label: [波向右传 $v = lambda / T$], label-off: (0, 0.25), label-size: 8pt)
  content((3.4, 2.15), text(size: 8.5pt, fill: gray-line, align(center, [波形整体平移：介质质点并不随波迁移，#linebreak()只在自己的平衡位置附近振动])))
})
