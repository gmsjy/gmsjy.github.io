#let title = "复数：给方程一个完整的解"
#let date = "2026-09-02"
#let tags = ("数学", "复数", "高中")
#let series = "高中数学"
#let series_weight = 10
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 数系的扩充与复数的概念

方程 $x^2 + 1 = 0$ 在实数范围内无解，因为负数的平方根不存在。数学家的对策不是认输，而是*发明一个新数*：规定 $i$ 满足 $i^2 = -1$，$i$ 叫*虚数单位*。于是 $x^2 + 1 = 0$ 的解为 $x = plus.minus i$。

把实数与虚数单位线性组合起来，得到*复数*的一般形式：

$ z = a + b i, quad a, b in RR $ <eq:complex-form>

$a$ 叫*实部*（记 $Re(z)$），$b$ 叫*虚部*（记 $Im(z)$）。注意虚部是 $b$ 本身而不含 $i$。

两个复数相等的充要条件非常朴素：

$ a + b i = c + d i <=> a = c, space b = d $ <eq:complex-eq>

即"实部等于实部，虚部等于虚部"——复数相等问题永远化归为实数方程组。复数的分类按虚部是否为 $0$ 一刀切：

#table(
  columns: (auto, auto, auto),
  inset: 6pt,
  [*类型*], [*条件*], [*例子*],
  [实数], [$b = 0$], [$3$、$-1/2$、$sqrt(2)$],
  [虚数], [$b != 0$], [$2 + i$、$-3 i$],
  [纯虚数], [$a = 0$ 且 $b != 0$], [$2 i$、$-sqrt(3) i$],
)

> 易错点：① 说"$z$ 是纯虚数"要同时满足实部为 $0$ *且*虚部不为 $0$——题设常给 $z = (m^2 - 1) + (m + 1) i$ 问何时为纯虚数，很多人漏掉 $b != 0$ 的检验；② 虚数（非实数）*不能比较大小*，只有实部虚部同为实数时才能谈大小；③ $i$ 与 $-i$ 都是 $x^2 = -1$ 的解，缺一不可。

= 复数的四则运算

== 加减与乘法：当多项式算

复数加减就是"实部相加减、虚部相加减"：$(a + b i) plus.minus (c + d i) = (a plus.minus c) + (b plus.minus d) i$。

复数乘法把 $i$ 当字母展开，再把 $i^2$ 换成 $-1$：

$ (a + b i)(c + d i) = (a c - b d) + (a d + b c) i $ <eq:complex-mul>

$i$ 的幂四个一循环，是化简的快捷方式：

$ i^1 = i, i^2 = -1, i^3 = -i, i^4 = 1, quad i^(4k + r) = i^r $ <eq:i-power>

> 速算：求 $i^2026$ 就看 $2026 = 4 times 506 + 2$，余数 $2$，故 $i^2026 = i^2 = -1$。*余数定幂*。

== 共轭复数：除法实数化的钥匙

与 $z = a + b i$ 实部相同、虚部相反的复数 $a - b i$ 叫 $z$ 的*共轭复数*，记 $overline(z)$。共轭的两条黄金性质：

$ z + overline(z) = 2 a, quad z overline(z) = a^2 + b^2 = abs(z)^2 $ <eq:conjugate>

第二条是*分母实数化*的依据——复数除法就是"乘以分母的共轭再约分"：

$ (a + b i)/(c + d i) = ((a + b i)(c - d i))/((c + d i)(c - d i)) = (a c + b d)/(c^2 + d^2) + (b c - a d)/(c^2 + d^2) i $ <eq:complex-div>

#table(
  columns: (auto, auto),
  inset: 6pt,
  [*运算*], [*要点*],
  [加减], [实部虚部分别算，当"同类项"合并],
  [乘法], [展开后 $i^2 -> -1$，结果仍是 $a + b i$ 形式],
  [除法], [分子分母同乘分母的共轭，分母化为实数 $c^2 + d^2$],
  [乘方], [先用 $i$ 的周期把幂次化到 1 至 4，或先用 $(a plus.minus b i)^2 = a^2 - b^2 plus.minus 2 a b i$ 展开],
)

= 复数的几何意义

== 复平面：复数 ↔ 点 ↔ 向量

用直角坐标系的横轴表示实部（*实轴*）、纵轴表示虚部（*虚轴*），这样的平面叫*复平面*。复数 $z = a + b i$ 与点 $Z(a, b)$、向量 $arrow(O Z)$ 一一对应——复数的加减因此有了平行四边形法则的几何图像，向量的一切工具（模、距离、中点）全部可以搬过来。

#fig(cetz.canvas(length: 0.9cm, {
  // 复平面：z 与共轭 z̄ 关于实轴对称
  line((-0.3, 0), (5.2, 0), mark: (end: ">"), stroke: black)
  line((0, -2.9), (0, 3), mark: (end: ">"), stroke: black)
  content((5.1, 0.4), "实轴", size: 8pt)
  content((0.55, 2.9), "虚轴", size: 8pt)
  line((0, 0), (3.4, 2.1), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.3pt))
  circle((3.4, 2.1), radius: 0.07, fill: rgb("#d64541"), stroke: none)
  circle((3.4, -2.1), radius: 0.07, stroke: rgb("#0b6fc0"), fill: none)
  line((3.4, 2.1), (3.4, -2.1), stroke: (paint: rgb("#9aa2ad"), dash: "dashed"))
  line((3.4, 0), (3.4, 0), stroke: black)
  content((3.75, 2.25), $ z = a + b i $, size: 9pt)
  content((3.8, -2.3), $ overline(z) = a - b i $, size: 9pt)
  content((1.6, 1.3), $ abs(z) $, size: 9pt)
  content((4.15, 0.3), $ a $, size: 8pt)
  content((-0.35, 1.05), $ b $, size: 8pt)
  content((4.4, 1.15), "关于实轴对称", size: 8pt)
}))

复数的*模*就是向量的长度：

$ abs(z) = sqrt(a^2 + b^2) $ <eq:complex-mod>

共轭的几何意义：$z$ 与 $overline(z)$ 关于实轴对称。$abs(z_1 - z_2)$ 表示复平面上两点 $Z_1$、$Z_2$ 间的距离——于是 $abs(z - z_0) = r$ 描述以 $z_0$ 为圆心、$r$ 为半径的*圆*，这是复数几何题的常客。

模的三条性质要会用（都源于 $abs(z)^2 = z overline(z)$）：

- $abs(z_1 z_2) = abs(z_1) abs(z_2)$（乘积的模等于模的乘积）；
- $abs(z_1/z_2) = abs(z_1)/abs(z_2)$（$z_2 != 0$）；
- $abs(z_1 + z_2) <= abs(z_1) + abs(z_2)$（三角不等式，几何上是"两边之和大于第三边"）。

> 例题感：$abs(z) = 1$ 时求 $abs(z - 3 + 4 i)$ 的最小值——$z$ 在以原点为圆心半径 $1$ 的圆上，$z - (3 - 4 i)$ 的模是圆上的点到定点 $(3, -4)$ 的距离，最小值为"距离减半径"：$sqrt(3^2 + 4^2) - 1 = 4$。*复数最值问题先想几何意义*。

== 实系数一元二次方程的虚根

实系数方程 $a x^2 + b x + c = 0$（$a != 0$）判别式 $Delta < 0$ 时，在复数范围内仍有两个根：

$ x = (-b plus.minus sqrt(4 a c - b^2) i)/(2 a) $ <eq:quadratic-complex>

它们是一对*共轭虚根*（$b = 0$ 时退化为一对相反纯虚数）。*韦达定理在复数范围内照常成立*：$x_1 + x_2 = -b/a$，$x_1 x_2 = c/a$——这给"已知一根求另一根"的题提供了免解方程的捷径。

> 总结：数系从自然数到整数、有理数、实数、复数，每扩充一次都解决一类"不够用"：减法、除法、开方（负数平方根）。复数集是最大的封闭数系——四则运算、乘方开方都"关得住门"，高中阶段到此为止，不再扩充。
