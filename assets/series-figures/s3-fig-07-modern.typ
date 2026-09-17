// s3-fig-07-modern.typ — 分子动理论：速度分布与温度
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let bell(x, mu, sg, h) = {
  let t = (x - mu) / sg
  h * calc.exp(-t * t / 2)
}

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 6.9, -0.3, 4.4, x-step: 2, y-step: 2, x-label: [分子速率], y-label: [分子占比], tick-labels: false)
  curve(x => bell(x, 2.6, 1.3, 3.8), 0.1, 6.5, stroke: 1.4pt + math-blue)
  curve(x => bell(x, 3.4, 1.9, 2.6), 0.1, 6.6, stroke: (paint: phys-orange, thickness: 1.3pt, dash: "dashed"))
  content((2.6, 4.15), text(size: 8.5pt, fill: math-blue, weight: "bold", [低温]))
  content((5.6, 2.85), text(size: 8.5pt, fill: phys-orange, weight: "bold", [高温]))
  arrow((2.85, 3.5), (4.3, 2.75), paint: warn-red, thickness: 1.2pt, label: [升温：峰右移变矮], label-off: (0.6, 0.15), label-size: 7.8pt)
  content((3.4, -0.05), text(size: 8.5pt, fill: gray-line, align(center, [温度是分子平均动能的量度：升温不改变「多数」，#linebreak()但整体分布向高速区挪动（麦克斯韦速率分布思想）])))
})
