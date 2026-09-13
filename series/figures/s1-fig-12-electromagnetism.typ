// s1-fig-12-electromagnetism.typ — 串联与并联电路对比
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  // 串联
  content((3.2, 4.6), text(size: 9pt, fill: ink, weight: "bold", [串联：一条路，电流处处相等]))
  circuit-battery((0.8, 3.2), d: (0, 1), len: 0.9)
  circuit-lamp((2.4, 3.2), label: $L_1$, label-off: (0, 0.45))
  circuit-lamp((4.2, 3.2), label: $L_2$, label-off: (0, 0.45))
  wire((0.8, 2.5), (0.8, 3.2 - 0.45) )
  wire((0.8, 3.65), (0.8, 3.2) )
  wire((1.05, 3.2), (2.16, 3.2))
  wire((2.64, 3.2), (3.96, 3.2))
  wire((4.44, 3.2), (5.6, 3.2))
  wire((5.6, 3.2), (5.6, 2.2))
  wire((5.6, 2.2), (0.8, 2.2))
  wire((0.8, 2.2), (0.8, 2.75))
  // 并联
  content((3.2, 1.15), text(size: 9pt, fill: ink, weight: "bold", [并联：分叉路，各走各的、互不干扰]))
  circuit-battery((0.8, -1.1), d: (0, 1), len: 0.9)
  wire((1.05, -1.1), (4.6, -1.1))
  wire((0.8, -0.65), (0.8, -1.1 - 0.45))
  wire((0.8, -0.65), (4.6, -0.65))
  circuit-lamp((2.2, -0.65), label: $L_1$, label-off: (-0.55, 0))
  circuit-lamp((3.6, -0.65), label: $L_2$, label-off: (-0.55, 0))
  content((6.6, 1.0), text(size: 8.5pt, fill: gray-line, align(center, [初中认符号、记规律；#linebreak()高中把这些电路放进#linebreak()含电源与电阻的计算里])))
})
