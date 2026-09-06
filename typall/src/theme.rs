//! 主题系统：内置默认主题 + 自定义主题（`themes/<name>/`）。
//!
//! 设计原则（规格书 §2.3）：
//! - Typst 只负责语义化正文（含 MathML），主题负责页面骨架与外观。
//! - 自定义主题目录结构：
//!   - `style.css`：普通 CSS 文件，定义全局样式与主题外观（可选，缺失用内置默认）。
//!   - `template.html`：HTML 骨架模板（可选，缺失用内置默认）。
//!   - `static/`：主题静态资源（图片、字体、额外 CSS/JS），复制到输出根。
//! - 主题参数通过 `{{params.xxx}}` 占位符在 `template.html` 与 `style.css` 中访问。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::config::Config;
use crate::site_html::slugify;

/// 默认主题 CSS（明/暗模式自适应）。
pub use crate::theme_defaults::*;

/// 一个已加载的主题（内存态，供一次构建复用）。
pub struct Theme {
    pub name: String,
    css: String,
    template: String,
    static_dir: Option<PathBuf>,
    /// 片段模板 `partials/<name>.html` → 内容（P1）。
    ///
    /// 内置默认主题为空表，全部走 `DEFAULT_*` 内置常量回退。
    partials: HashMap<String, String>,
}

impl Theme {
    /// 从项目根目录加载主题。
    ///
    /// `theme.name = "default"`（或空）使用内置默认；否则查找 `themes/<name>/`。
    pub fn load(root: &Path, config: &Config) -> anyhow::Result<Self> {
        let name = if config.theme.name.is_empty() {
            "default".to_string()
        } else {
            config.theme.name.clone()
        };

        if name == "default" {
            return Ok(Self::builtin());
        }

        let dir = root.join("themes").join(&name);
        if !dir.is_dir() {
            anyhow::bail!(
                "主题不存在: {}（请在 themes/{} 下创建 style.css 或 template.html，\
                 或把 theme.name 改回 \"default\"）",
                dir.display(),
                name
            );
        }

        let css = if dir.join("style.css").is_file() {
            std::fs::read_to_string(dir.join("style.css"))?
        } else {
            DEFAULT_CSS.to_string()
        };
        let template = if dir.join("template.html").is_file() {
            std::fs::read_to_string(dir.join("template.html"))?
        } else {
            DEFAULT_TEMPLATE.to_string()
        };
        let static_dir = if dir.join("static").is_dir() {
            Some(dir.join("static"))
        } else {
            None
        };
        let partials = load_partials(&dir)?;

        Ok(Self {
            name,
            css,
            template,
            static_dir,
            partials,
        })
    }

    /// 内置默认主题（`name = "default"`）：无 `partials/`，片段全部走内置常量回退。
    pub fn builtin() -> Self {
        Self {
            name: "default".to_string(),
            css: DEFAULT_CSS.to_string(),
            template: DEFAULT_TEMPLATE.to_string(),
            static_dir: None,
            partials: HashMap::new(),
        }
    }

    /// 在内置默认主题上覆盖片段模板（测试用）。
    #[cfg(test)]
    pub(crate) fn with_partials(partials: HashMap<String, String>) -> Self {
        Self {
            partials,
            ..Self::builtin()
        }
    }

    /// 应用主题参数后的 CSS。
    pub fn css(&self, params: &toml::Table) -> String {
        apply_params(&self.css, params)
    }

    /// 主题静态资源目录（`themes/<name>/static/`），无则返回 `None`。
    pub fn static_dir(&self) -> Option<&Path> {
        self.static_dir.as_deref()
    }

    /// 主题提供的片段模板 `partials/<name>.html`，未提供则返回 `None`。
    pub fn partial(&self, name: &str) -> Option<&str> {
        self.partials.get(name).map(String::as_str)
    }

    /// 主题是否提供了某个 partial（区别于「回退默认」）。
    ///
    /// 供 `post_list.html` / `meta.html` 这类「命中则整体接管、未命中则走 Rust 兼容路径」
    /// 的调用点使用。
    pub fn has_partial(&self, name: &str) -> bool {
        self.partials.contains_key(name)
    }

    /// 渲染片段模板：优先用主题提供的 `partials/<name>.html`，缺失则回退 `default`。
    ///
    /// `text` 中的值按 HTML 转义注入；`html` 中的值原样注入。
    /// P2 起用 tera 渲染（`autoescape=false`），partial 内可用 `{% if %}` 等控制块。
    pub fn render_partial(
        &self,
        name: &str,
        default: &str,
        text: &[(&str, &str)],
        html: &[(&str, &str)],
    ) -> anyhow::Result<String> {
        let tpl = self.partial(name).unwrap_or(default);
        let mut context = tera::Context::new();
        for (k, v) in text {
            let escaped = escape(v);
            context.insert(*k, &escaped);
        }
        for (k, v) in html {
            context.insert(*k, v);
        }
        tera::Tera::one_off(tpl, &context, false).map_err(Into::into)
    }
}

/// 加载 `themes/<name>/partials/*.html`，以文件名（不含扩展名）为键。
fn load_partials(dir: &Path) -> anyhow::Result<HashMap<String, String>> {
    let mut out = HashMap::new();
    let pdir = dir.join("partials");
    if !pdir.is_dir() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(&pdir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("html") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        out.insert(stem.to_string(), std::fs::read_to_string(&path)?);
    }
    Ok(out)
}

/// 简单 HTML 转义。
pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

// ===== P2 模板数据模型（整体塞进 `tera::Context`）=====
//
// tera 采用 autoescape=false（渲染时传 false），转义仍由 Rust 侧 `escape()` 负责：
// 文本类字段在下方构造时已转义，HTML 类字段原样注入。这样 `{{body}}`、`{{params.*}}`
// 等旧模板语义不变，同时新增 `{% for %}` / `{% if %}` 能力（规格书 §5）。

/// 站点级数据（规格书 §2.1 `site.*`）。
#[derive(Serialize)]
struct SiteData {
    title: String,
    description: String,
    author: String,
    url: String,
    language: String,
}

/// 页面级数据（规格书 §2.1 `page.*`）。
#[derive(Serialize)]
struct PageData {
    title: String,
    full_title: String,
    path: String,
    og_type: String,
    current: usize,
    total: usize,
}

/// 文章级数据（规格书 §2.1 `post.*`），仅文章页 `Some`。
///
/// `tags`/`categories` 为逗号拼接的字符串（如 `"Rust, Typst"`），保持 P0 的
/// `{{post.tags}}` 语义；如需逐项遍历可用 `| split(sep=", ")` 过滤器。
#[derive(Serialize)]
struct PostData {
    title: String,
    date: String,
    updated: String,
    excerpt: String,
    slug: String,
    tags: String,
    categories: String,
    /// 专栏名（非专栏文章为空串）。
    series: String,
    raw: HashMap<String, String>,
}

/// 列表页单篇文章摘要（`posts` 集合元素，供 `{% for post in posts %}`）。
#[derive(Serialize)]
struct PostSummary {
    url: String,
    title: String,
    date: String,
    tags: String,
    excerpt: String,
}

/// 模板数据模型（P2 引入，整体塞进 `tera::Context`）。
///
/// 扁平键（`site_title` / `body` / …）为旧模板兼容（规格书 §6「双轨并存」）；
/// 嵌套域（`site` / `page` / `post` / `posts` / `params`）为规格书 §2.1 命名空间。
#[derive(Serialize)]
struct ThemeData {
    site_title: String,
    site_description: String,
    site_author: String,
    site_url: String,
    language: String,
    title: String,
    full_title: String,
    body: String,
    math_style: String,
    nav_extra: String,
    seo_head: String,
    analytics: String,
    site: SiteData,
    page: PageData,
    post: PostData,
    posts: Vec<PostSummary>,
    params: toml::Table,
}

/// 文章级模板变量（规格书 §3.3，P0 引入）。
///
/// 仅文章页提供；非文章页时 `PageContext::post` 为 `None`，`render_page` 会把
/// `post.<field>` 全部替换为空串。`post.raw.*` 键名动态：主题引用缺失键时应改用
/// `{% if post.raw.<key> %}` 守卫（P2），直接 `{{post.raw.<key>}}` 会因 tera 严格语义报错。
pub struct PostVars<'a> {
    pub title: &'a str,
    pub date: &'a str,
    pub updated: &'a str,
    pub excerpt: &'a str,
    pub slug: &'a str,
    pub tags: &'a [String],
    pub categories: &'a [String],
    /// 专栏名（`{{post.series}}`；非专栏文章传空串）。
    pub series: &'a str,
    /// 自定义字段（已拍平为 `key → 字符串`），如 `("cover", "/img/x.png")`。
    ///
    /// 拍平规则见规格书 §3.3，在 `build.rs` 完成，避免本模块依赖 `content::MetaValue`。
    pub raw: &'a [(String, String)],
}

/// 列表页文章摘要（`PageContext::posts` 的元素，P2 引入）。
///
/// 字段为 owned，避免与 `CompiledDoc` 的生命周期纠缠；`render_page` 里转义后塞入 tera。
/// `tags` 为逗号拼接字符串（与 `post.tags` 语义一致）。
#[derive(Serialize)]
pub struct PostSummaryRef {
    pub url: String,
    pub title: String,
    pub date: String,
    pub tags: String,
    pub excerpt: String,
}

pub struct PageContext<'a> {
    pub site_title: &'a str,
    pub site_description: &'a str,
    /// 站点作者（`{{site_author}}`，P0）。
    pub site_author: &'a str,
    /// 站点 URL（`{{site_url}}`，P0）。
    pub site_url: &'a str,
    pub language: &'a str,
    pub title: &'a str,
    /// 文章级变量，非文章页为 `None`（P0）。
    pub post: Option<PostVars<'a>>,
    /// 页面主体 HTML。
    pub body: &'a str,
    /// Typst 生成的 MathML 样式（`<style>` 块）。
    pub math_style: &'a str,
    /// 额外导航链接 HTML（可选）。
    pub nav_extra: &'a str,
    /// SEO head 块（OG/Twitter/canonical 标签，整段原样注入；站点 URL 未配置时为空串）。
    pub seo_head: &'a str,
    /// 统计/分析 HTML（`{{analytics}}`，**原样注入**不转义；未配置为空串）。
    pub analytics: &'a str,
    /// 主题参数（`{{params.xxx}}` 占位符的来源）。
    pub params: &'a toml::Table,
    /// 当前页面路径（`page.path`，P2），如 `/posts/foo/`。
    pub page_path: &'a str,
    /// 页面类型（`page.og_type`，P2）：`article` / `website`。
    pub page_og_type: &'a str,
    /// 分页当前页 / 总页数（`page.current` / `page.total`，P2）；无分页时为 `1` / `1`。
    pub page_current: usize,
    pub page_total: usize,
    /// 列表页文章摘要集合（`posts`，供 `{% for post in posts %}`，P2）；非列表页为空。
    pub posts: &'a [PostSummaryRef],
}

/// 用主题模板渲染完整 HTML 页面（P2 起用 tera，支持 `{% for %}` / `{% if %}`）。
///
/// tera 渲染时 `autoescape=false`，转义由 Rust 侧 `escape()` 完成（文本类转义、HTML 类原样），
/// 与 P0/P1 的 `String::replace` 语义一致，旧模板无需改。
pub fn render_page(theme: &Theme, ctx: &PageContext<'_>) -> anyhow::Result<String> {
    let lang = if ctx.language.is_empty() {
        "zh-CN".to_string()
    } else {
        ctx.language.to_string()
    };
    let full_title = if ctx.title.is_empty() {
        ctx.site_title.to_string()
    } else {
        format!("{} · {}", ctx.title, ctx.site_title)
    };

    // 文本类字段转义（HTML 类原样）。
    let site_title = escape(ctx.site_title);
    let site_description = escape(ctx.site_description);
    let site_author = escape(ctx.site_author);
    let site_url = escape(ctx.site_url);
    let language = escape(&lang);
    let title = escape(ctx.title);
    let full_title = escape(&full_title);
    let page_path = escape(ctx.page_path);
    let page_og_type = escape(ctx.page_og_type);

    // 文章级：非文章页塞全空 `PostData`（`{{post.title}}` 输出空串，与旧行为一致）。
    let post = match &ctx.post {
        Some(p) => PostData {
            title: escape(p.title),
            date: escape(p.date),
            updated: escape(p.updated),
            excerpt: escape(p.excerpt),
            slug: escape(p.slug),
            tags: escape(&p.tags.join(", ")),
            categories: escape(&p.categories.join(", ")),
            series: escape(p.series),
            raw: p.raw.iter().map(|(k, v)| (k.clone(), escape(v))).collect(),
        },
        None => PostData {
            title: String::new(),
            date: String::new(),
            updated: String::new(),
            excerpt: String::new(),
            slug: String::new(),
            tags: String::new(),
            categories: String::new(),
            series: String::new(),
            raw: HashMap::new(),
        },
    };

    let posts = ctx
        .posts
        .iter()
        .map(|s| PostSummary {
            url: escape(&s.url),
            title: escape(&s.title),
            date: escape(&s.date),
            tags: escape(&s.tags),
            excerpt: escape(&s.excerpt),
        })
        .collect();

    let data = ThemeData {
        site_title: site_title.clone(),
        site_description: site_description.clone(),
        site_author: site_author.clone(),
        site_url: site_url.clone(),
        language: language.clone(),
        title: title.clone(),
        full_title: full_title.clone(),
        body: ctx.body.to_string(),
        math_style: ctx.math_style.to_string(),
        nav_extra: ctx.nav_extra.to_string(),
        seo_head: ctx.seo_head.to_string(),
        analytics: ctx.analytics.to_string(),
        site: SiteData {
            title: site_title,
            description: site_description,
            author: site_author,
            url: site_url,
            language,
        },
        page: PageData {
            title,
            full_title,
            path: page_path,
            og_type: page_og_type,
            current: ctx.page_current,
            total: ctx.page_total,
        },
        post,
        posts,
        params: ctx.params.clone(),
    };

    let context = tera::Context::from_serialize(&data)?;
    tera::Tera::one_off(&theme.template, &context, false).map_err(Into::into)
}

/// 文章列表条目。
///
/// 优先用主题 `partials/post_item.html`，缺失回退 [`DEFAULT_POST_ITEM`]。
/// 除规格书 §4.3 的 `{{url}}`/`{{title}}`/`{{date}}`/`{{tags}}`/`{{excerpt}}` 外，
/// 还提供预拼好的 `{{date_html}}`/`{{excerpt_html}}`/`{{tags_html}}`——P1 阶段模板
/// 尚无 `{% if %}`，主题要靠这些「空值即空串」的变量实现条件显示。
pub fn post_list_item(
    theme: &Theme,
    href: &str,
    title: &str,
    date: &str,
    tags: &[String],
    excerpt: &str,
) -> anyhow::Result<String> {
    let tags_html = if tags.is_empty() {
        String::new()
    } else {
        let tags: Vec<String> = tags
            .iter()
            .map(|t| format!(r#"<span class="tag">{}</span>"#, escape(t)))
            .collect();
        format!(r#"<div class="post-tags">{}</div>"#, tags.join(""))
    };
    let date_html = if date.is_empty() {
        String::new()
    } else {
        format!(r#"<div class="post-date">{}</div>"#, escape(date))
    };
    let excerpt_html = if excerpt.is_empty() {
        String::new()
    } else {
        format!(r#"<p class="post-excerpt">{}</p>"#, escape(excerpt))
    };
    let tags_text = tags.join(", ");

    let text: &[(&str, &str)] = &[
        ("url", href),
        ("title", title),
        ("date", date),
        ("excerpt", excerpt),
        ("tags", tags_text.as_str()),
    ];
    let html: &[(&str, &str)] = &[
        ("date_html", date_html.as_str()),
        ("excerpt_html", excerpt_html.as_str()),
        ("tags_html", tags_html.as_str()),
    ];
    theme.render_partial("post_item", DEFAULT_POST_ITEM, text, html)
}

/// 渲染整篇列表 partial（`partials/post_list.html`，P2 后新增）。
///
/// 传入文章摘要集合 `posts`（`{% for post in posts %}`）与 `count`；`tags` 为逗号拼接字符串。
/// 由 [`build::render_post_list`] 在主题提供了该 partial 时调用；未提供时走 Rust 兼容路径。
pub fn render_post_list_partial(
    theme: &Theme,
    posts: &[PostSummaryRef],
) -> anyhow::Result<String> {
    let tpl = theme.partial("post_list").unwrap_or(DEFAULT_POST_LIST);
    let mut context = tera::Context::new();
    context.insert("posts", posts);
    context.insert("count", &posts.len().to_string());
    tera::Tera::one_off(tpl, &context, false).map_err(Into::into)
}

/// 渲染文章元信息 partial（`partials/meta.html`，P2 后新增）。
///
/// `date` / `updated` 为 `Option`（无则空串）；`tags` / `categories` 为字符串切片（转义后拼
/// `<a class="tag">` / `<a class="category">` 链接）。提供 text 键 `date`/`updated` 与 html 键
/// `date_html`/`updated_html`/`tags_html`/`categories_html`（空值即空串）。
/// 由 [`build::render_article`] 在主题提供了该 partial 时调用；未提供时走 Rust 兼容路径。
pub fn render_meta_partial(
    theme: &Theme,
    date: Option<&str>,
    updated: Option<&str>,
    tags: &[String],
    categories: &[String],
) -> anyhow::Result<String> {
    let date_html = date
        .map(|d| format!(r#"<span>{}</span>"#, escape(d)))
        .unwrap_or_default();
    let updated_html = updated
        .map(|u| {
            format!(
                r#"<span class="updated" title="最后更新">更新于 {}</span>"#,
                escape(u)
            )
        })
        .unwrap_or_default();
    let tags_html = if tags.is_empty() {
        String::new()
    } else {
        let links: Vec<String> = tags
            .iter()
            .map(|t| format!(r#"<a class="tag" href="/tags/{}/">{}</a>"#, slugify(t), escape(t)))
            .collect();
        links.join("")
    };
    let categories_html = if categories.is_empty() {
        String::new()
    } else {
        let links: Vec<String> = categories
            .iter()
            .map(|c| {
                format!(
                    r#"<a class="category" href="/categories/{}/">{}</a>"#,
                    slugify(c),
                    escape(c)
                )
            })
            .collect();
        links.join("")
    };

    let date = date.unwrap_or("");
    let updated = updated.unwrap_or("");
    let text: &[(&str, &str)] = &[("date", date), ("updated", updated)];
    let html: &[(&str, &str)] = &[
        ("date_html", date_html.as_str()),
        ("updated_html", updated_html.as_str()),
        ("tags_html", tags_html.as_str()),
        ("categories_html", categories_html.as_str()),
    ];
    theme.render_partial("meta", DEFAULT_META, text, html)
}

/// 把 `{{params.<key>}}` 占位符替换为主题参数值。
fn apply_params(template: &str, params: &toml::Table) -> String {
    let mut out = template.to_string();
    for (key, val) in flatten_params(params) {
        out = out.replace(&format!("{{{{params.{key}}}}}"), &val);
    }
    out
}

/// 将嵌套的 `toml::Table` 拍平为「点号路径 → 字符串」的键值对。
///
/// 叶子值（字符串/整数/浮点/布尔/日期时间）转为字符串；数组以 `, ` 连接；嵌套表递归展开。
fn flatten_params(table: &toml::Table) -> Vec<(String, String)> {
    fn value_to_string(v: &toml::Value) -> String {
        match v {
            toml::Value::String(s) => s.clone(),
            toml::Value::Integer(i) => i.to_string(),
            toml::Value::Float(f) => f.to_string(),
            toml::Value::Boolean(b) => b.to_string(),
            toml::Value::Datetime(d) => d.to_string(),
            toml::Value::Array(a) => a.iter().map(value_to_string).collect::<Vec<_>>().join(", "),
            toml::Value::Table(_) => String::new(),
        }
    }

    fn walk(prefix: &str, table: &toml::Table, out: &mut Vec<(String, String)>) {
        for (k, v) in table {
            let key = if prefix.is_empty() {
                k.clone()
            } else {
                format!("{prefix}.{k}")
            };
            match v {
                toml::Value::Table(t) => walk(&key, t, out),
                other => out.push((key, value_to_string(other))),
            }
        }
    }

    let mut out = Vec::new();
    walk("", table, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// class 契约清单（THEME-SPEC §7 / §9 挂载点）——内置默认主题与
    /// `themes/` 下四套画廊主题的 style.css 都必须完整覆盖，防止任何一方
    /// 在改版时悄悄丢掉某个挂载点导致页面局部退回无样式状态。
    #[test]
    fn gallery_themes_cover_class_contract() {
        let required: &[&str] = &[
            // 布局骨架
            ".site-header", ".inner", ".site-title", ".site-nav", ".skip-link",
            ".site-footer", ".footer-brand", ".footer-meta", ".accent-dot",
            // 文章列表
            ".post-list", ".post-item", ".post-title", ".post-date",
            ".post-excerpt", ".post-tags", ".meta .tag", ".updated",
            // TOC
            ".toc", ".toc-list", ".toc-l2", ".toc-l3",
            // 上/下一篇与专栏
            ".post-nav", ".series-nav", ".series-home", ".series-pos", ".series-links",
            // 分页 / 标签云 / 搜索
            ".pagination", ".current",
            ".tag-cloud", ".tag-cloud-item", ".count",
            ".search-input", ".search-hint",
            // 集合页标题
            "main > h1", "main > h2",
            // 正文契约（§9）
            "article blockquote", "article pre", ":not(pre) > code",
            "article table", "article hr", "article img",
            "div[data-equation=\"block\"]", ".eq-num",
            "math[display=\"block\"]", "mfrac",
            // 工具类
            ".hidden",
        ];
        let tokens: &[&str] = &[
            "--bg:", "--surface:", "--fg:", "--accent:", "--quote:",
            "--border:", "--code-bg:", "--maxw:", "--serif:", "--sans:", "--mono:",
        ];
        let gallery = ["obsidian", "paper", "minimal", "citrus"];

        let mut cases: Vec<(String, String)> =
            vec![("default(墨理)".into(), DEFAULT_CSS.to_string())];
        for name in gallery {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("themes")
                .join(name)
                .join("style.css");
            let css = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("画廊主题 {name} 缺少 style.css: {e}"));
            cases.push((name.into(), css));
        }

        for (name, css) in &cases {
            let missing: Vec<&str> =
                required.iter().copied().filter(|sel| !css.contains(sel)).collect();
            assert!(missing.is_empty(), "主题 {name} 缺少 class 契约: {missing:?}");
            let missing_tokens: Vec<&str> =
                tokens.iter().copied().filter(|t| !css.contains(t)).collect();
            assert!(
                missing_tokens.is_empty(),
                "主题 {name} 缺少设计令牌变量: {missing_tokens:?}"
            );
        }
    }

    #[test]
    fn escape_handles_html_special_chars() {
        assert_eq!(
            escape("<a & 'b' \"c\">"),
            "&lt;a &amp; &#39;b&#39; &quot;c&quot;&gt;"
        );
        assert_eq!(escape("普通文本"), "普通文本");
    }

    #[test]
    fn flatten_params_expands_nested_tables() {
        let mut table = toml::Table::new();
        table.insert("accent".into(), toml::Value::String("#ff5500".into()));
        let mut font = toml::Table::new();
        font.insert("family".into(), toml::Value::String("serif".into()));
        table.insert("font".into(), toml::Value::Table(font));
        table.insert("sizes".into(), toml::Value::Array(vec![
            toml::Value::Integer(1),
            toml::Value::Integer(2),
        ]));

        let flat: std::collections::HashMap<_, _> = flatten_params(&table).into_iter().collect();
        assert_eq!(flat.get("accent").map(String::as_str), Some("#ff5500"));
        assert_eq!(flat.get("font.family").map(String::as_str), Some("serif"));
        assert_eq!(flat.get("sizes").map(String::as_str), Some("1, 2"));
    }

    #[test]
    fn apply_params_replaces_placeholders() {
        let mut table = toml::Table::new();
        table.insert("accent_color".into(), toml::Value::String("#ff5500".into()));
        let out = apply_params("--accent: {{params.accent_color}};", &table);
        assert_eq!(out, "--accent: #ff5500;");
    }

    fn make_theme(template: &str) -> Theme {
        Theme {
            name: "default".into(),
            css: DEFAULT_CSS.into(),
            template: template.into(),
            static_dir: None,
            partials: HashMap::new(),
        }
    }

    fn base_ctx<'a>(
        params: &'a toml::Table,
        post: Option<PostVars<'a>>,
    ) -> PageContext<'a> {
        PageContext {
            site_title: "站点",
            site_description: "描述",
            site_author: "张三",
            site_url: "https://example.com",
            language: "zh-CN",
            title: "文章",
            body: "<p>内容</p>",
            post,
            math_style: "",
            nav_extra: "",
            seo_head: "",
            analytics: "",
            params,
            page_path: "/posts/test/",
            page_og_type: "article",
            page_current: 1,
            page_total: 1,
            posts: &[],
        }
    }

    #[test]
    fn render_page_fills_text_and_html_placeholders() {
        let theme = make_theme(DEFAULT_TEMPLATE);
        let params = toml::Table::new();
        let html = render_page(&theme, &base_ctx(&params, None)).unwrap();
        assert!(html.contains("<title>文章 · 站点</title>"));
        assert!(html.contains("<p>内容</p>"));
    }

    // ===== P0：数据出口（规格书 §3） =====

    #[test]
    fn p0_site_author_and_url_are_exposed() {
        let theme = make_theme("A={{site_author}}|U={{site_url}}");
        let params = toml::Table::new();
        let html = render_page(&theme, &base_ctx(&params, None)).unwrap();
        assert_eq!(html, "A=张三|U=https://example.com");
    }

    #[test]
    fn p0_site_level_values_are_escaped() {
        let theme = make_theme("{{site_author}}");
        let params = toml::Table::new();
        let mut ctx = base_ctx(&params, None);
        ctx.site_author = "<b>&x</b>";
        assert_eq!(render_page(&theme, &ctx).unwrap(), "&lt;b&gt;&amp;x&lt;/b&gt;");
    }

    #[test]
    fn p0_post_vars_are_filled_on_article_pages() {
        let theme = make_theme(
            "{{post.title}}|{{post.date}}|{{post.updated}}|{{post.slug}}|{{post.tags}}|{{post.categories}}|{{post.excerpt}}",
        );
        let params = toml::Table::new();
        let tags = vec!["Rust".to_string(), "Typst".to_string()];
        let cats = vec!["技术".to_string()];
        let raw = vec![("cover".to_string(), "/img/a.png".to_string())];
        let post = PostVars {
            title: "标题",
            date: "2026-09-01",
            updated: "2026-09-03",
            excerpt: "摘要",
            slug: "hello",
            tags: &tags,
            categories: &cats,
            series: "高中物理",
            raw: &raw,
        };
        let html = render_page(&theme, &base_ctx(&params, Some(post))).unwrap();
        assert_eq!(html, "标题|2026-09-01|2026-09-03|hello|Rust, Typst|技术|摘要");
    }

    #[test]
    fn p0_post_series_is_exposed_and_emptied_on_non_article_pages() {
        let theme = make_theme("[{{post.series}}]");
        let params = toml::Table::new();
        let raw: Vec<(String, String)> = Vec::new();
        let post = PostVars {
            title: "",
            date: "",
            updated: "",
            excerpt: "",
            slug: "",
            tags: &[],
            categories: &[],
            series: "高中物理",
            raw: &raw,
        };
        assert_eq!(render_page(&theme, &base_ctx(&params, Some(post))).unwrap(), "[高中物理]");
        assert_eq!(render_page(&theme, &base_ctx(&params, None)).unwrap(), "[]");
    }

    #[test]
    fn p0_post_raw_custom_field_is_exposed() {
        let theme = make_theme("cover={{post.raw.cover}}");
        let params = toml::Table::new();
        let raw = vec![("cover".to_string(), "/img/a.png".to_string())];
        let post = PostVars {
            title: "",
            date: "",
            updated: "",
            excerpt: "",
            slug: "",
            tags: &[],
            categories: &[],
            series: "",
            raw: &raw,
        };
        assert_eq!(
            render_page(&theme, &base_ctx(&params, Some(post))).unwrap(),
            "cover=/img/a.png"
        );
    }

    #[test]
    fn p0_post_raw_is_escaped_against_xss() {
        let theme = make_theme("{{post.raw.cover}}");
        let params = toml::Table::new();
        let raw = vec![("cover".to_string(), "<script>&".to_string())];
        let post = PostVars {
            title: "",
            date: "",
            updated: "",
            excerpt: "",
            slug: "",
            tags: &[],
            categories: &[],
            series: "",
            raw: &raw,
        };
        assert_eq!(
            render_page(&theme, &base_ctx(&params, Some(post))).unwrap(),
            "&lt;script&gt;&amp;"
        );
    }

    #[test]
    fn p0_post_placeholders_are_emptied_on_non_article_pages() {
        let theme = make_theme("[{{post.title}}][{{post.date}}][{{post.tags}}]");
        let params = toml::Table::new();
        let html = render_page(&theme, &base_ctx(&params, None)).unwrap();
        // 固定字段清空，且不留字面量占位符
        assert_eq!(html, "[][][]");
    }

    #[test]
    fn p2_missing_raw_key_in_interpolation_errors() {
        // tera 严格语义：`{{ }}` 直接引用缺失键会报错（非文章页 post.raw 为空映射）。
        // 主题应改用 `{% if post.raw.cover %}` 守卫（见 p2_if_condition_on_raw_field），
        // 这比 P0 的 String::replace「原样保留字面量」更安全——把模板拼写错误暴露在构建期。
        let theme = make_theme("{{post.raw.cover}}");
        let params = toml::Table::new();
        assert!(render_page(&theme, &base_ctx(&params, None)).is_err());
    }

    #[test]
    fn post_list_item_escapes_and_includes_all_parts() {
        let theme = Theme::builtin();
        let item = post_list_item(&theme, "/posts/test/", "标题 <script>", "2026-09-01", &["标签A".into()], "这是摘要").unwrap();
        assert!(item.contains("href=\"/posts/test/\""));
        assert!(item.contains("标题 &lt;script&gt;"));
        assert!(item.contains("2026-09-01"));
        assert!(item.contains("标签A"));
        assert!(item.contains("这是摘要"));
        assert!(item.contains("post-excerpt"));
        assert!(item.contains("post-item"));
    }

    #[test]
    fn post_list_item_no_tags_no_date() {
        let theme = Theme::builtin();
        let item = post_list_item(&theme, "/p/", "T", "", &[], "").unwrap();
        assert!(!item.contains("post-date"));
        assert!(!item.contains("post-tags"));
        assert!(!item.contains("post-excerpt"));
    }

    // ===== P1：片段模板下沉（规格书 §4） =====

    #[test]
    fn render_partial_falls_back_to_default_when_unprovided() {
        // 内置默认主题无 partials/，任何片段名都回退到传入的 default。
        let theme = Theme::builtin();
        let out = theme.render_partial("post_item", "<li>{{title}}</li>", &[("title", "X")], &[]).unwrap();
        assert_eq!(out, "<li>X</li>");
    }

    #[test]
    fn render_partial_uses_theme_override() {
        // 主题提供 partials/post_item.html 时优先使用它。
        let mut p = HashMap::new();
        p.insert(
            "post_item".to_string(),
            r#"<div class="custom">{{title}}@{{date}}</div>"#.to_string(),
        );
        let theme = Theme::with_partials(p);
        let out = theme
            .render_partial(
                "post_item",
                "<li>{{title}}</li>",
                &[("title", "标题"), ("date", "2026")],
                &[],
            )
            .unwrap();
        assert_eq!(out, r#"<div class="custom">标题@2026</div>"#);
    }

    #[test]
    fn post_list_item_uses_custom_partial_override() {
        // 主题 `post_item` 片段覆盖后，渲染出的列表项结构随之改变。
        let mut p = HashMap::new();
        p.insert(
            "post_item".to_string(),
            r#"<li class="x">{{url}}|{{title}}</li>"#.to_string(),
        );
        let theme = Theme::with_partials(p);
        let item = post_list_item(&theme, "/p/", "T", "", &[], "").unwrap();
        assert_eq!(item, r#"<li class="x">/p/|T</li>"#);
    }

    #[test]
    fn render_partial_html_values_are_injected_raw() {
        // html 类变量（如预拼好的 date_html）原样注入，不被转义。
        let mut p = HashMap::new();
        p.insert(
            "post_item".to_string(),
            "<li>{{date_html}}</li>".to_string(),
        );
        let theme = Theme::with_partials(p);
        let item = post_list_item(&theme, "/p/", "T", "2026-09-01", &[], "").unwrap();
        assert_eq!(item, r#"<li><div class="post-date">2026-09-01</div></li>"#);
    }

    // ===== P2：tera 模板引擎（规格书 §5） =====

    #[test]
    fn p2_posts_loop_renders_each_item() {
        let theme = make_theme("{% for p in posts %}[{{p.title}}|{{p.url}}]{% endfor %}");
        let params = toml::Table::new();
        let summaries = vec![
            PostSummaryRef {
                url: "/a/".into(),
                title: "甲".into(),
                date: "2026-09-01".into(),
                tags: "Rust".into(),
                excerpt: "".into(),
            },
            PostSummaryRef {
                url: "/b/".into(),
                title: "乙".into(),
                date: "2026-09-02".into(),
                tags: "".into(),
                excerpt: "".into(),
            },
        ];
        let mut ctx = base_ctx(&params, None);
        ctx.posts = &summaries;
        let html = render_page(&theme, &ctx).unwrap();
        assert_eq!(html, "[甲|/a/][乙|/b/]");
    }

    #[test]
    fn p2_if_condition_on_raw_field() {
        // 有 cover 的文章输出 img；非文章页 raw 为空映射，条件为假跳过。
        let theme = make_theme(
            r#"{% if post.raw.cover %}<img src="{{post.raw.cover}}">{% endif %}END"#,
        );
        let params = toml::Table::new();
        let raw = vec![("cover".to_string(), "/img/a.png".to_string())];
        let post = PostVars {
            title: "",
            date: "",
            updated: "",
            excerpt: "",
            slug: "",
            tags: &[],
            categories: &[],
            series: "",
            raw: &raw,
        };
        let html = render_page(&theme, &base_ctx(&params, Some(post))).unwrap();
        assert_eq!(html, r#"<img src="/img/a.png">END"#);

        let empty = render_page(&theme, &base_ctx(&params, None)).unwrap();
        assert_eq!(empty, "END");
    }

    // ===== P2 后新增：post_list.html / meta.html 整块 partial =====

    #[test]
    fn post_list_partial_default_renders_loop() {
        // 未提供 post_list.html 时用 DEFAULT_POST_LIST，等价于 Rust 逐条拼接。
        let theme = Theme::builtin();
        let summaries = vec![
            PostSummaryRef {
                url: "/a/".into(),
                title: "甲".into(),
                date: "2026-09-01".into(),
                tags: "Rust".into(),
                excerpt: "摘要".into(),
            },
            PostSummaryRef {
                url: "/b/".into(),
                title: "乙".into(),
                date: "".into(),
                tags: "".into(),
                excerpt: "".into(),
            },
        ];
        let html = render_post_list_partial(&theme, &summaries).unwrap();
        assert!(html.contains("post-list"));
        assert!(html.contains("甲"));
        assert!(html.contains("乙"));
        assert!(html.contains("post-excerpt"));
        // 空日期/空标签不渲染对应块
        assert_eq!(html.matches("post-date").count(), 1);
    }

    #[test]
    fn post_list_partial_uses_theme_override() {
        // 主题提供 post_list.html 时整体接管列表结构。
        let mut p = HashMap::new();
        p.insert(
            "post_list".to_string(),
            r#"<ol>{% for p in posts %}<li>{{loop.index}}/{{count}}:{{p.title}}</li>{% endfor %}</ol>"#
                .to_string(),
        );
        let theme = Theme::with_partials(p);
        let summaries = vec![
            PostSummaryRef {
                url: "/a/".into(),
                title: "甲".into(),
                date: "".into(),
                tags: "".into(),
                excerpt: "".into(),
            },
            PostSummaryRef {
                url: "/b/".into(),
                title: "乙".into(),
                date: "".into(),
                tags: "".into(),
                excerpt: "".into(),
            },
        ];
        let html = render_post_list_partial(&theme, &summaries).unwrap();
        assert_eq!(html, "<ol><li>1/2:甲</li><li>2/2:乙</li></ol>");
    }

    #[test]
    fn has_partial_distinguishes_override_from_default() {
        let builtin = Theme::builtin();
        assert!(!builtin.has_partial("post_list"));
        assert!(!builtin.has_partial("meta"));

        let mut p = HashMap::new();
        p.insert("meta".to_string(), "{{date}}".to_string());
        let theme = Theme::with_partials(p);
        assert!(theme.has_partial("meta"));
        assert!(!theme.has_partial("post_list"));
    }

    #[test]
    fn meta_partial_default_renders_meta() {
        let theme = Theme::builtin();
        let tags = vec!["Rust".to_string(), "Typst".to_string()];
        let html =
            render_meta_partial(&theme, Some("2026-09-01"), Some("2026-09-02"), &tags, &[])
                .unwrap();
        assert!(html.contains("2026-09-01"));
        assert!(html.contains("更新于 2026-09-02"));
        assert!(html.contains(r#"href="/tags/Rust/"#));
        assert!(html.contains(r#"href="/tags/Typst/"#));
        assert!(html.contains("class=\"tag\""));
    }

    #[test]
    fn meta_partial_empty_fields_render_empty() {
        let theme = Theme::builtin();
        let html = render_meta_partial(&theme, None, None, &[], &[]).unwrap();
        assert!(html.trim().is_empty());
    }

    #[test]
    fn meta_partial_uses_theme_override() {
        let mut p = HashMap::new();
        p.insert(
            "meta".to_string(),
            r#"<time>{{date}}</time>{{categories_html}}"#.to_string(),
        );
        let theme = Theme::with_partials(p);
        let cats = vec!["生活".to_string()];
        let html = render_meta_partial(&theme, Some("2026-09-01"), None, &[], &cats).unwrap();
        assert!(html.starts_with(r#"<time>2026-09-01</time>"#));
        assert!(html.contains(r#"href="/categories/生活/"#));
        assert!(html.contains("class=\"category\""));
    }

    // ===== 专栏 + analytics（P3） =====

    #[test]
    fn analytics_is_injected_unescaped() {
        // analytics 是原始 HTML（`<script>` 等），不得转义。
        let theme = make_theme("HEAD|{{analytics}}|END");
        let params = toml::Table::new();
        let mut ctx = base_ctx(&params, None);
        ctx.analytics = r#"<script>if (a && b) track();</script>"#;
        assert_eq!(
            render_page(&theme, &ctx).unwrap(),
            r#"HEAD|<script>if (a && b) track();</script>|END"#
        );
        // 未配置（空串）不产生多余内容。
        let empty = render_page(&theme, &base_ctx(&params, None)).unwrap();
        assert_eq!(empty, "HEAD||END");
    }

    #[test]
    fn render_partial_supports_series_nav_partial() {
        // 主题可用 partials/series_nav.html 整体接管专栏导航。
        let mut p = HashMap::new();
        p.insert(
            "series_nav".to_string(),
            r#"<div class="s">{{series_name}}#{{index}}/{{total}}{{prev_link}}{{next_link}}</div>"#
                .to_string(),
        );
        let theme = Theme::with_partials(p);
        let out = theme
            .render_partial(
                "series_nav",
                DEFAULT_SERIES_NAV,
                &[("series_name", "高中物理"), ("series_url", "/series/gao-zhong-wu-li/"), ("index", "2"), ("total", "3")],
                &[("prev_link", "<a class=\"prev\">A</a>"), ("next_link", "")],
            )
            .unwrap();
        assert_eq!(out, r#"<div class="s">高中物理#2/3<a class="prev">A</a></div>"#);
    }
}
