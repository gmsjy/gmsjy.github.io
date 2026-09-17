// s2-fig-17-gaokao.typ — 压轴题拆解：四问切分与分值预算
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let steps = (
    (x: 1.9, t: [第 1 问：基础模型#linebreak()人人可得 · 4 分], d: [套路化，10 分钟内], fill: soft-green, st: green-ok),
    (x: 5.7, t: [第 2 问：综合应用#linebreak()中档生目标 · 6 分], d: [列出全部方程再解], fill: soft-blue, st: math-blue),
    (x: 9.5, t: [第 3 问：临界讨论#linebreak()尖子生战场 · 6 分], d: [分类 + 检验], fill: soft-orange, st: phys-orange),
    (x: 13.2, t: [最后一环：创新设问#linebreak()可放弃 · 2 分], d: [分数性价比最低], fill: soft-red, st: warn-red),
  )
  for s in steps {
    node((s.x, 2.6), s.t, w: 3.1, h: 1.5, fill: s.fill, stroke: s.st, text-size: 7.8pt)
    content((s.x, 1.5), text(size: 7.5pt, fill: gray-line, s.d))
  }
  arrow((3.5, 2.6), (4.1, 2.6), paint: gray-line, thickness: 1.1pt)
  arrow((7.3, 2.6), (7.9, 2.6), paint: gray-line, thickness: 1.1pt)
  arrow((11.1, 2.6), (11.6, 2.6), paint: gray-line, thickness: 1.1pt)
  content((7.5, 4.3), text(size: 9.5pt, fill: ink, weight: "bold", [压轴题是一道「分层设问」的题：按环抢分，不追求全对]))
  content((7.5, 0.35), text(size: 8.5pt, fill: gray-line, align(center, [取舍策略：时间预算只保前两环的完整书写；#linebreak()第三环写出方程与讨论框架也能拿步骤分])))
})
