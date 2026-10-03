// s3-fig-03-sequence-final.typ — 递推蛛网：a(n+1) = f(a(n)) 的迭代可视化
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let f(x) = 0.25 * x + 2.0

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 4.4, -0.3, 4.2, x-step: 1, y-step: 1)
  curve(f, 0, 4.1, stroke: 1.3pt + math-blue)
  line((0, 0), (4.0, 4.0), stroke: (paint: gray-line, thickness: 0.8pt, dash: "dashed"))
  // 蛛网迭代：a1=0.8 → a2=f(0.8)=2.2 → ...
  let pts = ((0.8, 0), (0.8, 2.2), (2.2, 2.2), (2.2, 2.55), (2.55, 2.55), (2.55, 2.64), (2.64, 2.64))
  line(..pts, stroke: 1.3pt + warn-red)
  for i in range(7).filter(x => calc.rem(x, 2) == 0) {
    circle(pts.at(i), radius: 0.06, fill: warn-red, stroke: none)
  }
  content((1.0, 0.35), text(size: 8pt, fill: warn-red, weight: "bold", [$a_1$]))
  content((2.35, 1.9), text(size: 8pt, fill: warn-red, weight: "bold", [$a_2$]))
  content((0.7, 3.85), text(size: 8.5pt, fill: math-blue, [不动点 $x = 8/3$：递推数列#linebreak()单调逼近它——放缩的「靶子」就在这]))
})
