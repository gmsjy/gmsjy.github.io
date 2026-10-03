// s1-fig-07c-balance.typ — 建模三级：天平 → 方程 → 应用
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 天平
  let ox = 2.4
  let oy = 2.2
  line((ox - 1.7, oy + 0.8), (ox + 1.7, oy + 0.8), stroke: 1.2pt + ink)
  line((ox - 0.3, oy), (ox + 0.3, oy), (ox, oy + 0.78), close: true, fill: soft-gray, stroke: ink)
  line((ox - 1.7, oy + 0.8), (ox - 1.7, oy + 0.35), stroke: 0.6pt + gray-line)
  line((ox + 1.7, oy + 0.8), (ox + 1.7, oy + 0.35), stroke: 0.6pt + gray-line)
  rect((ox - 2.05, oy - 0.05), (ox - 1.35, oy + 0.35), fill: soft-blue, stroke: math-blue)
  rect((ox + 1.35, oy - 0.05), (ox + 2.05, oy + 0.35), fill: soft-orange, stroke: phys-orange)
  content((ox - 1.7, oy + 0.15), text(size: 8pt, fill: math-blue, weight: "bold", [$3x$]))
  content((ox + 1.7, oy + 0.15), text(size: 8pt, fill: phys-orange, weight: "bold", [$2x + 3$]))
  node((2.4, 0.5), [天平平衡 = 等量关系], w: 4.2, h: 0.7, fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
  // 流程
  node((8.3, 2.6), [列方程#linebreak()$3x = 2x + 3$], w: 3.4, h: 1.2, fill: soft-orange, stroke: phys-orange, text-size: 8.5pt)
  node((12.6, 2.6), [解与检验#linebreak()$x = 3$，代回验证], w: 3.4, h: 1.2, fill: soft-green, stroke: green-ok, text-size: 8.5pt)
  arrow((4.6, 2.6), (6.55, 2.6), paint: gray-line, thickness: 1.2pt)
  arrow((10.05, 2.6), (10.85, 2.6), paint: gray-line, thickness: 1.2pt)
  content((8.55, 0.9), text(size: 8.5pt, fill: gray-line, align(center, [高中升级版：等量关系可能藏在变化率、守恒律里，#linebreak()但「找平衡」这套动作一模一样])))
})
