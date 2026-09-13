// s1-fig-18b-summer.typ — 衔接暑假 8 周规划表
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((1.5, 0), [周次], w: 2.6, h: 0.7, fill: gray-line, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((6.2, 0), [数学任务（每天 60 分钟）], w: 6.6, h: 0.7, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((11.5, 0), [物理任务（每天 40 分钟）], w: 5.6, h: 0.7, fill: phys-orange, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  let rows = (
    (y: -1.2, n: [1—2 周], a: [初高中衔接教材：数与式、#linebreak()方程与不等式（回炉 + 预习）], b: [测量与估算入门、#linebreak()矢量概念启蒙]),
    (y: -2.5, n: [3—4 周], a: [函数概念与五类图像初识，#linebreak()每天 5 道基础题限时], b: [质点与参考系、#linebreak()受力分析启蒙]),
    (y: -3.8, n: [5—6 周], a: [二次函数含参入门，#linebreak()学会「列表讨论」], b: [力的合成分解、#linebreak()牛二定律初识]),
    (y: -5.1, n: [7—8 周], a: [综合小卷 × 4 份，#linebreak()建立错题本与闭环习惯], b: [运动学图像初识，#linebreak()配套本系列第 9—10 篇]),
  )
  for r in rows {
    node((1.5, r.y), r.n, w: 2.6, h: 1.15, fill: soft-gray, stroke: gray-line, text-size: 9pt, weight: "bold")
    node((6.2, r.y), r.a, w: 6.6, h: 1.15, fill: soft-blue, stroke: math-blue, text-size: 7.8pt)
    node((11.5, r.y), r.b, w: 5.6, h: 1.15, fill: soft-orange, stroke: phys-orange, text-size: 7.8pt)
  }
  content((6.9, -6.3), text(size: 8.5pt, fill: gray-line, [总量约 100 小时：不到高考复习量的十分之一，却决定高一上学期的起点]))
})
