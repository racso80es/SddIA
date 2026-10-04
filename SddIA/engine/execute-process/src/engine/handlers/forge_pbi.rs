//! Sellado lab de `forge-pbi`: formato físico, registro Linear (HU-A 07), sin inventar negocio.

use super::super::capsules::invoke_tool_for_process;
use super::super::project_binding::{self, resolve_doc_path, TrackerConfig};
use super::super::route_domain_core::materialize_pending_domain_event;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::fs;
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

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("FORGE_INVALID: sin frontmatter".into());
    }
    let mut fm = String::new();
    for line in lines.by_ref() {
        if line == "---" {
            let body: String = lines.collect::<Vec<_>>().join("\n");
            return Ok((fm, body));
        }
        fm.push_str(line);
        fm.push('\n');
    }
    Err("FORGE_INVALID: frontmatter sin cierre".into())
}

fn parse_fm(text: &str) -> Result<YamlValue, String> {
    let (fm, _) = split_frontmatter(text)?;
    serde_yaml::from_str(&fm).map_err(|e| format!("FORGE_INVALID: yaml {e}"))
}

fn fm_str(fm: &YamlValue, key: &str) -> Option<String> {
    fm.get(key)
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

fn resolve_hu_path(bound: &project_binding::BoundProject, hu_ref: &str) -> Result<PathBuf, String> {
    let full = resolve_doc_path(&bound.project_root, hu_ref)?;
    let rel = full
        .strip_prefix(&bound.project_root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    if !rel.contains("historias") {
        return Err("FORGE_SCOPE: historia_ref debe residir bajo historias/".into());
    }
    if !full.is_file() {
        return Err(format!("FORGE_NOT_FOUND: {}", full.display()));
    }
    Ok(full)
}

fn hu_parent_tracker_ref(
    bound: &project_binding::BoundProject,
    historia_ref: &str,
) -> Result<Option<String>, String> {
    let path = resolve_hu_path(bound, historia_ref)?;
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let fm = parse_fm(&raw)?;
    Ok(fm_str(&fm, "tracker_ref"))
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

fn try_create_pbi_issue(
    repo: &Path,
    tc: &TrackerConfig,
    inputs: &Value,
    process: &str,
    title: &str,
    description: &str,
    parent_ref: &str,
) -> Result<String, String> {
    let labels = build_pbi_labels(repo, inputs, tc, process)?;
    let mut request = json!({
        "operation": "create_issue",
        "team_key": tc.team_key,
        "title": title,
        "description": description,
        "labels": labels,
        "parent_ref": parent_ref,
    });
    if let Some(pid) = &tc.project_id {
        request["project_id"] = json!(pid);
    }
    if let Some(state) = backlog_state_name(tc) {
        request["state_name"] = json!(state);
    }
    let tool_body = invoke_tool_for_process(
        repo,
        "linear-tracker-adapter",
        &json!({ "request": request }),
        Some(PROCESS_NAME),
    )?;
    tool_body
        .get("issue_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| "LINEAR_CREATE_ISSUE_EMPTY: sin issue_ref".into())
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
    let tracker_cfg = match project_binding::tracker_config_for_inputs(repo, inputs) {
        Ok(v) => v,
        Err(e) => {
            return TrackerPhase {
                tracker_ref: None,
                invoked: false,
                skipped_reason: Some(e),
            };
        }
    };
    if tracker_cfg.is_none() {
        return TrackerPhase {
            tracker_ref: None,
            invoked: false,
            skipped_reason: Some("sin tracker en manifiesto".into()),
        };
    }
    let tc = tracker_cfg.unwrap();
    let historia_ref = str_field(inputs, "historia_ref");
    let parent = match historia_ref {
        Some(href) => match hu_parent_tracker_ref(bound, &href) {
            Ok(Some(p)) => p,
            Ok(None) => {
                let _ = emit_create_issue_failed(
                    repo,
                    &href,
                    "LINEAR_PARENT_NOT_FOUND",
                    &bound.slug,
                );
                return TrackerPhase {
                    tracker_ref: None,
                    invoked: false,
                    skipped_reason: Some("HU sin tracker_ref".into()),
                };
            }
            Err(e) => {
                let _ = emit_create_issue_failed(repo, &href, &e, &bound.slug);
                return TrackerPhase {
                    tracker_ref: None,
                    invoked: false,
                    skipped_reason: Some(e),
                };
            }
        },
        None => {
            let _ = emit_create_issue_failed(
                repo,
                "local:missing-historia_ref",
                "LINEAR_PARENT_NOT_FOUND",
                &bound.slug,
            );
            return TrackerPhase {
                tracker_ref: None,
                invoked: false,
                skipped_reason: Some("historia_ref ausente".into()),
            };
        }
    };
    match try_create_pbi_issue(repo, &tc, inputs, process, title, description, &parent) {
        Ok(issue_ref) => TrackerPhase {
            tracker_ref: Some(issue_ref),
            invoked: true,
            skipped_reason: None,
        },
        Err(code) => {
            let err_code = if code.len() > 120 {
                code.chars().take(120).collect()
            } else {
                code.clone()
            };
            let _ = emit_create_issue_failed(repo, &parent, &err_code, &bound.slug);
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
    let pending_dir = resolve_doc_path(&bound.project_root, &bound.docs.todos_pending)?;
    fs::create_dir_all(&pending_dir).map_err(|e| e.to_string())?;
    let document_id = format!("PBI-{}", &Uuid::new_v4().to_string()[..8]);
    let entity_uuid = Uuid::new_v4().to_string();
    let created = Utc::now().format("%Y-%m-%d").to_string();
    let slug = format!(
        "{}-{}",
        process,
        entity_uuid.split('-').next().unwrap_or("pbi")
    );
    let title = document_id.clone();
    let tracker = tracker_registration_phase(
        repo,
        &bound,
        inputs,
        process,
        &title,
        raw_idea,
    );
    let mut fm_lines = format!(
        "document_id: {document_id}\nuuid: \"{entity_uuid}\"\nstatus: pending\nprocess: {process}\ncreated: \"{created}\"\nproject_slug: {slug_project}\ndelivery_mode: {mode}\ndelivery_mode_source: {source}",
        slug_project = bound.slug,
        mode = bound.delivery_mode.as_str(),
        source = bound.delivery_mode_source,
    );
    if let Some(href) = str_field(inputs, "historia_ref") {
        fm_lines.push_str(&format!("\nhistoria_ref: \"{href}\""));
    }
    if let Some(tr) = &tracker.tracker_ref {
        fm_lines.push_str(&format!("\ntracker_ref: \"{tr}\""));
    }
    let body = format!("---\n{fm_lines}\n---\n\n# {document_id}\n\n{raw_idea}\n");
    let path = pending_dir.join(format!("{slug}.md"));
    fs::write(&path, body).map_err(|e| e.to_string())?;
    let rel = path.to_string_lossy().replace('\\', "/");
    let event_rel = materialize_pending_domain_event(
        repo,
        "PBI_Forged",
        PROCESS_NAME,
        json!({
            "document_id": document_id,
            "project_slug": bound.slug,
            "process": process,
            "artifact_path": rel,
        }),
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
        "artifact_path": rel,
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
    use crate::core::repo::find_repo_root;
    use std::sync::{Mutex, OnceLock};

    fn lab_guard() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn with_lab<F: FnOnce()>(f: F) {
        let _g = lab_guard();
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        std::env::remove_var("LINEAR_API_TOKEN");
        f();
        std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
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
                    "historia_ref": "docs/todos/historias/ac-forge-hu.md",
                }),
            )
            .expect("run");
            assert_eq!(
                out.data
                    .as_ref()
                    .and_then(|d| d.get("tracker_ref"))
                    .and_then(|v| v.as_str()),
                Some("OSC-42")
            );
            let artifact = out
                .data
                .as_ref()
                .and_then(|d| d.get("artifact_path"))
                .and_then(|v| v.as_str())
                .expect("path");
            let text = fs::read_to_string(client.join(artifact)).unwrap();
            assert!(text.contains("tracker_ref: \"OSC-42\""));
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
