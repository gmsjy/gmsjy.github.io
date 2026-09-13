// s1-fig-05c-nature.typ — 函数性质导图：一张图挂起高中函数全部话题
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  node((7.0, 4.9), [一个函数，问五件事], w: 4.6, h: 0.9, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 10pt)
  let items = (
    (x: 1.7, y: 2.9, t: [① 定义域#linebreak()x 能取哪些值], ex: [分母 $!= 0$、根号内 $>= 0$]),
    (x: 5.15, y: 2.9, t: [② 对应法则#linebreak()怎么算出输出], ex: [解析式、图像、表格]),
    (x: 8.85, y: 2.9, t: [③ 值域#linebreak()输出能到哪些值], ex: [图像的最高与最低]),
    (x: 12.3, y: 2.9, t: [④ 单调性#linebreak()升降如何变化], ex: [$f' $ 或定义法作差]),
    (x: 7.0, y: 0.8, t: [⑤ 奇偶与对称#linebreak()图像的对称性], ex: [$f(-x) = plus.minus f(x)$]),
  )
  for it in items {
    arrow((7.0, 4.45), (it.x, it.y + 0.65), paint: gray-line, thickness: 0.9pt)
    node((it.x, it.y), it.t, w: 2.9, h: 1.15, fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
    content((it.x, it.y - 1.0), text(size: 7.8pt, fill: gray-line, it.ex))
  }
})
