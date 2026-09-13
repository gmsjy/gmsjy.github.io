// s3-fig-09-optics.typ — 折射定律：界面上的角度关系
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  // 界面与法线
  rect((-0.2, 2.2), (6.2, 4.4), fill: soft-blue.lighten(50%), stroke: none)
  content((5.6, 4.05), text(size: 8pt, fill: math-blue, [介质 1（$n_1$ 小）]), anchor: "east")
  content((5.6, 2.45), text(size: 8pt, fill: phys-orange, [介质 2（$n_2$ 大）]), anchor: "east")
  line((-0.2, 2.2), (6.2, 2.2), stroke: 1.1pt + ink)
  line((3.0, 0.8), (3.0, 4.6), stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  content((3.15, 4.45), text(size: 7.8pt, fill: gray-line, [法线]), anchor: "west")
  // 入射、反射、折射光线
  vector((1.2, 4.2), (3.0, 2.2), paint: math-blue, thickness: 1.3pt, label: [入射], label-off: (-0.55, 0.15), label-size: 8pt)
  vector((3.0, 2.2), (4.8, 4.2), paint: gray-line, thickness: 1pt, label: [反射], label-off: (0.3, 0.15), label-size: 8pt)
  vector((3.0, 2.2), (3.85, 0.8), paint: warn-red, thickness: 1.3pt, label: [折射（偏向法线）], label-off: (0.2, -0.05), label-size: 8pt)
  content((1.75, 2.75), text(size: 8.5pt, fill: ink, [$theta_1$]))
  content((3.15, 1.75), text(size: 8.5pt, fill: ink, [$theta_2$]))
  content((3.0, 0.1), text(size: 9pt, fill: ink, weight: "bold", align(center, [折射定律：$n_1 sin theta_1 = n_2 sin theta_2$，$n = c / v$])))
})
