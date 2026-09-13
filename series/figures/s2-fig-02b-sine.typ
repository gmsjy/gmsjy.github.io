// s2-fig-02b-sine.typ — 正弦曲线：关键点与周期
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let pi = calc.pi

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.6, 7.9, -1.7, 1.9, x-step: 1, y-step: 1)
  curve(calc.sin, -0.4, 7.5, stroke: 1.4pt + math-blue)
  // 关键点：π/2, π, 3π/2, 2π
  let marks = ((pi / 2, 1, [最高 1]), (pi, 0, [零点]), (3 * pi / 2, -1, [最低 -1]), (2 * pi, 0, [一个周期]))
  for m in marks {
    let x = m.at(0)
    circle((x, m.at(1)), radius: 0.08, fill: warn-red, stroke: white)
    content((x, m.at(1) + (if m.at(1) > 0.5 { 0.4 } else { -0.42 })), text(size: 7.8pt, fill: warn-red, weight: "bold", m.at(2)))
  }
  content((pi, 1.45), text(size: 8pt, fill: gray-line, [正弦线的 y = 圆上点的 $y$ 坐标，随角扫过而起伏]))
})
