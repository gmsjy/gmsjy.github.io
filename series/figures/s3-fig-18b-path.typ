// s3-fig-18b-path.typ — 大学先修学习路径
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let steps = (
    (x: 1.9, t: [① 微积分入门#linebreak()导数 · 积分 · 级数], d: [一切工科与理科的通用语言], fill: soft-blue, st: math-blue),
    (x: 7.0, t: [② 线性代数初步#linebreak()矩阵 · 向量空间], d: [数据时代的第一语言], fill: soft-blue, st: math-blue),
    (x: 12.1, t: [③ 普通物理#linebreak()力学 · 电磁学], d: [微积分的第一次大规模实战], fill: soft-orange, st: phys-orange),
  )
  for s in steps {
    node((s.x, 2.8), s.t, w: 4.0, h: 1.35, fill: s.fill, stroke: s.st, text-size: 8.8pt)
    content((s.x, 1.75), text(size: 7.8pt, fill: gray-line, s.d))
  }
  arrow((3.95, 2.8), (4.95, 2.8), paint: gray-line, thickness: 1.3pt)
  arrow((9.05, 2.8), (10.05, 2.8), paint: gray-line, thickness: 1.3pt)
  // 学习法
  node((7.0, 0.5), [④ 贯通检验：用新语言重推旧结论（如用微积分重讲全部匀变速公式）], w: 11.0, h: 0.85, fill: soft-green, stroke: green-ok, text-size: 8.8pt)
})
