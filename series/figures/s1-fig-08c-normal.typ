// s1-fig-08c-normal.typ — 正态曲线与 3σ 原则
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let normal(x, mu: 0, sg: 1) = {
  let t = (x - mu) / sg
  4.2 * calc.exp(-t * t / 2)
}

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-3.9, 3.9, -0.4, 4.9, x-step: 1, y-step: 1, x-label: $x$, y-label: none, tick-labels: false)
  // μ±1σ 着色（垫底）
  let band = range(25).map(i => { let t = -1 + 2 * i / 24; (t, normal(t)) })
  line(..band, close: true, fill: soft-orange, stroke: none)
  curve(x => normal(x), -3.6, 3.6, stroke: 1.4pt + math-blue)
  line((0, 0), (0, normal(0)), stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  for x in (-1, 1) { line((x, 0), (x, normal(x)), stroke: (paint: phys-orange, thickness: 0.7pt, dash: "dashed")) }
  content((0, 4.55), text(size: 8.5pt, fill: gray-line, [$mu$]))
  content((-1, -0.42), text(size: 8pt, fill: phys-orange, [$mu - sigma$]))
  content((1, -0.42), text(size: 8pt, fill: phys-orange, [$mu + sigma$]))
  content((0, 1.5), text(size: 9pt, fill: phys-orange, weight: "bold", [68%]))
  for x in (-2, 2) { line((x, 0), (x, normal(x)), stroke: (paint: math-blue, thickness: 0.7pt, dash: "dashed")) }
  content((2.05, 2.5), text(size: 8.5pt, fill: math-blue, weight: "bold", [±2σ 内 95%]))
  content((0, -1.35), text(size: 8.5pt, fill: gray-line, align(center, [±1σ 内约 68%，±2σ 内约 95%——高中的正态题，#linebreak()本质是「把概率翻译成面积」])))
})
