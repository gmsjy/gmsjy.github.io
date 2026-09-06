//! Typst 包解析与下载缓存。
//!
//! Typst 0.15 通过 `@namespace/name:version` 语法引用包，编译时由 [`World`] 的
//! `source`/`file` 回调按 `VirtualRoot::Package(spec)` 定位到磁盘上的包文件。
//! 本模块负责把 `PackageSpec` 映射到本地缓存目录，缺失时从官方源
//! `packages.typst.org` 下载并解压。
//!
//! 设计要点：
//! - 缓存目录：`<project>/.typall/packages/`，与编译缓存同级，随 `.typall` 一并
//!   被 `.gitignore` 忽略（可复现、自包含，符合 Typall 单二进制理念）。
//! - 下载串行化：用进程内互斥锁避免并行编译时同一包被并发下载。
//! - 解压原子性：先解压到临时目录再重命名，避免半成品被误判为完整缓存。
//!
//! [`World`]: typst::World

use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use typst::diag::{FileError, FileResult, PackageError};
use typst::syntax::package::PackageSpec;

/// 包下载与缓存解析器。
pub struct PackageResolver {
    /// 包缓存根目录，如 `<project>/.typall/packages/`。
    cache_dir: PathBuf,
    /// 串行化下载，避免并发编译时重复下载同一包。
    lock: Mutex<()>,
}

impl PackageResolver {
    /// 创建解析器，包缓存根目录为 `cache_dir`。
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir, lock: Mutex::new(()) }
    }

    /// 返回指定包在磁盘上的目录路径，缺失时自动下载解压。
    ///
    /// 错误以 [`FileError::Package`] 返回，便于在诊断信息里就地展示。
    pub fn ensure(&self, spec: &PackageSpec) -> FileResult<PathBuf> {
        let dir = self.package_dir(spec);
        if dir.join("typst.toml").is_file() {
            return Ok(dir);
        }

        // 串行化下载；等锁期间可能已被其他线程下载完成，故二次检查。
        let _guard = self.lock.lock().unwrap();
        if dir.join("typst.toml").is_file() {
            return Ok(dir);
        }

        match spec.namespace.as_str() {
            "preview" => self.download(spec, &dir)?,
            other => {
                return Err(FileError::Package(PackageError::Other(Some(
                    format!("不支持的包命名空间 `@{other}`（Typall 目前仅支持 @preview）").into(),
                ))));
            }
        }

        Ok(dir)
    }

    /// 包在缓存中的目录：`<cache>/<namespace>/<name>/<version>`。
    fn package_dir(&self, spec: &PackageSpec) -> PathBuf {
        self.cache_dir
            .join(spec.namespace.as_str())
            .join(spec.name.as_str())
            .join(spec.version.to_string())
    }

    /// 下载并解压包到 `dir`（原子：先解压到临时目录再重命名）。
    fn download(&self, spec: &PackageSpec, dir: &Path) -> FileResult<()> {
        let url = format!(
            "https://packages.typst.org/{}/{}-{}.tar.gz",
            spec.namespace, spec.name, spec.version
        );

        // 1. 下载到内存（包体积小，219KB 级别的 cetz 完全无压力）。
        let mut bytes = Vec::new();
        let resp = ureq::get(&url)
            .call()
            .map_err(|e| {
                FileError::Package(PackageError::NetworkFailed(Some(e.to_string().into())))
            })?;
        match resp.status() {
            404 => return Err(FileError::Package(PackageError::NotFound(spec.clone()))),
            s if s >= 400 => {
                return Err(FileError::Package(PackageError::NetworkFailed(Some(
                    format!("HTTP {s}").into(),
                ))));
            }
            _ => {}
        }
        resp.into_reader().read_to_end(&mut bytes).map_err(|e| {
            FileError::Package(PackageError::NetworkFailed(Some(e.to_string().into())))
        })?;

        // 2. 解压到临时目录，成功后原子重命名到目标目录。
        let tmp = dir.with_file_name(format!(
            "{}.tmp-{}",
            dir.file_name().map(|n| n.to_string_lossy()).unwrap_or_default(),
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).map_err(|e| {
            FileError::Package(PackageError::Other(Some(format!(
                "创建缓存目录失败: {e}"
            )
            .into())))
        })?;

        let decoder = flate2::read::GzDecoder::new(bytes.as_slice());
        let mut archive = tar::Archive::new(decoder);
        archive.unpack(&tmp).map_err(|e| {
            FileError::Package(PackageError::MalformedArchive(Some(e.to_string().into())))
        })?;

        if !tmp.join("typst.toml").is_file() {
            let _ = std::fs::remove_dir_all(&tmp);
            return Err(FileError::Package(PackageError::MalformedArchive(Some(
                "缺少 typst.toml".into(),
            ))));
        }

        // 原子提交：清理残留旧目录后重命名。
        if let Some(parent) = dir.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                FileError::Package(PackageError::Other(Some(format!(
                    "创建缓存目录失败: {e}"
                )
                .into())))
            })?;
        }
        let _ = std::fs::remove_dir_all(dir);
        std::fs::rename(&tmp, dir).map_err(|e| {
            FileError::Package(PackageError::Other(Some(format!("写入缓存失败: {e}").into())))
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_dir_uses_namespace_name_version() {
        let resolver = PackageResolver::new(PathBuf::from("/tmp/cache"));
        let spec: PackageSpec = "@preview/cetz:0.5.2".parse().unwrap();
        let dir = resolver.package_dir(&spec);
        assert_eq!(
            dir,
            PathBuf::from("/tmp/cache").join("preview").join("cetz").join("0.5.2")
        );
    }

    #[test]
    fn ensure_rejects_unknown_namespace() {
        let resolver = PackageResolver::new(PathBuf::from("/tmp/cache"));
        let spec: PackageSpec = "@local/foo:1.0.0".parse().unwrap();
        let err = resolver.ensure(&spec).unwrap_err();
        assert!(matches!(err, FileError::Package(PackageError::Other(_))));
    }
}
