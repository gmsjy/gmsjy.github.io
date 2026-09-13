// s3-fig-05b-vt.typ — 多过程 v-t 图：分段与临界
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  axes(-0.3, 7.4, -3.9, 3.9, x-step: 1, y-step: 1, x-label: $t/"s"$, y-label: $v/("m/s")$)
  // 段1：曲面下滑（变加速→近似） 加速
  line((0, -3), (2, 0), stroke: 1.5pt + math-blue)
  // 段2：水平匀速
  line((2, 0), (2, 2), stroke: 1.5pt + gray-line) // 落地缓冲示意（速度方向转为水平，用小竖线示意突变的速度切换）
  line((2, 2), (4, 2), stroke: 1.5pt + math-blue)
  // 段3：遇弹簧减速至 0
  line((4, 2), (5.5, 0), stroke: 1.5pt + math-blue)
  circle((5.5, 0), radius: 0.08, fill: warn-red, stroke: none)
  content((5.5, -0.5), text(size: 8pt, fill: warn-red, weight: "bold", [最远点 $v = 0$]))
  // 分段标注
  content((1.0, 3.1), text(size: 7.8pt, fill: ink, align(center, [曲面下滑#linebreak()加速度渐减])))
  content((3.0, 2.45), text(size: 7.8pt, fill: ink, [粗糙面匀速/减速]))
  content((4.75, 1.4), text(size: 7.8pt, fill: ink, align(center, [压缩弹簧#linebreak()加速度增大])))
  line((2, 0), (2, 2), stroke: (paint: gray-line, thickness: 0.5pt, dash: "dashed"))
  content((3.7, -1.6), text(size: 8.5pt, fill: gray-line, align(center, [多过程题先画 v–t 草图：每一段一个模型，#linebreak()拐点 = 临界（落地、共速、压缩最短）])))
})
