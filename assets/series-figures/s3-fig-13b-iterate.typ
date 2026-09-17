// s3-fig-13b-iterate.typ — 模型迭代：拟合从粗到细
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 5.4, -0.3, 4.4, x-step: 1, y-step: 1, x-label: [$t$/h], y-label: $y$, tick-labels: false)
  let pts = ((0.5, 0.9, 0.2), (1.4, 1.9, 0.18), (2.3, 2.4, 0.2), (3.2, 2.65, 0.18), (4.1, 2.8, 0.22), (5.0, 2.9, 0.2))
  scatter-error(pts, dot: math-blue, bar: gray-line)
  // 粗模型：直线
  fit-line(0.62, 0.5, 0.2, 5.1, paint: gray-line, thickness: 1pt, dash: "dashed")
  // 细模型：饱和曲线
  curve(x => 3.1 * (1 - calc.exp(-x / 1.5)), 0, 5.2, stroke: 1.4pt + warn-red)
  content((3.9, 1.05), text(size: 8.5pt, fill: warn-red, weight: "bold", [修正模型（饱和）]))
  content((1.3, 2.5), text(size: 8.5pt, fill: gray-line, [初版（直线）]))
  content((2.7, -0.05), text(size: 8.5pt, fill: gray-line, align(center, [验证不合格 → 怀疑「增长率不变」的假设 → 换成「增长率随接近上限而衰减」#linebreak()→ 重新拟合 → 残差变小。建模的价值在迭代，不在一次成型])))
})
