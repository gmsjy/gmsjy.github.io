// s2-fig-04b-mean.typ — 均值不等式的几何证明：半圆上的三种平均
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 直径 AB = a + b，圆心 O
  let a = (0.8, 1.0)
  let c = (3.2, 1.0)   // 分点
  let b = (5.6, 1.0)
  let r = 2.4
  circle((3.2, 1.0), radius: r, stroke: 1.2pt + math-blue)
  line(a, b, stroke: 1.2pt + ink)
  content((0.8, 0.6), text(size: 8.5pt, fill: ink, weight: "bold", [$A$]))
  content((5.6, 0.6), text(size: 8.5pt, fill: ink, weight: "bold", [$B$]))
  content((3.2, 0.6), text(size: 8.5pt, fill: ink, weight: "bold", [$C$]))
  // 分段长度
  line((0.8, 0.78), (3.2, 0.78), stroke: 0.6pt + gray-line)
  line((3.2, 0.78), (5.6, 0.78), stroke: 0.6pt + gray-line)
  content((2.0, 0.55), text(size: 8.5pt, fill: math-blue, [$a$]))
  content((4.4, 0.55), text(size: 8.5pt, fill: phys-orange, [$b$]))
  // 半径 OC' = (a+b)/2
  content((3.2, 3.55), text(size: 8.5pt, fill: green-ok, weight: "bold", [半径 $= (a + b) / 2$]))
  // 竖直半弦 CD = sqrt(ab)
  let h = calc.sqrt((r * r) - (1.6 * 1.6))
  line((3.2, 1.0), (3.2, 1.0 + h), stroke: 1.3pt + warn-red)
  content((3.35, 1.0 + h / 2), text(size: 8.5pt, fill: warn-red, weight: "bold", [$sqrt(a b)$]), anchor: "west")
  content((6.4, 2.6), text(size: 9pt, fill: ink, align(center, [半径永远 ≥ 半弦#linebreak()$(a + b) / 2 >= sqrt(a b)$#linebreak()（$a = b$ 时取等号）])))
})
