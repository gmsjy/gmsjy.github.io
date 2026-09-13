// s3-fig-11-strong-math.typ — 微积分基本定理：求导与积分互逆
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 左：面积（积分）
  axes(-0.3, 4.4, -0.3, 3.9, x-step: 1, y-step: 1, x-label: $x$, y-label: $f(x)$, tick-labels: false)
  curve(x => 0.3 * x * x + 0.4, 0, 3.8, stroke: 1.4pt + math-blue)
  let band = range(31).map(i => { let t = 1.0 + 2.0 * i / 30; (t, 0.3 * t * t + 0.4) })
  line((1.0, 0), ..band, (3.0, 0), close: true, fill: soft-orange, stroke: none)
  content((2.0, 1.0), text(size: 8.5pt, fill: phys-orange, weight: "bold", [面积 $= F(b) - F(a)$]))
  line((1, 0), (1, 0.7), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  line((3, 0), (3, 3.1), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  content((1, -0.4), text(size: 8pt, fill: ink, [$a$]))
  content((3, -0.4), text(size: 8pt, fill: ink, [$b$]))
  content((3.7, 3.4), text(size: 8.5pt, fill: gray-line, align(center, [积分 = 曲线下面积，#linebreak()求和思想的正式化])))
  // 右：导数（切线）
  axes(5.4, 10.1, -0.3, 3.9, x-step: 1, y-step: 1, x-label: $x$, y-label: $F'(x)$, tick-labels: false)
  curve(x => 0.6 * x + 0.2, 5.6, 9.8, stroke: 1.4pt + phys-orange)
  circle((7.0, 0.6 * 7.0 - 5.4 + 0.4), radius: 0, stroke: none)
  line((6.2, 1.6), (8.2, 2.8), stroke: 1.2pt + green-ok)
  content((8.05, 3.05), text(size: 8.5pt, fill: green-ok, weight: "bold", [切线斜率 = $F'(x)$]), anchor: "west")
  content((9.9, 0.4), text(size: 8.5pt, fill: gray-line, align(center, [导数 = 瞬时变化率，#linebreak()差商思想的正式化])), anchor: "east")
})
