// s1-fig-15c-practice.typ — 刷题决策树：先诊断再动手
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((7.0, 5.3), [拿到一道题，先过三道闸], w: 4.6, h: 0.85, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 9.5pt)
  node((7.0, 3.9), [考点认得出吗？], w: 3.6, h: 0.85, fill: soft-gray, stroke: gray-line, text-size: 9pt)
  node((2.6, 2.3), [先看例题，#linebreak()10 分钟后重做], w: 3.2, h: 1.15, fill: soft-red, stroke: warn-red, text-size: 8.5pt)
  node((7.0, 2.3), [限时独立做], w: 2.9, h: 0.85, fill: soft-blue, stroke: math-blue, text-size: 9pt)
  node((11.4, 2.3), [直接跳过], w: 2.4, h: 0.85, fill: soft-gray, stroke: gray-line, text-size: 9pt)
  node((4.4, 0.7), [卡壳 15 分钟 →#linebreak()看提示不看答案], w: 3.9, h: 1.15, fill: soft-orange, stroke: phys-orange, text-size: 8pt)
  node((9.6, 0.7), [顺利通过 →#linebreak()标记为「可跳过类型」], w: 4.1, h: 1.15, fill: soft-green, stroke: green-ok, text-size: 8pt)
  arrow((7.0, 3.45), (7.0, 2.75), paint: gray-line, thickness: 1pt, label: [认得出], label-off: (0.85, 0))
  arrow((5.15, 3.7), (3.1, 2.9), paint: gray-line, thickness: 1pt, label: [认不出], label-off: (-0.2, 0.25))
  arrow((8.85, 3.7), (10.9, 2.9), paint: gray-line, thickness: 1pt, label: [考试不考], label-off: (0.5, 0.25))
  arrow((4.4, 1.7), (5.3, 1.35), paint: gray-line, thickness: 1pt)
  arrow((7.0, 1.85), (7.6, 1.35), paint: gray-line, thickness: 1pt, label: [做完了], label-off: (0.9, 0.1))
})
