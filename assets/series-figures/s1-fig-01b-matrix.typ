// s1-fig-01b-matrix.typ — 初中 vs 高中 四维对比矩阵
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

// 列中心：维度 1.1 / 初中 4.4 / 高中 8.7 / 坎 12.4
#let c1 = 1.1
#let c2 = 4.4
#let c3 = 8.7
#let c4 = 12.4

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  // 表头
  node((c1, 0), [维度], w: 2.2, h: 0.7, fill: gray-line, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((c2, 0), [初中], w: 4.2, h: 0.7, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((c3, 0), [高中], w: 4.2, h: 0.7, fill: phys-orange, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((c4, 0), [你会遇到的坎], w: 3.4, h: 0.7, fill: warn-red, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)

  let rows = (
    (y: -1.25, name: [知识量],
      a: [三年约 30 个核心公式#linebreak()题型单一、重复训练],
      b: [三年 100+ 公式与模型#linebreak()每章引入新工具],
      c: [同一晚要消化#linebreak()2—3 个全新概念]),
    (y: -2.5, name: [抽象度],
      a: [看得见的图形与数字#linebreak()代入具体数值计算],
      b: [字母系数与含参讨论#linebreak()用图像语言描述],
      c: [「把字母当数」#linebreak()通常要 2—4 周适应]),
    (y: -3.75, name: [模型化],
      a: [现象与结论直接对应#linebreak()套公式即可得分],
      b: [先建理想模型#linebreak()再列方程求解],
      c: [不知道从哪下手#linebreak()「题目读不懂」]),
    (y: -5.0, name: [考试节奏],
      a: [90—120 分钟#linebreak()综合题少、计算直接],
      b: [120 分钟 19—22 题#linebreak()压轴题分层设卡],
      c: [会但不熟练#linebreak()时间总不够用]),
  )
  for r in rows {
    node((c1, r.y), r.name, w: 2.2, h: 1.15, fill: soft-gray, stroke: gray-line, text-size: 8.5pt, weight: "bold")
    node((c2, r.y), r.a, w: 4.2, h: 1.15, fill: soft-blue, stroke: math-blue, text-size: 7.3pt)
    node((c3, r.y), r.b, w: 4.2, h: 1.15, fill: soft-orange, stroke: phys-orange, text-size: 7.3pt)
    node((c4, r.y), r.c, w: 3.4, h: 1.15, fill: soft-red, stroke: warn-red, text-size: 7.3pt, text-fill: rgb("#9B2C2C"))
  }
})

#let fig-b = fig
