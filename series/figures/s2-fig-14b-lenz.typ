// s2-fig-14b-lenz.typ — 楞次定律：来拒去留的三种时刻
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  let scenes = ((1.5, [插入：磁通增，感应电流「拒」], warn-red), (6.4, [静止：磁通不变，无感应], gray-line), (11.3, [拔出：磁通减，感应电流「留」], math-blue))
  for s in scenes {
    let x = s.at(0)
    // 线圈
    for i in range(3) {
      let cx = x + 0.5 * i
      let pts = range(21).map(k => { let t = 360deg * k / 20; (cx + 0.16 * calc.cos(t), 2.4 + 0.62 * calc.sin(t)) })
      line(..pts, stroke: 1pt + ink)
    }
    // 磁铁 N
    rect((x + 0.35, 3.55), (x + 1.15, 4.15), fill: soft-red, stroke: ink)
    content((x + 0.75, 3.85), text(size: 7.5pt, fill: warn-red, weight: "bold", [N]))
    circle((x + 1.35, 2.4), radius: 0.1, fill: ink, stroke: none)
    content((x + 0.75, 5.0), text(size: 7.8pt, fill: s.at(2), weight: "bold", align(center, s.at(1))))
  }
  // 各状态磁铁位置
  line((1.95, 3.5), (1.95, 3.3), stroke: 1pt + warn-red)
  arrow((2.7, 4.3), (2.7, 3.6), paint: warn-red, thickness: 1.1pt, scale: 0.6)
  arrow((7.6, 3.6), (7.6, 4.0), paint: gray-line, thickness: 0.9pt, scale: 0.5)
  line((11.95, 3.55), (11.95, 3.75), stroke: 1pt + math-blue)
  arrow((12.45, 3.35), (12.45, 4.0), paint: math-blue, thickness: 1.1pt, scale: 0.6)
  content((7.0, 0.5), text(size: 9pt, fill: ink, weight: "bold", align(center, [感应电流的效果永远*阻碍*磁通量的变化——「来拒去留」])))
})
