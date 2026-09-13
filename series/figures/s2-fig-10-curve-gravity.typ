// s2-fig-10-curve-gravity.typ — 平抛：水平匀速 + 竖直自由落体
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  axes(-0.4, 6.4, -3.9, 0.8, x-step: 1, y-step: 1, x-label: $x/"m"$, y-label: $y/"m"$)
  // 抛出点
  circle((0, 0), radius: 0.1, fill: ink, stroke: none)
  content((0.35, -0.3), text(size: 8.5pt, fill: ink, weight: "bold", [抛出]))
  // 轨迹 x² = -y（取 y = -0.25x²）
  curve(x => -0.25 * x * x, 0, 3.6, stroke: 1.4pt + math-blue)
  // 三个等时点的水平位移与竖直位移
  for (i, t) in ((0, 1), (1, 2), (2, 3)) {
    let x = i * 1.2
    let y = -0.25 * x * x
    circle((x, y), radius: 0.08, fill: warn-red, stroke: none)
    line((x, 0), (x, y), stroke: (paint: phys-orange, thickness: 0.8pt, dash: "dashed"))
    content((x + 0.05, y - 0.42), text(size: 7.5pt, fill: gray-line, [$y_"落" prop t^2$]))
  }
  // 水平刻度（等间距）
  for i in range(4) {
    content((i * 1.2, 0.35), text(size: 7.5pt, fill: math-blue, [等间距]))
  }
  content((4.9, -3.4), text(size: 8.5pt, fill: gray-line, align(center, [水平方向等间距（匀速），竖直方向按 $t^2$ 落下——#linebreak()两条分运动互不干扰，轨迹是它们的合成])))
})
