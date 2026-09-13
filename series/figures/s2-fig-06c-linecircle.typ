// s2-fig-06c-linecircle.typ — 直线与圆：三种位置关系由 d 与 r 决定
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 相离
  circle((1.5, 2.4), radius: 0.85, stroke: 1.2pt + math-blue)
  line((-0.3, 4.4), (3.4, 4.4), stroke: 1.1pt + phys-orange)
  content((1.5, 0.7), text(size: 8.5pt, fill: ink, align(center, [相离#linebreak()$d > r$：0 个交点])))
  // 相切
  circle((5.9, 2.4), radius: 0.85, stroke: 1.2pt + math-blue)
  line((4.2, 3.25), (7.6, 3.25), stroke: 1.1pt + phys-orange)
  circle((5.9, 3.25), radius: 0.06, fill: warn-red, stroke: none)
  content((5.9, 0.7), text(size: 8.5pt, fill: ink, align(center, [相切#linebreak()$d = r$：1 个切点])))
  // 相交
  circle((10.5, 2.4), radius: 0.85, stroke: 1.2pt + math-blue)
  line((8.9, 2.6), (12.1, 2.6), stroke: 1.1pt + phys-orange)
  circle((9.7, 2.6), radius: 0.06, fill: warn-red, stroke: none)
  circle((11.3, 2.6), radius: 0.06, fill: warn-red, stroke: none)
  content((10.5, 0.7), text(size: 8.5pt, fill: ink, align(center, [相交#linebreak()$d < r$：2 个交点])))
  content((6.7, 5.0), text(size: 9pt, fill: gray-line, align(center, [圆心到直线距离 d 与半径 r 的大小，决定一切（圆与椭圆、抛物线的联立同理）])))
})
