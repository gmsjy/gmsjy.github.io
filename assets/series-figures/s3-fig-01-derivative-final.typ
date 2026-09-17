// s3-fig-01-derivative-final.typ — 含参函数族：f(x) = ln x − a x 的零点随 a 变化
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let e = 2.718281828
#let fa(a, x) = calc.ln(x) - a * x

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(0, 7.3, -2.6, 2.3, x-step: 1, y-step: 1, y-label: $y$)

  // a = 0：单调增，一个零点
  curve(x => fa(0, x), 0.06, 7.0, stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  // a = 0.3：两个零点
  curve(x => fa(0.3, x), 0.06, 7.0, stroke: 1.5pt + math-blue)
  circle((1.64, 0), radius: 0.1, fill: warn-red, stroke: white)
  circle((5.95, 0), radius: 0.1, fill: warn-red, stroke: white)
  // a = 1/e：恰好相切，一个零点
  curve(x => fa(1 / e, x), 0.06, 7.0, stroke: 1.3pt + phys-orange)
  circle((e, 0), radius: 0.1, fill: phys-orange, stroke: white)
  // a = 0.6：无零点
  curve(x => fa(0.6, x), 0.06, 7.0, stroke: 1.1pt + green-ok)

  // 曲线端点标签
  content((7.15, 1.85), text(size: 8pt, fill: gray-line, [$a = 0$]), anchor: "west")
  content((7.15, -0.25), text(size: 8pt, fill: math-blue, weight: "bold", [$a = 0.3$]), anchor: "west")
  content((7.15, -0.85), text(size: 8pt, fill: phys-orange, weight: "bold", [$a = 1/e$]), anchor: "west")
  content((7.15, -1.5), text(size: 8pt, fill: green-ok, [$a = 0.6$]), anchor: "west")

  // e 刻度
  line((e, -0.1), (e, 0.1), stroke: 0.9pt + ink)
  content((e, -0.42), text(size: 8.5pt, fill: ink, [$e$]))

  // 注释
  content((5.35, 1.02), text(size: 8pt, fill: phys-orange, weight: "bold", [$a = 1/e$ 时恰好相切]))
  arrow((5.3, 0.87), (2.95, 0.2), paint: phys-orange, thickness: 0.6pt, scale: 0.45)
  content((3.3, -1.9), text(size: 8pt, fill: warn-red, weight: "bold", [$a = 0.3$：两个零点]))
  arrow((3.6, -1.7), (1.75, -0.2), paint: warn-red, thickness: 0.6pt, scale: 0.45)
  arrow((4.6, -1.7), (5.8, -0.2), paint: warn-red, thickness: 0.6pt, scale: 0.45)

  // 函数表达式
  node((1.7, 1.75), [$f(x) = ln x - a x$ $(x > 0)$], w: 3.6, h: 0.75,
    fill: soft-gray.lighten(40%), stroke: gray-line, text-size: 8.5pt)
})
