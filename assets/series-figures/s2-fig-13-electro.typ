// s2-fig-13-electro.typ — 点电荷的电场线与等势面
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 正电荷
  charge((2.6, 2.9), sign: "+", r: 0.26)
  for i in range(8) {
    let a = 45deg * i
    field-line((2.6 + 0.38 * calc.cos(a), 2.9 + 0.38 * calc.sin(a)), (2.6 + 1.8 * calc.cos(a), 2.9 + 1.8 * calc.sin(a)), paint: math-blue)
  }
  circle((2.6, 2.9), radius: 1.0, stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  circle((2.6, 2.9), radius: 1.5, stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  content((2.6, 0.75), text(size: 7.8pt, fill: gray-line, [正电荷：辐射向外，虚线 = 等势面]))
  // 负电荷
  charge((9.4, 2.9), sign: "−", r: 0.26, paint: math-blue)
  for i in range(8) {
    let a = 45deg * i
    field-line((9.4 + 1.8 * calc.cos(a), 2.9 + 1.8 * calc.sin(a)), (9.4 + 0.38 * calc.cos(a), 2.9 + 0.38 * calc.sin(a)), paint: math-blue)
  }
  circle((9.4, 2.9), radius: 1.0, stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  content((9.4, 0.75), text(size: 7.8pt, fill: gray-line, [负电荷：指向圆心，电势沿电场线降低]))
  content((6.0, 5.3), text(size: 9pt, fill: ink, weight: "bold", align(center, [电场线永不相交、不闭合；#linebreak()顺着电场线 = 电势降低的方向])))
})
