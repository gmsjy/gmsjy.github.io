//! Typall 命令行入口。

mod build;
mod cache;
mod compile;
mod config;
mod content;
mod deploy;
mod feed;
mod ir;
mod packages;
mod pdf;
mod publish;
mod serve;
mod site;
mod social_card;
mod site_html;
mod theme;
mod typst_compat;
mod theme_defaults;
mod theme_pkg;
mod wechat;
mod writer;
mod world;

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "typall", version, about = "A self-contained static blog generator powered by Typst")]
struct Cli {
    /// 项目根目录（应含 typall.toml 与 posts/；默认为当前目录）
    #[arg(long, short = 'C', global = true, value_name = "DIR")]
    dir: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 创建新项目脚手架
    Init { name: Option<String> },
    /// 在 posts/ 下生成新文章
    New { post: String },
    /// 构建完整站点
    Build {
        /// 启用严格模式（死链检查）
        #[arg(long)]
        strict: bool,
        /// 构建时包含草稿
        #[arg(long)]
        drafts: bool,
        /// 自定义输出目录
        #[arg(long)]
        output: Option<String>,
    },
    /// 启动开发服务器（Live Reload）
    Serve {
        #[arg(long)]
        port: Option<u16>,
        /// 绑定地址：127.0.0.1 仅本机；0.0.0.0 开放局域网（手机扫码预览）
        #[arg(long)]
        host: Option<String>,
        /// 自动在浏览器打开
        #[arg(long)]
        open: bool,
    },
    /// 按配置部署站点
    Deploy {
        #[arg(long)] target: Option<String>,
        /// 仅模拟，不实际执行
        #[arg(long)]
        dry_run: bool,
    },
    /// 将文章发布到目标平台格式（单篇 → 平台，与 deploy 整站部署相对）
    Publish {
        /// 发布目标：markdown / wechat（省略时默认 markdown）
        #[arg(long)]
        to: Option<String>,
        /// 自定义导出目录（默认取 [publish] out_dir，再默认 publish/）
        #[arg(long)]
        output: Option<String>,
        /// 只发布指定文章（slug 如 posts/quantum 或 quantum）
        #[arg(long)]
        slug: Option<String>,
        /// 包含草稿
        #[arg(long)]
        drafts: bool,
        /// 内容未变也强制重新发布
        #[arg(long)]
        force: bool,
        /// 只查看发布状态，不执行发布
        #[arg(long)]
        status: bool,
        /// 仅打印计划，不写盘、不改动状态库
        #[arg(long)]
        dry_run: bool,
    },
    /// 删除构建产物
    Clean,
    /// 列出文章清单（状态：已发布 / 草稿 / 定时）
    List,
    /// 站点概况（文章统计 / 配置 / 上次构建）
    Status,
    /// 重命名文章 slug，并自动把旧 slug 写入 aliases（旧链接不 404）
    Mv {
        /// 旧 slug（如 posts/old-name 或 old-name）
        old: String,
        /// 新 slug（如 posts/new-name 或 new-name）
        new: String,
    },
    /// 写入 CI 工作流模板（当前支持：github）
    Ci {
        /// CI 平台
        #[arg(default_value = "github")]
        target: String,
    },
    /// 语法检查（不生成输出）
    Check {
        /// 启用严格模式（若存在构建产物则做死链检查）
        #[arg(long)]
        strict: bool,
    },
    /// 主题管理（打包 / 安装）
    Theme {
        #[command(subcommand)]
        cmd: ThemeCommand,
    },
    /// 导出文章为 PDF（单篇或全部）
    Pdf {
        /// 只导出指定文章（slug 如 posts/quantum 或 quantum；省略则导出全部）
        #[arg(long)]
        slug: Option<String>,
        /// 输出目录（默认 pdf/）
        #[arg(long)]
        output: Option<String>,
        /// 包含草稿
        #[arg(long)]
        drafts: bool,
    },
    /// 显示版本信息
    Version,
}

#[derive(Subcommand)]
enum ThemeCommand {
    /// 打包 themes/<name>/ 为 themes-<name>.zip（默认取配置 theme.name）
    Package {
        /// 主题名
        #[arg(long)]
        name: Option<String>,
    },
    /// 从本地 zip 路径或 http(s) URL 安装主题到 themes/
    Install {
        /// zip 文件路径或 URL
        source: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    // 项目根：--dir 指定（相对路径按当前目录解析），否则取当前目录。
    // 指定的目录必须真实存在——尽早报错，好过「空站点无文章」的静默陷阱。
    let root = match &cli.dir {
        Some(dir) => {
            let abs = if dir.is_absolute() {
                dir.clone()
            } else {
                std::env::current_dir()?.join(dir)
            };
            if !abs.exists() {
                anyhow::bail!("指定的项目目录不存在：{}", abs.display());
            }
            abs.canonicalize()?
        }
        None => std::env::current_dir()?,
    };

    match cli.command {
        Command::Init { name } => cmd_init(name.as_deref()),
        Command::New { post } => cmd_new(&root, &post),
        Command::Build { strict, drafts, output } => {
            let mut config = config::Config::load(&root)?;
            if strict {
                config.build.strict = true;
            }
            if let Some(dir) = output {
                config.build.output_dir = dir;
            }
            build::build(&root, &config, drafts)
        }
        Command::Check { strict } => build::check(&root, strict),
        Command::List => cmd_list(&root),
        Command::Status => cmd_status(&root),
        Command::Mv { old, new } => cmd_mv(&root, &old, &new),
        Command::Ci { target } => cmd_ci(&root, &target),
        Command::Clean => {
            let config = config::Config::load(&root)?;
            let out = config.output_dir(&root);
            if out.exists() {
                // 防呆：只允许删除项目根内的输出目录。output_dir 误配为 "."
                // 或指向项目外的路径时拒绝执行，避免删掉整个项目。
                let root_canon = root.canonicalize()?;
                let out_canon = out.canonicalize()?;
                if out_canon == root_canon || !out_canon.starts_with(&root_canon) {
                    anyhow::bail!(
                        "output_dir = {:?} 不在项目根 {} 内，已拒绝删除",
                        out.display(),
                        root.display()
                    );
                }
                std::fs::remove_dir_all(&out)?;
                println!("✅ 已删除 {}", out.display());
            } else {
                println!("无构建产物需要清理");
            }
            // 编译缓存与 manifest 可安全重建；发布状态（publish.json）保留，
            // 丢了会导致下次全量重新导出。微信 token/媒体缓存已随网络层移除，
            // 这里顺手清掉历史项目残留的死文件。
            let cache = root.join(".typall");
            let cache_dir = cache.join("cache");
            if cache_dir.exists() {
                std::fs::remove_dir_all(&cache_dir)?;
                println!("✅ 已删除编译缓存 {}", cache_dir.display());
            }
            let manifest = cache.join("manifest.txt");
            if manifest.exists() {
                std::fs::remove_file(&manifest)?;
                println!("✅ 已删除 {}", manifest.display());
            }
            for stale in ["wechat_token.json", "wechat_media.json"] {
                let p = cache.join(stale);
                if p.exists() {
                    std::fs::remove_file(&p)?;
                    println!("✅ 已删除废弃微信缓存 {}", p.display());
                }
            }
            println!("ℹ️ 发布状态已保留（publish.json）");
            Ok(())
        }
        Command::Version => {
            println!("typall {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Command::Theme { cmd } => match cmd {
            ThemeCommand::Package { name } => {
                let config = config::Config::load(&root)?;
                let n = name.unwrap_or_else(|| {
                    if config.theme.name.is_empty() {
                        "default".to_string()
                    } else {
                        config.theme.name.clone()
                    }
                });
                if n == "default" {
                    anyhow::bail!("内置默认主题无需打包（请先创建 themes/<name>/）");
                }
                let out = theme_pkg::package(&root, &n)?;
                println!("✅ 已打包主题: {}", out.display());
                Ok(())
            }
            ThemeCommand::Install { source } => {
                let name = theme_pkg::install(&root, &source)?;
                println!(
                    "✅ 已安装主题: {name}（在 typall.toml 设置 theme.name = \"{name}\" 后生效）"
                );
                Ok(())
            }
        },
        Command::Serve { port, open, host } => {
            let config = config::Config::load(&root)?;
            serve::serve(&root, config, port.unwrap_or(8080), open, host.as_deref())
        }
        Command::Deploy { target, dry_run } => {
            let config = config::Config::load(&root)?;
            deploy::deploy(&root, &config, target.as_deref(), dry_run)
        }
        Command::Publish { to, output, slug, drafts, force, status, dry_run } => {
            let config = config::Config::load(&root)?;
            publish::publish(
                &root,
                &config,
                &publish::PublishOptions {
                    target: to,
                    output,
                    slug,
                    drafts,
                    force,
                    status,
                    dry_run,
                },
            )
        }
        Command::Pdf { slug, output, drafts } => {
            let config = config::Config::load(&root)?;
            pdf::export(&root, &config, slug.as_deref(), output.as_deref(), drafts)
        }
    }
}

/// 文章条目状态（`typall list` / `status` 用）。
#[derive(Debug, Clone, Copy, PartialEq)]
enum PostState {
    Published,
    Draft,
    Scheduled,
}

impl PostState {
    fn label(&self) -> &'static str {
        match self {
            PostState::Published => "已发布",
            PostState::Draft => "草稿",
            PostState::Scheduled => "定时",
        }
    }
}

/// 从单个 .typ 源解析文章条目（slug / 状态 / 日期 / 标题）。零编译，毫秒级。
fn collect_post_states(root: &Path) -> anyhow::Result<Vec<(String, PostState, String, String)>> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut rows = Vec::new();
    for file in content::scan_typ_files(&root.join("posts")) {
        let source = std::fs::read_to_string(&file)?;
        let meta = content::DocumentMeta::from_map(&content::extract_meta(&source));
        let slug = content::rel_path(root, &file)
            .trim_end_matches(".typ")
            .replace('\\', "/");
        let state = if meta.draft {
            PostState::Draft
        } else if let Some(d) = &meta.date {
            if !d.is_empty() && d.as_str() > today.as_str() {
                PostState::Scheduled
            } else {
                PostState::Published
            }
        } else {
            PostState::Published
        };
        let title = if meta.title.is_empty() {
            slug.clone()
        } else {
            meta.title
        };
        rows.push((slug, state, meta.date.unwrap_or_default(), title));
    }
    // 日期倒序，同新文章在前
    rows.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    Ok(rows)
}

fn cmd_list(root: &Path) -> anyhow::Result<()> {
    let rows = collect_post_states(root)?;
    if rows.is_empty() {
        println!("posts/ 下没有文章");
        return Ok(());
    }
    println!("{:<6} {:<12} {:<44} slug", "状态", "日期", "标题");
    for (slug, state, date, title) in &rows {
        let title = if title.chars().count() > 20 {
            format!("{}…", title.chars().take(19).collect::<String>())
        } else {
            title.clone()
        };
        println!("{:<6} {:<12} {:<44} {}", state.label(), date, title, slug);
    }
    let (mut pub_n, mut draft_n, mut sched_n) = (0, 0, 0);
    for (_, s, _, _) in &rows {
        match s {
            PostState::Published => pub_n += 1,
            PostState::Draft => draft_n += 1,
            PostState::Scheduled => sched_n += 1,
        }
    }
    println!(
        "\n共 {} 篇：已发布 {} · 草稿 {} · 定时 {}",
        rows.len(),
        pub_n,
        draft_n,
        sched_n
    );
    Ok(())
}

fn cmd_status(root: &Path) -> anyhow::Result<()> {
    let rows = collect_post_states(root)?;
    let (pub_n, draft_n, sched_n) = rows.iter().fold((0, 0, 0), |acc, (_, s, _, _)| match s {
        PostState::Published => (acc.0 + 1, acc.1, acc.2),
        PostState::Draft => (acc.0, acc.1 + 1, acc.2),
        PostState::Scheduled => (acc.0, acc.1, acc.2 + 1),
    });
    let pages_n = content::scan_typ_files(&root.join("pages")).len();
    let config = config::Config::load(root)?;

    println!("站点概况");
    println!("  站点名   : {}", config.site.title);
    println!("  主题     : {}", config.theme.name);
    println!(
        "  site.url : {}",
        if config.site.url.is_empty() { "（未配置：SEO/订阅/分享卡已停用）" } else { &config.site.url }
    );
    println!("  输出目录 : {}", config.build.output_dir);
    println!(
        "  文章     : 共 {} 篇 —— 已发布 {} · 草稿 {} · 定时 {}",
        rows.len(),
        pub_n,
        draft_n,
        sched_n
    );
    println!("  独立页面 : {pages_n} 个");
    let manifest = root.join(".typall").join("manifest.txt");
    if let Ok(meta) = std::fs::metadata(&manifest)
        && let Ok(mtime) = meta.modified()
    {
        let ago = chrono::Local::now()
            .signed_duration_since(chrono::DateTime::<chrono::Local>::from(mtime));
        println!("  上次构建 : {} 前（manifest.txt）", format_mins(ago.num_minutes().max(0)));
    } else {
        println!("  上次构建 : 无记录（尚未构建过）");
    }
    let ledger = root.join(".typall").join("publish.json");
    if ledger.exists() {
        println!("  发布账本 : {}（微信/Markdown 发布状态已记录）", ledger.display());
    }
    if sched_n > 0 {
        println!("\n💡 有 {} 篇定时文章等待上线：到点后需重新构建（`typall build` / CI 定时任务 / serve 常驻）才会发布。", sched_n);
    }
    Ok(())
}

/// 分钟数 → 「x 天 x 小时」式可读时长（status 用，非关键路径）。
fn format_mins(mins: i64) -> String {
    if mins < 60 {
        format!("{mins} 分钟")
    } else if mins < 60 * 24 {
        format!("{} 小时", mins / 60)
    } else {
        let days = mins / (60 * 24);
        let hours = (mins % (60 * 24)) / 60;
        if hours > 0 {
            format!("{days} 天 {hours} 小时")
        } else {
            format!("{days} 天")
        }
    }
}

/// slug 规范化：去 `posts/` 前缀与首尾斜杠，返回相对 posts/ 的纯名。
fn normalize_slug(slug: &str) -> String {
    slug.trim_matches('/')
        .strip_prefix("posts/")
        .unwrap_or(slug.trim_matches('/'))
        .to_string()
}

fn cmd_mv(root: &Path, old: &str, new: &str) -> anyhow::Result<()> {
    let old = normalize_slug(old);
    let new = normalize_slug(new);
    if new.is_empty() || old.is_empty() {
        anyhow::bail!("slug 不能为空");
    }
    for s in [&old, &new] {
        if s.contains("..") || s.contains('\\') || s.contains('"') {
            anyhow::bail!("非法 slug：{s}");
        }
    }
    let old_path = root.join("posts").join(format!("{old}.typ"));
    let new_path = root.join("posts").join(format!("{new}.typ"));
    if !old_path.exists() {
        anyhow::bail!("源文章不存在：{}", old_path.display());
    }
    if new_path.exists() {
        anyhow::bail!("目标已存在：{}", new_path.display());
    }
    let mut source = std::fs::read_to_string(&old_path)?;

    // 在 front-matter 区（首个空行前）注入/补全 aliases，使旧链接继续可达。
    // 别名记录完整 slug（含 posts/ 前缀）——旧文章 URL 是 /posts/<name>/。
    let old_full = format!("posts/{old}");
    let alias_entry = format!("{old_full:?}");
    if !source.contains("#let aliases") {
        let mut lines: Vec<String> = source.lines().map(String::from).collect();
        let fm_end = lines.iter().position(|l| l.trim().is_empty()).unwrap_or(0);
        lines.insert(fm_end, format!("#let aliases = ({alias_entry},)"));
        source = lines.join("\n") + "\n";
    } else {
        // 已有 aliases 行：把旧 slug 追加进元组
        let mut replaced = false;
        source = source
            .lines()
            .map(|l| {
                if !replaced && l.trim_start().starts_with("#let aliases") {
                    replaced = true;
                    if let Some(open) = l.find('(')
                        && let Some(close) = l.rfind(')') {
                            let items = &l[open + 1..close];
                            let trimmed = items.trim();
                            let comma = if trimmed.is_empty() {
                                String::new()
                            } else {
                                format!("{},", trimmed.trim_end_matches(','))
                            };
                            return format!(
                                "{}({}{alias_entry},){}",
                                &l[..open],
                                comma,
                                &l[close + 1..]
                            );
                        }
                    l.to_string()
                } else {
                    l.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    std::fs::write(&old_path, source)?;
    std::fs::rename(&old_path, &new_path)?;
    println!("✅ posts/{old}.typ → posts/{new}.typ");
    println!("   旧 slug `{old}` 已写入 aliases，重新构建后 /{old}/ 会自动跳转到 /{new}/");
    println!("   下一步：typall build");
    Ok(())
}

const GITHUB_WORKFLOW: &str = r#"# 由 `typall ci github` 生成：构建并部署静态站到 GitHub Pages。
# 同时内置每日定时构建——「定时发布」（未来日期文章到点自动上线）由此生效。
name: deploy

on:
  push:
    branches: [main]
  schedule:
    - cron: "0 21 * * *"   # 每天 UTC 21:00（北京时间次日 5:00）构建一次
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # —— 获取 typall 二进制（二选一，按你的分发方式改）——
      # 方式 A：typall 发布到 crates.io 后
      - run: cargo install typall
      # 方式 B：从生成器仓库的 release 下载（产物名与 release.yml 矩阵一致）
      # - run: |
      #     curl -L -o typall.tar.gz https://github.com/gmsjy/typall/releases/latest/download/typall-linux-x64.tar.gz
      #     tar xzf typall.tar.gz && mv typall ~/.cargo/bin/

      - name: Build site
        run: typall build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: public

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
    steps:
      - id: deployment
        uses: actions/deploy-pages@v4
"#;

fn cmd_ci(root: &Path, target: &str) -> anyhow::Result<()> {
    match target {
        "github" => {
            let dir = root.join(".github/workflows");
            std::fs::create_dir_all(&dir)?;
            let path = dir.join("deploy.yml");
            if path.exists() {
                anyhow::bail!("{} 已存在，避免覆盖请手动处理", path.display());
            }
            std::fs::write(&path, GITHUB_WORKFLOW)?;
            println!("✅ 已写入 {}", path.display());
            println!("   - push 到 main 即构建发布");
            println!("   - 每日 UTC 21:00 定时构建：未来日期文章（定时发布）到点自动上线");
            println!("   - 仓库 Settings → Pages → Source 选 GitHub Actions 后生效");
            Ok(())
        }
        other => anyhow::bail!("暂不支持 CI 平台：{other}（当前支持 github）"),
    }
}

fn cmd_init(name: Option<&str>) -> anyhow::Result<()> {
    let dir = match name {
        Some(n) => PathBuf::from(n),
        None => PathBuf::from("."),
    };
    for sub in ["posts", "pages", "assets/images", "assets/fonts", "themes"] {
        std::fs::create_dir_all(dir.join(sub))?;
    }

    let config_toml = r##"[site]
title = "我的博客"
description = "由 Typall 驱动"
author = ""
language = "zh-CN"

[build]
output_dir = "public"
strict = false
drafts = false

[build.math]
renderer = "mathml"
number_equations = true
equation_prefix = "公式"

[theme]
name = "default"              # 或改成 "my-theme"，对应 themes/my-theme/ 目录
# 主题参数：在 themes/<name>/template.html 与 style.css 中用 {{params.xxx}} 引用
# [theme.params]
# accent_color = "#007acc"
"##;
    std::fs::write(dir.join("typall.toml"), config_toml)?;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let sample = format!(
        r#"#let title = "你好，Typall"
#let date = "{today}"
#let tags = ("示例",)
#let draft = false

= 欢迎使用 Typall

这是一个示例文章，用 *Typst* 编写。

行内公式：$E = m c^2$。

独立公式：

$ integral_(-oo)^(oo) e^(-x^2) dif x = sqrt(pi) $
"#
    );
    std::fs::write(dir.join("posts").join("hello.typ"), sample)?;

    println!("✅ 项目已初始化于 {}", dir.display());
    println!("   下一步：cd {} && typall build && typall serve", dir.display());
    Ok(())
}

fn cmd_new(root: &Path, post: &str) -> anyhow::Result<()> {
    let name = post.trim();
    // 输入校验：文件名与生成的 Typst 源都来自用户输入。
    // 拒绝路径分隔符/`..`（越界写文件）与 `"`（注入 #let title = "…" 破坏源码）。
    if name.is_empty() {
        anyhow::bail!("文章名不能为空");
    }
    if name.starts_with('.') || name.contains("..") {
        anyhow::bail!("文章名不能以 . 开头或包含 ..：{post}");
    }
    let forbidden = ['/', '\\', '"', '\'', '<', '>', ':', '|', '?', '*'];
    if let Some(c) = name.chars().find(|c| forbidden.contains(c) || c.is_control()) {
        anyhow::bail!("文章名含非法字符 `{c}`（禁止路径分隔符、引号与 Windows 保留字符）：{post}");
    }
    let posts = root.join("posts");
    std::fs::create_dir_all(&posts)?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("{today}-{name}.typ");
    let content = format!(
        r#"#let title = "{name}"
#let date = "{today}"
#let tags = ()
#let draft = false

#import "../assets/preview.typ": fig, eq-numbering
#show math.equation.where(block: true): set math.equation(numbering: eq-numbering)
#show math.equation.where(block: false): set math.equation(numbering: none)

= {name}
"#
    );
    std::fs::write(posts.join(&filename), content)?;
    println!("✅ 已创建 posts/{filename}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_project() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let posts = tmp.path().join("posts");
        std::fs::create_dir_all(&posts).unwrap();
        std::fs::write(
            posts.join("published.typ"),
            "#let title = \"已发布\"\n#let date = \"2026-09-01\"\n\n正文。\n",
        )
        .unwrap();
        std::fs::write(
            posts.join("drafted.typ"),
            "#let title = \"草稿\"\n#let date = \"2026-09-02\"\n#let draft = true\n\n正文。\n",
        )
        .unwrap();
        std::fs::write(
            posts.join("scheduled.typ"),
            "#let title = \"定时\"\n#let date = \"2099-01-01\"\n\n正文。\n",
        )
        .unwrap();
        tmp
    }

    #[test]
    fn collect_post_states_classifies() {
        let tmp = temp_project();
        let rows = collect_post_states(tmp.path()).unwrap();
        assert_eq!(rows.len(), 3);
        let by_slug: std::collections::HashMap<_, _> =
            rows.iter().map(|(s, st, d, t)| (s.as_str(), (*st, d.clone(), t.clone()))).collect();
        assert_eq!(by_slug["posts/published"].0, PostState::Published);
        assert_eq!(by_slug["posts/drafted"].0, PostState::Draft);
        assert_eq!(by_slug["posts/scheduled"].0, PostState::Scheduled);
        // 日期倒序：定时(2099) 最前
        assert_eq!(rows[0].0, "posts/scheduled");
    }

    #[test]
    fn mv_renames_and_injects_alias() {
        let tmp = temp_project();
        std::fs::write(
            tmp.path().join("posts/rename-me.typ"),
            "#let title = \"改名\"\n#let date = \"2026-09-03\"\n\n正文。\n",
        )
        .unwrap();
        cmd_mv(tmp.path(), "posts/rename-me", "rename-done").unwrap();
        let src = std::fs::read_to_string(tmp.path().join("posts/rename-done.typ")).unwrap();
        assert!(src.contains("#let aliases = (\"posts/rename-me\",)"));
        assert!(!tmp.path().join("posts/rename-me.typ").exists());
        // 幂等：新文件再次 mv 到另一名，aliases 行被追加而非新建
        cmd_mv(tmp.path(), "posts/rename-done", "rename-done2").unwrap();
        let src2 = std::fs::read_to_string(tmp.path().join("posts/rename-done2.typ")).unwrap();
        // 追加进已有元组（格式紧凑但合法）
        assert!(src2.contains("\"posts/rename-me\""));
        assert!(src2.contains("\"posts/rename-done\""));
        assert!(!src2.contains("\"posts/rename-done2\""));
    }

    #[test]
    fn ci_github_writes_workflow_with_cron() {
        let tmp = temp_project();
        cmd_ci(tmp.path(), "github").unwrap();
        let wf = tmp.path().join(".github/workflows/deploy.yml");
        assert!(wf.exists());
        let text = std::fs::read_to_string(wf).unwrap();
        assert!(text.contains("cron:"));
        assert!(text.contains("typall build"));
        // 二次生成拒绝覆盖
        assert!(cmd_ci(tmp.path(), "github").is_err());
    }
}
