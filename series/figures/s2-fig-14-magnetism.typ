// s2-fig-14-magnetism.typ — 磁感线 / 洛伦兹力圆周 / 导轨感应电流
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

// ---------- 面板 A：条形磁铁磁感线 ----------
#let fig-a = cetz.canvas(length: 0.78cm, {
  import cetz.draw: line, content, rect, circle
  // 磁铁：N（左，红）S（右，蓝）
  rect((-1.2, -0.55), (0, 0.55), fill: soft-red, stroke: (paint: ink, thickness: 0.9pt))
  rect((0, -0.55), (1.2, 0.55), fill: soft-blue, stroke: (paint: ink, thickness: 0.9pt))
  content((-0.6, 0), text(size: 9pt, fill: warn-red, weight: "bold", [N]))
  content((0.6, 0), text(size: 9pt, fill: math-blue, weight: "bold", [S]))
  // 磁感线：上半从 N 绕到 S（顶端向右），下半对称
  for pair in ((2.0, 0.85), (2.9, 1.7), (3.8, 2.6)) {
    let a = pair.at(0)
    let b = pair.at(1)
    for sgn in ((1,), (-1,)) {
      let pts = range(61).map(i => {
        let t = 180deg - 180deg * i / 60
        (a * calc.cos(t), sgn.first() * b * calc.sin(t))
      })
      line(..pts, stroke: 0.8pt + math-blue)
    }
    vector((-0.3, b), (0.3, b), paint: math-blue, thickness: 0.8pt, scale: 0.6)
    vector((-0.3, -b), (0.3, -b), paint: math-blue, thickness: 0.8pt, scale: 0.6)
  }
  content((0, 3.15), text(size: 8pt, fill: math-blue, [外部：N → S]))
})

// ---------- 面板 B：洛伦兹力下的圆周运动 ----------
#let fig-b = cetz.canvas(length: 0.9cm, {
  import cetz.draw: line, content, circle
  // 磁场垂直纸面向里
  for i in range(8) {
    let a = 22.5deg + 45deg * i
    b-into((2.55 * calc.cos(a), 2.55 * calc.sin(a)))
  }
  circle((0, 0), radius: 1.6, stroke: (paint: ink, thickness: 1.1pt))
  circle((0, 0), radius: 0.04, fill: ink, stroke: none)
  content((-0.22, -0.3), text(size: 8.5pt, fill: ink, [$O$]))
  line((0, 0), (0, 1.6), stroke: (paint: gray-line, thickness: 0.7pt, dash: "dashed"))
  content((-0.32, 0.85), text(size: 8.5pt, fill: gray-line, [$r$]))
  // 电子（负电荷）：v 向右、B 向里时，负电荷受力指向圆心
  circle((0, 1.6), radius: 0.1, fill: math-blue, stroke: none)
  content((-0.18, 1.78), text(size: 8pt, fill: math-blue, weight: "bold", [$-q$（电子）]), anchor: "east")
  vector((0, 1.6), (1.05, 1.6), paint: math-blue, thickness: 1.2pt, label: $v$, label-off: (0.05, 0.2))
  vector((0, 1.6), (0, 0.62), paint: phys-orange, thickness: 1.2pt, label: $F$, label-off: (0.14, -0.1))
  content((0, -3.25), text(size: 8pt, fill: gray-line, [B 向里，电子受力指向圆心]))
})

// ---------- 面板 C：导轨 + 导体棒切割磁感线 ----------
#let fig-c = cetz.canvas(length: 0.82cm, {
  import cetz.draw: line, content
  // 导轨
  line((1.2, 0), (8.4, 0), stroke: 1.1pt + ink)
  line((1.2, 2.4), (8.4, 2.4), stroke: 1.1pt + ink)
  // 左端电阻
  circuit-resistor((1.2, 1.2), d: (0, 1), len: 1.2, label: $R$, label-off: (0.5, 0))
  // 导体棒
  line((5.8, 0), (5.8, 2.4), stroke: (paint: ink, thickness: 2.2pt))
  vector((5.9, 1.85), (7.0, 1.85), paint: math-blue, thickness: 1.2pt, label: $v$, label-off: (0.05, 0.2))
  // 感应电流方向（逆时针）
  vector((5.8, 0.65), (5.8, 1.45), paint: warn-red, thickness: 1pt, label: $I$, label-off: (0.2, -0.05))
  vector((4.7, 2.4), (3.5, 2.4), paint: warn-red, thickness: 1pt)
  vector((3.2, 0), (4.4, 0), paint: warn-red, thickness: 1pt)
  // 磁场符号
  b-into((2.7, 1.85))
  b-into((4.1, 0.95))
  b-into((7.3, 0.75))
  b-into((7.6, 1.95))
  content((4.9, 2.95), text(size: 8pt, fill: gray-line, [$B$ 向里，$I$ 沿逆时针]))
})

#let fig = grid(columns: 3, gutter: 12pt, fig-a, fig-b, fig-c)
