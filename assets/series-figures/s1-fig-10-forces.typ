// s1-fig-10-forces.typ — 斜面受力分析 + 连接体隔离法
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

// ---------- 面板 A：斜面上的物块 ----------
#let fig-a = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 斜面（直角三角形）
  line((0.8, 0), (6.0, 0), (6.0, 2.85), close: true, fill: soft-gray.lighten(40%), stroke: (paint: ink, thickness: 0.9pt))
  let th = calc.atan(2.85 / 5.2)
  let ux = calc.cos(th)
  let uy = calc.sin(th)
  let nx = -uy
  let ny = ux
  // 物块（贴斜面）
  let bx = 0.8 + 5.2 * 0.42
  let by = 2.85 * 0.42
  let cx = bx + nx * 0.5
  let cy = by + ny * 0.5
  let s = 0.42
  line((cx + (ux + nx) * s, cy + (uy + ny) * s),
       (cx + (ux - nx) * s, cy + (uy - ny) * s),
       (cx - (ux + nx) * s, cy - (uy + ny) * s),
       (cx + (nx - ux) * s, cy + (ny - uy) * s),
       close: true, fill: soft-orange, stroke: ink)
  // 三个实际力：G、F_N、f
  vector((cx, cy), (cx, cy - 1.5), paint: phys-orange, thickness: 1.2pt, label: $G$, label-off: (0.16, -0.1))
  vector((cx, cy), (cx + nx * 1.35, cy + ny * 1.35), paint: math-blue, thickness: 1.2pt, label: $F_N$, label-off: (0.16, 0.12))
  vector((cx, cy), (cx + ux * 0.95, cy + uy * 0.95), paint: warn-red, thickness: 1.2pt, label: $f$, label-off: (-0.42, 0.14))
  // G 的两个虚分量：G1 = G sinθ 沿斜面向下，G2 = G cosθ 垂直斜面压向表面
  let g1 = (cx - ux * 1.5 * ux, cy - uy * 1.5 * ux)
  let g2 = (cx - nx * 1.5 * ux, cy - ny * 1.5 * ux)
  vector((cx, cy), g1, paint: gray-line, thickness: 0.8pt, dash: "dashed", scale: 0.5, label: $G_1 = G sin theta$, label-size: 7pt, label-off: (-0.65, -0.42))
  vector((cx, cy), g2, paint: gray-line, thickness: 0.8pt, dash: "dashed", scale: 0.5, label: $G_2 = G cos theta$, label-size: 7pt, label-off: (0.28, 0.14))
  line((cx, cy - 1.5), g1, stroke: (paint: gray-line, thickness: 0.5pt, dash: "dashed"))
  line((cx, cy - 1.5), g2, stroke: (paint: gray-line, thickness: 0.5pt, dash: "dashed"))
  content((1.7, 0.26), text(size: 8.5pt, fill: warn-red, [$theta$]))
  content((5.35, 3.4), text(size: 8pt, fill: gray-line, [光滑 or 粗糙？先看 $f$ 有没有]))
})

// ---------- 面板 B：连接体（桌面 + 悬挂） ----------
#let fig-b = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 桌面与桌腿
  line((0, 2.6), (6.0, 2.6), stroke: 1.1pt + ink)
  line((0.6, 2.6), (0.6, 0.2), stroke: 0.9pt + ink)
  line((5.4, 2.6), (5.4, 0.2), stroke: 0.9pt + ink)
  line((0.1, 0.2), (6.6, 0.2), stroke: 0.7pt + gray-line)
  // 定滑轮
  circle((5.9, 2.6), radius: 0.35, fill: white, stroke: (paint: ink, thickness: 1pt))
  circle((5.9, 2.6), radius: 0.04, fill: ink, stroke: none)
  // 物块 1（桌面）
  rect((1.15, 2.6), (2.65, 3.3), fill: soft-blue, stroke: ink)
  content((1.9, 2.95), text(size: 8pt, fill: math-blue, weight: "bold", [$m_1$]))
  // 物块 2（悬挂）
  rect((5.875, 1.25), (6.625, 2.25), fill: soft-orange, stroke: ink)
  content((6.25, 1.75), text(size: 8pt, fill: phys-orange, weight: "bold", [$m_2$]))
  // 绳子：水平 → 绕轮 → 竖直
  let arc = range(13).map(i => {
    let a = 90deg - 90deg * i / 12
    (5.9 + 0.35 * calc.cos(a), 2.6 + 0.35 * calc.sin(a))
  })
  line((2.65, 2.95), (5.9, 2.95), stroke: 0.8pt + ink)
  line(..arc, stroke: 0.8pt + ink)
  line((6.25, 2.6), (6.25, 2.25), stroke: 0.8pt + ink)
  // m1 受力：T 向右、f 向左、G 向下、N 向上
  vector((2.65, 2.95), (3.7, 2.95), paint: math-blue, label: $T$, label-off: (0, 0.24))
  vector((2.4, 2.6), (1.45, 2.6), paint: warn-red, thickness: 1pt, label: $f$, label-off: (0.0, 0.24))
  vector((1.9, 2.6), (1.9, 3.75), paint: math-blue, thickness: 1pt, label: $F_N$, label-off: (0.18, 0.05))
  vector((1.55, 3.3), (1.55, 2.5), paint: phys-orange, thickness: 1pt, label: $G_1$, label-off: (-0.52, 0.0))
  // m2 受力：T 向上、G 向下
  vector((6.6, 1.8), (6.6, 2.7), paint: math-blue, thickness: 1pt, label: $T$, label-off: (0.12, 0.1))
  vector((6.25, 1.25), (6.25, 0.45), paint: phys-orange, thickness: 1pt, label: $G_2$, label-off: (0.18, -0.05))
  // 隔离框
  rect((0.85, 2.35), (3.95, 4.05), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"), radius: 0.1)
  content((0.95, 4.25), text(size: 7.5pt, fill: gray-line, [隔离 ①：$T - f = m_1 a$]), anchor: "west")
  rect((5.6, 0.25), (7.3, 2.95), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"), radius: 0.1)
  content((5.7, 0.05), text(size: 7.5pt, fill: gray-line, [隔离 ②：$m_2 g - T = m_2 a$]), anchor: "west")
})

#let fig = grid(columns: 2, gutter: 12pt, fig-a, fig-b)
