// s2-fig-06-conic.typ — 三种圆锥曲线同框：椭圆、双曲线、抛物线
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 椭圆
  conic(kind: "ellipse", center: (2.8, 3.2), a: 2.0, b: 1.25, stroke: 1.3pt + math-blue)
  circle((1.35, 3.2), radius: 0.07, fill: warn-red, stroke: none)
  circle((4.25, 3.2), radius: 0.07, fill: warn-red, stroke: none)
  content((2.8, 3.2), text(size: 7.5pt, fill: math-blue, weight: "bold", [椭圆]))
  content((2.8, 1.4), text(size: 7.8pt, fill: gray-line, [到两焦点距离之和 = 定值]))
  // 双曲线
  conic(kind: "hyperbola", center: (9.3, 3.2), a: 1.0, b: 0.85, xmax: 2.6, stroke: 1.3pt + phys-orange)
  content((9.3, 4.9), text(size: 7.5pt, fill: phys-orange, weight: "bold", [双曲线]))
  content((9.3, 1.4), text(size: 7.8pt, fill: gray-line, [到两焦点距离之差 = 定值]))
  // 抛物线
  conic(kind: "parabola", center: (14.2, 3.2), p: 0.55, ymax: 2.1, stroke: 1.3pt + green-ok)
  content((14.2, 5.3), text(size: 7.5pt, fill: green-ok, weight: "bold", [抛物线]))
  content((14.2, 1.4), text(size: 7.8pt, fill: gray-line, [到焦点 = 到准线]))
  content((7.6, 0.2), text(size: 9pt, fill: ink, weight: "bold", align(center, [同一个圆锥的三种截面，三套标准方程——统称圆锥曲线])))
})
