// s1-fig-08-probability.typ — 概率树：两步试验的完整分解
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  circle((1.0, 2.6), radius: 0.1, fill: ink, stroke: none)
  content((1.0, 3.15), text(size: 8.5pt, fill: ink, weight: "bold", [开始]))
  // 第一层：摸球（3 红 2 白）
  let l1 = ((4.2, 4.0, [红 $3/5$], math-blue), (4.2, 1.2, [白 $2/5$], phys-orange))
  for n in l1 {
    arrow((1.15, 2.6), (n.at(0) - 0.15, n.at(1)), paint: gray-line, thickness: 1pt)
    circle((n.at(0), n.at(1)), radius: 0.09, fill: n.at(3), stroke: none)
    content((n.at(0), n.at(1) + 0.42), text(size: 8.5pt, fill: n.at(3), weight: "bold", n.at(2)))
  }
  // 第二层
  let l2 = ((8.0, 4.7, [红 $2/4$], [$(3)/(5) dot (2)/(4) = (3)/(10)$], math-blue),
            (8.0, 3.3, [白 $2/4$], [$(3)/(5) dot (2)/(4) = (3)/(10)$], phys-orange),
            (8.0, 1.9, [红 $3/4$], [$(2)/(5) dot (3)/(4) = (3)/(10)$], math-blue),
            (8.0, 0.5, [白 $1/4$], [$(2)/(5) dot (1)/(4) = (1)/(10)$], phys-orange))
  for n in l2 {
    arrow((4.35, if n.at(1) > 2.6 { 4.0 } else { 1.2 }), (n.at(0) - 0.15, n.at(1)), paint: gray-line, thickness: 1pt)
    circle((n.at(0), n.at(1)), radius: 0.09, fill: n.at(4), stroke: none)
    content((n.at(0), n.at(1) + 0.42), text(size: 8.5pt, fill: n.at(4), weight: "bold", n.at(2)))
    content((n.at(0) + 0.35, n.at(1) - 0.42), text(size: 8pt, fill: gray-line, n.at(3)), anchor: "west")
  }
  content((4.2, -0.35), text(size: 8.5pt, fill: ink, align(center, [两次都是白：沿「白 → 白」的路径把#linebreak()分支概率相乘；两类结局相加即总概率])))
})
