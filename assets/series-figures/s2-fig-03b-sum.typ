// s2-fig-03b-sum.typ — 求和的几何证明：1+3+5+…+(2n−1) = n²
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect
  let n = 5
  // L 形分层：第 k 层是 2k-1 个小方块（右移错开）
  for k in range(1, n + 1) {
    for j in range(2 * k - 1) {
      let bx = 0.5
      let by = 0.5
      // 从左下角往右上铺：第 k 层填第 k 行和第 k 列
      let pos = if j < k { (bx + j * 0.55, by + (k - 1) * 0.55) } else { (bx + (k - 1) * 0.55, by + (2 * k - 2 - j) * 0.55) }
      rect((pos.at(0), pos.at(1)), (pos.at(0) + 0.5, pos.at(1) + 0.5), fill: if calc.odd(k) { soft-blue } else { soft-orange }, stroke: 0.8pt + (if calc.odd(k) { math-blue } else { phys-orange }))
    }
  }
  content((4.4, 3.7), text(size: 9.5pt, fill: ink, weight: "bold", [5 层 L 形 = 一个 $5 times 5$ 正方形]))
  content((4.4, -0.35), text(size: 8.5pt, fill: gray-line, [每层蓝橙相间：$1 + 3 + 5 + 7 + 9 = 25$。数列求和的很多公式，都能这样「拼个图形看出来」]))
})
