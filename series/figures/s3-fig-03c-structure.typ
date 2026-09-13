// s3-fig-03c-structure.typ — 数列压轴三件套：结构决定解法
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((7.0, 4.7), [拿到递推式，先认结构], w: 5.4, h: 0.9, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 10pt)
  let rows = (
    (x: 2.4, t: [$a_(n+1) = p a_n + q$#linebreak()待定系数构造等比], d: [补常数成 $b_(n+1) = p b_n$]),
    (x: 7.0, t: [$a_(n+1) = p a_n + f(n)$#linebreak()累加 / 累乘], d: [望远镜求和]),
    (x: 11.6, t: [$a_(n+1) = f(a_n)$#linebreak()不动点 + 单调性], d: [蛛网图定位放缩靶子]),
  )
  for r in rows {
    arrow((7.0, 4.25), (r.x, 3.0), paint: gray-line, thickness: 1pt)
    node((r.x, 2.5), r.t, w: 4.2, h: 1.15, fill: soft-blue, stroke: math-blue, text-size: 8.3pt)
    content((r.x, 1.55), text(size: 7.8pt, fill: gray-line, r.d))
  }
  content((7.0, 0.55), text(size: 8.5pt, fill: gray-line, align(center, [证明环节交 给数学归纳法——「构造出公式」与「证明公式成立」是两问])))
})
