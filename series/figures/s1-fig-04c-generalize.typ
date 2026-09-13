// s1-fig-04c-generalize.typ — 从具体到抽象：同一件事的三级台阶
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  node((2.2, 3.4), [具体数字#linebreak()$2^2 dot 2^3 = 2^5$], w: 3.4, h: 1.25, fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
  node((7.4, 3.4), [字母规律#linebreak()$a^m dot a^n = a^(m + n)$], w: 3.8, h: 1.25, fill: soft-orange, stroke: phys-orange, text-size: 8.5pt)
  node((12.6, 3.4), [含参应用#linebreak()$2^(x + 1) = 8 ==> x = 2$], w: 4.0, h: 1.25, fill: soft-green, stroke: green-ok, text-size: 8.5pt)
  arrow((4.0, 3.4), (5.45, 3.4), paint: gray-line, thickness: 1.2pt, label: [换个写法], label-off: (0, 0.3))
  arrow((9.35, 3.4), (10.55, 3.4), paint: gray-line, thickness: 1.2pt, label: [字母当数], label-off: (0, 0.3))
  // 底部：三个台阶验证同一规律
  line((0.8, 1.4), (14.8, 1.4), stroke: 0.8pt + gray-line, mark: (end: ">>", fill: gray-line, scale: 0.55))
  content((14.55, 1.0), text(size: 8pt, fill: gray-line, [抽象程度]))
  for p in ((2.2, [算一算]), (7.4, [证一证]), (12.6, [用一用])) {
    circle((p.at(0), 1.4), radius: 0.08, fill: math-blue, stroke: none)
    content((p.at(0), 0.95), text(size: 8pt, fill: ink, p.at(1)))
  }
  content((7.6, 1.95), text(size: 8.5pt, fill: gray-line, [同样的规律，在三个台阶上各走一遍，才算真正掌握]))
})
