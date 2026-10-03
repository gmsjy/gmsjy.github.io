// s3-fig-17-project.typ — 研究项目六阶段流程
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let rows = (
    (y: 4.6, items: (([① 选题], soft-blue, math-blue), ([② 文献], soft-blue, math-blue), ([③ 方法设计], soft-blue, math-blue))),
    (y: 2.4, items: (([④ 数据采集], soft-orange, phys-orange), ([⑤ 分析建模], soft-orange, phys-orange), ([⑥ 报告答辩], soft-green, green-ok))),
  )
  for r in rows {
    for (k, it) in r.items.enumerate() {
      node((2.3 + k * 4.6, r.y), it.at(0), w: 3.4, h: 1.15, fill: it.at(1), stroke: it.at(2), text-size: 9.5pt, weight: "bold")
      if k < 2 {
        arrow((4.0 + k * 4.6, r.y), (4.55 + k * 4.6, r.y), paint: gray-line, thickness: 1.2pt)
      }
    }
  }
  arrow((13.0, 4.0), (13.0, 3.0), paint: gray-line, thickness: 1.2pt)
  // 阶段产物
  content((2.3, 3.6), text(size: 7.5pt, fill: gray-line, align(center, [小而真：可测量、可建模])))
  content((6.9, 3.6), text(size: 7.5pt, fill: gray-line, align(center, [至少 3 篇参考文献])))
  content((11.5, 3.6), text(size: 7.5pt, fill: gray-line, align(center, [变量、控制、样本量])))
  content((2.3, 1.4), text(size: 7.5pt, fill: gray-line, align(center, [原始数据 + 记录])))
  content((6.9, 1.4), text(size: 7.5pt, fill: gray-line, align(center, [图表 + 检验])))
  content((11.5, 1.4), text(size: 7.5pt, fill: gray-line, align(center, [五段式 + 5 分钟讲述])))
})
