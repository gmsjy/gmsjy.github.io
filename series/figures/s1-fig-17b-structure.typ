// s1-fig-17b-structure.typ — 一个最小 Typst 工程的结构
#import "../styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  node((2.6, 4.6), [my-notes/　工程根目录], w: 4.4, h: 0.85, fill: math-blue, stroke: none, text-fill: white, weight: "bold", text-size: 9pt)
  let kids = (
    (y: 3.2, t: [template.typ　模板：页面与样式], d: [全工程只写一次], fill: soft-blue, st: math-blue),
    (y: 1.9, t: [figures/　绘图源码目录], d: [一图一文件，可复用], fill: soft-orange, st: phys-orange),
    (y: 0.6, t: [notes/　笔记正文], d: [用 import 引用模板与图], fill: soft-green, st: green-ok),
  )
  for k in kids {
    line((2.6, 4.15), (2.6, k.y + 0.3), stroke: 0.9pt + gray-line)
    node((3.1, k.y), k.t, w: 5.6, h: 0.85, fill: k.fill, stroke: k.st, text-size: 8.5pt, weight: "bold")
    content((6.3, k.y), text(size: 8pt, fill: gray-line, k.d), anchor: "west")
  }
  content((10.4, 2.6), text(size: 8.5pt, fill: ink, align(center, [内容与样式分离：#linebreak()笔记只管写，长相模板管。#linebreak()三年后回看依然统一])))
})
