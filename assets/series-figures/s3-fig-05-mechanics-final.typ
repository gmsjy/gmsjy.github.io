// s3-fig-05-mechanics-final.typ — 力学多过程（曲面→粗糙面→弹簧）+ 能量流
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

// ---------- 面板 A：多过程轨迹 + 各位置能量条 ----------
#let fig-a = cetz.canvas(length: 0.9cm, {
  import cetz.draw: line, content, rect, circle
  // 四分之一圆弧轨道：圆心 (3.3, 3.2)，r = 2.5
  let arc = range(41).map(i => {
    let t = 180deg + 90deg * i / 40
    (3.3 + 2.5 * calc.cos(t), 3.2 + 2.5 * calc.sin(t))
  })
  line(..arc, stroke: 1.2pt + ink)
  // 水平段
  line((3.3, 0.7), (7.6, 0.7), stroke: 1.2pt + ink)
  // 粗糙段刻痕
  let hx = 4.2
  while hx <= 6.0 {
    line((hx, 0.7), (hx - 0.16, 0.52), stroke: 0.6pt + gray-line)
    hx += 0.3
  }
  content((5.1, 0.25), text(size: 7.5pt, fill: gray-line, [粗糙段 $mu$]))
  // 弹簧（自然长度 + 压缩状态）与墙
  let zz(x0, x1, amp, n) = range(n + 1).map(i => {
    let t = i / n
    let sx = x0 + (x1 - x0) * t
    let sy = 0.7 + (if i == 0 or i == n { 0 } else if calc.odd(i) { amp } else { -amp })
    (sx, sy)
  })
  line(..zz(7.6, 8.7, 0.11, 6), stroke: 1pt + ink)
  line((8.7, 0.15), (8.7, 1.45), stroke: (paint: ink, thickness: 1.4pt))
  line(..zz(7.15, 7.6, 0.15, 4), stroke: 1pt + warn-red)
  // 滑块位置 A / B / C
  rect((0.62, 3.2), (0.98, 3.56), fill: soft-blue, stroke: ink)
  content((0.8, 3.86), text(size: 8.5pt, fill: ink, weight: "bold", [A]))
  rect((3.72, 0.7), (4.08, 1.06), fill: soft-blue, stroke: ink)
  content((3.9, 1.34), text(size: 8.5pt, fill: ink, weight: "bold", [B]))
  rect((6.52, 0.7), (6.88, 1.06), fill: soft-blue, stroke: ink)
  content((6.7, 1.34), text(size: 8.5pt, fill: warn-red, weight: "bold", [C（最远点）]))
  vector((4.35, 1.2), (5.0, 1.2), paint: math-blue, thickness: 1pt, label: $v_B$, label-off: (0.05, 0.18))
  // 能量条：橙=重力势能 蓝=动能 绿=弹性势能 灰=内能
  rect((0.55, 4.0), (1.05, 5.2), fill: phys-orange, stroke: none)
  rect((3.65, 1.8), (4.15, 2.6), fill: math-blue, stroke: none)
  rect((3.65, 2.6), (4.15, 3.0), fill: gray-line, stroke: none)
  rect((6.45, 1.8), (6.95, 2.3), fill: green-ok, stroke: none)
  rect((6.45, 2.3), (6.95, 2.8), fill: gray-line, stroke: none)
  // 图例
  let leg = ((2.0, phys-orange, [重力势能]), (3.9, math-blue, [动能]), (5.4, green-ok, [弹性势能]), (7.2, gray-line, [内能（摩擦）]))
  for l in leg {
    rect((l.at(0), -0.5), (l.at(0) + 0.28, -0.24), fill: l.at(1), stroke: none)
    content((l.at(0) + 0.4, -0.37), text(size: 7.5pt, fill: ink, l.at(2)), anchor: "west")
  }
  content((4.5, 5.5), text(size: 8pt, fill: gray-line, [每个位置的「能量条」总高相同：能量只换形式，不凭空消失]))
})

// ---------- 面板 B：全程能量流 ----------
#let fig-b = cetz.canvas(length: 0.9cm, {
  import cetz.draw: line, content
  node((1.6, 2.7), [重力势能#linebreak()$m g h$], w: 2.5, h: 1.0, fill: soft-orange, stroke: phys-orange, text-size: 8pt)
  node((5.1, 2.7), [动能#linebreak()$(1/2) m v^2$], w: 2.5, h: 1.0, fill: soft-blue, stroke: math-blue, text-size: 8pt)
  node((8.7, 3.5), [弹性势能#linebreak()$E_"p" = (1/2) k x^2$], w: 3.0, h: 1.0, fill: soft-green, stroke: green-ok, text-size: 8pt)
  node((8.7, 1.7), [内能#linebreak()$Q = f dot s$], w: 3.0, h: 1.0, fill: soft-gray, stroke: gray-line, text-size: 8pt)
  arrow((2.9, 2.7), (3.8, 2.7), paint: phys-orange, thickness: 1.2pt, label: [下滑], label-off: (0, 0.28))
  arrow((6.4, 2.95), (7.15, 3.35), paint: green-ok, thickness: 1.2pt, label: [压缩], label-off: (-0.1, 0.3))
  arrow((6.4, 2.45), (7.15, 1.95), paint: gray-line, thickness: 1.2pt, label: [摩擦], label-off: (0.1, -0.35))
  content((5.2, 0.5), text(size: 9.5pt, fill: ink, weight: 600, [全程账本：$m g h = (1/2) k x^2 + f dot s$]))
})

#let fig = grid(columns: 2, gutter: 12pt, fig-a, fig-b)
