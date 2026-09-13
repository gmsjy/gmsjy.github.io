// s1-fig-18-roadmap.typ — 三年路线图：初三暑假到高考
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  timeline((
    (1.4, [初三暑假#linebreak()衔接黄金期], warn-red),
    (4.2, [高一上#linebreak()函数 + 力学], math-blue),
    (7.0, [高一下#linebreak()解析几何 + 电学], math-blue),
    (9.8, [高二#linebreak()导数 · 电磁感应], phys-orange),
    (12.6, [高三#linebreak()一轮 → 冲刺], phys-orange),
  ), x0: 0.6, x1: 13.4, alt: 1.15, label-size: 8.5pt)
  // 里程碑强调
  content((1.4, -1.85), text(size: 8.5pt, fill: warn-red, weight: "bold", [断层带：3 成人掉队于此]))
  content((7.0, -1.85), text(size: 8.5pt, fill: math-blue, weight: "bold", [分水岭：数理开始拉开差距]))
  content((12.6, -1.85), text(size: 8.5pt, fill: phys-orange, weight: "bold", [高考：水平早已在前两年定型]))
  content((7.0, 3.2), text(size: 9pt, fill: gray-line, align(center, [每一格的准确率，都由前一个格子的闭环质量决定])))
})
