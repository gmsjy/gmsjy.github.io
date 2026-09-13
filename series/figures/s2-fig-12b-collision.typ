// s2-fig-12b-collision.typ — 三种碰撞：恢复系数从 1 到 0
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  let rows = (
    (y: 4.2, t: [弹性碰撞 $e = 1$], d: [动能不损失（钢球、冰壶）], c1: math-blue, c2: phys-orange, v1: 1.6, v2: -1.6),
    (y: 2.4, t: [非弹性碰撞 $0 < e < 1$], d: [损失一部分动能（橡皮球）], c1: green-ok, c2: green-ok, v1: 1.2, v2: -1.2),
    (y: 0.6, t: [完全非弹性 $e = 0$], d: [共速而行，损失最多（粘一起）], c1: warn-red, c2: warn-red, v1: 0, v2: 0),
  )
  for r in rows {
    content((2.4, r.y + 0.55), text(size: 8.5pt, fill: r.c1, weight: "bold", r.t))
    content((2.4, r.y - 0.5), text(size: 7.8pt, fill: gray-line, r.d))
    // 碰前
    circle((5.6, r.y), radius: 0.3, fill: white, stroke: 1.1pt + ink)
    vector((4.5, r.y), (5.28, r.y), paint: math-blue, thickness: 1pt, scale: 0.55)
    content((4.75, r.y + 0.28), text(size: 7pt, fill: math-blue, [$v$]))
    circle((6.7, r.y), radius: 0.3, fill: white, stroke: 1.1pt + ink)
    // 碰后速度
    vector((7.6, r.y), (7.6 + r.v1, r.y), paint: r.c1, thickness: 1.2pt, scale: 0.6)
    vector((9.0, r.y), (9.0 + r.v2, r.y), paint: r.c2, thickness: 1.2pt, scale: 0.6)
    content((11.6, r.y), text(size: 7.8pt, fill: gray-line, [碰后两球速度]))
  }
  content((8.0, -0.75), text(size: 8.5pt, fill: ink, weight: "bold", align(center, [无论哪种碰撞，动量都守恒；区别只在动能的「损耗率」。#linebreak()恢复系数 $e = |v'_2 - v'_1| / (v_1 - v_2)$ 量化分离速度与接近速度之比])))
})
