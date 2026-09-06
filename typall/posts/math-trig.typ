#let title = "三角函数：把圆周运动翻译成函数"
#let date = "2026-09-02"
#let tags = ("数学", "三角函数", "高中")
#let series = "高中数学"
#let series_weight = 5
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= 任意角与弧度制

初中只认识 $0°$ 到 $360°$ 的角，但"旋转"可以转好几圈、也可以反向转。于是把角的概念推广：一条射线绕端点*逆时针*旋转得到*正角*，*顺时针*旋转得到*负角*，不旋转是*零角*。这样角就有了任意大小与方向。

- 角的终边落在第几象限，就叫第几*象限角*（始边固定为 $x$ 轴正半轴）；落在坐标轴上叫*轴线角*；
- 与角 $alpha$ 终边相同的角的集合：
  $ {beta | beta = alpha + 2 k pi, space k in ZZ} $ <eq:coterminal>

> 易错点：终边相同的角有*无穷多个*，写集合时一定要带上 $k in ZZ$。判断象限角时先把角化到 $(0, 2 pi)$：如 $-210°$ 先加 $360°$ 得 $150°$，在第二象限。

== 弧度制

*弧度*用"弧长与半径之比"度量角。当弧长恰好等于半径时，对应圆心角定义为 $1 "rad"$。一整圈弧长 $2 pi r$，所以：

$ 180° = pi "rad", quad 1 "rad" = (180°)/(pi) approx 57.3°, quad 1° = (pi)/180 "rad" $ <eq:rad-conv>

换算口诀：角度乘 $pi/180$ 变弧度，弧度乘 $180/pi$ 变角度。高中做题默认用弧度制，几个常用值要秒答：

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto, auto),
  inset: 6pt,
  [*角度*], [$0°$], [$30°$], [$45°$], [$60°$], [$90°$], [$180°$], [$360°$],
  [*弧度*], [$0$], [$pi/6$], [$pi/4$], [$pi/3$], [$pi/2$], [$pi$], [$2 pi$],
)

弧度制的价值在于弧长、扇形面积公式不再含 $pi/180$ 的系数：

$ l = alpha r, quad S = 1/2 alpha r^2 = 1/2 l r $ <eq:sector>

> 记忆法：扇形面积长得像三角形——$1/2$"底"（弧长 $l$）乘"高"（半径 $r$）。

= 任意角的三角函数

== 定义：单位圆视角

把角放进*单位圆*（半径 $1$），终边与单位圆交于点 $P(x, y)$，则：

$ sin alpha = y, quad cos alpha = x, quad tan alpha = y/x space (x != 0) $ <eq:def-trig>

一句话记牢：*正弦看纵坐标，余弦看横坐标，正切看斜率*。若点 $P(x, y)$ 在半径为 $r$ 的圆上，则 $sin alpha = y/r$，$cos alpha = x/r$，$tan alpha = y/x$。

#fig(cetz.canvas(length: 0.9cm, {
  // 单位圆定义：终边交圆于 P，纵坐标是 sin，横坐标是 cos
  circle((1.9, 1.7), radius: 1.6, stroke: rgb("#9aa2ad"))
  line((-0.3, 1.7), (4.4, 1.7), mark: (end: ">"), stroke: black)
  line((1.9, -0.3), (1.9, 3.7), mark: (end: ">"), stroke: black)
  content((4.3, 1.45), $ x $, size: 9pt)
  content((2.2, 3.6), $ y $, size: 9pt)
  content((1.65, 1.4), $ O $, size: 8pt)
  line((1.9, 1.7), (2.93, 2.93), mark: (end: ">"), stroke: (paint: rgb("#d64541"), thickness: 1.4pt))
  circle((2.93, 2.93), radius: 0.08, fill: rgb("#d64541"), stroke: none)
  line((2.93, 2.93), (2.93, 1.7), stroke: (paint: rgb("#0b6fc0"), dash: "dashed"))
  content((3.15, 3.1), $ P (cos theta, sin theta) $, size: 8pt)
  content((2.4, 2.1), $ theta $, size: 9pt)
  content((2.4, 1.45), $ cos theta $, size: 8pt)
  content((3.05, 2.35), $ sin theta $, size: 8pt)
}))

特殊角的三角函数值直接背表（用弧度记，斜杠线是分数线不是除号，分子照抄分母照抄即可）：

#table(
  columns: (auto, auto, auto, auto),
  inset: 6pt,
  [*角（弧度）*], [$sin$], [$cos$], [$tan$],
  [$0$], [$0$], [$1$], [$0$],
  [$pi/6$], [$1/2$], [$sqrt(3)/2$], [$sqrt(3)/3$],
  [$pi/4$], [$sqrt(2)/2$], [$sqrt(2)/2$], [$1$],
  [$pi/3$], [$sqrt(3)/2$], [$1/2$], [$sqrt(3)$],
  [$pi/2$], [$1$], [$0$], [不存在],
)

== 符号法则与同角关系

各象限三角函数的正负由"谁为正"决定：*一全正、二正弦、三正切、四余弦*（按 $sin / cos / tan$ 记）——第一象限全正，第二象限只有正弦为正，第三象限只有正切为正，第四象限只有余弦为正。

*同角基本关系*只有两条，却是恒等变换的全部底料：

$ sin^2 alpha + cos^2 alpha = 1, quad tan alpha = (sin alpha)/(cos alpha) $ <eq:same-angle>

> 技巧："弦化切"。已知 $tan alpha = t$ 时，凡遇到 $sin alpha cos alpha$、$sin alpha plus.minus cos alpha$ 这类式子，把分母凑成 $sin^2 alpha + cos^2 alpha$（即隐形的 $1$），再整体除以 $cos^2 alpha$ 就全部化为 $t$ 的式子。符号归属（开方取正负）由 $alpha$ 所在象限决定。

= 诱导公式

*奇变偶不变，符号看象限。* 这句话是全部诱导公式的总纲：

- "奇变偶不变"：$k pi/2 plus.minus alpha$ 中 $k$ 为奇数时函数名变（$sin$ 与 $cos$ 互换）；$k$ 为偶数时函数名不变；
- "符号看象限"：把 $alpha$ *假想为锐角*，看 $k pi/2 plus.minus alpha$ 落在哪个象限，取原函数在该象限的符号。

#table(
  columns: (auto, auto, auto, auto),
  inset: 6pt,
  [*角*], [$sin$], [$cos$], [$tan$],
  [$-alpha$], [$-sin alpha$], [$cos alpha$], [$-tan alpha$],
  [$pi - alpha$], [$sin alpha$], [$-cos alpha$], [$-tan alpha$],
  [$pi + alpha$], [$-sin alpha$], [$-cos alpha$], [$tan alpha$],
  [$2 pi - alpha$], [$-sin alpha$], [$cos alpha$], [$-tan alpha$],
  [$pi/2 - alpha$], [$cos alpha$], [$sin alpha$], [$1/(tan alpha)$],
  [$pi/2 + alpha$], [$cos alpha$], [$-sin alpha$], [$-1/(tan alpha)$],
)

化简的固定套路是"*负角化正、大角化小、化到锐角*"三步走。例：$sin(-1500°)$——先去负号、再减去 $4 times 360°$ 得 $sin(300°)$？不，$-1500° + 4 times 360° = -60°$，实际应加 $5 times 360°$ 得 $300°$，$sin 300° = -sin 60° = -sqrt(3)/2$。

> 易错点：用公式前先判断函数所在的象限符号，符号错了整题白做；$pi/2 plus.minus alpha$ 的公式是函数名互变的根源（正弦变余弦）。

= 三角函数的图像与性质

== 五点法与图像

$y = sin x$ 在 $[0, 2 pi]$ 上描*五个关键点*：$(0, 0)$、$(pi/2, 1)$、$(pi, 0)$、$(3 pi)/2, -1$、$(2 pi, 0)$，光滑连接即得正弦曲线。$y = cos x$ 的图像是正弦曲线左移 $pi/2$；$y = tan x$ 在 $x = pi/2 + k pi$ 处有垂直渐近线，被"切"成一段段上升曲线。

== 三大性质对比表

#table(
  columns: (auto, auto, auto, auto),
  inset: 6pt,
  [*函数*], [$y = sin x$], [$y = cos x$], [$y = tan x$],
  [*定义域*], [$RR$], [$RR$], [$x != pi/2 + k pi, k in ZZ$],
  [*值域*], [$[-1, 1]$], [$[-1, 1]$], [$RR$],
  [*最小正周期*], [$2 pi$], [$2 pi$], [$pi$],
  [*奇偶性*], [奇函数], [偶函数], [奇函数],
)

#table(
  columns: (auto, auto, auto, auto, auto),
  inset: 6pt,
  [*函数*], [单调递增区间], [单调递减区间], [对称轴], [对称中心],
  [$sin x$], [$[-pi/2 + 2 k pi, pi/2 + 2 k pi]$], [$[pi/2 + 2 k pi, 3 pi/2 + 2 k pi]$], [$x = pi/2 + k pi$], [$(k pi, 0)$],
  [$cos x$], [$[-pi + 2 k pi, 2 k pi]$], [$[2 k pi, pi + 2 k pi]$], [$x = k pi$], [$(pi/2 + k pi, 0)$],
  [$tan x$], [$(-pi/2 + k pi, pi/2 + k pi)$], [无], [无], [$(k pi)/2, 0$],
)

== y = A sin(omega x + phi) 的图像

参数各管一件事：$A$ 管*振幅*（纵向拉伸，值域 $[-A, A]$），$omega$ 管*周期*，$phi$ 管*左右平移*。

$ T = (2 pi)/|omega| $ <eq:period>

由 $y = sin x$ 到 $y = A sin(omega x + phi)$ 的变换有两种顺序：

- 先平移后伸缩：向左平移 $phi/omega$（若 $phi > 0$）再横坐标变为 $1/omega$；
- 先伸缩后平移：横坐标变为 $1/omega$ 后，再平移的量是 $phi/omega$ 而非 $phi$。

> 易错点：平移量要看 $x$ 的系数。$y = sin 2 x$ 到 $y = sin(2 x + pi/3)$ 是向左平移 $pi/6$，不是 $pi/3$！口诀：*提系数、再平移*。

由图像求解析式"三看"：看*振幅*定 $A$（峰谷差的一半），看*周期*定 $omega$（$omega = 2 pi/T$），代*最值点*定 $phi$（最值点代入比零点可靠，零点有二义性）。

= 三角恒等变换

== 两角和与差公式

$ sin(alpha plus.minus beta) = sin alpha cos beta plus.minus cos alpha sin beta $ <eq:sin-sum>

$ cos(alpha + beta) = cos alpha cos beta - sin alpha sin beta, quad cos(alpha - beta) = cos alpha cos beta + sin alpha sin beta $

$ tan(alpha + beta) = (tan alpha + tan beta)/(1 - tan alpha tan beta), quad tan(alpha - beta) = (tan alpha - tan beta)/(1 + tan alpha tan beta) $

记忆口诀：正弦展开是"*正余余正、符号照抄*"；余弦展开是"*余余正正、符号变号*"（$cos(alpha + beta)$ 中间是减号）。

== 二倍角公式（降幂公式的源头）

$ sin 2 alpha = 2 sin alpha cos alpha $ <eq:sin-double>

$ cos 2 alpha = cos^2 alpha - sin^2 alpha = 2 cos^2 alpha - 1 = 1 - 2 sin^2 alpha $ <eq:cos-double>

$cos 2 alpha$ 的三种写法是降幂公式的来源，由后两种反解即得*降幂公式*：

$ cos^2 alpha = (1 + cos 2 alpha)/2, quad sin^2 alpha = (1 - cos 2 alpha)/2 $

降幂是为了"降次升角"，升幂是为了"升次降角"，恒等变换的第一直觉就是朝目标角的*单角单次*凑。

== 辅助角公式：异名化同名

$ a sin x + b cos x = sqrt(a^2 + b^2) sin(x + phi), quad tan phi = b/a space (a != 0) $ <eq:aux>

推导即"逆用"两角和正弦公式：提出 $sqrt(a^2 + b^2)$ 后，令 $cos phi = a/sqrt(a^2 + b^2)$、$sin phi = b/sqrt(a^2 + b^2)$ 即可。$phi$ 所在象限由 $(a, b)$ 的符号决定。

> 考场速记：$sin x plus.minus cos x = sqrt(2) sin(x plus.minus pi/4)$；$sin x plus.minus sqrt(3) cos x = 2 sin(x plus.minus pi/3)$。辅助角公式是求最值、求单调区间的第一把钥匙——先把式子化成 $y = A sin(omega x + phi) + B$ 的单名函数，一切性质照表抄。
