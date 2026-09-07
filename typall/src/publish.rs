//! 发布目标层：把文档 IR 变换为各平台格式（`typall publish`）。
//!
//! 设计意图：
//! - `publish`（单篇 → 平台格式）与 `deploy`（整站 → 主机）是两个动作：
//!   publish 是 per-document、带本地状态库、per-target 格式；
//! - 每个发布目标都是 [`CompiledDoc`](crate::ir::CompiledDoc) 的一个消费者，
//!   从同一份 IR 按目标重变换，不复用站点 HTML；
//! - 所有目标**零 API 依赖**（Markdown 导出、公众号富文本本地导出），
//!   触网分发由「复制到平台」按钮承担（见 serve.rs 注入脚本）。
//!
//! 公式处理：Markdown 目标输出 **Typst 数学源码**（`$..$` / `$$..$$`），
//! frontmatter 标注 `math: typst`。LaTeX 转换是知乎等目标的 per-target
//! 关注点，不在管线层做。

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail};
use base64::Engine as _;
use ego_tree::NodeRef;
use scraper::Node;
use serde::{Deserialize, Serialize};

use crate::build::{compile_documents, hash_bytes};
use crate::config::Config;
use crate::ir::{CompiledDoc, MathItem};

/// 公式占位标记：转换时替换正文中的公式渲染物，Markdown 生成后还原。
/// 纯字母数字，能原样穿过 HTML 解析与 Markdown 转义。
fn math_marker(i: usize) -> String {
    format!("TYPALLMATH{i}TYPALL")
}

/// 单篇文档在单个发布目标上的最后发布记录。
///
/// 兼容性：旧版本记录含 `pushed_hash` / `remote_id`（远端推送已移除）。
/// serde 默认忽略未知字段，旧 publish.json 可正常读取，下次保存时键消失。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRecord {
    pub platform: String,
    pub slug: String,
    /// 本地产物指纹（编译指纹，见 [`doc_fingerprint`]）。与当前不一致即视为变更。
    pub content_hash: String,
    /// 最后发布时间（本地时区 RFC3339）。
    pub published_at: String,
    /// 本次发布引用的资产文件（相对目标输出目录，如 `assets/x.svg`）。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<String>,
}

/// 原子写文件：先写 `<path>.tmp` 再 rename 覆盖，避免进程中断留下半截文件。
///
/// 状态文件（publish.json）存着发布指纹，写坏一半等同于丢失状态。
/// `std::fs::rename` 在 Windows 上也会替换已存在目标。
pub(crate) fn atomic_write(path: &Path, contents: &[u8]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, contents)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// 发布状态库（`.typall/publish.json`）。
///
/// 键为 `(platform, slug)`，值含内容指纹——指纹未变则重跑自动跳过，
/// 发布幂等；`--status` 可查询。随项目工作区存放（不入库），只记本地状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublishState {
    pub records: Vec<PublishRecord>,
}

impl PublishState {
    fn path(root: &Path) -> PathBuf {
        root.join(".typall").join("publish.json")
    }

    /// 读取状态库；文件不存在视为空库（首次发布）。
    fn load(root: &Path) -> anyhow::Result<Self> {
        let p = Self::path(root);
        if !p.exists() {
            return Ok(Self::default());
        }
        let s = std::fs::read_to_string(&p)?;
        serde_json::from_str(&s).map_err(|e| anyhow::anyhow!("解析 {} 失败: {e}", p.display()))
    }

    /// 原子写（状态库存着发布指纹，写坏一半等同丢状态）。
    fn save(&self, root: &Path) -> anyhow::Result<()> {
        let p = Self::path(root);
        atomic_write(&p, (serde_json::to_string_pretty(self)? + "\n").as_bytes())
    }

    fn find(&self, platform: &str, slug: &str) -> Option<&PublishRecord> {
        self.records
            .iter()
            .find(|r| r.platform == platform && r.slug == slug)
    }

    /// 同 (platform, slug) 覆盖，否则追加。
    fn upsert(&mut self, rec: PublishRecord) {
        match self
            .records
            .iter_mut()
            .find(|r| r.platform == rec.platform && r.slug == rec.slug)
        {
            Some(slot) => *slot = rec,
            None => self.records.push(rec),
        }
    }
}

/// 编译指纹：覆盖所有会进入发布产物的字段（frontmatter / 草稿元数据）+ 预处理后正文。
///
/// 在平台渲染**之前**即可计算——判重（P1-2）不依赖具体目标，内容未变的文档
/// 完全跳过渲染管线（含触网上传准备）。`body_html` 为引擎预处理
/// （data URI 资产提取，内容寻址）后的正文，跨运行稳定。
fn doc_fingerprint(doc: &CompiledDoc, body_html: &str) -> String {
    let m = &doc.meta;
    let mut buf = String::with_capacity(body_html.len() + 256);
    buf.push_str("v2");
    for part in [
        m.title.as_str(),
        m.date.as_deref().unwrap_or(""),
        m.updated.as_deref().unwrap_or(""),
        m.excerpt.as_deref().unwrap_or(""),
        if m.draft { "1" } else { "0" },
        &m.tags.join(","),
        &m.categories.join(","),
    ] {
        buf.push('\x00');
        buf.push_str(part);
    }
    // 自定义元数据（author / source_url 等），按键序稳定序列化。
    let mut keys: Vec<&String> = m.raw.keys().collect();
    keys.sort();
    for k in keys {
        buf.push('\x00');
        buf.push_str(k);
        buf.push('=');
        buf.push_str(&format!("{:?}", m.raw[k]));
    }
    buf.push('\x00');
    buf.push_str(body_html);
    hash_bytes(buf.as_bytes())
}

/// 发布上下文：publisher 变换文档时可用的只读信息。
pub struct PublishCtx<'a> {
    pub config: &'a Config,
}

/// 一次渲染的结果。
pub struct RenderedDoc {
    /// 相对 out_root 的产物路径，如 `posts/quantum.md`。
    pub rel_path: String,
    /// 产物全文。
    pub content: String,
}

/// 发布目标：把文档 IR 变换为目标格式。
///
/// 引擎（[`publish`]）负责编译、资产提取、状态库判重与写盘；
/// 实现只做纯变换（零网络副作用）。新平台目标实现此 trait 并注册进
/// [`publisher_for`] 即接入管线。
pub trait Publisher {
    fn name(&self) -> &'static str;

    /// 产物相对 out_root 的路径（不触发渲染即可确定）。
    /// 孤儿清理用它保护"内容未变被跳过"的文档的既有产物。
    fn rel_path(&self, doc: &CompiledDoc) -> String;

    /// 渲染单篇文档。`body_html` 为引擎预处理（data URI 资产提取
    /// 并重写为相对路径）后的正文，实现应使用它而非 `doc.body_html`。
    fn render(
        &self,
        doc: &CompiledDoc,
        body_html: &str,
        ctx: &PublishCtx<'_>,
    ) -> anyhow::Result<RenderedDoc>;
}

/// 知乎发布目标：生成知乎编辑器可粘贴的富文本 HTML。
///
/// 公式策略：typst 数学源码经 [`crate::latex`] 转为 LaTeX，输出为
/// `<span data-formula="latex">…</span>` 形态——知乎编辑器粘贴时会把
/// LaTeX 公式转换为原生公式块；正文为最小内联样式（知乎比公众号宽容）。
struct ZhihuPublisher;

impl Publisher for ZhihuPublisher {
    fn name(&self) -> &'static str {
        "zhihu"
    }

    fn rel_path(&self, doc: &CompiledDoc) -> String {
        format!("{}.html", doc.slug)
    }

    fn render(
        &self,
        doc: &CompiledDoc,
        body_html: &str,
        _ctx: &PublishCtx<'_>,
    ) -> anyhow::Result<RenderedDoc> {
        Ok(RenderedDoc {
            rel_path: self.rel_path(doc),
            content: render_zhihu(doc, body_html),
        })
    }
}

/// 知乎富文本渲染：标题 + 元信息 + 正文（公式 MathItem → LaTeX span）。
///
/// 公式替换策略：`body_html` 中公式渲染物按序与 `doc.math` 配对（与
/// markdown 目标同款逻辑），替换为 LaTeX span；配对失败保守回退原 HTML。
fn render_zhihu(doc: &CompiledDoc, body_html: &str) -> String {
    use std::fmt::Write as _;

    // 公式渲染物占位与 MathItem 配对：逐个替换为 LaTeX span
    let mut html = body_html.to_string();
    if !doc.math.is_empty() {
        // 公式渲染物按序替换为 LaTeX 形态（知乎公式）；数量不匹配（动态
        // 生成公式等罕见场景）时保守保留原渲染物
        replace_math_with_latex_zhihu(&mut html, &doc.math);
    }

    let mut out = String::with_capacity(html.len() + 512);
    let _ = writeln!(out, "<h1>{}</h1>", doc.meta.title);
    if let Some(d) = &doc.meta.date {
        let _ = writeln!(out, "<p><em>{}</em></p>", d);
    }
    let _ = write!(out, "{}", html);
    let _ = doc.meta.updated;
    out
}

/// 知乎公式替换：公式渲染物占位后按序重写为知乎 LaTeX 形态。
///
/// 块级公式 → `$$…$$` 独立段；行内 → `$…$`。渲染物数量与 MathItem
/// 不匹配时返回 false（调用方保留原 HTML 降级）。
fn replace_math_with_latex_zhihu(html: &mut String, math: &[crate::ir::MathItem]) -> bool {
    let (marked, spans) = substitute_math(html);
    if spans.len() != math.len() {
        return false;
    }
    let mut out = marked;
    for (i, item) in math.iter().enumerate() {
        let latex = crate::latex::typst_math_to_latex(&item.source);
        let replacement = if item.block {
            format!(r#"<p>$$ {} $$</p>"#, latex)
        } else {
            format!(r#"<span class="ztext-math">$ {} $</span>"#, latex)
        };
        let marker = math_marker(i);
        if let Some(pos) = out.find(&marker) {
            out.replace_range(pos..pos + marker.len(), &replacement);
        }
    }
    *html = out;
    true
}

/// 注册表条目：`(目标名, 构造器)`。
type PublisherEntry = (&'static str, fn() -> Box<dyn Publisher>);

/// 发布目标注册表：新增目标 = 实现 [`Publisher`] + 在此登记一行。
const REGISTRY: &[PublisherEntry] = &[
    ("markdown", || Box::new(MarkdownPublisher) as Box<dyn Publisher>),
    ("wechat", || Box::new(crate::wechat::WechatPublisher) as Box<dyn Publisher>),
    ("zhihu", || Box::new(ZhihuPublisher) as Box<dyn Publisher>),
];

fn publisher_for(name: &str) -> anyhow::Result<Box<dyn Publisher>> {
    REGISTRY
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, ctor)| ctor())
        .ok_or_else(|| {
            let names: Vec<&str> = REGISTRY.iter().map(|(n, _)| *n).collect();
            anyhow!("未知发布目标 `{name}`。当前可用目标：{}", names.join("、"))
        })
}

/// Markdown 导出目标（阶段 1 落地的格式，阶段 2 迁入 trait 体系）。
struct MarkdownPublisher;

impl Publisher for MarkdownPublisher {
    fn name(&self) -> &'static str {
        "markdown"
    }

    fn rel_path(&self, doc: &CompiledDoc) -> String {
        format!("{}.md", doc.slug)
    }

    fn render(
        &self,
        doc: &CompiledDoc,
        body_html: &str,
        _ctx: &PublishCtx<'_>,
    ) -> anyhow::Result<RenderedDoc> {
        Ok(RenderedDoc {
            rel_path: self.rel_path(doc),
            content: render_markdown(doc, body_html),
        })
    }
}

/// 发布选项（CLI `typall publish` 的全部开关）。
#[derive(Debug, Clone, Default)]
pub struct PublishOptions {
    /// 发布目标（None = 默认 markdown）。
    pub target: Option<String>,
    /// 覆盖 `[publish] out_dir` 的导出目录。
    pub output: Option<String>,
    /// 只发布匹配 slug 的文档。
    pub slug: Option<String>,
    /// 包含草稿。
    pub drafts: bool,
    /// 内容未变也强制重新发布。
    pub force: bool,
    /// 只查询发布状态，不执行发布。
    pub status: bool,
    /// 仅打印计划，不写盘、不改动状态库。
    pub dry_run: bool,
}

/// 发布引擎：编译 → 资产提取 → 按目标渲染 → 状态库判重 → 写盘。
pub fn publish(
    root: &Path,
    config: &Config,
    opts: &PublishOptions,
) -> anyhow::Result<()> {
    let PublishOptions {
        ref target,
        ref output,
        ref slug,
        drafts: cli_drafts,
        force,
        status: status_only,
        dry_run,
    } = *opts;
    let target = target.as_deref();
    let output_override = output.as_deref();
    let slug_filter = slug.as_deref();

    // --status：只查询状态库，不编译不写盘。
    if status_only {
        return show_status(root, slug_filter);
    }

    let target = target.unwrap_or("markdown");
    let publisher = publisher_for(target)?;
    let compiled = compile_documents(root, config)?;

    // 与 build 一致的草稿 / 定时发布过滤（pages 同套规则，防止草稿页外泄）。
    let include_drafts = config.build.drafts || cli_drafts;
    let mut posts = compiled.posts;
    let mut pages = compiled.pages;
    crate::build::filter_unpublished(&mut posts, include_drafts);
    crate::build::filter_unpublished(&mut pages, include_drafts);
    posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));

    let docs: Vec<&CompiledDoc> = posts
        .iter()
        .chain(pages.iter())
        .filter(|d| {
            slug_filter.is_none_or(|s| {
                d.slug == s || d.slug.ends_with(&format!("/{s}")) || d.slug.ends_with(&format!("\\{s}"))
            })
        })
        .collect();
    if let Some(s) = slug_filter
        && docs.is_empty()
    {
        bail!("未找到匹配 `{s}` 的文章（slug 形如 posts/quantum，也可只写 quantum）");
    }

    let out_dir = output_override
        .map(str::to_string)
        .or_else(|| {
            let d = config.publish.out_dir.clone();
            (!d.is_empty()).then_some(d)
        })
        .unwrap_or_else(|| "publish".to_string());
    // 各目标在发布目录下各占一个子目录，互不混放。
    let out_root = root.join(&out_dir).join(target);

    let mut state = PublishState::load(root)?;
    let mut counts = [0usize; 3]; // create / update / unchanged
    let mut written: HashSet<String> = HashSet::new();

    for doc in &docs {
        // 引擎统一做资产提取：data URI 图片落盘为内容寻址文件，
        // 正文 src 重写为相对路径（Markdown / 富文本平台都需要）。
        let depth = doc.slug.matches('/').count();
        let assets_dir = out_root.join("assets");
        let (body_html, assets) = extract_data_uri_images(&doc.body_html, &assets_dir, depth)?;

        // 判重前移（P1-2）：编译指纹在平台渲染前计算，内容未变的文档直接跳过。
        let fp = doc_fingerprint(doc, &body_html);
        let rec = state.find(publisher.name(), &doc.slug).cloned();
        let local_stale = force || rec.as_ref().is_none_or(|r| r.content_hash != fp);
        if !local_stale {
            counts[2] += 1;
            // 未变的文档不渲染，但其既有产物不能被孤儿清理误删。
            written.insert(publisher.rel_path(doc));
            continue;
        }
        counts[usize::from(rec.is_some())] += 1;

        let rendered = publisher.render(doc, &body_html, &PublishCtx {
            config,
        })?;

        // P0-1：dry-run 只读——不写盘、不删孤儿、不动状态库。
        if dry_run {
            continue;
        }

        let path = out_root.join(&rendered.rel_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, &rendered.content)?;
        written.insert(rendered.rel_path);

        let record = PublishRecord {
            platform: publisher.name().to_string(),
            slug: doc.slug.clone(),
            content_hash: fp,
            published_at: chrono::Local::now().to_rfc3339(),
            assets,
        };
        state.upsert(record);
    }

    if dry_run {
        println!(
            "🔍 {} dry-run 预览：{} 新建，{} 更新，{} 未变（未写盘、未改动状态库）",
            target,
            counts[0],
            counts[1],
            counts[2]
        );
        return Ok(());
    }

    // 清理孤儿文件：源文章删除后，旧的导出残留无意义。
    // assets/ 是内容寻址共享目录（跨文档去重），不参与孤儿清理。
    for entry in walkdir::WalkDir::new(&out_root)
        .into_iter()
        .filter_entry(|e| e.file_name() != std::ffi::OsStr::new("assets"))
    {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(&out_root)?
            .to_string_lossy()
            .replace('\\', "/");
        if !written.contains(&rel) {
            std::fs::remove_file(entry.path())?;
        }
    }

    if let Err(e) = state.save(root) {
        eprintln!("⚠️ 发布状态库写入失败（不影响本次产物）: {e}");
    }

    println!(
        "✅ {} 发布完成：{} 新建，{} 更新，{} 未变 → {}",
        target,
        counts[0],
        counts[1],
        counts[2],
        out_root.display()
    );
    Ok(())
}

/// 查询发布状态：列出状态库记录（可按 slug 过滤）。
fn show_status(root: &Path, slug_filter: Option<&str>) -> anyhow::Result<()> {
    let state = PublishState::load(root)?;
    let mut rows: Vec<&PublishRecord> = state
        .records
        .iter()
        .filter(|r| {
            slug_filter.is_none_or(|s| {
                r.slug == s || r.slug.ends_with(&format!("/{s}")) || r.slug.ends_with(&format!("\\{s}"))
            })
        })
        .collect();
    if let Some(s) = slug_filter
        && rows.is_empty()
    {
        bail!("状态库中没有匹配 `{s}` 的发布记录");
    }
    rows.sort_by(|a, b| a.platform.cmp(&b.platform).then(a.slug.cmp(&b.slug)));
    if rows.is_empty() {
        println!("（状态库为空：还没有任何发布记录）");
        return Ok(());
    }
    println!(
        "{:<10}  {:<26}  {:<28}  指纹",
        "平台", "文档", "最后发布"
    );
    for r in &rows {
        println!(
            "{:<10}  {:<26}  {:<28}  {}",
            r.platform,
            r.slug,
            r.published_at,
            &r.content_hash[..r.content_hash.len().min(12)]
        );
    }
    println!(
        "\n共 {} 条记录（状态库：{}）",
        rows.len(),
        PublishState::path(root).display()
    );
    Ok(())
}

/// 从正文 HTML 提取 data URI 图片到内容寻址资产目录。
///
/// `src="data:image/<t>;base64,<payload>"` 解码写盘为
/// `assets/<fnv8>.<ext>`（相同内容天然去重），`src` 重写为从文档
/// 所在目录到资产目录的相对路径（按 slug 深度回溯 `../`）。
/// 返回（重写后的 HTML, 资产相对目标输出目录的路径列表）。
fn extract_data_uri_images(
    body_html: &str,
    assets_dir: &Path,
    slug_depth: usize,
) -> anyhow::Result<(String, Vec<String>)> {
    const MARK: &str = "data:image/";
    let mut out = String::with_capacity(body_html.len());
    let mut rest = body_html;
    let mut assets: Vec<String> = Vec::new();
    while let Some(pos) = rest.find(MARK) {
        let tail = &rest[pos..];
        // data URI 必须以属性引号收尾，否则不是 img src（防御性跳过）。
        let Some(end_rel) = tail.find('"') else { break };
        let uri = &tail[..end_rel];
        let Some((ext, bytes)) = decode_data_uri(uri) else {
            // 非 base64 形态（如 svg+xml;utf8），原样保留。
            out.push_str(&rest[..pos + end_rel]);
            rest = &rest[pos + end_rel..];
            continue;
        };
        let name = format!("{}.{}", hash_bytes(&bytes), ext);
        let rel = format!("assets/{name}");
        let path = assets_dir.join(&name);
        if !path.exists() {
            std::fs::create_dir_all(assets_dir)?;
            std::fs::write(&path, &bytes)?;
        }
        // 文档在 <out_root>/<slug>.md，slug 深度即需回溯的层级数。
        out.push_str(&rest[..pos]);
        out.push_str(&"../".repeat(slug_depth));
        out.push_str(&rel);
        rest = &rest[pos + end_rel..];
        if !assets.contains(&rel) {
            assets.push(rel);
        }
    }
    out.push_str(rest);
    Ok((out, assets))
}

/// 解析 `data:image/<sub>;base64,<payload>`，返回（扩展名, 字节）。
/// 非 base64 形态返回 None。
fn decode_data_uri(uri: &str) -> Option<(&'static str, Vec<u8>)> {
    let rest = uri.strip_prefix("data:image/")?;
    let (sub, payload) = rest.split_once(";base64,")?;
    let ext = match sub {
        "png" => "png",
        "jpeg" | "jpg" => "jpg",
        "gif" => "gif",
        "webp" => "webp",
        "svg+xml" => "svg",
        _ => return None,
    };
    // 容忍属性内的空白（HTML 序列化可能折叠换行）。
    let compact: String = payload.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(compact.as_bytes())
        .ok()?;
    Some((ext, bytes))
}

/// 渲染单篇文档为 Markdown（YAML frontmatter + 正文）。
///
/// `body_html` 为引擎资产提取重写后的正文（与 `doc.body_html` 可能不同）。
fn render_markdown(doc: &CompiledDoc, body_html: &str) -> String {
    let meta = &doc.meta;
    let mut out = String::from("---\n");
    out.push_str(&format!("title: {}\n", yaml_str(&meta.title)));
    if let Some(d) = &meta.date {
        out.push_str(&format!("date: {}\n", yaml_str(d)));
    }
    if let Some(u) = &meta.updated {
        out.push_str(&format!("updated: {}\n", yaml_str(u)));
    }
    if !meta.tags.is_empty() {
        out.push_str(&format!("tags: [{}]\n", yaml_str_list(&meta.tags)));
    }
    if !meta.categories.is_empty() {
        out.push_str(&format!(
            "categories: [{}]\n",
            yaml_str_list(&meta.categories)
        ));
    }
    out.push_str(&format!("draft: {}\n", meta.draft));
    // 摘要：手动 excerpt 优先；否则走 strip_html（已解码实体含 &nbsp;）再截取。
    let excerpt = match &meta.excerpt {
        Some(s) => s.clone(),
        None => {
            let plain = crate::site_html::strip_html(body_html);
            crate::content::auto_excerpt(&plain, 200)
        }
    };
    if !excerpt.is_empty() {
        out.push_str(&format!("excerpt: {}\n", yaml_str(&excerpt)));
    }
    // 公式语法标注：正文公式为 Typst 数学源码（阶段 3 按目标转 LaTeX）。
    out.push_str("math: typst\n");
    out.push_str("---\n\n");
    out.push_str(&convert_body(&doc.slug, body_html, &doc.math));
    out
}

/// YAML 双引号字符串（转义反斜杠与双引号）。
fn yaml_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn yaml_str_list(items: &[String]) -> String {
    items.iter().map(|s| yaml_str(s)).collect::<Vec<_>>().join(", ")
}

/// 把正文 HTML 中的公式渲染物替换为占位标记，返回（标记后 HTML, 按序提取的原始片段）。
///
/// 两种渲染策略下的公式形态：
/// - `svg`：`<div data-equation="block">` / `<span data-equation="inline">` 包裹
///   （见 `build::math_preamble`；cetz 插图无此标记，不会误伤）；
/// - `mathml`：裸 `<math>` 元素（typst-html 只从公式生成 MathML）。
fn substitute_math(html: &str) -> (String, Vec<String>) {
    let mut spans: Vec<String> = Vec::new();
    let mut out = String::with_capacity(html.len());
    let mut i = 0usize;
    while i < html.len() {
        if html[i..].starts_with('<')
            && let Some((tag, attrs, after, self_closing)) = parse_open_tag(html, i)
        {
            let is_math = tag == "math"
                || ((tag == "span" || tag == "div") && attrs.contains("data-equation"));
            if is_math && !self_closing
                && let Some(end) = find_matching_close(html, after, tag)
            {
                spans.push(html[i..end].to_string());
                out.push_str(&math_marker(spans.len() - 1));
                i = end;
                continue;
            }
        }
        let ch = html[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    (out, spans)
}

/// 解析 `html[i..]` 处的开始标签，返回（标签名, 属性串, '>' 之后位置, 是否自闭合）。
fn parse_open_tag(html: &str, i: usize) -> Option<(&str, &str, usize, bool)> {
    let rest = &html[i + 1..];
    let name_len = rest
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == ':'))
        .unwrap_or(rest.len());
    if name_len == 0 {
        return None;
    }
    let tag = &rest[..name_len];
    if !tag.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    // 扫描到 '>'，跳过属性值内的引号。
    let mut j = name_len;
    let bytes = rest.as_bytes();
    while j < bytes.len() {
        match bytes[j] {
            b'"' | b'\'' => {
                let q = bytes[j];
                j += 1;
                while j < bytes.len() && bytes[j] != q {
                    j += 1;
                }
                // 跳过闭合引号（j 停在闭合引号上，不前移会把它当新引号再跳一次）
                j += 1;
            }
            b'>' => {
                let self_closing = j > 0 && bytes[j - 1] == b'/';
                return Some((tag, &rest[name_len..j - if self_closing { 1 } else { 0 }], i + 1 + j + 1, self_closing));
            }
            _ => j += 1,
        }
    }
    None
}

/// 从 `from` 起找与开始标签配对的闭合标签（同名标签深度计数）。
fn find_matching_close(html: &str, from: usize, tag: &str) -> Option<usize> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut depth = 1usize;
    let mut i = from;
    while i < html.len() {
        // 逐字节前进会落进多字节字符（如中文）中间，切片将 panic——
        // 非边界位置直接跳过（标签序列本身是 ASCII，不受影响）。
        if !html.is_char_boundary(i) {
            i += 1;
            continue;
        }
        if html[i..].starts_with(&open) {
            let after = html[i + open.len()..].chars().next();
            if after.is_none_or(|c| c.is_whitespace() || c == '>' || c == '/') {
                depth += 1;
                i += open.len();
                continue;
            }
        }
        if html[i..].starts_with(&close) {
            depth -= 1;
            if depth == 0 {
                return Some(i + close.len());
            }
            i += close.len();
            continue;
        }
        i += 1;
    }
    None
}

/// 正文变换：公式占位 → HTML → Markdown → 还原公式。
fn convert_body(slug: &str, body_html: &str, math: &[MathItem]) -> String {
    let (marked, extracted) = substitute_math(body_html);
    let md = html_to_markdown(&marked);
    if extracted.len() != math.len() {
        // 数量不配对（动态生成公式等罕见场景）：保守回退，保留原始 HTML。
        eprintln!(
            "⚠️ [{slug}] 公式配对失败（渲染物 {} 个 vs 源码 {} 个），公式保留原始 HTML",
            extracted.len(),
            math.len()
        );
        let mut out = md;
        for (i, span) in extracted.iter().enumerate() {
            out = out.replace(&math_marker(i), span);
        }
        return out;
    }
    let mut out = md;
    for (i, item) in math.iter().enumerate() {
        let rep = if item.block {
            format!("$$ {} $$", item.source)
        } else {
            format!("${}$", item.source)
        };
        out = out.replace(&math_marker(i), &rep);
    }
    out
}

// ---------------------------------------------------------------------------
// HTML → Markdown 转换（scraper DOM 走树）
// ---------------------------------------------------------------------------

type TreeNode<'a> = NodeRef<'a, Node>;

/// HTML 片段转 Markdown。块级元素之间以空行分隔。
fn html_to_markdown(html: &str) -> String {
    let dom = scraper::Html::parse_fragment(html);
    let root = dom.tree.root();
    let mut blocks: Vec<String> = Vec::new();
    for child in root.children() {
        blocks.extend(node_blocks(child));
    }
    blocks
        .into_iter()
        .filter(|b| !b.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// 块级节点 → Markdown 块列表。
fn node_blocks(node: TreeNode<'_>) -> Vec<String> {
    let Some(el) = node.value().as_element() else {
        // 顶层散落的文本（如公式占位标记）：当作一个块。
        if let Some(t) = node.value().as_text() {
            let s = collapse_ws(t);
            return if s.is_empty() { Vec::new() } else { vec![s] };
        }
        return Vec::new();
    };
    let tag = el.name();
    match tag {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            let level = "#".repeat(tag[1..].parse::<usize>().unwrap_or(1));
            vec![format!("{level} {}", inline_children(node))]
        }
        "p" => {
            let s = inline_children(node);
            if s.trim().is_empty() {
                Vec::new()
            } else {
                vec![s]
            }
        }
        "ul" | "ol" => list_blocks(node, tag == "ol", 0),
        "pre" => vec![code_block(node)],
        "blockquote" => {
            let mut inner: Vec<String> = Vec::new();
            for child in node.children() {
                inner.extend(node_blocks(child));
            }
            inner
                .into_iter()
                .filter(|b| !b.trim().is_empty())
                .map(|b| {
                    b.lines()
                        .map(|l| format!("> {}", l.trim()))
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .collect()
        }
        "table" => table_blocks(node),
        "hr" => vec!["---".to_string()],
        "figure" => {
            let mut inner: Vec<String> = Vec::new();
            for child in node.children() {
                inner.extend(node_blocks(child));
            }
            inner
        }
        "figcaption" => {
            let s = inline_children(node);
            if s.trim().is_empty() {
                Vec::new()
            } else {
                vec![format!("*{}*", s.trim())]
            }
        }
        // cetz 等插图的内联 SVG：Markdown 平台不支持矢量渲染，保留为原生
        // HTML 块（CommonMark 允许；不支持的平台会剥离，但不至于静默丢失）。
        "svg" => vec![element_html(node)],
        // 透明容器（typst 分组 div 等）：递归展开。
        _ => {
            let mut inner: Vec<String> = Vec::new();
            for child in node.children() {
                inner.extend(node_blocks(child));
            }
            if !inner.is_empty() {
                return inner;
            }
            // 无块级子节点：按行内元素处理（img / 带文本的 span 等）。
            let s = inline(node);
            if s.trim().is_empty() {
                Vec::new()
            } else {
                vec![s]
            }
        }
    }
}

/// 列表（支持嵌套，`depth` 为嵌套层数）。
fn list_blocks(node: TreeNode<'_>, ordered: bool, depth: usize) -> Vec<String> {
    let indent = "  ".repeat(depth);
    let mut lines: Vec<String> = Vec::new();
    let mut index = 0usize;
    for child in node.children() {
        let Some(el) = child.value().as_element() else {
            continue;
        };
        if el.name() != "li" {
            continue;
        }
        index += 1;
        let marker = if ordered {
            format!("{index}. ")
        } else {
            "- ".to_string()
        };
        let mut item_text = String::new();
        let mut nested: Vec<String> = Vec::new();
        for sub in child.children() {
            let is_list = sub
                .value()
                .as_element()
                .is_some_and(|e| e.name() == "ul" || e.name() == "ol");
            if is_list {
                let ordered_sub = sub.value().as_element().is_some_and(|e| e.name() == "ol");
                nested.extend(list_blocks(sub, ordered_sub, depth + 1));
            } else {
                item_text.push_str(&inline(sub));
            }
        }
        let item = item_text.trim().to_string();
        lines.push(format!("{indent}{marker}{item}"));
        lines.extend(nested);
    }
    vec![lines.join("\n")]
}

/// `<pre>` → 围栏代码块（语言取 pre/code 的 `data-lang` 或 code 的 class）。
fn code_block(node: TreeNode<'_>) -> String {
    let mut lang = String::new();
    let el = node.value().as_element().unwrap();
    if let Some(l) = el.attr("data-lang") {
        lang = l.to_string();
    }
    let mut code = String::new();
    for child in node.children() {
        if let Some(ce) = child.value().as_element()
            && ce.name() == "code"
        {
            if lang.is_empty() {
                if let Some(l) = ce.attr("data-lang") {
                    lang = l.to_string();
                } else if let Some(cls) = ce.attr("class")
                    && let Some(l) = cls.strip_prefix("language-")
                {
                    lang = l.to_string();
                }
            }
            collect_text(child, &mut code);
        } else {
            collect_text(child, &mut code);
        }
    }
    let fence = "`".repeat(3);
    format!("{fence}{lang}\n{}\n{fence}", code.trim_end())
}

/// `<table>` → GFM 管道表格。typst 输出首行常用 `<strong>` 代替 `<th>`，
/// 统一以首行作为表头。
fn table_blocks(node: TreeNode<'_>) -> Vec<String> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    collect_table_rows(node, &mut rows);
    if rows.is_empty() {
        return Vec::new();
    }
    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let rows: Vec<Vec<String>> = rows
        .into_iter()
        .map(|r| {
            let mut r = r;
            r.resize(width, String::new());
            r
        })
        .collect();
    let header = rows[0].clone();
    let sep: Vec<String> = vec!["---".to_string(); width];
    let body = &rows[1..];
    let line = |cells: &[String]| format!("| {} |", cells.join(" | "));
    let mut out = vec![line(&header), line(&sep)];
    for r in body {
        out.push(line(r));
    }
    vec![out.join("\n")]
}

fn collect_table_rows(node: TreeNode<'_>, rows: &mut Vec<Vec<String>>) {
    let Some(el) = node.value().as_element() else {
        return;
    };
    match el.name() {
        "tr" => {
            let mut cells = Vec::new();
            for td in node.children() {
                if td
                    .value()
                    .as_element()
                    .is_some_and(|e| e.name() == "td" || e.name() == "th")
                {
                    cells.push(inline_children(td).trim().to_string());
                }
            }
            rows.push(cells);
        }
        "table" | "thead" | "tbody" | "tfoot" => {
            for child in node.children() {
                collect_table_rows(child, rows);
            }
        }
        _ => {}
    }
}

/// 元素子节点的行内内容（单空格折叠后拼接）。
fn inline_children(node: TreeNode<'_>) -> String {
    let mut out = String::new();
    for child in node.children() {
        out.push_str(&inline(child));
    }
    out.trim().to_string()
}

/// 把元素节点序列化回 HTML（用于 svg 等需要原样保留的块）。
/// 元素的完整 HTML（含自身标签，供 svg 栅格化等目标复用）。
pub(crate) fn element_html(node: TreeNode<'_>) -> String {
    scraper::ElementRef::wrap(node)
        .map(|e| e.html())
        .unwrap_or_default()
}

/// 行内节点 → Markdown 行内文本。
fn inline(node: TreeNode<'_>) -> String {
    match node.value() {
        Node::Text(t) => escape_md(&collapse_ws(t)),
        Node::Element(el) => {
            let tag = el.name();
            match tag {
                "strong" | "b" => {
                    let inner = inline_children(node);
                    if inner.is_empty() {
                        String::new()
                    } else {
                        format!("**{inner}**")
                    }
                }
                "em" | "i" => {
                    let inner = inline_children(node);
                    if inner.is_empty() {
                        String::new()
                    } else {
                        format!("*{inner}*")
                    }
                }
                "del" | "s" => {
                    let inner = inline_children(node);
                    if inner.is_empty() {
                        String::new()
                    } else {
                        format!("~~{inner}~~")
                    }
                }
                "code" => {
                    let mut raw = String::new();
                    collect_text(node, &mut raw);
                    let raw = raw.trim();
                    if raw.is_empty() {
                        String::new()
                    } else if raw.contains('`') {
                        format!("`` {raw} ``")
                    } else {
                        format!("`{raw}`")
                    }
                }
                "a" => {
                    let inner = inline_children(node);
                    match el.attr("href") {
                        Some(href) if !href.starts_with('#') => {
                            format!("[{inner}]({href})")
                        }
                        // 站内锚点（如公式引用"式 1"）在平台端无意义，只留文本。
                        _ => inner,
                    }
                }
                "img" => {
                    let alt = el.attr("alt").unwrap_or("").to_string();
                    let src = el.attr("src").unwrap_or("").to_string();
                    format!("![{alt}]({src})")
                }
                "br" => "\n".to_string(),
                // script/style 与未知元素：内容照常展开（透明），script/style 跳过。
                "script" | "style" => String::new(),
                _ => {
                    let mut out = String::new();
                    for child in node.children() {
                        out.push_str(&inline(child));
                    }
                    out
                }
            }
        }
        Node::Comment(_) | Node::Doctype(_) | Node::Document | Node::Fragment => String::new(),
        Node::ProcessingInstruction(_) => String::new(),
    }
}

/// 递归收集节点子树的原始文本（用于 code/pre，不做 Markdown 转义）。
fn collect_text(node: TreeNode<'_>, out: &mut String) {
    match node.value() {
        Node::Text(t) => out.push_str(t),
        Node::Element(el) => {
            if el.name() == "br" {
                out.push('\n');
                return;
            }
            for child in node.children() {
                collect_text(child, out);
            }
        }
        _ => {}
    }
}

/// 折叠连续空白为单空格（typst-html pretty 输出带缩进换行）。
fn collapse_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// 保守的 Markdown 转义（代码块内不走此路径）。
fn escape_md(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' | '`' | '*' | '_' | '[' | ']' | '~' | '|' | '<' | '>' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_matching_close_walks_multibyte_safely() {
        // 回归：逐字节扫描遇中文曾在字符中间切片 panic（byte index not a char boundary）
        // 注意约定：from = 开始标签**之后**的位置（与 substitute_math 的真实调用一致），
        // 深度从 1 起算；若传标签自身位置会被重复计数导致永远配不上。
        let html = "<div>中文「公式」内容与<a href=\"x\">链接</a>混杂</div>尾巴";
        let end = find_matching_close(html, 5, "div").expect("应找到闭合位置");
        assert_eq!(&html[end..], "尾巴");
        // 嵌套同名标签按深度配对
        let nested = "<div>外<div>内</div></div>尾";
        let end2 = find_matching_close(nested, 5, "div").expect("嵌套应配对");
        assert_eq!(&nested[end2..], "尾");
    }

    #[test]
    fn state_roundtrip_upsert_find() {
        let dir = std::env::temp_dir().join(format!("typall-state-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let mut state = PublishState::default();
        assert!(state.find("markdown", "posts/a").is_none());
        state.upsert(PublishRecord {
            platform: "markdown".into(),
            slug: "posts/a".into(),
            content_hash: "h1".into(),
            published_at: "t".into(),
            assets: vec!["assets/x.svg".into()],
        });
        state.upsert(PublishRecord {
            platform: "markdown".into(),
            slug: "posts/a".into(),
            content_hash: "h2".into(),
            published_at: "t2".into(),
            assets: Vec::new(),
        });
        // 同键覆盖而非追加。
        assert_eq!(state.records.len(), 1);
        assert_eq!(state.find("markdown", "posts/a").unwrap().content_hash, "h2");
        state.save(&dir).unwrap();
        let loaded = PublishState::load(&dir).unwrap();
        assert_eq!(loaded.records.len(), 1);
        assert_eq!(loaded.records[0].published_at, "t2");

        // 损坏文件 → 明确报错而不是静默清零。
        std::fs::write(PublishState::path(&dir), "{not json").unwrap();
        assert!(PublishState::load(&dir).is_err());

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// 旧版状态库（含已移除的 `pushed_hash` / `remote_id` 字段）可正常读取；
    /// 重新落库后旧键消失（serde 忽略未知字段，向后兼容无需迁移）。
    #[test]
    fn publish_record_tolerates_legacy_remote_fields() {
        let dir = std::env::temp_dir().join(format!("typall-legacy-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".typall")).unwrap();
        std::fs::write(
            PublishState::path(&dir),
            r#"{"records":[{"platform":"wechat","slug":"posts/b","content_hash":"h",
            "pushed_hash":"h","published_at":"t","remote_id":"DRAFT-9","assets":[]}]}"#,
        )
        .unwrap();
        let state = PublishState::load(&dir).unwrap();
        assert_eq!(state.records.len(), 1);
        assert_eq!(state.records[0].platform, "wechat");
        // 重新保存：旧键不再出现。
        state.save(&dir).unwrap();
        let saved = std::fs::read_to_string(PublishState::path(&dir)).unwrap();
        assert!(!saved.contains("pushed_hash"));
        assert!(!saved.contains("remote_id"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// 编译指纹：meta 或正文任一变化 → 指纹变；内容不变 → 稳定。
    #[test]
    fn doc_fingerprint_sensitivity() {
        let mk = |title: &str, body: &str, author: Option<&str>| CompiledDoc {
            slug: "posts/fp".into(),
            meta: crate::content::DocumentMeta {
                title: title.into(),
                date: Some("2026-09-01".into()),
                excerpt: Some("摘要".into()),
                raw: match author {
                    Some(a) => {
                        let mut m = std::collections::HashMap::new();
                        m.insert("author".into(), crate::content::MetaValue::Str(a.into()));
                        m
                    }
                    None => std::collections::HashMap::new(),
                },
                ..Default::default()
            },
            body_html: body.into(),
            math_style: String::new(),
            math: Vec::new(),
        };
        let base = doc_fingerprint(&mk("标题", "<p>正文</p>", None), "<p>正文</p>");
        // 稳定
        assert_eq!(doc_fingerprint(&mk("标题", "<p>正文</p>", None), "<p>正文</p>"), base);
        // 正文变
        assert_ne!(doc_fingerprint(&mk("标题", "<p>正文改</p>", None), "<p>正文改</p>"), base);
        // 标题变
        assert_ne!(doc_fingerprint(&mk("新标题", "<p>正文</p>", None), "<p>正文</p>"), base);
        // 自定义 raw 字段变（author 影响微信草稿）
        assert_ne!(
            doc_fingerprint(&mk("标题", "<p>正文</p>", Some("龙焰喵")), "<p>正文</p>"),
            base
        );
    }

    #[test]
    fn decode_data_uri_forms() {
        // 1x1 红点 PNG。
        let png_b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
        let (ext, bytes) = decode_data_uri(&format!("data:image/png;base64,{png_b64}")).unwrap();
        assert_eq!(ext, "png");
        assert_eq!(&bytes[..4], b"\x89PNG");
        // svg+xml base64 形态可解码；utf8 形态与未知子类型拒绝。
        let svg_b64 = base64::engine::general_purpose::STANDARD.encode(b"<svg/>");
        let (ext, _) = decode_data_uri(&format!("data:image/svg+xml;base64,{svg_b64}")).unwrap();
        assert_eq!(ext, "svg");
        assert!(decode_data_uri("data:image/svg+xml;utf8,<svg/>").is_none());
        assert!(decode_data_uri("data:application/pdf;base64,AAAA").is_none());
    }

    #[test]
    fn extract_data_uri_dedupes_and_rewrites() {
        let dir = std::env::temp_dir().join(format!("typall-asset-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let assets = dir.join("assets");
        let png = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
        let html = format!(r#"<p><img src="{png}" alt="a">中 <img src="{png}" alt="b">重复</p>"#);

        let (rewritten, list) = extract_data_uri_images(&html, &assets, 1).unwrap();
        // 同内容两次引用 → 一个资产文件。
        assert_eq!(list, vec![format!("assets/{}.png", {
            let (_, bytes) = decode_data_uri(png).unwrap();
            hash_bytes(&bytes)
        })]);
        // slug 深度 1（posts/xxx）→ ../assets/ 相对路径，两个 src 都被重写。
        assert_eq!(rewritten.matches("../assets/").count(), 2);
        assert!(!rewritten.contains("data:image"));
        assert_eq!(std::fs::read_dir(&assets).unwrap().count(), 1);
        // 第二次调用：文件已存在不重写，列表一致（内容寻址幂等）。
        let (_, list2) = extract_data_uri_images(&html, &assets, 1).unwrap();
        assert_eq!(list, list2);

        // 非 base64 的 data URI 原样保留。
        let (keep, none) =
            extract_data_uri_images(r#"<img src="data:image/svg+xml;utf8,<svg/>">"#, &assets, 0)
                .unwrap();
        assert!(none.is_empty() && keep.contains("data:image/svg+xml"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn substitute_math_handles_span_and_div() {
        let html = r#"<p>前 <span data-equation="inline"><svg>inline-eq</svg></span> 后</p>
<div data-equation="block"><svg>block-eq</svg></div>
<p>普通 <em>span</em> 不动</p>"#;
        let (marked, spans) = substitute_math(html);
        assert_eq!(spans.len(), 2);
        assert!(spans[0].contains("inline-eq"));
        assert!(spans[1].contains("block-eq"));
        assert_eq!(
            marked,
            format!(
                "<p>前 {} 后</p>\n{}\n<p>普通 <em>span</em> 不动</p>",
                math_marker(0),
                math_marker(1)
            )
        );
    }

    #[test]
    fn substitute_math_handles_mathml_tag() {
        let html = "<p>x <math><mi>a</mi></math> y</p>";
        let (marked, spans) = substitute_math(html);
        assert_eq!(spans.len(), 1);
        assert_eq!(marked, format!("<p>x {} y</p>", math_marker(0)));
    }

    #[test]
    fn substitute_math_nested_same_tag() {
        let html = r#"<div data-equation="block"><div>inner</div><svg>x</svg></div>尾"#;
        let (marked, spans) = substitute_math(html);
        assert_eq!(spans.len(), 1);
        assert!(spans[0].contains("inner"));
        assert_eq!(marked, format!("{}尾", math_marker(0)));
    }

    #[test]
    fn convert_body_restores_math_by_order() {
        let html = r#"<p>行内 <span data-equation="inline"><svg>a</svg></span> 与块级：</p><div data-equation="block"><svg>b</svg></div>"#;
        let math = vec![
            MathItem { source: "x^2".into(), block: false },
            MathItem { source: "sum i".into(), block: true },
        ];
        let md = convert_body("posts/t", html, &math);
        assert!(md.contains("行内 $x^2$ 与块级："));
        assert!(md.contains("$$ sum i $$"));
    }

    #[test]
    fn convert_body_mismatch_falls_back_to_raw_html() {
        let html = r#"<p>a <span data-equation="inline"><svg>x</svg></span> b</p>"#;
        let math: Vec<MathItem> = Vec::new();
        let md = convert_body("posts/t", html, &math);
        // 配对失败 → 原始片段回填
        assert!(md.contains("<svg>x</svg>"));
        assert!(!md.contains("TYPALLMATH"));
    }

    #[test]
    fn html_to_markdown_basics() {
        let html = "<h2>标题</h2><p><strong>粗</strong>与<em>斜</em>和<code>x=1</code></p>";
        let md = html_to_markdown(html);
        assert!(md.contains("## 标题"));
        assert!(md.contains("**粗**"));
        assert!(md.contains("*斜*"));
        assert!(md.contains("`x=1`"));
    }

    #[test]
    fn html_to_markdown_links_and_images() {
        let html = r##"<p><a href="https://a.b">外链</a> <a href="#loc-1">式 1</a></p><img src="data:image/png;base64,xx" alt="图">"##;
        let md = html_to_markdown(html);
        assert!(md.contains("[外链](https://a.b)"));
        // 站内锚点链接只留文本
        assert!(md.contains("式 1"));
        assert!(!md.contains("#loc-1"));
        assert!(md.contains("![图](data:image/png;base64,xx)"));
    }

    #[test]
    fn html_to_markdown_list_nesting() {
        let html = "<ul><li>外一</li><li>外二<ol><li>内一</li><li>内二</li></ol></li></ul>";
        let md = html_to_markdown(html);
        assert!(md.contains("- 外一"));
        assert!(md.contains("- 外二"));
        assert!(md.contains("  1. 内一"));
        assert!(md.contains("  2. 内二"));
    }

    #[test]
    fn html_to_markdown_table() {
        let html = "<table><tbody><tr><td>A</td><td>B</td></tr><tr><td>1</td><td>2</td></tr></tbody></table>";
        let md = html_to_markdown(html);
        assert!(md.contains("| A | B |"));
        assert!(md.contains("| --- | --- |"));
        assert!(md.contains("| 1 | 2 |"));
    }

    #[test]
    fn html_to_markdown_code_block() {
        let html = r#"<pre><code data-lang="python">def f():
    return 1</code></pre>"#;
        let md = html_to_markdown(html);
        assert!(md.starts_with("```python\n"));
        assert!(md.contains("def f():"));
        assert!(md.trim_end().ends_with("```"));
    }

    #[test]
    fn html_to_markdown_blockquote_and_hr() {
        let html = "<blockquote><p>引文</p></blockquote><hr>";
        let md = html_to_markdown(html);
        assert!(md.contains("> 引文"));
        assert!(md.contains("---"));
    }

    #[test]
    fn html_to_markdown_escapes_markdown_chars() {
        let html = "<p>1 * 2 _ 3 [x] |y|</p>";
        let md = html_to_markdown(html);
        assert!(md.contains("\\*"));
        assert!(md.contains("\\_"));
        assert!(md.contains("\\[x\\]"));
        assert!(md.contains("\\|y\\|"));
    }

    #[test]
    fn html_to_markdown_preserves_svg_blocks() {
        let html = "<p>插图：</p><svg viewBox=\"0 0 10 10\"><path d=\"M0 0\"></path></svg><p>之后</p>";
        let md = html_to_markdown(html);
        assert!(md.contains("插图："));
        assert!(md.contains("<svg viewBox=\"0 0 10 10\">"));
        assert!(md.contains("<path d=\"M0 0\">"));
        assert!(md.contains("之后"));
    }

    #[test]
    fn marker_survives_conversion() {
        let html = "<p>前 TYPALLMATH0TYPALL 后</p>";
        let md = html_to_markdown(html);
        assert!(md.contains("TYPALLMATH0TYPALL"));
    }

    #[test]
    fn render_markdown_frontmatter() {
        let doc = CompiledDoc {
            slug: "posts/t".into(),
            meta: crate::content::DocumentMeta {
                title: "标题: 带冒号".into(),
                date: Some("2026-09-01".into()),
                tags: vec!["物理".into()],
                ..Default::default()
            },
            body_html: "<p>正文</p>".into(),
            math_style: String::new(),
            math: Vec::new(),
        };
        let md = render_markdown(&doc, &doc.body_html);
        assert!(md.starts_with("---\n"));
        assert!(md.contains("title: \"标题: 带冒号\""));
        assert!(md.contains("date: \"2026-09-01\""));
        assert!(md.contains("tags: [\"物理\"]"));
        assert!(md.contains("draft: false"));
        assert!(md.contains("math: typst"));
        assert!(md.ends_with("正文"));
    }
}
