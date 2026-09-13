// s2-fig-03-sequence.typ — 等差与等比：两列点的生长方式
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle, rect
  // 等差：+2 恒定步长
  content((1.3, 3.9), text(size: 9pt, fill: math-blue, weight: "bold", align(center, [等差：公差 $d = 2$])))
  let ax = 1.3
  for i in range(6) {
    circle((ax + i * 0.62, 3.0), radius: 0.12, fill: math-blue, stroke: none)
  }
  arrow((ax + 2 * 0.62 + 0.16, 3.0), (ax + 3 * 0.62 - 0.16, 3.0), paint: gray-line, thickness: 0.8pt, scale: 0.4, label: [+2], label-off: (0, 0.25), label-size: 7pt)
  content((5.6, 3.0), text(size: 8.5pt, fill: math-blue, […]))
  content((1.3, 2.35), text(size: 7.8pt, fill: gray-line, align(center, [每步等长：直线的生长])))
  // 等比：×2 指数生长
  content((1.3, 1.5), text(size: 9pt, fill: phys-orange, weight: "bold", align(center, [等比：公比 $q = 2$])))
  let sizes = (0.1, 0.16, 0.24, 0.36, 0.54, 0.8)
  for i in range(6) {
    rect((ax + i * 0.85 - sizes.at(i), 1.05 - sizes.at(i)), (ax + i * 0.85 + sizes.at(i), 1.05 + sizes.at(i)), fill: soft-orange, stroke: 1pt + phys-orange)
  }
  content((6.3, 1.05), text(size: 8.5pt, fill: phys-orange, […]))
  content((1.3, 0.05), text(size: 7.8pt, fill: gray-line, align(center, [每步翻倍：曲线（爆炸式）的生长])))
})
