// s2-fig-07-stat.typ — 两条正态曲线：均值与方差的直观
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let bell(x, mu, sg, h) = {
  let t = (x - mu) / sg
  h * calc.exp(-t * t / 2)
}

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  axes(-4.9, 6.4, -0.4, 4.9, x-step: 2, y-step: 2, tick-labels: false)
  curve(x => bell(x, 0, 1, 4.2), -3.6, 3.6, stroke: 1.4pt + math-blue)
  curve(x => bell(x, 2.2, 1.6, 2.6), -3.2, 6.2, stroke: 1.4pt + phys-orange)
  content((-0.1, 4.45), text(size: 8.5pt, fill: math-blue, weight: "bold", [$mu_1, sigma_1$]))
  content((3.6, 2.85), text(size: 8.5pt, fill: phys-orange, weight: "bold", [$mu_2, sigma_2$]))
  content((0.4, 1.4), text(size: 8pt, fill: math-blue, [矮胖 = 数据散]))
  content((-3.6, 3.1), text(size: 8pt, fill: gray-line, [瘦高 = 数据集中]))
})
