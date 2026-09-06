//! Typst `World` 实现：把项目根目录映射为 Typst 的虚拟文件系统。
//!
//! 设计要点：
//! - 字体与标准库是昂贵资源，通过 [`SharedAssets`] 跨文章共享，只加载一次。
//! - 每篇文章实例化一个 [`TypallWorld`]，主文件指向该文章。

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use chrono::Datelike;
use typst::diag::{FileError, FileResult, SourceDiagnostic};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World, WorldExt};

use crate::typst_compat::html_features;

use crate::packages::PackageResolver;

/// 跨文章共享的昂贵资源。
pub struct SharedAssets {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    /// Typst 包解析器（下载 + 缓存），跨文章共享以复用已下载的包。
    packages: Arc<PackageResolver>,
}

impl SharedAssets {
    pub fn load(root: &std::path::Path) -> anyhow::Result<Self> {
        Self::load_with_font_dirs(root, &[])
    }

    /// 带额外字体目录加载（`typall pdf` 传入系统字体目录以兜底 CJK 等
    /// typst-assets 未内置的字体；站点构建保持确定性字体集，不扫系统）。
    pub fn load_with_font_dirs(root: &std::path::Path, extra_dirs: &[PathBuf]) -> anyhow::Result<Self> {
        // HTML 导出是运行时 Feature（无需 cargo feature）。
        let features = html_features();
        let library = Library::builder().with_features(features).build();

        let mut book = FontBook::new();
        let mut fonts = Vec::new();
        for data in typst_assets::fonts() {
            let bytes = Bytes::new(data.to_vec());
            for font in Font::iter(bytes) {
                book.push(font.info().clone());
                fonts.push(font);
            }
        }
        if fonts.is_empty() {
            anyhow::bail!("未加载到任何字体（typst-assets 的 fonts feature 未启用？）");
        }

        // 项目字体：assets/fonts/（分页 PDF、html.frame SVG 等所有 paged
        // 渲染路径的字体来源——内置字体集只有西文，中文帖子/插图靠这里补）。
        let project_fonts = root.join("assets").join("fonts");
        load_fonts_from_dir(&project_fonts, &mut book, &mut fonts);
        for dir in extra_dirs {
            load_fonts_from_dir(dir, &mut book, &mut fonts);
        }

        Ok(Self {
            library: LazyHash::new(library),
            book: LazyHash::new(book),
            fonts,
            packages: Arc::new(PackageResolver::new(root.join(".typall").join("packages"))),
        })
    }
}

/// 把目录（递归）里的字体文件注册进字体簿。非法/不可读文件静默跳过：
/// 字体目录是用户资产，单个坏文件不应让整个构建失败。
fn load_fonts_from_dir(dir: &std::path::Path, book: &mut FontBook, fonts: &mut Vec<Font>) {
    if !dir.is_dir() {
        return;
    }
    for entry in walkdir::WalkDir::new(dir).follow_links(true) {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_file() {
            continue;
        }
        let ext = entry
            .path()
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase());
        // typst 支持 ttf/otf/ttc（woff2 供网页场景，引擎不支持）
        if !matches!(ext.as_deref(), Some("ttf") | Some("otf") | Some("ttc")) {
            continue;
        }
        let Ok(data) = std::fs::read(entry.path()) else { continue };
        for font in Font::iter(Bytes::new(data)) {
            book.push(font.info().clone());
            fonts.push(font);
        }
    }
}

/// 各平台的标准系统字体目录（`typall pdf` 兜底用，存在才返回）。
///
/// 站点构建**不调用**：系统字体因机器而异，混入 HTML/SVG 渲染会破坏
/// 构建产物确定性；PDF 本身内嵌字体、天然机器相关，则默认加载。
pub(crate) fn system_font_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    #[cfg(target_os = "windows")]
    {
        if let Some(windir) = std::env::var_os("WINDIR") {
            dirs.push(PathBuf::from(windir).join("Fonts"));
        }
        // 用户级安装字体（Windows 10 1809+ 右键"为当前用户安装"）
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            dirs.push(PathBuf::from(local).join("Microsoft").join("Windows").join("Fonts"));
        }
    }
    #[cfg(target_os = "macos")]
    {
        dirs.push(PathBuf::from("/System/Library/Fonts"));
        dirs.push(PathBuf::from("/System/Library/Fonts/Supplemental"));
        dirs.push(PathBuf::from("/Library/Fonts"));
        if let Some(home) = std::env::var_os("HOME") {
            dirs.push(PathBuf::from(home).join("Library").join("Fonts"));
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        dirs.push(PathBuf::from("/usr/share/fonts"));
        dirs.push(PathBuf::from("/usr/local/share/fonts"));
        if let Some(home) = std::env::var_os("HOME") {
            dirs.push(PathBuf::from(home).join(".fonts"));
            dirs.push(PathBuf::from(home).join(".local/share/fonts"));
        }
    }
    dirs.retain(|p| p.is_dir());
    dirs
}

pub struct TypallWorld {
    root: PathBuf,
    main: FileId,
    shared: Arc<SharedAssets>,
    /// 注入到主文件开头的代码（如公式编号设置）。
    preamble: String,
    sources: RwLock<HashMap<FileId, Source>>,
    /// 编译过程中 import 的依赖文件（磁盘绝对路径），供增量编译追踪失效。
    deps: RwLock<HashSet<PathBuf>>,
}

impl TypallWorld {
    /// `main_rel` 为相对项目根的路径，如 `posts/quantum.typ`。
    pub fn new(
        root: impl Into<PathBuf>,
        main_rel: &str,
        shared: Arc<SharedAssets>,
        preamble: &str,
    ) -> anyhow::Result<Self> {
        let root: PathBuf = root.into();
        let vpath = VirtualPath::new(format!("/{main_rel}"))
            .map_err(|e| anyhow::anyhow!("非法虚拟路径 /{main_rel}: {e}"))?;
        let main = FileId::new(RootedPath::new(VirtualRoot::Project, vpath));
        Ok(Self {
            root,
            main,
            shared,
            preamble: preamble.to_string(),
            sources: RwLock::new(HashMap::new()),
            deps: RwLock::new(HashSet::new()),
        })
    }

    /// 虚拟路径 → 真实磁盘路径（官方函数，自带路径逃逸防护）。
    ///
    /// `VirtualRoot::Project` 映射到项目根；`VirtualRoot::Package(spec)` 映射到
    /// 包缓存目录（缺失时经 [`PackageResolver`] 下载）。
    fn disk_path(&self, id: FileId) -> FileResult<PathBuf> {
        match id.root() {
            VirtualRoot::Project => id.vpath().realize(&self.root).map_err(FileError::Realize),
            VirtualRoot::Package(spec) => {
                let dir = self.shared.packages.ensure(spec)?;
                id.vpath().realize(&dir).map_err(FileError::Realize)
            }
        }
    }

    /// 把诊断的 span 解析为 `(路径, 行, 列)`，行列为 1-based。
    ///
    /// 主文件注入了 preamble（语言/公式编号等），此处扣除其行数偏移，使
    /// 行号对应原始文章；依赖文件（如 import 的 `macros.typ`）无 preamble。
    pub fn location(&self, diag: &SourceDiagnostic) -> Option<(String, usize, usize)> {
        let id = diag.span.id()?;
        // WorldExt::range 正确处理 Number variant（节点编号 → 字节范围）。
        let byte = self.range(diag.span)?.start;
        let src = self.source(id).ok()?;
        let (line, col) = src.lines().byte_to_line_column(byte)?;
        // 包内文件在诊断中带 `@namespace/name:version/` 前缀，避免歧义
        // （如 cetz 多个包文件都叫 `lib.typ`）。
        let path = match id.root() {
            VirtualRoot::Project => id.vpath().get_without_slash().to_string(),
            VirtualRoot::Package(spec) => {
                format!("{spec}/{}", id.vpath().get_without_slash())
            }
        };
        let line = if id == self.main && !self.preamble.is_empty() {
            // preamble 以 `\n` 结尾，注入时再补一个 `\n`，故偏移 = 换行数 + 1。
            line.saturating_sub(self.preamble.matches('\n').count() + 1)
        } else {
            line
        };
        Some((path, line + 1, col + 1))
    }

    /// 本次编译 import 的依赖文件（磁盘绝对路径），去重排序后返回。
    pub fn deps(&self) -> Vec<PathBuf> {
        let mut v: Vec<PathBuf> = self.deps.read().unwrap().iter().cloned().collect();
        v.sort();
        v
    }
}

impl World for TypallWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.shared.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.shared.book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if let Some(src) = self.sources.read().unwrap().get(&id) {
            return Ok(src.clone());
        }
        let path = self.disk_path(id)?;
        // 主文件之外的 source 请求即 import 依赖，记录其磁盘路径供增量失效判断。
        if id != self.main {
            self.deps.write().unwrap().insert(path.clone());
        }
        let text = std::fs::read_to_string(&path).map_err(|e| FileError::from_io(e, &path))?;
        // 主文件注入 preamble（如公式编号设置）
        let full = if id == self.main && !self.preamble.is_empty() {
            format!("{}\n{}", self.preamble, text)
        } else {
            text
        };
        let source = Source::new(id, full);
        self.sources.write().unwrap().insert(id, source.clone());
        Ok(source)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = self.disk_path(id)?;
        // 二进制资源（.wasm 插件、图片等）同样属于依赖，记录磁盘路径供增量失效
        // 判断——否则 cetz 内核或图片更新后，编译缓存会命中旧产物却无任何提示。
        self.deps.write().unwrap().insert(path.clone());
        let data = std::fs::read(&path).map_err(|e| FileError::from_io(e, &path))?;
        Ok(Bytes::new(data))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.shared.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        // 用系统真实日期，避免 Typst 的 datetime.today() 拿到固定值。
        let now = chrono::Local::now().naive_local().date();
        Datetime::from_ymd(now.year(), now.month() as u8, now.day() as u8)
    }
}
