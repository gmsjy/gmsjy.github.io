// s1-fig-12b-field.typ — 电场线与等势面：点电荷 + 匀强场
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  // 左：正点电荷辐射场
  charge((2.4, 2.4), sign: "+", r: 0.26)
  for i in range(8) {
    let a = 45deg * i
    let f = (2.4 + 0.38 * calc.cos(a), 2.4 + 0.38 * calc.sin(a))
    let t = (2.4 + 1.75 * calc.cos(a), 2.4 + 1.75 * calc.sin(a))
    field-line(f, t, paint: math-blue)
  }
  circle((2.4, 2.4), radius: 1.05, stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  circle((2.4, 2.4), radius: 1.55, stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  content((2.4, 0.05), text(size: 8pt, fill: gray-line, [点电荷：辐射状，虚线为等势面]))
  // 右：平行板匀强场
  rect((6.6, 1.0), (6.9, 3.8), fill: soft-blue, stroke: math-blue)
  rect((9.9, 1.0), (10.2, 3.8), fill: soft-red, stroke: warn-red)
  for i in range(5) {
    let y = 1.35 + 0.55 * i
    field-line((6.9, y), (9.9, y), paint: math-blue, thickness: 0.8pt)
  }
  for i in range(2) {
    let y = 1.9 + 1.1 * i
    line((7.1, y), (9.7, y), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  }
  content((8.4, 0.35), text(size: 8pt, fill: gray-line, [平行板：均匀平行（匀强场）]))
  content((6.2, 4.15), text(size: 8.5pt, fill: ink, align(center, [电场线：疏密 = 强弱，#linebreak()切线 = 方向；等势面与它处处垂直])))
})
