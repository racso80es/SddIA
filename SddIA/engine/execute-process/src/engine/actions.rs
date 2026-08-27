//! Handlers nativos de acciones EDA (D-P5.1) — sin `execute-action.py` para emit-* registradas.

use super::capsules::invoke_capsule_json;
use super::workspace::load_paths_config;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn generate_uuid(repo: &Path) -> Result<String, String> {
    let payload = json!({"operation": "GENERATE_UUID", "target_payload": null});
    if let Ok(result) = invoke_capsule_json(repo, "cryptography-manager", &payload, true) {
        if result.exit_code == 0 {
            if let Some(inner) = result.body.get("result") {
                if let Some(id) = inner.as_str() {
                    return Ok(id.to_string());
                }
                if let Some(id) = inner.get("result").and_then(|v| v.as_str()) {
                    return Ok(id.to_string());
                }
            }
        }
    }
    Ok(Uuid::new_v4().to_string())
}

fn write_pending_event(repo: &Path, event: &Value) -> Result<Value, String> {
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .ok_or("event_id required")?;
    let cfg = load_paths_config(repo)?;
    let pending_rel = cfg
        .get("eda_bus")
        .and_then(|b| b.get("pending"))
        .and_then(|v| v.as_str())
        .unwrap_or("./.events/pending");
    let pending = if pending_rel.starts_with("./") {
        repo.join(pending_rel.trim_start_matches("./"))
    } else {
        repo.join(pending_rel)
    };
    fs::create_dir_all(&pending).map_err(|e| e.to_string())?;
    let target = pending.join(format!("{event_id}.json"));
    let text = serde_json::to_string_pretty(event).map_err(|e| e.to_string())?;
    fs::write(&target, format!("{text}\n")).map_err(|e| e.to_string())?;
    Ok(json!({
        "event_id": event_id,
        "target_path": target
            .strip_prefix(repo)
            .unwrap_or(&target)
            .to_string_lossy()
            .replace('\\', "/"),
    }))
}

fn iso_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn emit_pr_presented(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let branch = str_field(inputs, "branch").ok_or("branch es obligatorio (string)")?;
    let status = inputs
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("presented");
    // Paridad github-bridge: aduana RBAC_SIGNER_PRESENT exige firmante no nulo.
    let signer = str_field(inputs, "signer_identity_rbac")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Vertice_Biologico_Relay".into());
    let event_id = generate_uuid(repo)?;
    let mut payload = json!({
        "branch": branch,
        "status": status,
        "signer_identity_rbac": signer,
    });
    if let Some(url) = str_field(inputs, "pr_url") {
        payload["pr_url"] = json!(url);
    }
    let mut event = json!({
        "event_id": event_id,
        "event_type": "PullRequest_Presented",
        "timestamp": iso_timestamp(),
        "emitter_agent": inputs.get("emitter_agent").unwrap_or(&json!("delivery-close-cycle")),
        "payload": payload,
        "delivery_state": {},
    });
    if let Some(corr) = str_field(inputs, "correlation_id") {
        event["correlation_id"] = json!(corr);
    }
    let seal = write_pending_event(repo, &event)?;
    Ok(json!({
        "success": true,
        "event_id": seal.get("event_id"),
        "target_path": seal.get("target_path"),
        "event_type": "PullRequest_Presented",
    }))
}

fn emit_pr_merged(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let merge_hash = str_field(inputs, "merge_commit_hash")
        .or_else(|| str_field(inputs, "hash_signature"))
        .ok_or("merge_commit_hash o hash_signature es obligatorio")?;
    let source_branch = str_field(inputs, "source_branch").unwrap_or_else(|| {
        str_field(inputs, "pr_url")
            .and_then(|u| u.rsplit('/').next().map(str::to_string))
            .filter(|_| inputs.get("pr_url").and_then(|v| v.as_str()).unwrap_or("").contains("feature/"))
            .unwrap_or_else(|| "feature/eda-bus-v1".into())
    });
    let correlation_id = match str_field(inputs, "correlation_id") {
        Some(c) => c,
        None => generate_uuid(repo)?,
    };
    let event_id = generate_uuid(repo)?;
    let mut payload = json!({
        "source_branch": source_branch,
        "target_branch": "main",
        "merge_commit_hash": merge_hash,
        "author": inputs.get("author").unwrap_or(&json!("integration-operator")),
        "security_clearance": {
            "auditor": "Argos",
            "audit_event_reference": inputs.get("audit_event_reference").unwrap_or(&json!(correlation_id)),
            "policy_applied": "pr-acceptance-protocol",
        },
    });
    for key in ["pr_url", "repository_name", "hash_signature", "traceability_anomaly", "traceability_note"] {
        if let Some(v) = inputs.get(key) {
            if !v.is_null() {
                payload[key] = v.clone();
            }
        }
    }
    let event = json!({
        "event_id": event_id,
        "event_type": "PullRequest_Merged",
        "timestamp": iso_timestamp(),
        "emitter_agent": inputs.get("emitter_agent").unwrap_or(&json!("emit-pr-merged-event")),
        "correlation_id": correlation_id,
        "payload": payload,
        "delivery_state": {},
    });
    let seal = write_pending_event(repo, &event)?;
    Ok(json!({
        "success": true,
        "event_id": seal.get("event_id"),
        "target_path": seal.get("target_path"),
        "event_type": "PullRequest_Merged",
        "merge_commit_hash": merge_hash,
    }))
}

fn emit_pr_audited(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let target_entity_id = str_field(inputs, "target_entity_id")
        .ok_or("target_entity_id es obligatorio (string)")?;
    let resolution = str_field(inputs, "resolution").ok_or("resolution es obligatorio")?;
    if !matches!(resolution.as_str(), "PASS" | "REJECT" | "FLAG") {
        return Err("resolution debe ser PASS, REJECT o FLAG".into());
    }
    let audit_event_reference = str_field(inputs, "audit_event_reference")
        .or_else(|| str_field(inputs, "correlation_id"))
        .unwrap_or_else(|| generate_uuid(repo).unwrap_or_default());
    let event_id = generate_uuid(repo)?;
    let violated_rules = inputs
        .get("violated_rules")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let payload = json!({
        "audit_event_reference": audit_event_reference,
        "target_entity_id": target_entity_id,
        "resolution": resolution,
        "violated_rules": violated_rules,
    });
    let mut event = json!({
        "event_id": event_id,
        "event_type": "PullRequest_Audited",
        "timestamp": iso_timestamp(),
        "emitter_agent": inputs.get("emitter_agent").unwrap_or(&json!("argos")),
        "payload": payload,
        "delivery_state": {},
    });
    if let Some(corr) = str_field(inputs, "correlation_id") {
        event["correlation_id"] = json!(corr);
    } else if !audit_event_reference.is_empty() {
        event["correlation_id"] = json!(audit_event_reference);
    }
    let seal = write_pending_event(repo, &event)?;
    Ok(json!({
        "success": true,
        "event_id": seal.get("event_id"),
        "target_path": seal.get("target_path"),
        "event_type": "PullRequest_Audited",
        "audit_event_reference": audit_event_reference,
    }))
}

/// Ejecuta handler nativo si la acción está registrada. `None` = delegar a cápsula/bridge.
pub fn try_run_native(repo: &Path, action_name: &str, inputs: &Value) -> Result<Option<Value>, String> {
    let data = match action_name {
        "emit-pr-presented-event" => emit_pr_presented(repo, inputs)?,
        "emit-pr-merged-event" => emit_pr_merged(repo, inputs)?,
        "emit-pr-audited-event" => emit_pr_audited(repo, inputs)?,
        "emit-domain-mutation" => super::domain_mutation::run(repo, inputs)?,
        "crypto-broker" => super::crypto_broker::run(repo, inputs)?,
        "emit-suite-execution-requested" => super::suite_execution_requested::run(repo, inputs)?,
        "emit-user-preference-change-requested" => {
            super::user_preference_change_requested::run(repo, inputs)?
        }
        "policy-validator" => super::policy_validator::run(repo, inputs)?,
        "sync-entity-index" => super::sync_entity_index::run(repo, inputs)?,
        "materialize-fracture-pbi" => super::materialize_fracture_pbi::run(repo, inputs)?,
        "materialize-kaizen-alert-doc" => super::materialize_kaizen_alert_doc::run(repo, inputs)?,
        "enrich-fracture-pbi-kaizen" => super::enrich_fracture_pbi_kaizen::run(repo, inputs)?,
        "persist-pec-correlation-proof" => super::persist_pec_correlation_proof::run(repo, inputs)?,
        _ => return Ok(None),
    };
    Ok(Some(data))
}
