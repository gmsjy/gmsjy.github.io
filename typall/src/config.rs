//! 配置系统：加载 `typall.toml` / `typall.yaml`，支持 `TP_` 环境变量覆盖。

use std::path::Path;

use serde::Deserialize;

/// 串行化所有涉及环境变量的测试：`set_var`/`remove_var` 是进程级全局状态，
/// 并行测试会互相污染（谁后 set 谁生效）。所有动 env 的测试统一取此锁。
#[cfg(test)]
pub(crate) static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub site: SiteConfig,
    pub build: BuildConfig,
    pub theme: ThemeConfig,
    pub deploy: DeployConfig,
    /// 平台发布（扩展余地核查 · 阶段 0 预留）：与 `deploy`（整站 → 主机）
    /// 是两种动作——publish 是单篇 → 平台，带远端状态。字段先行占位，
    /// 当前版本**解析但不消费**；`typall publish` 子命令在后续阶段实现。
    pub publish: PublishConfig,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub url: String,
    pub author: String,
    pub language: String,
    /// 原样注入每个页面 `</head>` 前的统计/分析 HTML（`{{analytics}}`）。
    /// 内容不做转义，可放 `<script>`（umami / Google Analytics 等）；
    /// 未配置为空串。
    pub analytics: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct BuildConfig {
    pub output_dir: String,
    pub strict: bool,
    pub drafts: bool,
    pub posts_per_page: usize,
    pub math: BuildMathConfig,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct BuildMathConfig {
    /// 公式渲染策略：`mathml`（默认）或 `svg`（v0.3 实现）。
    pub renderer: String,
    pub number_equations: bool,
    pub equation_prefix: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct ThemeConfig {
    pub name: String,
    pub params: toml::Table,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct DeployConfig {
    pub strategy: Option<String>,
    /// 部署前执行的 shell 命令（非空时在部署前执行）。
    pub pre_deploy: String,
    /// 部署后执行的 shell 命令（非空时在部署成功后执行）。
    pub post_deploy: String,
    pub git: GitDeploy,
    pub copy: CopyDeploy,
    pub netlify: NetlifyDeploy,
    pub vercel: VercelDeploy,
    /// 定时/周期部署（`[deploy.schedule]`）：serve 常驻期间按计划自动
    /// 构建并执行一次部署。缺省（两字段皆空）= 停用。
    pub schedule: DeploySchedule,
}

/// 定时/周期部署计划。
///
/// ```toml
/// [deploy.schedule]
/// every = "30m"                 # 周期触发：30m / 2h / 1d（≥1 分钟）
/// daily = ["06:30", "21:00"]    # 每日定点触发（本地时区 HH:MM，可多个）
/// ```
/// 两者可同时配置，到点即构建并按 `[deploy] strategy` 部署一次；
/// 定时发布（未来日期文章到点自动上线）因此无需依赖外部 CI cron。
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct DeploySchedule {
    /// 周期间隔（如 "30m" / "2h" / "1d"），空 = 停用。
    pub every: String,
    /// 每日定点时刻列表（"HH:MM"），空 = 停用。
    pub daily: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct GitDeploy {
    pub repo: String,
    pub branch: String,
    pub commit_message: String,
    #[serde(default = "default_true")]
    pub auto_push: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct CopyDeploy {
    pub target_dir: String,
}

/// Netlify 部署配置（v0.3 实现）。
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct NetlifyDeploy {
    pub site_id: String,
    pub auth_token: String,
}

/// Vercel 部署配置（v0.3 实现）。
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct VercelDeploy {
    pub project_id: String,
    pub auth_token: String,
}

/// 平台发布配置：本地导出目录（`typall publish` 各目标的输出根）。
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct PublishConfig {
    #[serde(default = "default_publish_out_dir")]
    pub out_dir: String,
}

fn default_publish_out_dir() -> String {
    "publish".to_string()
}

fn default_true() -> bool {
    true
}

impl Config {
    /// 从项目根目录加载配置；文件不存在时使用默认值。
    /// 之后用 `TP_` 前缀环境变量覆盖（便于 CI/CD）。
    pub fn load(root: &Path) -> anyhow::Result<Self> {
        let toml_path = root.join("typall.toml");
        let yaml_path = root.join("typall.yaml");

        let mut config = if toml_path.exists() {
            let text = std::fs::read_to_string(&toml_path)?;
            toml::from_str::<Config>(&text)?
        } else if yaml_path.exists() {
            let text = std::fs::read_to_string(&yaml_path)?;
            serde_yml::from_str::<Config>(&text)?
        } else {
            Config::default()
        };

        // 兜底关键字段默认值（serde 的 default 已处理结构体，这里补语义默认）
        if config.site.language.is_empty() {
            config.site.language = "zh-CN".into();
        }
        if config.build.output_dir.is_empty() {
            config.build.output_dir = "public".into();
        }
        if config.build.math.renderer.is_empty() {
            config.build.math.renderer = "mathml".into();
        }
        if !matches!(config.build.math.renderer.as_str(), "mathml" | "svg") {
            anyhow::bail!(
                "不支持的公式渲染器: {}（可选 mathml | svg）",
                config.build.math.renderer
            );
        }
        if config.theme.name.is_empty() {
            config.theme.name = "default".into();
        }
        if config.build.posts_per_page == 0 {
            config.build.posts_per_page = 20;
        }
        // 定时部署计划尽早校验：格式错误在启动时报人话错误，
        // 而不是等 serve 挂上后台线程后第一次触发才发现。
        crate::deploy::validate_schedule(&config.deploy.schedule)?;

        // 环境变量覆盖（TP_ 前缀，便于 CI/CD 注入）
        apply_env_overrides(&mut config);

        Ok(config)
    }

    /// 输出目录（绝对路径）
    pub fn output_dir(&self, root: &Path) -> std::path::PathBuf {
        root.join(&self.build.output_dir)
    }
}

/// 用 `TP_` 前缀环境变量覆盖配置（规格书 §2.2）。
fn apply_env_overrides(config: &mut Config) {
    if let Ok(v) = std::env::var("TP_SITE_TITLE") {
        config.site.title = v;
    }
    if let Ok(v) = std::env::var("TP_SITE_DESCRIPTION") {
        config.site.description = v;
    }
    if let Ok(v) = std::env::var("TP_SITE_URL") {
        config.site.url = v;
    }
    if let Ok(v) = std::env::var("TP_SITE_AUTHOR") {
        config.site.author = v;
    }
    if let Ok(v) = std::env::var("TP_SITE_LANGUAGE") {
        config.site.language = v;
    }
    if let Ok(v) = std::env::var("TP_SITE_ANALYTICS") {
        config.site.analytics = v;
    }
    if let Ok(v) = std::env::var("TP_BUILD_OUTPUT_DIR") {
        config.build.output_dir = v;
    }
    if let Ok(v) = std::env::var("TP_BUILD_STRICT") {
        config.build.strict = parse_bool(&v);
    }
    if let Ok(v) = std::env::var("TP_BUILD_DRAFTS") {
        config.build.drafts = parse_bool(&v);
    }
    if let Ok(v) = std::env::var("TP_DEPLOY_STRATEGY") {
        config.deploy.strategy = Some(v);
    }
}

fn parse_bool(s: &str) -> bool {
    s == "1" || s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("yes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ENV_LOCK;

    #[test]
    fn load_applies_semantic_defaults_without_config_file() {
        let _guard = ENV_LOCK.lock().unwrap();
        let root = std::env::temp_dir().join("typall-test-nonexistent");
        let _ = std::fs::remove_dir_all(&root);
        let config = Config::load(&root).unwrap();
        assert_eq!(config.site.language, "zh-CN");
        assert_eq!(config.build.output_dir, "public");
        assert_eq!(config.build.math.renderer, "mathml");
        assert_eq!(config.theme.name, "default");
        assert_eq!(config.build.posts_per_page, 20);
    }

    #[test]
    fn load_rejects_unknown_renderer() {
        let _guard = ENV_LOCK.lock().unwrap();
        let root = std::env::temp_dir().join("typall-test-bad-renderer");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("typall.toml"),
            "[site]\ntitle = \"t\"\n\n[build.math]\nrenderer = \"katex\"\n",
        )
        .unwrap();
        let err = Config::load(&root).unwrap_err();
        assert!(format!("{err:#}").contains("渲染器"), "报错应提及渲染器: {err:#}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn env_overrides_apply_to_config() {
        let _guard = ENV_LOCK.lock().unwrap();
        // SAFETY: 本测试是唯一操作 TP_ 环境变量的测试，且用后即清理；
        // Rust 2024 中 set_var/remove_var 因进程级全局状态而标记为 unsafe。
        unsafe {
            std::env::set_var("TP_SITE_TITLE", "覆盖标题");
            std::env::set_var("TP_BUILD_STRICT", "1");
            std::env::set_var("TP_SITE_LANGUAGE", "en-US");
        }

        let mut config = Config::default();
        apply_env_overrides(&mut config);
        assert_eq!(config.site.title, "覆盖标题");
        assert!(config.build.strict);
        assert_eq!(config.site.language, "en-US");

        unsafe {
            std::env::remove_var("TP_SITE_TITLE");
            std::env::remove_var("TP_BUILD_STRICT");
            std::env::remove_var("TP_SITE_LANGUAGE");
        }
    }

    #[test]
    fn publish_unknown_keys_are_ignored() {
        let _guard = ENV_LOCK.lock().unwrap();
        let root = std::env::temp_dir().join("typall-test-publish");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        // 旧版配置里的 enabled/targets 已随远端推送移除；老项目文件必须继续可解析。
        std::fs::write(
            root.join("typall.toml"),
            "[site]\ntitle = \"t\"\n\n[publish]\nenabled = true\ntargets = [\"wechat\", \"zhihu\"]\n",
        )
        .unwrap();
        let config = Config::load(&root).unwrap();
        assert_eq!(config.publish.out_dir, "publish");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn load_rejects_bad_deploy_schedule() {
        let _guard = ENV_LOCK.lock().unwrap();
        let root = std::env::temp_dir().join("typall-test-bad-schedule");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("typall.toml"),
            "[deploy.schedule]\nevery = \"5x\"\n",
        )
        .unwrap();
        let err = Config::load(&root).unwrap_err();
        assert!(format!("{err:#}").contains("every"), "报错应提及 every: {err:#}");
        // 合法计划通过校验
        std::fs::write(
            root.join("typall.toml"),
            "[deploy.schedule]\nevery = \"30m\"\ndaily = [\"21:00\"]\n",
        )
        .unwrap();
        assert!(Config::load(&root).is_ok());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn parse_bool_recognizes_truthy_values() {
        assert!(parse_bool("1"));
        assert!(parse_bool("true"));
        assert!(parse_bool("True"));
        assert!(parse_bool("TRUE"));
        assert!(parse_bool("yes"));
        assert!(parse_bool("YES"));
        assert!(!parse_bool("0"));
        assert!(!parse_bool("false"));
        assert!(!parse_bool("no"));
        assert!(!parse_bool(""));
    }
}
