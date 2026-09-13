// s1-fig-15b-quadrant.typ — 错题四象限：按「会不会 × 对没对」分拣
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  import cetz.draw: line as _l
  let l(x0, y0, x1, y1) = { import cetz.draw: line; line((x0, y0), (x1, y1), stroke: 1.1pt + gray-line, mark: (end: ">>", fill: gray-line, scale: 0.55)) }
  l(1.2, 1.0, 12.6, 1.0)
  l(1.2, 1.0, 1.2, 6.2)
  content((12.4, 0.55), text(size: 8.5pt, fill: gray-line, [做对了 →]))
  content((0.75, 6.0), text(size: 8.5pt, fill: gray-line, [会做 ↑]))
  node((4.4, 4.6), [会做 + 做对#linebreak()绿区：不整理], w: 5.4, h: 1.9, fill: soft-green, stroke: green-ok, text-size: 9pt)
  node((10.2, 4.6), [不会 + 蒙对#linebreak()最危险：假装会了#linebreak()→ 重做并讲给别人听], w: 4.2, h: 2.4, fill: soft-orange, stroke: phys-orange, text-size: 8.5pt)
  node((4.4, 2.3), [会做 + 做错#linebreak()熟练度问题 → 限时重练 3 天], w: 5.4, h: 1.9, fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
  node((10.2, 2.3), [不会 + 做错#linebreak()真漏洞 → 回课本补概念], w: 4.2, h: 1.9, fill: soft-red, stroke: warn-red, text-size: 8.5pt)
  content((6.9, 0.4), text(size: 8.5pt, fill: gray-line, [错题本不是抄题本：先分拣象限，再决定「抄不抄、怎么练」]))
})
