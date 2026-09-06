//! 构建主流程：扫描 → 并行编译 → 组装 → 输出到 `public/`。
//!
//! 采用增量策略：不删除旧产物，直接覆盖写入，再用 manifest 清理孤儿文件。

use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use rayon::prelude::*;

use crate::config::Config;
use crate::content::{self};
use crate::ir::CompiledDoc;
use crate::theme::{self};
use crate::world::SharedAssets;

pub(crate) use crate::cache::{content_hash, hash_bytes};
pub(crate) use crate::compile::{compile_documents, ensure_project_root, filter_unpublished, format_diag, math_preamble, typst_lang};
pub(crate) use crate::site::*;
pub(crate) use crate::writer::SiteWriter;
pub(crate) use crate::compile::compile_doc;

/// 构建整个站点。
pub fn build(root: &Path, config: &Config, cli_include_drafts: bool) -> anyhow::Result<()> {
    // 草稿开关合并：typall.toml 的 [build] drafts（含 TP_BUILD_DRAFTS 环境变量）
    // 与 CLI --drafts 任一为真即包含草稿。
    let include_drafts = config.build.drafts || cli_include_drafts;
    let t_start = std::time::Instant::now();
    let theme = theme::Theme::load(root, config)?;
    if theme.name != "default" {
        println!("🎨 使用自定义主题: {}", theme.name);
    }

    let compiled = compile_documents(root, config)?;
    let mut posts = compiled.posts;
    let mut pages = compiled.pages;
    let t_compile = t_start.elapsed();

    // 过滤草稿/未来日期（pages 同样受 draft 约束，否则草稿页会泄入站点与 sitemap）
    filter_unpublished(&mut posts, include_drafts);
    filter_unpublished(&mut pages, include_drafts);
    posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));

    // 专栏导航按课程顺序互链，需在并行渲染前预计算（与 posts 平行对齐）
    let series_navs = series_nav_index(&posts)?;

    let out = config.output_dir(root);
    let old_files = read_manifest(root);
    let mut writer = SiteWriter::new(out.clone());

    // 主题 CSS（应用主题参数）
    writer.write_str("assets/style.css", &theme.css(&config.theme.params))?;

    // 集合页导航链接（标签/归档）
    let nav = nav_links(&posts);

    // 分享卡（og:image）：无正文首图的文章生成 1200×630 PNG，
    // 有首图的直接引用首图，不重复产卡。
    // 输入指纹缓存：resvg 每次栅格化都要加载字体（42 卡 ≈ 30s，增量构建的
    // 最大开销），标题/站点名/日期/配色未变时直接复用既有 PNG 跳过栅格化。
    if !config.site.url.trim().is_empty() {
        let accent = theme_accent(config);
        let card_map_path = root.join(".typall/cards.json");
        let mut card_map: std::collections::HashMap<String, String> = std::fs::read_to_string(&card_map_path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default();
        for post in &posts {
            if crate::social_card::first_image_path(&post.body_html).is_some() {
                continue;
            }
            let date = post.meta.date.clone().unwrap_or_default();
            let key = content_hash(&format!(
                "{}|{}|{}|{}|{}",
                post_title(post),
                config.site.title,
                date,
                accent,
                env!("CARGO_PKG_VERSION")
            ));
            if card_map.get(&post.slug) == Some(&key)
                && writer.out.join(og_card_filename(&post.slug)).exists()
            {
                // 复用既有卡片：仍需登记路径，否则孤儿清理会将其删除
                writer.mark(&og_card_filename(&post.slug));
                continue;
            }
            let png = crate::social_card::render_card_png(
                &post_title(post),
                &config.site.title,
                &date,
                &accent,
            )?;
            writer.write(&og_card_filename(&post.slug), &png)?;
            card_map.insert(post.slug.clone(), key);
        }
        if let Some(parent) = card_map_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&card_map_path, serde_json::to_string(&card_map)?)?;
    }

    // 首页（分页）
    let per_page = if config.build.posts_per_page > 0 {
        config.build.posts_per_page
    } else {
        20
    };
    let total_pages = if posts.is_empty() {
        1
    } else {
        posts.len().div_ceil(per_page)
    };
    for page_num in 1..=total_pages {
        let start = (page_num - 1) * per_page;
        let end = (start + per_page).min(posts.len());
        let chunk = &posts[start..end];
        let index_html = render_index(&theme, config, chunk, &nav, page_num, total_pages)?;
        let path = if page_num == 1 {
            "index.html".to_string()
        } else {
            format!("page/{}/index.html", page_num)
        };
        writer.write_str(&path, &index_html)?;
    }
    // 文章页（含上下篇导航）——并行渲染，串行写盘（IO 不并行，避免写盘竞争）
    // posts 按日期倒序：prev（更旧）= i+1，next（更新）= i-1
    let article_pages: Vec<(String, String)> = (0..posts.len())
        .into_par_iter()
        .map(|i| -> anyhow::Result<(String, String)> {
            let post = &posts[i];
            // 标题为动态生成的 String，需先绑定变量再借出，避免临时值悬垂（E0716）。
            let prev: Option<(String, String)> = posts.get(i + 1).map(|p| (p.slug.clone(), post_title(p)));
            let next: Option<(String, String)> = if i > 0 {
                let p = &posts[i - 1];
                Some((p.slug.clone(), post_title(p)))
            } else {
                None
            };
            let prev_ref = prev.as_ref().map(|(s, t)| (s.as_str(), t.as_str()));
            let next_ref = next.as_ref().map(|(s, t)| (s.as_str(), t.as_str()));
            let html = render_article(&theme, config, post, &nav, prev_ref, next_ref, series_navs[i].as_ref())?;
            Ok((format!("{}/index.html", post.slug), html))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    for (path, html) in &article_pages {
        writer.write_str(path, html)?;
    }

    // 旧地址重定向（aliases）：slug 变更后旧链接 301 语义落地（meta refresh），
    // 走 SiteWriter 登记 → 别名移除后随孤儿清理消失。
    for post in &posts {
        for alias in &post.meta.aliases {
            let target = format!("/{}/", post.slug);
            writer.write_str(&format!("{alias}/index.html"), &redirect_page(&target))?;
        }
    }

    // 独立页面（无上下篇导航）——并行渲染
    let page_pages: Vec<(String, String)> = pages
        .par_iter()
        .map(|page| -> anyhow::Result<(String, String)> {
            let html = render_article(&theme, config, page, &nav, None, None, None)?;
            Ok((format!("{}/index.html", page.slug), html))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    for (path, html) in &page_pages {
        writer.write_str(path, html)?;
    }
    let t_render = t_start.elapsed();

    // 集合页面（标签 / 分类 / 日期归档）
    generate_collections(&theme, config, &posts, &nav, &mut writer)?;

    // 站内搜索（search.json + /search/ 页面）
    generate_search(&theme, config, &posts, &nav, &mut writer)?;

    // SEO 基础文件（robots.txt + 404.html）
    generate_seo_files(&theme, config, &nav, &mut writer)?;

    // RSS + sitemap（需要 site.url）
    generate_feeds(config, &mut writer, &posts, &pages)?;

    // 复制 assets/ 静态资源（图片、字体等）
    copy_static(root, &mut writer)?;

    // 复制主题静态资源（themes/<name>/static/ → 输出根）
    copy_theme_static(&theme, &mut writer)?;
    let t_assets = t_start.elapsed();

    // 生成 .gz / .br 预压缩产物（文本类文件，供 nginx gzip_static 等服务）
    generate_precompressed(&mut writer)?;
    let t_gzip = t_start.elapsed();

    // 清理孤儿文件 + 保存 manifest
    clean_orphans(&out, &old_files, &writer.written)?;
    write_manifest(root, &writer.written)?;

    // 严格模式：死链检查
    if config.build.strict {
        let broken = check_dead_links(&out)?;
        if broken.is_empty() {
            println!("✅ 死链检查通过");
        } else {
            for b in &broken {
                eprintln!("死链: {b}");
            }
            anyhow::bail!("发现 {} 个死链", broken.len());
        }
    }

    // 清理孤儿缓存（源文件已删除，其缓存残留无意义）
    let valid: HashSet<String> = compiled
        .post_files
        .iter()
        .chain(compiled.page_files.iter())
        .map(|p| content::rel_path(root, p))
        .collect();
    compiled.cache.clean_orphans(&valid)?;

    println!(
        "✅ 构建完成：{} 篇文章，{} 个独立页面（增量：{} 命中缓存，{} 重新编译）→ {}",
        posts.len(),
        pages.len(),
        compiled.cache.hits(),
        compiled.cache.misses(),
        out.display()
    );
    println!(
        "⏱ 阶段耗时 —— 编译 {:.2}s · 文章/页面渲染+写盘 {:.2}s · 集合/搜索/SEO/Feed/静态 {:.2}s · 预压缩 {:.2}s · 收尾 {:.2}s · 总计 {:.2}s",
        t_compile.as_secs_f64(),
        (t_render - t_compile).as_secs_f64(),
        (t_assets - t_render).as_secs_f64(),
        (t_gzip - t_assets).as_secs_f64(),
        (t_start.elapsed() - t_gzip).as_secs_f64(),
        t_start.elapsed().as_secs_f64()
    );
    Ok(())
}

/// 编译单个 `.typ` 文件，返回正文 HTML + MathML 样式。
///
/// `renderer` 为 `svg` 时正文公式已是内联 SVG，MathML 对齐 CSS 无匹配元素，
pub fn check(root: &Path, strict: bool) -> anyhow::Result<()> {
    let config = crate::config::Config::load(root)?;
    let shared = Arc::new(SharedAssets::load(root)?);
    let preamble = math_preamble(&config, root);
    ensure_project_root(root)?;
    let mut files = content::scan_typ_files(&root.join("posts"));
    files.extend(content::scan_typ_files(&root.join("pages")));

    let results: Vec<anyhow::Result<CompiledDoc>> = files
        .par_iter()
        .map(|f| compile_doc(root, f, shared.clone(), &preamble, &config.build.math.renderer, &config, None))
        .collect();

    let mut ok = 0;
    let mut errors: Vec<String> = Vec::new();
    for r in results {
        match r {
            Ok(_) => ok += 1,
            Err(e) => errors.push(format!("{e:#}")),
        }
    }
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("❌ {e}");
        }
        anyhow::bail!("{} 个文件编译失败", errors.len());
    }

    // 严格模式：若存在构建产物则做死链检查
    if strict {
        let out = config.output_dir(root);
        if out.exists() {
            let broken = check_dead_links(&out)?;
            if broken.is_empty() {
                println!("✅ 死链检查通过");
            } else {
                for b in &broken {
                    eprintln!("死链: {b}");
                }
                anyhow::bail!("发现 {} 个死链", broken.len());
            }
        } else {
            println!("ℹ️ 严格模式：无构建产物，跳过死链检查（先 `typall build`）");
        }
    }

    println!("✅ 检查通过：{ok} 个文件无语法错误");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::CompileCache;
    use crate::content::DocumentMeta;
    use crate::theme::Theme;
    use std::collections::BTreeMap;
    use crate::cache::content_hash;
    use crate::compile::split_body_and_style;
    use crate::config::SiteConfig;
    use std::collections::HashMap;

    #[test]
    fn year_month_extracts_ym_or_marks_undated() {
        assert_eq!(year_month(&Some("2026-09-01".into())), "2026-09");
        assert_eq!(year_month(&None), "未标注日期");
        assert_eq!(year_month(&Some("".into())), "未标注日期");
    }

    #[test]
    fn content_hash_is_stable_and_distinct() {
        let a = content_hash("hello");
        let b = content_hash("hello");
        let c = content_hash("world");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.len(), 16);
    }

    #[test]
    fn compile_documents_rejects_wrong_working_dir() {
        // 既无 typall.toml 也无 posts/ 的目录 = 在错误目录启动，应报人话错误
        // 而不是静默构建 0 篇文章的空站点。
        let dir = std::env::temp_dir().join("typall-wrong-dir-test");
        std::fs::create_dir_all(&dir).unwrap();
        let config = Config::default();
        let err = match compile_documents(&dir, &config) {
            Err(e) => e,
            Ok(_) => panic!("空目录（无 typall.toml 与 posts/）应当被拒绝"),
        };
        assert!(err.to_string().contains("typall.toml"), "报错应提示 typall.toml: {err}");
        // 项目根有 typall.toml（即使 posts 为空）则放行——init 出的新项目合法
        std::fs::write(dir.join("typall.toml"), "").unwrap();
        let _ = std::fs::create_dir_all(dir.join("posts"));
        let compiled = compile_documents(&dir, &config).unwrap();
        assert!(compiled.posts.is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn split_body_and_style_extracts_parts() {
        let html = r#"<!DOCTYPE html><html><head><style>.x{}</style></head><body><p>hi</p></body></html>"#;
        let (body, style) = split_body_and_style(html);
        assert!(body.contains("<p>hi</p>"));
        assert!(style.contains("<style>"));
    }

    #[test]
    fn seo_head_omitted_without_site_url() {
        let config = Config {
            site: SiteConfig {
                url: String::new(),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(build_seo_head(&config, "/posts/foo/", "article", "标题", "摘要", None), "");
    }

    #[test]
    fn seo_head_includes_og_and_canonical() {
        let config = Config {
            site: SiteConfig {
                url: "https://blog.example.com".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let head =
            build_seo_head(&config, "/posts/foo/", "article", "标题 <x>", "摘要 & 更多", None);
        assert!(head.contains("rel=\"canonical\" href=\"https://blog.example.com/posts/foo/\""));
        assert!(head.contains("property=\"og:type\" content=\"article\""));
        assert!(head.contains("property=\"og:title\" content=\"标题 &lt;x&gt;\""));
        assert!(head.contains("property=\"og:description\" content=\"摘要 &amp; 更多\""));
        assert!(head.contains("name=\"twitter:card\" content=\"summary\""));
        // 无 og_image：不应出现 og:image，卡片维持 summary
        assert!(!head.contains("og:image"));
    }

    #[test]
    fn seo_head_with_og_image_upgrades_twitter_card() {
        let config = Config {
            site: SiteConfig {
                url: "https://blog.example.com".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let head = build_seo_head(
            &config,
            "/posts/foo/",
            "article",
            "标题",
            "摘要",
            Some("https://blog.example.com/og/posts-foo.png"),
        );
        assert!(head.contains("property=\"og:image\" content=\"https://blog.example.com/og/posts-foo.png\""));
        assert!(head.contains("name=\"twitter:card\" content=\"summary_large_image\""));
    }

    #[test]
    fn seo_head_falls_back_to_site_description() {
        let config = Config {
            site: SiteConfig {
                url: "https://blog.example.com".into(),
                description: "站点描述".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let head = build_seo_head(&config, "/", "website", "首页", "", None);
        assert!(head.contains("og:description\" content=\"站点描述\""));
    }

    #[test]
    fn math_preamble_default_has_no_frame_rules() {
        let config = Config::default();
        let p = math_preamble(&config, std::path::Path::new("/nonexistent-root"));
        assert!(!p.contains("html.frame"));
    }

    #[test]
    fn math_preamble_svg_injects_frame_show_rules() {
        let mut config = Config::default();
        config.build.math.renderer = "svg".into();
        let p = math_preamble(&config, std::path::Path::new("/nonexistent-root"));
        // 块级公式 html.elem(div, data-equation) 包裹 frame；行内公式 span 包裹
        // 保持 inline，并带 data-equation 标记供 Markdown 目标配对公式源
        assert!(p.contains("\"data-equation\": \"block\""));
        assert!(p.contains("\"data-equation\": \"inline\""));
        assert!(p.contains("html.frame(it)"));
        assert!(p.contains("box(html.frame(it))"));
        // ⚠ target() 守卫：html.frame(cetz.canvas) 内部 target=="paged" 时
        // 不套 frame，否则 cetz content 里的公式文字会丢失（typst 论坛 4139）。
        assert!(p.contains("target() == \"html\""));
    }

    #[test]
    fn compile_cache_roundtrip_and_invalidation() {
        let root = std::env::temp_dir().join("typall-cache-test");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();

        let cache = CompileCache::new(&root, "#set text(lang: \"zh\")\n");
        let rel = "posts/foo.typ";
        let source_hash = content_hash("body content");

        // 无缓存 → 未命中
        assert!(cache.load(rel, &source_hash).is_none());

        // 写入后 → 命中
        cache.store(rel, &source_hash, &[], "<p>hi</p>", "").unwrap();
        assert_eq!(
            cache.load(rel, &source_hash),
            Some(("<p>hi</p>".to_string(), String::new()))
        );

        // 源文件内容变化 → 失效
        assert!(cache.load(rel, &content_hash("changed")).is_none());

        // 二进制依赖（.wasm 插件/图片）进入缓存且变更时失效
        let dep_dir = root.join("assets");
        std::fs::create_dir_all(&dep_dir).unwrap();
        let dep = dep_dir.join("plugin.wasm");
        std::fs::write(&dep, [0x00, 0x61, 0x73, 0x6d]).unwrap();
        cache.store(rel, &source_hash, std::slice::from_ref(&dep), "<p>hi</p>", "").unwrap();
        assert_eq!(
            cache.load(rel, &source_hash),
            Some(("<p>hi</p>".to_string(), String::new()))
        );
        // 修改二进制内容 → 失效重编译
        std::fs::write(&dep, [0x00, 0x61, 0x73, 0x6d, 0x01]).unwrap();
        assert!(cache.load(rel, &source_hash).is_none());

        // 编译上下文（preamble）变化 → 失效
        let other = CompileCache::new(&root, "#set text(lang: \"en\")\n");
        assert!(other.load(rel, &source_hash).is_none());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn render_prev_next_generates_nav_links() {
        let html = render_prev_next(
            &Theme::builtin(),
            Some(("posts/older", "旧文")),
            Some(("posts/newer", "新文")),
        )
        .unwrap();
        assert!(html.contains("post-nav"));
        assert!(html.contains("← 旧文"));
        assert!(html.contains("新文 →"));
        assert!(html.contains("href=\"/posts/older/\""));
        assert!(html.contains("href=\"/posts/newer/\""));
    }

    #[test]
    fn render_prev_next_omits_when_both_none() {
        assert_eq!(render_prev_next(&Theme::builtin(), None, None).unwrap(), "");
    }

    #[test]
    fn render_prev_next_handles_one_sided() {
        let html = render_prev_next(&Theme::builtin(), Some(("posts/old", "旧")), None).unwrap();
        assert!(html.contains("← 旧"));
        assert!(!html.contains("→"));
    }

    #[test]
    fn render_pagination_hidden_for_single_page() {
        assert_eq!(render_pagination(&Theme::builtin(), 1, 1).unwrap(), "");
    }

    #[test]
    fn render_pagination_shows_navigation() {
        let html = render_pagination(&Theme::builtin(), 2, 5).unwrap();
        assert!(html.contains("pagination"));
        assert!(html.contains("← 上一页"));
        assert!(html.contains("下一页 →"));
        assert!(html.contains("2 / 5"));
        assert!(html.contains("href=\"/page/3/\""));
    }

    #[test]
    fn render_pagination_first_page_no_prev() {
        let html = render_pagination(&Theme::builtin(), 1, 3).unwrap();
        assert!(!html.contains("上一页"));
        assert!(html.contains("下一页 →"));
    }

    #[test]
    fn render_pagination_last_page_no_next() {
        let html = render_pagination(&Theme::builtin(), 3, 3).unwrap();
        assert!(html.contains("← 上一页"));
        assert!(!html.contains("下一页"));
    }

    // ===== P1：片段模板下沉（规格书 §4） =====

    fn theme_with(partials: &[(&str, &str)]) -> Theme {
        let mut map = HashMap::new();
        for (k, v) in partials {
            map.insert(k.to_string(), v.to_string());
        }
        Theme::with_partials(map)
    }

    #[test]
    fn p1_post_nav_partial_overrides_builtin() {
        let theme = theme_with(&[(
            "post_nav",
            r#"<div class="nav">{{prev_title}} ← → {{next_title}}</div>"#,
        )]);
        let html = render_prev_next(
            &theme,
            Some(("posts/old", "旧文")),
            Some(("posts/new", "新文")),
        )
        .unwrap();
        assert_eq!(html, r#"<div class="nav">旧文 ← → 新文</div>"#);
    }

    #[test]
    fn p1_pagination_partial_overrides_builtin() {
        let theme = theme_with(&[("pagination", "<p>{{current}}/{{total}} {{next_url}}</p>")]);
        let html = render_pagination(&theme, 2, 5).unwrap();
        assert_eq!(html, "<p>2/5 /page/3/</p>");
    }

    #[test]
    fn p1_pagination_prev_link_empty_on_first_page() {
        // 无 {% if %} 时靠「空值即空串」的 *_link 变量实现条件显示
        let theme = theme_with(&[("pagination", "[{{prev_link}}][{{next_link}}]")]);
        let html = render_pagination(&theme, 1, 3).unwrap();
        assert_eq!(html, "[][<a href=\"/page/2/\">下一页 →</a>]");
    }

    #[test]
    fn p1_missing_partials_fall_back_to_builtin_shapes() {
        let theme = theme_with(&[]);
        let nav = render_prev_next(&theme, Some(("posts/old", "旧")), Some(("posts/new", "新"))).unwrap();
        assert!(nav.starts_with(r#"<nav class="post-nav"><a class="prev""#));
        let page = render_pagination(&theme, 2, 5).unwrap();
        assert!(page.starts_with(r#"<nav class="pagination">"#));
        assert!(page.contains(r#"<span class="current">2 / 5</span>"#));
    }

    #[test]
    fn p1_tag_cloud_item_partial_overrides_builtin() {
        let theme = theme_with(&[("tag_cloud_item", "<b>{{name}}({{count}})</b>")]);
        let doc = CompiledDoc {
            slug: "posts/x".into(),
            meta: Default::default(),
            body_html: "<p>hi</p>".into(),
            math_style: String::new(),
            math: Vec::new(),
        };
        let mut map: BTreeMap<String, Vec<&CompiledDoc>> = BTreeMap::new();
        map.insert("Rust".into(), vec![&doc, &doc]);
        let html = render_cloud(&theme, "标签", "tags", &map).unwrap();
        assert!(html.contains("<b>Rust(2)</b>"));
        assert!(html.contains(r#"<div class="tag-cloud">"#));
    }

    // ===== 专栏（series） =====

    fn doc_with(slug: &str, date: &str, series: Option<&str>, weight: Option<i64>) -> CompiledDoc {
        CompiledDoc {
            slug: slug.into(),
            meta: DocumentMeta {
                date: Some(date.into()),
                series: series.map(Into::into),
                series_weight: weight,
                ..Default::default()
            },
            body_html: String::new(),
            math_style: String::new(),
            math: Vec::new(),
        }
    }

    #[test]
    fn series_nav_index_orders_by_course_and_skips_untitled() {
        // 课程顺序：权重升序（缺省最后）→ 同权重按日期升序。与站内时间序相反。
        let posts = vec![
            doc_with("p/three", "2026-01-01", Some("物理"), Some(3)),
            doc_with("p/one", "2026-05-01", Some("物理"), Some(1)),
            doc_with("p/plain", "2026-06-01", None, None),
            doc_with("p/two", "2026-02-01", Some("物理"), Some(1)),
            doc_with("p/last", "2026-07-01", Some("物理"), None),
        ];
        let navs = series_nav_index(&posts).unwrap();
        assert!(navs[2].is_none(), "非专栏文章无导航");

        // 第一课：p/two（权重 1、日期最早）
        let first = navs[3].as_ref().unwrap();
        assert_eq!(first.series, "物理");
        assert_eq!(first.series_url, "/series/物理/");
        assert_eq!((first.index, first.total), (1, 4));
        assert!(first.prev.is_none());
        assert_eq!(first.next.as_ref().unwrap().0, "p/one");

        // 第二课 p/one 前后互链；第三课 p/three
        assert_eq!(navs[1].as_ref().unwrap().prev.as_ref().unwrap().0, "p/two");
        assert_eq!(navs[1].as_ref().unwrap().next.as_ref().unwrap().0, "p/three");
        assert_eq!(navs[0].as_ref().unwrap().prev.as_ref().unwrap().0, "p/one");
        assert_eq!(navs[0].as_ref().unwrap().next.as_ref().unwrap().0, "p/last");

        // 缺权重的 p/last 排最后
        let last = navs[4].as_ref().unwrap();
        assert_eq!((last.index, last.total), (4, 4));
        assert!(last.next.is_none());
    }

    #[test]
    fn series_nav_index_bails_on_slug_collision() {
        // 「!!」与「??」都解析为 slugify 的兜底 `untitled`
        let posts = vec![
            doc_with("p/a", "2026-01-01", Some("!!"), None),
            doc_with("p/b", "2026-01-02", Some("??"), None),
        ];
        let err = series_nav_index(&posts).unwrap_err();
        assert!(format!("{err:#}").contains("slug 冲突"), "实际报错: {err:#}");
    }

    #[test]
    fn render_series_nav_markup_and_empty_cases() {
        let nav = SeriesNav {
            series: "高中物理".into(),
            series_url: "/series/高中物理/".into(),
            index: 2,
            total: 3,
            prev: Some(("posts/a".into(), "力学".into())),
            next: None,
        };
        let html = render_series_nav(&Theme::builtin(), Some(&nav)).unwrap();
        assert!(html.contains(r#"<nav class="series-nav">"#));
        assert!(html.contains(r#"href="/series/高中物理/""#));
        assert!(html.contains("专栏：高中物理"));
        assert!(html.contains("2 / 3"));
        assert!(html.contains(r#"href="/posts/a/""#));
        assert!(html.contains("← 力学"));
        assert!(!html.contains("class=\"next\""), "无下一篇时不输出 next 链接");

        // 单篇专栏：1 / 1，无前后链接
        let solo = SeriesNav {
            series: "S".into(),
            series_url: "/series/S/".into(),
            index: 1,
            total: 1,
            prev: None,
            next: None,
        };
        let solo_html = render_series_nav(&Theme::builtin(), Some(&solo)).unwrap();
        assert!(solo_html.contains("1 / 1"));
        assert!(!solo_html.contains("href=\"/posts/"));

        // 非专栏文章：整体为空串
        assert_eq!(render_series_nav(&Theme::builtin(), None).unwrap(), "");
    }
}
