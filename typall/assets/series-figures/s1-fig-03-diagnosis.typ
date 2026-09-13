// s1-fig-03-diagnosis.typ — 五维能力雷达：初中入学水平 vs 高中要求
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let dims = ([运算], [理解], [建模], [推理], [实验])
#let ring(r) = range(5).map(i => {
  let a = 90deg + 72deg * i
  (2.6 + r * 1.85 * calc.cos(a), 2.75 + r * 1.85 * calc.sin(a))
})

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 参考环
  for r in (0.33, 0.66, 1.0) {
    line(..ring(r), close: true, stroke: (paint: soft-gray, thickness: 0.5pt))
  }
  // 轴
  for i in range(5) {
    let a = 90deg + 72deg * i
    line((2.6, 2.75), (2.6 + 1.85 * calc.cos(a), 2.75 + 1.85 * calc.sin(a)), stroke: 0.5pt + soft-gray)
    content((2.6 + 2.25 * calc.cos(a), 2.75 + 2.25 * calc.sin(a)), text(size: 9pt, fill: ink, weight: "bold", dims.at(i)))
  }
  // 高中要求（蓝面）
  line(..ring(0.92), close: true, fill: soft-blue.lighten(30%), stroke: 1.2pt + math-blue)
  // 初中毕业典型水平（橙虚线）
  let lv = (0.85, 0.62, 0.38, 0.45, 0.35)
  let pts = range(5).map(i => {
    let a = 90deg + 72deg * i
    (2.6 + lv.at(i) * 1.85 * calc.cos(a), 2.75 + lv.at(i) * 1.85 * calc.sin(a))
  })
  line(..pts, close: true, stroke: (paint: phys-orange, thickness: 1.2pt, dash: "dashed"))
  for p in pts { circle(p, radius: 0.06, fill: phys-orange, stroke: none) }
  // 图例
  line((5.6, 4.4), (6.15, 4.4), stroke: 1.2pt + math-blue)
  content((6.25, 4.4), text(size: 8pt, fill: math-blue, [高中要求]), anchor: "west")
  line((5.6, 3.9), (6.15, 3.9), stroke: (paint: phys-orange, thickness: 1.2pt, dash: "dashed"))
  content((6.25, 3.9), text(size: 8pt, fill: phys-orange, [初中毕业典型水平]), anchor: "west")
  content((5.6, 3.3), text(size: 7.8pt, fill: gray-line, [虚线与实线之间的缺口，]), anchor: "west")
  content((5.6, 2.9), text(size: 7.8pt, fill: gray-line, [就是你第一学期的爬坡量]), anchor: "west")
})
