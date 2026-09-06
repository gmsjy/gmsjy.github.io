//! 主题内置默认值：默认样式 / 骨架模板 / 各 partial 片段。
//! 由 theme.rs 经 `pub use` 原样再导出，外部路径不变。

pub const DEFAULT_CSS: &str = r#"
/* ============================================================
   typall · default theme —— 「墨理」杂志编辑风
   设计原则：亮色纸感为主 · 衬线编辑标题 · 克制的青蓝强调 ·
   同时跟随系统提供打磨过的深色夜墨。保留全部站点 class 契约，
   仅替换视觉层，适配 SVG 数学公式与 cetz 插图。
   ============================================================ */

:root {
  /* —— 亮色令牌（日常主视角）—— */
  --bg: #faf9f6;              /* 暖纸底 */
  --surface: #ffffff;         /* 卡片 / 浮层纸面 */
  --surface-2: #f4f1ea;       /* 次级底：代码 / 引用 / tag */
  --fg: #20222a;              /* 墨色正文 */
  --fg-strong: #14161b;       /* 更强强调（标题 hover 等） */
  --muted: #686a72;           /* 次级文本 */
  --faint: #9a9ba3;           /* 提示级文本 */
  --accent: #0b6fc0;          /* 编辑青 */
  --accent-strong: #0a5c9e;   /* hover 深化 */
  --accent-soft: #e7f0f9;     /* 链接/选中浅青底 */
  --quote: #3c5a49;           /* 引用墨绿 */
  --quote-soft: #eef2ec;      /* 引用浅底 */
  --border: #e6e2d6;          /* 暖灰边框 */
  --border-strong: #d3cdbd;   /* 分隔线 / 表格 */
  --code-bg: #f3f1ec;
  --code-ink: #4a4d55;
  --maxw: 52rem;
  --serif: "Iowan Old Style", "Palatino Linotype", Palatino, Georgia,
           "Songti SC", "Noto Serif CJK SC", "Source Han Serif SC",
           "STSong", SimSun, serif;
  --sans: -apple-system, "Segoe UI", "PingFang SC", "Hiragino Sans GB",
          "Microsoft YaHei", "Noto Sans CJK SC", sans-serif;
  --mono: "SFMono-Regular", "Cascadia Code", Consolas, "DejaVu Sans Mono",
          "Noto Sans Mono CJK SC", monospace;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #14151a;            /* 夜墨 */
    --surface: #1b1d23;       /* 卡片 */
    --surface-2: #23252d;     /* 次级底 */
    --fg: #e6e7ec;
    --fg-strong: #f4f5f8;
    --muted: #a0a2ad;
    --faint: #70727d;
    --accent: #4aa3f5;
    --accent-strong: #7dbdf9;
    --accent-soft: #1c2a38;
    --quote: #9fc4ab;
    --quote-soft: #1f2923;
    --border: #2c2e37;
    --border-strong: #3a3c47;
    --code-bg: #23252c;
    --code-ink: #c6c8d2;
  }
}

* { box-sizing: border-box; }
html { -webkit-text-size-adjust: 100%; }
body {
  margin: 0;
  background: var(--bg);
  color: var(--fg);
  font-family: var(--sans);
  line-height: 1.78;
  font-size: 16px;
  font-feature-settings: "kern" 1, "liga" 1;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
}

/* ============ 顶栏 ============ */
.site-header {
  border-bottom: 1px solid var(--border);
  background: color-mix(in srgb, var(--bg) 88%, transparent);
  backdrop-filter: saturate(1.4) blur(6px);
  position: sticky; top: 0; z-index: 20;
}
.site-header .inner {
  max-width: var(--maxw); margin: 0 auto;
  padding: 1.05rem 1.4rem;
  display: flex; align-items: center; justify-content: space-between; gap: 1rem;
}
.site-title {
  font-family: var(--serif);
  font-weight: 600;
  font-size: 1.3rem;
  letter-spacing: 0.01em;
  color: var(--fg-strong);
  text-decoration: none;
  white-space: nowrap;
}
.site-title::after {
  content: "·";
  color: var(--accent);
  margin-left: 0.15em;
  font-weight: 400;
}
.site-title:hover { color: var(--accent); }
.site-nav { display: flex; align-items: center; gap: 1.1rem; flex-wrap: wrap; }
.site-nav a {
  color: var(--muted); text-decoration: none;
  font-size: 0.95rem;
  position: relative;
  transition: color .18s ease;
}
.site-nav a:hover { color: var(--accent); }

/* ============ 主区与页脚 ============ */
main {
  max-width: var(--maxw); margin: 0 auto;
  padding: 2.6rem 1.4rem 4rem;
}
/* 无障碍跳转链接 */
.skip-link {
  position: absolute;
  left: -999px;
  top: 0;
  background: var(--accent);
  color: #fff;
  padding: 0.5rem 1rem;
  border-radius: 0 0 8px 0;
  z-index: 100;
  font-size: 0.85rem;
}
.skip-link:focus { left: 0; }

/* 编辑风页脚 */
.site-footer {
  border-top: 1px solid var(--border);
  color: var(--faint);
  text-align: center;
  padding: 2.4rem 1.4rem 2.8rem;
  font-size: 0.82rem;
  letter-spacing: 0.03em;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.55rem;
}
.site-footer a { color: var(--accent); text-decoration: none; }
.footer-brand {
  font-family: var(--serif);
  font-size: 1.05rem;
  font-weight: 600;
  color: var(--fg-strong);
}
.footer-meta { font-size: 0.8rem; color: var(--faint); line-height: 1.6; }
.accent-dot { color: var(--accent); font-family: var(--serif); }

/* ============ 文章正文：衬线编辑标题 ============ */
article h1, article h2, article h3, article h4 {
  font-family: var(--serif);
  line-height: 1.32;
  color: var(--fg-strong);
  letter-spacing: 0.005em;
}
article h1 {
  font-size: 1.85rem; font-weight: 600;
  margin: 0.5rem 0 1.2rem;
}
article h2 {
  font-size: 1.42rem; font-weight: 600;
  margin: 2.6rem 0 0.9rem;
  padding-bottom: 0.35rem;
  border-bottom: 1px solid var(--border);
}
article h3 {
  font-size: 1.18rem; font-weight: 600;
  margin: 2rem 0 0.7rem;
}
article h4 {
  font-size: 1.02rem; font-weight: 600;
  margin: 1.6rem 0 0.5rem;
  color: var(--accent-strong);
  letter-spacing: 0.02em;
}
article p { margin: 0.95rem 0; }
article a { color: var(--accent); text-decoration: none; }
article a:hover { color: var(--accent-strong); text-decoration: underline; text-underline-offset: 3px; }
article strong { color: var(--fg-strong); font-weight: 600; }
article em { font-family: var(--serif); font-style: italic; }
article img {
  max-width: 100%; height: auto;
  border-radius: 10px;
  box-shadow: 0 1px 2px rgba(20, 22, 27, 0.05);
  display: block; margin: 1.2rem auto;
}

/* —— 引用：墨绿编辑语 —— */
article blockquote {
  margin: 1.4rem 0;
  padding: 0.15rem 0 0.15rem 1.15rem;
  border-left: 3px solid var(--quote);
  color: var(--muted);
  font-family: var(--serif);
  font-style: italic;
  font-size: 1.02rem;
  line-height: 1.72;
}
article blockquote p { margin: 0.45rem 0; }

/* —— 代码 —— */
article pre {
  background: var(--code-bg);
  border: 1px solid var(--border);
  color: var(--code-ink);
  padding: 1rem 1.15rem;
  overflow-x: auto;
  border-radius: 10px;
  font-size: 0.86rem;
  line-height: 1.62;
  font-family: var(--mono);
}
article code { font-family: var(--mono); }
article :not(pre) > code {
  background: var(--surface-2);
  border: 1px solid var(--border);
  color: var(--fg-strong);
  padding: 0.12em 0.4em;
  border-radius: 5px;
  font-size: 0.9em;
  font-family: var(--mono);
}

/* —— 列表 —— */
article ul, article ol { padding-left: 1.5rem; }
article ul { list-style: disc; }
article ul li::marker { color: var(--accent); }
article ol { list-style: decimal; }
article li { margin: 0.42rem 0; }
article li > ul, article li > ol { margin: 0.25rem 0; }

/* —— 表格 —— */
article table {
  border-collapse: collapse;
  width: 100%;
  margin: 1.4rem 0;
  font-size: 0.94rem;
}
article th, article td {
  border: 1px solid var(--border-strong);
  padding: 0.55rem 0.85rem;
  text-align: left;
}
article th {
  background: var(--surface-2);
  font-weight: 600;
  color: var(--fg-strong);
  border-bottom-width: 1.5px;
}
article tr:nth-child(even) td { background: color-mix(in srgb, var(--surface-2) 40%, transparent); }
article hr {
  border: none;
  border-top: 1px solid var(--border);
  margin: 2.2rem 0;
  position: relative;
  text-align: center;
}

/* —— 数学：MathML 兜底 —— */
math { font-size: 1.05em; line-height: 1.5; }
math[display="block"] {
  display: block;
  margin: 1.2rem auto;
  text-align: center;
  min-height: 2.4em;
  overflow-x: auto;
  overflow-y: visible;
}
math[display="block"] mfrac {
  display: inline-block;
  vertical-align: middle;
  padding: 0 0.3em;
  line-height: 1.4;
}
math[display="block"] mfrac > * { text-align: center; }

/* —— 数学：SVG 渲染模式（renderer = "svg"）
   块级公式容器：flex 居中主体，编号绝对定位于右缘居中，还原标准排版。
   样式层不破坏 serve 预览注入的「复制到公众号」CSS-inliner（其只读数据属性）。 */
article div[data-equation="block"] {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 1.6rem 0;
}
article div[data-equation="block"] > svg { flex: none; }
article div[data-equation="block"] > span.eq-num {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  margin: 0;
  font-family: var(--sans);
  font-size: 0.85em;
  color: var(--faint);
  white-space: nowrap;
}

/* ============ 文章列表（首页/集合页） ============ */
.post-list { list-style: none; padding: 0; margin: 0; }
.post-item {
  margin: 0;
  padding: 1.1rem 0;
  border-bottom: 1px solid var(--border);
}
.post-item:last-child { border-bottom: none; }
.post-item a {
  display: block;
  text-decoration: none;
  color: var(--fg);
  transition: transform .15s ease;
}
.post-item a:hover .post-title { color: var(--accent); }
.post-title {
  font-family: var(--serif);
  font-size: 1.32rem;
  font-weight: 600;
  color: var(--fg-strong);
  line-height: 1.4;
  transition: color .15s ease;
}
.post-date {
  display: block;
  color: var(--faint);
  font-size: 0.82rem;
  margin: 0.4rem 0 0;
  letter-spacing: 0.04em;
  font-variant-numeric: tabular-nums;
}
.post-excerpt {
  color: var(--muted);
  font-size: 0.92rem;
  margin: 0.55rem 0 0;
  line-height: 1.7;
}
.post-tags { margin-top: 0.65rem; }

/* —— 标签胶囊 —— */
.post-tags .tag,
.meta .tag {
  display: inline-block;
  background: var(--surface-2);
  border: 1px solid var(--border);
  color: var(--muted);
  border-radius: 999px;
  padding: 0.12em 0.75em;
  font-size: 0.76rem;
  margin-right: 0.4rem;
  letter-spacing: 0.01em;
  transition: all .15s ease;
}
.meta .tag { text-decoration: none; }
.post-tags .tag:hover, .meta .tag:hover {
  color: var(--accent);
  border-color: var(--accent);
  background: var(--accent-soft);
}

/* ============ 集合页面页眉（首页 / 归档 / 标签 / 分类 — main 直子 h1） ============ */
main > h1 {
  font-family: var(--serif);
  font-size: 1.7rem;
  font-weight: 600;
  color: var(--fg-strong);
  margin: 0.2rem 0 1.4rem;
  padding-bottom: 0.55rem;
  display: inline-block;
  border-bottom: 3px solid var(--accent);
}

/* ============ 页面元信息 ============ */
.meta {
  color: var(--muted);
  font-size: 0.85rem;
  margin-bottom: 1.8rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border);
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.4rem 0.2rem;
}
.meta .tag { margin-right: 0.4rem; }
.updated { color: var(--faint); font-size: 0.82rem; }

/* ============ TOC（可选章节目录） ============ */
.toc {
  border: 1px solid var(--border);
  border-radius: 10px;
  background: var(--surface);
  padding: 0.9rem 1.15rem;
  margin: 1.4rem 0;
  font-size: 0.9rem;
}
.toc summary {
  cursor: pointer;
  color: var(--fg-strong);
  font-weight: 600;
  margin-bottom: 0.5rem;
  font-family: var(--serif);
  font-size: 0.95rem;
  list-style: none;
}
.toc summary::-webkit-details-marker { display: none; }
.toc summary::before { content: "▸ "; color: var(--accent); }
.toc[open] summary::before { content: "▾ "; }
.toc-list { list-style: none; padding-left: 0; margin: 0; }
.toc-list li { margin: 0.32rem 0; }
.toc-list a {
  color: var(--fg); text-decoration: none;
  transition: color .15s ease;
}
.toc-list a:hover { color: var(--accent); }
.toc-list .toc-l2 { font-weight: 500; }
.toc-list .toc-l3 { padding-left: 1.2rem; color: var(--muted); font-size: 0.88em; }

/* ============ 上一篇 / 下一篇 ============ */
.post-nav {
  display: flex;
  justify-content: space-between;
  gap: 1.2rem;
  margin-top: 3rem;
  padding-top: 1.4rem;
  border-top: 1px solid var(--border);
}
.post-nav a {
  color: var(--muted);
  text-decoration: none;
  font-size: 0.9rem;
  max-width: 46%;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: color .15s ease;
}
.post-nav a:hover { color: var(--accent); }
.post-nav .next { margin-left: auto; text-align: right; }

/* ============ 专栏内导航 ============ */
.series-nav {
  display: flex;
  align-items: baseline;
  flex-wrap: wrap;
  gap: 0.6rem 1rem;
  margin-top: 1.6rem;
  padding: 0.9rem 1.15rem;
  border: 1px solid var(--border);
  border-radius: 10px;
  background: var(--surface);
}
.series-nav .series-home {
  font-family: var(--serif);
  font-weight: 600;
  color: var(--accent);
  text-decoration: none;
}
.series-nav .series-home:hover { color: var(--accent-strong); }
.series-nav .series-pos {
  color: var(--faint);
  font-size: 0.85rem;
  font-variant-numeric: tabular-nums;
}
.series-nav .series-links {
  display: flex;
  justify-content: space-between;
  flex: 1 1 100%;
  min-width: 0;   /* flex 项 min-width:auto 会让 nowrap 内容撑破容器 */
  gap: 1.2rem;
  margin-top: 0.4rem;
  padding-top: 0.9rem;
  border-top: 1px dashed var(--border);
}
.series-nav .series-links a {
  color: var(--muted);
  text-decoration: none;
  font-size: 0.9rem;
  max-width: 46%;
  min-width: 0;   /* nowrap 下 flex 子项 min-width:auto 会顶掉 max-width，导致手机溢出 */
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: color .15s ease;
}
.series-nav .series-links a:hover { color: var(--accent); }
.series-nav .series-links .next { margin-left: auto; text-align: right; }

/* ============ 分页 ============ */
.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 0.6rem;
  margin-top: 2.6rem;
  font-size: 0.9rem;
}
.pagination a {
  color: var(--muted);
  text-decoration: none;
  padding: 0.42rem 0.9rem;
  border: 1px solid var(--border);
  border-radius: 999px;
  transition: all .15s ease;
}
.pagination a:hover {
  color: var(--accent);
  border-color: var(--accent);
  background: var(--accent-soft);
}
.pagination .current {
  color: var(--fg-strong);
  font-weight: 600;
  padding: 0.42rem 0.9rem;
  font-variant-numeric: tabular-nums;
}

/* ============ 标签云 ============ */
.tag-cloud { display: flex; flex-wrap: wrap; gap: 0.7rem; margin-top: 1.8rem; }
.tag-cloud-item {
  display: inline-flex;
  align-items: center;
  gap: 0.45rem;
  background: var(--surface);
  border: 1px solid var(--border);
  color: var(--fg-strong);
  text-decoration: none;
  border-radius: 999px;
  padding: 0.42rem 0.95rem;
  font-size: 0.9rem;
  font-family: var(--serif);
  transition: all .15s ease;
}
.tag-cloud-item:hover {
  border-color: var(--accent);
  color: var(--accent);
  background: var(--accent-soft);
  transform: translateY(-1px);
}
.tag-cloud-item .count {
  background: var(--accent-soft);
  color: var(--accent);
  border-radius: 999px;
  padding: 0 0.5rem;
  font-size: 0.75rem;
  font-family: var(--sans);
  font-variant-numeric: tabular-nums;
}

/* ============ 搜索 ============ */
.search-input {
  width: 100%;
  max-width: 32rem;
  padding: 0.72rem 1rem;
  font-size: 0.98rem;
  font-family: var(--sans);
  color: var(--fg);
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 10px;
  transition: border-color .15s ease, box-shadow .15s ease;
}
.search-input::placeholder { color: var(--faint); }
.search-input:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-soft);
}
.search-hint { color: var(--faint); font-size: 0.85rem; margin: 0.6rem 0 1.6rem; }

/* ============ 归档页（<main> 直子 h1/h2，正文在 <article> 内不冲突） ============ */
main > h2 {
  font-family: var(--serif);
  color: var(--accent-strong);
  font-size: 1.3rem;
  font-weight: 600;
  margin: 2rem 0 0.6rem;
  border-bottom: 1px solid var(--border);
  padding-bottom: 0.4rem;
  display: flex;
  align-items: baseline;
  gap: 0.6rem;
}

/* ============ 渲染辅助 class（保留兼容） ============ */
.hidden { display: none; }

/* ============ 窄屏适配（≤640px 手机）============ */
@media (max-width: 640px) {
  main { padding: 1.8rem 1rem 3rem; }
  /* 插图 SVG 带固定 pt 宽度，无约束会横向溢出 */
  article svg { max-width: 100%; height: auto; }
  /* 宽表格横向滚动，不撑破版面 */
  article table { display: block; overflow-x: auto; -webkit-overflow-scrolling: touch; }
  article pre { font-size: 0.8rem; }
  /* 块级公式编号改为公式下方右对齐，避免与公式重叠 */
  article div[data-equation="block"] { flex-direction: column; }
  article div[data-equation="block"] > span.eq-num {
    position: static;
    transform: none;
    align-self: flex-end;
    margin-top: 0.2rem;
  }
  article h1 { font-size: 1.6rem; }
  .post-title { font-size: 1.2rem; }
}
"#;


/// 默认 HTML 骨架模板。
///
/// 可用占位符（文本类会被 HTML 转义，HTML 类原样注入）：
/// - 文本类：`{{language}}` `{{full_title}}` `{{site_title}}` `{{site_description}}`
///   `{{site_author}}` `{{site_url}}` `{{title}}`
/// - HTML 类：`{{body}}` `{{math_style}}` `{{nav_extra}}` `{{seo_head}}` `{{analytics}}`
/// - 文章级（仅文章页有值，非文章页清空）：`{{post.title}}` `{{post.date}}`
///   `{{post.updated}}` `{{post.excerpt}}` `{{post.slug}}` `{{post.tags}}`
///   `{{post.categories}}` `{{post.series}}` `{{post.raw.<key>}}`
/// - 主题参数：`{{params.<key>}}`（支持点号访问嵌套，如 `{{params.font.family}}`）
pub const DEFAULT_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang="{{language}}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{{full_title}}</title>
<meta name="description" content="{{site_description}}">
<link rel="stylesheet" href="/assets/style.css">
{{seo_head}}
{{math_style}}
{{analytics}}
</head>
<body>
<a class="skip-link" href="#content">跳到正文</a>
<header class="site-header">
  <div class="inner">
    <a class="site-title" href="/">{{site_title}}</a>
    <nav class="site-nav" aria-label="主导航">
      <a href="/">首页</a>
      {{nav_extra}}
      <a href="/search/">搜索</a>
    </nav>
  </div>
</header>
<main id="content">
{{body}}
</main>
<footer class="site-footer">
  <div class="footer-brand">{{site_title}}</div>
  <div class="footer-meta">由 Typall 驱动的静态发布 · 数据在 Typst 中书写，设计在 <span class="accent-dot">CSS</span> 中呼吸</div>
</footer>
</body>
</html>"##;

/// 内置默认：文章列表项（`partials/post_item.html` 的回退）。
pub const DEFAULT_POST_ITEM: &str =
    r#"<li class="post-item"><a href="{{url}}"><div class="post-title">{{title}}</div>{{date_html}}{{excerpt_html}}{{tags_html}}</a></li>"#;

/// 内置默认：文章页主体（`partials/article.html` 的回退）。
pub const DEFAULT_ARTICLE: &str =
    r#"<article><h1>{{title}}</h1><div class="meta">{{meta}}</div>{{toc}}{{content}}{{series_nav}}{{prev_next}}</article>"#;

/// 内置默认：章节目录（`partials/toc.html` 的回退）。
///
/// 变量 `{{items}}` 是 `site_html::postprocess_body` 产出的 `<li>` 列表（无标题时为空串，
/// 此时调用方整块省略）。
pub const DEFAULT_TOC: &str =
    r#"<nav class="toc"><details><summary>目录</summary><ul class="toc-list">{{items}}</ul></details></nav>"#;

/// 内置默认：分页（`partials/pagination.html` 的回退）。
pub const DEFAULT_PAGINATION: &str =
    r#"<nav class="pagination">{{prev_link}}<span class="current">{{current}} / {{total}}</span>{{next_link}}</nav>"#;

/// 内置默认：上一篇 / 下一篇（`partials/post_nav.html` 的回退）。
pub const DEFAULT_POST_NAV: &str = r#"<nav class="post-nav">{{prev_link}}{{next_link}}</nav>"#;

/// 内置默认：专栏内导航（`partials/series_nav.html` 的回退）。
///
/// 仅专栏文章有值；非专栏文章时 Rust 侧不渲染（整块为空串）。
/// 文本键 `{{series_name}}` `{{series_url}}` `{{index}}` `{{total}}`；
/// HTML 键 `{{prev_link}}` / `{{next_link}}`（无上/下一篇为空串）。
pub const DEFAULT_SERIES_NAV: &str = r#"<nav class="series-nav"><a class="series-home" href="{{series_url}}">专栏：{{series_name}}</a><span class="series-pos">{{index}} / {{total}}</span><div class="series-links">{{prev_link}}{{next_link}}</div></nav>"#;

/// 内置默认：标签云 / 分类云的单个条目（`partials/tag_cloud_item.html` 的回退）。
pub const DEFAULT_TAG_CLOUD_ITEM: &str =
    r#"<a class="tag-cloud-item" href="{{url}}">{{name}}<span class="count">{{count}}</span></a>"#;

/// 内置默认：整篇列表（`partials/post_list.html` 的回退，P2 后新增）。
///
/// 与 [`render_post_list`] 在未提供 partial 时的 Rust 侧渲染等价（`post_item.html` 仍是单条目的
/// 覆盖点）。主题覆盖 `post_list.html` 后可用 `{% for post in posts %}` 整体接管列表，
/// 而无需逐条改 `post_item.html`。
pub const DEFAULT_POST_LIST: &str = r#"<ul class="post-list">
{% for post in posts %}<li class="post-item"><a href="{{post.url}}"><div class="post-title">{{post.title}}</div>{% if post.date %}<div class="post-date">{{post.date}}</div>{% endif %}{% if post.excerpt %}<p class="post-excerpt">{{post.excerpt}}</p>{% endif %}{% if post.tags %}<div class="post-tags"><span class="tag">{{post.tags}}</span></div>{% endif %}</a></li>{% endfor %}
</ul>"#;

/// 内置默认：文章元信息拼接（`partials/meta.html` 的回退，P2 后新增）。
///
/// 变量均为 HTML 片段（已转义 / 已拼好链接），空值即空串：
/// `{{date_html}}` / `{{updated_html}}` / `{{tags_html}}` / `{{categories_html}}`。
/// 与 [`render_article`] 在未提供 partial 时的 Rust 侧 `meta_parts.join(" ")` 等价。
pub const DEFAULT_META: &str =
    r#"{{date_html}}{% if updated_html %} {{updated_html}}{% endif %}{% if tags_html %} {{tags_html}}{% endif %}{% if categories_html %} {{categories_html}}{% endif %}"#;

