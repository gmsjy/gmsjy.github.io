// s2-fig-12-momentum.typ — 冲量的图像语言：F–t 面积 = 动量变化
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  axes(-0.4, 5.6, -0.4, 3.6, x-step: 1, y-step: 1, x-label: $t/"s"$, y-label: $F/"N"$)
  // 恒力冲量：矩形
  line((0, 2), (3, 2), stroke: 1.4pt + math-blue)
  rect((0, 0), (3, 2), fill: soft-blue.lighten(40%), stroke: none)
  line((3, 0), (3, 2), stroke: (paint: gray-line, thickness: 0.6pt, dash: "dashed"))
  content((1.5, 1.0), text(size: 9pt, fill: math-blue, weight: "bold", [面积 $= I$]))
  content((0.4, 2.3), text(size: 8.5pt, fill: math-blue, [恒力：$I = F t$]))
  // 变力冲量：曲线下面积
  let pts = range(41).map(i => { let t = 3.4 + 1.9 * i / 40; (t, 2.8 - 0.55 * (t - 3.4)) })
  line(..pts, close: true, fill: soft-orange, stroke: 1.3pt + phys-orange)
  content((5.0, 1.0), text(size: 8.5pt, fill: phys-orange, align(center, [变力（如球拍击球）#linebreak()面积同样有效])))
  content((2.8, -0.05), text(size: 8.5pt, fill: gray-line, align(center, [动量定理：$F_"合" t = m v - m v_0$——缓冲问题里「延长接触时间」的本质])))
})
