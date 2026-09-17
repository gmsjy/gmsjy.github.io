// s3-fig-16-advanced-cetz.typ — 三维曲面的斜二测线框
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let pr(p) = (p.at(0) + 0.45 * p.at(2), p.at(1) + 0.3 * p.at(2))

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  // 画 z = 0.5 * sin(x) + 0.3*y 的线框（近似抛物面谷）
  let lines = ()
  // 沿 x 方向的等 y 截线
  for j in range(5) {
    let y = j * 0.8
    let pts = range(21).map(i => {
      let x = i * 0.25
      let z = 2.4 * calc.exp(-((x - 2.2) * (x - 2.2) + (y - 1.6) * (y - 1.6)) / 1.6)
      pr((x, y, z))
    })
    line(..pts, stroke: 0.9pt + math-blue)
  }
  // 沿 y 方向的等 x 截线
  for i in range(6) { let i = i * 4;
    let x = i * 0.25
    let pts = range(11).map(j => {
      let y = j * 0.32
      let z = 2.4 * calc.exp(-((x - 2.2) * (x - 2.2) + (y - 1.6) * (y - 1.6)) / 1.6)
      pr((x, y, z))
    })
    line(..pts, stroke: (paint: phys-orange, thickness: 0.8pt))
  }
  content((3.4, 4.2), text(size: 9pt, fill: ink, weight: "bold", align(center, [三维曲面线框图：两组截面线交织出「山」])))
  content((3.4, -0.75), text(size: 8.5pt, fill: gray-line, align(center, [高度 $z = f(x, y)$ 被两族截面线撑起——本图完全由 CeTZ 循环代码生成，#linebreak()改一个参数（如峰高）即可重绘])))
})
