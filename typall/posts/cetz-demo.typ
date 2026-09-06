#let title = "CeTZ 插图演示：用 Typst 画图"
#let date = "2026-09-02"
#let tags = ("Typst", "CeTZ", "绘图")
#let draft = true

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 用 CeTZ 在文章里画图

Typall 现在支持 Typst 的第三方包（`@preview`）。这篇文章演示用
_CeTZ_（受 TikZ 启发的绘图库）在正文里直接编写矢量插图，编译为内联 SVG。

== 一个简单的坐标系

先画一条从原点出发的射线和一个标记点：

#fig(cetz.canvas(length: 4cm, {
  line((0, 0), (3, 2), name: "ray")
  circle((0, 0), radius: 0.06, fill: black)
  content((3, 2), $ P $, anchor: "west")
  content((0, -0.4), $ O $)
}))

图里的 $P$ 坐标是 $(3, 2)$，这是最基础的 ``line`` 与 ``content`` 组合。

== 三角函数单位圆

CeTZ 最经典的示例——单位圆上的正弦与余弦：

#fig(cetz.canvas({
  set-style(fill: rgb(0, 0, 0, 15%))
  circle((0, 0), radius: 3, name: "unit-circle")

  line((-3.5, 0), (3.5, 0), mark: (start: ">", end: ">"))
  line((0, -3.5), (0, 3.5), mark: (start: ">", end: ">"))
  content((3.5, -0.3), $ x $)
  content((-0.3, 3.5), $ y $)

  let theta = 55deg
  let p = (3 * calc.cos(theta), 3 * calc.sin(theta))
  line((0, 0), p, name: "radius")
  line((p.at(0), 0), p, name: "opposite")
  line((0, 0), (p.at(0), 0), name: "adjacent")

  content((0.4, 0.2), $ theta $)
  content((p.at(0) / 2, -0.35), $ cos(theta) $)
  content((p.at(0) + 0.3, p.at(1) / 2), $ sin(theta) $)
  content(p, $ P $, anchor: "south-east")
}))

== 流程图

用 ``cetz.tree`` 画一棵简单的决策树：

#fig(cetz.canvas({
  import cetz.draw: *
  cetz.tree.tree(
    ([开始], ([步骤 A]), ([步骤 B])),
    spread: 2.5,
    grow: 1.4,
    draw-node: (node, ..) => {
      circle((), radius: 0.55, fill: white, stroke: gray)
      content((), node.content)
    },
    draw-edge: (parent, child, ..) => {
      line(parent.group-name, child.group-name, mark: (end: ">"))
    },
  )
}))

= 小结

- 在文章里写 `#import "@preview/cetz:0.5.2"` 并导入 `assets/preview.typ` 的
  `fig` 即可使用；
- 插图要用 `#fig(cetz.canvas({ ... }))` 包一层：站点构建时 CeTZ 的矢量内容
  以 SVG 形式内联到页面，VS Code 预览与 PDF 导出则原生渲染，一份源码两边通用；
- CeTZ 自带的 WebAssembly 内核（贝塞尔、布尔运算等）也能正常加载，无需额外配置。
