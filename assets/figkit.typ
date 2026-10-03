// =====================================================================
// assets/figkit.typ — 理科插图模板库（个人常用图型，一键成图）
//
// 用法（文章内一行导入）：
//   #import "../assets/figkit.typ": *
//   #plot(-1, 5, -1, 4, curves: ((f: x => x * x, x0: 0, x1: 2.1),))
//
// 设计约定：
//   · 所有模板返回「成品图」：内部已完成 cetz.canvas 包装与 preview.fig
//     的 HTML/PDF 双模式适配，文章里直接放即可，无需 4 行样板；
//   · 颜色/字号/线宽只用本库与 series-styles 的统一梯度，禁止裸 rgb；
//   · 每个模板的参数「宽容默认」：只填关心的，其余用缺省；
//   · 组装特殊图用 raw(draw) + 零件（beaker/electrode/salt-bridge/gas-arrow），
//     与 series-styles 的 circuit-* 零件同一思路。
//
// 依赖：cetz 0.5.2（typst 0.14+）。
// =====================================================================
#import "@preview/cetz:0.5.2"
#import "series-styles.typ": *
#import "preview.typ": fig

// ---------- 扩展色（series-styles 未覆盖的材质色，集中在此） ----------
#let wood-tan  = rgb("#C9B38F") // 盐桥 / 软木塞
#let water-fill = rgb("#DCEBF7") // 溶液 / 液体填充

// ---------- 字号 / 线宽梯度（全库唯一来源） ----------
#let sz-title = 10pt    // 图内标题
#let sz-label = 8.5pt   // 坐标轴与节点文字
#let sz-note  = 7.5pt   // 注记与辅助说明
#let lw-axis  = 0.9pt   // 坐标轴
#let lw-curve = 1.15pt  // 主曲线
#let lw-guide = 0.6pt   // 虚参考线

// ---------- 内部工具 ----------

// 点到矩形边缘的缩短点：从中心 c 沿单位方向 d 前进到半宽 hw、半高 hh
// 的矩形边界（再外扩 pad），用于流程图连线自动避开节点框。
#let _edge-point(c, hw, hh, d, pad: 0.1) = {
  let dx = d.at(0)
  let dy = d.at(1)
  let L = calc.sqrt(dx * dx + dy * dy)
  if L < 1e-9 { return c }
  dx /= L
  dy /= L
  let tx = if calc.abs(dx) > 1e-9 { hw / calc.abs(dx) } else { 1e9 }
  let ty = if calc.abs(dy) > 1e-9 { hh / calc.abs(dy) } else { 1e9 }
  let t = calc.min(tx, ty) + pad
  (c.at(0) + dx * t, c.at(1) + dy * t)
}

// 线型构造：dash 为 none 时不带虚线
#let _stroke(color, thickness: lw-curve, dash: none) = {
  if dash == none { (paint: color, thickness: thickness) }
  else { (paint: color, thickness: thickness, dash: dash) }
}

// 列表参数容错：允许只传一个字典（Typst 里 ((k: v)) 是分组不是单元素数组）
#let _list(x) = if type(x) == dictionary { (x,) } else { x }

// 参考线容错：元素是数字或 (数字, 标签) 元组时，视为单条而非列表
#let _guide-list(x) = {
  if type(x) == array and x.len() > 0 and type(x.at(0)) in (int, float, str, content) { (x,) }
  else if type(x) == int or type(x) == float { ((x,),) }
  else { x }
}

// =====================================================================
// 模板 1：plot() —— 函数曲线图（v–t、钟形、滴定、J/S…… 的通用底座）
//
//   curves: 每条一个字典，三种形态任选其一：
//     (f: x => .., x0:, x1:, color:, dash:)      函数段
//     (pts: ((x, y), ..), color:, dash:)         折线段（数据）
//     (pieces: ((f, x0, x1), ..), color:)        分段函数（突变图）
//   marks: ((at: (x, y), t: [...], anchor: "west"), ..)  点 + 标注
//   h-guides / v-guides: ((y, label: none), ..)  水平/竖直虚参考线
// =====================================================================
#let plot(x-min, x-max, y-min, y-max,
    curves: (), marks: (), h-guides: (), v-guides: (),
    x-label: $x$, y-label: $y$, x-step: 1, y-step: 1,
    size: 0.72cm) = fig(cetz.canvas(length: size, {
  import cetz.draw: line, circle, content
  curves = _list(curves)
  marks = _list(marks)
  h-guides = _guide-list(h-guides)
  v-guides = _guide-list(v-guides)
  axes(x-min, x-max, y-min, y-max, x-step: x-step, y-step: y-step,
    x-label: x-label, y-label: y-label)
  // 虚参考线（先画，压在曲线下层）
  for g in h-guides {
    let (y, lab) = if type(g) == array { (g.at(0), g.at(1, default: none)) } else { (g, none) }
    line((x-min, y), (x-max, y), stroke: _stroke(gray-line, thickness: lw-guide, dash: "dashed"))
    if lab != none { content((x-max - 0.1, y + 0.12), text(size: sz-note, fill: gray-line, lab), anchor: "east") }
  }
  for g in v-guides {
    let (x, lab) = if type(g) == array { (g.at(0), g.at(1, default: none)) } else { (g, none) }
    line((x, y-min), (x, y-max), stroke: _stroke(gray-line, thickness: lw-guide, dash: "dashed"))
    if lab != none { content((x + 0.12, y-max - 0.1), text(size: sz-note, fill: gray-line, lab), anchor: "north") }
  }
  // 曲线
  for c in curves {
    let color = c.at("color", default: math-blue)
    let dash = c.at("dash", default: none)
    let st = _stroke(color, thickness: c.at("w", default: lw-curve), dash: dash)
    if c.at("f", default: none) != none {
      curve(c.f, c.at("x0", default: x-min), c.at("x1", default: x-max),
        stroke: st, yclip: c.at("yclip", default: none))
    } else if c.at("pts", default: none) != none {
      line(..c.pts, stroke: st)
    } else if c.at("pieces", default: none) != none {
      for p in c.pieces { curve(p.at(0), p.at(1), p.at(2), stroke: st) }
    }
    if c.at("label", default: none) != none {
      let at = c.at("label-at", default: (x-max - 0.15, y-max - 0.15))
      content(at, text(size: sz-label, fill: color, weight: "bold", c.label), anchor: "east")
    }
  }
  // 点标注
  for m in marks {
    let at = m.at("at")
    circle(at, radius: 0.06, fill: m.at("color", default: ink), stroke: none)
    if m.at("t", default: none) != none {
      content((at.at(0) + m.at("off", default: (0.15, 0.12)).at(0), at.at(1) + m.at("off", default: (0.15, 0.12)).at(1)),
        text(size: sz-note, fill: ink, m.t), anchor: m.at("anchor", default: "west"))
    }
  }
}))

// =====================================================================
// 模板 2：flow() —— 流程框图（分泌蛋白、光暗反应、实验流程……）
//
//   nodes: ((x:, y:, t: [...], fill:, st:, w:, h:), ..)   坐标即中心
//   links: ((a:, b:, label:, dash:, color:), ..)          a/b 为节点下标
//   连线自动缩短到节点框边缘，不必手算起止点。
// =====================================================================
#let flow(nodes, links: (), size: 0.85cm) = fig(cetz.canvas(length: size, {
  import cetz.draw: content
  nodes = _list(nodes)
  links = _list(links)
  // 先画连线（在框下层）
  for l in links {
    let na = nodes.at(l.a)
    let nb = nodes.at(l.b)
    let ca = (na.x, na.y)
    let cb = (nb.x, nb.y)
    let dx = cb.at(0) - ca.at(0)
    let dy = cb.at(1) - ca.at(1)
    let pa = _edge-point(ca, na.at("w", default: 2.6) / 2, na.at("h", default: 0.8) / 2, (dx, dy))
    let pb = _edge-point(cb, nb.at("w", default: 2.6) / 2, nb.at("h", default: 0.8) / 2, (-dx, -dy))
    arrow(pa, pb, paint: l.at("color", default: gray-line),
      dash: l.at("dash", default: none), label: l.at("label", default: none),
      label-pos: l.at("label-pos", default: 0.5))
  }
  // 再画节点
  for n in nodes {
    node((n.x, n.y), n.t,
      fill: n.at("fill", default: soft-blue), stroke: n.at("st", default: math-blue),
      w: n.at("w", default: 2.6), h: n.at("h", default: 0.8),
      text-size: n.at("size", default: sz-label))
  }
}))

// =====================================================================
// 模板 3：cycle() —— 关系环 / 转化三角（铝三角、铁三角、位—构—性、物质循环）
//
//   labels: ([Al³⁺], [Al(OH)₃], ..)   n 个节点，正多边形自动摆位（首节点在顶部）
//   edges:  ((a:, b:, label:, dash:), ..)   下标连线，自动避开节点
// =====================================================================
#let cycle(labels, edges: (), radius: auto, size: 0.8cm) = fig(cetz.canvas(length: size, {
  import cetz.draw: content
  edges = _list(edges)
  let n = labels.len()
  let R = if radius == auto { if n <= 3 { 2.9 } else if n == 4 { 3.1 } else { 3.5 } } else { radius }
  let hw = 1.15  // 节点半宽（近似，用于连线缩短）
  let hh = 0.45
  let pos(i) = {
    let a = (90 - i * 360 / n) * calc.pi / 180
    (R * calc.cos(a), R * calc.sin(a))
  }
  // 连线（下层）
  for e in edges {
    let pa = pos(e.a)
    let pb = pos(e.b)
    let dx = pb.at(0) - pa.at(0)
    let dy = pb.at(1) - pa.at(1)
    arrow(_edge-point(pa, hw, hh, (dx, dy), pad: 0.06),
          _edge-point(pb, hw, hh, (-dx, -dy), pad: 0.06),
      paint: e.at("color", default: gray-line), dash: e.at("dash", default: none),
      label: e.at("label", default: none), label-pos: e.at("label-pos", default: 0.5))
  }
  // 节点
  for (i, lab) in labels.enumerate() {
    let p = pos(i)
    node(p, lab, w: 2.3, h: 0.8, fill: soft-blue, stroke: math-blue, text-size: sz-label)
  }
}))

// =====================================================================
// 模板 4：card() —— 全景卡（标题栏 + 彩色节点阵列，一类知识一张卡）
//
//   title: [...]           卡片标题（橙底白字横栏）
//   items:  (t: [...], fill:, st:, ..) 或直接 [文字]（默认蓝系）
//   columns: 3             每行几个
//   footnote: none         底部灰色小注
// =====================================================================
#let card(title, items, columns: 3, footnote: none, size: 0.85cm) = fig(cetz.canvas(length: size, {
  import cetz.draw: content
  items = _list(items)
  let cw = 3.0   // 单元宽
  let ch = 1.15  // 单元高
  let gx = 0.35  // 列间距
  let gy = 0.5   // 行间距
  let rows = calc.ceil(items.len() / columns)
  let cols = calc.min(columns, items.len())
  let W = cols * cw + (cols - 1) * gx
  let H = rows * ch + (rows - 1) * gy
  let y0 = H / 2   // 网格顶部（标题栏在 y0 + 0.85）
  node((0, y0 + 0.85), title, w: W + 0.4, h: 0.9, fill: phys-orange, stroke: none,
    text-fill: white, weight: "bold", text-size: sz-title)
  for (i, it) in items.enumerate() {
    let r = calc.floor(i / columns)
    let c = calc.rem(i, columns)
    // 行内元素不足时整行居中
    let in-row = calc.min(columns, items.len() - r * columns)
    let row-w = in-row * cw + (in-row - 1) * gx
    let x = -row-w / 2 + cw / 2 + c * (cw + gx)
    let y = y0 - ch / 2 - r * (ch + gy)
    let t = if type(it) == dictionary { it.t } else { it }
    node((x, y), t, w: cw, h: ch,
      fill: if type(it) == dictionary { it.at("fill", default: soft-blue) } else { soft-blue },
      stroke: if type(it) == dictionary { it.at("st", default: math-blue) } else { math-blue },
      text-size: sz-label)
  }
  if footnote != none {
    content((0, -H / 2 - 0.75), text(size: sz-note, fill: gray-line, footnote))
  }
}))

// =====================================================================
// 模板 5：number-line() —— 数轴与区间（集合、不等式解集）
//
//   points:    ((x, "solid"|"open"), ..)     实心 / 空心点
//   intervals: ((a, b), ..)                  有限区间（a/b 为 none 表示射线）
//   step: 1    刻度步长；ticks: true 显示数字
// =====================================================================
#let number-line(x-min, x-max,
    points: (), intervals: (), step: 1, ticks: true, size: 0.75cm) = fig(cetz.canvas(length: size, {
  import cetz.draw: line, circle, content
  points = _list(points)
  intervals = _list(intervals)
  let y = 0
  line((x-min, y), (x-max + 0.4, y), stroke: _stroke(ink, thickness: lw-axis), mark: (end: ">>", fill: ink, scale: 0.55))
  if ticks {
    let t = calc.ceil(x-min / step) * step
    while t <= x-max + 1e-9 {
      line((t, y - 0.1), (t, y + 0.1), stroke: _stroke(ink, thickness: 0.7pt))
      content((t, y - 0.42), text(size: sz-note, fill: gray-line, fnum(t)))
      t += step
    }
  }
  // 区间粗线（画在轴上方 0.22 处）
  for iv in intervals {
    let a = iv.at(0)
    let b = iv.at(1)
    let ya = 0.22
    let a-open = if type(a) == dictionary { a.at("open", default: false) } else { false }
    let b-open = if type(b) == dictionary { b.at("open", default: false) } else { false }
    let ax = if type(a) == dictionary { a.at("x") } else { a }
    let bx = if type(b) == dictionary { b.at("x") } else { b }
    let left = if ax == none { x-min - 0.15 } else { ax }
    let right = if bx == none { x-max + 0.15 } else { bx }
    line((left, ya), (right, ya), stroke: _stroke(math-blue, thickness: 1.6pt))
    if ax == none { line((x-min - 0.5, ya), (x-min - 0.05, ya), stroke: _stroke(math-blue, thickness: 1.6pt), mark: (end: ">", fill: math-blue)) }
    if bx == none { line((x-max + 0.05, ya), (x-max + 0.5, ya), stroke: _stroke(math-blue, thickness: 1.6pt), mark: (end: ">", fill: math-blue)) }
    if ax != none { circle((ax, ya), radius: 0.09, fill: if a-open { white } else { math-blue }, stroke: _stroke(math-blue, thickness: 0.9pt)) }
    if bx != none { circle((bx, ya), radius: 0.09, fill: if b-open { white } else { math-blue }, stroke: _stroke(math-blue, thickness: 0.9pt)) }
  }
  // 独立点
  for p in points {
    let (x, style) = (p.at(0), p.at(1))
    circle((x, y), radius: 0.09, fill: if style == "open" { white } else { ink }, stroke: _stroke(ink, thickness: 0.9pt))
  }
}))

// =====================================================================
// 组合通道：raw() + 装置零件 —— 原电池、制备装置、滴定管……
//
//   raw(draw, size) 提供空画布；零件为「绘制指令」，在闭包内调用：
//     beaker(x, y, w, h, liquid: 0.6)   烧杯（x,y 为左下角；liquid 液面高度比例）
//     electrode(x, y-top, y-bot, paint) 电极竖板
//     salt-bridge(x1, x2, y-top, y-bot) 倒 U 盐桥
//     gas-arrow(a, b, label)            气流箭头
// =====================================================================
#let raw(draw, size: 0.8cm) = fig(cetz.canvas(length: size, draw))

#let beaker(x, y, w, h, liquid: none, liquid-fill: water-fill) = {
  import cetz.draw: line, rect
  line((x, y + h), (x, y), stroke: _stroke(ink, thickness: 1.1pt))          // 左壁
  line((x, y), (x + w, y), stroke: _stroke(ink, thickness: 1.1pt))          // 底
  line((x + w, y), (x + w, y + h), stroke: _stroke(ink, thickness: 1.1pt))  // 右壁
  if liquid != none {
    rect((x + 0.03, y + 0.03), (x + w - 0.03, y + h * liquid), fill: liquid-fill, stroke: none)
  }
}

#let electrode(x, y-top, y-bot, paint: ink, label: none, label-off: (0, 0.42)) = {
  import cetz.draw: line, content
  line((x, y-top), (x, y-bot), stroke: (paint: paint, thickness: 3pt))
  if label != none {
    content((x + label-off.at(0), y-top + label-off.at(1)), text(size: sz-label, fill: ink, label))
  }
}

#let salt-bridge(x1, x2, y-top, y-bot) = {
  import cetz.draw: line, content
  line((x1, y-bot), (x1, y-top), (x2, y-top), (x2, y-bot),
    stroke: (paint: wood-tan, thickness: 4.5pt))
  content(((x1 + x2) / 2, y-top + 0.4), text(size: sz-note, fill: gray-line, [盐桥]))
}

#let gas-arrow(a, b, label: none) = {
  arrow(a, b, paint: gray-line, label: label, label-pos: 0.5, label-off: (0, 0.25))
}
