// s2-fig-04-inequality.typ — 线性规划：可行域与目标函数
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.4, 5.6, -0.4, 4.6, x-step: 1, y-step: 1)
  // 约束：x>=1, y>=1, x+y<=5 → 顶点 (1,1) (1,4) (4,1)
  let poly = ((1, 1), (1, 4), (4, 1))
  line(..poly, close: true, fill: soft-blue.lighten(40%), stroke: 1.2pt + math-blue)
  for p in poly { circle(p, radius: 0.08, fill: math-blue, stroke: none) }
  content((1.0, 4.3), text(size: 8pt, fill: math-blue, weight: "bold", [(1, 4)]))
  content((4.15, 0.75), text(size: 8pt, fill: math-blue, weight: "bold", [(4, 1)]))
  content((0.7, 0.7), text(size: 8pt, fill: math-blue, weight: "bold", [(1, 1)]))
  // 目标函数等值线 z = x + 2y
  for z in ((3, gray-line), (6, phys-orange), (9, warn-red)) {
    let zv = z.at(0)
    line((0, zv / 2), (zv, 0), stroke: (paint: z.at(1), thickness: 0.8pt, dash: "dashed"))
  }
  content((4.85, 1.0), text(size: 8.5pt, fill: warn-red, weight: "bold", [目标线越远 $z$ 越大]), anchor: "west")
  content((1.9, 2.1), text(size: 8.5pt, fill: ink, weight: "bold", [可行域]))
  content((2.9, -0.05), text(size: 8.5pt, fill: gray-line, align(center, [最优解永远出现在可行域的*顶点*——把每个顶点代一遍即可])))
})
