//! Purga de directorios vacíos bajo `.SddIA/workspaces/*/`.

use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

fn is_empty_dir(path: &Path) -> bool {
    let Ok(mut entries) = fs::read_dir(path) else {
        return false;
    };
    entries.next().is_none()
}

fn prune_empty_under(workspaces_root: &Path) -> Result<usize, String> {
    if !workspaces_root.is_dir() {
        return Ok(0);
    }
    let mut pruned = 0usize;
    let process_dirs: Vec<PathBuf> = fs::read_dir(workspaces_root)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    for process_dir in process_dirs {
        let exec_dirs: Vec<PathBuf> = fs::read_dir(&process_dir)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        for exec_dir in exec_dirs {
            if is_empty_dir(&exec_dir) {
                fs::remove_dir(&exec_dir).map_err(|e| e.to_string())?;
                pruned += 1;
            }
        }
    }
    Ok(pruned)
}

pub fn run(repo: &Path, json_out: bool) -> i32 {
    let root = repo.join(".SddIA/workspaces");
    match prune_empty_under(&root) {
        Ok(pruned_count) => {
            if json_out {
                println!(
                    "{}",
                    serde_json::to_string(&json!({
                        "pruned_count": pruned_count,
                        "workspaces_root": root.to_string_lossy(),
                    }))
                    .unwrap_or_default()
                );
            } else {
                println!("workspace-prune: pruned_count={pruned_count}");
            }
            0
        }
        Err(e) => {
            eprintln!("workspace-prune: {e}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use uuid::Uuid;

    #[test]
    fn prune_empty_is_idempotent() {
        let repo = std::env::temp_dir().join(format!("sddia-ws-prune-{}", Uuid::new_v4()));
        let ws = repo.join(".SddIA/workspaces/pull-request-review/empty-one");
        fs::create_dir_all(&ws).unwrap();
        let kept = repo.join(".SddIA/workspaces/delivery-close-cycle/has-file");
        fs::create_dir_all(&kept).unwrap();
        fs::write(kept.join("execution_report.json"), b"{}").unwrap();

        let n1 = prune_empty_under(&repo.join(".SddIA/workspaces")).expect("first");
        assert_eq!(n1, 1);
        assert!(!ws.exists());
        assert!(kept.join("execution_report.json").is_file());

        let n2 = prune_empty_under(&repo.join(".SddIA/workspaces")).expect("second");
        assert_eq!(n2, 0);
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn hook_timing_jsonl_append_from_hook_common() {
        use std::process::Command;
        let repo = std::env::temp_dir().join(format!("sddia-hook-timing-{}", Uuid::new_v4()));
        let hooks = repo.join("SddIA/scripts/qa/git-hooks");
        let common_dir = repo.join("SddIA/scripts/common");
        fs::create_dir_all(&hooks).unwrap();
        fs::create_dir_all(&common_dir).unwrap();
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        fs::copy(
            manifest.join("../../scripts/qa/git-hooks/hook_common.sh"),
            hooks.join("hook_common.sh"),
        )
        .expect("copy hook_common");
        fs::copy(
            manifest.join("../../scripts/common/sddia_shell_lib.sh"),
            common_dir.join("sddia_shell_lib.sh"),
        )
        .expect("copy sddia_shell_lib");
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(&repo)
            .status()
            .expect("git init");
        let script = r#"
set -euo pipefail
source SddIA/scripts/qa/git-hooks/hook_common.sh
hook_timing_begin
hook_timing_record_process route-domain-event
hook_timing_flush pre-push feat-lab-timing
"#;
        let status = Command::new("bash")
            .arg("-c")
            .arg(script)
            .current_dir(&repo)
            .status()
            .expect("bash");
        assert!(status.success(), "hook timing script failed");
        let jsonl = repo.join(".SddIA/proofs/hook-timings/hook-timings.jsonl");
        let raw = fs::read_to_string(&jsonl).expect("jsonl");
        assert!(raw.contains("\"hook\":\"pre-push\""));
        assert!(raw.contains("\"total_ms\""));
        assert!(raw.contains("route-domain-event"));
        let _ = fs::remove_dir_all(&repo);
    }
}
