// s2-fig-15-microelement.typ — 微元法：把曲边问题切成矩形
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  axes(-0.3, 5.6, -0.3, 3.9, x-step: 1, y-step: 1, x-label: $x$, y-label: $F$, tick-labels: false)
  // 曲线 F(x) = 3 - 0.4x
  line((0, 3), (5, 1), stroke: 1.4pt + math-blue)
  // 细条
  for i in range(5) {
    let x = i * 1.0
    let h = 3 - 0.4 * (x + 0.5)
    rect((x, 0), (x + 1.0, h), fill: soft-orange.lighten(20%), stroke: (paint: phys-orange, thickness: 0.6pt))
  }
  content((2.5, 2.2), text(size: 9pt, fill: phys-orange, weight: "bold", [每条 ≈ 小矩形：$F_i dot Delta x$]))
  content((2.6, -0.05), text(size: 8.5pt, fill: gray-line, align(center, [变力做功 = 全部细条面积之和。把「弯的」切成「直的」，#linebreak()加起来——这就是微元法，也是积分的思想原型])))
})
