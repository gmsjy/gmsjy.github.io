#let title = "数列：等差与等比，规律排成的一串数"
#let date = "2026-09-04"
#let tags = ("数学", "数列", "高中")
#let series = "高中数学"
#let series_weight = 8
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 数列的概念

*数列*是按一定顺序排列的一列数 $a_1, a_2, a_3, dots$。第 $n$ 项 $a_n$ 用 $n$ 表示就是*通项公式*；前 $n$ 项的和记作 $S_n = a_1 + a_2 + dots + a_n$。

通项与求和的关系是"递推的钥匙"：

$ a_n = cases(S_1, n = 1, S_n - S_(n-1), n >= 2) $ <eq:ans>

> 易错点：用 $a_n = S_n - S_(n-1)$ 求通项时，必须验证 $n = 1$ 是否也满足（$a_1 = S_1$）。很多题"从第二项起才成立"，此时通项要写成分段形式。

= 等差数列

从第二项起，每一项与它的前一项的差等于同一个常数 $d$（公差）。核心公式：

$ a_n = a_1 + (n-1)d $ <eq:ap-an>

$ S_n = (n(a_1 + a_n))/(2) = n a_1 + (n(n-1))/(2) d $ <eq:ap-sn>

*等差中项*：$a$、$b$、$c$ 成等差数列 $<=>$ $2 b = a + c$。

> 记忆：求和公式两个式子——"首尾相加乘项数除以二"最直观（高斯求和），另一式把 $a_n$ 展开。已知 $a_1$、$d$、$n$、$a_n$、$S_n$ 五个量中任意三个，可求其余两个（"知三求二"）。

== 等差数列的性质

- 若 $m + n = p + q$，则 $a_m + a_n = a_p + a_q$（下标和相等，项和相等）；
- $S_n$、$S_(2n) - S_n$、$S_(3n) - S_(2n)$ 仍成等差数列；
- 前 $n$ 项和 $S_n = d/2 n^2 + (a_1 - d/2) n$ 是 $n$ 的二次函数（$d != 0$ 时），可用二次函数求最值。

= 等比数列

从第二项起，每一项与它前一项的比等于同一个非零常数 $q$（公比）。核心公式：

$ a_n = a_1 q^(n-1) $ <eq:gp-an>

$ S_n = cases((a_1(1 - q^n))/(1 - q), q != 1, n a_1, q = 1) $ <eq:gp-sn>

#fig(cetz.canvas(length: 0.9cm, {
  // 等差（线性增长）与等比（指数式增长）的对比，a1 = 0.5
  line((-0.2, 0), (7, 0), mark: (end: ">"), stroke: black)
  line((0, -0.2), (0, 3.4), mark: (end: ">"), stroke: black)
  content((6.85, 0.4), $ n $, size: 9pt)
  content((0.4, 3.25), $ a_n $, size: 9pt)
  let ap = range(1, 7).map(n => (n, 0.42 * (0.5 + 0.55 * (n - 1))))
  let gp = range(1, 7).map(n => (n, 0.42 * 0.5 * calc.pow(1.7, n - 1)))
  line(..ap, stroke: rgb("#0b6fc0"))
  line(..gp, stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  for p in ap { circle(p, radius: 0.06, fill: rgb("#0b6fc0"), stroke: none) }
  for p in gp { circle(p, radius: 0.06, fill: rgb("#d64541"), stroke: none) }
  content((1.35, 0.85), "等差：匀速爬坡", size: 8pt)
  content((4.6, 2.5), "等比：越跑越快", size: 8pt)
}))

*等比中项*：$a$、$b$、$c$ 成等比数列 $<=>$ $b^2 = a c$（$b = plus.minus sqrt(a c)$）。

> 两个高频坑：① 公比 $q != 0$（等比数列每一项都不能为零）；② $q = 1$ 时求和公式失效，$S_n = n a_1$，用求和公式前*先判断 $q$ 是否等于 1*。

== 等比数列的性质

- 若 $m + n = p + q$，则 $a_m a_n = a_p a_q$（下标和相等，项积相等）；
- $S_n$、$S_(2n) - S_n$、$S_(3n) - S_(2n)$（各段均非零）仍成等比数列。

= 递推关系求通项（两个套路）

#table(
  columns: (auto, 1fr, auto),
  inset: 6pt,
  [*递推式*], [*方法*], [*结果*],
  [$a_(n+1) = a_n + f(n)$], [累加法], [$a_n = a_1 + sum_(k=1)^(n-1) f(k)$],
  [$a_(n+1) = a_n dot f(n)$], [累乘法], [$a_n = a_1 product_(k=1)^(n-1) f(k)$],
  [$a_(n+1) = p a_n + q$], [待定系数法构造等比], [设 $a_(n+1) + x = p(a_n + x)$，解 $x = q/(p-1)$],
)

> 套路感："一阶线性递推"（$a_(n+1) = p a_n + q$）统一配方成 $a_(n+1) + q/(p-1) = p(a_n + q/(p-1))$，就得到一个以 $p$ 为公比的等比数列，剩下的交给等比公式。

= 数列求和技巧

- *分组求和*：等差 + 等比混合，分开各求再相加；
- *错位相减*：$a_n = b_n dot c_n$（等差 $times$ 等比），$S_n$ 两边同乘公比后错位相减；
- *裂项相消*：$1/(n(n+1)) = 1/n - 1/(n+1)$，中间项全部抵消，只剩首尾。

> 裂项经典：$1/((2n-1)(2n+1)) = 1/2 (1/(2n-1) - 1/(2n+1))$。错位相减最后要记得"再算一步"把公比项除过去，别漏乘 $(1-q)$。
