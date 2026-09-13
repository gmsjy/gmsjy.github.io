// s1-fig-09-model.typ — 实物 → 质点：抽象的尺度规则
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 火车过桥（不能看质点）
  rect((0.6, 2.6), (4.0, 3.3), fill: soft-blue, stroke: ink)
  for i in range(4) { circle((0.95 + i * 0.85, 2.45), radius: 0.14, fill: soft-gray, stroke: ink) }
  line((0, 2.25), (5.2, 2.25), stroke: 1pt + ink)
  content((2.3, 3.7), text(size: 8.5pt, fill: ink, weight: "bold", [火车过桥：车长不能忽略]))
  content((2.3, 1.85), text(size: 7.8pt, fill: warn-red, [✗ 不能视为质点]))
  // 地球公转（能看质点）
  circle((8.6, 2.95), radius: 0.55, fill: soft-orange, stroke: ink)
  content((8.6, 2.95), text(size: 7.5pt, fill: ink, [地]))
  line((6.6, 0.9), (10.6, 0.9), stroke: 0.9pt + gray-line, mark: (end: ">>", fill: gray-line, scale: 0.5))
  content((10.35, 1.25), text(size: 8pt, fill: gray-line, [公转轨道]))
  content((8.6, 3.7), text(size: 8.5pt, fill: ink, weight: "bold", [地球公转：轨道半径 ≈ 1.5 亿 km]))
  content((8.6, 0.25), text(size: 7.8pt, fill: green-ok, [✓ 可以视为质点]))
  // 规则
  content((5.8, 5.0), text(size: 9.5pt, fill: math-blue, weight: "bold", align(center, [判断标准只有一个：#h(6pt)对象的形状大小#linebreak()在所研究的问题里是否可以忽略])))
})
