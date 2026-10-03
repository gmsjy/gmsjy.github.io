// s3-fig-11b-matrix.typ — 矩阵变换：单位正方形的「变形记」
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.6, 4.9, -0.6, 4.4, x-step: 1, y-step: 1)
  // 单位正方形
  line((0, 0), (1, 0), (1, 1), (0, 1), close: true, fill: soft-blue.lighten(30%), stroke: 1.2pt + math-blue)
  // 变换后（矩阵 [[2,1],[1,2]]）
  let tp = ((0,0), (2,1), (3,3), (1,2))
  line(..tp, close: true, fill: soft-orange.lighten(20%), stroke: 1.2pt + phys-orange)
  content((3.4, 3.5), text(size: 8.5pt, fill: phys-orange, weight: "bold", [变换后]))
  content((0.45, 1.35), text(size: 8.5pt, fill: math-blue, weight: "bold", [原正方形]))
  content((5.3, 1.15), text(size: 8pt, fill: gray-line, align(center, [矩阵 = 一次线性变换：#linebreak()旋转、拉伸、剪切、投影])))
})
