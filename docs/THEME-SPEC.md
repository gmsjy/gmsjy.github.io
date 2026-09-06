# Typall 主题规格（THEME-SPEC）

> 版本 v2.0 · 2026-09-04
>
> **本文件为 Typall 主题系统的唯一权威文档**，合并自原「演进规格 v1.0 / 主题契约 v0.1 / 主题作者指南 / 默认主题重做「墨理」」四份，随主程序实现状态维护。
> 主题作者只依赖本文档；主程序内部怎么重构，只要不破坏契约，主题就不用改。

---

## 0. 一句话定位与使用方式

Typall 的主题系统是「**CSS 层 + 骨架模板层**」两层解耦，由**一个目录**里的几个文件组成：

- **能改「长什么样」**：写 `style.css`（配色/字体/排版）+ `template.html`（页面骨架）。
- **能改「是什么结构」**：`partials/`（9 个片段）+ `template.html` 里的 `{% for %}` / `{% if %}`（P2 起）可接管列表/文章/分页/TOC 等结构。

主程序负责产出**结构化数据**（`site.*` / `page.*` / `post.*` / `posts` / `params.*`），主题决定「长什么样 + 怎么摆」。

**设计原则（不变项）**

1. **数据与结构分离**：数据先于结构暴露；结构可由主题覆盖。
2. **命名空间化**：占位符分 `site.*` / `post.*` / `page.*` / `params.*` 四个域，避免撞名。
3. **字段只增不改**：占位符名、class 名、partial 文件名一旦发布即 API，向后兼容（老主题的扁平写法永久兼容，见 §11）。

**如何使用本文档**：写主题 → 先读 §1 目录结构 + §2 可用变量 + §4 tera 须知 + §5 关键坑；换结构 → §3 partials；换配色 → §7 CSS 挂载点 + §8 墨理令牌。

---

## 1. 目录结构与加载规则

```
themes/<name>/
  style.css          # 可选。全局样式，缺失用内置默认「墨理」
  template.html      # 可选。HTML 骨架，缺失用内置默认
  static/            # 可选。整目录复制到输出根（不打 assets/ 前缀）
  partials/          # 可选。片段模板，缺失回退内置默认
    post_item.html       # 列表单条
    post_list.html       # 整列表（{% for %}）
    article.html         # 文章页外壳
    meta.html            # 文章元信息
    toc.html             # 目录外壳
    pagination.html      # 分页条
    post_nav.html        # 上一篇 / 下一篇
    series_nav.html      # 专栏导航（专栏首页 + 位置 + 上一篇/下一篇）
    tag_cloud_item.html  # 标签云 / 分类云单条
```

启用：`typall.toml` 中 `[theme] name = "my-theme"`。

- `theme.name = "default"`（或空）→ 直接用内置默认主题，不读磁盘。
- 指定其他名字 → 查 `themes/<name>/`，目录不存在则构建报错。
- `style.css` / `template.html` 各自独立可选：缺失的那一项回退内置默认，**不会整主题报错**。
- 主题生效的最小要求：`themes/<name>/` 下存在 `style.css` **或** `template.html` 之一。
- `static/` 内文件原样落到输出根（用于字体、图片、额外 JS）。注意与项目 `assets/` 的区别：项目 `assets/` 复制到输出 `assets/` 子目录，主题 `static/` 复制到根。

---

## 2. 模板占位符全集

> 类型列：`text` = 文本，HTML 转义后注入；`html` = 原样注入；`join` = 数组以 `, ` 连接后转义。
> 命名空间（`site.*` / `page.*` 等）与旧扁平名（`site_title` 等）**双轨并存**，见 §11。

### 2.1 站点级 `site.*`

| 占位符 | 来源 | 类型 | 说明 |
|--------|------|------|------|
| `{{site.title}}` ≡ `{{site_title}}` | `config.site.title` | text | 站点名 |
| `{{site.description}}` ≡ `{{site_description}}` | `config.site.description` | text | 站点描述 |
| `{{site.author}}` ≡ `{{site_author}}` | `config.site.author` | text | 作者 |
| `{{site.url}}` ≡ `{{site_url}}` | `config.site.url` | text | 站点 URL，未配置时为空串 |
| `{{site.language}}` ≡ `{{language}}` | `config.site.language`（空则 `zh-CN`） | text | 站点语言 |

### 2.2 页面级 `page.*`

| 占位符 | 类型 | 说明 |
|--------|------|------|
| `{{title}}` ≡ `{{page.title}}` | text | 页面标题（首页/分页为空串） |
| `{{full_title}}` ≡ `{{page.full_title}}` | text | `标题 · 站点名`（无标题时仅站点名），建议用作 `<title>` |
| `{{page.path}}` | text | 当前路径（如 `/posts/foo/`） |
| `{{page.og_type}}` | text | `article` / `website` |
| `{{page.current}}` / `{{page.total}}` | text | 分页当前页 / 总页数；无分页时 `1` / `1` |

### 2.3 文章级 `post.*`（仅文章页有值）

| 占位符 | 来源 | 类型 | 说明 |
|--------|------|------|------|
| `{{post.title}}` | `post_title(doc)` | text | 文章标题，缺省为 slug |
| `{{post.date}}` | `meta.date` | text | 发布日期 |
| `{{post.updated}}` | `meta.updated` | text | 最后更新（可能为空） |
| `{{post.excerpt}}` | `meta.effective_excerpt(&body_html)` | text | 手动 `excerpt` 优先，否则正文自动截取 |
| `{{post.slug}}` | `doc.slug` | text | 文章 slug（如 `posts/foo`） |
| `{{post.tags}}` | `meta.tags` | join | 标签，以 `, ` 连接 |
| `{{post.categories}}` | `meta.categories` | join | 分类，以 `, ` 连接 |
| `{{post.series}}` | `meta.series` | text | 专栏名（`#let series = "高中物理"`）；非专栏文章为空串 |
| `{{post.raw.<key>}}` | `meta.raw` 拍平后的自定义字段 | text | `.typ` 里 `#let <key> = ...` 的自定义字段（见 §2.6） |

> **非文章页行为**：固定字段（`post.title` / `date` / `updated` / `excerpt` / `slug` / `tags` / `categories` / `series`）替换为**空串**，不报错；
> 动态键 `post.raw.<key>` 因键缺失会**构建期报错**（P2 tera 严格语义）——引用可选/自定义字段请一律用 `{% if %}` 守卫（见 §5.2）。

### 2.4 列表页 `posts` 集合

列表页（首页、标签页、分类页、归档页）可遍历：

```jinja
{% for p in posts %}
  <a href="{{p.url}}">{{p.title}}</a>
{% endfor %}
```

每个元素字段：`url` / `title` / `date` / `tags`（逗号拼接字符串）/ `excerpt`。

> `posts` 集合让主题在 `template.html` 里直接写 `{% for p in posts %}` 接管列表渲染，无需再改主程序。
> `tags` 为逗号拼接字符串（与 `{{post.tags}}` 一致），逐项遍历用 `| split(pat=", ")`（注意：tera 的 `split` 参数名是 `pat`，`join` 才是 `sep`）。

### 2.5 主题参数 `params.*`

来源：`typall.toml` 的 `[theme.params]`（TOML 表），在 `template.html` 与 `style.css` 中均可引用。

```toml
[theme]
name = "my-theme"

[theme.params]
accent_color = "#007acc"

[theme.params.font]
family = "serif"
size = "16px"
```

- 叶子值转字符串：字符串/整数/浮点/布尔/日期时间 → 文本；数组 → 以 `, ` 连接；嵌套表 → 递归展开为点号路径。
- 引用：`{{params.accent_color}}`、`{{params.font.family}}`、`{{params.font.size}}`。
- **值原样注入、不转义**（用于 CSS 值/片段，作者自控安全）。
- 未匹配的占位符**原样保留**（不会报错也不会清空）——`style.css` 走 `apply_params` 的 `String::replace`，不受 tera 严格语义影响。

### 2.6 文章自定义字段（`post.raw.<key>`）

来源：文章 `.typ` 顶层的任意 `#let <key> = <字面量>`（P0 新增）。只解析顶层绑定，不递归进入函数体/代码块。

```typst
#let cover = "/img/a.png"
#let series = "Typst 入门"
#let weight = 3
#let draft_note = none
```

在 `template.html` 中引用：`{{post.raw.cover}}`、`{{post.raw.series}}`。拍平规则：

| `.typ` 中的类型 | 占位符取值 |
|----------------|-----------|
| 字符串 | 原值 |
| 布尔 | `true` / `false` |
| 整数 / 浮点 | `3` / `3.14` |
| 数组 | 元素按上述规则转字符串后以 `, ` 连接 |
| `none` / 其他表达式 | 空串 |

> 已知字段（`title` / `date` / `tags` / `categories` / `excerpt` / `updated` / `draft` / `series` / `series_weight`）已被主程序解析为 `post.*` 的对应占位符，请优先用那些；`post.raw.*` 用于**自定义**字段。

### 2.7 HTML 块（非数据，原样注入）

| 占位符 | 类型 | 说明 |
|--------|------|------|
| `{{body}}` | html | 页面主体（文章/列表/归档等**已渲染好的 HTML**），**必须**在模板里出现 |
| `{{math_style}}` | html | MathML 对齐 `<style>` 块；`build.math.renderer="svg"` 时为空串 |
| `{{nav_extra}}` | html | 集合页导航（专栏/标签/归档链接），首页文章为空时为空 |
| `{{seo_head}}` | html | canonical/OG/Twitter 标签；`site.url` 为空时为空串 |
| `{{analytics}}` | html | 统计/分析 HTML（`[site] analytics` 配置，注入 `</head>` 前）；未配置为空串 |

> 两根支柱：`{{body}}` 与样式引用 `<link rel="stylesheet" href="/assets/style.css">` 缺一即坏。

---

## 3. partial 片段模板

主题可在 `themes/<name>/partials/*.html` 覆盖「结构片段」；文件名（不含扩展名）即片段名。缺失回退内置默认（内置默认 = Rust 侧 `format!` 产物抽成的常量）。片段内部用 `{{key}}` 占位符，与 `template.html` 同引擎。共 9 个：

| 片段文件 | 文本键（转义） | HTML 键（原样） | 说明 |
|----------|---------------|----------------|------|
| `post_item.html` | `url` `title` `date` `excerpt` `tags` | `date_html` `excerpt_html` `tags_html` | 列表单条；HTML 键是预拼好的整段（如 `<p class="post-excerpt">…</p>`），**空值即空串** |
| `post_list.html` | `count` | —（`posts` 为集合） | **整列表**（`{% for p in posts %}`），**整块接管**型 |
| `article.html` | `title` `date` `updated` `tags` `categories` `excerpt` | `meta` `toc` `content` `series_nav` `prev_next` | 文章页外壳 |
| `meta.html` | `date` `updated` | `date_html` `updated_html` `tags_html` `categories_html` | 文章元信息，**整块接管**型 |
| `toc.html` | — | `items` | 目录外壳；`items` 是后处理产出的 `<li>` 列表 |
| `pagination.html` | `current` `total` `prev_url` `next_url` | `prev_link` `next_link` | 分页条 |
| `post_nav.html` | `prev_url` `prev_title` `next_url` `next_title` | `prev_link` `next_link` | 上一篇/下一篇 |
| `series_nav.html` | `series_name` `series_url` `index` `total` `prev_url` `prev_title` `next_url` `next_title` | `prev_link` `next_link` | 专栏导航；非专栏文章整段为空串 |
| `tag_cloud_item.html` | `name` `url` `count` | — | 标签云/分类云单条（循环仍在 Rust 侧） |

> **整块接管 vs 回退默认**：`post_list.html` 与 `meta.html` 属「整块接管」型——主题**提供**时整体替换列表/元信息渲染，**未提供**时走 Rust 兼容路径（`post_list` → 逐条 `post_item`；`meta` → `meta_parts.join(" ")`），行为字节兼容。其余 7 个 partial 始终走「回退内置默认常量」路径（先查主题文件，命中即用，否则内置默认）。

> **新增键不破坏旧主题**：`article.html` 新增的 `{{series_nav}}` 键符合「只增不改」——tera 只对**引用了缺失变量**报错，不惩罚未使用的提供值。自定义 `article.html` 不写 `{{series_nav}}` 完全合法（专栏导航就不渲染）；要支持专栏时把它插在 `{{content}}` 与 `{{prev_next}}` 之间即可。

示例 —— 自定义列表结构（覆盖 `post_list.html`）：

```jinja
<ul class="cards">
{% for p in posts %}
  <li class="card"><a href="{{p.url}}"><h2>{{p.title}}</h2>
  {% if p.date %}<time>{{p.date}}</time>{% endif %}</a></li>
{% endfor %}
</ul>
```

---

## 4. 模板引擎：tera

模板用 [tera](https://keats.github.io/tera/)（Jinja2 风格）。`template.html` 的 `{{占位符}}` 语法与旧版兼容（tera 兼容 `{{ var }}`）；`{% %}` 是 P2 起的控制能力。

**输出变量**

```jinja
{{post.title}}          {# text：已由主程序转义，直接输出 #}
{{body}}                {# html：HTML 块，原样注入 #}
```

**控制流**

```jinja
{% for p in posts %}...{% endfor %}
{% if post.raw.cover %}...{% endif %}
```

**过滤器**

```jinja
{{post.tags | split(pat=", ") | join(sep=" · ")}}   {# 逐项重排逗号拼接的 tags #}
```

### 4.1 转义约定（必读）

Typall 在**主程序侧完成所有 HTML 转义**，模板引擎本身**不自动转义**（`Tera::one_off(.., autoescape=false)`）。因此：

- **text 变量**（`title` / `date` / `tags` / `excerpt` 等标量）：主程序已转义，`{{ }}` 直接输出即可，**不要**再 `| safe`。
- **html 变量**（`body` / `meta` / `toc` / `content` / `*_html` 等片段）：拼好的 HTML，原样注入。

一句话：**`{{变量}}` 写上去就是安全的**，无需关心转义细节。

### 4.2 严格语义（P2 行为变更）

tera 对 `{{ }}` 里引用不到的键**直接报错**（如非文章页的 `{{post.raw.cover}}`）。这比 P0 的「原样保留字面量」更安全——把模板拼写错误暴露在构建期。主题对可选字段（封面、自定义 meta）应改用 `{% if %}` 守卫（§5.2）。

---

## 5. 关键坑（务必读完）

### 5.1 `tags` / `categories` 是「逗号拼接字符串」

不是数组。所以：

- 直接显示：`{{post.tags}}` → `Rust, Typst`
- 逐项遍历：`{% for t in post.tags | split(pat=", ") %}` → `Rust`、`Typst`
- 判断有无：`{% if post.tags %}...{% endif %}`

### 5.2 引用「可能缺失」的键要 `{% if %}` 守卫

固定字段（`post.title` / `post.date` / `post.updated` 等）永远存在（无值时是空串），不会报错；但**动态键 `post.raw.<key>`** 在键缺失时会**构建期报错**：

```jinja
❌ {{post.raw.cover}}                         {# 无 cover 字段的文章 → 构建报错 #}
✅ {% if post.raw.cover %}{{post.raw.cover}}{% endif %}
```

`post.raw.<key>` 一律用 `{% if %}` 包裹。非文章页同理（§2.3）。

### 5.3 自定义字段 `post.raw.<key>`

键名动态，主程序不预枚举，故必须配合 `{% if %}` 使用。

---

## 6. 常见场景配方

**文章封面**

```jinja
{% if post.raw.cover %}<img class="cover" src="{{post.raw.cover}}">{% endif %}
```

**文章列表（在 template.html 里直接接管）**

```jinja
{% for p in posts %}
  <article><a href="{{p.url}}"><h2>{{p.title}}</h2></a></article>
{% endfor %}
```

**分页（template.html 内联或覆盖 pagination.html）**

```jinja
{% if page.current > 1 %}<a href="/page/{{page.current - 1}}/">上一页</a>{% endif %}
```

**文章元信息（覆盖 meta.html）**

```jinja
<time>{{date}}</time>
{% if updated_html %}{{updated_html}}{% endif %}
{{tags_html}}{{categories_html}}
```

---

## 7. CSS 挂载点（class / data 契约）

这是主题作者**最常依赖**的部分——换配色 80% 靠这些 class。

### 7.1 布局骨架（模板层，由 `template.html` 决定，默认模板如下）

| class | 位置 |
|-------|------|
| `.site-header` / `.inner` | 顶栏 / 顶栏内容容器 |
| `.site-title` | 站点名链接 |
| `.site-nav` | 主导航 |
| `.skip-link` | 无障碍「跳到正文」链接 |
| `main#content` | 主内容区（元素为 `<main>`） |
| `.site-footer` / `.footer-brand` / `.footer-meta` / `.accent-dot` | 页脚四件套 |

### 7.2 文章列表（首页/集合页共用）

```html
<ul class="post-list">
  <li class="post-item">
    <a href="/slug/">
      <div class="post-title">标题</div>
      <div class="post-date">2026-09-01</div>          <!-- 可选 -->
      <p class="post-excerpt">摘要</p>                  <!-- 可选 -->
      <div class="post-tags"><span class="tag">标签</span></div> <!-- 可选 -->
    </a>
  </li>
</ul>
```

| class | 说明 |
|-------|------|
| `.post-list` | 无序列表容器 |
| `.post-item` | 单项 |
| `.post-title` | 标题 |
| `.post-date` | 日期 |
| `.post-excerpt` | 摘要 |
| `.post-tags` / `.tag` | 标签容器 / 标签胶囊 |

### 7.3 文章页

```html
<article>
  <h1>标题</h1>
  <div class="meta">
    <span>2026-09-01</span>                              <!-- 可选 -->
    <span class="updated" title="最后更新">更新于 …</span> <!-- 可选 -->
    <a class="tag" href="/tags/slug/">标签</a>            <!-- 可选，可多个 -->
  </div>
  <nav class="toc">…</nav>                               <!-- 可选，见 7.4 -->
  {正文 HTML}                                            <!-- 见 §9 -->
  <nav class="post-nav">…</nav>                          <!-- 可选 -->
</article>
```

| class | 说明 |
|-------|------|
| `article` | 文章容器（正文选择器建议前缀 `article`） |
| `.meta` | 元信息条 |
| `.updated` | 更新日期 |
| `.meta .tag` | 文章页标签胶囊（与列表页 `.post-tags .tag` 共用 `.tag` 样式） |

### 7.4 目录（TOC）

```html
<nav class="toc">
  <details>
    <summary>目录</summary>
    <ul class="toc-list">
      <li class="toc-l2"><a href="#id">二级标题</a></li>
      <li class="toc-l3"><a href="#id">三级标题</a></li>
    </ul>
  </details>
</nav>
```

| class | 说明 |
|-------|------|
| `.toc` / `.toc-list` | TOC 容器 / 列表 |
| `.toc-l2` / `.toc-l3` | 二级/三级条目 |
| `summary` | 折叠开关（默认主题用 `::before` 画 ▸/▾） |

### 7.5 上一篇 / 下一篇与专栏导航

```html
<nav class="post-nav">
  <a class="prev" href="/slug/">← 标题</a>
  <a class="next" href="/slug/">标题 →</a>
</nav>

<nav class="series-nav">                                <!-- 可选，仅专栏文章 -->
  <a class="series-home" href="/series/slug/">专栏：高中物理</a>
  <span class="series-pos">2 / 17</span>
  <div class="series-links">
    <a class="prev" href="/slug/">← 标题</a>
    <a class="next" href="/slug/">标题 →</a>            <!-- 均可选 -->
  </div>
</nav>
```

| class | 说明 |
|-------|------|
| `.post-nav` `.prev` `.next` | 时间序上一篇/下一篇容器与链接 |
| `.series-nav` | 专栏导航容器（专栏内为**课程序**，与 `.post-nav` 的时间序无关） |
| `.series-home` / `.series-pos` | 专栏首页链接 / 位置徽标（`2 / 17`，tabular-nums） |
| `.series-links` `.prev` `.next` | 专栏内前后篇链接容器与链接 |

### 7.6 分页

```html
<nav class="pagination">
  <a href="/page/1/">← 上一页</a>        <!-- 可选 -->
  <span class="current">2 / 5</span>
  <a href="/page/3/">下一页 →</a>         <!-- 可选 -->
</nav>
```

| class | 说明 |
|-------|------|
| `.pagination` / `.current` | 分页容器 / 当前页标记 |

### 7.7 标签云 / 分类云

```html
<h1>标签</h1>
<div class="tag-cloud">
  <a class="tag-cloud-item" href="/tags/slug/">名称<span class="count">12</span></a>
</div>
```

| class | 说明 |
|-------|------|
| `.tag-cloud` / `.tag-cloud-item` / `.count` | 云容器 / 云项 / 计数徽标 |

### 7.8 搜索页

```html
<h1>搜索</h1>
<input type="search" id="search-input" class="search-input" …>
<p class="search-hint">匹配范围：标题 / 标签 / 摘要 / 正文全文</p>
<ul id="search-results" class="post-list"></ul>
```

| 钩子 | 说明 |
|------|------|
| `.search-input` / `.search-hint` | 输入框 / 提示 |
| `#search-input` / `#search-results` | 内联 JS 依赖的 **id，勿改**；结果复用 `.post-list` / `.post-item` 结构 |

### 7.9 集合页标题（`<main>` 直子）

| 选择器 | 命中 |
|--------|------|
| `main > h1` | 首页「文章」/ 标签云「标签」/ 归档「归档」等集合页大标题 |
| `main > h2` | 归档页的年月分组标题（`2026-09` 等） |

> 集合页标题是 `<main>` 的**直接子元素**；正文标题在 `<article>` 内，用 `main > h1` 精准命中、不与正文冲突。

### 7.10 渲染辅助

| class | 说明 |
|-------|------|
| `.hidden` | 保留兼容的隐藏工具类 |

---

## 8. 默认主题「墨理」（设计令牌）

默认主题为**「墨理」杂志编辑风**。自定义主题可自由覆盖以下变量，也可完全不用（用纯 class 选择器）；默认主题依赖它们实现明暗自适应。

### 8.1 设计取向

- **衬线标题**：文章标题 / 页面标题 / 列表标题 / 归档年月统一走衬线栈
  `Georgia → Songti SC / Noto Serif CJK SC`，营造杂志副刊气质。
- **无衬线正文**：中文正文保留现代屏显字体（PingFang / 雅黑），行高放宽到 1.78，保证长文可读性——中文正文全衬线会偏细伤眼。
- **暖纸底**：整体底色 `#faf9f6`（非纯白），正文墨色，接近纸张而非屏幕。
- **克制的强调**：唯一编辑青 `#0b6fc0`，只在链接 / 强调 / 胶囊 hover / 标题点缀线出现。

### 8.2 CSS 变量令牌

| 令牌 | 用途 | 亮色 | 深色（夜墨） |
|------|------|------|------|
| `--bg` | 页面底 | `#faf9f6` | `#14151a` |
| `--surface` | 卡片 | `#ffffff` | `#1b1d23` |
| `--surface-2` | 代码/引用/胶囊底 | `#f4f1ea` | `#23252d` |
| `--fg` / `--fg-strong` | 正文 / 强调 | `#20222a` / `#14161b` | `#e6e7ec` / `#f4f5f8` |
| `--muted` / `--faint` | 次级 / 提示文本 | `#686a72` / `#9a9ba3` | `#a0a2ad` / `#70727d` |
| `--accent` / `--accent-strong` | 强调色 / hover 深化 | `#0b6fc0` / `#0a5c9e` | `#4aa3f5` / `#7dbdf9` |
| `--accent-soft` | 浅青底 / hover | `#e7f0f9` | `#1c2a38` |
| `--quote` / `--quote-soft` | 引用墨绿 / 引用浅底 | `#3c5a49` / `#eef2ec` | `#9fc4ab` / `#1f2923` |
| `--border` / `--border-strong` | 边框 / 分隔线 | `#e6e2d6` / `#d3cdbd` | `#2c2e37` / `#3a3c47` |
| `--code-bg` / `--code-ink` | 代码背景 / 代码文字 | （同 `--surface-2` / `--fg` 系） | （同深色） |
| `--maxw` | 内容最大宽度 | — | — |
| `--serif` / `--sans` / `--mono` | 衬线 / 无衬线 / 等宽字体栈 | — | — |

明暗切换由 `@media (prefers-color-scheme: dark)` 完成；默认主题亮色为主。

> `--code-bg` / `--code-ink` / `--maxw` / `--serif` 等仅以名称列出的令牌，其取值由默认 `style.css` 定义；自定义主题覆盖同名变量即可换值。

---

## 9. 正文 HTML 结构契约（后处理产物）

`site_html.rs` 对 Typst 输出的正文做 4 项变换，主题作者需要知道这些结构以正确样式化：

| 变换 | 产物结构 |
|------|----------|
| 标题锚点 | `<h2 id="slug">` / `<h3 id="slug">`（slug 为纯文本派生，中文保留，空格→`-`） |
| 代码高亮 | `style="color: var(--tok-<hex>, #<hex>)"` —— 主题可覆盖 `--tok-<hex>` 换代码配色 |
| 引用提升 | `> 引用` 段落提升为 `<blockquote><p>…</p></blockquote>`（连续多段合并） |
| 图片 | 首图不加 `loading`（保 LCP）、后续加 `loading="lazy"`；缺 `alt` 用文件名兜底 |

数学公式（取决于 `build.math.renderer`）：

- **mathml**（默认）：块级 `<math display="block">`，对齐样式在 `{{math_style}}` 注入的 `<style>` 块里。
- **svg**：块级 `<div data-equation="block"><svg>…</svg><span class="eq-num">(公式 1)</span></div>`；`{{math_style}}` 为空。

| 钩子 | 说明 |
|------|------|
| `div[data-equation="block"]` | 块级公式容器（flex 居中） |
| `span.eq-num` | 公式编号（绝对定位于右缘） |
| `--tok-<hex>` | 代码高亮 token 色变量（如 `--tok-d73948`） |

---

## 10. 各页面 body 结构速查

| 页面 | 路径 | body 顶层结构 |
|------|------|--------------|
| 首页 | `/` | `<h1>文章</h1>` + `.post-list` + `.pagination` |
| 分页 | `/page/N/` | 同首页 |
| 文章 | `/{slug}/` | `<article>`（见 §7.3） |
| 标签云 | `/tags/` | `<h1>标签</h1>` + `.tag-cloud` |
| 标签页 | `/tags/{slug}/` | `<h1>标签：X</h1>` + `.post-list` |
| 分类云 | `/categories/` | `<h1>分类</h1>` + `.tag-cloud` |
| 分类页 | `/categories/{slug}/` | `<h1>分类：X</h1>` + `.post-list` |
| 专栏云 | `/series/` | `<h1>专栏</h1>` + `.tag-cloud` |
| 专栏页 | `/series/{slug}/` | `<h1>专栏：X</h1>` + `.post-list`（课程顺序） |
| 归档 | `/archive/` | `<h1>归档</h1>` + 多组 `<h2>YYYY-MM</h2>` + `.post-list` |
| 搜索 | `/search/` | 见 §7.8 |
| 404 | `404.html` | `<h1>404</h1><p>…</p>` |

---

## 11. 兼容性、迁移与稳定性承诺

1. **旧扁平占位符永久保留**：`{{site_title}}` ≡ `{{site.title}}`，`{{title}}` ≡ `{{page.title}}`，`{{full_title}}` ≡ `{{page.full_title}}`，`{{language}}` ≡ `{{site.language}}`。新命名空间与旧名双轨并存，不强制迁移。
2. **转义语义不变**：`site.*` / `post.*` / `page.*` 转义；`params.*` 与 HTML 块不转义。
3. **缺失键语义（P2 起）**：`{{params.*}}` 未匹配键仍「原样保留」；但 `template.html` 中 tera 变量引用缺失键会**构建期报错**（P0 的「非文章页原样保留」已废止，改为严格语义，见 §4.2）。
4. **class 契约不变**：各阶段均不改现有 class 名，只增字段/片段。

**三层稳定性承诺**

- **稳定层**：class 名、占位符名、`{{params.*}}` 语义、CSS 变量名——视为公开 API，加字段只增不改。
- **演进层**：§9 后处理细节、§10 页面结构——可能优化，但会先在此文档标注变更。
- **内部层**：Rust 函数签名、模块划分——主题作者不应依赖。

**当前边界与已知限制**

1. **列表与文章 meta 的默认渲染仍由 Rust 产出**：9 个片段均可经 `partials/` 覆盖；`post_list` / `meta` 两个「整块接管」型未提供时走 Rust 兼容路径（逐条 `post_item`、`meta_parts.join(" ")`）。
2. **集合数据已进模板**：P2 起 `posts` 集合与 `page.*` 进入模板上下文，主题可在 `template.html` 写 `{% for p in posts %}` 或覆盖 `post_list.html` 整体接管列表。
3. **搜索页内联 JS 依赖固定 id**：`#search-input` / `#search-results` 不可改名，否则搜索失效。
4. **样式入口固定**：CSS 写死在 `/assets/style.css`（模板里 `<link>` 指向它）。
5. **模板引擎能力**：P2 起 `template.html` 支持 `{% for %}` / `{% if %}`（tera，`autoescape=false`，转义仍由 Rust 侧完成）；`{{params.*}}` 在 `style.css` 侧仍为 `String::replace` 简单替换。
6. **P3 新增键均为只增不改**：`{{post.series}}`、`{{analytics}}`、partial `series_nav.html`、`article.html` 的 `{{series_nav}}` 键——自定义主题**不引用它们不报错**（tera 只惩罚引用缺失变量，不惩罚未使用的提供值），引用即得功能。

---

## 12. 命名与转义规范（权威约定）

| 域 | 前缀 | 转义 | 值的类型 | 空值行为 |
|----|------|------|---------|---------|
| 站点级 | `site.*` | text（转义） | 标量 | 空串 |
| 页面级 | `page.*` | text | 标量 | 空串 |
| 文章级 | `post.*` | text / join | 标量 / 数组 | 非文章页清空（raw 缺失键报错） |
| 主题参数 | `params.*` | **不转义** | 任意 | 原样保留 |
| HTML 块 | （无前缀） | html（不转义） | HTML 字符串 | 空串 |

新增占位符必须落表（§2）并注明类型，禁止临时拼名。字段名用小写下划线；数组型字段用 `join` 并注明连接符。

---

## 13. 演进与落地记录

| 阶段 | 内容 | 落地 | 实测 |
|------|------|------|------|
| P0（2026-09-03） | **打通数据出口**：`PostVars` + `PageContext` 扩 3 字段 + `render_page` 替换逻辑；`build.rs` 的 `page()` 增 `post` 参数；新增 `flatten_meta_raw()` 拍平 `DocumentMeta::raw`；站点作者/URL/文章级全元数据（含 `raw` 自定义字段）暴露到占位符 | ✅ | 临时主题 + 探针文章，见下注 |
| P1（2026-09-04） | **片段模板下沉**：`theme.rs` 新增 `Theme::partial` / `render_partial` / `load_partials`，抽 6 个 `DEFAULT_*` 内置常量；`build.rs` 的 `render_post_list` / `render_article` / `render_pagination` / `render_prev_next` / `render_cloud` 改走 partial 渲染器；`site_html.rs` TOC 改为产出 `<li>` 列表 | ✅ | 覆盖 `post-item` → `post-card` 生效，改结构不碰 Rust |
| P2（2026-09-04） | **循环与条件**：引入 `tera`（`autoescape=false`）；`SiteData`/`PageData`/`PostData`/`PostSummary`/`ThemeData` 数据模型（均 `Serialize`）+ `posts` 集合 + `page.*` 落地；补 `post_list.html` / `meta.html` 两个「整块接管」partial | ✅ | `{% for %}` / `{% if %}` 端到端 0.07s，旧扁平占位符无回归 |
| 墨理（2026-09-04） | **默认主题重做**：`DEFAULT_CSS` 整块重写为杂志编辑风 + 编辑青强调，保留全部 class 契约；`DEFAULT_TEMPLATE` 骨架微调（首页前置、`aria-label`、无障碍跳转、`main id="content"`、编辑风页脚） | ✅ | 全站截图无视觉回归 |
| P3（2026-09-04） | **专栏 + analytics**：`post.series` / `[site] analytics` 新占位符；第 9 个 partial `series_nav.html`（`DEFAULT_SERIES_NAV`）+ `article.html` 插入 `{{series_nav}}` 键；`DEFAULT_CSS` 增 `.series-nav` 块；专栏云/专栏页（`/series/`）；`nav_extra` 增「专栏」链接 | ✅ | 专栏文章底部渲染课程序导航；analytics 原样注入 head |

**默认主题「墨理」实现备注（踩坑记录）**

- **模板 raw string 需 `r##"..."##`**：模板内 `href="#content"` 的 `"#` 会提前终止单井 raw string（`r#"..."#`）。含 hash 链接的模板必须用多井号定界。
- **CSS 内不可含 `"#`**：若用 `r#"`，内容里任何 `"` 后跟 `#` 会断裂编译。
- 归档页为 `<main>` 直子 `<h1>/<h2>`，正文在 `<article>` 内——用 `main > h1` / `main > h2` 精准命中集合页标题，不与正文冲突。

**历史演进注记**

- 数据契约断层（`author` / `raw` / `categories` / `updated` 已收集未暴露）是本套主题系统启动的前因；P0 后不再存在。
- P1 阶段曾计划下沉 `page.*`（`page.path` / `page.og_type` / `page.current` / `page.total`），因与 partials 下沉是独立事项，后并入 P2 随 `ThemeData` 一并落地。
- P1 阶段实际 partial 键比初版契约多出若干 `*_html` 预拼片段（空值即空串）与 `prev_link` / `next_link` 整段 `<a>`——已并入 §3 契约表。

---

## 附：完整示例主题

```
themes/minimal/
├── style.css
├── template.html
└── partials/
    ├── post_list.html
    └── meta.html
```

`template.html`：

```html
<!DOCTYPE html>
<html lang="{{language}}">
<head><meta charset="utf-8"><title>{{full_title}}</title>{{seo_head}}
<link rel="stylesheet" href="/assets/style.css"></head>
<body>
<header><a href="/">{{site_title}}</a></header>
{{body}}
<footer>© {{site_author}}</footer>
</body>
</html>
```

`partials/post_list.html`：

```jinja
<ul class="posts">
{% for p in posts %}<li><a href="{{p.url}}">{{p.title}}</a></li>{% endfor %}
</ul>
```

`partials/meta.html`：

```jinja
<div class="meta"><time>{{date}}</time>{{tags_html}}{{categories_html}}</div>
```

---

## 附二：官方参考主题画廊

仓库 `themes/` 下随附四套参考主题（`obsidian` 曜石 / `paper` 纸砚 / `minimal` 留白 / `citrus` 柑橘），
`[theme] name` 一行启用。其设计令牌、落地范围与踩坑记录见 **[THEME-GALLERY.md](THEME-GALLERY.md)**；
其中「留白」的 `partials/post_list.html` 是本规格 §2.4 / §3「整块接管」能力的官方示范。

---

*本规格随实现进度维护：新增变量 / partial / class 时，同步更新 §2 / §3 / §7 对应清单。*
