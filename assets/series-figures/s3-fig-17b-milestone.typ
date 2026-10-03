// s3-fig-17b-milestone.typ — 项目里程碑时间轴
#import "../series-styles.typ": *
#import "@preview/cetz:0.5.2"

#let fig = cetz.canvas(length: 1cm, {
  import cetz.draw: line, content
  timeline((
    (1.5, [第 1 周#linebreak()定选题], warn-red),
    (4.3, [第 2—3 周#linebreak()查文献·定方案], math-blue),
    (7.1, [第 4—7 周#linebreak()采集与建模], math-blue),
    (9.9, [第 8—9 周#linebreak()分析·画图], phys-orange),
    (12.7, [第 10 周#linebreak()报告与答辩], phys-orange),
  ), x0: 0.6, x1: 13.5, alt: 1.2, label-size: 8.5pt)
  content((6.9, 3.4), text(size: 9pt, fill: ink, weight: "bold", align(center, [十周一个完整研究项目——每周投入 2—3 小时即可])))
  content((6.9, -1.8), text(size: 8.5pt, fill: warn-red, weight: "bold", align(center, [最大的风险不是做不完，而是第 1 周选题贪大——#linebreak()「缩小范围」是项目管理的第一课])))
})
