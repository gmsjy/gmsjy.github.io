// s1-fig-05-functions.typ — 函数家族谱系（6 类初等函数）+ 图像变换
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

// 单个函数小图：曲线相对于小图中心 (ox, oy) 平移绘制
// skip: (a, b) —— 该区间内不画线（用于跳过渐近线）
#let mini(ox, oy, w, h, title, fs, paint: math-blue, extra: none, skip: none) = cetz.canvas(length: 0.72cm, {
  import cetz.draw: line, content
  axes(ox - w, ox + w, oy - h, oy + h, overshoot: 0.22, tick-labels: false, x-label: none, y-label: none)
  let sample(a, b, n) = range(n + 1).map(i => {
    let t = a + (b - a) * i / n
    (ox + t, oy + fs(t))
  })
  if skip == none {
    line(..sample(-w, w, 60), stroke: 1.1pt + paint)
  } else {
    if skip.at(0) > -w + 1e-9 { line(..sample(-w, skip.at(0), 30), stroke: 1.1pt + paint) }
    if skip.at(1) < w - 1e-9 { line(..sample(skip.at(1), w, 30), stroke: 1.1pt + paint) }
  }
  if extra != none { extra(ox, oy, w, h) }
  content((ox - w + 0.18, oy + h - 0.32), text(size: 8pt, fill: ink, title), anchor: "west")
})

#let fig-family = grid(
  columns: 3, gutter: 9pt, row-gutter: 9pt,
  mini(0, 0, 2.1, 1.7, [一次 $y = k x + b$], t => 0.7 * t, extra: (ox, oy, w, h) => {
    import cetz.draw: line
    line((ox - w, oy + 0.7 * (-w) + 0.9), (ox + w, oy + 0.7 * w + 0.9), stroke: (paint: phys-orange, thickness: 0.9pt, dash: "dashed"))
  }),
  mini(0, 0, 1.9, 1.7, [二次 $y = x^2$], t => 0.5 * t * t, paint: phys-orange),
  mini(0, 0, 2.2, 1.6, [反比例 $y = 1/x$], t => 1 / t, paint: green-ok, skip: (-0.62, 0.62)),
  mini(0, 0, 1.6, 1.7, [幂 $y = x^3$], t => 0.55 * t * t * t, paint: gray-line),
  mini(0, 0, 2.1, 1.7, [指数 $y = 2^x$], t => calc.exp(0.693 * t) * 0.55, paint: warn-red),
  mini(0, 0, 2.1, 1.6, [对数 $y = log_2 x$], t => calc.ln(t) / 0.693 * 0.45, paint: math-blue, skip: (-2.1, 0.43)),
)

#let fig-transform = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-2.3, 3.7, -1.0, 4.9, x-step: 1, y-step: 1, show-grid: true)
  // 三条抛物线
  let p1 = range(61).map(i => { let t = -1.9 + 3.6 * i / 60; (t, t * t) })
  let p2 = range(61).map(i => { let t = -0.9 + 4.5 * i / 60; (t, (t - 1) * (t - 1)) })
  let p3 = range(61).map(i => { let t = -0.9 + 4.5 * i / 60; (t, (t - 1) * (t - 1) + 2) })
  line(..p1.map(p => (p.at(0), calc.min(p.at(1), 4.9))), stroke: 1.2pt + math-blue)
  line(..p2.map(p => (p.at(0), calc.min(p.at(1), 4.9))), stroke: 1.2pt + phys-orange)
  line(..p3.map(p => (p.at(0), calc.min(p.at(1), 4.9))), stroke: 1.2pt + green-ok)
  // 顶点变换标注
  circle((0, 0), radius: 0.06, fill: math-blue, stroke: none)
  circle((1, 0), radius: 0.06, fill: phys-orange, stroke: none)
  circle((1, 2), radius: 0.06, fill: green-ok, stroke: none)
  arrow((0.08, -0.4), (0.92, -0.4), paint: phys-orange, thickness: 0.8pt, label: [右移 1], label-off: (0, 0.18))
  arrow((1.35, 0.1), (1.35, 1.85), paint: green-ok, thickness: 0.8pt, label: [上移 2], label-off: (0.3, 0))
  content((-2.15, 3.3), text(size: 8pt, fill: math-blue, [$y = x^2$]))
  content((3.0, 2.6), text(size: 8pt, fill: phys-orange, [$y = (x-1)^2$]))
  content((3.05, 4.3), text(size: 8pt, fill: green-ok, [$y = (x-1)^2 + 2$]))
})

#let fig = grid(rows: 2, row-gutter: 10pt, fig-family, fig-transform)
