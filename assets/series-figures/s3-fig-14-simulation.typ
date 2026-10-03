// s3-fig-14-simulation.typ — 仿真流程：从物理到代码
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let steps = (
    (x: 1.7, t: [① 物理模型], d: [忽略什么、保留什么], fill: soft-blue, st: math-blue),
    (x: 5.1, t: [② 微分方程], d: [$dif v / dif t = F / m$], fill: soft-blue, st: math-blue),
    (x: 8.5, t: [③ 离散化], d: [$v += a Delta t$], fill: soft-orange, st: phys-orange),
    (x: 11.9, t: [④ 循环计算], d: [每步推进 $Delta t$], fill: soft-green, st: green-ok),
  )
  for s in steps {
    node((s.x, 2.6), s.t, w: 2.7, h: 1.15, fill: s.fill, stroke: s.st, text-size: 8.5pt)
    content((s.x, 1.65), text(size: 7.5pt, fill: gray-line, s.d))
  }
  arrow((3.1, 2.6), (3.7, 2.6), paint: gray-line, thickness: 1.2pt)
  arrow((6.5, 2.6), (7.1, 2.6), paint: gray-line, thickness: 1.2pt)
  arrow((9.9, 2.6), (10.5, 2.6), paint: gray-line, thickness: 1.2pt)
  content((6.8, 4.2), text(size: 9.5pt, fill: ink, weight: "bold", [数值仿真 = 会计算的物理题]))
  content((6.8, 0.4), text(size: 8.5pt, fill: gray-line, align(center, [计算机每秒做百万次「一小步」，弯的轨迹、变的力量，#linebreak()全部变成加法——物理直觉 + 离散化 + 循环，就是仿真的全部秘密])))
})
