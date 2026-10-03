// s1-fig-03c-levels.typ — 五维对照卡：典型表现 → 专属处方
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((1.0, 0), [维度], w: 1.9, h: 0.7, fill: gray-line, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((5.0, 0), [掉队的典型信号], w: 6.0, h: 0.7, fill: warn-red, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((10.6, 0), [对症处方], w: 5.4, h: 0.7, fill: green-ok, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  let rows = (
    (y: -1.15, n: [运算], a: [化简含参式子超过 3 步就出错], b: [每天 15 分钟限时口算与化简]),
    (y: -2.3, n: [理解], a: [概念辨析题靠感觉、背结论], b: [用自己的话复述定义并举反例]),
    (y: -3.45, n: [建模], a: [应用题读完了列不出式子], b: [每题先写「设谁为 x、找等量关系」]),
    (y: -4.6, n: [推理], a: [证明题只能写第一句], b: [抄 3 遍范例后默写推理链]),
    (y: -5.75, n: [实验], a: [实验题背步骤、不懂为什么], b: [每做一题先答「控制了什么变量」]),
  )
  for r in rows {
    node((1.0, r.y), r.n, w: 1.9, h: 1.0, fill: soft-gray, stroke: gray-line, text-size: 9pt, weight: "bold")
    node((5.0, r.y), r.a, w: 6.0, h: 1.0, fill: soft-red, stroke: warn-red, text-size: 7.8pt)
    node((10.6, r.y), r.b, w: 5.4, h: 1.0, fill: soft-green, stroke: green-ok, text-size: 7.8pt)
  }
})
