// s1-fig-16c-skip.typ — 取舍决策树：一道题的「留或走」
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((6.9, 5.2), [这道题，2 分钟没思路怎么办？], w: 6.2, h: 0.9, fill: phys-orange, stroke: none, text-fill: white, weight: "bold", text-size: 9.5pt)
  node((2.6, 3.5), [分值大 + 后面#linebreak()还有思路], w: 3.6, h: 1.2, fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
  node((6.9, 3.5), [完全没头绪], w: 3.0, h: 1.2, fill: soft-red, stroke: warn-red, text-size: 8.5pt)
  node((11.2, 3.5), [分值小], w: 2.6, h: 1.2, fill: soft-gray, stroke: gray-line, text-size: 8.5pt)
  node((2.6, 1.3), [做个记号先跳过，#linebreak()写完其他题回头打], w: 4.0, h: 1.3, fill: soft-green, stroke: green-ok, text-size: 8.5pt)
  node((6.9, 1.3), [果断放弃，#linebreak()把 2 分钟还给全卷], w: 3.6, h: 1.3, fill: soft-green, stroke: green-ok, text-size: 8.5pt)
  node((11.2, 1.3), [凭第一直觉填，#linebreak()绝不恋战], w: 3.2, h: 1.3, fill: soft-green, stroke: green-ok, text-size: 8.5pt)
  arrow((4.9, 4.75), (3.0, 4.15), paint: gray-line, thickness: 1pt)
  arrow((6.9, 4.75), (6.9, 4.15), paint: gray-line, thickness: 1pt, label: [三问：分值？思路？时间？], label-off: (2.9, 0.1), label-size: 7.5pt)
  arrow((8.9, 4.75), (10.8, 4.15), paint: gray-line, thickness: 1pt)
  content((6.9, -0.15), text(size: 8.5pt, fill: gray-line, [取舍不是认输，是分数最大的优化器：全卷 120 分，卡死一题 12 分，输的是另外 108 分]))
})
