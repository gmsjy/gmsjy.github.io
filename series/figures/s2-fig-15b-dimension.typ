// s2-fig-15b-dimension.typ — 量纲树：单位也能「列方程」
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((7.0, 4.6), [周期 $T = 2 pi sqrt(l / g)$ 的量纲检验], w: 7.4, h: 0.85, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 9.5pt)
  node((3.0, 2.8), [$l$：长度 L], w: 2.6, h: 0.8, fill: soft-blue, stroke: math-blue, text-size: 8.5pt)
  node((7.0, 2.8), [$g$：加速度 $L / T^2$], w: 3.0, h: 0.8, fill: soft-orange, stroke: phys-orange, text-size: 8.5pt)
  node((11.0, 2.8), [$sqrt(l / g) = sqrt(L / (L / T^2)) = T$ ✓], w: 4.6, h: 0.8, fill: soft-green, stroke: green-ok, text-size: 8.5pt)
  arrow((7.0, 4.15), (3.4, 3.25), paint: gray-line, thickness: 1pt)
  arrow((7.0, 4.15), (6.6, 3.25), paint: gray-line, thickness: 1pt)
  arrow((7.0, 4.15), (10.6, 3.25), paint: gray-line, thickness: 1pt)
  content((7.0, 1.2), text(size: 8.5pt, fill: gray-line, align(center, [等式两边量纲必须一致——量纲分析可以快速#linebreak()排查公式错误，甚至「猜」出公式形状（如单摆周期只可能 prop sqrt(l / g)）])))
})
