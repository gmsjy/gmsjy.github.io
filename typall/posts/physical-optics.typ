#let title = "物理光学：干涉、衍射与光的波动本色"
#let date = "2026-09-04"
#let tags = ("物理", "光学", "高中")
#let series = "高中物理"
#let series_weight = 17
#let draft = false

#import "@preview/cetz:0.5.2"
#import cetz.draw: *
#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

上一课用「光线」解释了反射折射，但有些现象它无能为力：光绕过障碍物边缘、两束光叠加出明暗条纹——这些是*波*才有的本事。这一课的证据链只有一个目标：证明*光是一种电磁波*。

= 双缝干涉：波动性的铁证

== 什么是干涉

两列*频率相同*、*相位差恒定*、振动方向相同的波叠加，空间中形成*稳定的*加强区与减弱区——干涉。水波可以，声波可以，光若也是波，就应该能做出条纹。

杨氏双缝实验（1801 年）正是这么做的：单缝 $S$ 保证相干性，双缝 $S_1$、$S_2$ 成为两个*相干光源*，屏上出现*等间距的明暗相间条纹*——微粒说完全无法解释。

#fig(cetz.canvas(length: 0.8cm, {
  // 双缝干涉示意：单缝 -> 双缝 -> 屏，波前用圆弧示意
  // 单缝 S
  content((0.6, 1.5), $ S $)
  // 双缝挡板
  line((2.4, 0), (2.4, 1.1), stroke: (paint: black, thickness: 2pt))
  line((2.4, 1.9), (2.4, 2.1), stroke: (paint: black, thickness: 2pt))
  line((2.4, 2.9), (2.4, 3), stroke: (paint: black, thickness: 2pt))
  content((2.15, 1.5), $ S_1 $, anchor: "east", size: 8pt)
  content((2.15, 0.55), $ S_2 $, anchor: "east", size: 8pt)
  // 波传播弧线（示意）：从 S 到双缝的弧 + 双缝后的弧
  for r in range(6, 15,step: 3) {
    let rr = r / 10.0
    let pts = range(0, 21,step: 2).map(k => {
      let a = 40 * k / 10.0 - 20
      (2.4 - rr * calc.cos(a * calc.pi / 180), 1.5 + rr * calc.sin(a * calc.pi / 180))
    })
    line(..pts, stroke: rgb("#9aa2ad"))
  }
  for r in range(5, 22,step: 4) {
    let rr = r / 10.0
    let pts = range(0, 13,step: 2).map(k => {
      let a = 40 * k / 10.0 - 20
      (2.4 + rr * calc.cos(a * calc.pi / 180), 1.5 + rr * calc.sin(a * calc.pi / 180))
    })
    line(..pts, stroke: rgb("#9aa2ad"))
  }
  // 屏：中央亮纹 + 两侧明暗纹
  line((7.6, 0), (7.6, 3), stroke: (paint: black, thickness: 1.5pt))
  let ys = (1.5, 1.06, 0.62, 1.94, 2.38, 0.18, 2.82)
  for y in ys {
    let w = 0.16
    line((7.5, y - w), (7.5, y + w), stroke: (paint: rgb("#d64541"), thickness: 2.5pt))
  }
  content((8.0, 1.5), "亮", anchor: "west", size: 7.5pt)
  content((8.0, 0.62), "暗", anchor: "west", size: 7.5pt)
}))

== 条纹间距公式

$ Delta y = lambda L / d $ <eq:fringe>

- $lambda$：光的波长（数百纳米量级）；
- $L$：双缝到屏的距离；
- $d$：双缝间距。

*红光条纹比紫光宽*（$lambda_"红" > lambda_"紫"$），白光入射时中央为白纹、两侧出现*彩色*条纹——紫光在内红光在外。

> 记忆点：判断屏上某点是亮是暗，看*路程差* $Delta = |P S_1 - P S_2|$：$Delta = k lambda$（波长的整数倍）→ 加强 → 亮纹；$Delta = (2k+1) lambda\/2$（半波长奇数倍）→ 抵消 → 暗纹。公式 @eq:fringe 就是从几何关系推出来的。

= 薄膜干涉：皂膜上的彩色

光在透明薄膜的*前表面*和*后表面*分别反射，两列反射波叠加发生干涉。皂膜、水面油膜上的彩色纹路，照相机镜头的增透膜，都是薄膜干涉。

- *厚度不均匀*（皂膜）→ 各处路程差不同 → 彩色条纹随厚度分布，重力使皂膜上薄下厚，条纹*水平*；
- *增透膜*：厚度取 $lambda \/ (4n)$，两反射光路程差恰为半波长，*相消*——反射减弱即透射增强；「绿镜头」正是因为膜厚按*绿光*（人眼最敏感）设计。

= 衍射：光会「绕弯」

光*偏离直线传播*绕过障碍物的现象。发生明显衍射的条件：*障碍物或孔的尺寸与波长相当或更小*。

- *单缝衍射*：中央亮纹*宽而亮*，两侧条纹*变窄变暗*（注意与双缝「等间距等亮度」的区别）；
- *圆孔衍射*：中央亮斑（艾里斑）+ 同心圆环；
- *泊松亮斑*：光照射不透明圆盘，影子的*正中央出现亮斑*——波动理论最漂亮的预言。

#fig(cetz.canvas(length: 0.85cm, {
  // 单缝衍射强度分布：中央主极大宽，次极大迅速衰减
  // 横轴（屏位置）
  line((0, 0), (9, 0), mark: (end: ">"), stroke: black)
  line((0.5, 0), (0.5, 2.6), stroke: (paint: rgb("#999"), dash: "dashed"))
  content((0.5, 2.8), "中央亮纹", size: 8pt)
  // 主极大钟形（用折线近似）
  let bell = range(0, 21,step: 1).map(k => {
    let x = 0.5 + k / 10.0
    let t = (x - 1.6) / 1.1
    (x, 2.5 * calc.exp(-t * t * 2.2))
  })
  line(..bell, stroke: (paint: rgb("#d64541"), thickness: 1.5pt))
  // 次极大（小钟形）
  let side = range(0, 11,step: 1).map(k => {
    let x = 4.2 + k / 10.0
    let t = (x - 4.7) / 0.35
    (x, 0.5 * calc.exp(-t * t * 2.2))
  })
  line(..side, stroke: rgb("#d64541"))
  let side2 = side.map(p => (p.at(0) + 2.6, p.at(1)))
  line(..side2, stroke: rgb("#d64541"))
  content((1.6, -0.4), "宽", size: 8pt)
  content((7.3, -0.4), "窄而暗", size: 8pt)
  content((8.9, 0.35), $ x $)
  content((0.1, 2.4), $ I $, anchor: "north-west")
}))

干涉与衍射都证明光是波，两者常同时出现：双缝实验里每条缝自身在*衍射*，两条衍射波再*干涉*。

= 偏振：光是横波

干涉衍射只能证明「是波」，偏振进一步锁定「是*横*波」——纵波（如声波）没有偏振现象。

- 自然光沿垂直传播方向的*所有方向*振动；只保留某一方向振动的光叫*偏振光*；
- 两块偏振片透振方向*平行*时光通过，*垂直*时（正交）光被挡住；
- 手机屏幕前戴偏光太阳镜旋转角度会看到屏幕明暗变化——液晶屏发出的是偏振光；
- 水面、玻璃面的*反射光*部分偏振——这就是偏光太阳镜能压暗眩光的原理。

= 电磁波谱：光是电磁波家族的一员

麦克斯韦预言、赫兹实验证实：电磁波速等于光速 → *光就是电磁波*。按波长从长到短排：

#fig(cetz.canvas(length: 1.05cm, {
  // 电磁波谱条带：波长递减
  let bands = (
    ("无线电波", rgb("#4a7fb5")),
    ("微波", rgb("#5f9bd1")),
    ("红外", rgb("#d6823a")),
    ("可见光", rgb("#7a4bbd")),
    ("紫外", rgb("#3c8a4d")),
    ("X 射线", rgb("#b53d3d")),
    ("γ 射线", rgb("#6b6b6b")),
  )
  let x0 = 0
  for b in bands {
    let x1 = x0 + 2.0
    rect((x0, 1), (x1, 2), fill: b.at(1), stroke: white)
    content(((x0 + x1) / 2, 0.55), b.at(0), size: 8pt)
    x0 = x1
  }
  line((0, 0.05), (14, 0.05), mark: (end: ">"), stroke: black)
  content((0, -0.5), "波长长 ←", anchor: "west", size: 8pt)
  content((14, -0.5), "→ 波长短、频率高", anchor: "east", size: 8pt)
}))

可见光只是很窄的一段（$400 \/ 700 "nm"$）：红外侧重「热效应」（遥控器、热成像），紫外侧重「化学效应」（杀菌、荧光），X 射线穿透力强（医学影像），$gamma$ 射线来自核反应（见原子核物理一课）。

#block(fill: rgb("#f4f1ea"), inset: 12pt, radius: 6pt)[
*例*：双缝间距 $d = 0.2 "mm"$，屏距 $L = 1.0 m$，红光波长 $lambda = 6.0 times 10^-7 m$，求条纹间距。

@eq:fringe：$Delta y = lambda L \/ d = 6.0 times 10^-7 times 1.0 \/ (0.2 times 10^-3) = 3.0 times 10^-3 m = 3 "mm"$。

*单位陷阱*：$d$ 是毫米、$lambda$ 是纳米、$L$ 是米——先全部换成米再代入。
]

= 解题方法论

- *条纹判断两步走*：先问「什么光、什么装置」——双缝等间距、单缝中间宽、白光则彩色；
- *路程差是钥匙*：亮暗判定永远回到 $Delta = k lambda$ 或 $(2k+1) lambda\/2$，公式记不清就现场推；
- *对比记忆*：干涉是「多束相干光叠加」，衍射是「单束光自己绕弯」，偏振锁定「横波」——三个证据的证明力递进；
- *薄膜干涉看厚度*：厚度均匀看颜色（增透膜），厚度不均匀看条纹走向（皂膜水平纹）。

一句话收束：从条纹到偏振，这一课的实验链条环环相扣——*光不仅是波，还是电磁波*。而「光究竟是波还是粒子」的完整答案，要到量子物理一课才能揭晓。
