// s3-fig-12c-emwave.typ — 电磁波：E 与 B 互相垂直、同步传播
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 传播方向
  arrow((0.4, 1.9), (4.6, 1.9), paint: ink, thickness: 1.3pt, label: [传播方向], label-off: (0.2, 0.28), label-size: 8.5pt)
  // E 沿 y 振动
  curve(x => 1.3 * calc.sin((x - 0.4) * 2.1), 0.4, 4.6, n: 60, stroke: 1.4pt + math-blue)
  content((4.75, 1.5), text(size: 8.5pt, fill: math-blue, weight: "bold", [$E$]), anchor: "west")
  // B 沿 z「竖直屏幕方向」用短竖线表示
  for x in (0.75, 1.5, 2.25, 3.0, 3.75, 4.5) {
    let h = 1.3 * calc.sin((x - 0.4) * 2.1)
    line((x, 1.9), (x, 1.9 + h * 0.45), stroke: 1pt + phys-orange)
    circle((x, 1.9 + h * 0.45), radius: 0.035, fill: phys-orange, stroke: none)
  }
  content((2.5, 3.3), text(size: 8.5pt, fill: phys-orange, weight: "bold", [$B$（垂直于 E）]))
  content((2.5, 0.35), text(size: 8.5pt, fill: gray-line, align(center, [变化的电场生磁场、变化的磁场生电场——电磁波自己喂养自己，#linebreak()光速传播，无需介质。$c = 1 / sqrt(mu_0 epsilon_0)$])))
})
