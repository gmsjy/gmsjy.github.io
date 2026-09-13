// s1-fig-13-experiment.typ — 打点纸带：把运动「拍」在纸条上
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 纸带
  rect((0.4, 2.2), (12.8, 3.4), fill: rgb("#FFFBEF"), stroke: 0.9pt + gray-line)
  // 打点计时器示意
  rect((0.2, 3.4), (1.6, 4.3), fill: soft-gray, stroke: ink)
  content((0.9, 3.85), text(size: 7.5pt, fill: ink, [计时器]))
  vector((1.6, 3.85), (2.9, 3.85), paint: math-blue, thickness: 1.2pt, label: [运动方向], label-off: (0, 0.3), label-size: 7.8pt)
  // 点距递增
  let xs = (2.4, 3.05, 3.9, 4.95, 6.2, 7.65, 9.3, 11.15)
  for x in xs {
    circle((x, 2.8), radius: 0.075, fill: warn-red, stroke: none)
  }
  // 点距标注
  line((2.4, 2.45), (3.05, 2.45), stroke: 0.6pt + gray-line)
  content((2.72, 2.05), text(size: 7.5pt, fill: gray-line, [$s_1$]))
  line((8.5, 2.45), (11.15, 2.45), stroke: 0.6pt + gray-line)
  content((9.8, 2.05), text(size: 7.5pt, fill: gray-line, [$s_n$]))
  content((6.6, 1.25), text(size: 8.5pt, fill: ink, align(center, [计时器每 0.02 s 打一个点：点距越大数据越远——纸带就是一条「时间标尺」])))
  content((6.6, 0.4), text(size: 8.5pt, fill: gray-line, align(center, [点距均匀 → 匀速｜点距均匀增大 → 匀加速｜点距忽大忽小 → 变加速])))
})
