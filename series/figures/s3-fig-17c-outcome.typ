// s3-fig-17c-outcome.typ — 成果树：一个项目的多重产出
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  node((7.0, 1.0), [一个完整的研究项目], w: 5.4, h: 1.0, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 10pt)
  let branches = (
    (x: 2.2, t: [报告 / 论文#linebreak()综合素质档案], fill: soft-blue, st: math-blue),
    (x: 7.0, t: [能力证据#linebreak()面试答辩素材], fill: soft-orange, st: phys-orange),
    (x: 11.8, t: [延伸课题#linebreak()下一阶段的种子], fill: soft-green, st: green-ok),
  )
  for b in branches {
    arrow((7.0, 1.55), (b.x, 2.9), paint: gray-line, thickness: 1.1pt)
    node((b.x, 3.45), b.t, w: 3.8, h: 1.15, fill: b.fill, stroke: b.st, text-size: 8.5pt)
  }
  // 延伸再延伸
  arrow((11.8, 4.05), (9.4, 5.0), paint: gray-line, thickness: 0.9pt, dash: "dashed")
  arrow((11.8, 4.05), (14.0, 5.0), paint: gray-line, thickness: 0.9pt, dash: "dashed")
  content((9.0, 5.25), text(size: 7.8pt, fill: gray-line, [改进版选题]))
  content((14.15, 5.25), text(size: 7.8pt, fill: gray-line, [相邻学科迁移]), anchor: "east")
  content((7.0, -0.05), text(size: 8.5pt, fill: gray-line, align(center, [好项目会「结果」也会「播种」：答辩中的追问，常常就是下一个选题])))
})
