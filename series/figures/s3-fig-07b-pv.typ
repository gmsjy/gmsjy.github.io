// s3-fig-07b-pv.typ — p-V 图：等温线与功的面积
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  axes(-0.3, 5.9, -0.3, 4.4, x-step: 1, y-step: 1, x-label: $V$, y-label: $p$, tick-labels: false)
  // 两条等温线 pV = C
  curve(x => 6 / x, 1.6, 5.5, stroke: 1.4pt + math-blue)
  curve(x => 3 / x, 0.8, 5.5, stroke: (paint: gray-line, thickness: 1.1pt, dash: "dashed"))
  content((5.0, 1.55), text(size: 8.5pt, fill: math-blue, weight: "bold", [$T_1$ 高温]), anchor: "west")
  content((5.0, 0.55), text(size: 8.5pt, fill: gray-line, [$T_0$ 低温]), anchor: "west")
  // 等温膨胀做功面积
  let band = range(31).map(i => { let v = 2.0 + 1.8 * i / 30; (v, 6 / v) })
  line(..band, close: true, fill: soft-orange, stroke: none)
  line((2.0, 0), (2.0, 3.0), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  line((3.8, 0), (3.8, 6 / 3.8), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  content((2.85, 1.55), text(size: 8.5pt, fill: phys-orange, weight: "bold", align(center, [面积#linebreak()$= W$])))
  content((2.9, 4.05), text(size: 8.5pt, fill: gray-line, align(center, [等温膨胀：压强随体积下降，#linebreak()气体对外做功 = 曲线下面积])))
})
