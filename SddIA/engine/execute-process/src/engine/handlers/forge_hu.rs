//! Alta de HU raíz vía tracker (AUT-50 / forge-hu).

use super::super::capsules::invoke_tool_for_process;
use super::super::project_binding::{self, TrackerConfig};
use super::super::route_domain_core::materialize_pending_domain_event;
use super::forge_pbi::distill_title_from_raw_idea;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::path::Path;
use uuid::Uuid;

const PROCESS_NAME: &str = "forge-hu";

fn str_field(inputs: &Value, key: &str) -> Option<String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn backlog_state_name(tc: &TrackerConfig) -> Option<String> {
    tc.state_map.get("backlog").cloned()
}

fn tracker_unconfigured(repo: &Path, inputs: &Value) -> bool {
    matches!(
        super::super::tracker_lifecycle_port::resolve_target(repo, inputs),
        Ok(super::super::tracker_lifecycle_port::LifecycleTarget::Unconfigured)
    )
}

pub fn run(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let raw_idea = inputs
        .get("raw_idea")
        .and_then(|v| v.as_str())
        .ok_or("raw_idea requerido")?;
    let title = match distill_title_from_raw_idea(raw_idea) {
        Ok(t) => t,
        Err(code) => {
            return Ok(OrchestratorEnvelope {
                success: false,
                status_code: 1,
                data: None,
                error: Some(code.to_string()),
                execution_report: Some(json!({
                    "phases": [
                        {"phase_name": "Recepcion", "status": "failed", "note": code},
                    ]
                })),
                exit_code: 1,
            });
        }
    };
    let bound = project_binding::bind(repo, inputs)?
        .ok_or("project_slug requerido para forge-hu")?;
    let created = Utc::now().format("%Y-%m-%d").to_string();
    let description = format!(
        "---\nprocess: feature\ntype: hu\ncreated: \"{created}\"\nproject_slug: {}\n---\n\n# {title}\n\n{raw_idea}\n",
        bound.slug
    );

    if tracker_unconfigured(repo, inputs) {
        return Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            data: None,
            error: Some("TRACKER_REQUIRED".into()),
            execution_report: Some(json!({
                "phases": [
                    {"phase_name": "Recepcion", "status": "executed"},
                    {
                        "phase_name": "Registro en tracker",
                        "status": "failed",
                        "note": "sin tracker en manifiesto",
                    },
                ]
            })),
            exit_code: 1,
        });
    }

    let tc = project_binding::tracker_config_for_inputs(repo, inputs)?
        .ok_or("tracker config ausente")?;
    let tool = super::tracker_stamp::lifecycle_tool_name_for(repo, inputs)?;
    let mut request = json!({
        "operation": "create_issue",
        "title": title,
        "description": description,
        "labels": [tc.label_hu.clone()],
        "project_slug": bound.slug,
        "team_key": tc.team_key,
    });
    if let Some(pid) = &tc.project_id {
        request["project_id"] = json!(pid);
    }
    if let Some(state) = backlog_state_name(&tc) {
        request["state_name"] = json!(state);
    }

    let body = invoke_tool_for_process(
        repo,
        &tool,
        &json!({ "request": request }),
        Some(PROCESS_NAME),
    )?;
    let tracker_ref = body
        .get("issue_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or("TRACKER_REF_MISSING")?;

    let event_id = Uuid::new_v4().to_string();
    let payload = json!({
        "event_id": event_id,
        "correlation_id": event_id,
        "source_process": PROCESS_NAME,
        "tracker_ref": tracker_ref,
        "project_slug": bound.slug,
        "occurred_at": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    });
    let event_path = materialize_pending_domain_event(repo, "HU_Forged", PROCESS_NAME, payload)?;

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "tracker_ref": tracker_ref,
            "event_path": event_path,
            "create_issue_invoked": true,
        })),
        error: None,
        execution_report: Some(json!({
            "phases": [
                {"phase_name": "Recepcion", "status": "executed"},
                {"phase_name": "Registro en tracker", "status": "executed", "handler": PROCESS_NAME},
                {"phase_name": "Sellado", "status": "executed"},
            ]
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use std::fs;
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

    fn register_lab_project(core: &Path, slug: &str, tracker: bool) -> std::path::PathBuf {
        let client = core.join(format!("_forge_hu_{slug}"));
        fs::create_dir_all(client.join(".git")).ok();
        fs::create_dir_all(client.join(".SddIA")).unwrap();
        let tracker_block = if tracker {
            "tracker:\n  provider: linear\n  team_key: OSC\n  state_map:\n    backlog: Backlog\n  labels:\n    hu: hu\n    pbi: pbi\n"
        } else {
            ""
        };
        fs::write(
            client.join(".SddIA/project.md"),
            format!(
                "---\nid: lab\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\ngit_remote: https://example.invalid/lab.git\ndefault_branch: main\ndelivery_mode: branch_pr\ncontract_version: \"1.3.0\"\ncodex_slug: codex-software-engineering\n{tracker_block}docs_layout:\n  features: docs/features\n  fixes: docs/fixes\n  todos_pending: docs/todos/pending\n  todos_done: docs/todos/done\n---\n\n# lab\n"
            ),
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
        let _ = fs::remove_dir_all(core.join(format!("_forge_hu_{slug}")));
    }

    #[test]
    fn aut50_ca5_forge_hu_returns_tracker_ref() {
        with_lab(|| {
            let core = find_repo_root().expect("repo");
            let slug = "forge-hu-ca5";
            register_lab_project(&core, slug, true);
            let out = run(
                &core,
                &json!({
                    "project_slug": slug,
                    "raw_idea": "HU raíz de prueba AUT-50",
                }),
            )
            .expect("run");
            assert!(out.success);
            let tr = out
                .data
                .as_ref()
                .and_then(|d| d.get("tracker_ref"))
                .and_then(|v| v.as_str())
                .expect("tracker_ref");
            assert!(tr.starts_with("OSC-"));
            cleanup(&core, slug);
        });
    }
}
