// s1-fig-07b-interval.typ — 数轴区间：三种解集的画法
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let numline(y, expr, x0, solid, fillto, fillright) = {
  import cetz.draw: line, content, circle, rect
  line((0.8, y), (8.6, y), stroke: 0.9pt + gray-line, mark: (end: ">>", fill: gray-line, scale: 0.5))
  circle((x0, y), radius: 0.09, fill: if solid { warn-red } else { white }, stroke: 1pt + warn-red)
  content((x0, y + 0.5), text(size: 8.5pt, fill: ink, expr))
  if fillright {
    rect((x0, y - 0.09), (8.3, y + 0.09), fill: soft-orange.lighten(20%), stroke: none)
  }
  if fillright == false {
    rect((1.0, y - 0.09), (x0, y + 0.09), fill: soft-orange.lighten(20%), stroke: none)
  }
  content((9.0, y), text(size: 8.5pt, fill: ink, if fillright { [向右延伸] } else { [向左延伸] }), anchor: "west")
}

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  numline(3.4, [$x > 2$（空心，不含 2）], 3.4, false, false, true)
  numline(1.8, [$x >= -1$（实心，含 -1）], 1.4, true, false, true)
  numline(0.2, [$x <= 0.5$（实心，含 0.5）], 4.6, true, false, false)
  content((4.7, -0.8), text(size: 8.5pt, fill: gray-line, [空心 = 不取等号；实心 = 取等号。高中的解集几乎都要写成区间或数轴形式]))
})
