// =====================================================================
// series/template.typ — 系列文章统一模板
// 六段固定结构：痛点引子 / 知识地图 / 核心讲解 / 例题拆解 / 思维方法 / 行动清单
// 用法：
//   #import "../template.typ": *
//   #show: article.with(标题, season: "第一季 · 初高衔接", module: "总览诊断", ...)
//   #section("痛点引子", "开头一句提问式标题")
//   ...
// =====================================================================
#import "styles.typ": *

#let series-name = "初高中数理衔接图解"

// ---------- 页面 / 字体 ----------
#let article(title, season: "第一季 · 初高衔接", module: "", minutes: 12, words: "约 2600 字", body) = {
  set page(
    paper: "a4",
    margin: (top: 2.1cm, bottom: 2.2cm, x: 1.9cm),
    footer: align(center, context text(size: 8pt, fill: gray-line)[— #counter(page).display("1") — · #series-name]),
  )
  set text(
    font: ((name: "Libertinus Serif", covers: "latin-in-cjk"), "Microsoft YaHei", "SimHei"),
    size: 10.5pt, lang: "zh", region: "cn", fill: ink,
  )
  set par(justify: true, leading: 0.75em, spacing: 1.05em)
  set list(indent: 0.9em, spacing: 0.7em, body-indent: 0.6em)
  set enum(indent: 0.9em, spacing: 0.7em, body-indent: 0.6em)
  set heading(numbering: none)

  align(center)[
    #text(size: 8.5pt)[
      #box(fill: math-blue, radius: 3pt, inset: (x: 7pt, y: 3pt), text(fill: white, weight: 600, season))
      #h(6pt)
      #text(fill: gray-line, weight: 500, module)
    ]
    #v(0.55em)
    #text(size: 19pt, weight: 800, title)
    #v(0.35em)
    #text(size: 8.5pt, fill: gray-line)[全文字数 #words ｜ 阅读约 #minutes 分钟 ｜ 配图均由 CeTZ 源码绘制]
  ]
  v(0.1em)
  line(length: 26%, stroke: 1.4pt + phys-orange)
  v(0.7em)

  body
}

// ---------- 六段结构小节标题 ----------
#let section(tag, title) = block(above: 1.5em, below: 0.9em, breakable: false, sticky: true)[
  #box(fill: math-blue, radius: 3pt, inset: (x: 6pt, y: 2.5pt), baseline: 30%,
    text(fill: white, size: 8pt, weight: 600, tag))
  #h(8pt)
  #text(size: 13pt, weight: 700, title)
  #v(-0.52em)
  #line(length: 100%, stroke: 0.5pt + gray-line.lighten(55%))
]

// ---------- 插图（编号 + 图注） ----------
#let figure-block(num, caption, body) = block(above: 1.1em, below: 1.2em, breakable: false)[
  #align(center, body)
  #v(0.35em)
  #align(center, text(size: 8.5pt, fill: gray-line, weight: 500)[图 #num　#caption])
]

// ---------- 「知识地图」三栏对照表 ----------
#let gap-table(rows) = table(
  columns: (1fr, 1fr, 1fr),
  align: center + horizon,
  inset: (x: 8pt, y: 6.5pt),
  stroke: none,
  table.hline(stroke: 1pt + math-blue),
  table.header(
    table.cell(fill: math-blue)[#text(fill: white, size: 9.5pt, weight: 700)[初中已学]],
    table.cell(fill: math-blue)[#text(fill: white, size: 9.5pt, weight: 700)[高中要求]],
    table.cell(fill: warn-red)[#text(fill: white, size: 9.5pt, weight: 700)[断层在哪]],
  ),
  ..rows.map(r => (
    table.cell(fill: soft-blue)[#text(size: 9.3pt, r.at(0))],
    table.cell(fill: soft-orange)[#text(size: 9.3pt, r.at(1))],
    table.cell(fill: soft-red)[#text(size: 9.3pt, r.at(2))],
  )).flatten(),
  table.hline(stroke: 1pt + math-blue),
)

// ---------- 盒子组件 ----------
// 例题框
#let problem-box(label, body) = block(above: 1.1em, below: 1.1em, width: 100%,
  fill: soft-gray, stroke: (left: 2.5pt + gray-line),
  inset: (x: 12pt, y: 10pt), radius: (top-right: 4pt, bottom-right: 4pt), breakable: false)[
  #text(size: 10pt, weight: 700, fill: gray-line)[✎ #label]
  #v(0.35em)
  #text(size: 9.8pt, body)
]

// 思维方法框
#let method-box(title, body) = block(above: 1.1em, below: 1.1em, width: 100%,
  fill: soft-orange, stroke: (left: 2.5pt + phys-orange),
  inset: (x: 12pt, y: 10pt), radius: (top-right: 4pt, bottom-right: 4pt), breakable: false)[
  #text(size: 10pt, weight: 700, fill: phys-orange)[⭐ #title]
  #v(0.35em)
  #text(size: 9.8pt, body)
]

// 行动清单（items：已带 ①②③ 的内容数组）
#let action-box(items) = block(above: 1.2em, below: 0.8em, width: 100%,
  fill: soft-blue, stroke: (left: 2.5pt + math-blue),
  inset: (x: 12pt, y: 10pt), radius: (top-right: 4pt, bottom-right: 4pt), breakable: false)[
  #text(size: 10.5pt, weight: 700, fill: math-blue)[✅ 行动清单：今天就能做的 3 件事]
  #v(0.45em)
  #list(..items.map(i => text(size: 9.8pt, i)), spacing: 0.85em)
]
