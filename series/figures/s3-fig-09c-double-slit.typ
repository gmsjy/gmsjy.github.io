// s3-fig-09c-double-slit.typ — 双缝干涉：条纹间距公式
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 双缝
  rect((1.4, 1.4), (1.7, 4.6), fill: soft-gray, stroke: ink)
  rect((1.4, 2.85), (1.7, 3.15), fill: white, stroke: ink)
  rect((1.4, 1.85), (1.7, 2.15), fill: white, stroke: ink)
  content((1.55, 5.0), text(size: 8pt, fill: ink, [双缝 $d$]))
  // 屏
  rect((6.6, 1.0), (6.9, 5.0), fill: soft-gray, stroke: ink)
  content((6.75, 5.35), text(size: 8pt, fill: ink, [屏]))
  // 中央亮纹与相邻条纹
  for y in (1.7, 2.45, 3.0, 3.55, 4.3) {
    line((1.75, y), (6.55, y), stroke: (paint: gray-line, thickness: 0.4pt, dash: "dashed"))
  }
  for (y, c) in ((3.0, warn-red), (2.45, math-blue), (3.55, math-blue), (1.7, math-blue), (4.3, math-blue)) {
    rect((6.62, y - 0.12), (6.88, y + 0.12), fill: c.lighten(50%), stroke: c)
  }
  content((7.35, 3.0), text(size: 7.5pt, fill: warn-red, [中央亮纹]), anchor: "west")
  content((7.35, 2.45), text(size: 7.5pt, fill: math-blue, [相邻亮纹]), anchor: "west")
  line((6.95, 2.45), (6.95, 3.0), stroke: 0.8pt + ink)
  content((6.82, 2.72), text(size: 8pt, fill: ink, weight: "bold", [Δx]), anchor: "east")
  content((4.2, 0.45), text(size: 9pt, fill: ink, weight: "bold", align(center, [条纹间距 $Delta x = lambda L / d$：红光条纹比紫光宽，缝距越大条纹越密])))
})
