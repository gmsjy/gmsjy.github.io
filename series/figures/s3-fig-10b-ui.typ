// s3-fig-10b-ui.typ — U-I 图线：截距与斜率的物理含义
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 4.4, -2.9, 3.4, x-step: 1, y-step: 1, x-label: $I/"A"$, y-label: $U/"V"$)
  // U = E - rI：截距 E，斜率 -r
  line((0, 3), (3.9, 0.95), stroke: 1.5pt + math-blue)
  content((2.4, 2.6), text(size: 8.5pt, fill: math-blue, weight: "bold", [数据点拟合直线]))
  line((0, 3), (0.4, 3), stroke: (paint: warn-red, thickness: 1pt))
  content((0.45, 3.05), text(size: 8.5pt, fill: warn-red, weight: "bold", [纵截距 $= E$]), anchor: "west")
  line((3.9, 0.95), (3.9, 0), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  line((3.9, 0.95), (0, 0.95), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  content((4.05, 0.6), text(size: 8.5pt, fill: phys-orange, weight: "bold", [斜率绝对值 $= r$]), anchor: "west")
  content((2.1, 0.5), text(size: 8.5pt, fill: gray-line, align(center, [延长线与横轴交点为短路电流#linebreak()（电压表内阻带来的系统误差：$E$ 偏小、$r$ 偏小）])))
})
