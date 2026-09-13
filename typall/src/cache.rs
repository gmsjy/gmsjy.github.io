//! 编译缓存：源码 + 上下文 + 依赖 三级哈希，命中则跳过 Typst 编译。


use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
struct CacheEntry {
    /// 源文件内容哈希。
    source_hash: String,
    /// 编译上下文（preamble：语言/公式编号/宏导入语句）哈希。
    context_hash: String,
    /// import 依赖文件及其内容哈希。
    deps: Vec<DepHash>,
    body_html: String,
    math_style: String,
}

/// 单个依赖文件的路径（相对项目根）与内容哈希。
#[derive(Serialize, Deserialize)]
struct DepHash {
    path: String,
    hash: String,
}

/// 编译缓存：按「源文件哈希 + 编译上下文哈希 + 依赖哈希」三级判断是否复用编译产物。
///
/// 命中条件（全部满足才复用，否则重新编译）：
/// 1. 源文件内容哈希未变；
/// 2. 编译上下文哈希未变（preamble + 影响产物的 HTML 后处理参数）；
/// 3. 所有 import 依赖文件的内容哈希未变（依赖经 [`TypallWorld`] 在编译时记录）。
pub(crate) struct CompileCache {
    root: PathBuf,
    context_hash: String,
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl CompileCache {
    /// `context` 必须涵盖一切影响 `body_html` 的生成器侧输入：preamble
    /// （语言/编号抑制/宏导入）以及 `equation_prefix`——后者在 HTML 后处理层
    /// 注入（[`inject_equation_numbers`]），不在 preamble 里，若不参与哈希，
    /// 改前缀会全部命中旧缓存，编号前缀永不更新。
    pub(crate) fn new(root: &Path, context: &str) -> Self {
        Self {
            root: root.to_path_buf(),
            context_hash: content_hash(context),
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    /// 缓存文件路径：`posts/foo.typ` → `.typall/cache/posts/foo.json`。
    fn entry_path(&self, rel: &str) -> PathBuf {
        let rel = rel.trim_end_matches(".typ");
        self.root.join(".typall").join("cache").join(format!("{rel}.json"))
    }

    pub(crate) fn load(&self, rel: &str, source_hash: &str) -> Option<(String, String)> {
        let json = std::fs::read_to_string(self.entry_path(rel)).ok()?;
        let entry: CacheEntry = serde_json::from_str(&json).ok()?;
        if entry.source_hash != source_hash || entry.context_hash != self.context_hash {
            return None;
        }
        for dep in &entry.deps {
            let dep_path = self.root.join(&dep.path);
            // 字节级比对：依赖可能是 .wasm/图片等二进制资源，不能按文本读。
            let Ok(content) = std::fs::read(&dep_path) else {
                return None;
            };
            if hash_bytes(&content) != dep.hash {
                return None;
            }
        }
        self.hits.fetch_add(1, Ordering::Relaxed);
        Some((entry.body_html, entry.math_style))
    }

    pub(crate) fn store(
        &self,
        rel: &str,
        source_hash: &str,
        deps: &[PathBuf],
        body_html: &str,
        math_style: &str,
    ) -> anyhow::Result<()> {
        self.misses.fetch_add(1, Ordering::Relaxed);
        let deps: Vec<DepHash> = deps
            .iter()
            .filter_map(|p| {
                let dep_rel = p.strip_prefix(&self.root).ok()?;
                // 二进制依赖（.wasm 等）也要落盘哈希，供下次字节级比对。
                match std::fs::read(p) {
    Ok(content) => Some(DepHash {
        path: dep_rel.to_string_lossy().replace('\\', "/"),
        hash: hash_bytes(&content),
    }),
    // 竞态兜底：依赖恰好不可读时跳过该条并留痕——缺失哈希意味着
    // 该依赖未来的变更不会触发缓存失效，必须留痕排查。
    Err(e) => {
        eprintln!("⚠ 依赖不可读，未纳入缓存哈希：{} ({e})", p.display());
        None
    }
}
            })
            .collect();
        let entry = CacheEntry {
            source_hash: source_hash.to_string(),
            context_hash: self.context_hash.clone(),
            deps,
            body_html: body_html.to_string(),
            math_style: math_style.to_string(),
        };
        let path = self.entry_path(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, serde_json::to_string(&entry)?)?;
        Ok(())
    }

    /// 清理孤儿缓存（源文件已删除，其缓存残留无意义）。
    pub(crate) fn clean_orphans(&self, valid: &HashSet<String>) -> anyhow::Result<()> {
        let cache_dir = self.root.join(".typall").join("cache");
        if !cache_dir.exists() {
            return Ok(());
        }
        for entry in walkdir::WalkDir::new(&cache_dir) {
            let Ok(entry) = entry else { continue };
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let rel = path
                .strip_prefix(&cache_dir)?
                .to_string_lossy()
                .replace('\\', "/");
            // `posts/foo.json` → 源文件 `posts/foo.typ`
            let src_rel = format!("{}.typ", rel.trim_end_matches(".json"));
            if !valid.contains(&src_rel) {
                std::fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    pub(crate) fn hits(&self) -> usize {
        self.hits.load(Ordering::Relaxed)
    }

    pub(crate) fn misses(&self) -> usize {
        self.misses.load(Ordering::Relaxed)
    }
}

/// 稳定内容哈希（FNV-1a 64 位），用于缓存失效判断。
///
/// 不用 `DefaultHasher`（SipHash13）：Rust 不保证跨版本稳定，且带与缓存
/// 无关的 DoS 防护开销。输出固定 16 位小写 hex。
pub(crate) fn content_hash(text: &str) -> String {
    hash_bytes(text.as_bytes())
}

/// FNV-1a 64 位字节哈希（文本与二进制依赖共用同一实现）。
pub(crate) fn hash_bytes(data: &[u8]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in data {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

