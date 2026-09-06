#let title = "电学实验：伏安法与电源的特性"
#let date = "2026-09-05"
#let tags = ("物理", "实验", "电磁学", "高中")
#let series = "高中物理实验"
#let series_weight = 2
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

电学实验的核心矛盾：电表不是理想的。电流表有微小内阻、电压表会分走电流——*怎么接表、怎么读图*决定了测得准不准。这一篇覆盖两个必做实验与一类重要元件。

= 伏安法测电阻：内接还是外接

用电压表测 $R_x$ 两端电压、电流表测通过它的电流，$R = U \/ I$。但两表必然一个"测不准"：

- *电流表外接*（电压表直接并联在 $R_x$ 两端）：电压读数准确，但电压表分走了一部分电流——测得值偏小。适合测*小电阻*（$R_x$ 远小于电压表内阻时分流可忽略）。
- *电流表内接*（电流表串联在 $R_x$ 支路里）：电流读数准确，但电流表分压——测得值偏大。适合测*大电阻*（$R_x$ 远大于电流表内阻时分压可忽略）。

#fig(cetz.canvas(length: 1.0cm, {
  // 外接法电路：电源(下) — A(上) — Rx(右) — 回路；V 并联 Rx
  // 回路
  line((0.4, 3.4), (0.4, 1.2), stroke: black)
  line((5.6, 3.4), (5.6, 1.2), stroke: black)
  line((0.4, 1.2), (2.4, 1.2), stroke: black)
  line((3.6, 1.2), (5.6, 1.2), stroke: black)
  line((0.4, 3.4), (1.6, 3.4), stroke: black)
  line((4.4, 3.4), (5.6, 3.4), stroke: black)
  // 电源：两短一长
  line((1.6, 3.2), (2.4, 3.2), stroke: (paint: black, thickness: 2pt))
  line((1.9, 3.05), (2.1, 3.05), stroke: black)
  content((0.9, 3.35), [E], size: 9pt)
  // 电流表
  circle((3.0, 3.4), radius: 0.4, stroke: black)
  content((3.0, 3.4), [A], size: 10pt)
  // 待测电阻 Rx（右竖边）
  rect((5.35, 1.9), (5.85, 2.7), stroke: black, fill: rgb("#e6e2d6"))
  content((6.1, 2.3), $ R_x $, anchor: "west", size: 9pt)
  // 电压表并联
  line((5.6, 2.7), (5.6, 3.4), stroke: black)
  line((5.6, 1.9), (5.6, 1.2), stroke: black)
  circle((5.6, 3.4), radius: 0.36, stroke: rgb("#d64541"))
  content((5.6, 3.4), [V], size: 9pt, stroke: white)
  content((4.1, 3.75), [外接：V 测准、A 偏大 → $R$ 偏小], size: 8pt)
}))

口诀记法：*大内小外*——大电阻用内接、小电阻用外接。更稳的办法是试触法：两次接法中哪个表的读数变化明显，就说明它被"污染"得更少，选变化小的那种接法。

= 测电源的电动势和内阻

电源的特性由两个参数描述：电动势 $E$ 与内阻 $r$，满足 $U = E - I r$。用电压表测路端电压 $U$、电流表测干路电流 $I$，改变滑动变阻器取多组 $(I, U)$：

#fig(cetz.canvas(length: 1.0cm, {
  line((-0.3, 0), (4.6, 0), mark: (end: ">"), stroke: black)
  line((0, -0.3), (0, 3.3), mark: (end: ">"), stroke: black)
  content((4.8, 0), $ I \/ "A" $, size: 9pt)
  content((0.45, 3.4), $ U \/ "V" $, size: 9pt)
  content((0.35, -0.35), [0], size: 8pt)
  for i in range(6) {
    let x = 0.6 + i * 0.55
    circle((x, 2.6 - 0.38 * i), radius: 0.05, fill: rgb("#0b6fc0"), stroke: none)
  }
  line((0, 2.75), (4.0, 0.35), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  line((0, 2.75), (0.5, 2.75), stroke: (paint: rgb("#3c8a4d"), thickness: 1.2pt))
  content((0.15, 2.95), $ E $, size: 9pt)
  content((3.3, 0.8), [斜率大小 = $r$], size: 9pt)
}))

$U$–$I$ 图线是一条下降直线：*纵轴截距就是 $E$*，*斜率大小就是 $r$*。让直线"跑"出大量数据点的共同趋势，偶然误差自动被平均掉——这是图像法在电学里的标准用法。注意内阻较小时电流要控制得小（放电太猛会发热改变内阻），必要时给电源串一个定值保护电阻。

= 多用电表与传感器初探

多用电表是"三表合一"：直流电压挡、直流电流挡、欧姆挡（外加交流挡）。欧姆挡内部自带电池，读数 = 刻度 × 倍率，*每次换挡必须调零*；测电阻时被测元件要与电路断开。

欧姆挡的原理恰好是理解*传感器*的钥匙——传感器把"非电学量"翻译成电阻变化：

- *热敏电阻*：半导体材质，温度升高电阻*减小*（与金属热电阻相反）；
- *光敏电阻*：光照增强电阻*减小*。

#fig(cetz.canvas(length: 1.0cm, {
  line((-0.3, 0), (5.6, 0), mark: (end: ">"), stroke: black)
  line((0, -0.3), (0, 2.9), mark: (end: ">"), stroke: black)
  content((5.8, 0), [光照强度], size: 9pt)
  content((0.5, 3.0), [电阻 $R$], size: 9pt)
  let pts1 = range(0, 41, step: 1).map(k => {
    let x = 5 * k / 40
    (x, 2.3 / (x + 0.4))
  })
  let pts2 = range(0, 41, step: 1).map(k => {
    let x = 5 * k / 40
    (x, 1.35 / (x + 0.4) + 0.35)
  })
  line(..pts1, stroke: (paint: rgb("#d64541"), thickness: 1.4pt))
  line(..pts2, stroke: (paint: rgb("#0b6fc0"), thickness: 1.4pt))
  content((4.3, 1.35), [热敏], size: 8.5pt, stroke: white)
  content((4.3, 0.75), [金属], size: 8.5pt, stroke: white)
  content((1.4, 2.5), [升温/增光 → R 下降], size: 8pt)
}))

应用俯拾皆是：光控路灯（光敏电阻 + 电磁继电器）、火灾报警（热敏电阻）、电子秤（应变片）……"非电学量 → 电阻 → 电压/电流 → 电路做出反应"，这条翻译链条就是传感器的全部逻辑。

= 小结

- 伏安法口诀"大内小外"：接法误差来自电表不理想，试触法可辅助判断；
- 测 $E$、$r$：$U$–$I$ 图线，纵截距 $E$、斜率大小 $r$；
- 多用电表欧姆挡换挡调零；热敏/光敏电阻把温度与光照翻译成电阻，是传感器的核心；
- 电学实验的通用心法：*读准图、选对表、控住流*。
