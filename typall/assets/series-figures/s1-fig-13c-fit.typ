// s1-fig-13c-fit.typ — 误差棒与两种拟合：好线穿点而过，坏线被点抛弃
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let pts = ((0.8, 0.9, 0.22), (1.8, 1.5, 0.18), (2.8, 2.4, 0.26), (3.8, 3.0, 0.2), (4.8, 3.7, 0.24))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  axes(-0.3, 5.7, -0.3, 4.4, x-step: 1, y-step: 1, x-label: $x$, y-label: $y$, tick-labels: false)
  scatter-error(pts, dot: math-blue, bar: gray-line)
  fit-line(0.72, 0.28, 0.3, 5.4, paint: green-ok, thickness: 1.3pt, dash: "solid")
  fit-line(0.55, 1.5, 0.3, 5.4, paint: warn-red, thickness: 1pt, dash: "dashed")
  content((3.9, 3.6), text(size: 8.5pt, fill: green-ok, weight: "bold", [好拟合：穿过误差棒]))
  content((0.5, 2.6), text(size: 8.5pt, fill: warn-red, weight: "bold", [坏拟合]), anchor: "west")
  content((5.55, 0.35), text(size: 8.5pt, fill: gray-line, align(center, [误差棒 = 测量的不确定范围。#linebreak()数据永远有抖动，直线取「折中」])), anchor: "east")
})
