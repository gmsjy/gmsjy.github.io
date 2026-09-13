// s3-fig-11c-pascal.typ — 帕斯卡三角：组合数的家谱树
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  let n = 5
  for r in range(n + 1) {
    for k in range(r + 1) {
      // 递推计算组合数
      let c = 1
      for j in range(k) { c = c * (r - j) / (j + 1) }
      let x = 7.0 - r * 0.7 + k * 1.4
      let y = 5.1 - r * 0.9
      circle((x, y), radius: 0.42, fill: if calc.rem(c, 2) == 1 { soft-orange } else { white }, stroke: 0.9pt + phys-orange)
      content((x, y), text(size: 7.5pt, fill: ink, weight: "bold", str(int(c))))
    }
  }
  content((7.0, 6.2), text(size: 9.5pt, fill: ink, weight: "bold", [每个数 = 上面两数之和（帕斯卡法则）]))
  content((7.0, -0.6), text(size: 8.5pt, fill: gray-line, align(center, [第 n 行恰是 $(a + b)^n$ 展开的系数（组合数 $binom(n, k)$）——数论、组合、#linebreak()概率三条线在三角里汇合，强基数论与组合的入门地图])))
})
