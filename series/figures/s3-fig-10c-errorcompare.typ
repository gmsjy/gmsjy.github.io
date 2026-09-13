// s3-fig-10c-errorcompare.typ — 真实值 vs 测量值：系统误差的方向
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  axes(-0.3, 4.4, -2.9, 3.4, x-step: 1, y-step: 1, x-label: $I/"A"$, y-label: $U/"V"$, tick-labels: false)
  // 真实线（更陡）
  line((0, 3), (3.5, 0.55), stroke: 1.4pt + green-ok)
  // 测量线（电压表分流导致偏平）
  line((0, 2.7), (3.5, 1.15), stroke: (paint: warn-red, thickness: 1.3pt, dash: "dashed"))
  content((1.5, 2.9), text(size: 8.5pt, fill: green-ok, weight: "bold", [真实线]))
  content((2.6, 1.25), text(size: 8.5pt, fill: warn-red, weight: "bold", [测量线]), anchor: "west")
  line((0, 3), (0.001, 2.72), stroke: 0.8pt + gray-line)
  content((0.35, 2.9), text(size: 7.8pt, fill: gray-line, [纵截距偏低]))
  content((2.0, 0.4), text(size: 8.5pt, fill: gray-line, align(center, [误差来源：电压表的分流使电流表读数#linebreak()小于真实干路电流——「安培计外接」型系统误差])))
})
