// s2-fig-10c-orbit.typ — 卫星轨道：万有引力 = 向心力
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 地球
  circle((4.5, 2.6), radius: 1.0, fill: soft-blue, stroke: 1.2pt + ink)
  content((4.5, 2.6), text(size: 9pt, fill: math-blue, weight: "bold", [地球]))
  // 两条轨道
  circle((4.5, 2.6), radius: 2.0, stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  circle((4.5, 2.6), radius: 3.2, stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  circle((4.5 + 2.0, 2.6), radius: 0.1, fill: warn-red, stroke: none)
  circle((4.5 + 3.2, 2.6 + 2.3), radius: 0.1, fill: warn-red, stroke: none)
  content((6.9, 2.2), text(size: 8pt, fill: ink, [近轨：线速度大]))
  content((8.9, 5.4), text(size: 8pt, fill: ink, [远轨：周期长]))
  // 公式
  content((6.7, 0.6), text(size: 9.5pt, fill: ink, weight: "bold", align(center, [$G (M m) / r^2 = m v^2 / r$])))
  content((6.7, -0.4), text(size: 8.5pt, fill: gray-line, align(center, [轨道越高，$v$ 越小、$T$ 越大——「高轨低速大周期」])))
})
