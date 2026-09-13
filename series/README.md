# 初高中数理衔接图解系列（Typst + CeTZ）

54 篇系列文章的写作工程：统一模板、统一配图组件库、统一编译校验。
每篇 2000—3000 字、3—5 张 CeTZ 源码绘制配图，六段固定结构。

## 目录结构

```text
series/
  template.typ        # 文章模板：页面/字体/六段小节/图注/盒子组件
  styles.typ          # 统一颜色 + 9 大 CeTZ 组件库
  preview.typ         # 全部配图一页纵览（工程校验）
  _smoke.typ          # 组件冒烟测试
  figures/            # 配图（每张一个 .typ，导出 fig / fig-a / fig-b …）
  articles/           # 文章正文
  build/              # 编译产物（PDF / PNG，不入库）
```

## 快速开始

```bash
# 编译一篇文章
typst compile --root . articles/s1-01-why-drop.typ build/s1-01.pdf

# 文章转 PNG（逐页）
typst compile --root . --format png --ppi 100 articles/s1-01-why-drop.typ build/s1-01-{p}.png

# 预览全部配图 / 冒烟测试
typst compile --root . preview.typ build/preview.pdf
typst compile --root . _smoke.typ build/_smoke.pdf
```

依赖：typst ≥ 0.14（仓库环境 0.15.1），`@preview/cetz:0.5.2`（本地包缓存已就绪）。

## 写作规范

### 文章骨架（六段固定）

```typst
#import "../template.typ": *
#import "../figures/s1-fig-xx.typ": fig as fig-xx

#show: article.with(
  [文章标题],
  season: "第一季 · 初高衔接",   // 第一季 · 初高衔接 / 第二季 · 高中深化 / 第三季 · 压轴与先修
  module: "总览诊断 · 01 / 18",
  minutes: 12, words: "约 2600 字",
)

#section("痛点引子", "小标题")   // 一个真实学习场景
#section("知识地图", "小标题")   // 初中已学 → 高中要求 → 断层在哪（可用 gap-table）
#section("核心讲解", "小标题")   // 只讲 1—2 个核心概念，公式用 Typst 数学模式
#section("例题拆解", "小标题")   // 数学用函数图/几何图；物理用受力图/实验图（problem-box）
#section("思维方法", "小标题")   // 可迁移方法（method-box）
#section("行动清单", "今天就能做") // action-box，恰好 3 条
```

模板组件：`section(标签, 标题)`、`figure-block(编号, 图注, 图)`、
`gap-table(三行一组)`、`problem-box(题干)`、`method-box(卡片)`、`action-box(3 条)`。

### 配图规范

- 文件命名：`s{季}-fig-{篇号}[-b|-c][-后缀].typ`；每文件导出 `fig`（组合图）与 `fig-a/fig-b`（分面板）。
- 画布统一 `cetz.canvas(length: 0.8—1cm)`，坐标单位 ≈ cm；文字 7—9.5pt。
- 统一颜色：`math-blue #2B6CB0`、`phys-orange #DD6B20`、`warn-red #E53E3E`、`gray-line #4A5568`，
  辅助色 `ink / green-ok / soft-*`（见 styles.typ）。
- 组件（styles.typ，需在 canvas 闭包内调用）：
  `axes()` 坐标轴与刻度（0 点不在范围内时自动钳到边界）、`vector()` 矢量、
  `node()` 节点、`arrow()` 连线、`circuit-battery/resistor/switch/lamp/meter()` 电路、
  `field-line()/charge()/b-into()/b-out()` 场线、`conic()` 圆锥曲线、
  `error-bar()/fit-line()/scatter-error()` 误差与拟合、`timeline()` 时间轴；
  辅助 `samples()/curve()/fnum()`。
- 坑位备忘：
  - 组件内部只用 `import cetz.draw: line, content, rect, circle`，避免通配导入
    遮蔽参数（cetz 的 `fill/stroke/mark/scale/grid` 是函数）；
  - 箭头 mark 写法：`mark: (end: ">>", fill: paint, scale: ..)`（0.5.x 没有 `"arrow"`）；
  - 自由多边形用 `line(..pts, close: true)`（`polygon` 是正多边形生成器）；
  - 偏移小图内的曲线要手动平移到小图中心（`axes` 只负责轴钳制）；
  - 渐近线用 `skip: (a, b)` 分段采样；
  - 文章正文避免「长行内公式 + 紧跟标点」（易产生行首标点），公式后接普通汉字。

## 54 篇总计划与进度

状态：✅ 全部 54 篇完成（2026-09-12）｜🖼 文章未写但配图已备（已全部完成）

### 第一季 · 初高衔接（18 篇）

| 篇 | 标题 | 配图 | 状态 |
|---|---|---|---|
| 01 | 初中数理好，高中为何掉队？ | s1-fig-01-difficulty / 01b-matrix / 01c-ladder | ✅ |
| 02 | 一张地图看懂初高数理知识树 | s1-fig-02-knowledge-tree / 02b-bridges / 02c-route | ✅ |
| 03 | 你卡在哪一层？数理能力诊断 | s1-fig-03-diagnosis（五维雷达 / 能力阶梯 / 诊断流程） | ✅ |
| 04 | 从算术到代数：数系扩充与运算律 | s1-fig-04-number-system（数系同心圈 / 运算律对照） | ✅ |
| 05 | 函数思想：初高中最大的分水岭 | s1-fig-05-functions（函数家族 / 图像变换） | ✅ |
| 06 | 几何进阶：从直观到论证 | s1-fig-06-geometry（辅助线 / 证明链 / 坐标系） | ✅ |
| 07 | 方程、不等式与建模 | s1-fig-07-equation（交点 / 数轴区间 / 天平） | ✅ |
| 08 | 概率统计与数据处理 | s1-fig-08-probability（概率树 / 直方图 / 正态） | ✅ |
| 09 | 从现象到模型：质点、参考系、矢量 | s1-fig-09-model（抽象图 / 矢量合成 / 参考系） | ✅ |
| 10 | 力学主线：受力分析与牛顿定律 | s1-fig-10-forces（斜面受力 / 连接体） | ✅ |
| 11 | 能量与动量：守恒思想 | s1-fig-11-energy（能量流 / 碰撞矢量） | ✅ |
| 12 | 电与磁：从电路到场 | s1-fig-12-electromagnetism（电路 / 电场线 / 磁感线） | ✅ |
| 13 | 物理实验与图像处理 | s1-fig-13-experiment（纸带 / v-t / 拟合线） | ✅ |
| 14 | 数理共通思想 | s1-fig-14-methods（方法迁移图 / 数形结合） | ✅ |
| 15 | 学习闭环 | s1-fig-15-learning-loop（闭环 / 错题四象限 / 决策树） | ✅ |
| 16 | 考试与心态 | s1-fig-16-exam（时间饼图 / 心态曲线 / 取舍树） | ✅ |
| 17 | 工具与资源：Typst、CeTZ、仿真 | s1-fig-17-tools（工具矩阵 / 工程结构 / 绘图流程） | ✅ |
| 18 | 家长支持与三年路线图 | s1-fig-18-roadmap（三年时间轴 / 沟通卡 / 规划表） | ✅ |

### 第二季 · 高中深化与高考衔接（18 篇）

| 篇 | 标题 | 配图 | 状态 |
|---|---|---|---|
| 01 | 导数与函数：从变化率到单调性 | s2-fig-01-derivative / 01b-derivative-graph / 01c-sign-table | ✅ |
| 02 | 三角与向量 | s2-fig-02-trig-vector（单位圆 / 正弦曲线 / 矢量合成） | ✅ |
| 03 | 数列与归纳 | s2-fig-03-sequence（点列 / 递推树 / 求和面积） | ✅ |
| 04 | 不等式与最值 | s2-fig-04-inequality（可行域 / 均值几何 / 柯西投影） | ✅ |
| 05 | 立体几何与空间向量 | s2-fig-05-solid（空间坐标 / 棱柱棱锥 / 空间向量） | ✅ |
| 06 | 解析几何：直线、圆与圆锥曲线 | s2-fig-06-conic（圆锥曲线 / 焦点准线 / 交点） | ✅ |
| 07 | 概率统计进阶 | s2-fig-07-stat（正态 / 散点回归 / 列联表） | ✅ |
| 08 | 运动学进阶：图像、追及与相对运动 | s2-fig-08-kinematics（三图联动 / 追及） | ✅ |
| 09 | 动力学综合：连接体、临界、板块与传送带 | s2-fig-09-dynamics（连接体 / 板块 / 传送带） | ✅ |
| 10 | 曲线运动与万有引力 | s2-fig-10-curve-gravity（平抛 / 圆周 / 卫星轨道） | ✅ |
| 11 | 功与能 | s2-fig-11-energy（能量流 / 动能定理面积 / 势能曲线） | ✅ |
| 12 | 动量与碰撞 | s2-fig-12-momentum（碰撞矢量 / 冲量面积 / 恢复系数） | ✅ |
| 13 | 静电场与恒定电流 | s2-fig-13-electro（电场线 / 等势面 / 电路图） | ✅ |
| 14 | 磁场与电磁感应 | s2-fig-14-magnetism（磁感线 / 洛伦兹圆周 / 导轨） | ✅ |
| 15 | 数理方法进阶：微元、极限、对称、守恒、量纲 | s2-fig-15-methods（微元 / 量纲树 / 对称守恒） | ✅ |
| 16 | 实验设计与误差分析 | s2-fig-16-experiment（误差棒 / 拟合线 / 传感器） | ✅ |
| 17 | 高考专题：选择、实验、计算与压轴拆解 | s2-fig-17-gaokao（时间分配 / 决策树 / 压轴拆解） | ✅ |
| 18 | 强基/竞赛启蒙与项目式学习 | s2-fig-18-project（微积分初步 / 项目流程） | ✅ |

### 第三季 · 压轴、强基与大学先修（18 篇）

| 篇 | 标题 | 配图 | 状态 |
|---|---|---|---|
| 01 | 导数压轴：零点、极值、不等式证明 | s3-fig-01-derivative-final / 01b-zero-existence / 01c-tangent-bound | ✅ |
| 02 | 圆锥曲线压轴：定点定值、最值、轨迹 | s3-fig-02-conic-final（函数族 / 焦点弦 / 定点定值） | ✅ |
| 03 | 数列与不等式压轴 | s3-fig-03-sequence-final（递推网络 / 放缩 / 求和界） | ✅ |
| 04 | 概率统计压轴：随机过程、决策与建模 | s3-fig-04-prob-final（状态转移 / 决策树 / 期望曲线） | ✅ |
| 05 | 力学压轴：多过程、临界、能量动量综合 | s3-fig-05-mechanics-final（多过程轨迹 / 能量流） | ✅ |
| 06 | 电磁感应压轴：双杆、电容、图像、能量 | s3-fig-06-induction-final（双杆 / 感应电流 / 能量流） | ✅ |
| 07 | 热学与近代物理 | s3-fig-07-modern（分子运动 / p-V / 光电效应） | ✅ |
| 08 | 振动与波 | s3-fig-08-wave（简谐 / 波形 / 干涉条纹） | ✅ |
| 09 | 光学 | s3-fig-09-optics（光路 / 全反射 / 干涉衍射） | ✅ |
| 10 | 实验压轴 | s3-fig-10-experiment-final（装置 / 误差棒 / 拟合线） | ✅ |
| 11 | 强基数学 | s3-fig-11-strong-math（微积分图 / 矩阵变换 / 组合树） | ✅ |
| 12 | 强基物理 | s3-fig-12-strong-phys（微元 / 刚体 / 电磁场） | ✅ |
| 13 | 数学建模 | s3-fig-13-modeling（流程 / 迭代 / 验证） | ✅ |
| 14 | 物理建模与仿真 | s3-fig-14-simulation（仿真流程 / 数值结果 / 动画分帧） | ✅ |
| 15 | 数据分析与统计建模 | s3-fig-15-data（回归 / 分类边界 / 聚类） | ✅ |
| 16 | Typst + CeTZ 高级绘图 | s3-fig-16-advanced-cetz（三维 / 分帧 / 交互组件） | ✅ |
| 17 | 课题研究与项目式学习 | s3-fig-17-project（流程 / 里程碑 / 成果树） | ✅ |
| 18 | 大学先修与终身学习 | s3-fig-18-university（知识体系 / 学习路径） | ✅ |

## 续写一篇的操作流程

1. `figures/` 里按命名规范新增 3 张配图（复用 styles 组件，能不手算坐标就算）；
2. 把新图加进 `preview.typ`，`typst compile preview.typ` 校验；
3. `articles/` 新建 `s{季}-{篇}-{slug}.typ`，套六段骨架，每段 300—500 字；
4. 编译 → `--format png` 逐页渲染 → 自查：行首标点、孤行标题、图字重叠、物理方向；
5. 更新本 README 的状态列。
