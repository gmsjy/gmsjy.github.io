// s2-fig-18-project.typ — 强基启蒙：微积分初步 + 项目流程
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  // 左：微积分初步——切线斜率即导数
  axes(-0.3, 4.4, -0.3, 3.6, x-step: 1, y-step: 1, tick-labels: false)
  curve(x => 0.25 * x * x, 0, 3.5, stroke: 1.3pt + math-blue)
  line((1.4, 0.35), (3.1, 1.75), stroke: 1.2pt + phys-orange)
  circle((2.0, 1.0), radius: 0.07, fill: warn-red, stroke: none)
  content((2.15, 1.3), text(size: 8pt, fill: ink, [$P$]))
  content((2.9, 2.0), text(size: 8pt, fill: phys-orange, [切线]))
  content((2.2, 3.3), text(size: 8.5pt, fill: gray-line, [导数 = 切线斜率]))
  // 右：项目流程
  let steps = (([选题], soft-blue, math-blue), ([设计], soft-orange, phys-orange), ([执行], soft-green, green-ok), ([答辩], soft-gray, gray-line))
  for i in range(4) {
    node((7.6 + calc.rem(i, 2) * 3.6, 3.0 - calc.floor(i / 2) * 1.5), steps.at(i).at(0), w: 2.2, h: 0.95, fill: steps.at(i).at(1), stroke: steps.at(i).at(2), text-size: 9.5pt, weight: "bold")
  }
  arrow((8.75, 3.0), (9.75, 3.0), paint: gray-line, thickness: 1.1pt)
  arrow((11.45, 2.45), (11.45, 1.85), paint: gray-line, thickness: 1.1pt)
  arrow((9.75, 1.5), (8.75, 1.5), paint: gray-line, thickness: 1.1pt)
  content((10.4, 0.3), text(size: 8.5pt, fill: gray-line, align(center, [项目式学习：研究一个真实问题，#linebreak()产出一份可答辩的报告])))
  content((2.1, 0.3), text(size: 8.5pt, fill: gray-line, align(center, [微积分是大学数理的通行语言，#linebreak()强基笔试的常客])))
})
