//! Sellado lab de `forge-pbi`: formato físico y registro en el tracker.

use super::super::capsules::invoke_tool_for_process;
use super::super::project_binding::{self, TrackerConfig};
use super::super::route_domain_core::materialize_pending_domain_event;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use uuid::Uuid;

const PROCESS_NAME: &str = "forge-pbi";

fn str_field(inputs: &Value, key: &str) -> Option<String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Clave de `tracker.labels.*` según R-3 (HU-A 07).
pub fn tipo_label_key(inputs: &Value, process: &str) -> Option<&'static str> {
    let raw = inputs
        .get("tipo")
        .or_else(|| inputs.get("type"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if let Some(t) = raw {
        return match t.to_ascii_lowercase().as_str() {
            "kaizen" | "mejora" => Some("kaizen"),
            "deuda" => Some("deuda"),
            "spike" => Some("spike"),
            "fix" | "bug-fix" | "bug_fix" => Some("fix"),
            "operativo" | "feature" | "refactorization" => None,
            _ => None,
        };
    }
    if process == "bug-fix" {
        return Some("fix");
    }
    None
}

fn emit_create_issue_failed(
    repo: &Path,
    issue_ref: &str,
    error_code: &str,
    project_slug: &str,
) -> Result<(), String> {
    let event_id = Uuid::new_v4().to_string();
    let payload = json!({
        "issue_ref": issue_ref,
        "operation": "create_issue",
        "error_code": error_code,
        "source_process": PROCESS_NAME,
        "occurred_at": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "project_slug": project_slug,
        "event_id": event_id,
        "correlation_id": event_id,
    });
    materialize_pending_domain_event(repo, "Tracker_Sync_Failed", PROCESS_NAME, payload)?;
    Ok(())
}

fn backlog_state_name(tc: &TrackerConfig) -> Option<String> {
    tc.state_map.get("backlog").cloned()
}

fn build_pbi_labels(
    repo: &Path,
    inputs: &Value,
    tc: &TrackerConfig,
    process: &str,
) -> Result<Vec<String>, String> {
    let mut labels = vec![tc.label_pbi.clone()];
    if let Some(key) = tipo_label_key(inputs, process) {
        if let Some(name) = project_binding::tracker_label_for_inputs(repo, inputs, key)? {
            if name != tc.label_pbi {
                labels.push(name);
            }
        }
    }
    Ok(labels)
}

struct TrackerPhase {
    tracker_ref: Option<String>,
    invoked: bool,
    skipped_reason: Option<String>,
}

fn tracker_registration_phase(
    repo: &Path,
    bound: &project_binding::BoundProject,
    inputs: &Value,
    process: &str,
    title: &str,
    description: &str,
) -> TrackerPhase {
    let parent = match str_field(inputs, "parent_ref") {
        Some(p) => p,
        None => {
            let href = str_field(inputs, "historia_ref").unwrap_or_else(|| "local:missing-parent_ref".into());
            let _ = emit_create_issue_failed(repo, &href, "PARENT_REF_REQUIRED", &bound.slug);
            return TrackerPhase {
                tracker_ref: None,
                invoked: false,
                skipped_reason: Some("parent_ref ausente".into()),
            };
        }
    };
    let tool = match super::tracker_stamp::lifecycle_tool_name_for(repo, inputs) {
        Ok(t) => t,
        Err(e) => {
            return TrackerPhase {
                tracker_ref: None,
                invoked: false,
                skipped_reason: Some(e),
            };
        }
    };
    if matches!(
        super::super::tracker_lifecycle_port::resolve_target(repo, inputs),
        Ok(super::super::tracker_lifecycle_port::LifecycleTarget::Unconfigured)
    ) {
        return TrackerPhase {
            tracker_ref: None,
            invoked: false,
            skipped_reason: Some("sin tracker en manifiesto".into()),
        };
    }
    let tc = super::super::project_binding::tracker_config_for_inputs(repo, inputs)
        .ok()
        .flatten();
    let labels = if let Some(tc) = &tc {
        match build_pbi_labels(repo, inputs, tc, process) {
            Ok(l) => l,
            Err(e) => {
                return TrackerPhase {
                    tracker_ref: None,
                    invoked: false,
                    skipped_reason: Some(e),
                };
            }
        }
    } else {
        vec!["pbi".into()]
    };
    let mut request = json!({
        "operation": "create_issue",
        "title": title,
        "description": description,
        "labels": labels,
        "parent_ref": parent,
        "project_slug": bound.slug,
    });
    if let Some(tc) = &tc {
        request["team_key"] = json!(tc.team_key);
        if let Some(pid) = &tc.project_id {
            request["project_id"] = json!(pid);
        }
        if let Some(state) = backlog_state_name(tc) {
            request["state_name"] = json!(state);
        }
    }
    match invoke_tool_for_process(repo, &tool, &json!({ "request": request }), Some(PROCESS_NAME)) {
        Ok(body) => {
            let issue_ref = body
                .get("issue_ref")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);
            match issue_ref {
                Some(issue_ref) => TrackerPhase {
                    tracker_ref: Some(issue_ref),
                    invoked: true,
                    skipped_reason: None,
                },
                None => TrackerPhase {
                    tracker_ref: None,
                    invoked: true,
                    skipped_reason: Some("sin tracker_ref".into()),
                },
            }
        }
        Err(code) => {
            let _ = emit_create_issue_failed(repo, &parent, &code, &bound.slug);
            TrackerPhase {
                tracker_ref: None,
                invoked: true,
                skipped_reason: Some(code),
            }
        }
    }
}

pub fn run(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let raw_idea = inputs
        .get("raw_idea")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("raw_idea requerido")?;
    let process = inputs
        .get("process")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| matches!(*s, "feature" | "bug-fix" | "refactorization"))
        .unwrap_or("feature");
    let bound = project_binding::bind(repo, inputs)?
        .ok_or("project_slug requerido para forge-pbi")?;
    let document_id = format!("PBI-{}", &Uuid::new_v4().to_string()[..8]);
    let entity_uuid = Uuid::new_v4().to_string();
    let created = Utc::now().format("%Y-%m-%d").to_string();
    let title = document_id.clone();
    let lifecycle_fm = super::super::item_lifecycle_frontmatter::forge_frontmatter_lines();
    let mut fm_lines = format!(
        "document_id: {document_id}\nuuid: \"{entity_uuid}\"\n{lifecycle_fm}\nprocess: {process}\ncreated: \"{created}\"\nproject_slug: {slug_project}\ndelivery_mode: {mode}\ndelivery_mode_source: {source}",
        slug_project = bound.slug,
        mode = bound.delivery_mode.as_str(),
        source = bound.delivery_mode_source,
    );
    if let Some(href) = str_field(inputs, "historia_ref") {
        fm_lines.push_str(&format!("\nhistoria_ref: \"{href}\""));
    }
    let draft = format!("---\n{fm_lines}\n---\n\n# {document_id}\n\n{raw_idea}\n");
    let tracker = tracker_registration_phase(
        repo,
        &bound,
        inputs,
        process,
        &title,
        &draft,
    );
    let body = if let Some(tr) = &tracker.tracker_ref {
        let with_ref = format!("{fm_lines}\ntracker_ref: \"{tr}\"");
        format!("---\n{with_ref}\n---\n\n# {document_id}\n\n{raw_idea}\n")
    } else {
        draft.clone()
    };
    if let Some(tr) = &tracker.tracker_ref {
        let tool = super::tracker_stamp::lifecycle_tool_name_for(repo, inputs)?;
        let _ = invoke_tool_for_process(
            repo,
            &tool,
            &json!({
                "request": {
                    "operation": "update_issue_description",
                    "issue_ref": tr,
                    "description": body,
                    "project_slug": bound.slug,
                }
            }),
            Some(PROCESS_NAME),
        );
    }
    let mut event_payload = json!({
        "document_id": document_id,
        "project_slug": bound.slug,
        "process": process,
    });
    if let Some(tr) = &tracker.tracker_ref {
        event_payload["tracker_ref"] = json!(tr);
    }
    let event_rel = materialize_pending_domain_event(
        repo,
        "PBI_Forged",
        PROCESS_NAME,
        event_payload,
    )?;
    let registro_status = if tracker.invoked {
        if tracker.tracker_ref.is_some() {
            "executed"
        } else {
            "warn"
        }
    } else {
        "skipped"
    };
    let mut data = json!({
        "document_id": document_id,
        "uuid": entity_uuid,
        "event_path": event_rel,
        "create_issue_invoked": tracker.invoked,
        "delivery_mode": bound.delivery_mode.as_str(),
        "delivery_mode_source": bound.delivery_mode_source,
        "tier_policy": "agent-contract",
    });
    if let Some(tr) = &tracker.tracker_ref {
        data["tracker_ref"] = json!(tr);
    }
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(data),
        error: None,
        execution_report: Some(json!({
            "phases": [
                {"phase_name": "Recepcion", "status": "executed", "agent_tier": "reflexivo"},
                {
                    "phase_name": "Registro en tracker",
                    "status": registro_status,
                    "handler": PROCESS_NAME,
                    "note": tracker.skipped_reason,
                },
                {"phase_name": "Sellado", "status": "executed", "agent_tier": "balistico"}
            ]
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::core::repo::find_repo_root;
    use std::sync::{Mutex, OnceLock};

    fn lab_guard() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
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

    #[test]
    fn tipo_label_key_maps_bug_fix_and_kaizen() {
        assert_eq!(tipo_label_key(&json!({}), "bug-fix"), Some("fix"));
        assert_eq!(
            tipo_label_key(&json!({"type": "kaizen"}), "feature"),
            Some("kaizen")
        );
        assert_eq!(tipo_label_key(&json!({}), "feature"), None);
    }

    fn write_git_root(root: &Path) {
        fs::create_dir_all(root.join(".git")).unwrap();
    }

    fn index_md(uuid: &str, client: &Path, slug: &str) -> String {
        format!(
            "---\nid: {slug}\nuuid: \"{uuid}\"\nproject_root: \"{}\"\nmanifest_ref: .SddIA/project.md\ncodex_slug: codex-software-engineering\nstatus: active\n---\n\n# {slug}\n",
            client.display()
        )
    }

    fn manifest_md(uuid: &str, tracker: bool) -> String {
        let tracker_block = if tracker {
            "tracker:\n  provider: linear\n  team_key: OSC\n  state_map:\n    backlog: Backlog\n    done: Done\n  labels:\n    hu: hu\n    pbi: pbi\n    fix: fix\n    kaizen: kaizen\n"
        } else {
            ""
        };
        format!(
            "---\nid: lab\nuuid: \"{uuid}\"\ngit_remote: https://example.invalid/lab.git\ndefault_branch: main\ndelivery_mode: branch_pr\ncontract_version: \"1.2.0\"\ncodex_slug: codex-software-engineering\n{tracker_block}docs_layout:\n  features: docs/features\n  fixes: docs/fixes\n  todos_pending: docs/todos/pending\n  todos_done: docs/todos/done\n---\n\n# lab\n"
        )
    }

    fn register_lab_project(core: &Path, slug: &str, tracker: bool) -> PathBuf {
        let client = core.join(format!("_forge_lab_{slug}"));
        write_git_root(&client);
        fs::create_dir_all(client.join(".SddIA")).unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        fs::write(
            client.join(".SddIA/project.md"),
            manifest_md(uuid, tracker),
        )
        .unwrap();
        fs::create_dir_all(core.join(".SddIA/projects")).unwrap();
        fs::write(
            core.join(".SddIA/projects").join(format!("{slug}.md")),
            index_md(uuid, &client, slug),
        )
        .unwrap();
        client
    }

    fn cleanup_project_index(core: &Path, slug: &str) {
        let _ = fs::remove_file(core.join(".SddIA/projects").join(format!("{slug}.md")));
        let _ = fs::remove_dir_all(core.join(format!("_forge_lab_{slug}")));
    }

    #[test]
    fn ac_0b2_without_tracker_no_capsule() {
        let core = find_repo_root().expect("repo");
        let slug = "forge-ac-0b2";
        register_lab_project(&core, slug, false);
        let out = run(
            &core,
            &json!({
                "project_slug": slug,
                "raw_idea": "idea",
                "process": "feature",
            }),
        )
        .expect("run");
        assert_eq!(
            out.data
                .as_ref()
                .and_then(|d| d.get("create_issue_invoked"))
                .and_then(|v| v.as_bool()),
            Some(false)
        );
        cleanup_project_index(&core, slug);
    }

    #[test]
    fn ac_0b_creates_tracker_ref_with_parent() {
        with_lab(|| {
            let core = find_repo_root().expect("repo");
            let slug = "forge-ac-0b";
            let client = register_lab_project(&core, slug, true);
            let hu_path = client.join("docs/todos/historias/ac-forge-hu.md");
            fs::create_dir_all(hu_path.parent().unwrap()).unwrap();
            fs::write(
                &hu_path,
                "---\ntracker_ref: LAB-HU-1\n---\n\n# HU\n",
            )
            .unwrap();
            let out = run(
                &core,
                &json!({
                    "project_slug": slug,
                    "raw_idea": "idea lab",
                    "process": "bug-fix",
                    "parent_ref": "LAB-HU-1",
                }),
            )
            .expect("run");
            let tr = out
                .data
                .as_ref()
                .and_then(|d| d.get("tracker_ref"))
                .and_then(|v| v.as_str())
                .expect("tracker_ref");
            assert!(tr.starts_with("OSC-"));
            assert!(out.data.as_ref().and_then(|d| d.get("artifact_path")).is_none());
            let pending = client.join("docs/todos/pending");
            if pending.is_dir() {
                for ent in std::fs::read_dir(&pending).unwrap().flatten() {
                    let name = ent.file_name().to_string_lossy().to_string();
                    assert!(!name.starts_with("bug-fix-"), "{name}");
                }
            }
            cleanup_project_index(&core, slug);
        });
    }

    #[test]
    fn ac_0b2_hu_without_tracker_ref_still_forges() {
        with_lab(|| {
            let core = find_repo_root().expect("repo");
            let slug = "forge-ac-0b2-hu";
            let client = register_lab_project(&core, slug, true);
            let hu_path = client.join("docs/todos/historias/no-tracker.md");
            fs::create_dir_all(hu_path.parent().unwrap()).unwrap();
            fs::write(&hu_path, "---\nstatus: pending\n---\n\n# HU\n").unwrap();
            let out = run(
                &core,
                &json!({
                    "project_slug": slug,
                    "raw_idea": "idea",
                    "process": "feature",
                    "historia_ref": "docs/todos/historias/no-tracker.md",
                }),
            )
            .expect("run");
            assert!(out
                .data
                .as_ref()
                .and_then(|d| d.get("tracker_ref"))
                .is_none());
            cleanup_project_index(&core, slug);
        });
    }
}
