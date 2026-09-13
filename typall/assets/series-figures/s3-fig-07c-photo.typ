// s3-fig-07c-photo.typ — 光电效应：E_k–ν 直线的截距与斜率
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-1.9, 4.9, -2.4, 3.4, x-step: 1, y-step: 1, x-label: [$nu$（频率）], y-label: $E_k$, tick-labels: false)
  line((1, 0), (4.3, 3.3), stroke: 1.5pt + math-blue)
  line((1, -2.2), (1, 0), stroke: (paint: warn-red, thickness: 1.2pt, dash: "dashed"))
  content((1.15, -1.3), text(size: 8.5pt, fill: warn-red, weight: "bold", [$|W_0|$]), anchor: "west")
  line((1, 0), (4.3, 0), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  circle((1, 0), radius: 0.07, fill: warn-red, stroke: none)
  content((1.15, 0.3), text(size: 8.5pt, fill: warn-red, weight: "bold", [截止频率 $nu_0$]), anchor: "west")
  content((3.6, 2.9), text(size: 8.5pt, fill: ink, align(center, [斜率 = 普朗克常量 $h$])))
  content((2.6, -1.9), text(size: 8.5pt, fill: gray-line, align(center, [$E_k = h nu - W_0$：频率不够，光再强也无电子——#linebreak()光电效应是「光的粒子性」的判决实验])))
})
