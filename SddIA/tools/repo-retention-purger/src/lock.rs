//! Lock exclusivo en `.SddIA/runtime/repo-retention-purger.lock`.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct LockError {
    pub code: &'static str,
    pub detail: String,
}

#[derive(Debug)]
pub struct RuntimeLock {
    path: PathBuf,
}

impl Drop for RuntimeLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub fn lock_path(repo: &Path) -> PathBuf {
    repo.join(".SddIA/runtime/repo-retention-purger.lock")
}

pub fn acquire_runtime_lock(repo: &Path) -> Result<RuntimeLock, LockError> {
    let path = lock_path(repo);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| LockError {
            code: "LOCK_MKDIR",
            detail: e.to_string(),
        })?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| LockError {
            code: "LOCK_HELD",
            detail: format!("{}: {e}", path.display()),
        })?;
    writeln!(file, "pid={}", std::process::id()).map_err(|e| LockError {
        code: "LOCK_WRITE",
        detail: e.to_string(),
    })?;
    Ok(RuntimeLock { path })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn lock_is_exclusive() {
        let repo = std::env::temp_dir().join(format!("sddia-ret-lock-{}", std::process::id()));
        let _ = fs::remove_dir_all(&repo);
        fs::create_dir_all(repo.join(".SddIA/runtime")).unwrap();
        let _a = acquire_runtime_lock(&repo).expect("first");
        let err = acquire_runtime_lock(&repo).expect_err("second");
        assert_eq!(err.code, "LOCK_HELD");
        drop(_a);
        let _b = acquire_runtime_lock(&repo).expect("after drop");
        let _ = fs::remove_dir_all(&repo);
    }
}
