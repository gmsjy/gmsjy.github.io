// 冒烟测试：九大组件 + 模板盒子（不作为正式文章）
#import "@preview/cetz:0.5.2"
#import "template.typ": *

#show: article.with([组件冒烟测试], season: "工程校验", module: "styles.typ", minutes: 1, words: "—")

#section("校验一", "CeTZ 九大组件")

#figure-block("0-1", [axes / vector / node / arrow / circuit / field-line / conic / error-bar / timeline], cetz.canvas(length: 0.85cm, {
  import cetz.draw: *
  // 1 坐标轴 + 曲线 + 矢量
  axes(-1, 4, -1, 3, show-grid: true)
  curve(x => 0.5 * x * x, -1, 3, stroke: 1.1pt + math-blue)
  vector((3, 2.2), (4, 2.2), paint: phys-orange, label: $v$, label-off: (0.05, 0.2))
  // 2 节点 + 箭头
  node((1.2, 2.6), [节点], w: 1.5, h: 0.65)
  arrow((2.05, 2.6), (2.9, 2.6), paint: gray-line, label: [连线])
  // 3 电路一排
  circuit-battery((-0.8, -0.8), d: (1, 0), len: 1.0)
  circuit-resistor((0.9, -0.8), len: 1.4, label: $R$)
  circuit-switch((2.6, -0.8), len: 1.0)
  circuit-lamp((3.7, -0.8))
  circuit-meter((4.4, -0.8))
  wire((-0.3, -0.8), (0.2, -0.8))
  wire((1.6, -0.8), (2.1, -0.8))
  wire((3.1, -0.8), (3.46, -0.8))
  wire((3.94, -0.8), (4.14, -0.8))
  // 4 电场线 / 电荷 / B 符号
  charge((-0.7, 1.9), sign: "+")
  field-line((-0.45, 1.9), (0.6, 1.9))
  b-into((1.0, 1.9))
  b-out((1.45, 1.9))
  // 5 圆锥曲线
  conic(kind: "ellipse", center: (3.0, 1.6), a: 0.7, b: 0.45, stroke: 0.9pt + math-blue)
  // 6 误差棒 + 拟合线
  error-bar((4.2, 1.4), 0.18)
  circle((4.2, 1.4), radius: 0.05, fill: math-blue, stroke: none)
  fit-line(0.2, 0.9, 3.8, 4.6, paint: warn-red)
}))

#figure-block("0-2", [timeline 时间轴], cetz.canvas(length: 1cm, {
  timeline((
    (1.2, [初一], math-blue),
    (4.0, [初三], math-blue),
    (6.8, [断层带], warn-red),
    (9.6, [高一], phys-orange),
  ), x0: 0.4, x1: 11.2)
}))

#section("校验二", "模板盒子与表格")

#gap-table((
  ([一次函数、正比例], [五类基本初等函数], [「函数是对应关系」]),
  ([合力、分力的直觉], [矢量运算、正交分解], [「箭头也能做加法」]),
))

#problem-box([例题（演示）], [已知 $f(x) = x^2 - 2x$，求 $f(x)$ 在 $[-1, 3]$ 上的最小值。配 $lim_(x -> 0) (sin x)/x = 1$ 行内式与
$ f'(x) = lim_(Delta x -> 0) (f(x + Delta x) - f(x)) / Delta x $
展示公式排版。])

#method-box([方法卡片（演示）], [数形结合：看到 $f'(x) > 0$，脑中先出现上升的曲线，再看不等式本身。])

#action-box((
  [① 画出自家知识树的第一层（数学 / 物理各 4 个分支）。],
  [② 找出上次月考错题，标注属于哪一层能力断层。],
  [③ 把本篇图 1-2 打印贴在书桌前。],
))
