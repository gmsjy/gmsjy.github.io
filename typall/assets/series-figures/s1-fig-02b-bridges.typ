// s1-fig-02b-bridges.typ — 三大衔接点放大：同一数学对象的两个身份
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

// 面板外框 + 标题徽章 + 中缝双向箭头 + 底部结论
#let panel(y0, idx, caption) = {
  import cetz.draw: line, content, rect, circle
  rect((0, y0), (14.0, y0 + 2.0), fill: if calc.odd(idx) { soft-gray.lighten(60%) } else { soft-blue.lighten(70%) }, stroke: (paint: gray-line, thickness: 0.5pt), radius: 0.12)
  circle((0.55, y0 + 1.62), radius: 0.28, fill: warn-red, stroke: none)
  content((0.55, y0 + 1.62), text(size: 8pt, fill: white, weight: "bold", str(idx)))
  content((7.0, y0 - 0.26), text(size: 8pt, fill: gray-line, caption))
  content((7.0, y0 + 1.15), text(size: 13pt, fill: gray-line.lighten(15%), [⇔]))
}

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle

  // ============ 面板 1：二次函数 ↔ v–t 图像 ============
  panel(4.9, 1, [匀变速位移 $x = v_0 t + (1 / 2) a t^2$，就是一个二次函数])
  { // 左：抛物线
    let ox = 2.6
    let oy = 5.65
    axes(ox - 1.7, ox + 1.9, oy - 0.7, oy + 0.9, overshoot: 0.25, tick-labels: false, x-label: none, y-label: none)
    let pp = range(61).map(i => {
      let t = -1.3 + 2.6 * i / 60
      (ox + t, oy - 0.7 + 0.55 * t * t)
    })
    line(..pp, stroke: 1.2pt + math-blue)
    content((ox + 1.75, oy + 0.55), text(size: 7.5pt, fill: math-blue, [$y = x^2$]), anchor: "west")
    content((ox - 1.55, oy + 0.62), text(size: 7.5pt, fill: ink, [二次函数]), anchor: "west")
  }
  { // 右：v-t 直线
    let ox = 11.4
    let oy = 5.65
    axes(ox - 1.7, ox + 1.9, oy - 0.7, oy + 0.9, overshoot: 0.25, tick-labels: false, x-label: none, y-label: none)
    line((ox - 1.3, oy - 0.55), (ox + 1.5, oy + 0.75), stroke: 1.2pt + phys-orange)
    content((ox + 1.0, oy + 0.85), text(size: 7.5pt, fill: phys-orange, [$v = a t$]), anchor: "west")
    content((ox - 1.55, oy + 0.62), text(size: 7.5pt, fill: ink, [v–t 图像]), anchor: "west")
  }

  // ============ 面板 2：解直角三角形 ↔ 力的分解 ============
  panel(2.45, 2, [斜面上重力的两个分量，就是解一个直角三角形])
  { // 左：直角三角形
    let ox = 1.3
    let oy = 2.9
    line((ox, oy), (ox + 2.6, oy), (ox + 2.6, oy + 1.5), close: true, stroke: 1pt + math-blue)
    rect((ox + 2.35, oy), (ox + 2.6, oy + 0.25), stroke: 0.6pt + math-blue)
    content((ox + 1.3, oy - 0.28), text(size: 7.5pt, fill: gray-line, [邻边]))
    content((ox + 2.92, oy + 0.75), text(size: 7.5pt, fill: gray-line, [对边]), anchor: "west")
    content((ox + 1.15, oy + 0.95), text(size: 7.5pt, fill: gray-line, [斜边]))
    content((ox + 0.42, oy + 0.2), text(size: 8pt, fill: warn-red, [$theta$]))
  }
  { // 右：斜面上物块 + 重力分解（全部由向量计算定位）
    let ax = 9.9
    let ay = 4.3
    let bx = 12.5
    let by = 2.8
    line((ax - 0.25, by), (bx + 0.35, by), stroke: 0.7pt + gray-line)
    line((ax, ay), (bx, by), stroke: 1pt + ink)
    let dx = bx - ax
    let dy = by - ay
    let len = calc.sqrt(dx * dx + dy * dy)
    let ux = dx / len // 沿斜面向下
    let uy = dy / len
    let nx = -uy // 垂直斜面向上
    let ny = ux
    let mx = (ax + bx) / 2 + nx * 0.28
    let my = (ay + by) / 2 + ny * 0.28
    let s = 0.26
    line((mx + (ux + nx) * s, my + (uy + ny) * s),
         (mx + (ux - nx) * s, my + (uy - ny) * s),
         (mx - (ux + nx) * s, my - (uy + ny) * s),
         (mx + (nx - ux) * s, my + (ny - uy) * s),
         close: true, fill: soft-orange, stroke: ink)
    // G 竖直向下
    let gtip = (mx, my - 1.05)
    vector((mx, my), gtip, paint: warn-red, thickness: 1pt, scale: 0.6, label: $G$, label-off: (0.15, -0.1), label-size: 7.5pt)
    // G·u 沿斜面向下、G·n 垂直压向斜面
    let g1 = (mx + ux * (-uy) * 0.95, my + uy * (-uy) * 0.95)
    let g2 = (mx + nx * (-ny) * 0.5, my + ny * (-ny) * 0.5)
    vector((mx, my), g1, paint: phys-orange, dash: "dashed", thickness: 0.9pt, scale: 0.5, label: $G_1$, label-size: 7pt, label-off: (0.18, -0.14))
    vector((mx, my), g2, paint: phys-orange, dash: "dashed", thickness: 0.9pt, scale: 0.5, label: $G_2$, label-size: 7pt, label-off: (0.32, -0.22))
    line(gtip, g1, stroke: (paint: gray-line, thickness: 0.5pt, dash: "dashed"))
    line(gtip, g2, stroke: (paint: gray-line, thickness: 0.5pt, dash: "dashed"))
  }

  // ============ 面板 3：天平 ↔ 守恒定律 ============
  panel(0.0, 3, [守恒定律就是一架天平：变化前后，「两边」始终相等])
  { // 左：天平
    let ox = 2.6
    let oy = 1.05
    line((ox - 1.5, oy + 0.75), (ox + 1.5, oy + 0.75), stroke: 1.1pt + ink)
    line((ox - 0.25, oy + 0.05), (ox + 0.25, oy + 0.05), (ox, oy + 0.72), close: true, fill: soft-gray, stroke: ink)
    line((ox - 1.5, oy + 0.75), (ox - 1.5, oy + 0.3), stroke: 0.6pt + gray-line)
    line((ox + 1.5, oy + 0.75), (ox + 1.5, oy + 0.3), stroke: 0.6pt + gray-line)
    rect((ox - 1.85, oy - 0.1), (ox - 1.15, oy + 0.3), fill: soft-blue, stroke: math-blue)
    rect((ox + 1.15, oy - 0.1), (ox + 1.85, oy + 0.3), fill: soft-orange, stroke: phys-orange)
    content((ox - 1.5, oy + 0.1), text(size: 7pt, fill: math-blue, [$m$]))
    content((ox + 1.5, oy + 0.1), text(size: 7pt, fill: phys-orange, [$m$]))
  }
  { // 右：碰撞前后动量
    let ox = 10.0
    let oy = 1.1
    circle((ox - 1.5, oy), radius: 0.32, fill: soft-blue, stroke: math-blue)
    content((ox - 1.5, oy), text(size: 7pt, fill: math-blue, [$m_1$]))
    circle((ox + 0.1, oy), radius: 0.42, fill: soft-orange, stroke: phys-orange)
    content((ox + 0.1, oy), text(size: 7pt, fill: phys-orange, [$m_2$]))
    vector((ox - 1.05, oy), (ox - 0.55, oy), paint: math-blue, thickness: 1pt, scale: 0.6, label: $v$, label-off: (0, 0.22), label-size: 7.5pt)
    content((ox + 1.15, oy), text(size: 10pt, fill: gray-line, [$->$]))
    circle((ox + 2.0, oy), radius: 0.32, fill: soft-blue, stroke: math-blue)
    circle((ox + 2.85, oy), radius: 0.42, fill: soft-orange, stroke: phys-orange)
    content((ox + 1.7, oy + 0.72), text(size: 7.5pt, fill: gray-line, [碰后 $m_1 v_1 + m_2 v_2$ 不变]))
  }
})
