//! Jail repo-root para candidatos de purga.

use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone)]
pub struct JailError {
    pub code: &'static str,
    pub detail: String,
}

impl JailError {
    pub fn code_and_msg(&self) -> (&'static str, String) {
        (self.code, self.detail.clone())
    }
}

pub fn canonical_repo_root(start: &Path) -> Result<PathBuf, JailError> {
    let mut dir = if start.is_file() {
        start
            .parent()
            .ok_or_else(|| JailError {
                code: "JAIL_INVALID_REPO",
                detail: "sin directorio padre".into(),
            })?
            .to_path_buf()
    } else {
        start.to_path_buf()
    };
    loop {
        if dir.join("SddIA/Cargo.toml").is_file() || dir.join(".git").exists() {
            return dir.canonicalize().map_err(|e| JailError {
                code: "JAIL_INVALID_REPO",
                detail: e.to_string(),
            });
        }
        if !dir.pop() {
            return Err(JailError {
                code: "JAIL_INVALID_REPO",
                detail: "no se encontró raíz SddIA".into(),
            });
        }
    }
}

fn rel_slash(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub fn assert_candidate_jail(repo_root: &Path, candidate: &Path) -> Result<(), JailError> {
    let repo = repo_root.canonicalize().map_err(|e| JailError {
        code: "JAIL_INVALID_REPO",
        detail: e.to_string(),
    })?;
    if candidate
        .components()
        .any(|c| matches!(c, Component::ParentDir))
    {
        return Err(JailError {
            code: "JAIL_PATH_TRAVERSAL",
            detail: format!("componente .. en {}", candidate.display()),
        });
    }
    let abs = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        repo.join(candidate)
    };
    let canon = abs.canonicalize().unwrap_or(abs);
    if !canon.starts_with(&repo) {
        return Err(JailError {
            code: "JAIL_OUT_OF_REPO",
            detail: format!("fuera del repo: {}", rel_slash(&canon)),
        });
    }
    let rel = rel_slash(
        &canon
            .strip_prefix(&repo)
            .map(Path::to_path_buf)
            .unwrap_or(canon),
    );
    if rel.contains(".git/") || rel == ".git" {
        return Err(JailError {
            code: "JAIL_FORBIDDEN_PREFIX",
            detail: ".git".into(),
        });
    }
    if rel.contains(".SddIA/runtime/events.redb") || rel.ends_with("events.redb") {
        return Err(JailError {
            code: "JAIL_FORBIDDEN_STORE",
            detail: "events.redb no es purgable".into(),
        });
    }
    if rel.contains("SddIA/target/native-debug-bins") {
        return Err(JailError {
            code: "JAIL_FORBIDDEN_NATIVE",
            detail: "native-debug-bins fuera de slice A".into(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture_repo() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "sddia-retention-jail-{}",
            uuid_simple()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("SddIA")).expect("mkdir");
        fs::write(root.join("SddIA/Cargo.toml"), "[workspace]\n").expect("cargo");
        fs::create_dir_all(root.join(".SddIA/runtime")).expect("runtime");
        fs::write(root.join(".SddIA/runtime/events.redb"), b"x").expect("redb");
        fs::create_dir_all(root.join("SddIA/target/native-debug-bins")).expect("native");
        fs::write(
            root.join("SddIA/target/native-debug-bins/execute-process"),
            b"elf",
        )
        .expect("elf");
        root
    }

    fn uuid_simple() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    #[test]
    fn rejects_events_redb() {
        let repo = fixture_repo();
        let root = canonical_repo_root(&repo).expect("root");
        let err = assert_candidate_jail(&root, Path::new(".SddIA/runtime/events.redb"))
            .expect_err("redb");
        assert_eq!(err.code, "JAIL_FORBIDDEN_STORE");
    }

    #[test]
    fn rejects_parent_dir() {
        let repo = fixture_repo();
        let root = canonical_repo_root(&repo).expect("root");
        let err = assert_candidate_jail(&root, Path::new("../etc/passwd")).expect_err("..");
        assert_eq!(err.code, "JAIL_PATH_TRAVERSAL");
    }

    #[test]
    fn rejects_native_debug_bins() {
        let repo = fixture_repo();
        let root = canonical_repo_root(&repo).expect("root");
        let p = Path::new("SddIA/target/native-debug-bins/execute-process");
        let err = assert_candidate_jail(&root, p).expect_err("native");
        assert_eq!(err.code, "JAIL_FORBIDDEN_NATIVE");
    }

    #[test]
    fn allows_tmp_under_repo() {
        let repo = fixture_repo();
        fs::create_dir_all(repo.join(".tmp/scratch")).expect("tmp");
        let f = repo.join(".tmp/scratch/old.txt");
        fs::write(&f, b"x").expect("file");
        let root = canonical_repo_root(&repo).expect("root");
        assert_candidate_jail(&root, Path::new(".tmp/scratch/old.txt")).expect("ok");
        let _ = fs::remove_dir_all(&repo);
    }
}
