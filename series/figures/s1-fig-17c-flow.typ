// s1-fig-17c-flow.typ — CeTZ 画图的迭代流程
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let steps = (
    (x: 1.9, t: [① 画草图], d: [纸上定大致布局], fill: soft-gray, st: gray-line),
    (x: 5.7, t: [② 写坐标], d: [先轴与框架，再内容], fill: soft-blue, st: math-blue),
    (x: 9.5, t: [③ 编译预览], d: [typst compile 一键出图], fill: soft-orange, st: phys-orange),
    (x: 13.2, t: [④ 微调], d: [挪标签、改颜色], fill: soft-green, st: green-ok),
  )
  for s in steps {
    node((s.x, 2.4), s.t, w: 2.9, h: 1.0, fill: s.fill, stroke: s.st, text-size: 9pt, weight: "bold")
    content((s.x, 1.5), text(size: 7.8pt, fill: gray-line, s.d))
  }
  arrow((3.4, 2.4), (4.2, 2.4), paint: gray-line, thickness: 1.2pt)
  arrow((7.2, 2.4), (8.0, 2.4), paint: gray-line, thickness: 1.2pt)
  arrow((11.0, 2.4), (11.7, 2.4), paint: gray-line, thickness: 1.2pt)
  arrow((13.2, 1.7), (13.2, 0.7), paint: gray-line, thickness: 1pt)
  arrow((13.2, 0.7), (5.7, 0.7), paint: gray-line, thickness: 1pt, dash: "dashed", label: [不满意就回炉], label-off: (0, 0.3))
  arrow((5.7, 0.7), (5.7, 1.85), paint: gray-line, thickness: 1pt, dash: "dashed")
})
