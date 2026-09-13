// s2-fig-09b-belt.typ — 传送带模型：三种初始情形
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, rect, circle
  // 传送带
  rect((1.0, 2.0), (9.0, 3.2), fill: soft-gray, stroke: ink)
  circle((1.4, 2.6), radius: 0.55, fill: white, stroke: 1.1pt + ink)
  circle((8.6, 2.6), radius: 0.55, fill: white, stroke: 1.1pt + ink)
  vector((4.6, 3.45), (5.9, 3.45), paint: math-blue, thickness: 1.3pt, label: [带速 $v_"带"$（向右）], label-off: (-0.7, 0.25), label-size: 8pt)
  // 物块
  rect((4.1, 3.2), (5.1, 3.85), fill: soft-orange, stroke: ink)
  content((4.6, 3.52), text(size: 8pt, fill: phys-orange, weight: "bold", [$m$]))
  content((4.6, 4.3), text(size: 8.5pt, fill: ink, align(center, [物块轻放到带上])))
  // 三种情形
  content((2.9, 1.35), text(size: 8.5pt, fill: warn-red, weight: "bold", [情形 1：$v_0 < v_"带"$]))
  content((2.9, 0.75), text(size: 7.8pt, fill: gray-line, align(center, [摩擦向右加速，#linebreak()先加速后共速])))
  content((6.9, 1.35), text(size: 8.5pt, fill: math-blue, weight: "bold", [情形 2：$v_0 > v_"带"$]))
  content((6.9, 0.75), text(size: 7.8pt, fill: gray-line, align(center, [摩擦向左减速，#linebreak()先减速后共速])))
  content((10.9, 1.35), text(size: 8.5pt, fill: green-ok, weight: "bold", [情形 3：$v_0 = v_"带"$]))
  content((10.9, 0.75), text(size: 7.8pt, fill: gray-line, align(center, [无相对滑动，#linebreak()全程匀速])))
})
