// s3-fig-09b-totalreflect.typ — 全反射：光从光密到光疏的三种命运
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  rect((-0.2, 2.4), (6.2, 4.6), fill: soft-blue.lighten(50%), stroke: none)
  content((5.7, 4.25), text(size: 8pt, fill: math-blue, [光密（水）]), anchor: "east")
  content((5.7, 2.6), text(size: 8pt, fill: phys-orange, [光疏（空气）]), anchor: "east")
  line((-0.2, 2.4), (6.2, 2.4), stroke: 1.1pt + ink)
  line((3.0, 1.0), (3.0, 4.8), stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  // 三条入射线（小角：折射+弱反射；临界：沿界面；大角：全反射）
  vector((0.9, 3.1), (2.55, 2.4), paint: math-blue, thickness: 1.1pt)
  vector((2.55, 2.4), (3.55, 0.9), paint: math-blue, thickness: 1.1pt, scale: 0.6)
  content((3.7, 1.15), text(size: 7.5pt, fill: math-blue, [$theta < theta_c$：折射 + 反射]))
  vector((2.2, 2.4), (3.0, 2.4), paint: phys-orange, thickness: 1.1pt)
  content((3.35, 2.6), text(size: 7.5pt, fill: phys-orange, [$theta = theta_c$：沿界面]))
  vector((4.3, 4.4), (5.1, 2.4), paint: warn-red, thickness: 1.2pt)
  vector((5.1, 2.4), (5.9, 4.4), paint: warn-red, thickness: 1.2pt, scale: 0.7)
  content((5.3, 3.6), text(size: 7.5pt, fill: warn-red, [$theta > theta_c$：全反射]))
  content((3.0, 0.25), text(size: 8.5pt, fill: gray-line, align(center, [临界角 $sin theta_c = 1 / n$——光纤通信、水下看天的「视窗」都由它决定])))
})
