// s1-fig-17-tools.typ — 数理学习工具箱：三件套的分工
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((2.0, 0), [工具], w: 2.6, h: 0.7, fill: gray-line, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((6.0, 0), [它擅长什么], w: 5.0, h: 0.7, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((11.3, 0), [高中怎么用], w: 4.4, h: 0.7, fill: phys-orange, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  let rows = (
    (y: -1.3, n: [Typst], a: [排版数学公式、整理#linebreak()结构化笔记], b: [公式笔记、错题本、#linebreak()小论文的标准化写作]),
    (y: -2.65, n: [CeTZ / 绘图], a: [函数图像、几何图、#linebreak()受力图与流程图], b: [把自己做错的题#linebreak()重画一遍图，吃透结构]),
    (y: -4.0, n: [仿真 / 表格], a: [数值模拟、数据处理、#linebreak()验证猜想], b: [物理过程模拟、#linebreak()实验数据的二次加工]),
  )
  for r in rows {
    node((2.0, r.y), r.n, w: 2.6, h: 1.2, fill: soft-gray, stroke: gray-line, text-size: 9.5pt, weight: "bold")
    node((6.0, r.y), r.a, w: 5.0, h: 1.2, fill: soft-blue, stroke: math-blue, text-size: 8pt)
    node((11.3, r.y), r.b, w: 4.4, h: 1.2, fill: soft-orange, stroke: phys-orange, text-size: 8pt)
  }
})
