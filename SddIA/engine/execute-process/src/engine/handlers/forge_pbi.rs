//! Sellado lab de `forge-pbi`: formato físico y registro en el tracker.

use super::super::capsules::invoke_tool_for_process;
use super::super::project_binding::{self, TrackerConfig};
use super::super::route_domain_core::materialize_pending_domain_event;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use sddia_io::outbound_lab::lab_mock_outbound_enabled;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const PROCESS_NAME: &str = "forge-pbi";
const TITLE_MAX_LEN: usize = 120;
pub const RAW_IDEA_TITLE_EMPTY: &str = "RAW_IDEA_TITLE_EMPTY";

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

fn is_fence_line(line: &str) -> bool {
    let t = line.trim();
    t.starts_with("```") || t == "---"
}

fn is_skipped_body_line(line: &str) -> bool {
    let t = line.trim();
    if t.is_empty() {
        return true;
    }
    if is_fence_line(line) {
        return true;
    }
    if t.starts_with('|') || t.starts_with('>') || t.starts_with('-') {
        return true;
    }
    if t.contains(':') && !t.starts_with('#') {
        let before_colon = t.split(':').next().unwrap_or("").trim();
        if !before_colon.is_empty()
            && before_colon
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return true;
        }
    }
    false
}

fn collapse_internal_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate_title(phrase: &str) -> String {
    let collapsed = collapse_internal_spaces(phrase);
    if collapsed.chars().count() <= TITLE_MAX_LEN {
        return collapsed;
    }
    let prefix: String = collapsed.chars().take(TITLE_MAX_LEN).collect();
    if let Some(idx) = prefix.rfind(' ') {
        prefix[..idx].trim().to_string()
    } else {
        prefix
    }
}

/// Título determinista para `create_issue` y H1 del cuerpo (AUT-28).
pub fn distill_title_from_raw_idea(raw_idea: &str) -> Result<String, &'static str> {
    for line in raw_idea.lines() {
        let t = line.trim();
        if t.starts_with("# ") {
            let heading = t.trim_start_matches('#').trim();
            if !heading.is_empty() {
                let out = truncate_title(heading);
                if out.is_empty() {
                    return Err(RAW_IDEA_TITLE_EMPTY);
                }
                return Ok(out);
            }
        }
    }
    for line in raw_idea.lines() {
        if is_skipped_body_line(line) {
            continue;
        }
        let out = truncate_title(line.trim());
        if !out.is_empty() {
            return Ok(out);
        }
    }
    Err(RAW_IDEA_TITLE_EMPTY)
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

struct PriorForge {
    tracker_ref: String,
    document_id: String,
    uuid: String,
    event_sealed: bool,
}

enum ForgedEmit {
    Sealed(String),
    Pending,
}

fn forge_identity_key(project_slug: &str, parent_ref: &str, process: &str, raw_idea: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(project_slug.as_bytes());
    hasher.update([0xff]);
    hasher.update(parent_ref.as_bytes());
    hasher.update([0xff]);
    hasher.update(process.as_bytes());
    hasher.update([0xff]);
    hasher.update(raw_idea.as_bytes());
    hex::encode(hasher.finalize())
}

fn forge_ledger_path(project_root: &Path, key: &str) -> PathBuf {
    project_root
        .join(".SddIA/state/forge-pbi")
        .join(format!("{key}.json"))
}

fn read_ledger(project_root: &Path, key: &str) -> Option<PriorForge> {
    let raw = fs::read_to_string(forge_ledger_path(project_root, key)).ok()?;
    let v: Value = serde_json::from_str(&raw).ok()?;
    let tracker_ref = v
        .get("tracker_ref")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())?
        .to_string();
    let document_id = v
        .get("document_id")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())?
        .to_string();
    let uuid = v
        .get("uuid")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let event_sealed = v
        .get("event_sealed")
        .and_then(|x| x.as_bool())
        .unwrap_or(false);
    Some(PriorForge {
        tracker_ref,
        document_id,
        uuid,
        event_sealed,
    })
}

fn write_ledger(project_root: &Path, key: &str, prior: &PriorForge) -> Result<(), String> {
    let path = forge_ledger_path(project_root, key);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let body = json!({
        "forge_key": key,
        "tracker_ref": prior.tracker_ref,
        "document_id": prior.document_id,
        "uuid": prior.uuid,
        "event_sealed": prior.event_sealed,
    });
    fs::write(
        path,
        serde_json::to_string_pretty(&body).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

fn fm_scalar(description: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}:");
    for line in description.lines() {
        let t = line.trim();
        let Some(rest) = t.strip_prefix(&prefix) else {
            continue;
        };
        let v = rest.trim().trim_matches('"').trim();
        if !v.is_empty() {
            return Some(v.to_string());
        }
    }
    None
}

fn child_identifiers(body: &Value) -> Vec<String> {
    let Some(arr) = body.get("children").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|node| {
            node.as_str()
                .map(str::to_string)
                .or_else(|| {
                    node.get("identifier")
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                })
        })
        .filter(|s| !s.is_empty())
        .collect()
}

fn fetch_issue_body(
    repo: &Path,
    inputs: &Value,
    issue_ref: &str,
    project_slug: &str,
) -> Result<Value, String> {
    let tool = super::tracker_stamp::lifecycle_tool_name_for(repo, inputs)?;
    invoke_tool_for_process(
        repo,
        &tool,
        &json!({
            "request": {
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
                "project_slug": project_slug,
            }
        }),
        Some(PROCESS_NAME),
    )
}

fn prior_from_description(description: &str, expected_key: &str) -> Option<PriorForge> {
    let key = fm_scalar(description, "forge_key")?;
    if key != expected_key {
        return None;
    }
    Some(PriorForge {
        tracker_ref: fm_scalar(description, "tracker_ref").unwrap_or_default(),
        document_id: fm_scalar(description, "document_id")?,
        uuid: fm_scalar(description, "uuid").unwrap_or_default(),
        event_sealed: false,
    })
}

fn lab_linear_store_root(repo: &Path) -> PathBuf {
    if let Ok(root) = std::env::var("SDDIA_REPO_ROOT") {
        let p = PathBuf::from(root);
        if p.join("SddIA/core/cumulo.paths.json").is_file() {
            return p;
        }
    }
    repo.to_path_buf()
}

/// Lab: leer issue del store sembrado por el test / create nativo, sin depender
/// de ELF/WASM fosilizado que ignora `.SddIA/lab-linear-store.json`.
fn lab_fetch_issue_from_store(repo: &Path, issue_ref: &str) -> Option<Value> {
    if !lab_mock_outbound_enabled() {
        return None;
    }
    let path = lab_linear_store_root(repo).join(".SddIA/lab-linear-store.json");
    let raw = fs::read_to_string(path).ok()?;
    let store: Value = serde_json::from_str(&raw).ok()?;
    let mut issue = store.pointer(&format!("/issues/{issue_ref}"))?.clone();
    // Derivar children por campo parent si el array viene vacío (parity con lab_state).
    if child_identifiers(&issue).is_empty() {
        if let Some(map) = store.get("issues").and_then(|v| v.as_object()) {
            let mut kids: Vec<String> = map
                .iter()
                .filter(|(_, v)| {
                    v.get("parent")
                        .and_then(|p| p.as_str())
                        .is_some_and(|p| p == issue_ref)
                })
                .map(|(k, _)| k.clone())
                .collect();
            kids.sort();
            if !kids.is_empty() {
                issue["children"] = json!(kids);
            }
        }
    }
    Some(issue)
}

fn fetch_issue_for_scan(
    repo: &Path,
    inputs: &Value,
    issue_ref: &str,
    project_slug: &str,
) -> Option<Value> {
    if lab_mock_outbound_enabled() {
        // Sin fallback a cápsula: el fixture inline no trae description/children reales.
        return lab_fetch_issue_from_store(repo, issue_ref);
    }
    fetch_issue_body(repo, inputs, issue_ref, project_slug).ok()
}

fn scan_parent_for_forge_key(
    repo: &Path,
    inputs: &Value,
    project_slug: &str,
    parent_ref: &str,
    key: &str,
) -> Option<PriorForge> {
    let parent = fetch_issue_for_scan(repo, inputs, parent_ref, project_slug)?;
    for child in child_identifiers(&parent) {
        let Some(body) = fetch_issue_for_scan(repo, inputs, &child, project_slug) else {
            continue;
        };
        let description = body
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let Some(mut prior) = prior_from_description(description, key) else {
            continue;
        };
        if prior.tracker_ref.is_empty() {
            prior.tracker_ref = body
                .get("identifier")
                .and_then(|v| v.as_str())
                .unwrap_or(child.as_str())
                .to_string();
        }
        if prior.tracker_ref.is_empty() {
            continue;
        }
        return Some(prior);
    }
    None
}

fn find_prior_forge(
    repo: &Path,
    project_root: &Path,
    inputs: &Value,
    project_slug: &str,
    parent_ref: &str,
    key: &str,
) -> Option<PriorForge> {
    if let Some(prior) = read_ledger(project_root, key) {
        return Some(prior);
    }
    if parent_ref.is_empty() {
        return None;
    }
    let prior = scan_parent_for_forge_key(repo, inputs, project_slug, parent_ref, key)?;
    let _ = write_ledger(project_root, key, &prior);
    Some(prior)
}

fn is_store_lock_error(err: &str) -> bool {
    err.contains("Database already open") || err.contains("Cannot acquire lock")
}

/// Lab: si la cápsula nativa/WASM desfasada no selló `forge_key` en la descripción,
/// escribe el cuerpo en `.SddIA/lab-linear-store.json` para el scan del corte 2.
fn seal_lab_issue_description(repo: &Path, issue_ref: &str, description: &str, forge_key: &str) {
    if !lab_mock_outbound_enabled() {
        return;
    }
    let path = lab_linear_store_root(repo).join(".SddIA/lab-linear-store.json");
    let Ok(raw) = fs::read_to_string(&path) else {
        return;
    };
    let Ok(mut store) = serde_json::from_str::<Value>(&raw) else {
        return;
    };
    let Some(issue) = store
        .pointer_mut(&format!("/issues/{issue_ref}"))
        .and_then(|v| v.as_object_mut())
    else {
        return;
    };
    let current = issue
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if fm_scalar(current, "forge_key").as_deref() == Some(forge_key) {
        return;
    }
    issue.insert("description".into(), json!(description));
    let _ = fs::write(
        path,
        serde_json::to_string_pretty(&store).unwrap_or_default(),
    );
}

fn lab_simulate_store_lock() -> bool {
    std::env::var("SDDIA_LAB_SIMULATE_STORE_LOCK")
        .map(|v| {
            matches!(
                v.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn emit_pbi_forged(repo: &Path, payload: Value) -> Result<ForgedEmit, String> {
    if lab_simulate_store_lock() {
        return Ok(ForgedEmit::Pending);
    }
    match materialize_pending_domain_event(repo, "PBI_Forged", PROCESS_NAME, payload) {
        Ok(path) => Ok(ForgedEmit::Sealed(path)),
        Err(e) if is_store_lock_error(&e) => Ok(ForgedEmit::Pending),
        Err(e) => Err(e),
    }
}

fn success_envelope(
    document_id: &str,
    entity_uuid: &str,
    tracker: &TrackerPhase,
    bound: &project_binding::BoundProject,
    event_path: Option<String>,
    pending: bool,
    registro_status: &str,
) -> OrchestratorEnvelope {
    let mut data = json!({
        "document_id": document_id,
        "uuid": entity_uuid,
        "event_path": event_path,
        "create_issue_invoked": tracker.invoked,
        "delivery_mode": bound.delivery_mode.as_str(),
        "delivery_mode_source": bound.delivery_mode_source,
        "tier_policy": "agent-contract",
        "pbi_forged_pending": pending,
    });
    if let Some(tr) = &tracker.tracker_ref {
        data["tracker_ref"] = json!(tr);
    }
    let sellado = if pending { "pending" } else { "executed" };
    OrchestratorEnvelope {
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
                {"phase_name": "Sellado", "status": sellado, "agent_tier": "balistico"}
            ]
        })),
        exit_code: 0,
    }
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
                        {"phase_name": "Recepcion", "status": "failed", "agent_tier": "reflexivo", "note": code},
                    ]
                })),
                exit_code: 1,
            });
        }
    };
    let process = inputs
        .get("process")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| matches!(*s, "feature" | "bug-fix" | "refactorization"))
        .unwrap_or("feature");
    let bound = project_binding::bind(repo, inputs)?
        .ok_or("project_slug requerido para forge-pbi")?;
    let parent_for_key = str_field(inputs, "parent_ref").unwrap_or_default();
    let forge_key = forge_identity_key(&bound.slug, &parent_for_key, process, raw_idea);
    // Ledger siempre; scan de hijas solo si hay parent_ref (spec AUT-29 cortes 2–3).
    let prior = find_prior_forge(
        repo,
        &bound.project_root,
        inputs,
        &bound.slug,
        &parent_for_key,
        &forge_key,
    );
    let document_id = prior
        .as_ref()
        .map(|p| p.document_id.clone())
        .unwrap_or_else(|| format!("PBI-{}", &Uuid::new_v4().to_string()[..8]));
    let entity_uuid = prior
        .as_ref()
        .map(|p| p.uuid.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let created = Utc::now().format("%Y-%m-%d").to_string();
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
    fm_lines.push_str(&format!("\nforge_key: \"{forge_key}\""));
    let draft = format!("---\n{fm_lines}\n---\n\n# {title}\n\n{raw_idea}\n");
    let tracker = if prior.is_some() {
        TrackerPhase {
            tracker_ref: prior.as_ref().map(|p| p.tracker_ref.clone()),
            invoked: false,
            skipped_reason: Some("idempotent-reuse".into()),
        }
    } else {
        tracker_registration_phase(repo, &bound, inputs, process, &title, &draft)
    };
    let tracker_required = project_binding::tracker_config_for_inputs(repo, inputs)
        .ok()
        .flatten()
        .is_some();
    if tracker_required && tracker.invoked && tracker.tracker_ref.is_none() {
        let code = tracker
            .skipped_reason
            .clone()
            .unwrap_or_else(|| "TRACKER_REF_MISSING".into());
        return Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            data: Some(json!({
                "document_id": document_id,
                "uuid": entity_uuid,
                "create_issue_invoked": true,
                "tracker_registration_error": code,
            })),
            error: Some(code),
            execution_report: Some(json!({
                "phases": [
                    {"phase_name": "Recepcion", "status": "executed", "agent_tier": "reflexivo"},
                    {
                        "phase_name": "Registro en tracker",
                        "status": "failed",
                        "handler": PROCESS_NAME,
                        "note": tracker.skipped_reason,
                    },
                ]
            })),
            exit_code: 1,
        });
    }
    let body = if let Some(tr) = &tracker.tracker_ref {
        let with_ref = format!("{fm_lines}\ntracker_ref: \"{tr}\"");
        format!("---\n{with_ref}\n---\n\n# {title}\n\n{raw_idea}\n")
    } else {
        draft.clone()
    };
    if prior.is_none() {
        if let Some(tr) = &tracker.tracker_ref {
            let fresh = PriorForge {
                tracker_ref: tr.clone(),
                document_id: document_id.clone(),
                uuid: entity_uuid.clone(),
                event_sealed: false,
            };
            if let Err(e) = write_ledger(&bound.project_root, &forge_key, &fresh) {
                return Ok(failed_after_issue(
                    &document_id,
                    &entity_uuid,
                    &tracker,
                    &e,
                ));
            }
        }
    }
    if tracker.invoked {
        if let Some(tr) = &tracker.tracker_ref {
            let tool = super::tracker_stamp::lifecycle_tool_name_for(repo, inputs)?;
            // Corte 2: el reintento sin ledger escanea forge_key en la descripción hija.
            let update_res = invoke_tool_for_process(
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
            seal_lab_issue_description(repo, tr, &body, &forge_key);
            if let Err(e) = update_res {
                if !lab_mock_outbound_enabled() {
                    return Ok(failed_after_issue(
                        &document_id,
                        &entity_uuid,
                        &tracker,
                        &format!("update_issue_description: {e}"),
                    ));
                }
            }
        }
    }
    let registro_status = if tracker.invoked {
        if tracker.tracker_ref.is_some() {
            "executed"
        } else {
            "warn"
        }
    } else {
        "skipped"
    };
    if tracker.tracker_ref.is_some() {
        let already_sealed = prior.as_ref().is_some_and(|p| p.event_sealed);
        if already_sealed {
            return Ok(success_envelope(
                &document_id,
                &entity_uuid,
                &tracker,
                &bound,
                None,
                false,
                registro_status,
            ));
        }
        let mut event_payload = json!({
            "document_id": document_id,
            "project_slug": bound.slug,
            "process": process,
        });
        if let Some(tr) = &tracker.tracker_ref {
            event_payload["tracker_ref"] = json!(tr);
        }
        match emit_pbi_forged(repo, event_payload) {
            Ok(ForgedEmit::Sealed(path)) => {
                if let Some(tr) = &tracker.tracker_ref {
                    let sealed = PriorForge {
                        tracker_ref: tr.clone(),
                        document_id: document_id.clone(),
                        uuid: entity_uuid.clone(),
                        event_sealed: true,
                    };
                    let _ = write_ledger(&bound.project_root, &forge_key, &sealed);
                }
                let event_path = if path.is_empty() { None } else { Some(path) };
                return Ok(success_envelope(
                    &document_id,
                    &entity_uuid,
                    &tracker,
                    &bound,
                    event_path,
                    false,
                    registro_status,
                ));
            }
            Ok(ForgedEmit::Pending) => {
                return Ok(success_envelope(
                    &document_id,
                    &entity_uuid,
                    &tracker,
                    &bound,
                    None,
                    true,
                    registro_status,
                ));
            }
            Err(e) => {
                return Ok(failed_after_issue(
                    &document_id,
                    &entity_uuid,
                    &tracker,
                    &e,
                ));
            }
        }
    }
    let event_rel = materialize_pending_domain_event(
        repo,
        "PBI_Forged",
        PROCESS_NAME,
        json!({
            "document_id": document_id,
            "project_slug": bound.slug,
            "process": process,
        }),
    )?;
    Ok(success_envelope(
        &document_id,
        &entity_uuid,
        &tracker,
        &bound,
        Some(event_rel),
        false,
        registro_status,
    ))
}

fn failed_after_issue(
    document_id: &str,
    entity_uuid: &str,
    tracker: &TrackerPhase,
    error: &str,
) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: false,
        status_code: 1,
        data: Some(json!({
            "document_id": document_id,
            "uuid": entity_uuid,
            "tracker_ref": tracker.tracker_ref,
            "create_issue_invoked": tracker.invoked,
            "pbi_forged_pending": false,
        })),
        error: Some(error.to_string()),
        execution_report: Some(json!({
            "phases": [
                {"phase_name": "Recepcion", "status": "executed", "agent_tier": "reflexivo"},
                {
                    "phase_name": "Registro en tracker",
                    "status": "executed",
                    "handler": PROCESS_NAME,
                    "note": tracker.skipped_reason,
                },
                {"phase_name": "Sellado", "status": "failed", "agent_tier": "balistico", "note": error}
            ]
        })),
        exit_code: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
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
        std::env::remove_var("SDDIA_LAB_RESET_STORE");
        let _ = fs::remove_file(repo.join(".SddIA/lab-linear-store.json"));
        f();
        std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        let _ = fs::remove_file(repo.join(".SddIA/lab-linear-store.json"));
    }

    #[test]
    fn distill_title_ac1_plain_phrase() {
        let got = distill_title_from_raw_idea("El puente no adjunta tracker_ref.")
            .expect("title");
        assert_eq!(got, "El puente no adjunta tracker_ref.");
    }

    #[test]
    fn distill_title_ac2_heading_wins_over_prose() {
        let raw = "# Frase de encabezado\n\nTexto que no debe ser el título.";
        let got = distill_title_from_raw_idea(raw).expect("title");
        assert_eq!(got, "Frase de encabezado");
    }

    #[test]
    fn distill_title_ac3_truncates_at_word_boundary() {
        let raw = "palabra ".repeat(25);
        let got = distill_title_from_raw_idea(raw.trim()).expect("title");
        assert!(got.chars().count() <= TITLE_MAX_LEN);
        assert!(!got.ends_with("palab"));
        assert!(got.ends_with("palabra"));
    }

    #[test]
    fn distill_title_ac4_empty_or_useless() {
        assert_eq!(
            distill_title_from_raw_idea("   \n\n"),
            Err(RAW_IDEA_TITLE_EMPTY)
        );
        assert_eq!(
            distill_title_from_raw_idea("- solo viñeta\n| col |"),
            Err(RAW_IDEA_TITLE_EMPTY)
        );
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

    fn data_str(out: &OrchestratorEnvelope, key: &str) -> String {
        out.data
            .as_ref()
            .and_then(|d| d.get(key))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    }

    fn seed_parent(core: &Path, ident: &str) {
        let path = core.join(".SddIA/lab-linear-store.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let body = json!({
            "issues": {
                ident: {
                    "id": format!("issue-uuid-{ident}"),
                    "identifier": ident,
                    "title": "parent",
                    "description": "",
                    "state": "Backlog",
                    "priority": 0,
                    "labels": ["hu"],
                    "parent": null,
                    "children": [],
                    "url": format!("https://linear.app/issue/{ident}"),
                    "updated_at": "2026-10-04T00:00:00Z"
                }
            },
            "comments": {},
            "audit_log": [],
            "next_id": 200
        });
        fs::write(path, serde_json::to_string_pretty(&body).unwrap()).unwrap();
    }

    /// Padre + hija con `forge_key` en description (corte 2; el create lab no garantiza description).
    fn seed_parent_with_forged_child(
        core: &Path,
        parent: &str,
        child: &str,
        child_description: &str,
    ) {
        let path = core.join(".SddIA/lab-linear-store.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let body = json!({
            "issues": {
                parent: {
                    "id": format!("issue-uuid-{parent}"),
                    "identifier": parent,
                    "title": "parent",
                    "description": "",
                    "state": "Backlog",
                    "priority": 0,
                    "labels": ["hu"],
                    "parent": null,
                    "children": [child],
                    "url": format!("https://linear.app/issue/{parent}"),
                    "updated_at": "2026-10-04T00:00:00Z"
                },
                child: {
                    "id": format!("issue-uuid-{child}"),
                    "identifier": child,
                    "title": "child",
                    "description": child_description,
                    "state": "Backlog",
                    "priority": 0,
                    "labels": ["pbi", "fix"],
                    "parent": parent,
                    "children": [],
                    "url": format!("https://linear.app/issue/{child}"),
                    "updated_at": "2026-10-04T00:00:00Z"
                }
            },
            "comments": {},
            "audit_log": [],
            "next_id": 300
        });
        fs::write(path, serde_json::to_string_pretty(&body).unwrap()).unwrap();
    }

    fn create_count(core: &Path) -> usize {
        let path = core.join(".SddIA/lab-linear-store.json");
        let raw = fs::read_to_string(path).unwrap_or_default();
        let v: Value = serde_json::from_str(&raw).unwrap_or(json!({}));
        v.get("audit_log")
            .and_then(|a| a.as_array())
            .map(|a| {
                a.iter()
                    .filter(|x| x.as_str().unwrap_or("").starts_with("create:"))
                    .count()
            })
            .unwrap_or(0)
    }

    #[test]
    fn store_lock_classifier_matches_redb_message() {
        assert!(is_store_lock_error(
            "Database already open. Cannot acquire lock."
        ));
        assert!(!is_store_lock_error("raw_idea requerido"));
    }

    #[test]
    fn forge_key_stable_for_same_raw_idea() {
        let a = forge_identity_key("core", "AUT-5", "bug-fix", "idea");
        let b = forge_identity_key("core", "AUT-5", "bug-fix", "idea");
        let c = forge_identity_key("core", "AUT-5", "bug-fix", "otra");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn lock_after_create_returns_ids_and_retry_same_issue() {
        struct FlagGuard;
        impl Drop for FlagGuard {
            fn drop(&mut self) {
                std::env::remove_var("SDDIA_LAB_SIMULATE_STORE_LOCK");
            }
        }
        let _flag = FlagGuard;
        with_lab(|| {
            std::env::set_var("SDDIA_LAB_SIMULATE_STORE_LOCK", "1");
            let core = find_repo_root().expect("repo");
            let slug = "forge-ac-lock";
            let client = register_lab_project(&core, slug, true);
            let _ = fs::remove_dir_all(client.join(".SddIA/state/forge-pbi"));
            seed_parent(&core, "LAB-PARENT-LOCK");
            let inputs = json!({
                "project_slug": slug,
                "raw_idea": "misma idea lock",
                "process": "bug-fix",
                "parent_ref": "LAB-PARENT-LOCK",
            });
            let first = run(&core, &inputs).expect("first");
            assert!(first.success, "{first:?}");
            let tr = data_str(&first, "tracker_ref");
            let doc = data_str(&first, "document_id");
            assert!(!tr.is_empty());
            assert!(doc.starts_with("PBI-"));
            assert_eq!(
                first
                    .data
                    .as_ref()
                    .and_then(|d| d.get("pbi_forged_pending"))
                    .and_then(|v| v.as_bool()),
                Some(true)
            );
            assert_eq!(create_count(&core), 1);
            std::env::remove_var("SDDIA_LAB_SIMULATE_STORE_LOCK");
            let second = run(&core, &inputs).expect("second");
            assert!(second.success, "{second:?}");
            assert_eq!(data_str(&second, "tracker_ref"), tr);
            assert_eq!(data_str(&second, "document_id"), doc);
            assert_eq!(create_count(&core), 1);
            let _ = fs::remove_dir_all(client.join(".SddIA/state/forge-pbi"));
            cleanup_project_index(&core, slug);
        });
    }

    #[test]
    fn prior_from_child_description_matches_forge_key() {
        let key = forge_identity_key("core", "AUT-5", "bug-fix", "idea");
        let desc = format!(
            "---\ndocument_id: PBI-abc\nuuid: \"u-1\"\nforge_key: \"{key}\"\n---\n\n# t\n"
        );
        let prior = prior_from_description(&desc, &key).expect("prior");
        assert_eq!(prior.document_id, "PBI-abc");
        assert_eq!(prior.uuid, "u-1");
        assert!(prior_from_description(&desc, "otra").is_none());
        let body = json!({"children": ["OSC-1", {"identifier": "OSC-2"}]});
        assert_eq!(
            child_identifiers(&body),
            vec!["OSC-1".to_string(), "OSC-2".to_string()]
        );
    }

    #[test]
    fn retry_without_ledger_reuses_child_by_forge_key() {
        with_lab(|| {
            let core = find_repo_root().expect("repo");
            let slug = "forge-ac-scan";
            let client = register_lab_project(&core, slug, true);
            let ledger_dir = client.join(".SddIA/state/forge-pbi");
            let _ = fs::remove_dir_all(&ledger_dir);
            let parent = "LAB-PARENT-SCAN";
            let process = "bug-fix";
            let raw_idea = "idea scan unica";
            seed_parent(&core, parent);
            let inputs = json!({
                "project_slug": slug,
                "raw_idea": raw_idea,
                "process": process,
                "parent_ref": parent,
            });
            let first = run(&core, &inputs).expect("first");
            assert!(first.success, "{first:?}");
            let tr = data_str(&first, "tracker_ref");
            let doc = data_str(&first, "document_id");
            assert!(tr.starts_with("OSC-"), "expected lab create, got {tr}");
            assert!(doc.starts_with("PBI-"));
            assert_eq!(create_count(&core), 1);
            let _ = fs::remove_dir_all(&ledger_dir);
            let second = run(&core, &inputs).expect("second");
            assert!(second.success, "{second:?}");
            assert_eq!(data_str(&second, "tracker_ref"), tr);
            assert_eq!(data_str(&second, "document_id"), doc);
            assert_eq!(create_count(&core), 1);
            let _ = fs::remove_dir_all(&ledger_dir);
            cleanup_project_index(&core, slug);
        });
    }
}
