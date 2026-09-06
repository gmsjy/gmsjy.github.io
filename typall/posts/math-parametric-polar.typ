#let title = "参数方程与极坐标：给曲线换一套描述语言"
#let date = "2026-09-05"
#let tags = ("数学", "解析几何", "参数方程", "极坐标", "高中")
#let series = "高中数学"
#let series_weight = 17
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 参数方程：让坐标随时间流动

前面学过的曲线方程都是"直接关系"：$y = f(x)$，一个 $x$ 对应一个 $y$。但有些曲线用这种写法很别扭——比如*圆*，一个 $x$ 对应两个 $y$。更自然的描述是：让一个参数 $t$（比如时间）牵着坐标走：

$ cases(x = r cos t, y = r sin t) quad (t "为参数") $ <eq:circle-param>

这就是圆心在原点、半径为 $r$ 的圆的*参数方程*。物理里的抛体运动 $cases(x = v_0 t, y = h - 1/2 g t^2)$ 其实就是抛物线的参数方程——参数方程是"运动的数学"，$t$ 流动，点就画出曲线。

#fig(cetz.canvas(length: 1.0cm, {
  // 单位圆上的动点：角度 t 与坐标 (cos t, sin t) 的对应
  circle((3, 2), radius: 1.5, stroke: (paint: black, thickness: 1.2pt))
  line((3, 2), (4.35, 2.8), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  circle((4.35, 2.8), radius: 0.06, fill: rgb("#d64541"), stroke: none)
  content((4.6, 2.85), $ P(cos t, sin t) $, anchor: "west", size: 9pt)
  line((3, 2), (4.9, 2), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  line((4.35, 2.8), (4.9, 2), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  content((4.6, 1.75), "1", size: 8pt)
  arc((3, 2), radius: 0.55, start: 0deg, stop: 30.5deg, mark: (end: ">"), stroke: rgb("#0b6fc0"))
  content((3.85, 2.18), $ t $, size: 9pt)
  content((3, 2.06), "O", anchor: "north-east", size: 8pt)
  content((4.9, 3.4), "t 流动 → 点画出圆", size: 8pt)
}))

*消参*可以把参数方程变回普通方程：对圆的参数方程，两式平方相加消去 $t$，得 $x^2 + y^2 = r^2$。反过来，普通方程也常常"制造"参数——比如椭圆 $x^2/a^2 + y^2/b^2 = 1$ 的参数方程 $cases(x = a cos theta, y = b sin theta)$，$theta$ 正是离心角。

= 直线的参数方程：$t$ 就是路程

过点 $P_0(x_0, y_0)$、倾斜角为 $alpha$ 的直线可写成

$ cases(x = x_0 + t cos alpha, y = y_0 + t sin alpha) quad (t "为参数") $ <eq:line-param>

这组参数方程有个珍贵的几何意义：*参数 $t$ 的绝对值就是 $P_0$ 到动点的距离*（$t > 0$ 在直线上方，$t < 0$ 在下方）。解"直线与曲线交点到某点的距离"问题时，把参数方程代入曲线方程解出 $t_1, t_2$，距离立刻现形——不用再算两点间距离公式。

= 极坐标：用方向和距离定位

直角坐标用"东西南北走多少"定位，*极坐标*用"朝哪个方向、走多远"定位：取原点 $O$ 为*极点*、水平射线 $O x$ 为*极轴*，平面上一点用有序对 $(rho, theta)$ 描述——$rho$ 是到极点的距离（*极径*），$theta$ 是射线 $O x$ 转到 $O P$ 的角度（*极角*）。

#fig(cetz.canvas(length: 1.0cm, {
  line((-0.3, 2), (5.6, 2), mark: (end: ">"), stroke: black)
  content((5.8, 2), $ x $, size: 9pt)
  content((-0.15, 1.75), "O", anchor: "north-east", size: 8pt)
  content((-0.35, 2.15), "极点", size: 8pt)
  line((0, 2), (2.9, 3.8), stroke: (paint: rgb("#d64541"), thickness: 1.2pt))
  circle((2.9, 3.8), radius: 0.06, fill: rgb("#d64541"), stroke: none)
  content((3.15, 3.9), $ P(rho, theta) $, anchor: "west", size: 9pt)
  arc((0, 2), radius: 1.0, start: 0deg, stop: 33deg, mark: (end: ">"), stroke: rgb("#0b6fc0"))
  content((1.3, 2.45), $ theta $, size: 9pt)
  line((2.9, 3.8), (2.9, 2), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  line((0, 2), (2.9, 2), mark: (start: "|", end: "|"), stroke: (paint: rgb("#3c8a4d"), thickness: 1.2pt))
  content((1.45, 1.7), $ rho cos theta $, size: 9pt)
  content((3.1, 2.85), $ rho sin theta $, size: 8pt, anchor: "west")
}))

两套坐标由一组公式互相翻译：

$ x = rho cos theta, quad y = rho sin theta, quad rho^2 = x^2 + y^2, quad tan theta = y / x $ <eq:polar-convert>

极坐标最擅长描述"绕着极点转"的曲线——距离只依赖方向的图形，在极坐标下方程往往短得惊人：以极点为圆心、半径 $a$ 的圆就是 $rho = a$；过极点的射线就是 $theta = theta_0$。

= 一条迷人的曲线：心形线

在极坐标下写下 $rho = 1 + cos theta$，让 $theta$ 从 $0$ 转到 $2 pi$，会得到一条自带"爱心"的曲线——*心形线*（ cardioid ）：

#fig(cetz.canvas(length: 0.85cm, {
  let pts = range(0, 181, step: 1).map(k => {
    let t = k * 2 * calc.pi / 180
    let r = 1 + calc.cos(t)
    (r * calc.cos(t) + 3.2, r * calc.sin(t) + 2)
  })
  line(..pts, stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  circle((3.2, 2), radius: 0.05, fill: black, stroke: none)
  content((3.3, 1.85), "O", anchor: "north-west", size: 8pt)
  content((5.1, 3.7), $ rho = 1 + cos theta $, size: 10pt)
  content((0.9, 2), "θ = π", anchor: "west", size: 8pt)
  content((5.0, 2.1), "θ = 0", anchor: "east", size: 8pt)
}))

参数方程与极坐标不改变曲线本身，改变的是*描述语言*。很多"解析几何里的硬骨头"——椭圆上动点与焦点的连线、摆线、螺线——换个坐标系后简洁得像一首诗。选择合适的表达方式，本身就是数学的核心品味。

= 小结

- 参数方程 $cases(x = f(t), y = g(t))$：参数 $t$ 牵着点走，消参可还原普通方程；直线参数方程中 $|t|$ 即距离；
- 极坐标 $(rho, theta)$ 用方向与距离定位，与直角坐标经 $x = rho cos theta$、$y = rho sin theta$ 互化；
- 圆 $rho = a$、射线 $theta = theta_0$、心形线 $rho = 1 + cos theta$——"绕极点"的曲线在极坐标下天生简洁；
- 描述方式的选择服务于问题本身：圆用参数、旋转用极坐标、直线用直角坐标，各得其所。
