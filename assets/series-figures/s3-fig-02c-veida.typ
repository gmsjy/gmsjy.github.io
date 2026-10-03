// s3-fig-02c-veida.typ — 联立四步舞：设而不求的标准流程
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  let steps = (
    (x: 1.9, t: [① 设线#linebreak()$x = m y + a_0$], d: [避免讨论斜率], fill: soft-blue, st: math-blue),
    (x: 5.7, t: [② 联立消元#linebreak()查判别式 $Delta > 0$], d: [合法性检查], fill: soft-blue, st: math-blue),
    (x: 9.5, t: [③ 韦达定理#linebreak()$y_1 + y_2$，$y_1 y_2$], d: [设而不求的核心], fill: soft-orange, st: phys-orange),
    (x: 13.3, t: [④ 目标翻译#linebreak()弦长 / 面积 / 定值], d: [代入消参], fill: soft-green, st: green-ok),
  )
  for s in steps {
    node((s.x, 2.6), s.t, w: 3.2, h: 1.35, fill: s.fill, stroke: s.st, text-size: 8.3pt)
    content((s.x, 1.55), text(size: 7.5pt, fill: gray-line, s.d))
  }
  arrow((3.55, 2.6), (4.05, 2.6), paint: gray-line, thickness: 1.2pt)
  arrow((7.35, 2.6), (7.85, 2.6), paint: gray-line, thickness: 1.2pt)
  arrow((11.15, 2.6), (11.65, 2.6), paint: gray-line, thickness: 1.2pt)
  content((7.6, 4.35), text(size: 9.5pt, fill: ink, weight: "bold", [圆锥曲线大题 90% 的前半程，都是这条流水线]))
  content((7.6, 0.4), text(size: 8.5pt, fill: gray-line, align(center, [定点定值的判定：第④步代入后参数完全消失 → 过定点 / 为定值；#linebreak()消参后剩一个线性关系 → 定点坐标立现])))
})
