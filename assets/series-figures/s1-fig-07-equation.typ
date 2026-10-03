// s1-fig-07-equation.typ — 函数交点：方程的根 = 图像的交点/零点
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let f(x) = 0.5 * x * x - x - 2
#let g(x) = -0.5 * x + 1

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-3.2, 4.6, -3.4, 4.6, x-step: 1, y-step: 1, show-grid: true)
  curve(f, -2.1, 3.7, stroke: 1.3pt + math-blue)
  line((-1.4, g(-1.4)), (4.2, g(4.2)), stroke: 1.3pt + phys-orange)
  // 交点：0.5x²-x-2 = -0.5x+1 → x²-0.5x-6=0 → x=-2.216? 取整数设计改为交点在 x=-2 与 3
  circle((-2, 2), radius: 0.08, fill: warn-red, stroke: white)
  circle((3, -0.5), radius: 0.08, fill: warn-red, stroke: white)
  content((-2.15, 2.4), text(size: 8.5pt, fill: warn-red, weight: "bold", [交点 1]))
  content((3.1, -0.95), text(size: 8.5pt, fill: warn-red, weight: "bold", [交点 2]))
  content((-3.05, 3.9), text(size: 8.5pt, fill: math-blue, [抛物线 $y = f(x)$]))
  content((3.6, 1.6), text(size: 8.5pt, fill: phys-orange, [直线 $y = g(x)$]), anchor: "west")
  content((0.4, -2.6), text(size: 8.5pt, fill: ink, align(center, [方程 $f(x) = g(x)$ 的解#linebreak()= 两图像交点的横坐标])))
})
