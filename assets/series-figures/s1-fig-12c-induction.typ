// s1-fig-12c-induction.typ — 电磁感应：磁铁插拔线圈，电流表偏转
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 条形磁铁（N 端朝下插入）
  rect((2.5, 4.4), (3.5, 5.1), fill: soft-red, stroke: ink)
  content((3.0, 4.75), text(size: 8.5pt, fill: warn-red, weight: "bold", [N]))
  vector((3.0, 4.15), (3.0, 3.6), paint: warn-red, thickness: 1.2pt, scale: 0.6, label: [插入], label-off: (0.25, -0.05), label-size: 7.8pt)
  // 线圈（多匝椭圆剖面）
  for i in range(4) {
    let x = 2.2 + 0.55 * i
    content((x, 2.6), text(size: 0pt, []))
  }
  for i in range(5) {
    let cx = 2.15 + 0.5 * i
    let pts = range(25).map(k => { let t = 360deg * k / 24; (cx + 0.18 * calc.cos(t), 2.6 + 0.75 * calc.sin(t)) })
    line(..pts, stroke: 1pt + ink)
  }
  // 引线到电流表
  wire((2.15, 1.85), (2.15, 1.2))
  wire((2.15, 1.2), (5.4, 1.2))
  wire((4.3, 1.85), (4.3, 1.55))
  wire((4.3, 1.55), (6.9, 1.55))
  wire((6.9, 1.55), (6.9, 1.2))
  circuit-meter((6.15, 1.2), sym: $G$, r: 0.34)
  wire((5.4, 1.2), (5.81, 1.2))
  wire((6.49, 1.2), (6.9, 1.2))
  content((4.6, 0.3), text(size: 8.5pt, fill: gray-line, align(center, [磁通量变化 → 感应电流：插入偏转、拔出反向、#linebreak()停住不动就归零（电磁感应现象）])))
})
