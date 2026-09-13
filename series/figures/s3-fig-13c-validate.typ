// s3-fig-13c-validate.typ — 结果验证：预测值与实测值对比
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  axes(-0.3, 6.4, -0.3, 4.4, x-step: 1, y-step: 1, x-label: [试验编号], y-label: [结果], tick-labels: false)
  let pairs = ((1.0, 2.4, 2.2), (2.0, 3.1, 3.4), (3.0, 3.8, 3.7), (4.0, 2.9, 3.0), (5.0, 3.9, 3.6))
  for p in pairs {
    rect((p.at(0) - 0.28, 0), (p.at(0), p.at(1)), fill: soft-blue, stroke: 0.8pt + math-blue)
    rect((p.at(0) + 0.02, 0), (p.at(0) + 0.3, p.at(2)), fill: soft-orange, stroke: 0.8pt + phys-orange)
  }
  content((1.0, 4.0), text(size: 8pt, fill: math-blue, weight: "bold", [模型预测]))
  content((2.35, 4.0), text(size: 8pt, fill: phys-orange, weight: "bold", [实测]))
  content((3.2, -0.05), text(size: 8.5pt, fill: gray-line, align(center, [逐组对比预测与实测：趋势一致 → 模型可用；#linebreak()系统偏差 → 回到假设环节修模。误差要给出量级，不说「差不多」])))
})
