// s2-fig-13b-circuit.typ — 闭合电路欧姆定律：内外分账
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  // 矩形回路
  wire((1.0, 0.8), (1.0, 4.2))
  wire((1.0, 0.8), (10.0, 0.8))
  wire((10.0, 0.8), (10.0, 4.2))
  wire((1.0, 4.2), (3.6, 4.2))
  wire((6.4, 4.2), (10.0, 4.2))
  // 电源（下边换成电池符号更直观：放左边中段）
  circuit-battery((1.0, 2.5), d: (0, 1), len: 1.0)
  wire((1.0, 0.8), (1.0, 2.0))
  wire((1.0, 3.0), (1.0, 4.2))
  content((0.45, 2.5), text(size: 8.5pt, fill: ink, [E, r]), anchor: "east")
  // 内阻
  circuit-resistor((1.0, 3.6), d: (0, 1), len: 0.9, label: $r$, label-off: (0.45, 0))
  // 外阻
  circuit-resistor((10.0, 2.5), d: (0, 1), len: 1.4, label: $R$, label-off: (0.5, 0))
  // 电流方向
  arrow((3.0, 4.2), (3.9, 4.2), paint: warn-red, thickness: 1.1pt, label: $I$, label-off: (0, 0.25))
  arrow((9.0, 0.8), (8.1, 0.8), paint: warn-red, thickness: 1.1pt, label: $I$, label-off: (0, 0.25))
  content((5.5, 5.0), text(size: 9.5pt, fill: ink, weight: "bold", align(center, [$I = E / (R + r)$，$U_"外" = E - I r$])))
  content((5.5, -0.35), text(size: 8.5pt, fill: gray-line, align(center, [电源是一本总账：电动势按电阻正比分给内、外两个用账人])))
})
