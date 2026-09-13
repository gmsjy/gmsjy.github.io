// s3-fig-12b-rigidbody.typ — 刚体定轴转动：角量与线量
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  circle((3.0, 2.8), radius: 1.9, fill: soft-blue.lighten(40%), stroke: 1.2pt + ink)
  circle((3.0, 2.8), radius: 0.08, fill: ink, stroke: none)
  // 转轴标记
  content((3.0, 5.15), text(size: 8.5pt, fill: gray-line, [转轴 ⊙（垂直纸面）]))
  // 半径与角速度
  line((3.0, 2.8), (4.7, 2.5), stroke: 1.1pt + phys-orange)
  circle((4.7, 2.5), radius: 0.07, fill: phys-orange, stroke: none)
  content((4.85, 2.4), text(size: 8pt, fill: phys-orange, weight: "bold", [$r$ 处质点]), anchor: "west")
  vector((3.9, 1.35), (2.7, 1.5), paint: math-blue, thickness: 1.2pt, label: $v$, label-off: (-0.35, -0.05), label-size: 8.5pt)
  content((6.9, 3.4), text(size: 9.5pt, fill: ink, weight: "bold", align(center, [$v = omega r$，$a_t = alpha r$])))
  content((6.9, 2.2), text(size: 8.5pt, fill: gray-line, align(center, [同一刚体上，角量（$omega, alpha$）#linebreak()处处相同，线量随 $r$ 变大——#linebreak()「角量语言」是转动世界的母语])))
})
