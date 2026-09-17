// s2-fig-07c-table.typ — 2×2 列联表：两类变量是否相关
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  // 表头
  node((2.3, 0), [], w: 2.6, h: 0.75, fill: gray-line, stroke: none)
  node((6.4, 0), [近视], w: 3.8, h: 0.75, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 9pt)
  node((10.6, 0), [不近视], w: 3.8, h: 0.75, fill: soft-blue, stroke: math-blue, text-fill: math-blue, weight: "bold", text-size: 9pt)
  node((14.8, 0), [合计], w: 2.8, h: 0.75, fill: soft-gray, stroke: gray-line, weight: "bold", text-size: 9pt)
  let rows = (
    (y: -1.4, a: [户外时间少], b: [60], c: [20], d: [80]),
    (y: -2.8, a: [户外时间多], b: [30], c: [50], d: [80]),
    (y: -4.2, a: [合计], b: [90], c: [70], d: [160]),
  )
  for r in rows {
    let fill = if r.a == [合计] { soft-gray } else { white }
    node((2.3, r.y), r.a, w: 2.6, h: 1.2, fill: if r.a == [合计] { soft-gray } else { soft-orange }, stroke: phys-orange, text-size: 9pt, weight: "bold")
    node((6.4, r.y), r.b, w: 3.8, h: 1.2, fill: fill, stroke: gray-line, text-size: 10pt)
    node((10.6, r.y), r.c, w: 3.8, h: 1.2, fill: fill, stroke: gray-line, text-size: 10pt)
    node((14.8, r.y), r.d, w: 2.8, h: 1.2, fill: fill, stroke: gray-line, text-size: 10pt)
  }
  content((8.6, -5.5), text(size: 8.5pt, fill: gray-line, align(center, [若「户外少」组的近视率显著更高（卡方检验），#linebreak()就可以说户外时间与近视*相关*——但相关不等于因果])))
})
