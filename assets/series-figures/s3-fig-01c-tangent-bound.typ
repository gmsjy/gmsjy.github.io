// s3-fig-01c-tangent-bound.typ — 切线放缩：e^x ≥ x+1 与 ln x ≤ x−1
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig-a = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, polygon
  axes(-2.3, 2.3, -0.7, 5.7, x-step: 1, y-step: 2)
  // 切线 y = x + 1（先画，垫底）
  line((-1.7, -0.7), (2.3, 3.3), stroke: (paint: phys-orange, thickness: 1.2pt, dash: "dashed"))
  // 曲线与切线之间的放缩区域（x ≥ 0）
  let band = ()
  band = band + samples(calc.exp, 0, 1.62, n: 30)
  band = band + ((1.62, 2.62), (0, 1))
  line(..band, close: true, fill: soft-green.lighten(30%), stroke: none)
  curve(calc.exp, -2.3, 1.62, yclip: (-0.7, 5.7), stroke: 1.4pt + math-blue)
  circle((0, 1), radius: 0.07, fill: ink, stroke: none)
  content((0.1, 1.38), text(size: 7.8pt, fill: ink, [$(0, 1)$]))
  content((1.05, 4.75), text(size: 8.5pt, fill: math-blue, [$y = e^x$]))
  content((1.95, 3.55), text(size: 8.5pt, fill: phys-orange, [$y = x + 1$]), anchor: "west")
  content((-2.15, 5.1), text(size: 8.5pt, fill: warn-red, weight: "bold", [恒有 $e^x >= x + 1$]))
  content((-2.15, -0.45), text(size: 7.8pt, fill: gray-line, [绿色区域 = 放缩的「余量」]))
})

#let fig-b = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, polygon
  axes(-0.5, 4.7, -2.5, 2.7, x-step: 1, y-step: 1)
  // 切线 y = x − 1
  line((0, -1), (3.7, 2.7), stroke: (paint: phys-orange, thickness: 1.2pt, dash: "dashed"))
  // 曲线与切线之间的区域（x ≥ 1）
  let band = ()
  band = band + samples(x => calc.ln(x), 1, 3.65, n: 30)
  band = band + ((3.65, 2.65), (1, 0))
  line(..band, close: true, fill: soft-green.lighten(30%), stroke: none)
  curve(x => calc.ln(x), 0.1, 3.65, yclip: (-2.5, 2.7), stroke: 1.4pt + math-blue)
  circle((1, 0), radius: 0.07, fill: ink, stroke: none)
  content((1.12, -0.4), text(size: 7.8pt, fill: ink, [$(1, 0)$]))
  content((3.15, 1.35), text(size: 8.5pt, fill: math-blue, [$y = ln x$]))
  content((3.15, 2.5), text(size: 8.5pt, fill: phys-orange, [$y = x - 1$]))
  content((3.0, -2.1), text(size: 8.5pt, fill: warn-red, weight: "bold", [恒有 $ln x <= x - 1$]))
})

#let fig = grid(columns: 2, gutter: 10pt, fig-a, fig-b)
