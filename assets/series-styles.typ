// =====================================================================
// series/styles.typ — 系列统一颜色 + CeTZ 组件库
// 依赖：cetz 0.5.2（typst 0.14+）
// 所有组件均为「绘制指令」：必须在 cetz.canvas 的闭包内调用。
// =====================================================================
#import "@preview/cetz:0.5.2"

// ---------- 统一颜色 ----------
#let math-blue   = rgb("#2B6CB0") // 数学
#let phys-orange = rgb("#DD6B20") // 物理
#let warn-red    = rgb("#E53E3E") // 警示 / 断层
#let gray-line   = rgb("#4A5568") // 坐标轴与辅助线
#let ink         = rgb("#1A202C") // 正文墨色
#let green-ok    = rgb("#38A169") // 达标 / 正确
// 浅色填充（节点、面板底色）
#let soft-blue   = rgb("#EBF4FF")
#let soft-orange = rgb("#FFF4E6")
#let soft-red    = rgb("#FFF5F5")
#let soft-gray   = rgb("#EDF2F7")
#let soft-green  = rgb("#F0FFF4")

// ---------- 小工具 ----------
// 数字转字符串：整数去掉小数点
#let fnum(x, digits: 2) = {
  if calc.abs(x - calc.round(x)) < 1e-9 { str(int(calc.round(x))) } else { str(calc.round(x, digits)) }
}

// 曲线采样：f 从 x0 到 x1 取 n+1 个点；yclip: (ymin, ymax) 可选裁剪
#let samples(f, x0, x1, n: 60, yclip: none) = {
  range(n + 1).map(i => {
    let x = x0 + (x1 - x0) * i / n
    let y = f(x)
    if yclip != none { y = calc.min(calc.max(y, yclip.at(0)), yclip.at(1)) }
    (x, y)
  })
}

// 折线（可裁剪采样后直接连线）
#let curve(f, x0, x1, n: 60, yclip: none, stroke: 1.1pt + math-blue, mark: none) = {
  import cetz.draw: line, content, rect, circle
  line(..samples(f, x0, x1, n: n, yclip: yclip), stroke: stroke, mark: mark)
}

// =====================================================================
// 组件 1：axes() —— 统一坐标轴与刻度
// =====================================================================
#let axes(x-min, x-max, y-min, y-max,
    x-step: 1, y-step: 1,
    x-label: $x$, y-label: $y$,
    show-grid: false, tick-labels: true, tick-size: 0.16, overshoot: 0.35,
    label-size: 8pt, paint: gray-line, grid-color: rgb("#E2E8F0")) = {
  import cetz.draw: line, content, rect, circle
  if show-grid {
    let gx = calc.ceil(x-min / x-step) * x-step
    while gx <= x-max + 1e-9 {
      if calc.abs(gx) > 1e-9 { line((gx, y-min), (gx, y-max), stroke: (paint: grid-color, thickness: 0.4pt)) }
      gx += x-step
    }
    let gy = calc.ceil(y-min / y-step) * y-step
    while gy <= y-max + 1e-9 {
      if calc.abs(gy) > 1e-9 { line((x-min, gy), (x-max, gy), stroke: (paint: grid-color, thickness: 0.4pt)) }
      gy += y-step
    }
  }
  // 轴所在位置：0 在范围内取 0，否则钳制到最近边界（用于偏移的小图）
  let xa = calc.min(calc.max(0, y-min), y-max)
  let ya = calc.min(calc.max(0, x-min), x-max)
  line((x-min, xa), (x-max + overshoot, xa),
    stroke: 0.9pt + paint, mark: (end: ">>", fill: paint, scale: 0.55))
  line((ya, y-min), (ya, y-max + overshoot),
    stroke: 0.9pt + paint, mark: (end: ">>", fill: paint, scale: 0.55))
  if tick-labels {
    let tx = calc.ceil(x-min / x-step) * x-step
    while tx <= x-max + 1e-9 {
      if calc.abs(tx) > 1e-9 {
        line((tx, xa - tick-size / 2), (tx, xa + tick-size / 2), stroke: 0.7pt + paint)
        content((tx, xa - tick-size * 1.6), text(size: label-size, fill: paint, fnum(tx)))
      }
      tx += x-step
    }
    let ty = calc.ceil(y-min / y-step) * y-step
    while ty <= y-max + 1e-9 {
      if calc.abs(ty) > 1e-9 {
        line((ya - tick-size / 2, ty), (ya + tick-size / 2, ty), stroke: 0.7pt + paint)
        content((ya - tick-size * 1.35, ty), text(size: label-size, fill: paint, fnum(ty)), anchor: "east")
      }
      ty += y-step
    }
  }
  if x-label != none { content((x-max + overshoot + 0.18, xa), text(size: label-size + 1pt, fill: paint, x-label)) }
  if y-label != none { content((ya, y-max + overshoot + 0.22), text(size: label-size + 1pt, fill: paint, y-label)) }
}

// =====================================================================
// 组件 2：vector() —— 矢量箭头与标注
// =====================================================================
#let vector(from, to, paint: math-blue, thickness: 1.1pt, dash: none, scale: 0.75,
    label: none, label-off: (0.18, 0.18), label-size: 8.5pt) = {
  import cetz.draw: line, content, rect, circle
  line(from, to,
    stroke: (paint: paint, thickness: thickness, dash: dash),
    mark: (end: ">>", fill: paint, scale: scale))
  if label != none {
    content((to.at(0) + label-off.at(0), to.at(1) + label-off.at(1)),
      text(size: label-size, fill: paint, weight: "bold", label))
  }
}

// =====================================================================
// 组件 3：node() —— 知识树 / 流程图节点（显式定宽高，居中文字）
// =====================================================================
#let node(pos, body, w: 2.6, h: 0.8, fill: soft-blue, stroke: math-blue, thickness: 0.8pt,
    radius: 0.09, name: none, dash: none, text-size: 8.5pt, text-fill: ink, weight: "regular") = {
  import cetz.draw: line, content, rect, circle
  let cx = pos.at(0)
  let cy = pos.at(1)
  rect((cx - w / 2, cy - h / 2), (cx + w / 2, cy + h / 2),
    fill: fill, radius: radius,
    stroke: if stroke == none { none } else { (paint: stroke, thickness: thickness, dash: dash) })
  content((cx, cy), text(size: text-size, fill: text-fill, weight: weight, body), name: name)
}

// =====================================================================
// 组件 4：arrow() —— 节点连线与方向（可带虚线与沿线标注）
// =====================================================================
#let arrow(a, b, paint: gray-line, thickness: 0.9pt, dash: none, scale: 0.65,
    label: none, label-pos: 0.5, label-off: (0, 0.22), label-size: 7.5pt) = {
  import cetz.draw: line, content, rect, circle
  line(a, b,
    stroke: (paint: paint, thickness: thickness, dash: dash),
    mark: (end: ">>", fill: paint, scale: scale))
  if label != none {
    let mx = a.at(0) + (b.at(0) - a.at(0)) * label-pos + label-off.at(0)
    let my = a.at(1) + (b.at(1) - a.at(1)) * label-pos + label-off.at(1)
    content((mx, my), text(size: label-size, fill: paint, label))
  }
}

// =====================================================================
// 组件 5：circuit() —— 电池、电阻、开关、导线、灯泡、电表
// 方向 d 为元件轴向（任意非零向量），内部自动归一化。
// =====================================================================
#let wire(a, b, paint: ink, thickness: 0.9pt) = {
  import cetz.draw: line, content, rect, circle
  line(a, b, stroke: (paint: paint, thickness: thickness))
}

#let circuit-battery(center, d: (1, 0), len: 1.4, paint: ink) = {
  import cetz.draw: line, content, rect, circle
  let n = calc.sqrt(d.at(0) * d.at(0) + d.at(1) * d.at(1))
  let ux = d.at(0) / n
  let uy = d.at(1) / n
  let px = -uy
  let py = ux
  let cx = center.at(0)
  let cy = center.at(1)
  let hl = len / 2
  // 长板 = 正极（+0.18·u），短厚板 = 负极（-0.18·u）
  line((cx + ux * 0.18 - px * 0.30, cy + uy * 0.18 - py * 0.30),
       (cx + ux * 0.18 + px * 0.30, cy + uy * 0.18 + py * 0.30), stroke: 1pt + paint)
  line((cx - ux * 0.18 - px * 0.14, cy - uy * 0.18 - py * 0.14),
       (cx - ux * 0.18 + px * 0.14, cy - uy * 0.18 + py * 0.14), stroke: 2.4pt + paint)
  line((cx - ux * hl, cy - uy * hl), (cx - ux * 0.18, cy - uy * 0.18), stroke: 0.9pt + paint)
  line((cx + ux * 0.18, cy + uy * 0.18), (cx + ux * hl, cy + uy * hl), stroke: 0.9pt + paint)
}

#let circuit-resistor(center, d: (1, 0), len: 1.6, paint: ink, label: none, label-off: (0, 0.4)) = {
  import cetz.draw: line, content, rect, circle
  let n = calc.sqrt(d.at(0) * d.at(0) + d.at(1) * d.at(1))
  let ux = d.at(0) / n
  let uy = d.at(1) / n
  let px = -uy
  let py = ux
  let cx = center.at(0)
  let cy = center.at(1)
  let hl = len / 2
  let bl = hl * 0.62
  let amp = 0.13
  let k = 6
  line((cx - ux * hl, cy - uy * hl), (cx - ux * bl, cy - uy * bl), stroke: 0.9pt + paint)
  line((cx + ux * hl, cy + uy * hl), (cx + ux * bl, cy + uy * bl), stroke: 0.9pt + paint)
  let pts = range(k + 1).map(i => {
    let t = i / k
    let bx = cx - ux * bl + 2 * bl * ux * t
    let by = cy - uy * bl + 2 * bl * uy * t
    let s = if i == 0 or i == k { 0 } else if calc.odd(i) { amp } else { -amp }
    (bx + px * s, by + py * s)
  })
  line(..pts, stroke: 0.9pt + paint)
  if label != none {
    content((cx + label-off.at(0), cy + label-off.at(1)), text(size: 8pt, fill: paint, label))
  }
}

#let circuit-switch(center, d: (1, 0), len: 1.2, paint: ink) = {
  import cetz.draw: line, content, rect, circle
  let n = calc.sqrt(d.at(0) * d.at(0) + d.at(1) * d.at(1))
  let ux = d.at(0) / n
  let uy = d.at(1) / n
  let px = -uy
  let py = ux
  let cx = center.at(0)
  let cy = center.at(1)
  let hl = len / 2
  circle((cx - ux * hl, cy - uy * hl), radius: 0.05, fill: white, stroke: 0.8pt + paint)
  circle((cx + ux * hl, cy + uy * hl), radius: 0.05, fill: white, stroke: 0.8pt + paint)
  line((cx - ux * hl, cy - uy * hl),
       (cx + ux * hl + px * 0.32, cy + uy * hl + py * 0.32), stroke: 1pt + paint)
}

#let circuit-lamp(center, r: 0.24, paint: ink, label: none, label-off: (0, 0.42)) = {
  import cetz.draw: line, content, rect, circle
  let cx = center.at(0)
  let cy = center.at(1)
  circle((cx, cy), radius: r, fill: white, stroke: 0.9pt + paint)
  let k = r * 0.72
  line((cx - k, cy - k), (cx + k, cy + k), stroke: 0.8pt + paint)
  line((cx - k, cy + k), (cx + k, cy - k), stroke: 0.8pt + paint)
  if label != none {
    content((cx + label-off.at(0), cy + label-off.at(1)), text(size: 8pt, fill: paint, label))
  }
}

#let circuit-meter(center, sym: $A$, r: 0.26, paint: ink) = {
  import cetz.draw: line, content, rect, circle
  circle(center, radius: r, fill: white, stroke: 0.9pt + paint)
  content(center, text(size: 8pt, fill: paint, sym))
}

// =====================================================================
// 组件 6：field-line() —— 电场线、磁感线（中点箭头）；另有电荷与 B 符号
// =====================================================================
#let field-line(from, to, paint: math-blue, thickness: 0.7pt, arrow-at: 0.5, scale: 0.5) = {
  import cetz.draw: line, content, rect, circle
  let mx = from.at(0) + (to.at(0) - from.at(0)) * arrow-at
  let my = from.at(1) + (to.at(1) - from.at(1)) * arrow-at
  let st = (paint: paint, thickness: thickness)
  line(from, (mx, my), stroke: st, mark: (end: ">>", fill: paint, scale: scale))
  line((mx, my), to, stroke: st)
}

#let charge(pos, sign: "+", r: 0.24, paint: warn-red) = {
  import cetz.draw: line, content, rect, circle
  circle(pos, radius: r, fill: white, stroke: 1pt + paint)
  content(pos, text(size: 9pt, fill: paint, weight: "bold", sign))
}

// 垂直纸面向里（×）与向外（·）的磁感应强度符号
#let b-into(pos, r: 0.1, paint: gray-line) = {
  import cetz.draw: line, content, rect, circle
  circle(pos, radius: r, stroke: 0.7pt + paint)
  let k = r * 0.72
  line((pos.at(0) - k, pos.at(1) - k), (pos.at(0) + k, pos.at(1) + k), stroke: 0.7pt + paint)
  line((pos.at(0) - k, pos.at(1) + k), (pos.at(0) + k, pos.at(1) - k), stroke: 0.7pt + paint)
}

#let b-out(pos, r: 0.1, paint: gray-line) = {
  import cetz.draw: line, content, rect, circle
  circle(pos, radius: r, stroke: 0.7pt + paint)
  circle(pos, radius: 0.025, fill: paint, stroke: none)
}

// =====================================================================
// 组件 7：conic() —— 椭圆 / 双曲线 / 抛物线（参数采样折线）
//   ellipse:   中心 center，半轴 a、b
//   hyperbola: x²/a² - y²/b² = 1，画到 |x| = xmax
//   parabola:  y² = 2 p x，顶点 center，|y| ≤ ymax
// =====================================================================
#let conic(kind: "ellipse", center: (0, 0), a: 2, b: 1, p: 0.8,
    xmax: 4, ymax: 3, n: 80, stroke: 1pt + math-blue) = {
  import cetz.draw: line, content, rect, circle
  if kind == "ellipse" {
    let pts = range(n + 1).map(i => {
      let t = i / n * 360deg
      (center.at(0) + a * calc.cos(t), center.at(1) + b * calc.sin(t))
    })
    line(..pts, stroke: stroke)
  } else if kind == "hyperbola" {
    let tmax = calc.acosh(xmax / a)
    let right = range(n + 1).map(i => {
      let t = -tmax + 2 * tmax * i / n
      (center.at(0) + a * calc.cosh(t), center.at(1) + b * calc.sinh(t))
    })
    let left = right.map(q => (2 * center.at(0) - q.at(0), q.at(1)))
    line(..right, stroke: stroke)
    line(..left, stroke: stroke)
  } else {
    let pts = range(n + 1).map(i => {
      let y = -ymax + 2 * ymax * i / n
      (center.at(0) + y * y / (2 * p), center.at(1) + y)
    })
    line(..pts, stroke: stroke)
  }
}

// =====================================================================
// 组件 8：error-bar() —— 误差棒与拟合线
// =====================================================================
#let error-bar(p, ey, paint: gray-line, cap: 0.09, thickness: 0.8pt) = {
  import cetz.draw: line, content, rect, circle
  let x = p.at(0)
  let y = p.at(1)
  let st = thickness + paint
  line((x, y - ey), (x, y + ey), stroke: st)
  line((x - cap, y - ey), (x + cap, y - ey), stroke: st)
  line((x - cap, y + ey), (x + cap, y + ey), stroke: st)
}

// 拟合直线：y = k x + b 画在 [x0, x1]
#let fit-line(k, b, x0, x1, paint: warn-red, thickness: 1pt, dash: "dashed") = {
  import cetz.draw: line, content, rect, circle
  line((x0, k * x0 + b), (x1, k * x1 + b),
    stroke: (paint: paint, thickness: thickness, dash: dash))
}

// 散点 + 误差棒一步到位：pts = ((x, y, ey), ..)
#let scatter-error(pts, dot: math-blue, bar: gray-line, dot-r: 0.055) = {
  import cetz.draw: line, content, rect, circle
  for p in pts {
    error-bar((p.at(0), p.at(1)), p.at(2), paint: bar)
    circle((p.at(0), p.at(1)), radius: dot-r, fill: dot, stroke: none)
  }
}

// =====================================================================
// 组件 9：timeline() —— 时间轴与阶段节点
// events = ((x, label), ..) 或 ((x, label, color), ..)；标签上下交替
// =====================================================================
#let timeline(events, x0: 0, x1: 12, y: 0, paint: gray-line, dot-r: 0.075,
    alt: 0.9, label-size: 8pt, arrow: true) = {
  import cetz.draw: line, content, rect, circle
  if arrow {
    line((x0, y), (x1 + 0.4, y), stroke: 1pt + paint, mark: (end: ">>", fill: paint, scale: 0.55))
  } else {
    line((x0, y), (x1, y), stroke: 1pt + paint)
  }
  let up = true
  for ev in events {
    let x = ev.at(0)
    let lab = ev.at(1)
    let col = if ev.len() > 2 { ev.at(2) } else { paint }
    circle((x, y), radius: dot-r, fill: col, stroke: none)
    let dy = if up { alt } else { -alt }
    line((x, y), (x, y + dy * 0.55), stroke: (paint: col, thickness: 0.6pt, dash: "dashed"))
    content((x, y + dy), text(size: label-size, fill: col, align(center, lab)))
    up = not up
  }
}
