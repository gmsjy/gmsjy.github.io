//! 站点组装：文章页 / 集合页 / 分页 / 搜索 / feed / SEO / 预压缩 / 清理。


use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use serde::Serialize;

use crate::config::Config;
use crate::content::{self, DocumentMeta, MetaValue};
use crate::feed;
use crate::ir::CompiledDoc;
use crate::site_html::{postprocess_body, slugify};
use crate::theme::{self, PageContext, Theme};
use crate::writer::SiteWriter;


#[derive(Serialize)]
pub(crate) struct SearchEntry {
    title: String,
    url: String,
    date: String,
    tags: Vec<String>,
    excerpt: String,
    /// 正文纯文本（去标签、压缩空白），供全文检索。
    text: String,
}

/// 页面级 SEO 信息（canonical / og:type / 页面描述）。
pub(crate) struct SeoInfo<'a> {
    /// 站点相对路径（如 `/`、`/posts/foo/`），用于 canonical / og:url。
    path: &'a str,
    /// `"article"`（文章页）或 `"website"`（其余）。
    og_type: &'a str,
    /// 页面级描述（文章页传摘要），空串则回退站点描述。
    description: &'a str,
    /// 分享图绝对 URL（og:image / twitter 卡升级 large）。仅文章页携带，
    /// `None` 维持无图卡片（`summary`）。
    og_image: Option<&'a str>,
    /// 文章发布日期（JSON-LD datePublished），仅文章页携带。
    date: Option<&'a str>,
    /// 最后更新日期（JSON-LD dateModified），仅文章页携带。
    updated: Option<&'a str>,
}

/// 渲染完整页面（统一包装，避免每处重复拼 PageContext）。
///
/// `post` 为文章级变量，仅文章页传 `Some(..)`（规格书 §3.2 · P0）；
/// `page_current`/`page_total` 与 `posts` 为 P2 新增（`page.*` 与 `{% for %}` 数据源）。
#[allow(clippy::too_many_arguments)]
pub(crate) fn page(
    theme: &Theme,
    config: &Config,
    title: &str,
    body: &str,
    math_style: &str,
    nav: &str,
    seo: Option<&SeoInfo<'_>>,
    post: Option<theme::PostVars<'_>>,
    page_current: usize,
    page_total: usize,
    posts: &[theme::PostSummaryRef],
) -> anyhow::Result<String> {
    let (path, og_type) = seo.map(|s| (s.path, s.og_type)).unwrap_or(("", ""));
    let seo_head = seo
        .map(|s| build_seo_head(config, s.path, s.og_type, title, s.description, s.og_image, s.date, s.updated))
        .unwrap_or_default();
    theme::render_page(
        theme,
        &PageContext {
            site_title: &config.site.title,
            site_description: &config.site.description,
            site_author: &config.site.author,
            site_url: &config.site.url,
            language: &config.site.language,
            title,
            body,
            post,
            math_style,
            nav_extra: nav,
            seo_head: &seo_head,
            analytics: &config.site.analytics,
            params: &config.theme.params,
            page_path: path,
            page_og_type: og_type,
            page_current,
            page_total,
            posts,
        },
    )
}

/// 组装 SEO head 块（canonical + OG + Twitter Card）。
///
/// 站点未配置 `site.url` 时返回空串（整块省略，避免输出残缺标签）；
/// `description` 为空串时回退站点描述。
#[allow(clippy::too_many_arguments)]
pub(crate) fn build_seo_head(
    config: &Config,
    path: &str,
    og_type: &str,
    title: &str,
    description: &str,
    og_image: Option<&str>,
    date: Option<&str>,
    updated: Option<&str>,
) -> String {
    let url = config.site.url.trim();
    if url.is_empty() {
        return String::new();
    }
    let base = url.trim_end_matches('/');
    let canonical = format!("{base}{path}");
    let desc = if description.is_empty() {
        config.site.description.as_str()
    } else {
        description
    };
    let (image_tag, card) = match og_image {
        Some(img) => (
            format!(
                r#"<meta property="og:image" content="{img}">"#
            ),
            "summary_large_image",
        ),
        None => (String::new(), "summary"),
    };
    let mut head = format!(
        r#"<link rel="canonical" href="{canonical}">
<meta property="og:type" content="{og_type}">
<meta property="og:title" content="{title}">
<meta property="og:description" content="{desc}">
<meta property="og:url" content="{canonical}">{image_tag}
<meta name="twitter:card" content="{card}">"#,
        canonical = theme::escape(&canonical),
        og_type = theme::escape(og_type),
        title = theme::escape(title),
        desc = theme::escape(desc),
        image_tag = image_tag,
        card = card,
    );

    // JSON-LD 结构化数据（文章页）：供搜索引擎富摘要
    if og_type == "article" {
        let mut fields: Vec<String> = vec![
            r#""@context": "https://schema.org""#.to_string(),
            r#""@type": "BlogPosting""#.to_string(),
        ];
        fields.push(format!(r#""headline": {}"#, json_escape(title)));
        fields.push(format!(r#""url": {}"#, json_escape(&canonical)));
        fields.push(format!(
            r#""author": {{"@type": "Person", "name": {}}}"#,
            json_escape(&config.site.author)
        ));
        fields.push(format!(r#""description": {}"#, json_escape(desc)));
        if let Some(d) = date {
            fields.push(format!(r#""datePublished": {}"#, json_escape(d)));
        }
        if let Some(u) = updated {
            fields.push(format!(r#""dateModified": {}"#, json_escape(u)));
        }
        if let Some(img) = og_image {
            fields.push(format!(r#""image": {}"#, json_escape(img)));
        }
        head.push('\n');
        head.push_str(r#"<script type="application/ld+json">{"#);
        head.push_str(&fields.join(","));
        head.push_str("}</script>");
    }
    head
}

/// JSON 字符串转义：复用 serde_json 的字符串序列化（引号/控制符/Unicode 转义全部正确）。
fn json_escape(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_default()
}

/// 生成别名跳转页（`aliases` 旧地址 → 新文章地址）。
///
/// `meta refresh` 即时跳转 + canonical 指向新地址（对搜索引擎声明正主），
/// noscript 场景保留可点链接。目标为站内绝对路径（如 `/posts/new-slug/`）。
/// target 按 HTML 属性转义：slug/别名理论上可含 `"`、`&` 等字符，
/// 未转义会破坏属性边界（属性注入）。
pub(crate) fn redirect_page(target: &str) -> String {
    let target = theme::escape(target);
    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta http-equiv="refresh" content="0; url={target}">
<link rel="canonical" href="{target}">
<title>页面已迁移</title>
</head>
<body>
<p>页面已迁移到 <a href="{target}">{target}</a>。</p>
</body>
</html>"#
    )
}

/// 文章标题（缺省用 slug）。
pub(crate) fn post_title(p: &CompiledDoc) -> String {
    if p.meta.title.is_empty() {
        p.slug.clone()
    } else {
        p.meta.title.clone()
    }
}

/// 文章日期字符串。
pub(crate) fn post_date(p: &CompiledDoc) -> String {
    p.meta.date.clone().unwrap_or_default()
}

/// 生成文章列表。
///
/// 主题提供了 `partials/post_list.html` 时整体交给 tera 循环（`{% for post in posts %}`，
/// P2 后新增）；否则回退 Rust 逐条 `partials/post_item.html` 拼接（P1 行为，字节兼容）。
pub(crate) fn render_post_list(theme: &Theme, items: &[&CompiledDoc]) -> anyhow::Result<String> {
    if theme.has_partial("post_list") {
        let summaries = post_summaries(items);
        return theme::render_post_list_partial(theme, &summaries);
    }
    let mut html: Vec<String> = Vec::with_capacity(items.len());
    for p in items {
        html.push(theme::post_list_item(
            theme,
            &format!("/{}/", p.slug),
            &post_title(p),
            &post_date(p),
            &p.meta.tags,
            &p.meta.effective_excerpt(&p.body_html),
        )?);
    }
    Ok(format!(r#"<ul class="post-list">{}</ul>"#, html.join("\n")))
}

/// 列表页文章摘要集合（P2：塞进 `posts`，供模板 `{% for post in posts %}`）。
pub(crate) fn post_summaries(posts: &[&CompiledDoc]) -> Vec<theme::PostSummaryRef> {
    posts
        .iter()
        .map(|p| theme::PostSummaryRef {
            url: format!("/{}/", p.slug),
            title: post_title(p),
            date: post_date(p),
            tags: p.meta.tags.join(", "),
            excerpt: p.meta.effective_excerpt(&p.body_html),
        })
        .collect()
}

pub(crate) fn render_index(
    theme: &Theme,
    config: &Config,
    posts: &[CompiledDoc],
    nav: &str,
    page_num: usize,
    total_pages: usize,
) -> anyhow::Result<String> {
    let refs: Vec<&CompiledDoc> = posts.iter().collect();
    let list = format!(r#"<h1>文章</h1>{}"#, render_post_list(theme, &refs)?);
    let pagination = render_pagination(theme, page_num, total_pages)?;
    let body = format!("{}{}", list, pagination);
    let path = if page_num == 1 { "/" } else { &format!("/page/{}/", page_num) };
    let seo = SeoInfo { path, og_type: "website", description: "", og_image: None, date: None, updated: None };
    let summaries = post_summaries(&refs);
    page(theme, config, "", &body, "", nav, Some(&seo), None, page_num, total_pages, &summaries)
}

/// 分页导航（规格书 §4.3：`partials/pagination.html`）。
///
/// P1 阶段模板无 `{% if %}`，故额外提供「空值即空串」的 `{{prev_link}}` / `{{next_link}}`；
/// `{{prev_url}}` / `{{next_url}}` 则是纯文本，供主题自行排版。
pub(crate) fn render_pagination(theme: &Theme, current: usize, total: usize) -> anyhow::Result<String> {
    if total <= 1 {
        return Ok(String::new());
    }
    let prev_path = if current > 1 {
        if current == 2 {
            "/".to_string()
        } else {
            format!("/page/{}/", current - 1)
        }
    } else {
        String::new()
    };
    let next_path = if current < total {
        format!("/page/{}/", current + 1)
    } else {
        String::new()
    };
    let prev_link = if prev_path.is_empty() {
        String::new()
    } else {
        format!(
            r#"<a href="{href}">← 上一页</a>"#,
            href = theme::escape(&prev_path)
        )
    };
    let next_link = if next_path.is_empty() {
        String::new()
    } else {
        format!(
            r#"<a href="{href}">下一页 →</a>"#,
            href = theme::escape(&next_path)
        )
    };
    let cur = current.to_string();
    let tot = total.to_string();
    let text: &[(&str, &str)] = &[
        ("current", cur.as_str()),
        ("total", tot.as_str()),
        ("prev_url", prev_path.as_str()),
        ("next_url", next_path.as_str()),
    ];
    let html: &[(&str, &str)] = &[
        ("prev_link", prev_link.as_str()),
        ("next_link", next_link.as_str()),
    ];
    theme.render_partial("pagination", theme::DEFAULT_PAGINATION, text, html)
}

pub(crate) fn render_article(
    theme: &Theme,
    config: &Config,
    doc: &CompiledDoc,
    nav: &str,
    prev: Option<(&str, &str)>,
    next: Option<(&str, &str)>,
    series_nav: Option<&SeriesNav>,
) -> anyhow::Result<String> {
    let title = post_title(doc);

    // 文章元信息（date/updated/tags/categories）：主题提供 `partials/meta.html` 时整体接管
    // （P2 后新增）；否则走 Rust 拼接（字节兼容 P1 行为）。
    let meta_html = if theme.has_partial("meta") {
        theme::render_meta_partial(
            theme,
            doc.meta.date.as_deref(),
            doc.meta.updated.as_deref(),
            &doc.meta.tags,
            &doc.meta.categories,
        )?
    } else {
        let mut meta_parts = Vec::new();
        if let Some(date) = &doc.meta.date {
            meta_parts.push(format!("<span>{}</span>", theme::escape(date)));
        }
        if let Some(updated) = &doc.meta.updated {
            meta_parts.push(format!(
                r#"<span class="updated" title="最后更新">更新于 {}</span>"#,
                theme::escape(updated)
            ));
        }
        if !doc.meta.tags.is_empty() {
            let tags: Vec<String> = doc
                .meta
                .tags
                .iter()
                .map(|t| {
                    format!(
                        r#"<a class="tag" href="/tags/{}/">{}</a>"#,
                        slugify(t),
                        theme::escape(t)
                    )
                })
                .collect();
            meta_parts.push(tags.join(""));
        }
        meta_parts.join(" ")
    };

    let prev_next = render_prev_next(theme, prev, next)?;
    let series_nav_html = render_series_nav(theme, series_nav)?;

    // 正文后处理：注入 heading 锚点 + 提取 TOC + 代码高亮颜色变量化
    let (content, toc_items) = postprocess_body(&doc.body_html);
    // P1：TOC 外壳可覆盖（`partials/toc.html`），`{{items}}` 是后处理产出的 `<li>` 列表。
    let toc_html = if toc_items.is_empty() {
        String::new()
    } else {
        theme.render_partial("toc", theme::DEFAULT_TOC, &[], &[("items", toc_items.as_str())])?
    };

    let excerpt = doc.meta.effective_excerpt(&doc.body_html);
    let date = doc.meta.date.as_deref().unwrap_or("");
    let updated = doc.meta.updated.as_deref().unwrap_or("");
    let tags_text = doc.meta.tags.join(", ");
    let cats_text = doc.meta.categories.join(", ");

    // P1：文章外壳可覆盖（`partials/article.html`）。
    let text: &[(&str, &str)] = &[
        ("title", &title),
        ("date", date),
        ("updated", updated),
        ("tags", tags_text.as_str()),
        ("categories", cats_text.as_str()),
        ("excerpt", excerpt.as_str()),
    ];
    let html: &[(&str, &str)] = &[
        ("meta", meta_html.as_str()),
        ("toc", toc_html.as_str()),
        ("content", content.as_str()),
        ("series_nav", series_nav_html.as_str()),
        ("prev_next", prev_next.as_str()),
    ];
    let body = theme.render_partial("article", theme::DEFAULT_ARTICLE, text, html)?;

    let path = format!("/{}/", doc.slug);
    // P0：文章级模板变量（`{{post.*}}`），raw 自定义字段已按规格书 §3.3 拍平。
    let raw = flatten_meta_raw(&doc.meta);
    let series = doc.meta.series.as_deref().unwrap_or("");
    let post = theme::PostVars {
        title: &title,
        date,
        updated,
        excerpt: &excerpt,
        slug: &doc.slug,
        tags: &doc.meta.tags,
        categories: &doc.meta.categories,
        series,
        raw: &raw,
    };
    let og_image = og_image_for(doc, config);
    let seo = SeoInfo { path: &path, og_type: "article", description: &excerpt, og_image: og_image.as_deref(),
        date: doc.meta.date.as_deref(),
        updated: doc.meta.updated.as_deref(),
    };
    page(theme, config, &title, &body, &doc.math_style, nav, Some(&seo), Some(post), 1, 1, &[])
}

/// 文章页 og:image 取值链：正文首图（绝对化）→ 预生成的分享卡
/// `og/<slug>.png`（卡片本体在 build 主流程串行写入）。站点未配置
/// `site.url` 时无 og:image（与整块 seo head 省略的条件一致）。
pub(crate) fn og_image_for(doc: &CompiledDoc, config: &Config) -> Option<String> {
    let url = config.site.url.trim();
    if url.is_empty() {
        return None;
    }
    if let Some(src) = crate::social_card::first_image_path(&doc.body_html)
        && let Some(abs) = crate::social_card::absolutize_og_image(&doc.slug, &src, url) {
            return Some(abs);
        }
    let base = url.trim_end_matches('/');
    Some(format!("{base}/{}", og_card_filename(&doc.slug)))
}

/// 分享卡产物路径（slug 段分隔符扁平化，避免子目录）。
pub(crate) fn og_card_filename(slug: &str) -> String {
    format!("og/{}.png", slug.replace('/', "-"))
}

/// 主题强调色（分享卡配色用）：读 `[theme.params]` 的 `accent` /
/// `accent_color`（`#` 开头），缺失回退墨理「编辑青」。
pub(crate) fn theme_accent(config: &Config) -> String {
    for key in ["accent", "accent_color"] {
        if let Some(toml::Value::String(s)) = config.theme.params.get(key)
            && s.starts_with('#') {
                return s.clone();
            }
    }
    "#0b6fc0".into()
}

/// 把文章的自定义元数据（`DocumentMeta::raw`）拍平为「键 → 字符串」的键值对。
///
/// 供 `{{post.raw.<key>}}` 占位符消费（规格书 §3.3）。按键排序以保证输出确定性。
/// 拍平在这里完成，使 `theme` 模块无需依赖 `content::MetaValue`。
pub(crate) fn flatten_meta_raw(meta: &DocumentMeta) -> Vec<(String, String)> {
    fn value_to_string(v: &MetaValue) -> String {
        match v {
            MetaValue::Str(s) => s.clone(),
            MetaValue::Bool(b) => b.to_string(),
            MetaValue::Int(i) => i.to_string(),
            MetaValue::Float(f) => f.to_string(),
            MetaValue::Array(a) => a.iter().map(value_to_string).collect::<Vec<_>>().join(", "),
            MetaValue::None => String::new(),
        }
    }

    let mut out: Vec<(String, String)> = meta
        .raw
        .iter()
        .map(|(k, v)| (k.clone(), value_to_string(v)))
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// 上一篇 / 下一篇（规格书 §4.3：`partials/post_nav.html`）。
pub(crate) fn render_prev_next(theme: &Theme, prev: Option<(&str, &str)>, next: Option<(&str, &str)>) -> anyhow::Result<String> {
    if prev.is_none() && next.is_none() {
        return Ok(String::new());
    }
    let prev_link = match prev {
        Some((slug, title)) => format!(
            r#"<a class="prev" href="/{slug}/">← {title}</a>"#,
            slug = theme::escape(slug),
            title = theme::escape(title),
        ),
        None => String::new(),
    };
    let next_link = match next {
        Some((slug, title)) => format!(
            r#"<a class="next" href="/{slug}/">{title} →</a>"#,
            slug = theme::escape(slug),
            title = theme::escape(title),
        ),
        None => String::new(),
    };
    let text: &[(&str, &str)] = &[
        ("prev_url", prev.map(|(s, _)| s).unwrap_or("")),
        ("prev_title", prev.map(|(_, t)| t).unwrap_or("")),
        ("next_url", next.map(|(s, _)| s).unwrap_or("")),
        ("next_title", next.map(|(_, t)| t).unwrap_or("")),
    ];
    let html: &[(&str, &str)] = &[
        ("prev_link", prev_link.as_str()),
        ("next_link", next_link.as_str()),
    ];
    theme.render_partial("post_nav", theme::DEFAULT_POST_NAV, text, html)
}

/// 文章页专栏导航数据：同专栏内的位置与前后篇（课程顺序）。
#[derive(Debug)]
pub(crate) struct SeriesNav {
    /// 专栏名（如 `高中物理`）。
    pub(crate) series: String,
    /// 专栏详情页路径（如 `/series/高中物理/`）。
    pub(crate) series_url: String,
    /// 当前篇在专栏内的位置（1-based）。
    pub(crate) index: usize,
    pub(crate) total: usize,
    /// 专栏内上一篇（更靠课程前）：slug + 标题。
    pub(crate) prev: Option<(String, String)>,
    /// 专栏内下一篇（更靠课程后）。
    pub(crate) next: Option<(String, String)>,
}

/// 按专栏名分组文章，返回 `(专栏名, [posts 下标])`（名字典序）。
pub(crate) fn group_series(posts: &[CompiledDoc]) -> BTreeMap<&str, Vec<usize>> {
    let mut groups: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, p) in posts.iter().enumerate() {
        if let Some(s) = p.meta.series.as_deref() {
            groups.entry(s).or_default().push(i);
        }
    }
    groups
}

/// 专栏内排序键：权重升序（`series_weight` 缺省排最后）→ 日期升序。
///
/// 与站内列表「最新在前」的时间序相反——专栏是教材序列。无日期视为
/// 排最后（`9999-12-31` 哨兵，日期为 YYYY-MM-DD 字典序即时间序）；
/// 同键保持原有相对顺序（稳定排序）。
pub(crate) fn series_sort_key(post: &CompiledDoc) -> (i64, &str) {
    (
        post.meta.series_weight.unwrap_or(i64::MAX),
        post.meta.date.as_deref().unwrap_or("9999-12-31"),
    )
}

/// 为每篇文章预计算专栏导航（与 `posts` 平行对齐；非专栏文章为 `None`）。
///
/// 专栏名 → URL slug 出现冲突时直接报错：否则第二个专栏详情页会静默
/// 覆盖第一个，已生成文章的 `series_nav` 专栏链接随之悬空。
pub(crate) fn series_nav_index(posts: &[CompiledDoc]) -> anyhow::Result<Vec<Option<SeriesNav>>> {
    let groups = group_series(posts);

    // 先解析专栏 slug 并检测冲突（如「数学!」与「数学?」都解析为 `数学`）
    let mut slug_owner: BTreeMap<String, &str> = BTreeMap::new();
    let mut name_slug: BTreeMap<&str, String> = BTreeMap::new();
    for name in groups.keys() {
        let slug = slugify(name);
        if let Some(prev) = slug_owner.insert(slug.clone(), name) {
            anyhow::bail!(
                "专栏 slug 冲突：「{prev}」与「{name}」都解析为 /series/{slug}/，请修改其中一个专栏名"
            );
        }
        name_slug.insert(name, slug);
    }

    let mut navs: Vec<Option<SeriesNav>> = (0..posts.len()).map(|_| None).collect();
    for (name, mut idxs) in groups {
        idxs.sort_by_key(|&i| series_sort_key(&posts[i]));
        let total = idxs.len();
        let series_url = format!("/series/{}/", name_slug[name]);
        for (pos, &i) in idxs.iter().enumerate() {
            let prev = if pos > 0 {
                let p = &posts[idxs[pos - 1]];
                Some((p.slug.clone(), post_title(p)))
            } else {
                None
            };
            let next = if pos + 1 < total {
                let p = &posts[idxs[pos + 1]];
                Some((p.slug.clone(), post_title(p)))
            } else {
                None
            };
            navs[i] = Some(SeriesNav {
                series: name.to_string(),
                series_url: series_url.clone(),
                index: pos + 1,
                total,
                prev,
                next,
            });
        }
    }
    Ok(navs)
}

/// 专栏导航（规格书 §4.3：`partials/series_nav.html`）。非专栏文章返回空串。
pub(crate) fn render_series_nav(theme: &Theme, nav: Option<&SeriesNav>) -> anyhow::Result<String> {
    let Some(nav) = nav else {
        return Ok(String::new());
    };
    let prev_link = match &nav.prev {
        Some((slug, title)) => format!(
            r#"<a class="prev" href="/{slug}/">← {title}</a>"#,
            slug = theme::escape(slug),
            title = theme::escape(title),
        ),
        None => String::new(),
    };
    let next_link = match &nav.next {
        Some((slug, title)) => format!(
            r#"<a class="next" href="/{slug}/">{title} →</a>"#,
            slug = theme::escape(slug),
            title = theme::escape(title),
        ),
        None => String::new(),
    };
    let index = nav.index.to_string();
    let total = nav.total.to_string();
    let text: &[(&str, &str)] = &[
        ("series_name", nav.series.as_str()),
        ("series_url", nav.series_url.as_str()),
        ("index", index.as_str()),
        ("total", total.as_str()),
        ("prev_url", nav.prev.as_ref().map(|(s, _)| s.as_str()).unwrap_or("")),
        ("prev_title", nav.prev.as_ref().map(|(_, t)| t.as_str()).unwrap_or("")),
        ("next_url", nav.next.as_ref().map(|(s, _)| s.as_str()).unwrap_or("")),
        ("next_title", nav.next.as_ref().map(|(_, t)| t.as_str()).unwrap_or("")),
    ];
    let html: &[(&str, &str)] = &[
        ("prev_link", prev_link.as_str()),
        ("next_link", next_link.as_str()),
    ];
    theme.render_partial("series_nav", theme::DEFAULT_SERIES_NAV, text, html)
}

/// 集合页面导航链接（专栏 / 标签 / 分类 / 归档），注入默认模板的 `{{nav_extra}}`。
pub(crate) fn nav_links(posts: &[CompiledDoc]) -> String {
    let has_series = posts.iter().any(|p| p.meta.series.is_some());
    let has_tags = posts.iter().any(|p| !p.meta.tags.is_empty());
    let has_categories = posts.iter().any(|p| !p.meta.categories.is_empty());
    let mut s = String::new();
    if has_series {
        s.push_str(r#"<a href="/series/">专栏</a>"#);
    }
    if has_tags {
        s.push_str(r#"<a href="/tags/">标签</a>"#);
    }
    if has_categories {
        s.push_str(r#"<a href="/categories/">分类</a>"#);
    }
    s.push_str(r#"<a href="/archive/">归档</a>"#);
    s
}

/// 从日期字符串提取年月（`YYYY-MM`），无法识别则归入「未标注日期」。
pub(crate) fn year_month(date: &Option<String>) -> String {
    match date {
        Some(d) if !d.is_empty() => d.chars().take(7).collect(),
        _ => "未标注日期".to_string(),
    }
}

/// 标签/分类名 → URL slug 冲突检测（与 series 同口径）：slugify 会把
/// `!`/`?`/`+` 等折叠为 `-`，不同名字可能解析出同一路径；不拦截时两个
/// 集合页写同一路径，后写的静默覆盖先写的。
fn check_slug_conflicts(
    groups: &BTreeMap<String, Vec<&CompiledDoc>>,
    kind: &str,
    url_prefix: &str,
) -> anyhow::Result<()> {
    let mut slug_owner: BTreeMap<String, &str> = BTreeMap::new();
    for name in groups.keys() {
        let slug = slugify(name);
        if let Some(prev) = slug_owner.insert(slug.clone(), name) {
            anyhow::bail!(
                "{kind} slug 冲突：「{prev}」与「{name}」都解析为 /{url_prefix}/{slug}/，请修改其中一个名字"
            );
        }
    }
    Ok(())
}

/// 生成集合页面：标签云、分类页、日期归档（规格书 §2.1）。
pub(crate) fn generate_collections(
    theme: &Theme,
    config: &Config,
    posts: &[CompiledDoc],
    nav: &str,
    writer: &mut SiteWriter,
) -> anyhow::Result<()> {
    // 按标签 / 分类 / 年月 分组（BTreeMap 保证字典序，利于稳定输出）
    let mut tags: BTreeMap<String, Vec<&CompiledDoc>> = BTreeMap::new();
    let mut categories: BTreeMap<String, Vec<&CompiledDoc>> = BTreeMap::new();
    let mut archive: BTreeMap<String, Vec<&CompiledDoc>> = BTreeMap::new();
    for p in posts {
        for t in &p.meta.tags {
            tags.entry(t.clone()).or_default().push(p);
        }
        for c in &p.meta.categories {
            categories.entry(c.clone()).or_default().push(p);
        }
        archive.entry(year_month(&p.meta.date)).or_default().push(p);
    }

    // 标签云 + 每个标签页
    if !tags.is_empty() {
        // slug 冲突检测（与 series 同口径）：否则两个标签页写同一路径，
        // 后写的静默覆盖先写的，站内链接指向错误内容。
        check_slug_conflicts(&tags, "标签", "tags")?;
        let cloud = render_cloud(theme, "标签", "tags", &tags)?;
        let seo = SeoInfo { path: "/tags/", og_type: "website", description: "", og_image: None, date: None, updated: None };
        let html = page(theme, config, "标签", &cloud, "", nav, Some(&seo), None, 1, 1, &[])?;
        writer.write_str("tags/index.html", &html)?;
        for (tag, list) in &tags {
            let body = format!(
                r#"<h1>标签：{}</h1>{}"#,
                theme::escape(tag),
                render_post_list(theme, list)?
            );
            let path = format!("/tags/{}/", slugify(tag));
            let seo = SeoInfo { path: &path, og_type: "website", description: "", og_image: None, date: None, updated: None };
            let html = page(theme, config, &format!("标签：{tag}"), &body, "", nav, Some(&seo), None, 1, 1, &[])?;
            writer.write_str(&format!("tags/{}/index.html", slugify(tag)), &html)?;
        }
    }

    // 分类列表 + 每个分类页
    if !categories.is_empty() {
        check_slug_conflicts(&categories, "分类", "categories")?;
        let cloud = render_cloud(theme, "分类", "categories", &categories)?;
        let seo = SeoInfo { path: "/categories/", og_type: "website", description: "", og_image: None, date: None, updated: None };
        let html = page(theme, config, "分类", &cloud, "", nav, Some(&seo), None, 1, 1, &[])?;
        writer.write_str("categories/index.html", &html)?;
        for (cat, list) in &categories {
            let body = format!(
                r#"<h1>分类：{}</h1>{}"#,
                theme::escape(cat),
                render_post_list(theme, list)?
            );
            let path = format!("/categories/{}/", slugify(cat));
            let seo = SeoInfo { path: &path, og_type: "website", description: "", og_image: None, date: None, updated: None };
            let html = page(theme, config, &format!("分类：{cat}"), &body, "", nav, Some(&seo), None, 1, 1, &[])?;
            writer.write_str(&format!("categories/{}/index.html", slugify(cat)), &html)?;
        }
    }

    // 归档页（按年月倒序分组）
    if !archive.is_empty() {
        let body = render_archive(theme, &archive)?;
        let seo = SeoInfo { path: "/archive/", og_type: "website", description: "", og_image: None, date: None, updated: None };
        let html = page(theme, config, "归档", &body, "", nav, Some(&seo), None, 1, 1, &[])?;
        writer.write_str("archive/index.html", &html)?;
    }

    // 专栏列表 + 每个专栏详情页（详情按课程顺序：权重升序 → 日期升序，
    // 与标签页「最新在前」相反——专栏是教材序列，从第一课读到最后一课）
    let mut series: BTreeMap<String, Vec<&CompiledDoc>> = BTreeMap::new();
    for p in posts {
        if let Some(s) = &p.meta.series {
            series.entry(s.clone()).or_default().push(p);
        }
    }
    if !series.is_empty() {
        let cloud = render_cloud(theme, "专栏", "series", &series)?;
        let seo = SeoInfo { path: "/series/", og_type: "website", description: "", og_image: None, date: None, updated: None };
        let html = page(theme, config, "专栏", &cloud, "", nav, Some(&seo), None, 1, 1, &[])?;
        writer.write_str("series/index.html", &html)?;
        for (name, list) in series.iter_mut() {
            list.sort_by_key(|p| series_sort_key(p));
            let body = format!(
                r#"<h1>专栏：{}</h1>{}"#,
                theme::escape(name),
                render_post_list(theme, list)?
            );
            let path = format!("/series/{}/", slugify(name));
            let seo = SeoInfo { path: &path, og_type: "website", description: "", og_image: None, date: None, updated: None };
            let html = page(theme, config, &format!("专栏：{name}"), &body, "", nav, Some(&seo), None, 1, 1, &[])?;
            writer.write_str(&format!("series/{}/index.html", slugify(name)), &html)?;
        }
    }

    Ok(())
}

/// 标签云 / 分类云页面主体（按文章数倒序，其次字典序）。
pub(crate) fn render_cloud(
    theme: &Theme,
    title: &str,
    prefix: &str,
    map: &BTreeMap<String, Vec<&CompiledDoc>>,
) -> anyhow::Result<String> {
    let mut entries: Vec<(&String, &Vec<&CompiledDoc>)> = map.iter().collect();
    entries.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0)));
    let mut items = Vec::with_capacity(entries.len());
    for (name, list) in &entries {
        let url = format!("/{prefix}/{}/", slugify(name));
        let count = list.len().to_string();
        // P1：单个条目可覆盖（`partials/tag_cloud_item.html`）；循环在 Rust 侧。
        let text: &[(&str, &str)] = &[
            ("name", name.as_str()),
            ("url", url.as_str()),
            ("count", count.as_str()),
        ];
        items.push(theme.render_partial("tag_cloud_item", theme::DEFAULT_TAG_CLOUD_ITEM, text, &[])?);
    }
    Ok(format!(
        r#"<h1>{}</h1><div class="tag-cloud">{}</div>"#,
        theme::escape(title),
        items.join("\n")
    ))
}

/// 归档页主体（按年月倒序，最新在前；「未标注日期」排最后）。
pub(crate) fn render_archive(theme: &Theme, archive: &BTreeMap<String, Vec<&CompiledDoc>>) -> anyhow::Result<String> {
    let mut years: Vec<(&String, &Vec<&CompiledDoc>)> = archive.iter().collect();
    years.sort_by(|a, b| {
        // 「未标注日期」映射为最小值（排最后），其余按年月字符串倒序（字典序 = 时间序）
        let ka = if a.0 == "未标注日期" { "" } else { a.0.as_str() };
        let kb = if b.0 == "未标注日期" { "" } else { b.0.as_str() };
        kb.cmp(ka)
    });
    let mut sections = Vec::with_capacity(years.len());
    for (ym, list) in &years {
        sections.push(format!(
            r#"<h2>{}</h2>{}"#,
            theme::escape(ym),
            render_post_list(theme, list)?
        ));
    }
    Ok(format!(r#"<h1>归档</h1>{}"#, sections.join("\n")))
}

/// 复制 assets/ 静态资源到输出目录（跳过 style.css，已由主题写入）。
pub(crate) fn copy_static(root: &Path, writer: &mut SiteWriter) -> anyhow::Result<()> {
    let src = root.join("assets");
    if !src.exists() {
        return Ok(());
    }
    copy_dir_recursive(&src, "assets", writer, true)
}

/// 复制主题静态资源（`themes/<name>/static/`）到输出根目录。
pub(crate) fn copy_theme_static(theme: &Theme, writer: &mut SiteWriter) -> anyhow::Result<()> {
    if let Some(dir) = theme.static_dir() {
        copy_dir_recursive(dir, "", writer, false)?;
    }
    Ok(())
}

/// 为输出目录中的文本文件生成 `.gz` / `.br` 预压缩副本（跳过图片等二进制）。
///
/// 部署到支持预压缩静态服务的服务器（nginx `gzip_static`、Cloudflare /
/// Netlify 的 precompressed 文件）时可直接服务，减少传输体积。
/// 增量策略：产物已存在且不旧于源文件时跳过压缩（仅登记），
/// 避免每次构建重复压缩全部产物。生成的文件计入 `written` 清单，随 manifest 正常清理。
pub(crate) fn generate_precompressed(writer: &mut SiteWriter) -> anyhow::Result<()> {
    let t = precompress(writer, "gz", gzip_encode)?;
    let b = precompress(writer, "br", brotli_encode)?;
    if t.0 > 0 || t.1 > 0 || b.0 > 0 || b.1 > 0 {
        println!(
            "✅ 预压缩：.gz 新增 {} 个 / 跳过 {} 个，.br 新增 {} 个 / 跳过 {} 个",
            t.0, t.1, b.0, b.1
        );
    }
    Ok(())
}

/// 编码器统一签名：纯内存变换，供 [`precompress`] 并行调用。
type Encoder = fn(&[u8]) -> anyhow::Result<Vec<u8>>;

pub(crate) fn gzip_encode(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

pub(crate) fn brotli_encode(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    // quality 9：比 gzip fast 再省 ~15-20%；q=11 在百 KB 级 HTML 上耗时陡增，
    // 不适合默认构建路径（追求极致可后置为配置项）。
    let params = brotli::enc::BrotliEncoderParams {
        quality: 9,
        ..Default::default()
    };
    let mut out = Vec::new();
    brotli::BrotliCompress(&mut &data[..], &mut out, &params)?;
    Ok(out)
}

/// 为一种预压缩格式执行"筛选 → 增量判定 → 并行压缩 → 串行写盘"。
/// 返回 `(新增数, 跳过数)`。
pub(crate) fn precompress(
    writer: &mut SiteWriter,
    ext: &str,
    encode: Encoder,
) -> anyhow::Result<(usize, usize)> {
    const TEXT_EXTS: &[&str] = &["html", "css", "js", "json", "xml", "txt", "svg"];

    let out = writer.out.clone();
    // 第一遍：只遍历本次构建产出的文件（writer.written），过滤文本扩展名 + 增量判定
    // （产物不旧于源则跳过）。不再遍历整个输出目录——否则会把上次构建残留的孤儿
    // 文件（如下线的草稿文章）重新登记进清单，导致 clean_orphans 无法清理。
    let text_files: Vec<String> = writer
        .written
        .iter()
        .filter(|rel| {
            Path::new(rel)
                .extension()
                .and_then(|e| e.to_str())
                .map(|ext| TEXT_EXTS.contains(&ext))
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    let mut pending: Vec<(PathBuf, String)> = Vec::new();
    let mut skip_count = 0usize;
    for rel in &text_files {
        let path = out.join(rel);
        let enc_rel = format!("{rel}.{ext}");
        let enc_path = out.join(&enc_rel);

        // 增量：产物已存在且不旧于源文件 → 跳过（仍登记，防孤儿清理误删）
        let up_to_date = match (
            path.metadata().and_then(|m| m.modified()),
            enc_path.metadata().and_then(|m| m.modified()),
        ) {
            (Ok(src), Ok(dst)) => dst >= src,
            _ => false,
        };
        if up_to_date {
            writer.mark(&enc_rel);
            skip_count += 1;
            continue;
        }
        pending.push((path, enc_rel));
    }

    // 第二遍：并行压缩（读 + 压缩到内存，压缩为 CPU 密集可并行），主线程串行写盘。
    let results: Vec<anyhow::Result<(String, Vec<u8>, usize)>> = pending
        .par_iter()
        .map(|(path, enc_rel)| {
            let data = std::fs::read(path)?;
            let compressed = encode(&data)?;
            Ok((enc_rel.clone(), compressed, data.len()))
        })
        .collect();
    let mut new_count = 0usize;
    for r in results {
        let (enc_rel, compressed, src_len) = r?;
        // 压缩无收益（体积未变小）则跳过，避免徒增产物
        if compressed.len() >= src_len {
            continue;
        }
        writer.write(&enc_rel, &compressed)?;
        new_count += 1;
    }
    Ok((new_count, skip_count))
}

pub(crate) fn copy_dir_recursive(
    src: &Path,
    rel: &str,
    writer: &mut SiteWriter,
    skip_style_css: bool,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let child_rel = if rel.is_empty() {
            name.clone()
        } else {
            format!("{rel}/{name}")
        };
        if from.is_dir() {
            copy_dir_recursive(&from, &child_rel, writer, skip_style_css)?;
        } else if !(skip_style_css && name == "style.css") {
            let data = std::fs::read(&from)?;
            writer.write(&child_rel, &data)?;
        }
    }
    Ok(())
}

/// 生成站内搜索：`search.json`（全站文章纯文本索引）+ `/search/` 页面。
///
/// 检索端为零依赖内联 JS（不引入外部库），读取 `search.json` 后在标题/摘要/正文
/// 中做大小写不敏感的子串匹配。索引缺省生成，体积随文章量增长（纯文本）。
pub(crate) fn generate_search(
    theme: &Theme,
    config: &Config,
    posts: &[CompiledDoc],
    nav: &str,
    writer: &mut SiteWriter,
) -> anyhow::Result<()> {
    // 1. 构建索引（serde_json 序列化，仅文章；独立页面不入索引）
    let entries: Vec<SearchEntry> = posts
        .iter()
        .map(|p| SearchEntry {
            title: post_title(p),
            url: format!("/{}/", p.slug),
            date: p.meta.date.clone().unwrap_or_default(),
            tags: p.meta.tags.clone(),
            excerpt: p.meta.effective_excerpt(&p.body_html),
            text: content::plain_text(&p.body_html),
        })
        .collect();
    let json = serde_json::to_string_pretty(&entries)?;
    writer.write_str("search.json", &json)?;

    // 2. 搜索页（零依赖内联 JS）
    let search_body = r#"<h1>搜索</h1>
<input type="search" id="search-input" class="search-input" placeholder="输入关键词，即时检索…" autocomplete="off">
<p class="search-hint">匹配范围：标题 / 标签 / 摘要 / 正文全文</p>
<ul id="search-results" class="post-list"></ul>
<script>
(() => {
  const input = document.getElementById('search-input');
  const results = document.getElementById('search-results');
  let index = [];
  fetch('/search.json').then(r => r.json()).then(d => { index = d; }).catch(() => {
    results.innerHTML = '<li class="post-item">索引加载失败（请先运行 typall build）</li>';
  });
  const esc = s => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  const match = (q) => {
    const ql = q.toLowerCase();
    const out = [];
    for (const it of index) {
      const hay = (it.title + ' ' + it.tags.join(' ') + ' ' + it.excerpt + ' ' + it.text).toLowerCase();
      if (hay.includes(ql)) {
        out.push(`<li class="post-item"><a href="${esc(it.url)}"><div class="post-title">${esc(it.title)}</div>` +
          (it.date ? `<div class="post-date">${esc(it.date)}</div>` : '') +
          `<p class="post-excerpt">${esc(it.excerpt)}</p></a></li>`);
      }
    }
    return out;
  };
  input.addEventListener('input', () => {
    const q = input.value.trim();
    if (!q) { results.innerHTML = ''; return; }
    const hits = match(q);
    results.innerHTML = hits.length
      ? hits.slice(0, 50).join('\n')
      : '<li class="post-item">未找到匹配文章</li>';
  });
})();
</script>"#;
    let seo = SeoInfo { path: "/search/", og_type: "website", description: "", og_image: None, date: None, updated: None };
    let html = page(theme, config, "搜索", search_body, "", nav, Some(&seo), None, 1, 1, &[])?;
    writer.write_str("search/index.html", &html)?;

    println!("✅ 已生成 search.json（{} 篇）与 /search/ 页面", entries.len());
    Ok(())
}

/// 生成 SEO 基础文件：`robots.txt`（含 Sitemap 指引）与 `404.html`。
///
/// robots.txt 在配置了 `site.url` 时附带 `Sitemap:` 行；404 页始终生成，
/// 供静态托管（GitHub Pages / Netlify 等）自动使用。
pub(crate) fn generate_seo_files(
    theme: &Theme,
    config: &Config,
    nav: &str,
    writer: &mut SiteWriter,
) -> anyhow::Result<()> {
    let mut robots = String::from("User-agent: *\nAllow: /\n");
    let url = config.site.url.trim();
    if !url.is_empty() {
        let base = url.trim_end_matches('/');
        robots.push_str(&format!("Sitemap: {base}/sitemap.xml\n"));
    }
    writer.write_str("robots.txt", &robots)?;

    let body = r#"<h1>404</h1><p>页面不存在或已被移动。<a href="/">回到首页</a></p>"#;
    let seo = SeoInfo { path: "/404.html", og_type: "website", description: "", og_image: None, date: None, updated: None };
    let html = page(theme, config, "404", body, "", nav, Some(&seo), None, 1, 1, &[])?;
    writer.write_str("404.html", &html)?;

    println!("✅ 已生成 robots.txt 与 404.html");
    Ok(())
}

/// 生成 RSS/Atom 订阅源与 sitemap.xml（依赖 site.url）。
pub(crate) fn generate_feeds(
    config: &Config,
    writer: &mut SiteWriter,
    posts: &[CompiledDoc],
    pages: &[CompiledDoc],
) -> anyhow::Result<()> {
    let url = config.site.url.trim();
    if url.is_empty() {
        println!("ℹ️ 未配置 site.url，跳过 RSS 与 sitemap 生成");
        return Ok(());
    }
    let base = url.trim_end_matches('/');

    // Atom 订阅源
    let items: Vec<feed::FeedItem> = posts
        .iter()
        .map(|p| {
            let title = if p.meta.title.is_empty() {
                p.slug.clone()
            } else {
                p.meta.title.clone()
            };
            feed::FeedItem {
                title,
                link: format!("{base}/{}/", p.slug),
                date: p.meta.date.clone(),
                content: Some(p.body_html.clone()),
            }
        })
        .collect();
    // Atom 源级 <updated>：取最新文章的 updated/date（确定性，不随构建时间变化；
    // 否则每次构建 atom.xml 内容都变，破坏增量产物与订阅器缓存）。
    let updated = posts
        .iter()
        .filter_map(|p| p.meta.updated.clone().or_else(|| p.meta.date.clone()))
        .max()
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
    let atom = feed::atom_feed(&config.site.title, base, &updated, &config.site.author, &items);
    writer.write_str("atom.xml", &atom)?;

    // sitemap（附带 lastmod：优先 updated，回退 date）
    let mut urls: Vec<feed::SitemapEntry> = vec![feed::SitemapEntry {
        url: format!("{base}/"),
        lastmod: None,
    }];
    for p in posts {
        urls.push(feed::SitemapEntry {
            url: format!("{base}/{}/", p.slug),
            lastmod: p.meta.updated.clone().or_else(|| p.meta.date.clone()),
        });
    }
    for pg in pages {
        urls.push(feed::SitemapEntry {
            url: format!("{base}/{}/", pg.slug),
            lastmod: pg.meta.updated.clone().or_else(|| pg.meta.date.clone()),
        });
    }
    let sitemap = feed::sitemap(&urls);
    writer.write_str("sitemap.xml", &sitemap)?;

    println!("✅ 已生成 atom.xml 与 sitemap.xml");
    Ok(())
}

/// 检查输出站点的内部死链（仅检查 `/` 开头的页面链接）。
pub(crate) fn check_dead_links(out: &Path) -> anyhow::Result<Vec<String>> {
    use scraper::{Html, Selector};

    // 1. 收集所有生成页面的规范化 URL（去尾部斜杠、去 index.html）
    let mut valid: HashSet<String> = HashSet::new();
    valid.insert(String::new()); // 首页 "/"
    for entry in walkdir::WalkDir::new(out) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().is_some_and(|e| e != "html") {
            continue;
        }
        let rel = path.strip_prefix(out)?.to_string_lossy().replace('\\', "/");
        let url = if rel == "index.html" {
            String::new()
        } else if let Some(s) = rel.strip_suffix("/index.html") {
            format!("/{s}")
        } else if let Some(s) = rel.strip_suffix(".html") {
            format!("/{s}")
        } else {
            format!("/{rel}")
        };
        valid.insert(url);
    }

    // 2. 提取每个 HTML 的内部链接并校验
    let sel = Selector::parse("a[href]").unwrap();
    let mut broken = Vec::new();
    for entry in walkdir::WalkDir::new(out) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().is_some_and(|e| e != "html") {
            continue;
        }
        let html = std::fs::read_to_string(path)?;
        let doc = Html::parse_document(&html);
        for a in doc.select(&sel) {
            let Some(href) = a.value().attr("href") else { continue };
            if !href.starts_with('/') || href.starts_with("//") {
                continue;
            }
            let clean = href.split(['#', '?']).next().unwrap_or(href);
            if clean.is_empty() {
                continue;
            }
            // 跳过带扩展名的静态资源（css/js/图片/xml 等）
            let last = clean.rsplit('/').next().unwrap_or(clean);
            if last.contains('.') {
                continue;
            }
            let norm = clean.trim_end_matches('/');
            if !valid.contains(norm) {
                let src = path.strip_prefix(out).unwrap_or(path);
                broken.push(format!("{} → {href}", src.display()));
            }
        }
    }
    Ok(broken)
}

/// manifest 路径：项目根下 `.typall/manifest.txt`。
pub(crate) fn manifest_path(root: &Path) -> PathBuf {
    root.join(".typall").join("manifest.txt")
}

/// 读取上次构建的文件清单（不存在则空）。
pub(crate) fn read_manifest(root: &Path) -> HashSet<String> {
    std::fs::read_to_string(manifest_path(root))
        .map(|s| s.lines().map(String::from).collect())
        .unwrap_or_default()
}

/// 保存本次构建的文件清单。
pub(crate) fn write_manifest(root: &Path, files: &HashSet<String>) -> anyhow::Result<()> {
    let dir = root.join(".typall");
    std::fs::create_dir_all(&dir)?;
    let mut sorted: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    sorted.sort();
    std::fs::write(manifest_path(root), sorted.join("\n"))?;
    Ok(())
}

/// 删除上次构建有、本次构建没有的孤儿文件。
pub(crate) fn clean_orphans(out: &Path, old: &HashSet<String>, new: &HashSet<String>) -> anyhow::Result<()> {
    for f in old {
        if !new.contains(f) {
            let p = out.join(f);
            if p.is_file() {
                std::fs::remove_file(&p)?;
            }
        }
    }
    remove_empty_dirs(out)?;
    Ok(())
}

/// 递归删除孤儿文件清理后残留的空目录。
pub(crate) fn remove_empty_dirs(dir: &Path) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            remove_empty_dirs(&path)?;
            if std::fs::read_dir(&path)?.next().is_none() {
                std::fs::remove_dir(&path)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirect_page_escapes_target_into_attributes() {
        // 回归：target 此前未转义，含 `"` 的 slug 会突破 href 属性边界。
        let page = redirect_page(r#"/posts/a"b&c/"#);
        assert!(page.contains(r#"url=/posts/a&quot;b&amp;c/""#), "meta refresh 目标应转义: {page}");
        assert!(page.contains(r#"href="/posts/a&quot;b&amp;c/""#));
        // 不存在未转义的原始引号注入点
        assert!(!page.contains(r#"a"b"#));
        // 常规 target 原样可用
        let plain = redirect_page("/posts/new-slug/");
        assert!(plain.contains(r#"url=/posts/new-slug/""#));
        assert!(plain.contains(r#"<a href="/posts/new-slug/">"#));
    }
}

