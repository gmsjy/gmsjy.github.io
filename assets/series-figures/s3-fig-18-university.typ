// s3-fig-18-university.typ — 高中到大学的知识体系地图
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((7.0, 1.0), [高中数理：语言与直觉的地基], w: 6.8, h: 1.0, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 10pt)
  let pillars = (
    (x: 2.3, t: [数学系#linebreak()分析 · 代数 · 概率], fill: soft-blue, st: math-blue),
    (x: 7.0, t: [物理系#linebreak()力 · 电 · 量子], fill: soft-orange, st: phys-orange),
    (x: 11.7, t: [工程与计算机#linebreak()算法 · 系统 · 优化], fill: soft-green, st: green-ok),
  )
  for p in pillars {
    arrow((7.0, 1.6), (p.x, 2.7), paint: gray-line, thickness: 1.2pt)
    node((p.x, 3.3), p.t, w: 4.2, h: 1.2, fill: p.fill, stroke: p.st, text-size: 8.8pt)
  }
  content((2.3, 5.3), text(size: 8.5pt, fill: math-blue, weight: "bold", align(center, [分析 = 极限语言升级])))
  content((7.0, 5.3), text(size: 8.5pt, fill: phys-orange, weight: "bold", align(center, [普物 = 微积分重讲物理])))
  content((11.7, 5.3), text(size: 8.5pt, fill: green-ok, weight: "bold", align(center, [算法 = 递推与离散])))
  content((7.0, 0.05), text(size: 8.5pt, fill: gray-line, align(center, [高中练的每一种思想（模型/微元/对称/守恒），到了大学都会以更精确的语言重逢])))
})
