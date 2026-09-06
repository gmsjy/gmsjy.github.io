//! 端到端黄金基线测试：跑真实 CLI 二进制构建微型夹具项目，
//! 对产物清单与内容哈希做黄金比对——守住流水线级回归
//! （缓存失效、过滤、feed、主题回退、确定性输出）。
//!
//! 重新生成黄金基线：`GOLDEN_UPDATE=1 cargo test --test e2e`

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_typall");
const FIXTURE: &str = "tests/fixtures/mini";
const GOLDEN: &str = "tests/golden/mini-manifest.txt";

/// FNV-1a：无依赖、跨平台稳定的字节校验和。
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// 递归收集目录下（rel 路径, 内容校验和）清单，按路径排序。
fn manifest_of(dir: &Path) -> Vec<(String, u64)> {
    fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, u64)>) {
        for entry in fs::read_dir(dir).expect("read_dir").flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            let rel = if prefix.is_empty() { name.clone() } else { format!("{prefix}/{name}") };
            if path.is_dir() {
                walk(&path, &rel, out);
            } else {
                let bytes = fs::read(&path).expect("read file");
                out.push((rel, fnv1a(&bytes)));
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, "", &mut out);
    out.sort();
    out
}

fn manifest_lines(m: &[(String, u64)]) -> String {
    let mut s = String::new();
    for (rel, hash) in m {
        s.push_str(&format!("{hash:016x} {rel}\n"));
    }
    s
}

/// 把夹具复制到一个全新的临时项目根（不带任何缓存/产物）。
fn setup_root(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("typall-e2e-{tag}"));
    let _ = fs::remove_dir_all(&root);
    copy_dir(Path::new(FIXTURE), &root);
    root
}

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create dir");
    for entry in fs::read_dir(src).expect("read_dir").flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy file");
        }
    }
}

/// 运行 CLI 子命令，返回 (成功?, stdout, stderr)。
fn run_cli(root: &Path, args: &[&str], envs: &[(&str, &str)]) -> (bool, String, String) {
    let out = Command::new(BIN)
        .args(args)
        .current_dir(root)
        .envs(envs.iter().copied())
        .output()
        .expect("spawn typall binary");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn build_golden_and_idempotent() {
    let root = setup_root("golden");

    // 第一次构建：全量编译
    let (ok, stdout, stderr) = run_cli(&root, &["build"], &[]);
    assert!(ok, "首次构建应成功\nstdout: {stdout}\nstderr: {stderr}");
    assert!(stdout.contains("2 重新编译") || stderr.is_empty(), "sanity");

    let out_dir = root.join("public");
    let m1 = manifest_of(&out_dir);

    // 草稿与未来日期文章不出现在产物中
    assert!(!out_dir.join("posts/03-draft").exists(), "草稿不应生成");
    assert!(!out_dir.join("posts/04-future").exists(), "未来文章不应生成");
    // 两篇可见文章 + 独立页面 + 集合页等均存在
    assert!(out_dir.join("posts/01-math/index.html").exists());
    assert!(out_dir.join("posts/02-physics/index.html").exists());
    assert!(out_dir.join("pages/about/index.html").exists());
    assert!(out_dir.join("atom.xml").exists());
    assert!(out_dir.join("sitemap.xml").exists());
    assert!(out_dir.join("search.json").exists());

    // 黄金基线比对（或更新）
    let golden = Path::new(GOLDEN);
    let actual = manifest_lines(&m1);
    if std::env::var("GOLDEN_UPDATE").is_ok() {
        fs::create_dir_all(golden.parent().unwrap()).expect("mkdir golden");
        fs::write(golden, &actual).expect("write golden");
    } else {
        let expected = fs::read_to_string(golden).expect("读取黄金基线");
        assert_eq!(
            actual, expected,
            "产物清单与黄金基线不一致！若为预期变更，用 GOLDEN_UPDATE=1 重新生成并人工过目 diff"
        );
    }

    // 第二次构建：全部命中缓存，产物逐字节一致
    let (ok2, stdout2, _) = run_cli(&root, &["build"], &[]);
    assert!(ok2, "二次构建应成功");
    assert!(stdout2.contains("0 重新编译"), "二次构建应全量命中缓存：{stdout2}");
    let m2 = manifest_of(&out_dir);
    assert_eq!(m1, m2, "二次构建产物必须逐字节一致");
}

#[test]
fn drafts_env_override_and_guard() {
    let root = setup_root("drafts");

    // TP_BUILD_DRAFTS=1 时草稿出现
    let (ok, _, stderr) = run_cli(&root, &["build"], &[("TP_BUILD_DRAFTS", "true")]);
    assert!(ok, "drafts 构建应成功: {stderr}");
    assert!(root.join("public/posts/03-draft/index.html").exists(), "TP_BUILD_DRAFTS=1 应包含草稿");

    // 错误目录启动应被人话错误拒绝（而非静默空站点）
    let empty = std::env::temp_dir().join("typall-e2e-empty");
    let _ = fs::remove_dir_all(&empty);
    fs::create_dir_all(&empty).expect("mkdir empty");
    let (ok, _, stderr) = run_cli(&empty, &["build"], &[]);
    assert!(!ok, "空目录构建应失败");
    assert!(stderr.contains("typall.toml"), "错误信息应提示 typall.toml：{stderr}");
}
