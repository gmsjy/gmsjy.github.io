// s3-fig-12-strong-phys.typ — 转动惯量的微元定义
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 细杆切成微元
  line((0.8, 2.2), (9.2, 2.2), stroke: 1.4pt + ink)
  content((0.8, 2.6), text(size: 8.5pt, fill: ink, weight: "bold", [转轴 O]))
  for i in range(8) {
    let x = 1.5 + i * 0.95
    rect((x, 2.05), (x + 0.5, 2.35), fill: if calc.odd(i) { soft-blue } else { soft-orange }, stroke: 0.7pt + gray-line)
    content((x + 0.25, 1.7), text(size: 7pt, fill: gray-line, [dm]))
  }
  content((5.0, 3.2), text(size: 9.5pt, fill: ink, weight: "bold", [把每个微元的质量乘以距离平方，再全部相加]))
  content((5.0, 1.05), text(size: 10pt, fill: warn-red, weight: "bold", [I = integral r^2 dif m]))
  content((5.0, 0.2), text(size: 8.5pt, fill: gray-line, align(center, [细杆绕端点：$I = M L^2 / 3$；绕中点：$I = M L^2 / 12$——离轴越远贡献越大，#linebreak()这就是花样滑冰收臂转速猛增的全部秘密])))
})
