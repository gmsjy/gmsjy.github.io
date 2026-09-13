// s1-fig-04-number-system.typ — 数系同心圈：N ⊂ Z ⊂ Q ⊂ R
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  let cs = ((4.4, 3.0, 3.9, soft-blue, [R 实数], [π, √2, √3]),
            (4.4, 2.55, 3.0, soft-orange, [Q 有理数], [(2)/(3), −0.75, 0.6　]),
            (4.4, 2.0, 2.05, soft-green, [Z 整数], [−3, −1, 0, 7]),
            (4.4, 1.55, 1.1, soft-gray, [N 自然数], [0, 1, 2, 3…]))
  for c in cs {
    circle((c.at(0), c.at(1)), radius: c.at(2), fill: c.at(3), stroke: 1pt + gray-line)
  }
  // 标签放各圈左上内沿
  content((2.6, 5.3), text(size: 9.5pt, fill: math-blue, weight: "bold", [R 实数]))
  content((2.6, 4.15), text(size: 9pt, fill: phys-orange, weight: "bold", [Q 有理数]))
  content((3.1, 3.15), text(size: 8.5pt, fill: green-ok, weight: "bold", [Z 整数]))
  content((4.15, 2.3), text(size: 8.5pt, fill: gray-line, weight: "bold", [N 自然数]))
  // 代表元素
  content((6.1, 5.0), text(size: 8.5pt, fill: gray-line, [π ≈ 3.14159…]))
  content((6.1, 4.55), text(size: 8.5pt, fill: gray-line, [√2 ≈ 1.41421…]))
  content((6.05, 3.2), text(size: 8.5pt, fill: gray-line, [(2)/(3), −0.75]))
  content((4.4, 1.85), text(size: 8.5pt, fill: gray-line, [0, 1, 2, 3, …]))
  // 右侧说明
  content((9.2, 4.6), text(size: 8.5pt, fill: ink, [初中：见到的大多在 Q 里]))
  content((9.2, 4.0), text(size: 8.5pt, fill: ink, [高中：√、π 与三角比是常客]))
  arrow((8.0, 4.3), (7.05, 4.3), paint: gray-line, thickness: 0.9pt)
  content((9.2, 3.2), text(size: 8.5pt, fill: warn-red, [扩充不是替换：旧运算律全部保留]))
  arrow((9.05, 3.35), (8.2, 3.7), paint: warn-red, thickness: 0.7pt, scale: 0.5)
})
