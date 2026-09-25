//! Sellado lab de `forge-pbi`: formato físico, sin inventar negocio. Tiers en el process, no aquí.

use super::super::project_binding::{self, resolve_doc_path};
use super::super::route_domain_core::materialize_pending_domain_event;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

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
    let path = pending_dir.join(format!("{slug}.md"));
    let body = format!(
        "---\ndocument_id: {document_id}\nuuid: \"{entity_uuid}\"\nstatus: pending\nprocess: {process}\ncreated: \"{created}\"\nproject_slug: {slug_project}\ndelivery_mode: {mode}\ndelivery_mode_source: {source}\n---\n\n# {document_id}\n\n{raw_idea}\n",
        slug_project = bound.slug,
        mode = bound.delivery_mode.as_str(),
        source = bound.delivery_mode_source,
    );
    fs::write(&path, body).map_err(|e| e.to_string())?;
    let rel = path.to_string_lossy().replace('\\', "/");
    let event_rel = materialize_pending_domain_event(
        repo,
        "PBI_Forged",
        "forge-pbi",
        json!({
            "document_id": document_id,
            "project_slug": bound.slug,
            "process": process,
            "artifact_path": rel,
        }),
    )?;
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "artifact_path": rel,
            "document_id": document_id,
            "uuid": entity_uuid,
            "event_path": event_rel,
            "delivery_mode": bound.delivery_mode.as_str(),
            "delivery_mode_source": bound.delivery_mode_source,
            "tier_policy": "agent-contract",
        })),
        error: None,
        execution_report: Some(json!({
            "phases": [
                {"phase_name": "Recepcion", "status": "executed", "agent_tier": "reflexivo"},
                {"phase_name": "Sellado", "status": "executed", "agent_tier": "balistico"}
            ]
        })),
        exit_code: 0,
    })
}
