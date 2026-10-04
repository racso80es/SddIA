//! Suite lab AC-7..AC-10 — ciclo directo Linear (mock, sin red).

use super::handlers::forge_pbi;
use super::handlers::refine;
use super::handlers::tracker_stamp;
use crate::core::repo::find_repo_root;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

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
    std::env::remove_var("LINEAR_API_TOKEN");
    let _ = fs::remove_file(repo.join(".SddIA/lab-linear-store.json"));
    f();
    std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
    let _ = fs::remove_file(repo.join(".SddIA/lab-linear-store.json"));
}

fn manifest(tracker: bool, delivery_mode: &str) -> String {
    let tracker_block = if tracker {
        "tracker:\n  provider: linear\n  team_key: OSC\n  state_map:\n    backlog: Backlog\n    todo: Todo\n    in_progress: In Progress\n    in_review: In Review\n    done: Done\n  labels:\n    hu: hu\n    pbi: pbi\n    fix: fix\n"
    } else {
        ""
    };
    format!(
        "---\nid: lab\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\ngit_remote: https://example.invalid/lab.git\ndefault_branch: main\ndelivery_mode: {delivery_mode}\ncontract_version: \"1.3.0\"\ncodex_slug: codex-software-engineering\n{tracker_block}docs_layout:\n  features: docs/features\n  fixes: docs/fixes\n  todos_pending: docs/todos/pending\n  todos_done: docs/todos/done\n---\n\n# lab\n"
    )
}

fn register_project(core: &Path, slug: &str, tracker: bool, delivery_mode: &str) -> PathBuf {
    let client = core.join(format!("_linear_e2e_{slug}"));
    fs::create_dir_all(client.join(".git")).ok();
    fs::create_dir_all(client.join(".SddIA")).unwrap();
    fs::write(
        client.join(".SddIA/project.md"),
        manifest(tracker, delivery_mode),
    )
    .unwrap();
    fs::create_dir_all(core.join(".SddIA/projects")).unwrap();
    fs::write(
        core.join(".SddIA/projects").join(format!("{slug}.md")),
        format!(
            "---\nid: {slug}\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\nproject_root: \"{}\"\nmanifest_ref: .SddIA/project.md\ncodex_slug: codex-software-engineering\nstatus: active\n---\n\n# {slug}\n",
            client.display()
        ),
    )
    .unwrap();
    client
}

fn cleanup(core: &Path, slug: &str) {
    let _ = fs::remove_file(core.join(".SddIA/projects").join(format!("{slug}.md")));
    let _ = fs::remove_dir_all(core.join(format!("_linear_e2e_{slug}")));
}

fn stamp(repo: &Path, event_type: &str, payload: Value) -> bool {
    let event_id = format!("e2e-{}", uuid::Uuid::new_v4());
    let rel = format!(".events/pending/{event_id}.json");
    fs::create_dir_all(repo.join(".events/pending")).ok();
    let event = json!({
        "event_id": event_id,
        "event_type": event_type,
        "payload": payload,
    });
    fs::write(repo.join(&rel), serde_json::to_string(&event).unwrap()).unwrap();
    let out = tracker_stamp::run(repo, &json!({ "event_file_path": rel }))
        .expect("tracker-stamp");
    let stamped = out
        .data
        .as_ref()
        .and_then(|d| d.get("stamped"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if !stamped {
        panic!(
            "stamp {event_type} no aplicado: data={:?} error={:?}",
            out.data,
            out.error
        );
    }
    stamped
}

fn yaml_field(text: &str, key: &str) -> Option<String> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let rest = trimmed.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    for line in rest[..end].lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix(&format!("{key}:")) {
            let v = val.trim().trim_matches('"');
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

fn assert_no_secrets(repo: &Path) {
    let forbidden = ["LINEAR_API_TOKEN", "super-secret-token", "\"data\":{\"issue\""];
    let dirs = [".events/pending", ".events/processed"];
    for dir in dirs {
        let p = repo.join(dir);
        if !p.is_dir() {
            continue;
        }
        for entry in fs::read_dir(p).into_iter().flatten().flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let raw = fs::read_to_string(entry.path()).unwrap_or_default();
            for needle in forbidden {
                assert!(
                    !raw.contains(needle),
                    "secreto o cuerpo Linear en {}",
                    entry.path().display()
                );
            }
        }
    }
}

fn run_branch_pr_cycle(repo: &Path, slug: &str) -> (String, String) {
    let _client = register_project(repo, slug, true, "branch_pr");
    let hu_rel = "docs/todos/historias/hu-e2e.md";
    let hu_path = repo.join(format!("_linear_e2e_{slug}")).join(hu_rel);
    fs::create_dir_all(hu_path.parent().unwrap()).unwrap();
    fs::write(
        &hu_path,
        "---\ndocument_id: HU-E2E\nstatus: pending\n---\n\n# HU E2E\n",
    )
    .unwrap();

    refine::run_hu(
        repo,
        &json!({
            "project_slug": slug,
            "hu_ref": hu_rel,
        }),
    )
    .expect("refine-hu");
    let hu_text = fs::read_to_string(&hu_path).unwrap();
    let hu_tracker = yaml_field(&hu_text, "tracker_ref").expect("tracker_ref HU");

    stamp(
        repo,
        "HU_Refined",
        json!({
            "tracker_ref": hu_tracker,
            "hu_ref": hu_rel,
            "project_slug": slug,
        }),
    );

    let forge = forge_pbi::run(
        repo,
        &json!({
            "project_slug": slug,
            "raw_idea": "PBI lab E2E",
            "process": "feature",
            "historia_ref": hu_rel,
        }),
    )
    .expect("forge-pbi");
    let pbi_rel = forge
        .data
        .as_ref()
        .and_then(|d| d.get("artifact_path"))
        .and_then(|v| v.as_str())
        .expect("artifact_path");
    let pbi_tracker = forge
        .data
        .as_ref()
        .and_then(|d| d.get("tracker_ref"))
        .and_then(|v| v.as_str())
        .expect("tracker_ref PBI");
    let document_id = forge
        .data
        .as_ref()
        .and_then(|d| d.get("document_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("PBI");

    stamp(
        repo,
        "PBI_Forged",
        json!({
            "tracker_ref": pbi_tracker,
            "document_id": document_id,
            "project_slug": slug,
            "expected_hu_tracker_ref": hu_tracker,
        }),
    );

    refine::run_pbi(
        repo,
        &json!({
            "project_slug": slug,
            "pbi_ref": pbi_rel,
        }),
    )
    .expect("refine-pbi");

    assert!(stamp(
        repo,
        "PBI_Refined",
        json!({
            "tracker_ref": pbi_tracker,
            "pbi_ref": pbi_rel,
            "project_slug": slug,
        }),
    ));

    assert!(stamp(
        repo,
        "Work_Initiated",
        json!({
            "tracker_ref": pbi_tracker,
            "branch": "feat/linear-e2e",
            "persist_ref": "docs/features/linear-e2e-lab",
            "expected_hu_tracker_ref": hu_tracker,
        }),
    ));

    assert!(stamp(
        repo,
        "PullRequest_Presented",
        json!({
            "tracker_ref": pbi_tracker,
            "pr_url": "https://github.com/o/r/pull/99",
        }),
    ));

    assert!(stamp(
        repo,
        "PullRequest_Merged",
        json!({
            "tracker_ref": pbi_tracker,
            "merge_commit_hash": "abc123deadbeefcafe0000000000000000000000",
        }),
    ));

    (hu_tracker, pbi_tracker.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ac7_branch_pr_full_cycle_with_comments() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let slug = "linear-e2e-ac7";
            let (hu_tr, pbi_tr) = run_branch_pr_cycle(&repo, slug);
            assert!(!hu_tr.is_empty());
            assert!(!pbi_tr.is_empty());
            assert_no_secrets(&repo);
            cleanup(&repo, slug);
        });
    }

    #[test]
    fn ac8_trunk_direct_delivery_committed() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let slug = "linear-e2e-ac8";
            let _client = register_project(&repo, slug, true, "trunk_direct");
            let hu_rel = "docs/todos/historias/hu-trunk.md";
            let hu_path = repo.join(format!("_linear_e2e_{slug}")).join(hu_rel);
            fs::create_dir_all(hu_path.parent().unwrap()).unwrap();
            fs::write(&hu_path, "---\nstatus: pending\n---\n\n# HU\n").unwrap();

            refine::run_hu(
                &repo,
                &json!({ "project_slug": slug, "hu_ref": hu_rel }),
            )
            .expect("refine-hu");
            let hu_tracker = yaml_field(&fs::read_to_string(&hu_path).unwrap(), "tracker_ref")
                .expect("hu tracker");

            stamp(
                &repo,
                "HU_Refined",
                json!({ "tracker_ref": hu_tracker, "hu_ref": hu_rel }),
            );

            let forge = forge_pbi::run(
                &repo,
                &json!({
                    "project_slug": slug,
                    "raw_idea": "trunk",
                    "process": "feature",
                    "historia_ref": hu_rel,
                }),
            )
            .expect("forge");
            let pbi_tr = forge
                .data
                .as_ref()
                .and_then(|d| d.get("tracker_ref"))
                .and_then(|v| v.as_str())
                .expect("pbi tr");
            let pbi_rel = forge
                .data
                .as_ref()
                .and_then(|d| d.get("artifact_path"))
                .and_then(|v| v.as_str())
                .expect("path");

            stamp(
                &repo,
                "PBI_Forged",
                json!({ "tracker_ref": pbi_tr, "document_id": "PBI-X" }),
            );
            refine::run_pbi(
                &repo,
                &json!({ "project_slug": slug, "pbi_ref": pbi_rel }),
            )
            .expect("refine-pbi");
            stamp(
                &repo,
                "PBI_Refined",
                json!({ "tracker_ref": pbi_tr, "pbi_ref": pbi_rel }),
            );
            stamp(
                &repo,
                "Work_Initiated",
                json!({
                    "tracker_ref": pbi_tr,
                    "branch": "main",
                    "persist_ref": "docs/features/x",
                    "expected_hu_tracker_ref": hu_tracker,
                }),
            );
            assert!(stamp(
                &repo,
                "Delivery_Committed",
                json!({
                    "tracker_ref": pbi_tr,
                    "commit_sha": "deadbeef",
                    "project_slug": slug,
                }),
            ));
            assert_no_secrets(&repo);
            cleanup(&repo, slug);
        });
    }

    #[test]
    fn ac9_no_secrets_in_pending_events() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let slug = "linear-e2e-ac9";
            run_branch_pr_cycle(&repo, slug);
            assert_no_secrets(&repo);
            cleanup(&repo, slug);
        });
    }

    #[test]
    fn ac10_core_self_without_tracker_still_green() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let slug = "linear-e2e-ac10";
            let _client = register_project(&repo, slug, false, "branch_pr");
            let hu_rel = "docs/todos/historias/hu-core.md";
            let hu_path = repo.join(format!("_linear_e2e_{slug}")).join(hu_rel);
            fs::create_dir_all(hu_path.parent().unwrap()).unwrap();
            fs::write(&hu_path, "---\nstatus: pending\n---\n\n# HU\n").unwrap();

            let hu = refine::run_hu(
                &repo,
                &json!({ "project_slug": slug, "hu_ref": hu_rel }),
            )
            .expect("refine-hu sin tracker");
            assert!(hu.success);

            let forge = forge_pbi::run(
                &repo,
                &json!({
                    "project_slug": slug,
                    "raw_idea": "sin tracker",
                    "process": "feature",
                    "historia_ref": hu_rel,
                }),
            )
            .expect("forge sin tracker");
            assert!(
                forge
                    .data
                    .as_ref()
                    .and_then(|d| d.get("create_issue_invoked"))
                    .is_none()
                    || forge
                        .data
                        .as_ref()
                        .and_then(|d| d.get("create_issue_invoked"))
                        .and_then(|v| v.as_bool())
                        == Some(false)
            );

            let pbi_rel = forge
                .data
                .as_ref()
                .and_then(|d| d.get("artifact_path"))
                .and_then(|v| v.as_str())
                .expect("pbi");
            let pbi = refine::run_pbi(
                &repo,
                &json!({ "project_slug": slug, "pbi_ref": pbi_rel }),
            )
            .expect("refine-pbi sin tracker");
            assert!(pbi.success);
            cleanup(&repo, slug);
        });
    }
}
