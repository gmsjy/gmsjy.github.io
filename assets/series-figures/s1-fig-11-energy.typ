// s1-fig-11-energy.typ — 能量流：势能 → 动能 → 内能（宽度 = 多少）
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  node((1.9, 2.8), [重力势能#linebreak()$m g h$], w: 2.6, h: 1.2, fill: soft-orange, stroke: phys-orange, text-size: 8.5pt)
  node((6.2, 3.4), [动能#linebreak()$(1/2) m v^2$], w: 2.4, h: 1.2, fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
  node((6.2, 1.4), [内能（热）#linebreak()$Q = f s$], w: 2.4, h: 1.1, fill: soft-gray, stroke: gray-line, text-size: 8.5pt)
  node((10.6, 3.4), [落地时的动能#linebreak()与声、形变], w: 3.0, h: 1.2, fill: soft-green, stroke: green-ok, text-size: 8.5pt)
  // 粗箭头 = 能量多；细箭头 = 能量少
  vector((3.3, 3.15), (4.9, 3.5), paint: phys-orange, thickness: 2.2pt, label: [大部分], label-off: (-0.2, 0.28), label-size: 7.5pt)
  vector((3.3, 2.3), (4.9, 1.5), paint: gray-line, thickness: 1pt, label: [少量（摩擦）], label-off: (-0.3, -0.28), label-size: 7.5pt)
  vector((7.5, 3.4), (9.0, 3.4), paint: math-blue, thickness: 2.2pt, label: [传递], label-off: (-0.15, 0.28), label-size: 7.5pt)
  content((5.9, 5.1), text(size: 9pt, fill: ink, weight: "bold", [总账不变：$m g h = (1/2) m v^2 + Q$]))
  content((5.9, -0.1), text(size: 8.5pt, fill: gray-line, align(center, [箭头越粗表示流走的能量越多——能量只会换形式和位置，#linebreak()总量始终守恒（能量守恒定律）])))
})
