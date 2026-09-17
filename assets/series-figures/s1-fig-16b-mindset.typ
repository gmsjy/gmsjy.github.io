// s1-fig-16b-mindset.typ — 心态曲线：遇到难题的正常波动与回到基线
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 10.6, -0.3, 4.6, x-step: 2, y-step: 1, x-label: [考试进程], y-label: [心态], tick-labels: false)
  line((0, 3.0), (10.3, 3.0), stroke: (paint: green-ok, thickness: 1pt, dash: "dashed"))
  content((10.35, 3.0), text(size: 8.5pt, fill: green-ok, [冷静基线]), anchor: "west")
  let pts = ((0.2, 3.0), (1.2, 3.3), (2.6, 3.15), (3.8, 1.2), (5.0, 0.7), (6.2, 1.5), (7.4, 2.6), (8.8, 3.1), (10.0, 3.2))
  line(..pts, stroke: 1.6pt + math-blue)
  circle((5.0, 0.7), radius: 0.1, fill: warn-red, stroke: white)
  content((5.0, -0.35), text(size: 8pt, fill: warn-red, weight: "bold", [卡在一道压轴，心一沉]))
  arrow((5.2, 0.95), (6.05, 1.4), paint: warn-red, thickness: 0.7pt, scale: 0.5)
  content((8.4, 4.0), text(size: 8.5pt, fill: ink, align(center, [差生和好生的区别不是不慌，#linebreak()而是回到基线要几分钟])))
})
