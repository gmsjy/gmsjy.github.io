// s3-fig-18c-knowledge.typ — 知识管理：输入 → 整理 → 输出 → 复用
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let nodes = (
    (x: 7.0, y: 4.6, t: [① 输入：课堂·书·论文], fill: soft-blue, st: math-blue),
    (x: 12.6, y: 2.6, t: [② 整理：结构图化笔记], fill: soft-blue, st: math-blue),
    (x: 7.0, y: 0.6, t: [③ 输出：讲解·写作·项目], fill: soft-orange, st: phys-orange),
    (x: 1.4, y: 2.6, t: [④ 复用：错题库·模板库], fill: soft-green, st: green-ok),
  )
  for n in nodes {
    node((n.x, n.y), n.t, w: 4.6, h: 1.2, fill: n.fill, stroke: n.st, text-size: 9pt, weight: "bold")
  }
  arrow((9.35, 4.35), (11.35, 3.05), paint: gray-line, thickness: 1.2pt)
  arrow((11.35, 2.15), (9.35, 0.85), paint: gray-line, thickness: 1.2pt)
  arrow((4.65, 0.85), (2.65, 2.15), paint: gray-line, thickness: 1.2pt)
  arrow((2.65, 3.05), (4.65, 4.35), paint: gray-line, thickness: 1.2pt)
  content((7.0, 2.6), text(size: 9.5pt, fill: ink, weight: "bold", align(center, [终身学习闭环])))
  content((7.0, 5.4), text(size: 8.5pt, fill: gray-line, align(center, [输出倒逼输入：能讲给别人听的知识才真正属于你；#linebreak()复用让每一次学习都为下一次垫高起点])))
})
