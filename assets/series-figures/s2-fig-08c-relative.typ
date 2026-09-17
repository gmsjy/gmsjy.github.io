// s2-fig-08c-relative.typ — 小船渡河：分运动与合运动
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.4, 8.4, -0.4, 5.4, x-step: 1, y-step: 1, tick-labels: false)
  // 河岸
  line((0, 0.3), (8, 0.3), stroke: 1.2pt + math-blue)
  line((0, 4.8), (8, 4.8), stroke: 1.2pt + math-blue)
  content((7.6, 5.15), text(size: 8pt, fill: math-blue, [对岸]))
  content((7.6, -0.1), text(size: 8pt, fill: math-blue, [出发点]))
  // 船与速度三角形
  let o = (1.2, 0.9)
  circle(o, radius: 0.14, fill: ink, stroke: none)
  content((o.at(0) - 0.15, o.at(1) - 0.35), text(size: 8.5pt, fill: ink, weight: "bold", [船]))
  vector(o, (o.at(0), o.at(1) + 2.6), paint: math-blue, thickness: 1.4pt, label: $v_"船"$, label-off: (0.15, 0.05), label-size: 8.5pt)
  vector(o, (o.at(0) + 2.0, o.at(1)), paint: gray-line, thickness: 1.4pt, label: $v_"水"$, label-off: (0.25, -0.05), label-size: 8.5pt)
  vector(o, (o.at(0) + 2.0, o.at(1) + 2.6), paint: warn-red, thickness: 1.6pt, label: $v_"合"$, label-off: (0.2, 0.15), label-size: 8.5pt)
  line((o.at(0), o.at(1) + 2.6), (o.at(0) + 2.0, o.at(1) + 2.6), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  content((5.6, 3.6), text(size: 8.5pt, fill: ink, align(center, [合速度 = 矢量和：#linebreak()船头垂直对岸时，#linebreak()过河时间最短；#linebreak()实际落点偏向下游])))
})
