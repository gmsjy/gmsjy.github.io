#let title = "条件概率与正态分布：在信息中更新判断"
#let date = "2026-09-04"
#let tags = ("数学", "概率统计", "高中")
#let series = "高中数学"
#let series_weight = 19
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

概率论的核心问题从来不是「抛硬币第几次正面」，而是：*已知一些信息后，另一件事的可能性如何改变？*这一课前半讲条件概率——处理信息的代数；后半讲正态分布——自然界最钟爱的钟形曲线。

= 条件概率：信息改变概率

== 定义

设 $A$、$B$ 是两个事件，且 $P(A) > 0$，则在 *$A$ 已经发生*的条件下 $B$ 发生的概率：

$ P(B | A) = (P(A B)) / (P(A)) $ <eq:cond>

直觉读法：$P(B|A)$ 是把样本空间*缩小*到 $A$ 内部之后，$B$ 占据的比例。分母 $P(A)$ 就是「缩小后的世界有多大」。

> 记忆点：$P(A B)$ 读作「A 且 B」（积事件），不是乘法先验。$P(B|A) = P(B)$（信息不改变概率）恰恰是*独立*的定义——独立性是条件概率的特例，不要反过来死记。

== 乘法公式：把条件概率倒过来用

由 @eq:cond 移项：

$ P(A B) = P(A) P(B | A) quad quad quad P(A B) = P(A) P(B_1 | A) P(B_2 | A B_1) $ <eq:mult>

后者是「逐级抽取」问题的标配：不放回抽样、逐个检验、分步选择。

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：一批产品 10 件，其中 3 件次品。不放回地抽 2 件，求两件都是次品的概率。

逐级用 @eq:mult：第一次抽到次品 $P = 3\/10$；在此时条件下第二次又是次品 $P = 2\/9$。

$P = 3/10 times 2/9 = 1/15$。

对比古典概型 $C_3^2 \/ C_10^2 = 3\/45 = 1\/15$——两条路殊途同归。
]

== 全概率公式：按「原因」分账

事件 $B$ 的发生可能经由多个互斥的「渠道」$A_1, A_2, dots, A_n$（它们构成完备事件组），则：

$ P(B) = sum_(i=1)^n P(A_i) P(B | A_i) $ <eq:total>

两渠道版本最好记：$P(B) = P(A)P(B|A) + P(overline(A))P(B|overline(A))$。

== 贝叶斯公式：从结果反推原因

全概率是「由因求果」，贝叶斯是「由果溯因」——已知 $B$ 发生了，问它来自渠道 $A_i$ 的概率：

$ P(A_i | B) = (P(A_i) P(B | A_i)) / (P(B)) $ <eq:bayes>

#fig(cetz.canvas(length: 0.9cm, {
  // 树形图：两个渠道 × 两个结果
  // 根
  circle((0.5, 1.6), radius: 0.08, fill: black, stroke: none)
  content((0.5, 1.95), "起点", size: 8pt)
  // 一级分支：A1 / A2
  line((0.5, 1.6), (2.6, 2.6), stroke: black)
  line((0.5, 1.6), (2.6, 0.6), stroke: black)
  content((2.8, 2.65), $ A_1 $, anchor: "west", size: 9pt)
  content((2.8, 0.55), $ A_2 $, anchor: "west", size: 9pt)
  content((1.4, 2.35), $ P(A_1) $, size: 8pt)
  content((1.4, 0.8), $ P(A_2) $, size: 8pt)
  // 二级分支
  line((3.5, 2.6), (5.6, 3.3), stroke: rgb("#d64541"))
  line((3.5, 2.6), (5.6, 1.9), stroke: rgb("#d64541"))
  line((3.5, 0.6), (5.6, 1.3), stroke: rgb("#0b6fc0"))
  line((3.5, 0.6), (5.6, -0.1), stroke: rgb("#0b6fc0"))
  content((5.8, 3.3), $ B $, anchor: "west", size: 9pt)
  content((5.8, 1.9), $ overline(B) $, anchor: "west", size: 9pt)
  content((5.8, 1.3), $ B $, anchor: "west", size: 9pt)
  content((5.8, -0.1), $ overline(B) $, anchor: "west", size: 9pt)
  content((4.3, 3.15), $ P(B|A_1) $, size: 8pt)
  content((4.3, 2.1), $ P(overline(B)|A_1) $, size: 8pt)
  content((4.3, 1.15), $ P(B|A_2) $, size: 8pt)
  content((4.3, 0.05), $ P(overline(B)|A_2) $, size: 8pt)
  // 每条到 B 的路径 = 一条「渠道」
  content((8.2, 3.2), "到 B 的每条路径", anchor: "west", size: 8pt)
  content((8.2, 2.8), "概率沿途相乘", anchor: "west", size: 8pt)
  content((8.2, 2.4), "全概率：各路径相加", anchor: "west", size: 8pt)
  content((8.2, 2.0), "贝叶斯：除以总和", anchor: "west", size: 8pt)
}))

树形图是这一节的*万能工具*：每条到 $B$ 的路径概率 = 沿途条件概率*相乘*；$P(B)$ = 所有路径*相加*；贝叶斯 = 某一条路径*除以总和*。

= 正态分布：钟形曲线

== 密度曲线的性质

连续型随机变量 $X$ 服从正态分布 $N(mu, sigma^2)$，密度曲线是一条钟形曲线：

- *关于 $x = mu$ 对称*（$mu$ 决定位置）；
- $sigma$ 越小曲线*越瘦高*（取值越集中），$sigma$ 越大越*矮胖*；
- 曲线与 $x$ 轴之间的面积恒为 $1$；$x$ 轴是渐近线。

#fig(cetz.canvas(length: 0.95cm, {
  // 正态曲线 + 3σ 区间标注
  line((0, 0), (10.4, 0), mark: (end: ">"), stroke: black)
  content((10.3, 0.45), $ x $)
  // 曲线：mu=5, 钟形
  let pts = range(0, 41,step: 1).map(k => {
    let x = 1.2 + k / 5.0
    let t = (x - 5.0) / 1.1
    (x, 3.1 * calc.exp(-t * t / 2))
  })
  line(..pts, stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  // mu 虚线
  line((5, 0), (5, 3.15), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  content((5, 3.4), $ mu $, size: 9pt)
  // 3σ 区间竖线（mu=5, sigma=1.1）
  let xs = (1.7, 2.8, 3.9, 6.1, 7.2, 8.3)
  for x in xs {
    line((x, 0), (x, 0.35), stroke: rgb("#555"))
  }
  content((1.7, -0.5), $ -3sigma $, size: 8pt)
  content((2.8, -0.5), $ -2sigma $, size: 8pt)
  content((3.9, -0.5), $ -sigma $, size: 8pt)
  content((6.1, -0.5), $ +sigma $, size: 8pt)
  content((7.2, -0.5), $ +2sigma $, size: 8pt)
  content((8.3, -0.5), $ +3sigma $, size: 8pt)
  // 区间面积标示
  content((5, 0.9), $ 68.3% $, size: 8pt)
  content((5, 0.45), $ 95.4%, 99.7% "覆盖更宽" $, size: 8pt)
}))

== $3 sigma$ 原则：三个黄金数字

当 $X$ 服从 $N(mu, sigma^2)$ 时：

$ P(mu - sigma < X < mu + sigma) approx 68.3% quad quad P(mu - 2sigma < X < mu + 2sigma) approx 95.4% quad quad P(mu - 3sigma < X < mu + 3sigma) approx 99.7% $ <eq:sigma>

*对称性*是解题杠杆：$P(X > mu) = 0.5$；$P(X > mu + sigma) = (1 - 68.3%) \/ 2 approx 15.9%$——尾部概率对半劈。

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：某地区高三男生身高 $X$ 服从 $N(172, 25)$（$sigma = 5$）。随机抽查一名男生，身高在 $(167, 182)$ 内的概率？

区间拆解：$(167, 182) = (mu - sigma, mu + 2sigma)$。

$P = (68.3% + 95.4%) \/ 2 = 81.85%$。

（对称轴两侧各取半个对称区间——画一条数轴标上 $mu +- sigma$、$mu +- 2sigma$ 再算，几乎不会错。）
]

== 正态分布为什么无处不在

误差定律：大量*独立微小*因素的叠加（遗传、营养、测量抖动……）趋向正态分布。测量误差、人群身高、批量产品的尺寸——「中间多、两头少、左右对称」是自然界的默认形状。这也是质量控制「$3 sigma$ 管理」的数学根基：落在 $3sigma$ 之外的概率仅约 $0.3%$，出现即视为异常。

= 解题方法论

- *条件概率先画树*：两渠道三渠道一律画树形图，乘法、全概率、贝叶斯三公式全部从图上「读」出来，不硬背；
- *区分 $P(A B)$ 与 $P(B|A)$*：前者是两事件*同时*发生（全空间），后者是 $A$ 发生*前提下*（缩小后的空间）——条件概率题一半的错误源于混用；
- *独立性的判据*：$P(A B) = P(A)P(B)$ 或 $P(B|A) = P(B)$，题给「相互独立」直接乘；
- *正态题先写 $mu$、$sigma$*：把区间改写成 $mu +- k sigma$ 的形式再套 @eq:sigma，配合对称性拆分；
- *独立重复*：若一节与二项分布结合（$n$ 重伯努利），认出结构后直接 $C_n^k p^k (1-p)^(n-k)$。

一句话收束：条件概率教我们*用信息更新概率*，贝叶斯把这件事反过来问；正态分布则保证——只要随机因素足够多、足够独立，钟形曲线终将浮现。
