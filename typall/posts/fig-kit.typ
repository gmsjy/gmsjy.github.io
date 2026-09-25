#let title = "figkit 插图模板库：理科常用图一键生成"
#let date = "2026-09-19"
#let tags = ("工具", "绘图", "Typst", "高中")
#let series = "教学工具"
#let series_weight = 2
#let draft = false

#import "../assets/preview.typ": eq-numbering
#import "../assets/figkit.typ": *
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

写文章时最费时间的不是文字，是图。这个站已有 151 张成品图和 33 个图元（`series-styles.typ`），但「图元要自己组装、成品图不能复用」，中间缺一层*常用图型模板*。`assets/figkit.typ` 补上这一层：一行导入，六类理科最常用的图各一个函数，参数填多少画多少，默认值全部就位。

#import "../assets/figkit.typ": *
#plot(-1, 5, -1, 4, curves: ((f: x => x * x, x0: 0, x1: 2.1, color: phys-orange)))

上面这行就是全部用法：`plot` 一个调用，坐标系、曲线、颜色梯度都自动就位。下面逐个过六类模板，每个都是「何时用 + 最小示例 + 常用参数」。

= plot：函数曲线图

*何时用*：v–t 图、速率—浓度曲线、滴定曲线、酶活性钟形、J/S 型增长、能量变化图——一切「坐标系 + 曲线 + 注记」的图。

#block(fill: rgb("#f7f5ef"), inset: 10pt, radius: 6pt, width: 100%)[
```typst
#plot(-0.5, 7, 0, 3.4,
  // 分段函数：突变图（v–t 的经典形状）
  curves: ((
    pieces: ((x => 1.6, 0.3, 2.2), (x => 1.6 + (x - 2.2) * 10, 2.2, 2.28),
             (x => 2.4, 2.28, 6.6)),
    color: math-blue,
  ), (
    f: x => 1.6 + (x - 2.35) * 0.32, x0: 2.28, x1: 6.6,
    color: phys-orange, dash: "dashed", label: [$v'_"逆"$], label-at: (6.6, 2.75),
  )),
  v-guides: ((2.2, [$t_1$])),
  marks: ((at: (2.2, 1.6), t: [平衡被打破], anchor: "south-west")),
  x-label: $t$, y-label: $v$,
)
```
]

#plot(-0.5, 7, 0, 3.4,
  curves: ((
    pieces: ((x => 1.6, 0.3, 2.2), (x => 1.6 + (x - 2.2) * 10, 2.2, 2.28),
             (x => 2.4, 2.28, 6.6)),
    color: math-blue,
  ), (
    f: x => 1.6 + (x - 2.35) * 0.32, x0: 2.28, x1: 6.6,
    color: phys-orange, dash: "dashed", label: [$v'_"逆"$], label-at: (6.6, 2.75),
  )),
  v-guides: ((2.2, [$t_1$])),
  marks: ((at: (2.2, 1.6), t: [平衡被打破], anchor: "south-west")),
  x-label: $t$, y-label: $v$,
)

要点：

- `curves` 每条一个字典，三种形态任选：`f`（函数段）、`pts`（数据折线 `((x, y), ...)`）、`pieces`（分段，突变图用）；
- `h-guides / v-guides` 画虚参考线（平衡线、$K$ 值线、饱和点），`marks` 打点加标注；
- 曲线颜色默认 `math-blue`，可传 `color: phys-orange`、`dash: "dashed"`。

= flow：流程框图

*何时用*：分泌蛋白运输、光反应暗反应衔接、实验操作流程、解题步骤——一切「框 + 箭头」的图。*连线自动缩短到框边*，不必手算起止坐标。

#block(fill: rgb("#f7f5ef"), inset: 10pt, radius: 6pt, width: 100%)[
```typst
#flow((
  (x: 0, y: 2.2,  t: [核糖体#linebreak()合成肽链], fill: soft-blue,  st: math-blue),
  (x: 0, y: 0.7,  t: [内质网#linebreak()加工折叠], fill: soft-green, st: green-ok),
  (x: 0, y: -0.8, t: [高尔基体#linebreak()分类包装], fill: soft-orange, st: phys-orange),
  (x: 0, y: -2.3, t: [细胞膜#linebreak()胞吐分泌], fill: soft-red,   st: warn-red),
), links: (
  (a: 0, b: 1, label: [囊泡]), (a: 1, b: 2, label: [囊泡]), (a: 2, b: 3, label: [融合]),
))
```
]

#flow((
  (x: 0, y: 2.2,  t: [核糖体#linebreak()合成肽链], fill: soft-blue,  st: math-blue),
  (x: 0, y: 0.7,  t: [内质网#linebreak()加工折叠], fill: soft-green, st: green-ok),
  (x: 0, y: -0.8, t: [高尔基体#linebreak()分类包装], fill: soft-orange, st: phys-orange),
  (x: 0, y: -2.3, t: [细胞膜#linebreak()胞吐分泌], fill: soft-red,   st: warn-red),
), links: (
  (a: 0, b: 1, label: [囊泡]), (a: 1, b: 2, label: [囊泡]), (a: 2, b: 3, label: [融合]),
))

要点：节点是 `(x, y, t: [...], fill:, st:, w:, h:)`，坐标即中心；连线 `(a: , b: , label: , dash: )` 用节点下标；配色直接用 `soft-*` 底 + 对应描边色的梯度。

= cycle：转化三角与关系环

*何时用*：铝三角、铁三角、位—构—性、氮循环、碳循环——n 个概念两两转化。

#block(fill: rgb("#f7f5ef"), inset: 10pt, radius: 6pt, width: 100%)[
```typst
#cycle(([$"Al"^(3+)$], [$"Al"("OH")_3$], [$"AlO"_2^(-)$]), edges: (
  (0, 1, label: [加适量氨水]),
  (1, 0, label: [加强酸]),
  (1, 2, label: [加强碱]),
  (2, 1, label: [通足量 $"CO"_2$]),
  (2, 0, label: [过量强酸＋过量强碱], dash: "dashed"),
))
```
]

#cycle(([$"Al"^(3+)$], [$"Al"("OH")_3$], [$"AlO"_2^(-)$]), edges: (
  (a: 0, b: 1, label: [加适量氨水]),
  (a: 1, b: 0, label: [加强酸]),
  (a: 1, b: 2, label: [加强碱]),
  (a: 2, b: 1, label: [通足量 $"CO"_2$]),
  (a: 2, b: 0, label: [先沉淀后溶解], dash: "dashed"),
))

要点：`labels` 给 n 个节点（首节点在顶部，正多边形摆位）；`edges` 是 `(起点, 终点, label:)` 下标对；`radius: auto` 按节点数自动给（也可手动调）。

= card：全景卡

*何时用*：一类知识的「常客清单」——六个临界条件、常用检验试剂、易错点集锦。一张卡代替一节罗列。

#block(fill: rgb("#f7f5ef"), inset: 10pt, radius: 6pt, width: 100%)[
```typst
#card([常见离子的检验], (
  (t: [$"Cl"^(-)$：稀硝酸酸化后加 $"AgNO"_3$，白色沉淀], fill: soft-blue, st: math-blue),
  (t: [$"SO"_4^(2-)$：足量盐酸无沉淀，再加 $"BaCl"_2$ 白沉], fill: soft-blue, st: math-blue),
  (t: [$"CO"_3^(2-)$：稀盐酸产气，石灰水变浑], fill: soft-green, st: green-ok),
  (t: [$"NH"_4^+$：浓碱加热，湿润红石蕊变蓝], fill: soft-green, st: green-ok),
  (t: [$"Fe"^(3+)$：$"KSCN"$ 变血红色], fill: soft-orange, st: phys-orange),
  (t: [$"Fe"^(2+)$：先加 $"KSCN"$ 不红，加氯水变红], fill: soft-orange, st: phys-orange),
), columns: 2, footnote: [先排除干扰，再做特征反应——检验的通用逻辑])
```
]

#card([常见离子的检验], (
  (t: [$"Cl"^(-)$：稀硝酸酸化后加 $"AgNO"_3$，白色沉淀], fill: soft-blue, st: math-blue),
  (t: [$"SO"_4^(2-)$：足量盐酸无沉淀，再加 $"BaCl"_2$ 白沉], fill: soft-blue, st: math-blue),
  (t: [$"CO"_3^(2-)$：稀盐酸产气，石灰水变浑], fill: soft-green, st: green-ok),
  (t: [$"NH"_4^+$：浓碱加热，湿润红石蕊变蓝], fill: soft-green, st: green-ok),
  (t: [$"Fe"^(3+)$：$"KSCN"$ 变血红色], fill: soft-orange, st: phys-orange),
  (t: [$"Fe"^(2+)$：先加 $"KSCN"$ 不红，加氯水变红], fill: soft-orange, st: phys-orange),
), columns: 2, footnote: [先排除干扰，再做特征反应——检验的通用逻辑])

要点：`items` 可以是字典（自选配色）也可以直接 `[文字]`（默认蓝系）；`columns` 控制每行几个，不足一行的自动居中。

= number-line：数轴与区间

*何时用*：集合的表示、不等式解集、函数定义域。

#block(fill: rgb("#f7f5ef"), inset: 10pt, radius: 6pt, width: 100%)[
```typst
#number-line(-4, 4, step: 1, intervals: (
  ((x: -1, open: true), (x: 3)),      // (-1, 3]：左开右闭
  ((x: none), (x: 3.8)),              // 射线 (-∞, 3.8]
), points: ((2.5, "solid"), (-3, "open")))
```
]

#number-line(-4, 4, step: 1, intervals: (
  ((x: -1, open: true), (x: 3)),
  ((x: none), (x: 3.8)),
), points: ((2.5, "solid"), (-3, "open")))

要点：区间端点 `(x: 值, open: true)` 控制空心实心，`(x: none)` 表示射线；`points` 单独打点。

= raw + 零件：装置图

*何时用*：原电池、电解池、气体制备——装置千变万化，不做成整图模板，而是给*零件*：`beaker`（烧杯，可带液面）、`electrode`（电极板）、`salt-bridge`（盐桥）、`gas-arrow`（气流箭头），在 `raw()` 的空画布里自由组装。

#block(fill: rgb("#f7f5ef"), inset: 10pt, radius: 6pt, width: 100%)[
```typst
#raw(size: 0.7cm, {
  beaker(-1.4, -1.8, 2.4, 2.3, liquid: 0.6)      // 左杯：ZnSO₄
  beaker(2.6, -1.8, 2.4, 2.3, liquid: 0.6)       // 右杯：CuSO₄
  electrode(-0.5, 0.3, -1.4, paint: gray-line, label: [Zn（负极）], label-off: (-0.1, 0.42))
  electrode(4.1, 0.3, -1.4, paint: rgb("#C8823A"), label: [Cu（正极）], label-off: (0.1, 0.42))
  salt-bridge(-1.4, 5.0, 1.15, -0.9)             // 两端浸入液面、中段高于杯口
  // 导线与电子流向
  import cetz.draw: line, circle, content
  line((-0.5, 0.3), (-0.5, 2.75), (4.1, 2.75), (4.1, 0.3), stroke: (paint: ink, thickness: 0.9pt))
  circle((1.8, 2.75), radius: 0.28, stroke: ink)
  content((1.8, 2.75), [A], size: 8pt)
  line((0.35, 2.75), (1.05, 2.75), mark: (end: ">"), stroke: (paint: warn-red, thickness: 1.2pt))
  content((0.7, 3.15), [$e^(-)$], size: 8pt)
})
```
]

#raw(size: 0.7cm, {
  beaker(-1.4, -1.8, 2.4, 2.3, liquid: 0.6)      // 左杯：ZnSO₄
  beaker(2.6, -1.8, 2.4, 2.3, liquid: 0.6)       // 右杯：CuSO₄
  electrode(-0.5, 0.3, -1.4, paint: gray-line, label: [Zn（负极）], label-off: (-0.1, 0.42))
  electrode(4.1, 0.3, -1.4, paint: rgb("#C8823A"), label: [Cu（正极）], label-off: (0.1, 0.42))
  salt-bridge(-1.4, 5.0, 1.15, -0.9)             // 两端浸入液面、中段高于杯口
  import cetz.draw: line, circle, content
  line((-0.5, 0.3), (-0.5, 2.75), (4.1, 2.75), (4.1, 0.3), stroke: (paint: ink, thickness: 0.9pt))
  circle((1.8, 2.75), radius: 0.28, stroke: ink)
  content((1.8, 2.75), [A], size: 8pt)
  line((0.35, 2.75), (1.05, 2.75), mark: (end: ">"), stroke: (paint: warn-red, thickness: 1.2pt))
  content((0.7, 3.15), [$e^(-)$], size: 8pt)
  content((-0.5, -2.35), [$"ZnSO"_4$], size: 7.5pt)
  content((4.1, -2.35), [$"CuSO"_4$], size: 7.5pt)
})

零件之外的一切（导线、电表、标签）直接用 CeTZ 原语和 `series-styles` 的图元（`vector / arrow / node`、`circuit-*` 系列）——模板管高频八成，原语兜底剩下两成。

= 与既有体系的关系

- *颜色*：全部走 `series-styles` 六色 + soft 填充，模板内不出现裸 `rgb`（零件的铜电极色除外，集中在库顶部的扩展色区）；
- *字号线宽*：`sz-title / sz-label / sz-note`、`lw-axis / lw-curve / lw-guide` 六个常量是全库唯一来源，改一处全站统一；
- *双模式*：所有模板内部已走 `preview.fig` 包装，HTML 导出与 PDF/预览自动兼容，文章里不再写那 4 行样板；
- *兜底*：模板覆盖不了的图，退到 `raw()` + 零件 + 图元手搓，不要退到裸 canvas。

> 使用建议：把这篇当目录收藏。写新文章时先问「这图是不是 plot / flow / cycle / card / number-line 之一」——八成是；剩下两成用零件组装。每次手搓出新的通用画法，就提炼一个参数加进 figkit，库会越用越顺手。
