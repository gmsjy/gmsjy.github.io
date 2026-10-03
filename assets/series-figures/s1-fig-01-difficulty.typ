// s1-fig-01-difficulty.typ — 三线折线图：知识量 / 抽象度 / 考试节奏 + 初升高断层带
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let d-know = ((0.8, 2.2), (3.2, 3.2), (5.2, 4.4), (8.4, 7.6), (10.6, 8.6))
#let d-abstract = ((0.8, 1.4), (3.2, 2.0), (5.2, 2.8), (8.4, 7.0), (10.6, 8.0))
#let d-pace = ((0.8, 2.6), (3.2, 3.4), (5.2, 4.2), (8.4, 6.4), (10.6, 9.2))

#let polyline(data, paint, thickness: 1.4pt) = {
  import cetz.draw: line, circle
  line(..data, stroke: (paint: paint, thickness: thickness))
  for p in data { circle(p, radius: 0.07, fill: paint, stroke: none) }
}

#let fig = cetz.canvas(length: 0.85cm, {
  import cetz.draw: line, content, rect, circle
  // 初升高断层带（底色先画，垫在底层）
  rect((6.4, 0), (7.6, 9.2), fill: soft-red, stroke: none)
  line((6.4, 0), (6.4, 9.2), stroke: (paint: warn-red, thickness: 0.7pt, dash: "dashed"))
  line((7.6, 0), (7.6, 9.2), stroke: (paint: warn-red, thickness: 0.7pt, dash: "dashed"))
  content((7.0, 9.75), text(size: 8.5pt, fill: warn-red, weight: "bold", [初升高「断层带」]))
  arrow((7.0, 9.5), (7.0, 9.05), paint: warn-red, thickness: 0.7pt, scale: 0.5)

  axes(0, 11.3, 0, 9.4, show-grid: true, x-step: 2, y-step: 2,
    x-label: none, y-label: [难度 / 强度], tick-labels: false)

  polyline(d-know, math-blue)
  polyline(d-abstract, phys-orange)
  polyline(d-pace, warn-red)

  // 图例
  let legend = ((0.25, [知识量], math-blue), (2.3, [抽象度], phys-orange), (4.35, [考试节奏], warn-red))
  for it in legend {
    line((it.at(0), 8.9), (it.at(0) + 0.55, 8.9), stroke: 1.4pt + it.at(2))
    content((it.at(0) + 0.7, 8.9), text(size: 8pt, fill: it.at(2), it.at(1)), anchor: "west")
  }

  // 横轴阶段刻度
  let stages = ((0.8, [初一]), (3.2, [初二]), (5.2, [初三]), (8.4, [高一]), (10.6, [高二]))
  for s in stages {
    line((s.at(0), -0.12), (s.at(0), 0.12), stroke: 0.8pt + gray-line)
    content((s.at(0), -0.5), text(size: 8.5pt, fill: ink, s.at(1)))
  }

  // 关键注释
  content((9.75, 8.78), text(size: 7.8pt, fill: math-blue, [知识量陡增]))
  arrow((9.7, 8.62), (9.0, 7.85), paint: math-blue, thickness: 0.6pt, scale: 0.45)
  content((9.75, 6.3), text(size: 7.8pt, fill: warn-red, [节奏明显加快]))
  arrow((9.7, 6.45), (8.55, 6.42), paint: warn-red, thickness: 0.6pt, scale: 0.45)
})

#let fig-b = fig
