// s3-fig-15b-classify.typ — 分类边界：一条线分开两类样本
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 5.9, -0.3, 4.4, x-step: 1, y-step: 1, x-label: [学习时长], y-label: [得分], tick-labels: false)
  line((0.8, 4.0), (5.2, 0.6), stroke: 1.4pt + warn-red)
  content((5.0, 0.25), text(size: 8pt, fill: warn-red, weight: "bold", [分类边界]), anchor: "west")
  // 及格类（右下）
  for p in ((3.0, 0.9), (3.8, 1.4), (4.5, 0.7), (4.2, 2.0)) {
    circle(p, radius: 0.11, fill: soft-orange, stroke: 1pt + phys-orange)
  }
  // 不及格类（左上）
  for p in ((1.0, 2.6), (1.6, 3.3), (2.2, 2.9), (2.9, 3.6)) {
    circle(p, radius: 0.11, fill: soft-blue, stroke: 1pt + math-blue)
  }
  content((4.35, 1.35), text(size: 8pt, fill: phys-orange, weight: "bold", [及格]), anchor: "west")
  content((1.0, 3.85), text(size: 8pt, fill: math-blue, weight: "bold", [不及格]), anchor: "west")
  content((3.0, 4.15), text(size: 8.5pt, fill: gray-line, align(center, [新样本落在边界哪一侧，就归入哪一类——#linebreak()这是分类算法（感知机、支持向量机）的核心思想])))
})
