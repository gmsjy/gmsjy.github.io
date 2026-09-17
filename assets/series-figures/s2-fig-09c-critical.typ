// s2-fig-09c-critical.typ — 动力学临界条件全景卡
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((7.0, 5.0), [动力学综合题的六个「临界」常客], w: 7.6, h: 0.95, fill: phys-orange, stroke: none, text-fill: white, weight: "bold", text-size: 10pt)
  let items = (
    (x: 2.0, y: 3.3, t: [落地瞬间#linebreak()速度方向切换], fill: soft-blue, st: math-blue),
    (x: 5.35, y: 3.3, t: [共速瞬间#linebreak()摩擦力反转], fill: soft-blue, st: math-blue),
    (x: 8.7, y: 3.3, t: [弹簧最短#linebreak()两者共速、势能最大], fill: soft-orange, st: phys-orange),
    (x: 12.05, y: 3.3, t: [恰好分离#linebreak()弹力为零], fill: soft-orange, st: phys-orange),
    (x: 3.7, y: 1.0, t: [恰好到某点#linebreak()该点速度为零], fill: soft-green, st: green-ok),
    (x: 10.3, y: 1.0, t: [最终停止#linebreak()能量耗尽的位置], fill: soft-green, st: green-ok),
  )
  for it in items {
    node((it.x, it.y), it.t, w: 3.0, h: 1.15, fill: it.fill, stroke: it.st, text-size: 8.3pt)
  }
  content((7.0, -0.15), text(size: 8.5pt, fill: gray-line, align(center, [压轴设问几乎全部从这六条里出——每条都对应一个「令某量恰好为零」的方程])))
})
