// s3-fig-06b-itgraph.typ — 双杆动态的 i–t / v–t 趋势
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 6.4, -2.9, 2.9, x-step: 1, y-step: 1, x-label: $t$, y-label: $i$)
  // i 从峰值指数衰减到 0
  curve(t => 2.2 * calc.exp(-t / 1.6), 0, 6.0, stroke: 1.4pt + math-blue)
  content((3.4, 1.9), text(size: 8.5pt, fill: math-blue, weight: "bold", [$i (t)$：从峰值衰减到 0]))
  content((0.35, 2.45), text(size: 8pt, fill: gray-line, [$I_0 = B L v_0 / R$]))
  line((0, 0), (0, 2.2), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  content((3.6, -1.5), text(size: 8.5pt, fill: gray-line, align(center, [棒1加速、棒2减速 → 速度差缩小 → 感应电动势减小 → 电流趋零，#linebreak()最终两棒共速（动量守恒定共同速度）])))
})
