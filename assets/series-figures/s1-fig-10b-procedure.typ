// s1-fig-10b-procedure.typ — 受力分析四步流程
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let steps = (
    (y: 4.6, t: [① 定对象：把谁「隔离」出来？], d: [单独画一个物体，别贪多], fill: soft-blue, st: math-blue),
    (y: 3.3, t: [② 画重力：竖直向下，先画它], d: [重心处，永远存在], fill: soft-blue, st: math-blue),
    (y: 2.0, t: [③ 找接触：有几个面挨着？], d: [每个接触点查「支持/拉/摩」], fill: soft-orange, st: phys-orange),
    (y: 0.7, t: [④ 查闭合：合不上就漏了力], d: [平衡时合力为零，加速度题对牛二], fill: soft-green, st: green-ok),
  )
  for s in steps {
    node((3.3, s.y), s.t, w: 5.4, h: 0.95, fill: s.fill, stroke: s.st, text-size: 9pt, weight: "bold")
    content((7.3, s.y), text(size: 8pt, fill: gray-line, s.d), anchor: "west")
  }
  for i in range(3) {
    arrow((3.3, steps.at(i).y - 0.55), (3.3, steps.at(i + 1).y + 0.55), paint: gray-line, thickness: 1.1pt)
  }
  content((3.3, 5.55), text(size: 9.5pt, fill: ink, weight: "bold", [受力分析口诀：一重二弹三摩擦，最后检查合不合]))
})
