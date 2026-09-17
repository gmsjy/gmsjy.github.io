// s3-fig-04-prob-final.typ — 马尔可夫状态转移图
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 两个状态
  node((2.6, 2.8), [状态 A#linebreak()（答对）], w: 2.6, h: 1.3, fill: soft-green, stroke: green-ok, text-size: 8.5pt)
  node((9.4, 2.8), [状态 B#linebreak()（答错）], w: 2.6, h: 1.3, fill: soft-red, stroke: warn-red, text-size: 8.5pt)
  // 转移箭头
  arrow((3.95, 3.15), (8.05, 3.15), paint: gray-line, thickness: 1.3pt, label: [$1/3$（答对转错）], label-off: (0, 0.3), label-size: 8pt)
  arrow((8.05, 2.35), (3.95, 2.35), paint: gray-line, thickness: 1.3pt, label: [$1/2$（错后答对率）], label-off: (0, 0.3), label-size: 8pt)
  // 自环
  circle((2.6, 4.6), radius: 0.28, stroke: 1pt + green-ok)
  content((2.6, 5.25), text(size: 8pt, fill: green-ok, weight: "bold", [留 A：2/3]))
  circle((9.4, 4.6), radius: 0.28, stroke: 1pt + warn-red)
  content((9.4, 5.25), text(size: 8pt, fill: warn-red, weight: "bold", [留 B：1/2]))
  content((6.0, 0.5), text(size: 8.5pt, fill: gray-line, align(center, [下一状态只依赖当前状态——这就是「马尔可夫性」。#linebreak()高考概率压轴的新宠：循环赛、翻转硬币、传球游戏全是它])))
})
