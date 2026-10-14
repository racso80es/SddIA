//! Gate de Done (`tracker.done_gate`): git | linear | both (HU-B 05).

use super::actions::try_run_native;
use super::capsules::invoke_tool_for_process;
use super::project_binding::{self, DoneGate, TrackerConfig};
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub const PROOF_DIR_REL: &str = ".SddIA/proofs/tracker-done";

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn proof_filename(tracker_ref: &str) -> String {
    tracker_ref
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

pub fn proof_path(repo: &Path, tracker_ref: &str) -> PathBuf {
    repo.join(PROOF_DIR_REL).join(format!("{}.json", proof_filename(tracker_ref)))
}

pub fn write_tracker_done_proof(
    repo: &Path,
    tracker_ref: &str,
    commit_sha: &str,
    fetch: &Value,
) -> Result<(), String> {
    let dir = repo.join(PROOF_DIR_REL);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let body = json!({
        "tracker_ref": tracker_ref,
        "state": "done",
        "occurred_at": Utc::now().to_rfc3339(),
        "commit_sha": commit_sha,
        "issue": fetch,
    });
    let path = proof_path(repo, tracker_ref);
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(&body).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn read_tracker_done_proof(repo: &Path, tracker_ref: &str) -> Option<Value> {
    let path = proof_path(repo, tracker_ref);
    let text = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn fetch_issue(repo: &Path, issue_ref: &str, source_process: &str) -> Result<Value, String> {
    invoke_tool_for_process(
        repo,
        "linear-tracker-adapter",
        &json!({
            "request": {
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
            }
        }),
        Some(source_process),
    )
}

pub fn linear_to_canonical(linear_state: &str, tc: &TrackerConfig) -> String {
    for (canonical, name) in &tc.state_map {
        if name.eq_ignore_ascii_case(linear_state) {
            return canonical.clone();
        }
    }
    linear_state.to_ascii_lowercase().replace(' ', "_")
}

pub fn canonical_is_done(canonical: &str) -> bool {
    canonical == "done"
}

fn yaml_str_from_fm(fm: &std::collections::HashMap<String, serde_yaml::Value>, key: &str) -> Option<String> {
    fm.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

pub fn tracker_ref_from_persist(repo: &Path, persist_ref: &str) -> Option<String> {
    for name in ["objectives.md", "spec.md"] {
        let p = repo.join(persist_ref).join(name);
        if !p.is_file() {
            continue;
        }
        let fm = crate::core::parser::parse_frontmatter(&p).ok()?;
        if let Some(r) = yaml_str_from_fm(&fm, "tracker_ref") {
            return Some(r);
        }
    }
    None
}

pub fn project_slug_from_persist(repo: &Path, persist_ref: &str) -> String {
    let objectives = repo.join(persist_ref).join("objectives.md");
    if objectives.is_file() {
        if let Ok(fm) = crate::core::parser::parse_frontmatter(&objectives) {
            if let Some(s) = yaml_str_from_fm(&fm, "project_slug") {
                return s;
            }
        }
    }
    if project_binding::repo_is_core(repo) {
        "core".into()
    } else {
        "default".into()
    }
}

pub fn resolve_done_gate(repo: &Path, persist_ref: &str) -> Result<DoneGate, String> {
    let slug = project_slug_from_persist(repo, persist_ref);
    let inputs = json!({ "project_slug": slug });
    let gate = project_binding::done_gate_for_inputs(repo, &inputs)?;
    // L-DEGRADE / AC-21: Linear|Both sin tracker_ref → Git (no-op features locales).
    if matches!(gate, DoneGate::Linear | DoneGate::Both)
        && tracker_ref_from_persist(repo, persist_ref).is_none()
    {
        eprintln!(
            "warn: done_gate degraded to git: tracker_ref ausente (persist_ref={persist_ref}, slug={slug})"
        );
        return Ok(DoneGate::Git);
    }
    Ok(gate)
}

/// Pre-push: con `linear`, exige proof local (sin red). Con `git`/`both`+proof, no bloquea por proof.
pub fn pre_push_check(repo: &Path, persist_ref: Option<&str>) -> Result<(), String> {
    let persist_ref = persist_ref
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "DONE_GATE: persist_ref ausente".to_string())?;
    let gate = resolve_done_gate(repo, persist_ref).unwrap_or(DoneGate::Git);
    if gate == DoneGate::Git {
        return Ok(());
    }
    let tracker_ref = tracker_ref_from_persist(repo, persist_ref)
        .ok_or_else(|| "DONE_GATE: tracker_ref no resuelto en persist_ref".to_string())?;
    if read_tracker_done_proof(repo, &tracker_ref).is_some() {
        return Ok(());
    }
    if gate == DoneGate::Both {
        return Ok(());
    }
    Err(format!(
        "DONE_GATE_LINEAR: falta evidencia local {} para tracker_ref {}",
        proof_path(repo, &tracker_ref).display(),
        tracker_ref
    ))
}

pub enum LinearClosure {
    Satisfied,
    Rejected { tracker_ref: String, state: String },
    FallbackGit { warn: String },
}

pub fn check_linear_closure(
    repo: &Path,
    persist_ref: &str,
    gate: DoneGate,
) -> Result<LinearClosure, String> {
    let tracker_ref = tracker_ref_from_persist(repo, persist_ref)
        .ok_or_else(|| "DONE_GATE: tracker_ref no resuelto".to_string())?;
    let slug = project_slug_from_persist(repo, persist_ref);
    let inputs = json!({ "project_slug": slug });
    let tc = project_binding::tracker_config_for_inputs(repo, &inputs)?
        .ok_or_else(|| "DONE_GATE: proyecto sin tracker".to_string())?;

    let fetch = match fetch_issue(repo, &tracker_ref, "feature-pbi-archive") {
        Ok(v) => v,
        Err(e) if gate == DoneGate::Both => {
            let _ = try_run_native(
                repo,
                "emit-tracker-sync-failed",
                &json!({
                    "issue_ref": tracker_ref,
                    "operation": "fetch_issue",
                    "error_code": "LINEAR_UNAVAILABLE",
                    "source_process": "feature-pbi-archive",
                    "project_slug": slug,
                }),
            );
            return Ok(LinearClosure::FallbackGit {
                warn: format!("warn: Linear caído, fallback git: {e}"),
            });
        }
        Err(e) => return Err(e),
    };

    let linear_state = fetch
        .get("state")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let canonical = linear_to_canonical(linear_state, &tc);
    if canonical_is_done(&canonical) {
        return Ok(LinearClosure::Satisfied);
    }
    Ok(LinearClosure::Rejected {
        tracker_ref,
        state: canonical,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use crate::engine::phase_capsules::capsule_feature_pbi_archive;
    use crate::engine::project_binding::DoneGate;
    use serde_json::json;
    use std::env;
    use tempfile::TempDir;

    fn with_lab<F: FnOnce()>(f: F) {
        let prev = env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
        env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        f();
        if let Some(v) = prev {
            env::set_var("SDDIA_LAB_MOCK_OUTBOUND", v);
        } else {
            env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        }
    }

    #[test]
    fn proof_roundtrip_no_network() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        write_tracker_done_proof(
            repo,
            "OSC-42",
            "abc123",
            &json!({ "state": "Done" }),
        )
        .unwrap();
        let p = read_tracker_done_proof(repo, "OSC-42").unwrap();
        assert_eq!(p.get("tracker_ref"), Some(&json!("OSC-42")));
        assert_eq!(p.get("commit_sha"), Some(&json!("abc123")));
    }

    #[test]
    fn ac18_linear_done_defers_archive() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let persist = "docs/features/_fixture-done-gate-lab";
            let mut state = json!({});
            let out = capsule_feature_pbi_archive(
                &repo,
                &json!({ "persist_ref": persist }),
                &mut state,
            )
            .expect("archive");
            assert_eq!(out.get("archive_deferred"), Some(&json!(true)));
            assert_eq!(out.get("done_gate"), Some(&json!("linear")));
        });
    }

    #[test]
    fn ac18_linear_in_review_rejects() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let persist = "docs/features/_fixture-done-gate-lab-review";
            let err = capsule_feature_pbi_archive(
                &repo,
                &json!({ "persist_ref": persist }),
                &mut json!({}),
            )
            .unwrap_err();
            assert!(err.contains("LAB-DONE-GATE-REVIEW"), "{err}");
            assert!(err.contains("DONE_GATE_LINEAR"));
        });
    }

    #[test]
    fn ac19_both_linear_down_fallback_git_archive() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let pbi = repo.join("docs/todos/pending/PBI-TEST-DONE-GATE-FALLBACK.md");
            fs::write(
                &pbi,
                "---\ndocument_id: PBI-TEST-DG\ntitle: t\nstatus: pending\n---\n\n# t\n",
            )
            .unwrap();
            let mut state = json!({});
            let out = capsule_feature_pbi_archive(
                &repo,
                &json!({
                    "persist_ref": "docs/features/_fixture-done-gate-lab-both",
                    "related_todo": "docs/todos/pending/PBI-TEST-DONE-GATE-FALLBACK.md",
                }),
                &mut state,
            )
            .expect("archive");
            let _ = fs::remove_file(&pbi);
            let done = repo.join("docs/todos/done/PBI-TEST-DONE-GATE-FALLBACK.md");
            let _ = fs::remove_file(&done);
            assert_eq!(out.get("archived"), Some(&json!(true)));
            assert!(state.get("done_gate_warn").is_some());
        });
    }

    #[test]
    fn ac20_pre_push_reads_local_proof_without_network() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let persist = "docs/features/_fixture-done-gate-lab";
            let err = pre_push_check(&repo, Some(persist)).unwrap_err();
            assert!(err.contains("LAB-PBI-CHILD-A"));
            write_tracker_done_proof(
                &repo,
                "LAB-PBI-CHILD-A",
                "deadbeef",
                &json!({ "state": "Done" }),
            )
            .unwrap();
            assert!(pre_push_check(&repo, Some(persist)).is_ok());
            let _ = fs::remove_file(proof_path(&repo, "LAB-PBI-CHILD-A"));
        });
    }

    #[test]
    fn ac17_git_still_requires_pbi_archived() {
        let repo = find_repo_root().expect("repo");
        let persist = "docs/features/linear-hu-b-05-done-gate";
        let val = repo.join(persist).join("validacion.md");
        let original = fs::read(&val).expect("validacion original");
        struct Restore(PathBuf, Vec<u8>);
        impl Drop for Restore {
            fn drop(&mut self) {
                let _ = fs::write(&self.0, &self.1);
            }
        }
        let _restore = Restore(val.clone(), original);
        fs::write(
            &val,
            "---\nfeature_name: linear-hu-b-05-done-gate\nglobal: APTO\npbi_archived: false\nbranch: feat/x\n---\n\n# v\n",
        )
        .unwrap();
        let out = capsule_feature_pbi_archive(
            &repo,
            &json!({ "persist_ref": persist }),
            &mut json!({}),
        )
        .expect("archive");
        assert_eq!(
            out.get("reason").and_then(|v| v.as_str()),
            Some("pbi_archived != true")
        );
    }

    #[test]
    fn ac23_resolve_linear_gate_from_fixture_manifest() {
        let repo = find_repo_root().expect("repo");
        let g = resolve_done_gate(&repo, "docs/features/_fixture-done-gate-lab").unwrap();
        assert_eq!(g, DoneGate::Linear);
    }

    fn write_core_linear_layout(repo: &Path, uuid: &str) {
        fs::create_dir_all(repo.join(".SddIA/projects")).unwrap();
        fs::write(
            repo.join(".SddIA/projects/core.md"),
            format!(
                "---\nid: core\nuuid: \"{uuid}\"\nproject_root: .\nmanifest_ref: .SddIA/project.md\ncodex_slug: codex-software-engineering\nstatus: active\n---\n\n# core\n"
            ),
        )
        .unwrap();
        fs::write(
            repo.join(".SddIA/project.md"),
            format!(
                "---\nid: core\nuuid: \"{uuid}\"\ngit_remote: https://example.invalid/core.git\ndefault_branch: main\ndelivery_mode: branch_pr\ncontract_version: \"1.3.0\"\ncodex_slug: codex-software-engineering\ntracker:\n  provider: linear\n  team_key: OSC\n  done_gate: linear\n  state_map:\n    backlog: Backlog\n    done: Done\ndocs_layout:\n  features: docs/features\n  fixes: docs/fixes\n  todos_pending: docs/todos/pending\n  todos_done: docs/todos/done\n---\n\n# core\n"
            ),
        )
        .unwrap();
        // Marcador mínimo de Core para project_slug_from_persist → "core".
        fs::create_dir_all(repo.join("SddIA")).unwrap();
        fs::write(repo.join("SddIA/CONSTITUTION_CORE.md"), "# x\n").unwrap();
    }

    #[test]
    fn resolve_core_linear_with_tracker_ref() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        write_core_linear_layout(repo, uuid);
        let persist = "docs/features/_t-core-linear";
        fs::create_dir_all(repo.join(persist)).unwrap();
        fs::write(
            repo.join(persist).join("objectives.md"),
            "---\nfeature_name: t\nproject_slug: core\ntracker_ref: OSC-999\n---\n\n# o\n",
        )
        .unwrap();
        let g = resolve_done_gate(repo, persist).unwrap();
        assert_eq!(g, DoneGate::Linear);
        let err = pre_push_check(repo, Some(persist)).unwrap_err();
        assert!(err.contains("DONE_GATE_LINEAR") || err.contains("OSC-999"), "{err}");
        write_tracker_done_proof(repo, "OSC-999", "deadbeef", &json!({ "state": "Done" })).unwrap();
        assert!(pre_push_check(repo, Some(persist)).is_ok());
    }

    #[test]
    fn resolve_core_degrades_without_tracker_ref() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        write_core_linear_layout(repo, uuid);
        let persist = "docs/features/_t-core-no-tracker";
        fs::create_dir_all(repo.join(persist)).unwrap();
        fs::write(
            repo.join(persist).join("objectives.md"),
            "---\nfeature_name: t\nproject_slug: core\n---\n\n# o\n",
        )
        .unwrap();
        let g = resolve_done_gate(repo, persist).unwrap();
        assert_eq!(g, DoneGate::Git);
        assert!(pre_push_check(repo, Some(persist)).is_ok());
    }

    #[test]
    fn di_other_projects_unchanged() {
        let repo = find_repo_root().expect("repo");
        let bx = project_binding::done_gate_for_inputs(
            &repo,
            &json!({ "project_slug": "barcelonaxplorer" }),
        );
        // Puede fallar si el checkout externo no existe en CI; solo asertar si resuelve.
        if let Ok(g) = bx {
            assert_ne!(g, DoneGate::Linear, "D-I: barcelonaxplorer no debe ser linear");
        }
        let both = project_binding::done_gate_for_inputs(
            &repo,
            &json!({ "project_slug": "lab-done-gate-both" }),
        )
        .expect("lab-done-gate-both");
        assert_eq!(both, DoneGate::Both);
        let core = project_binding::done_gate_for_inputs(&repo, &json!({ "project_slug": "core" }))
            .expect("core registrado");
        assert_eq!(core, DoneGate::Linear);
    }
}
