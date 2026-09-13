// 预览：所有配图纵览（工程校验用，单栏流式）
#import "template.typ": *

#set page(paper: "a4", margin: 1.2cm)
#set text(font: ((name: "Libertinus Serif", covers: "latin-in-cjk"), "Microsoft YaHei", "SimHei"), size: 9pt, lang: "zh", fill: ink)

#import "figures/s1-fig-01-difficulty.typ": fig as f101
#import "figures/s1-fig-01b-matrix.typ": fig as f101b
#import "figures/s1-fig-01c-ladder.typ": fig as f101c
#import "figures/s1-fig-02-knowledge-tree.typ": fig as f102
#import "figures/s1-fig-02b-bridges.typ": fig as f102b
#import "figures/s1-fig-02c-route.typ": fig as f102c
#import "figures/s1-fig-05-functions.typ": fig as f105
#import "figures/s1-fig-10-forces.typ": fig as f110
#import "figures/s2-fig-01-derivative.typ": fig as f201
#import "figures/s2-fig-01b-derivative-graph.typ": fig as f201b
#import "figures/s2-fig-01c-sign-table.typ": fig as f201c
#import "figures/s2-fig-08-kinematics.typ": fig as f208
#import "figures/s2-fig-14-magnetism.typ": fig as f214
#import "figures/s3-fig-01-derivative-final.typ": fig as f301
#import "figures/s3-fig-01b-zero-existence.typ": fig as f301b
#import "figures/s3-fig-01c-tangent-bound.typ": fig as f301c
#import "figures/s3-fig-05-mechanics-final.typ": fig as f305

#let cell(body, cap) = block(breakable: false, above: 14pt, below: 4pt)[
  #align(center, body)
  #v(4pt)
  #align(center, text(size: 7.5pt, fill: gray-line, cap))
]

#cell(f101, [1-01 难度三线 + 断层带])
#cell(f101b, [1-01b 四维对比矩阵])
#cell(f101c, [1-01c 能力五级台阶])
#cell(f102, [1-02 双知识树 + 衔接点])
#cell(f102b, [1-02b 三大衔接点])
#cell(f102c, [1-02c 行动路线])
#cell(f105, [1-05 函数家族 + 变换])
#cell(f110, [1-10 受力分析 + 连接体])
#cell(f201, [2-01 割线趋近切线])
#cell(f201b, [2-01b f 与 f' 联动])
#cell(f201c, [2-01c 符号分析表])
#cell(f208, [2-08 三图联动 + 追及])
#cell(f214, [2-14 磁感线 / 洛伦兹力 / 导轨])
#cell(f301, [3-01 含参函数族])
#cell(f301b, [3-01b 零点存在性])
#cell(f301c, [3-01c 切线放缩])
#cell(f305, [3-05 力学多过程 + 能量流])
