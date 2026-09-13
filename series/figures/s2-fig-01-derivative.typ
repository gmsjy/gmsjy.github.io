// s2-fig-01-derivative.typ — 割线趋近切线：平均变化率的极限
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let f(x) = x * x / 4 + 0.4
#let px = 2
#let p = (px, f(px))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.6, 4.9, -1.4, 4.7, show-grid: true, x-step: 1, y-step: 1)

  // 曲线
  curve(f, -0.6, 4.45, stroke: 1.4pt + math-blue)
  content((-0.5, 0.9), text(size: 8.5pt, fill: math-blue, [$y = f(x)$]))

  // 割线 P→Q2（远）与 P→Q1（近），颜色由浅到深
  let q1 = (3.2, f(3.2))
  let q2 = (4.0, f(4.0))
  line(p, q2, stroke: (paint: gray-line.lighten(35%), thickness: 0.9pt, dash: "dashed"))
  line((0.6, f(px) - 1.5 * (f(4.0) - f(px)) / 2), q2, stroke: (paint: gray-line.lighten(35%), thickness: 0.9pt, dash: "dashed"))
  line((1.2, f(px) - (f(3.2) - f(px)) / 1.2 * 0.8), q1, stroke: (paint: gray-line, thickness: 1pt, dash: "dashed"))
  circle(q1, radius: 0.06, fill: gray-line, stroke: none)
  circle(q2, radius: 0.06, fill: gray-line, stroke: none)
  content((3.28, f(3.2) + 0.12), text(size: 8pt, fill: gray-line, [$Q_1$]), anchor: "west")
  content((4.06, f(4.0) + 0.1), text(size: 8pt, fill: gray-line, [$Q_2$]), anchor: "west")

  // 切线：过 P、斜率 f'(2) = 1
  line((0.3, -0.3), (4.35, 3.75), stroke: 1.4pt + phys-orange)
  content((4.4, 3.85), text(size: 8.5pt, fill: phys-orange, weight: "bold", [切线]), anchor: "west")

  // Δx / Δy 标注（以外侧割线 Q2 为例）
  line(p, (4.0, 1.4), stroke: (paint: gray-line, thickness: 0.5pt, dash: "dashed"))
  line((4.0, 1.4), q2, stroke: (paint: gray-line, thickness: 0.5pt, dash: "dashed"))
  content((3.0, 1.18), text(size: 8pt, fill: gray-line, [$Delta x$]))
  content((4.22, 2.8), text(size: 8pt, fill: gray-line, [$Delta y$]), anchor: "west")

  // P 点
  circle(p, radius: 0.07, fill: ink, stroke: none)
  content((1.8, 1.12), text(size: 8.5pt, fill: ink, weight: "bold", [$P$]))

  // 极限定义
  node((0.95, 4.1), [$k = lim_(Delta x -> 0) (f(x_0 + Delta x) - f(x_0)) / Delta x = f'(x_0)$],
    w: 4.8, h: 0.85, fill: soft-gray.lighten(40%), stroke: gray-line, text-size: 8pt)
})
