//! Gate KZ-CA5 / MAN-52: activación por diff de crates de órganos vivos (D-R3).

use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;

fn git_diff_name_only(repo: &Path, range: &str) -> Option<Vec<String>> {
    let out = Command::new("git")
        .args(["-C", &repo.to_string_lossy(), "diff", "--name-only", range])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Some(
        text.lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect(),
    )
}

pub fn branch_changed_paths(repo: &Path) -> Vec<String> {
    for range in ["origin/main...HEAD", "main...HEAD", "HEAD~1..HEAD"] {
        if let Some(paths) = git_diff_name_only(repo, range) {
            return paths;
        }
    }
    Vec::new()
}

pub fn branch_diff_touches_live_organs(repo: &Path) -> bool {
    let paths = branch_changed_paths(repo);
    sddia_runtime_parity::merge_touches_live_organs(&paths)
}

/// Activa el gate de paridad exclusivamente por diff de código de órganos vivos (D-R3).
pub fn persist_requires_runtime_parity_smoke(repo: &Path, _persist_ref: &str) -> bool {
    branch_diff_touches_live_organs(repo)
}

/// Misma lógica que `/api/system-health` → `runtime_binary_parity` (sin HTTP).
pub fn check_runtime_binary_parity_gate(repo: &Path) -> Result<Value, String> {
    if sddia_runtime_parity::allow_stale_runtime() {
        return Ok(json!({
            "ok": true,
            "skipped": true,
            "reason": "SDDIA_ALLOW_STALE_RUNTIME",
        }));
    }
    let orchestrator = sddia_daemon_runtime::resolve_execute_process_elf(repo);
    sddia_runtime_parity::assess_runtime_binary_parity(repo, None, orchestrator.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn git(repo: &Path, args: &[&str]) {
        let st = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .status()
            .unwrap();
        assert!(st.success(), "git {:?} falló", args);
    }

    fn init_repo_with_main() -> tempfile::TempDir {
        let td = tempfile::tempdir().unwrap();
        let r = td.path();
        git(r, &["init", "-b", "main"]);
        git(r, &["config", "user.email", "test@sddia.local"]);
        git(r, &["config", "user.name", "Test"]);
        fs::write(r.join("README.md"), "# base\n").unwrap();
        git(r, &["add", "."]);
        git(r, &["commit", "-m", "base"]);
        td
    }

    #[test]
    fn triggers_on_live_organ_crate_diff_without_markdown_mention() {
        let td = init_repo_with_main();
        let r = td.path();
        git(r, &["checkout", "-b", "feat/organ-change"]);
        fs::create_dir_all(r.join("SddIA/daemons/event-watcher/src")).unwrap();
        fs::write(
            r.join("SddIA/daemons/event-watcher/src/main.rs"),
            "fn main() {}\n",
        )
        .unwrap();
        fs::create_dir_all(r.join("docs/features/x")).unwrap();
        fs::write(r.join("docs/features/x/spec.md"), "# spec sin tokens\n").unwrap();
        git(r, &["add", "."]);
        git(r, &["commit", "-m", "touch event-watcher"]);

        assert!(persist_requires_runtime_parity_smoke(r, "docs/features/x"));
    }

    #[test]
    fn does_not_trigger_on_docs_only_diff_even_with_token() {
        let td = init_repo_with_main();
        let r = td.path();
        git(r, &["checkout", "-b", "feat/docs-only"]);
        fs::create_dir_all(r.join("docs/features/x")).unwrap();
        fs::write(
            r.join("docs/features/x/spec.md"),
            "# spec\n\nMenciona kalma2-bridge pero no toca código.\n",
        )
        .unwrap();
        git(r, &["add", "."]);
        git(r, &["commit", "-m", "docs only"]);

        assert!(!persist_requires_runtime_parity_smoke(r, "docs/features/x"));
    }
}
