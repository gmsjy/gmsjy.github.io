// s1-fig-05b-mapping.typ — 变量说 → 对应说：函数定义的升级
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 左：初中的「变量说」——一条 x 轴上的 y 随 x 变化
  node((2.4, 4.4), [初中的变量说], w: 3.4, h: 0.7, fill: soft-blue, stroke: math-blue, text-size: 9pt, weight: "bold")
  axes(0.6, 4.4, -0.4, 2.6, x-step: 1, y-step: 1, x-label: $x$, y-label: $y$, tick-labels: false)
  curve(x => 0.25 * (x - 2.5) * (x - 2.5) + 0.1, 0.7, 4.3, stroke: 1.3pt + math-blue)
  content((2.4, -0.15), text(size: 8pt, fill: gray-line, [「$y$ 随 $x$ 的变化而变化」]))

  // 中：箭头
  arrow((5.3, 2.2), (6.9, 2.2), paint: phys-orange, thickness: 1.6pt, label: [升级], label-off: (0, 0.35))

  // 右：高中的「对应说」——两个集合之间的映射
  node((11.3, 4.4), [高中的对应说], w: 3.6, h: 0.7, fill: soft-orange, stroke: phys-orange, text-size: 9pt, weight: "bold")
  circle((8.6, 2.0), radius: 1.15, stroke: 1pt + math-blue)
  content((8.6, 3.0), text(size: 8.5pt, fill: math-blue, weight: "bold", [集合 $A$]))
  circle((12.6, 2.0), radius: 1.15, stroke: 1pt + phys-orange)
  content((12.6, 3.0), text(size: 8.5pt, fill: phys-orange, weight: "bold", [集合 $B$]))
  let arr = ((8.15, 2.5, 12.15, 2.55), (8.0, 1.95, 12.0, 1.95), (8.15, 1.45, 12.15, 1.4))
  for a in arr { arrow((a.at(0), a.at(1)), (a.at(2), a.at(3)), paint: gray-line, thickness: 0.8pt, scale: 0.5) }
  for p in ((8.6, 2.5), (8.45, 1.95), (8.6, 1.4)) { circle(p, radius: 0.06, fill: math-blue, stroke: none) }
  for p in ((12.1, 2.55), (12.0, 1.95), (12.1, 1.4)) { circle(p, radius: 0.06, fill: phys-orange, stroke: none) }
  content((8.05, 0.35), text(size: 8pt, fill: gray-line, align(center, [每个输入都有唯一输出#linebreak()——对应关系本身就是函数])))
})
