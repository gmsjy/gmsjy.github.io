// s1-fig-04b-laws.typ — 运算律对照：数字 → 字母 → 含参验证
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((1.15, 0), [运算律], w: 2.1, h: 0.7, fill: gray-line, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((4.6, 0), [数字验证], w: 4.6, h: 0.7, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((9.5, 0), [字母表达], w: 3.4, h: 0.7, fill: phys-orange, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((13.1, 0), [高中用法], w: 3.2, h: 0.7, fill: green-ok, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  let rows = (
    (y: -1.2, n: [交换律], a: [$3 + 5 = 5 + 3$　$2 times 5 = 5 times 2$], b: [$a + b = b + a$], c: [整理含参式时自由重排]),
    (y: -2.4, n: [结合律], a: [$(2 + 3) + 4 = 2 + (3 + 4)$], b: [$(a b) c = a (b c)$], c: [凑项、凑系数简化计算]),
    (y: -3.6, n: [分配律], a: [$2 times (3 + 4) = 2 times 3 + 2 times 4$], b: [$a (b + c) = a b + a c$], c: [去括号、因式分解双向使用]),
  )
  for r in rows {
    node((1.15, r.y), r.n, w: 2.1, h: 1.0, fill: soft-gray, stroke: gray-line, text-size: 9pt, weight: "bold")
    node((4.6, r.y), r.a, w: 4.6, h: 1.0, fill: soft-blue, stroke: math-blue, text-size: 7.8pt)
    node((9.5, r.y), r.b, w: 3.4, h: 1.0, fill: soft-orange, stroke: phys-orange, text-size: 8pt)
    node((13.1, r.y), r.c, w: 3.2, h: 1.0, fill: soft-green, stroke: green-ok, text-size: 7.5pt)
  }
})
