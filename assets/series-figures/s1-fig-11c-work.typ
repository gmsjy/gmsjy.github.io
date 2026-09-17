// s1-fig-11c-work.typ — 功的图像语言：F-x 图线下面积
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  axes(-0.4, 5.6, -0.4, 3.6, x-step: 1, y-step: 1, x-label: $x/"m"$, y-label: $F/"N"$)
  // 恒力情形：矩形面积
  let band = ((0, 2), (3, 2))
  line((0, 2), (3, 2), stroke: 1.4pt + math-blue)
  rect((0, 0), (3, 2), fill: soft-blue.lighten(40%), stroke: none)
  line((3, 0), (3, 2), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  content((1.5, 1.0), text(size: 9pt, fill: math-blue, weight: "bold", [面积 $= W$]))
  content((1.5, 2.35), text(size: 8.5pt, fill: math-blue, [恒力 $F$]))
  content((3.15, 1.0), text(size: 8pt, fill: ink, [$W = F s = 2 dot 3 = 6$ "J"]), anchor: "west")
  // 变力情形：曲线下面积
  let pts = range(41).map(i => { let t = 3.4 + 1.9 * i / 40; (t, 2.6 - 0.5 * (t - 3.4)) })
  line(..pts, close: true, fill: soft-orange, stroke: 1.3pt + phys-orange)
  content((4.9, 0.9), text(size: 8.5pt, fill: phys-orange, align(center, [变力做功 =#linebreak()曲线下面积])))
})
