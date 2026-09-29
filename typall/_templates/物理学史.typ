// 物理学史系列模板 —— `typall new --series 物理学史 "文章名"` 使用。
// 占位符：{{title}} {{date}} {{series}}；series_weight 手动调整后删除本行注释。
#let title = "{{title}}"
#let date = "{{date}}"
#let tags = ("物理学史",)
#let series = "{{series}}"
#let series_weight = 0
#let draft = false

#import "../assets/preview.typ": fig, eq-numbering
#import "../assets/figkit.typ": *
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= {{title}}
