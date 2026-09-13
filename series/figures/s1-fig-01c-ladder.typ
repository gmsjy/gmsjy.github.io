// s1-fig-01c-ladder.typ — 能力五级台阶：掉队发生在哪一级
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let step-w = 2.45
#let names = ([记住], [听懂], [会模仿], [能迁移], [会提问])
#let fills = (soft-gray, soft-blue, soft-blue, soft-orange, soft-orange)
#let strokes = (gray-line, math-blue, math-blue, phys-orange, phys-orange)

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  for i in range(5) {
    let x0 = i * step-w
    let x1 = (i + 1) * step-w
    let h = 0.95 * (i + 1)
    rect((x0, 0), (x1, h), fill: fills.at(i), stroke: (paint: strokes.at(i), thickness: 0.7pt))
    content((x0 + step-w / 2, h - 0.38), text(size: 8.5pt, weight: "bold", fill: strokes.at(i), [L#(i + 1) ] + names.at(i)))
  }

  // 多数掉队者位置：L2 → L3 的跨越
  circle((3.4, 1.9), radius: 0.13, fill: warn-red, stroke: none)
  content((2.6, 3.15), text(size: 8pt, fill: warn-red, weight: "bold", [多数掉队者卡在 L2 → L3]))
  arrow((3.3, 2.9), (3.4, 2.08), paint: warn-red, thickness: 0.7pt, scale: 0.5)

  // 高中课堂默认起点
  content((6.1, 3.6), text(size: 8pt, fill: phys-orange, weight: "bold", [高中课堂默认从 L3 起步]))
  arrow((6.1, 3.42), (6.1, 3.0), paint: phys-orange, thickness: 0.7pt, scale: 0.5)

  // 高考最终要求
  content((10.9, 5.35), text(size: 8pt, fill: green-ok, weight: "bold", [高考最终要求：L4 及以上]))
  arrow((10.9, 5.18), (10.9, 4.9), paint: green-ok, thickness: 0.7pt, scale: 0.5)

  // 底部时间轴方向
  line((0, -0.45), (12.9, -0.45), stroke: 0.9pt + gray-line, mark: (end: ">>", fill: gray-line, scale: 0.55))
  content((12.6, -0.82), text(size: 8pt, fill: gray-line, [投入的时间与练习量]))
})
