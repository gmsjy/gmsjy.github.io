// s3-fig-15c-cluster.typ — 聚类：无标签数据自动分组
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 6.9, -0.3, 4.4, x-step: 1, y-step: 1, x-label: [兴趣], y-label: [成绩], tick-labels: false)
  // 三个簇
  let clusters = (((1.6, 3.4), math-blue), ((5.2, 3.5), green-ok), ((3.4, 1.0), phys-orange))
  let pts = (
    ((1.2, 3.1), (2.1, 3.6), (1.8, 2.9), (2.4, 3.2)),
    ((4.9, 3.1), (5.6, 3.7), (5.3, 3.0), (5.9, 3.4)),
    ((3.0, 0.8), (3.8, 1.3), (3.4, 0.9), (3.9, 1.2)),
  )
  for ci in range(3) {
    let col = clusters.at(ci).at(1)
    for p in pts.at(ci) { circle(p, radius: 0.09, fill: col.lighten(40%), stroke: 1pt + col) }
    // 簇中心
    circle(clusters.at(ci).at(0), radius: 0.14, fill: col, stroke: none)
  }
  content((5.9, 0.35), text(size: 8.5pt, fill: gray-line, align(center, [没有标签也能自动分成三群：#linebreak()K 均值等算法反复「找中心、归群」，直到稳定])))
})
