// s3-fig-13-modeling.typ — 数学建模流程：迭代闭环
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let steps = (
    (x: 1.8, y: 4.2, t: [① 实际问题], fill: soft-gray, st: gray-line),
    (x: 5.7, y: 4.2, t: [② 合理假设], fill: soft-blue, st: math-blue),
    (x: 9.6, y: 4.2, t: [③ 建立模型], fill: soft-blue, st: math-blue),
    (x: 13.4, y: 4.2, t: [④ 求解], fill: soft-orange, st: phys-orange),
    (x: 13.4, y: 1.6, t: [⑤ 验证与解释], fill: soft-green, st: green-ok),
    (x: 9.6, y: 1.6, t: [不符合 → 修假设], fill: soft-red, st: warn-red),
    (x: 5.7, y: 1.6, t: [符合 → 撰写报告], fill: soft-green, st: green-ok),
  )
  for s in steps {
    node((s.x, s.y), s.t, w: 3.2, h: 1.1, fill: s.fill, stroke: s.st, text-size: 8.5pt, weight: "bold")
  }
  arrow((3.5, 4.2), (4.0, 4.2), paint: gray-line, thickness: 1.2pt)
  arrow((7.4, 4.2), (7.9, 4.2), paint: gray-line, thickness: 1.2pt)
  arrow((11.3, 4.2), (11.7, 4.2), paint: gray-line, thickness: 1.2pt)
  arrow((13.4, 3.6), (13.4, 2.2), paint: gray-line, thickness: 1.2pt)
  arrow((11.7, 1.6), (11.3, 1.6), paint: gray-line, thickness: 1.2pt)
  arrow((8.0, 1.6), (7.3, 1.6), paint: gray-line, thickness: 1.2pt)
  // 迭代回路
  arrow((5.7, 2.2), (5.7, 3.6), paint: warn-red, thickness: 1.1pt, dash: "dashed", label: [回到②], label-off: (1.0, 0), label-size: 7.5pt)
})
