// s3-fig-14c-frames.typ — 动画分帧：抛体运动的四个瞬间
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle
  // 轨迹
  curve(x => x - 0.32 * x * x / 2, 0, 4.8, stroke: (paint: gray-line, thickness: 0.9pt, dash: "dashed"))
  // 四帧位置（等时）
  for i in range(4) {
    let t = 1.1 + i * 1.05
    let x = t
    let y = x - 0.32 * x * x / 2
    circle((x, y), radius: 0.16, fill: if calc.odd(i) { phys-orange } else { math-blue }, stroke: none)
    content((x + 0.05, y + 0.42), text(size: 7.5pt, fill: gray-line, ["t" + str(i + 1)]))
    // 速度矢量（水平不变、竖直变化）
    vector((x, y), (x + 0.62, y), paint: math-blue, thickness: 0.9pt, scale: 0.5)
    vector((x, y), (x, y - (1.55 - 0.45 * i) * 0.45), paint: phys-orange, thickness: 0.9pt, scale: 0.5)
  }
  content((2.4, 3.9), text(size: 8.5pt, fill: gray-line, align(center, [等时间隔的分帧：水平速度不变、竖直速度每帧等量变化——#linebreak()「分帧」就是仿真的可视化输出，也是动画课件的生产方式])))
})
