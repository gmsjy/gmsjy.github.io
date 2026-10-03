// s1-fig-15-learning-loop.typ — 学习闭环：六环节循环圈
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  let cx = 7.0
  let cy = 3.0
  let r = 2.5
  let names = ([预习], [听课], [笔记], [练习], [错题], [复盘])
  let subs = ([留问题], [抓主线], [结构化], [限时做], [当天清], [找规律])
  for i in range(6) {
    let a = 90deg - 60deg * i
    let px = cx + r * calc.cos(a)
    let py = cy + r * calc.sin(a)
    node((px, py), names.at(i) + linebreak() + subs.at(i), w: 1.7, h: 1.1,
      fill: if calc.odd(i) { soft-blue } else { soft-orange },
      stroke: if calc.odd(i) { math-blue } else { phys-orange }, text-size: 8pt)
  }
  // 环形箭头
  for i in range(6) {
    let a1 = 90deg - 60deg * i - 14deg
    let a2 = 90deg - 60deg * (i + 1) + 14deg
    let p1 = (cx + r * calc.cos(a1), cy + r * calc.sin(a1))
    let p2 = (cx + r * calc.cos(a2), cy + r * calc.sin(a2))
    arrow(p1, p2, paint: gray-line, thickness: 1.2pt)
  }
  content((cx, cy), text(size: 9.5pt, fill: ink, weight: "bold", align(center, [学习闭环#linebreak()每天转半圈，#linebreak()每周转满一圈])))
  content((cx, -0.4), text(size: 8.5pt, fill: gray-line, align(center, [多数人只做中间两环（听课＋练习），#linebreak()缺的头尾两环恰恰是拉开差距的部分])))
})
