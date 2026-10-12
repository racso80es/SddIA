//! Suite lab AC-21 — ciclo inverso `done_gate: both` (mock Linear, sin red).

use super::handlers::tracker_markdown_apply::{apply_tracker_issue_changed, sync_branch_name};
use super::handlers::tracker_stamp;
use crate::core::repo::find_repo_root;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

const PROJECT_SLUG: &str = "lab-done-gate-both";
const ISSUE_REF: &str = "LAB-AC21-BOTH";

fn lab_guard() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn with_lab<F: FnOnce()>(f: F) {
    let _g = lab_guard();
    let repo = find_repo_root().expect("repo");
    std::env::set_var("SDDIA_REPO_ROOT", repo.to_string_lossy().as_ref());
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
    std::env::set_var("SDDIA_TRACKER_MARKDOWN_APPLY_SKIP_GIT", "1");
    std::env::remove_var("LINEAR_API_TOKEN");
    let _ = fs::remove_file(repo.join(".SddIA/lab-linear-store.json"));
    f();
    std::env::remove_var("SDDIA_TRACKER_MARKDOWN_APPLY_SKIP_GIT");
    std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
    let _ = fs::remove_file(repo.join(".SddIA/lab-linear-store.json"));
}

fn outbound_path(repo: &Path, issue_ref: &str) -> PathBuf {
    let safe = issue_ref
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
        .collect::<String>();
    repo.join(".SddIA/state/tracker-outbound")
        .join(format!("{safe}.jsonl"))
}

fn cleanup_test_pbi(repo: &Path) {
    for sub in ["pending", "done"] {
        let dir = repo.join("docs/todos").join(sub);
        if let Ok(rd) = fs::read_dir(&dir) {
            for ent in rd.flatten() {
                let name = ent.file_name().to_string_lossy().to_string();
                if name.starts_with("PBI-TEST-AC21") {
                    let _ = fs::remove_file(ent.path());
                }
            }
        }
    }
    let _ = fs::remove_file(outbound_path(repo, ISSUE_REF));
}

fn fixture_pbi(repo: &Path) -> PathBuf {
    cleanup_test_pbi(repo);
    let dir = repo.join("docs/todos/pending");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("PBI-TEST-AC21-BOTH.md");
    fs::write(
        &path,
        format!(
            "---\ndocument_id: PBI-TEST-AC21\ntracker_ref: \"{ISSUE_REF}\"\ntitle: \"AC21 lab\"\nstatus: pending\ntracker_state: in_progress\n---\n\n# AC21\n"
        ),
    )
    .unwrap();
    path
}

fn stamp_merged(repo: &Path, tracker_ref: &str) {
    let event_id = format!("ac21-{}", uuid::Uuid::new_v4());
    let rel = format!(".events/pending/{event_id}.json");
    fs::create_dir_all(repo.join(".events/pending")).ok();
    let event = json!({
        "event_id": event_id,
        "event_type": "PullRequest_Merged",
        "payload": {
            "tracker_ref": tracker_ref,
            "project_slug": PROJECT_SLUG,
            "merge_commit_hash": "ac21merge00000000000000000000000000000001",
        }
    });
    fs::write(repo.join(&rel), serde_json::to_string(&event).unwrap()).unwrap();
    let out = tracker_stamp::run(repo, &json!({ "event_file_path": rel }))
        .expect("tracker-stamp PullRequest_Merged");
    let stamped = out
        .data
        .as_ref()
        .and_then(|d| d.get("stamped"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    assert!(stamped, "stamp data={:?} err={:?}", out.data, out.error);
}

fn tracker_done_event(linear_updated_at: &str) -> Value {
    json!({
        "event_id": uuid::Uuid::new_v4().to_string(),
        "event_type": "Tracker_Issue_Changed",
        "payload": {
            "correlation_id": uuid::Uuid::new_v4().to_string(),
            "occurred_at": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            "linear_updated_at": linear_updated_at,
            "project_slug": PROJECT_SLUG,
            "issue_ref": ISSUE_REF,
            "source": "poll",
            "changes": [{
                "field": "state",
                "from": "In Progress",
                "to": "Done",
            }],
        }
    })
}

fn collect_sync_failed_diverged(repo: &Path) -> Vec<String> {
    let mut hits = Vec::new();
    for sub in [".events/pending", ".events/processing", ".events/processed"] {
        let dir = repo.join(sub);
        if !dir.is_dir() {
            continue;
        }
        for ent in fs::read_dir(&dir).into_iter().flatten().flatten() {
            if ent.path().extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let raw = fs::read_to_string(ent.path()).unwrap_or_default();
            if raw.contains("Tracker_Sync_Failed") && raw.contains("TRACKER_STATE_DIVERGED") {
                hits.push(ent.path().to_string_lossy().to_string());
            }
        }
    }
    hits
}

/// AC-21d: este PBI no activa `done_gate: linear` en Core-self; el lab both sigue en `both`.
fn assert_ac21d_both_fixture_unchanged(repo: &Path) {
    let both_manifest = repo.join("docs/_fixtures/lab-done-gate-project-both/.SddIA/project.md");
    let text = fs::read_to_string(&both_manifest).expect("fixture both");
    assert!(
        text.contains("done_gate: both"),
        "AC-21d: fixture Core-self debe permanecer en both"
    );
    assert!(
        !text.contains("done_gate: linear"),
        "AC-21d: no se escribe linear en el manifiesto del ciclo"
    );
    let index = repo.join(".SddIA/projects/lab-done-gate-both.md");
    assert!(index.is_file(), "índice lab-done-gate-both");
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sddia_daemon_runtime::linear_bridge::outbound_suppresses;

    #[test]
    fn ac21_merge_stamp_then_apply_archives_without_manual_pbi_move() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let pending = fixture_pbi(&repo);
            let fname = pending.file_name().unwrap().to_owned();
            assert!(pending.starts_with(repo.join("docs/todos/pending")));

            stamp_merged(&repo, ISSUE_REF);

            let lu = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
            let event = tracker_done_event(&lu);
            let out = apply_tracker_issue_changed(&repo, &event).expect("apply");
            assert!(out.applied, "warn={:?}", out.warn);
            assert!(
                out.warn.as_deref() != Some("warn: TRACKER_STATE_DIVERGED"),
                "diverged: {:?}",
                out.warn
            );

            assert!(!pending.is_file(), "pending debe quedar vacío tras archive");
            let done_path = repo.join("docs/todos/done").join(fname);
            assert!(done_path.is_file(), "PBI archivado por tracker-markdown-apply");
            let text = fs::read_to_string(&done_path).unwrap();
            assert!(text.contains("status: done"));
            assert!(text.contains("tracker_state: done"));
            let git = out.git.expect("git plan");
            assert_eq!(git.get("branch"), Some(&json!(sync_branch_name(PROJECT_SLUG))));
            assert_ne!(git.get("branch"), Some(&json!("main")));

            let diverged = collect_sync_failed_diverged(&repo);
            assert!(diverged.is_empty(), "AC-21c: {diverged:?}");

            let _ = fs::remove_file(done_path);
            cleanup_test_pbi(&repo);
        });
    }

    #[test]
    fn ac21b_stamp_outbound_suppresses_poll_echo() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let _pending = fixture_pbi(&repo);
            stamp_merged(&repo, ISSUE_REF);
            let ob = outbound_path(&repo, ISSUE_REF);
            assert!(ob.is_file(), "sello debe registrar outbound anti-eco");
            let raw = fs::read_to_string(&ob).unwrap();
            assert!(raw.contains("update_issue_state"));
            assert!(outbound_suppresses(&repo, ISSUE_REF, Utc::now()));
            cleanup_test_pbi(&repo);
        });
    }

    #[test]
    fn ac21d_no_manifest_sets_done_gate_linear() {
        let repo = find_repo_root().expect("repo");
        assert_ac21d_both_fixture_unchanged(&repo);
    }
}
