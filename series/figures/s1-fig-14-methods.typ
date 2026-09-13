// s1-fig-14-methods.typ — 四大思想：一个中心，数学物理两边开花
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  node((7.0, 5.0), [数理共通的四大思想], w: 5.4, h: 0.95, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 11pt)
  let items = (
    (x: 2.2, y: 2.9, t: [模型思想], m: [函数建模、数列模型], p: [质点、点电荷、轻绳], fill: soft-blue, st: math-blue),
    (x: 11.8, y: 2.9, t: [极限思想], m: [$lim$、切线、渐进线], p: [瞬时速度、微元求和], fill: soft-orange, st: phys-orange),
    (x: 2.2, y: 0.6, t: [对称思想], m: [偶函数、圆的对称性], p: [镜像电荷、对称受力], fill: soft-green, st: green-ok),
    (x: 11.8, y: 0.6, t: [守恒思想], m: [恒等式、总量不变], p: [能量、动量、电荷守恒], fill: soft-gray, st: gray-line),
  )
  for it in items {
    arrow((7.0, 4.5), (it.x, it.y + 0.75), paint: gray-line, thickness: 1pt)
    node((it.x, it.y), it.t, w: 3.6, h: 0.8, fill: it.fill, stroke: it.st, text-size: 10pt, weight: "bold")
    node((it.x - 1.0, it.y - 1.35), [数学：#it.m], w: 4.4, h: 0.85, fill: white, stroke: math-blue, text-size: 7.5pt)
    node((it.x + 3.7, it.y - 1.35), [物理：#it.p], w: 4.6, h: 0.85, fill: white, stroke: phys-orange, text-size: 7.5pt)
  }
})
