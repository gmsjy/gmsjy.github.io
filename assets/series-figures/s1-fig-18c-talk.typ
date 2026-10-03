// s1-fig-18c-talk.typ — 家长沟通卡：怎么说 vs 别怎么说
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: content
  node((3.4, 0), [✗ 容易起反效果的话], w: 5.6, h: 0.75, fill: warn-red, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  node((10.6, 0), [✓ 更有效的说法], w: 5.6, h: 0.75, fill: green-ok, stroke: none, text-fill: white, weight: "bold", text-size: 8.5pt)
  let rows = (
    (y: -1.35, a: [「怎么又退步了，是不是玩手机了？」], b: [「这次比上次少丢的分在哪一科？我们一起看看卷子。」]),
    (y: -2.85, a: [「别人家孩子数学 140。」], b: [「你上个月不会做的题型，这次好像会了。」]),
    (y: -4.35, a: [「高中就这样，熬过去就好。」], b: [「高一是在换学习方法，不是在拼天赋，我们按计划来。」]),
  )
  for r in rows {
    node((3.4, r.y), r.a, w: 5.6, h: 1.3, fill: soft-red, stroke: warn-red, text-size: 8pt)
    node((10.6, r.y), r.b, w: 5.6, h: 1.3, fill: soft-green, stroke: green-ok, text-size: 8pt)
  }
  content((7.0, -5.6), text(size: 8.5pt, fill: gray-line, [核心区别：左列评价人，右列讨论事；左列比输赢，右列找方法]))
})
