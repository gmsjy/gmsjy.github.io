// s1-fig-02c-route.typ — 衔接行动路线：五步闭环 + 回诊箭头
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content

  let stages = (
    (x: 1.55, fill: soft-blue, st: math-blue, t: [① 定位诊断#linebreak()用月考卷找断层], time: [8 月上旬]),
    (x: 4.3, fill: soft-blue, st: math-blue, t: [② 旧知回炉#linebreak()每天 30 分钟], time: [8 月中下旬]),
    (x: 7.05, fill: soft-green, st: green-ok, t: [③ 同步跟课#linebreak()先保住主线], time: [9 月起 · 持续]),
    (x: 9.8, fill: soft-orange, st: phys-orange, t: [④ 方法复盘#linebreak()错题每周清], time: [每周末 30 分钟]),
    (x: 12.55, fill: soft-orange, st: phys-orange, t: [⑤ 适度拓展#linebreak()期末再眺望], time: [期末 / 寒假]),
  )
  for s in stages {
    node((s.x, 2.4), s.t, w: 2.5, h: 1.1, fill: s.fill, stroke: s.st, text-size: 7.8pt)
    content((s.x, 1.45), text(size: 7.8pt, fill: gray-line, s.time))
  }
  for i in range(4) {
    arrow((stages.at(i).x + 1.3, 2.4), (stages.at(i + 1).x - 1.3, 2.4), paint: gray-line, thickness: 1pt)
  }

  // 回诊虚线：④ → ①
  line((9.8, 1.75), (9.8, 0.55), stroke: (paint: warn-red, thickness: 0.8pt, dash: "dashed"))
  line((9.8, 0.55), (1.55, 0.55), stroke: (paint: warn-red, thickness: 0.8pt, dash: "dashed"))
  arrow((1.55, 0.55), (1.55, 1.8), paint: warn-red, thickness: 0.8pt, dash: "dashed")
  content((5.7, 0.82), text(size: 8pt, fill: warn-red, weight: "bold", [每月月底重新诊断一次，更新断层清单]))

  // 警示
  content((12.2, 4.35), text(size: 8pt, fill: warn-red, weight: "bold", [⚠ 还没走稳（L3 之前），不要狂刷压轴难题]))
})
