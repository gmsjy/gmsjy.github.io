// s2-fig-09-dynamics.typ — 板块模型：叠放与摩擦的博弈
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  // 地面
  line((0.4, 1.0), (8.6, 1.0), stroke: 1.1pt + ink)
  let hx = 0.7
  while hx <= 8.3 {
    line((hx, 1.0), (hx - 0.18, 0.82), stroke: 0.6pt + gray-line)
    hx += 0.42
  }
  // 木板 m2 + 滑块 m1
  rect((1.6, 1.0), (6.6, 1.5), fill: soft-orange, stroke: ink)
  content((4.1, 1.25), text(size: 8.5pt, fill: phys-orange, weight: "bold", [木板 $m_2$]))
  rect((2.4, 1.5), (4.0, 2.15), fill: soft-blue, stroke: ink)
  content((3.2, 1.82), text(size: 8.5pt, fill: math-blue, weight: "bold", [滑块 $m_1$]))
  // 拉力与摩擦
  vector((4.0, 1.82), (5.3, 1.82), paint: warn-red, thickness: 1.3pt, label: $F$, label-off: (0.1, 0.2))
  vector((2.4, 1.82), (1.7, 1.82), paint: gray-line, thickness: 1pt, label: $f_1$, label-off: (-0.15, 0.2), label-size: 8pt)
  vector((1.9, 1.25), (1.0, 1.25), paint: gray-line, thickness: 1pt, label: $f_2$, label-off: (-0.15, 0.2), label-size: 8pt)
  content((4.5, 2.9), text(size: 8.5pt, fill: ink, align(center, [拉滑块，木板被「拖」动：#linebreak()上下两对摩擦力等大反向（牛三）])))
  // 临界问题注记
  content((11.6, 2.4), text(size: 8.5pt, fill: ink, align(center, [临界问题三连问：#linebreak()① 一起动还是分开动？#linebreak()② 相对滑动何时结束？#linebreak()③ 最大静摩擦是多少？])))
})
