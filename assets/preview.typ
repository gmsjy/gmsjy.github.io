// 插图与公式编号的「双模式」兼容层。
//
// typall 站点构建（HTML 导出）运行时启用 html 特性；而 VS Code 的 Typst
// 预览（tinymist）、裸 `typst compile`、PDF 导出走原生分页目标，html 模块
// 不存在——词法引用 `html.frame` 会让整篇文档编译失败。本文件让同一份
// 笔记在两种环境下都能编译：
//
// - HTML 导出：`fig` 经 `html.frame` 把 CeTZ 画布渲染为内联 SVG（与直接
//   写 html.frame 完全等效）；公式编号返回 none，编号由构建期 HTML 后
//   处理注入（原生编号在 SVG 渲染器下有 typst#5512 缺陷）。
// - 原生分页：画布由 Typst 直接矢量渲染（VS Code 预览、PDF 均正常）；
//   文章级公式编号启用 "(1)"，使 `@eq:` 引用可用。
//
// `html` 模块的获取走 `eval`：字符串在分页目标下不会被求值，因此文件
// 本身无需 html 特性即可编译。
#let fig(body) = {
  // 画布内的数学标签（$ x $ 等块级公式）不参与公式编号——
  // 否则分页预览/PDF 里轴标签会被编成 (2) (3) (4) 且消耗计数器。
  show math.equation: set math.equation(numbering: none)
  context {
    if target() == "html" {
      eval("html").frame(body)
    } else {
      body
    }
  }
}

// 公式编号回调：numbering 函数在布局期被调用（context 可用）。
// 只应经文章里 `show math.equation.where(block: true): set` 作用于块级公式。
#let eq-numbering(..args) = context {
  if target() == "html" {
    none
  } else {
    numbering("(1)", ..args)
  }
}
