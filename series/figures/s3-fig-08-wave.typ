// s3-fig-08-wave.typ — 简谐运动：x–t 曲线与特征量
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let pi = calc.pi
#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 7.4, -2.9, 2.9, x-step: 1, y-step: 1, x-label: $t$, y-label: $x$, tick-labels: false)
  curve(calc.sin, 0, 2 * calc.pi, n: 80, stroke: 1.5pt + math-blue)
  // 振幅
  line((0, 0), (0, 1), stroke: (paint: phys-orange, thickness: 1pt, dash: "dashed"))
  content((-0.35, 0.6), text(size: 8.5pt, fill: phys-orange, weight: "bold", [$A$]), anchor: "east")
  // 周期
  line((pi / 2, -2.5), (pi / 2 + 2 * calc.pi, -2.5), stroke: 1pt + green-ok, mark: (end: ">>", fill: green-ok, scale: 0.5))
  content((pi / 2 + calc.pi, -2.85), text(size: 8.5pt, fill: green-ok, weight: "bold", [一个周期 $T$]))
  // 关键点
  circle((pi / 2, 1), radius: 0.07, fill: warn-red, stroke: none)
  circle((3 * calc.pi / 2, -1), radius: 0.07, fill: warn-red, stroke: none)
  content((3.4, 1.5), text(size: 8.5pt, fill: gray-line, [最大位移处速度为零、加速度最大——简谐运动的对称性]))
})
