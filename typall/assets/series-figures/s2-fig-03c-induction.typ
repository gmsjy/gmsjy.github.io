// s2-fig-03c-induction.typ — 数学归纳法：多米诺骨牌
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  // 骨牌：前 4 张倒下，第 5 张立着，后面立着
  for i in range(7) {
    let x = 1.4 + i * 1.8
    let h = 2.0
    if i < 4 {
      // 倒下的
      rect((x, 1.0), (x + 0.35, 1.0 + h), fill: soft-gray, stroke: gray-line)
      rect((x, 1.0), (x + h * 0.6, 1.0 + 0.3), fill: soft-blue, stroke: math-blue)
    } else {
      rect((x, 1.0), (x + 0.35, 1.0 + h), fill: soft-orange, stroke: phys-orange)
    }
  }
  arrow((4.6, 3.4), (6.0, 3.4), paint: warn-red, thickness: 1.4pt)
  content((5.3, 3.8), text(size: 8pt, fill: warn-red, weight: "bold", [推倒第一张]))
  content((7.6, 4.1), text(size: 8.5pt, fill: ink, weight: "bold", align(center, [第 1 张倒（奠基）#linebreak()第 $k$ 张倒 $==>$ 第 $k + 1$ 张倒（传递）])))
  content((6.9, -0.1), text(size: 8.5pt, fill: gray-line, [两条都成立，全体骨牌必倒——这就是数学归纳法的全部逻辑]))
})
