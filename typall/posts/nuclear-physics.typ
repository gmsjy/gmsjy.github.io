#let title = "原子核物理：放射性、结合能与核能"
#let date = "2026-09-04"
#let tags = ("物理", "近代物理", "高中")
#let series = "高中物理"
#let series_weight = 19
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

上一课看到光有波动性；这一课钻进原子*最深处*——原子核。放射性、核反应、质能方程，把物理学从「能量守恒」推向更根本的「质能守恒」，也把人类带到了核能（与核武器）的门口。

= 天然放射现象：原子核自己会变

贝克勒尔发现铀盐能使底片感光——*原子序数大于 83 的元素*（以及不少人工核素）会自发放出射线，这种现象叫*天然放射现象*。放射性*只与原子核有关*，与核外电子、化学状态、温度压强无关——它是核性质的直接展示。

三种射线一览：

| 射线 | 本质 | 电荷 | 穿透力 | 电离能力 |
|------|------|------|--------|----------|
| $alpha$ | 氦核 $"He"^4_2$ | $+2e$ | 一张纸可挡 | *最强* |
| $beta$ | 高速电子流 | $-e$ | 穿几毫米铝板 | 较弱 |
| $gamma$ | 高频电磁波（光子） | 0 | 穿几厘米铅板 | *最弱* |

#fig(cetz.canvas(length: 0.85cm, {
  // 磁场中三射线偏转：γ 直行，α β 反向偏折
  rect((0, -0.6), (9, 4.6), fill: rgb("#f2f0ec"), stroke: none)
  content((0.6, 4.2), "匀强磁场（指向纸外）", size: 8pt)
  // 放射源
  circle((1, 2), radius: 0.12, fill: black, stroke: none)
  content((1, 1.55), "放射源", size: 8pt)
  // γ 直行
  line((1, 2), (8.6, 2), mark: (end: ">"), stroke: (paint: rgb("#555"), thickness: 1.2pt))
  content((8.2, 2.3), $ gamma $, size: 10pt)
  // α 偏折（向一侧，弯曲半径大）
  let arc1 = range(0, 21,step: 1).map(k => {
    let a = 80 * k / 20.0
    let r = 9.0
    (1 + r * (calc.sin(a * calc.pi / 180)), 2 + r * (1 - calc.cos(a * calc.pi / 180)))
  })
  line(..arc1, mark: (end: ">"), stroke: rgb("#d64541"))
  content((6.3, 3.9), $ alpha $, size: 10pt)
  // β 偏折（另一侧，弯曲明显：半径小）
  let arc2 = range(0, 21,step: 1).map(k => {
    let a = 80 * k / 20.0
    let r = 4.2
    (1 + r * (calc.sin(a * calc.pi / 180)), 2 - r * (1 - calc.cos(a * calc.pi / 180)))
  })
  line(..arc2, mark: (end: ">"), stroke: rgb("#0b6fc0"))
  content((4.3, -0.25), $ beta $, size: 10pt)
}))

*左手定则*判定偏转方向：磁场指向纸外、速度向右 → 正电的 $alpha$ 向上弯，负电的 $beta$ 向下弯，中性 $gamma$ 不偏。三射线在磁场中的行为是区分它们最经典的实验手段。

= 半衰期：统计规律下的「寿命」

放射性核素有*一半*发生衰变所需的时间是确定的，叫*半衰期* $T$：

$ N = N_0 (1 / 2)^(t \/ T) $ <eq:halflife>

#fig(cetz.canvas(length: 0.85cm, {
  // 半衰期指数衰减曲线：每过一个 T 剩一半
  line((0, 0), (8.8, 0), mark: (end: ">"), stroke: black)
  line((0, 0), (0, 3.6), mark: (end: ">"), stroke: black)
  content((8.6, 0.4), $ t $)
  content((0.35, 3.5), $ N $)
  let pts = range(0, 41,step: 1).map(k => {
    let x = k / 5.0
    (x, 3.2 * calc.pow(0.5, x / 2))
  })
  line(..pts, stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  // 台阶标注 1/2, 1/4, 1/8
  let marks = ((2, 1.6, "1/2"), (4, 0.8, "1/4"), (6, 0.4, "1/8"))
  for m in marks {
    line((m.at(0), 0), (m.at(0), m.at(1)), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
    circle((m.at(0), m.at(1)), radius: 0.07, fill: rgb("#d64541"), stroke: none)
    content((m.at(0) + 0.15, m.at(1)), m.at(2), anchor: "west", size: 8pt)
  }
  line((0, 3.2), (2, 3.2), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  content((0.2, 3.35), $ N_0 $, size: 9pt)
  for i in range(1, 4) {
    content((i * 2 - 1, -0.4), $ T $, size: 9pt)
  }
}))

要点：

- *统计规律*：半衰期描述的是*大量*原子核的统计行为，对*少数几个*核没有意义（不能说「3 个核经过 1 个半衰期剩 1.5 个」）；
- *由核自身决定*：半衰期不受温度、压强、化学状态影响，无法用物理化学手段「加速」或「延缓」；
- 考古上用碳-14（$T approx 5730$ 年）测年代，医学上用短半衰期核素做显像以减少伤害。

> 记忆点：剩余质量/数目按 @eq:halflife 折半，但*已经衰变掉的量* = 原有 − 剩余。题目常在「剩余了」与「衰变了」之间设陷阱。

= 核反应方程与两类守恒

原子核发生变化（衰变、人工转变、裂变、聚变）时，写核反应方程要遵守两条铁律：

- *电荷数守恒*（下标之和相等）；
- *质量数守恒*（上标之和相等）。

典型核反应：

$ "铀-238 的 α 衰变：" quad "U"^238_92 -> "Th"^234_90 + "He"^4_2 $ <eq:alpha-decay>

$ "钴-60 的 β 衰变：" quad "Co"^60_27 -> "Ni"^60_28 + "e"^0_-1 $ <eq:beta-decay>

$ "卢瑟福发现质子：" quad "N"^14_7 + "He"^4_2 -> "O"^17_8 + "H"^1_1 $ <eq:rutherford>

$ "查德威克发现中子：" quad "Be"^9_4 + "He"^4_2 -> "C"^12_6 + "n"^1_0 $ <eq:chadwick>

注意 $beta$ 衰变里放出的电子是*核内中子转变为质子*时产生的（$n -> p + e$），不是核外电子。

= 质量亏损与质能方程

实验发现：原子核的质量*小于*组成它的核子质量之和，差值叫*质量亏损* $Delta m$。爱因斯坦给出解释：

$ E = Delta m c^2 $ <eq:emc2>

核子结合成核时「亏损」的质量以能量形式释放——这部分能量叫*结合能*，$Delta m c^2$ 越大核越稳定。*任何*核反应只要释放能量，就伴随质量亏损；反之要把核拆开，必须补回这么多能量。

> 记忆点：质量亏损不是「质量消失了」，而是*质量与能量本是一体*（质能守恒取代了孤立的质能各自守恒）。$1 u approx 931.5 "MeV"$ 是常用换算。

== 裂变与聚变：两种获能路径

- *重核裂变*：$"U"^235_92$ 俘获一个慢中子后分裂成两个中等核并放出 $2 \/ 3$ 个中子——中子再引发新的裂变，形成*链式反应*（核电站的反应堆用控制棒控制链式反应速率）；
- *轻核聚变*：氘、氚聚合成氦，单位质量释能*远大于*裂变——太阳的能量来源；实现需要*上亿度*高温克服核间库仑斥力（「人造太阳」托卡马克研究的正是可控聚变）。

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：某放射性核素半衰期 $T = 8$ 天，原有 $16 g$，经过 $24$ 天后剩多少？衰变掉了多少？

$24 \/ T = 3$ 个半衰期：剩余 $m = 16 times (1\/2)^3 = 2 g$；衰变掉 $16 - 2 = 14 g$。

*易错*：题问「衰变掉多少」时答「剩 2 g」就掉坑了。
]

= 解题方法论

- *核反应方程两查*：写完立刻查「上标和下标」各守恒；未知核/粒子用守恒两条解出；
- *射线三件套*：穿透力、电离能力、磁场偏转——按 $alpha \/ beta \/ gamma$ 顺序对号入座；
- *半衰期先数次数*：$n = t \/ T$，再套 $(1\/2)^n$；注意区分「剩余」与「衰变掉」；
- *质能方程单位*：质量用 "kg" 配 $E = Delta m c^2$，或用原子质量单位 $u$ 配 $931.5 "MeV"$——两套单位不要混。

一句话收束：核物理把「元素不可变」的信念打碎，又用 $E = Delta m c^2$ 把质量与能量焊在一起——微观世界的规则，恰恰在这里决定了人类世界的能源格局。
