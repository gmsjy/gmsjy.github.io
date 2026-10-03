// s1-fig-03b-flow.typ — 能力诊断流程：从一张卷子到一张处方
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  let steps = (
    (y: 5.4, t: [① 做一套 20 题诊断卷], d: [五个维度各 4 题，限时 40 分钟], fill: soft-blue, st: math-blue),
    (y: 4.1, t: [② 逐题标注错因], d: [不会做 / 会但慢 / 算错 / 读不懂题], fill: soft-blue, st: math-blue),
    (y: 2.8, t: [③ 按维度计分], d: [每维得分率填进雷达图], fill: soft-orange, st: phys-orange),
    (y: 1.5, t: [④ 找出最短板], d: [得分率最低的一维 = 未来 3 周主攻], fill: soft-orange, st: phys-orange),
    (y: 0.2, t: [⑤ 领取对策清单], d: [短板维度 → 专属练习类型], fill: soft-green, st: green-ok),
  )
  for s in steps {
    node((3.4, s.y), s.t, w: 4.6, h: 0.85, fill: s.fill, stroke: s.st, text-size: 9pt, weight: "bold")
    content((7.9, s.y), text(size: 8pt, fill: gray-line, s.d), anchor: "west")
  }
  for i in range(4) {
    arrow((3.4, steps.at(i).y - 0.5), (3.4, steps.at(i + 1).y + 0.5), paint: gray-line, thickness: 1.1pt)
  }
  // 回诊
  line((10.6, 0.2), (10.6, 5.4), stroke: (paint: warn-red, thickness: 0.8pt, dash: "dashed"))
  arrow((10.6, 5.4), (5.75, 5.4), paint: warn-red, thickness: 0.8pt, dash: "dashed")
  content((11.0, 2.8), text(size: 8pt, fill: warn-red, [3 周后重测]), anchor: "west")
})
