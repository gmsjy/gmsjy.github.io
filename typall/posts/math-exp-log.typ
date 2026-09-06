#let title = "指数、对数与幂函数：增长的三种面孔"
#let date = "2026-09-02"
#let tags = ("数学", "函数", "高中")
#let series = "高中数学"
#let series_weight = 4
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

指数爆炸、对数平缓、幂函数居中——三种基本初等函数刻画了自然界与社会科学中最常见的增长规律（细胞分裂、地震震级、GDP 增速）。这一篇打通指数与对数这对"互为逆运算"的孪生兄弟。

= 指数与指数幂的运算

== 分数指数幂

$n$ 次方根：$x^n = a$，则 $x$ 叫 $a$ 的 $n$ 次方根。当 $n$ 为奇数时 $root(n, a)$ 对一切实数有意义；当 $n$ 为偶数时要求 $a >= 0$，且 $root(n, a) >= 0$（*算术根*）。

$ a^(m/n) = root(n, a^m), quad a^(-m/n) = 1/(a^(m/n)) quad (a > 0, m, n in NN^"*", n > 1) $ <eq:fraction-exp>

运算性质（$a > 0, b > 0$）：

$ a^r a^s = a^(r+s), quad (a^r)^s = a^(r s), quad (a b)^r = a^r b^r $

> 易错点：$root(n, a^n) = cases(|a| "，" n "为偶数", a "，" n "为奇数")$——偶次根号下开出来要带绝对值！如 $sqrt(x^2) = |x|$，不是 $x$。化简根式先判断 $n$ 的奇偶。

= 指数函数

形如 $y = a^x$（$a > 0$ 且 $a != 1$）的函数叫*指数函数*。图像恒过 $(0, 1)$，定义域 $RR$，值域 $(0, +oo)$：

#table(
  columns: (auto, 1fr, 1fr),
  inset: 6pt,
  [*性质*], [$a > 1$（增长型）], [$0 < a < 1$（衰减型）],
  [图像], [从左下到右上，上升], [从左上到右下，下降],
  [单调性], [在 $RR$ 上单调递增], [在 $RR$ 上单调递减],
  [比较大小], [$x > 0$ 时 $a^x > 1$], [$x > 0$ 时 $a^x < 1$],
  [渐近行为], [$x arrow.r -oo$ 时 $y arrow.r 0^+$], [$x arrow.r +oo$ 时 $y arrow.r 0^+$],
)

> 图像记忆：指数函数*全部经过 $(0,1)$ 且恒在 $x$ 轴上方*。底数越大，$a > 1$ 时图像越"陡"（在 $y$ 轴右侧越高）；$0 < a < 1$ 时底数越小越"陡"。比较 $a^x$ 大小可借助 $y = 1$ 这条"分界线"：底数大于 1 时指数为正则大于 1。

= 对数与对数运算

== 对数的定义

$a^x = N$（$a > 0, a != 1$），则 $x = log_a N$，读作以 $a$ 为底 $N$ 的对数。$a$ 叫底数，$N$ 叫真数（*真数必须大于 0*）。

两个重要特例：常用对数 $lg N = log_(10) N$，自然对数 $ln N = log_e N$（$e approx 2.71828$）。

== 对数运算性质（换底公式是核心）

$ log_a (M N) = log_a M + log_a N, quad log_a (M)/(N) = log_a M - log_a N, quad log_a M^n = n log_a M $ <eq:log-ops>

$ log_a b = (log_c b)/(log_c a) quad ("换底公式"), quad log_a b dot log_b a = 1, quad a^(log_a N) = N $ <eq:log-change>

> 换底公式的灵活运用：$log_a b = 1/(log_b a)$（倒数关系，底真互换）；对数运算若底数不同，先换底再算。牢记恒等式 $a^(log_a N) = N$——这是"对数化指数"的桥。

= 对数函数

形如 $y = log_a x$（$a > 0, a != 1$）叫*对数函数*。它是 $y = a^x$ 的反函数，图像恒过 $(1, 0)$，定义域 $(0, +oo)$，值域 $RR$：

#table(
  columns: (auto, 1fr, 1fr),
  inset: 6pt,
  [*性质*], [$a > 1$], [$0 < a < 1$],
  [图像], [右上上升，过 $(1, 0)$], [右下下降，过 $(1, 0)$],
  [单调性], [在 $(0, +oo)$ 递增], [在 $(0, +oo)$ 递减],
  [比较大小], [$x > 1$ 时 $log_a x > 0$], [$x > 1$ 时 $log_a x < 0$],
  [渐近行为], [$x arrow.r 0^+$ 时 $y arrow.r -oo$], [$x arrow.r 0^+$ 时 $y arrow.r +oo$],
)

> 图像记忆：对数函数*全部经过 $(1, 0)$ 且在 $y$ 轴右侧*（$y$ 轴是渐近线）。与指数函数关于直线 $y = x$ 对称。比较 $log_a b$ 的大小，先看底数 $a$ 与 1 的关系定单调性，再看真数 $b$ 与 1 的关系定正负。

#fig(cetz.canvas(length: 0.9cm, {
  // 指数与对数互为反函数：图像关于 y = x 对称
  line((-2.6, 0), (3.5, 0), mark: (end: ">"), stroke: black)
  line((0, -2.3), (0, 3.5), mark: (end: ">"), stroke: black)
  content((3.4, 0.4), $ x $, size: 9pt)
  content((0.35, 3.4), $ y $, size: 9pt)
  line((-1.8, -1.8), (3.2, 3.2), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  content((2.95, 3.3), $ y = x $, size: 8pt)
  let pts = range(0, 33, step: 1).map(k => {
    let t = -2 + k * 3.6 / 32
    (t, calc.pow(2, t))
  })
  line(..pts, stroke: (paint: rgb("#d64541"), thickness: 1.4pt))
  let pts2 = pts.map(p => (p.at(1), p.at(0)))
  line(..pts2, stroke: (paint: rgb("#0b6fc0"), thickness: 1.4pt))
  circle((1, 1), radius: 0.09, stroke: rgb("#3c8a4d"), fill: none)
  content((1.2, 1.35), $ (1, 1) $, size: 8pt)
  content((2.3, 2.75), $ y = 2^x $, size: 8pt)
  content((2.9, 0.7), $ y = log_2 x $, size: 8pt)
  content((2.2, -2.15), "两支曲线关于对角线 y = x 成镜像", size: 8pt)
}))

= 幂函数与增长模型对比

== 幂函数

形如 $y = x^alpha$（$alpha in QQ$）叫*幂函数*。高中重点掌握 $alpha = -1, 1/2, 1, 2, 3$ 五种，共同性质：都过 $(1, 1)$；$alpha > 0$ 时在第一象限递增且过 $(0, 0)$；$alpha < 0$ 时在第一象限递减且不过原点。

== 三种增长的对比（选择压轴高频）

设 $x$ 充分大，增长速度：*指数函数 > 幂函数 > 对数函数*（底数均大于 1 时）：

$ a^x >> x^k >> log_a x quad (a > 1, k > 0, x arrow.r +oo) $

> 实际含义：指数增长"爆炸"（细菌繁殖），幂增长"稳定加速"（面积随边长），对数增长"越来越慢"（人耳对声音强度的感知）。题目给三种函数图像让你匹配，就看"谁最后蹿得最高"。

= 函数的零点与二分法

== 零点定理（连接函数与方程的桥）

$f(x)$ 的零点就是方程 $f(x) = 0$ 的根，即图像与 $x$ 轴交点的横坐标。*零点定理*：若 $f(x)$ 在 $[a, b]$ 上连续，且 $f(a) f(b) < 0$，则 $f(x)$ 在 $(a, b)$ 内至少有一个零点。

> 注意：① $f(a) f(b) < 0$ 是*充分不必要*条件——图像穿过 $x$ 轴但没穿（相切）时也有零点；② 零点存在性常用于"判断方程根的个数/区间"，结合单调性可确定*唯一*零点。

== 二分法求近似零点

每次取区间中点 $c = (a+b)/2$，计算 $f(c)$，用 $f(a) f(c) < 0$ 还是 $f(c) f(b) < 0$ 判断零点在哪半段，不断缩小范围，直到满足精度。

> 指数对数综合题套路：比较 $2^0.3$、$log_2 0.3$、$0.3^2$ 这类大小——① 先看正负（对数真数 < 1 且底 > 1 时为负，直接最小）；② 剩下的与 0、1 比，找"中间桥"（常用 $1/2$、$1$）；③ 同底用单调性，同指数用幂函数单调性，否则找中间量。三大函数图像性质熟练后，比较大小就是送分题。
