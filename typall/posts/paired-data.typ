#let title = "成对数据的统计分析：回归与独立性检验"
#let date = "2026-09-05"
#let tags = ("数学", "概率统计", "高中")
#let series = "高中数学"
#let series_weight = 22
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

前两课统计处理「一个变量」（身高、成绩），这一课处理「*两个变量*之间的关系」：广告费与销量相关吗？吸烟与患病有关吗？前者用*回归分析*量化趋势，后者用*独立性检验*裁决「有关还是无关」。

= 相关关系与散点图

函数关系是*一一确定*的（$y = k x$）；*相关关系*是「有趋势、不精确」——身高与体重、学习时长与成绩。拿到成对数据 $(x_1, y_1), dots, (x_n, y_n)$，第一件事永远是*画散点图*：

- 点自左向右上升——*正相关*；下降——*负相关*；
- 点紧贴一条直线——*线性相关*强；弥散一片——几乎不相关；
- 点贴一条曲线——*非线性相关*（后文可线性化）。

散点图是回归分析的安全带：不画图直接套公式，把弯曲趋势硬算成直线是最常见的错误。

= 一元线性回归：最小二乘法

== 经验回归方程

给散点配一条直线 $hat(y) = hat(b) x + hat(a)$，使每个点的*残差* $y_i - hat(y)_i$ 的平方和最小——这就是*最小二乘法*。公式：

$ hat(b) = (sum_(i=1)^n (x_i - overline(x))(y_i - overline(y))) / (sum_(i=1)^n (x_i - overline(x))^2) quad quad quad hat(a) = overline(y) - hat(b) overline(x) $ <eq:least>

计算器或题目数据表可直接出 $hat(b)$、$hat(a)$；手算时先算 $overline(x)$、$overline(y)$，再列 $x_i - overline(x)$、$y_i - overline(y)$ 两列乘起来求和。

> 记忆点：回归直线*必过中心点* $(overline(x), overline(y))$——检验算出的 $hat(a)$ 是否正确的秒杀判据，也是选择填空的常客。

#fig(cetz.canvas(length: 0.85cm, {
  // 散点 + 回归直线：回归直线必过中心点
  line((0.4, 0), (10.6, 0), mark: (end: ">"), stroke: black)
  line((0.4, 0), (0.4, 4.2), mark: (end: ">"), stroke: black)
  content((10.5, 0.4), $ x $, size: 9pt)
  content((0.75, 4.05), $ y $, size: 9pt)
  // 回归直线
  line((0.8, 0.7), (9.8, 3.7), stroke: (paint: rgb("#0b6fc0"), thickness: 1.3pt))
  // 散点（大致沿直线散布）
  let pts = ((1.6, 1.15), (2.7, 1.6), (3.4, 1.1), (4.6, 2.0), (5.5, 2.6), (6.3, 2.1), (7.5, 3.0), (8.6, 2.7), (9.3, 3.5))
  for p in pts {
    circle(p, radius: 0.07, fill: rgb("#d64541"), stroke: none)
  }
  // 中心点
  circle((5.2, 2.23), radius: 0.1, stroke: rgb("#3c8a4d"), fill: none)
  content((5.45, 1.8), $ (overline(x), overline(y)) $, size: 8pt)
}))

== 相关系数 $r$：相关的强弱

回归方程任何一组数据都能算出来——哪怕点散得毫无趋势。$r$ 用来先问「*配不配*回归」：

$ r = (sum_(i=1)^n (x_i - overline(x))(y_i - overline(y))) / (sqrt(sum_(i=1)^n (x_i - overline(x))^2) sqrt(sum_(i=1)^n (y_i - overline(y))^2)) $ <eq:rcoef>

- $r in [-1, 1]$；$r > 0$ 正相关，$r < 0$ 负相关；
- $|r|$ *越接近 1，线性相关程度越强*；接近 0 说明几乎无线性关系；
- $|r| >= 0.75$ 左右常视为「较强」的参考线（具体阈值看题目规定）。

$R^2$（决定系数）是 $r$ 的推广版：$R^2 = 1 - (sum (y_i - hat(y)_i)^2) / (sum (y_i - overline(y))^2)$，*越接近 1* 拟合效果越好，曲线回归也能用。

== 非线性模型：先化直，再回归

若散点贴指数曲线 $y = c e^(d x)$，取对数 $z = ln y = d x + ln c$，则 $z$ 与 $x$ *线性相关*——对 $(x, z)$ 做线性回归得 $hat(z) = hat(b) x + hat(a)$，再由 $hat(y) = e^(hat(z))$ 还原。对数型、幂型同理：*换变量把弯拉直*。

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：某商品广告费 $x$（万元）与销量 $y$（万件）5 组数据算得 $overline(x) = 4$，$overline(y) = 6$，$hat(b) = 1.2$。求回归方程并预测广告费 8 万元时的销量。

由 @eq:least 的第二式：$hat(a) = 6 - 1.2 times 4 = 1.2$，故 $hat(y) = 1.2 x + 1.2$（验证：过 $(4, 6)$ ✓）。

$x = 8$ 时 $hat(y) = 1.2 times 8 + 1.2 = 10.8$ 万件。回答须说「*预测值/估计值*」，不说「一定」。
]

= 独立性检验：$chi^2$ 裁决

== 列联表与零假设

判断两个*分类变量*（吸烟/不吸烟 × 患病/不患病）是否相关，先整理 $2 times 2$ *列联表*：

#table(
  columns: (auto, auto, auto, auto),
  inset: 6pt,
  stroke: 0.5pt + rgb("#c4cad2"),
  [], [患病], [不患病], [合计],
  [吸烟], [$a$], [$b$], [$a + b$],
  [不吸烟], [$c$], [$d$], [$c + d$],
  [合计], [$a + c$], [$b + d$], [$n$],
)

*零假设* $H_0$：两个变量*相互独立*（没关系）。检验就是问：数据与 $H_0$ 的差距有多大？

== $chi^2$ 统计量与临界值

$ chi^2 = (n (a d - b c)^2) / ((a + b)(c + d)(a + c)(b + d)) $ <eq:chi2>

$chi^2$ 越大，「独立」越难解释数据。对照临界值表：

#table(
  columns: (auto, auto, auto),
  inset: 6pt,
  stroke: 0.5pt + rgb("#c4cad2"),
  [$chi^2$], [$3.841$], [$6.635$],
  [$P(chi^2 >= x_0)$], [$0.05$], [$0.01$],
  [结论], [有 95% 把握认为*不独立*], [有 99% 把握认为*不独立*],
)

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：抽查 200 人得 $a = 40$，$b = 60$，$c = 20$，$d = 80$。判断吸烟与患病是否有关。

$chi^2 = (200 (40 times 80 - 60 times 20)^2) / (100 times 100 times 60 times 140) = (200 times 2000^2) / (8.4 times 10^7) approx 9.52$。

$9.52 > 6.635$，故有 99% 把握认为吸烟与患病*有关*。
]

> 记忆点：结论的标准句式是「*有 95%（99%）的把握认为 A 与 B 有关*」，不是「A 导致 B」——$chi^2$ 检验只说*相关*，不说*因果*；$chi^2$ 小于临界值只能说「*没有足够证据认为有关*」，不能说「证明无关」。

= 解题方法论

- *回归三步*：散点图 → 算 $hat(b)$、$hat(a)$（中心点验算）→ 用方程预测并注明「估计」；
- *先 $r$ 后回归*：$|r|$ 很小就别硬配直线，题给 $r$ 时先答「线性相关程度强/弱」；
- *非线性换元*：指数取 $ln$、幂取对数双换，化直后回归，还原时用反函数；
- *$chi^2$ 手算纪律*：分子先算 $a d - b c$，分母四个边缘和一个不落；$n$ 别丢；
- *表述是得分点*：「有 …% 把握认为有关」「预测约 …」——统计结论的措辞本身就是考点。

一句话收束：回归分析给「关系有多强」一个*方程*，独立性检验给「有没有关系」一个*判决*——统计不替你断言因果，它只负责告诉你在数据面前，多大的把握说多大的话。
