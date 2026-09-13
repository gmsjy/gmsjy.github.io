// s1-fig-02-knowledge-tree.typ — 双知识树：数学主线 ↔ 物理主线 + 衔接点高亮
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content, circle

  // ---- 数学树（左） ----
  node((2.3, 0.9), [高中数学], w: 2.6, h: 0.85, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 9.5pt)
  let m-children = (
    (x: 0.9, t: [代数#linebreak()方程不等式]),
    (x: 2.65, t: [函数#linebreak()五类初等]),
    (x: 4.4, t: [几何#linebreak()立体解析]),
    (x: 6.15, t: [概率#linebreak()统计初步]),
  )
  for c in m-children {
    arrow((2.3, 1.35), (c.x, 2.55), paint: math-blue, thickness: 0.8pt)
    node((c.x, 3.1), c.t, w: 1.6, h: 1.05, fill: soft-blue, stroke: math-blue, text-size: 7pt)
  }

  // ---- 物理树（右，阶梯上升） ----
  node((12.0, 0.6), [高中物理], w: 2.6, h: 0.85, fill: phys-orange, stroke: none, text-fill: white, weight: "bold", text-size: 9.5pt)
  let p-children = (
    (x: 9.9, y: 4.2, t: [运动学#linebreak()x–v–a 图像]),
    (x: 11.6, y: 3.3, t: [力学#linebreak()受力 · 牛顿定律]),
    (x: 13.3, y: 2.4, t: [电磁#linebreak()电路 · 场与感应]),
    (x: 15.0, y: 1.5, t: [能量动量#linebreak()守恒思想]),
  )
  for c in p-children {
    arrow((12.6, 1.0), (c.x, c.y - 0.55), paint: phys-orange, thickness: 0.8pt)
    node((c.x, c.y), c.t, w: 1.6, h: 1.05, fill: soft-orange, stroke: phys-orange, text-size: 7pt)
  }

  // ---- 衔接点（高亮虚线桥） ----
  let bridges = (
    (a: (3.47, 3.35), b: (9.1, 4.2), lab: (6.2, 4.35), t: [图像语言]),
    (a: (5.22, 3.15), b: (10.8, 3.3), lab: (7.95, 3.72), t: [矢量与三角]),
    (a: (1.55, 2.55), b: (14.2, 1.5), lab: (7.6, 1.72), t: [方程与守恒]),
  )
  for br in bridges {
    line(br.a, br.b, stroke: (paint: phys-orange, thickness: 1pt, dash: "dashed"))
    circle(br.a, radius: 0.09, fill: warn-red, stroke: white)
    circle(br.b, radius: 0.09, fill: warn-red, stroke: white)
    content(br.lab, text(size: 8pt, fill: warn-red, weight: "bold", br.t))
  }
  content((14.4, 0.62), text(size: 7.5pt, fill: warn-red, [红色虚线 = 衔接点]))

  // 主线标题
  content((2.8, 6.2), text(size: 9.5pt, fill: math-blue, weight: "bold", [数学主线：工具箱]))
  content((12.4, 6.2), text(size: 9.5pt, fill: phys-orange, weight: "bold", [物理主线：问题对象]))
})
