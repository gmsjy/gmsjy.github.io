// s1-fig-16-exam.typ — 考试时间分配条（以 120 分钟理科卷为例）
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let seg = (([选择题 45 分钟], 45, math-blue), ([实验题 15 分钟], 15, green-ok), ([大题 50 分钟], 50, phys-orange), ([机动回查 10 分钟], 10, gray-line))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  let x = 0.6
  for s in seg {
    let w = s.at(1) * 12.6 / 120
    rect((x, 2.6), (x + w, 3.8), fill: s.at(2).lighten(70%), stroke: 1pt + s.at(2))
    content((x + w / 2, 3.2), text(size: 8pt, fill: ink, weight: "bold", align(center, s.at(0))))
    content((x + w / 2, 2.2), text(size: 8.5pt, fill: s.at(2), weight: "bold", str(s.at(1)) + "′"))
    x += w
  }
  content((6.9, 4.5), text(size: 9.5pt, fill: ink, weight: "bold", [时间跟着分值走：选择题平均每题 ≤ 3 分钟]))
  content((6.9, 1.2), text(size: 8.5pt, fill: gray-line, align(center, [机动时间不是「剩余时间」而是「预算好的回查额度」——#linebreak()被大题吃掉，说明取舍失败，与难度无关])))
})
