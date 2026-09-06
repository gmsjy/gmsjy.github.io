#let title = "解三角形：正弦定理与余弦定理"
#let date = "2026-09-04"
#let tags = ("数学", "三角函数", "解三角形", "高中")
#let series = "高中数学"
#let series_weight = 6
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 解三角形的两条定理

三角形 $A B C$ 中，角 $A$、$B$、$C$ 所对的边分别为 $a$、$b$、$c$，外接圆半径为 $R$。

#fig(cetz.canvas(length: 0.9cm, {
  // 记号约定：角 A、B、C 与所对边 a、b、c
  line((0.4, 0.4), (6, 0.4), stroke: (paint: black, thickness: 1.2pt))
  line((6, 0.4), (2.2, 3.2), stroke: (paint: black, thickness: 1.2pt))
  line((2.2, 3.2), (0.4, 0.4), stroke: (paint: black, thickness: 1.2pt))
  content((3.2, 0.05), $ c $, size: 9pt)
  content((4.75, 2.0), $ a $, size: 9pt)
  content((0.95, 2.0), $ b $, size: 9pt)
  content((0.85, 0.8), $ A $, size: 9pt)
  content((5.45, 0.8), $ B $, size: 9pt)
  content((2.3, 2.7), $ C $, size: 9pt)
  content((3.2, 3.6), "每个角与它对面的边共用一个字母（大小写有别）", size: 8pt)
}))

== 正弦定理

$ a/(sin A) = b/(sin B) = c/(sin C) = 2 R $ <eq:sine>

变形：$a = 2 R sin A$、$b = 2 R sin B$、$c = 2 R sin C$（"边化角"）；反过来 $sin A = a/(2R)$ 等（"角化边"）。

== 余弦定理

$ a^2 = b^2 + c^2 - 2 b c cos A $ <eq:cosine>

（轮换得另两式）变形求角：$cos A = (b^2 + c^2 - a^2)/(2 b c)$。

> 选择口诀：*"已知两角一边用正弦，已知两边一角（夹角）用余弦，已知三边求角用余弦变形"*。已知两边及其中一边的对角（$S S A$）用正弦定理，但要讨论解的个数。

= 三角形面积公式

$ S = 1/2 a b sin C = 1/2 b c sin A = 1/2 c a sin B $ <eq:area>

海伦公式（已知三边）：$S = sqrt(p(p-a)(p-b)(p-c))$，其中半周长 $p = (a+b+c)/2$。

= "边边角"解的个数判断

已知两边 $a$、$b$ 及角 $A$（$A$ 为锐角），由 $sin B = (b sin A)/a$ 讨论：

#table(
  columns: (auto, 1fr),
  inset: 6pt,
  [*条件*], [*结论*],
  [$a < b sin A$], [无解],
  [$a = b sin A$], [一解（$B = 90 degree$，直角三角形）],
  [$b sin A < a < b$], [两解（$B$ 可为锐角或钝角）],
  [$a >= b$], [一解],
)

> 判断核心：$sin B$ 必须在 $(0, 1]$ 内，且 $A + B < 180 degree$。$A$ 为钝角时，$a$ 必须大于 $b$ 才有解，且只有一解。

= 边角关系与三角形形状判断

- *大边对大角*：$a > b <=> A > B$；
- *正弦判断*：$sin A > sin B <=> a > b <=> A > B$；
- *余弦判断*：$a^2 < b^2 + c^2$ 则 $A$ 为锐角，$a^2 = b^2 + c^2$ 则 $A = 90 degree$，$a^2 > b^2 + c^2$ 则 $A$ 为钝角。

> 判断三角形形状：先看余弦（谁最大谁定钝角），或用正弦定理统一化为边或角。结论常见"等腰""直角""等边""等腰直角"四种。

= 实际应用：测量与方位

- *仰角 / 俯角*：视线与水平线的夹角，抬头为仰角、低头为俯角；
- *方位角*：从正北方向顺时针转到目标方向的夹角（如"北偏东 $30 degree$"）；
- *距离测量*：隔河测距、测山高，构造可解的三角形，用正弦或余弦定理求未知边。

> 应用题四步：① 画示意图，标出已知的角与边；② 确定要求解的三角形；③ 判断用正弦还是余弦定理；④ 代入求解并回代检验。*画图是解三角形应用题的灵魂，图对了题就成了一半。*

= 综合套路

解三角形大题常与三角函数、三角恒等变换、最值综合：

1. 用正弦定理"边化角"或余弦定理"角化边"；
2. 结合 $A + B + C = pi$、诱导公式、辅助角公式化简；
3. 求边长范围或面积最值（常落到"已知一边及其对角，求面积最大值"——$S = 1/2 b c sin A$，用余弦定理 + 基本不等式求 $b c$ 最大值）。
