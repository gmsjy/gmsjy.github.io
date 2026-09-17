// s2-fig-18b-plan.typ — 强基启蒙六周计划：两条线并进
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 微积分线（蓝，上）
  timeline((
    (1.4, [W1—2#linebreak()导数定义与求导], math-blue),
    (4.6, [W3—4#linebreak()用 $f'$ 求极值], math-blue),
    (7.8, [W5#linebreak()微元求和→积分], math-blue),
    (11.0, [W6#linebreak()基本定理合龙], math-blue),
  ), x0: 0.5, x1: 12.9, alt: 1.05, label-size: 7.8pt)
  content((12.9, 3.05), text(size: 9pt, fill: math-blue, weight: "bold", [知识线：微积分雏形]), anchor: "east")
  // 项目线（橙，下）
  timeline((
    (3.0, [W2—3#linebreak()选题三问筛选], phys-orange),
    (7.0, [W4—7#linebreak()执行与数据], phys-orange),
    (10.5, [W8—9#linebreak()报告与答辩], phys-orange),
    (13.4, [持续#linebreak()下一个种子], phys-orange),
  ), x0: 0.5, x1: 13.6, alt: -1.05, label-size: 7.8pt)
  content((13.6, -2.9), text(size: 9pt, fill: phys-orange, weight: "bold", [能力线：项目式学习]), anchor: "east")
  content((7.0, 3.0), text(size: 9pt, fill: gray-line, align(center, [六周两条线并进：每周 2—3 小时，80% 主线 + 20% 拓展])))
})
