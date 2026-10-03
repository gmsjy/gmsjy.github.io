// s2-fig-16-design.typ — 控制变量实验设计：传感器测加速度
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 气垫导轨 + 小车 + 传感器
  line((0.6, 2.6), (8.8, 2.6), stroke: 1.2pt + ink)
  rect((1.4, 2.75), (3.0, 3.55), fill: soft-blue, stroke: ink)
  content((2.2, 3.15), text(size: 8.5pt, fill: math-blue, weight: "bold", [小车]))
  rect((7.6, 2.6), (8.8, 3.7), fill: soft-gray, stroke: ink)
  content((8.2, 3.15), text(size: 7.5pt, fill: ink, [光电门]))
  // 挂钩与砝码
  line((3.0, 3.0), (5.4, 3.0), stroke: 0.8pt + ink)
  line((5.4, 3.0), (5.4, 1.6), stroke: 0.8pt + ink)
  rect((5.05, 1.0), (5.75, 1.6), fill: soft-orange, stroke: ink)
  content((5.4, 1.3), text(size: 7.5pt, fill: phys-orange, weight: "bold", [钩码]))
  content((4.7, 4.2), text(size: 8.5pt, fill: ink, align(center, [光电门自动计时 → 瞬时速度])))
  // 控制变量表
  content((10.9, 3.9), text(size: 8.5pt, fill: ink, weight: "bold", align(center, [探究 $a$ 与 $F$、$m$ 的关系：#linebreak()])))
  content((10.9, 2.8), text(size: 8pt, fill: math-blue, align(center, [保持 $m$ 不变，#linebreak()改变 $F$（加钩码）→ $a prop F$])))
  content((10.9, 1.55), text(size: 8pt, fill: phys-orange, align(center, [保持 $F$ 不变，#linebreak()改变 $m$（加配重）→ $a prop 1/m$])))
  content((4.7, -0.1), text(size: 8.5pt, fill: gray-line, align(center, [传感器替代人工计时：误差更小、采样更密——实验的数字化升级])))
})
