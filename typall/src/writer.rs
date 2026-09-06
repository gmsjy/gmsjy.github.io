//! 站点写盘：内容感知增量 + tmp+rename 原子写。


use std::collections::HashSet;
use std::path::{Path, PathBuf};




/// serve 的热重建天然「边服务边替换」，这类冲突是常态而非异常。
pub(crate) fn rename_with_retry(tmp: &Path, full: &Path) -> std::io::Result<()> {
    let mut attempts = 0;
    loop {
        match std::fs::rename(tmp, full) {
            Ok(()) => return Ok(()),
            Err(e) if attempts < 4 && matches!(e.raw_os_error(), Some(5) | Some(32)) => {
                attempts += 1;
                std::thread::sleep(std::time::Duration::from_millis(50 * attempts));
            }
            Err(e) => return Err(e),
        }
    }
}

/// 增量写入器：覆盖写文件并记录相对路径，供后续清理孤儿文件。
pub(crate) struct SiteWriter {
    pub(crate) out: PathBuf,
    pub(crate) written: HashSet<String>,
}

impl SiteWriter {
    pub(crate) fn new(out: PathBuf) -> Self {
        Self {
            out,
            written: HashSet::new(),
        }
    }

    pub(crate) fn write(&mut self, rel: &str, content: &[u8]) -> anyhow::Result<()> {
        let full = self.out.join(rel);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // 内容感知增量：磁盘上已是相同内容则不重写。
        // 保持源文件 mtime 不变，使 gzip 等下游增量判断（`.gz` 不旧于源）得以生效，
        // 同时减少无意义写盘与 CDN/nginx 端的缓存失效。
        if let Ok(old) = std::fs::read(&full)
            && old == content
        {
            self.written.insert(rel.to_string());
            return Ok(());
        }
        // 原子写：先写同目录临时文件再 rename，`serve` 重建期间 HTTP 线程
        // 并发读 public/ 时不会读到写了一半的截断文件（与 publish 的
        // atomic_write 同思路）。
        let tmp = full.with_extension("typall-tmp");
        let written = std::fs::write(&tmp, content).and_then(|()| rename_with_retry(&tmp, &full));
        if let Err(e) = written {
            let _ = std::fs::remove_file(&tmp);
            return Err(e.into());
        }
        self.written.insert(rel.to_string());
        Ok(())
    }

    pub(crate) fn write_str(&mut self, rel: &str, content: &str) -> anyhow::Result<()> {
        self.write(rel, content.as_bytes())
    }

    /// 仅登记路径（不写文件），用于"产物已是最新、无需重写"的增量场景。
    pub(crate) fn mark(&mut self, rel: &str) {
        self.written.insert(rel.to_string());
    }
}

