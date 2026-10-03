// s2-fig-01c-sign-table.typ — 导数符号分析表（列表法模板）
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let xs = (0, 2.3, 4.6, 6.9, 9.2, 11.5, 13.8)
#let rows-y = (0, -1.1, -2.2)

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  // 表格线
  for y in rows-y + (-3.3,) {
    line((0, y), (13.8, y), stroke: (paint: gray-line, thickness: if y == 0 or y == -3.3 { 1pt } else { 0.5pt }))
  }
  for x in xs {
    line((x, 0), (x, -3.3), stroke: (paint: gray-line, thickness: if x == 0 { 1pt } else { 0.5pt }))
  }

  // 表头行
  content((1.15, -0.55), text(size: 8.5pt, weight: "bold", fill: ink, [$x$]))
  content((3.45, -0.55), text(size: 8pt, fill: ink, [$(-oo, -1)$]))
  content((5.75, -0.55), text(size: 8.5pt, fill: ink, [$-1$]))
  content((8.05, -0.55), text(size: 8pt, fill: ink, [$(-1, 1)$]))
  content((10.35, -0.55), text(size: 8.5pt, fill: ink, [$1$]))
  content((12.65, -0.55), text(size: 8pt, fill: ink, [$(1, +oo)$]))

  // f'(x) 行
  content((1.15, -1.65), text(size: 8.5pt, weight: "bold", fill: phys-orange, [$f'(x)$]))
  content((3.45, -1.65), text(size: 9pt, fill: green-ok, weight: "bold", [$+$]))
  content((5.75, -1.65), text(size: 9pt, fill: ink, weight: "bold", [$0$]))
  content((8.05, -1.65), text(size: 9pt, fill: warn-red, weight: "bold", [$-$]))
  content((10.35, -1.65), text(size: 9pt, fill: ink, weight: "bold", [$0$]))
  content((12.65, -1.65), text(size: 9pt, fill: green-ok, weight: "bold", [$+$]))

  // f(x) 行
  content((1.15, -2.75), text(size: 8.5pt, weight: "bold", fill: math-blue, [$f(x)$]))
  content((3.45, -2.75), text(size: 8.5pt, fill: green-ok, [↗ 增]))
  content((5.75, -2.75), text(size: 8pt, fill: ink, [极大值 $2$]))
  content((8.05, -2.75), text(size: 8.5pt, fill: warn-red, [↘ 减]))
  content((10.35, -2.75), text(size: 8pt, fill: ink, [极小值 $-2$]))
  content((12.65, -2.75), text(size: 8.5pt, fill: green-ok, [↗ 增]))

  // 顶部说明：研究对象
  node((6.9, 0.95), [$f(x) = x^3 - 3x,$  $x in RR$], w: 5.6, h: 0.75,
    fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
})
