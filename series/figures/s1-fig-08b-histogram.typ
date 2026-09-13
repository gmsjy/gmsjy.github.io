// s1-fig-08b-histogram.typ — 频数直方图：数据的第一张脸
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let data = ((1.0, 3), (2.0, 7), (3.0, 12), (4.0, 9), (5.0, 5), (6.0, 2))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  axes(0.4, 7.2, 0, 13.6, x-step: 1, y-step: 4, x-label: [得分], y-label: [频数])
  for d in data {
    rect((d.at(0) - 0.38, 0), (d.at(0) + 0.38, d.at(1)), fill: soft-blue, stroke: (paint: math-blue, thickness: 0.9pt))
    content((d.at(0), d.at(1) + 0.38), text(size: 8pt, fill: math-blue, weight: "bold", str(d.at(1))))
  }
  content((3.8, -1.1), text(size: 8.5pt, fill: gray-line, [单峰、略右偏：多数数据挤在 3 分附近，右侧拖一条小尾巴]))
})
