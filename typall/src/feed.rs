//! RSS/Atom 订阅源与 sitemap.xml 生成。

/// 一条订阅条目。
pub struct FeedItem {
    pub title: String,
    /// 绝对链接（含站点 URL）。
    pub link: String,
    /// 发布日期（`YYYY-MM-DD` 或更完整格式）。
    pub date: Option<String>,
    /// 正文 HTML（可选，用于 Atom `<content>` 元素）。
    pub content: Option<String>,
}

/// 生成 Atom 订阅源。
pub fn atom_feed(
    site_title: &str,
    site_url: &str,
    updated: &str,
    author: &str,
    items: &[FeedItem],
) -> String {
    let mut entries = String::new();
    for item in items {
        let iso = to_iso(item.date.as_deref().unwrap_or_default());
        let content_block = match &item.content {
            // CDATA 内无法转义 `]]>`，按 XML 规范拆成两个 CDATA 段拼接。
            Some(c) => format!(
                "\n    <content type=\"html\"><![CDATA[{}]]></content>",
                c.replace("]]>", "]]]]><![CDATA[>")
            ),
            None => String::new(),
        };
        entries.push_str(&format!(
            "  <entry>\n    <title>{}</title>\n    <link href=\"{}\"/>\n    <id>{}</id>\n    <published>{}</published>\n    <updated>{}</updated>{}\n  </entry>\n",
            xml_escape(&item.title),
            xml_escape(&item.link),
            xml_escape(&item.link),
            iso,
            iso,
            content_block,
        ));
    }

    let author_block = if author.is_empty() {
        String::new()
    } else {
        format!("  <author><name>{}</name></author>\n", xml_escape(author))
    };

    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{title}</title>
  <link href="{base}/"/>
  <link rel="self" href="{base}/atom.xml"/>
  <updated>{updated}</updated>
  <id>{base}/</id>
  <generator uri="https://github.com/typall" version="{version}">typall</generator>
{author_block}{entries}
</feed>
"#,
        title = xml_escape(site_title),
        base = xml_escape(site_url),
        // updated 来自文章 meta（用户可控），与其余插值一样必须过转义。
        updated = xml_escape(updated),
        version = env!("CARGO_PKG_VERSION"),
        author_block = author_block,
        entries = entries,
    )
}

/// 生成 sitemap.xml（每条 URL 可附带 lastmod 日期）。
pub fn sitemap(urls: &[SitemapEntry]) -> String {
    let mut body = String::new();
    for entry in urls {
        body.push_str("  <url><loc>");
        body.push_str(&xml_escape(&entry.url));
        body.push_str("</loc>");
        if let Some(date) = &entry.lastmod {
            let iso = to_iso(date);
            if !iso.is_empty() {
                body.push_str("<lastmod>");
                body.push_str(&iso);
                body.push_str("</lastmod>");
            }
        }
        body.push_str("</url>\n");
    }
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}</urlset>
"#,
        body
    )
}

/// sitemap 中的一条记录。
pub struct SitemapEntry {
    pub url: String,
    /// 最后修改日期（`YYYY-MM-DD` 或 ISO 格式），可选。
    pub lastmod: Option<String>,
}

fn to_iso(date: &str) -> String {
    if date.is_empty() {
        String::new()
    } else if date.len() == 10 {
        // `YYYY-MM-DD` → 补时间
        format!("{date}T00:00:00Z")
    } else {
        date.to_string()
    }
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atom_feed_escapes_and_formats_entries() {
        let items = vec![FeedItem {
            title: "A < B".into(),
            link: "https://x.com/p/".into(),
            date: Some("2026-09-01".into()),
            content: None,
        }];
        let feed = atom_feed("站点", "https://x.com", "2026-09-01T00:00:00Z", "作者", &items);
        assert!(feed.contains("<title>A &lt; B</title>"));
        assert!(feed.contains("<published>2026-09-01T00:00:00Z</published>"));
        assert!(feed.contains("xmlns=\"http://www.w3.org/2005/Atom\""));
        assert!(feed.contains("rel=\"self\""));
        assert!(feed.contains("<generator"));
        assert!(feed.contains(&format!(
            "version=\"{}\">typall</generator>",
            env!("CARGO_PKG_VERSION")
        )));
        assert!(feed.contains("<name>作者</name>"));
        assert!(!feed.contains("<content"));
    }

    #[test]
    fn atom_feed_includes_content_when_present() {
        let items = vec![FeedItem {
            title: "文章".into(),
            link: "https://x.com/p/".into(),
            date: Some("2026-09-01".into()),
            content: Some("<p>正文内容</p>".into()),
        }];
        let feed = atom_feed("站点", "https://x.com", "2026-09-01T00:00:00Z", "", &items);
        assert!(feed.contains("<content type=\"html\"><![CDATA[<p>正文内容</p>]]></content>"));
    }

    #[test]
    fn atom_feed_escapes_cdata_terminator() {
        // 正文含 `]]>` 时拆分 CDATA 段，保证 XML 仍良构
        let items = vec![FeedItem {
            title: "t".into(),
            link: "https://x.com/p/".into(),
            date: None,
            content: Some("<p>x]]&gt;".replace("&gt;", ">")),
        }];
        let feed = atom_feed("站点", "https://x.com", "2026-09-01T00:00:00Z", "", &items);
        assert!(feed.contains("<![CDATA[<p>x]]]]><![CDATA[>]]></content>"));
    }

    #[test]
    fn atom_feed_escapes_feed_level_updated() {
        // 源级 <updated> 来自文章 meta（用户可控），必须转义
        let feed = atom_feed("站点", "https://x.com", "2026<09", "", &[]);
        assert!(feed.contains("<updated>2026&lt;09</updated>"));
    }

    #[test]
    fn sitemap_lists_all_urls() {
        let urls = vec![
            SitemapEntry { url: "https://x.com/".into(), lastmod: None },
            SitemapEntry { url: "https://x.com/p/".into(), lastmod: Some("2026-09-01".into()) },
        ];
        let s = sitemap(&urls);
        assert!(s.contains("<loc>https://x.com/</loc>"));
        assert!(s.contains("<loc>https://x.com/p/</loc>"));
        assert!(s.contains("<lastmod>2026-09-01T00:00:00Z</lastmod>"));
    }

    #[test]
    fn to_iso_pads_plain_dates() {
        assert_eq!(to_iso("2026-09-01"), "2026-09-01T00:00:00Z");
        assert_eq!(to_iso(""), "");
        assert_eq!(to_iso("2026-09-01T12:00:00Z"), "2026-09-01T12:00:00Z");
    }
}
