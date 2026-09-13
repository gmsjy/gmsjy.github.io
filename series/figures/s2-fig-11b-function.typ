// s2-fig-11b-function.typ — 功能关系对照表：每个力对应一笔账
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((1.9, 0), [做功的力], w: 2.6, h: 0.75, fill: gray-line, stroke: none, text-fill: white, weight: "bold", text-size: 9pt)
  node((6.9, 0), [对应的「账本条目」], w: 6.4, h: 0.75, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 9pt)
  node((12.4, 0), [高中叫法], w: 3.4, h: 0.75, fill: phys-orange, stroke: none, text-fill: white, weight: "bold", text-size: 9pt)
  let rows = (
    (y: -1.25, a: [重力做功], b: [重力势能减少 $m g (h_1 - h_2)$], c: [重力势能]),
    (y: -2.55, a: [弹簧弹力做功], b: [弹性势能减少], c: [弹性势能]),
    (y: -3.85, a: [合力做功], b: [动能增加 $(1/2) m v^2$ 变化], c: [动能定理]),
    (y: -5.15, a: [滑动摩擦做功], b: [系统内能增加 $Q = f s_"相"$], c: [摩擦生热]),
  )
  for r in rows {
    node((1.9, r.y), r.a, w: 2.6, h: 1.15, fill: soft-gray, stroke: gray-line, text-size: 8.5pt, weight: "bold")
    node((6.9, r.y), r.b, w: 6.4, h: 1.15, fill: soft-blue, stroke: math-blue, text-size: 8.3pt)
    node((12.4, r.y), r.c, w: 3.4, h: 1.15, fill: soft-orange, stroke: phys-orange, text-size: 8.5pt)
  }
})
