// s3-fig-16c-frames2.typ — 动画分帧：单摆的相位与能量
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  let cx = 5.2
  let cy = 4.4
  let L = 2.6
  line((cx, cy), (cx, cy - 0.25), stroke: 1.1pt + ink)
  // 五帧摆角
  for (i, ang) in ((0, -38deg), (1, -18deg), (2, 0deg), (3, 18deg), (4, 38deg)) {
    let bx = cx + L * calc.sin(ang)
    let by = cy - L * calc.cos(ang)
    let col = if calc.abs(ang) > 30deg { math-blue } else if ang == 0deg { phys-orange } else { gray-line }
    line((cx, cy - 0.25), (bx, by), stroke: 0.8pt + col)
    circle((bx, by), radius: 0.18, fill: col.lighten(30%), stroke: col)
    content((bx, by - 0.45), text(size: 7pt, fill: col, ["t" + str(i + 1)]))
  }
  content((cx, cy - 0.35), text(size: 8pt, fill: gray-line, [悬点]))
  content((cx, 0.45), text(size: 8.5pt, fill: gray-line, align(center, [摆从左到右的五帧：蓝色=速度慢（势能大），橙色=最快（动能大）。#linebreak()分帧图 + 能量着色 = 一眼看懂能量交换的物理动画])))
})
