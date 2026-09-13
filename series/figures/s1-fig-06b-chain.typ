// s1-fig-06b-chain.typ — 证明链：从已知到结论的因果链
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let steps = (
    (x: 1.9, t: [已知#linebreak()AB = DC，∠ABC = ∠DCB], fill: soft-gray, st: gray-line),
    (x: 5.9, t: [判定#linebreak()SAS：两边及夹角对应相等], fill: soft-blue, st: math-blue),
    (x: 9.9, t: [结论 ①#linebreak()△ABC ≌ △DCB], fill: soft-orange, st: phys-orange),
    (x: 13.6, t: [结论 ②#linebreak()∠ACB = ∠DBC], fill: soft-green, st: green-ok),
  )
  for s in steps {
    node((s.x, 2.2), s.t, w: 3.3, h: 1.3, fill: s.fill, stroke: s.st, text-size: 8pt)
  }
  arrow((3.6, 2.2), (4.2, 2.2), paint: gray-line, thickness: 1.2pt)
  arrow((7.6, 2.2), (8.2, 2.2), paint: gray-line, thickness: 1.2pt)
  arrow((11.6, 2.2), (11.9, 2.2), paint: gray-line, thickness: 1.2pt)
  content((7.1, 0.7), text(size: 8.5pt, fill: gray-line, align(center, [证明题的写法 = 把这条链的每一环#linebreak()用「因为…所以…」翻译成文字])))
})
