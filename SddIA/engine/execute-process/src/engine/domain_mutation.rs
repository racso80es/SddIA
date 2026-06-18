//! Handler nativo `emit-domain-mutation` (Sello Universal EDA, paridad `execute-action.py`).

use super::capsules::invoke_capsule_json;
use super::ecst_validation::validate_domain_mutation_event;
use super::eda_bus::find_existing_domain_event;
use super::eda_coverage::{remove_entity_coverage, upsert_entity_coverage};
use super::workspace::load_paths_config;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

const ENTITY_CLASSES: &[&str] = &[
    "process", "agent", "skill", "tool", "action", "norm", "codex", "event",
];
const LIFECYCLE_OPS: &[&str] = &["create", "update", "delete"];

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn optional_str(v: &Value, key: &str) -> Option<String> {
    match v.get(key) {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s.trim().is_empty() => None,
        Some(Value::String(s)) => Some(s.trim().to_string()),
        _ => None,
    }
}

fn is_sha256_hash(v: &Value) -> bool {
    v.as_str()
        .map(|s| s.starts_with("sha256:") && s.len() > 7)
        .unwrap_or(false)
}

fn is_uuid_v4(s: &str) -> bool {
    Uuid::parse_str(s).is_ok()
}

fn lifecycle_to_event_type(op: &str) -> Option<&'static str> {
    match op {
        "create" => Some("Domain_Entity_Created"),
        "update" => Some("Domain_Entity_Updated"),
        "delete" => Some("Domain_Entity_Deleted"),
        _ => None,
    }
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

fn validate_inputs(inputs: &Value) -> Result<(), String> {
    let entity_class = str_field(inputs, "entity_class")
        .ok_or("entity_class es obligatorio (string)")?;
    if !ENTITY_CLASSES.contains(&entity_class.as_str()) {
        return Err(format!("entity_class no soportada: {entity_class}"));
    }

    let entity_name = str_field(inputs, "entity_name")
        .ok_or("entity_name es obligatorio (string)")?;

    let lifecycle = str_field(inputs, "lifecycle_operation")
        .ok_or("lifecycle_operation es obligatorio (string)")?;
    if !LIFECYCLE_OPS.contains(&lifecycle.as_str()) {
        return Err(format!("lifecycle_operation no soportada: {lifecycle}"));
    }

    if lifecycle != "delete" {
        str_field(inputs, "entity_uuid").ok_or("entity_uuid obligatorio salvo delete")?;
    }

    if let Some(summary) = inputs.get("changes_summary").and_then(|v| v.as_str()) {
        if summary.chars().count() > 2048 {
            return Err("changes_summary excede 2048 caracteres".into());
        }
    }

    if let Some(corr) = optional_str(inputs, "correlation_id") {
        if !is_uuid_v4(&corr) {
            return Err("correlation_id debe ser UUID v4".into());
        }
    }

    let origin = optional_str(inputs, "origin_topology").unwrap_or_else(|| "core".into());
    if origin != "core" && origin != "local" {
        return Err("origin_topology debe ser core o local".into());
    }

    let hash_new = inputs.get("hash_signature_new");
    let hash_old = inputs.get("hash_signature_old");

    match lifecycle.as_str() {
        "create" => {
            if !hash_old.map(|v| v.is_null()).unwrap_or(true) {
                return Err("create: hash_signature_old debe ser null".into());
            }
            if !is_sha256_hash(hash_new.unwrap_or(&Value::Null)) {
                return Err("create: hash_signature_new (sha256:) es obligatorio".into());
            }
        }
        "update" => {
            if !is_sha256_hash(hash_old.unwrap_or(&Value::Null)) {
                return Err("update: hash_signature_old (sha256:) es obligatorio".into());
            }
            if !is_sha256_hash(hash_new.unwrap_or(&Value::Null)) {
                return Err("update: hash_signature_new (sha256:) es obligatorio".into());
            }
        }
        "delete" => {
            if !hash_new.map(|v| v.is_null()).unwrap_or(true) {
                return Err("delete: hash_signature_new debe ser null".into());
            }
            if !is_sha256_hash(hash_old.unwrap_or(&Value::Null)) {
                return Err("delete: hash_signature_old (sha256:) es obligatorio".into());
            }
        }
        _ => {}
    }

    let _ = entity_name;
    Ok(())
}

/// Ejecuta `emit-domain-mutation` nativamente (paridad `execute-action.py::_run_emit_domain_mutation`).
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    validate_inputs(inputs)?;

    let entity_class = str_field(inputs, "entity_class").unwrap();
    let entity_name = str_field(inputs, "entity_name").unwrap();
    let lifecycle = str_field(inputs, "lifecycle_operation").unwrap();
    let entity_uuid = optional_str(inputs, "entity_uuid");

    let event_type = lifecycle_to_event_type(&lifecycle)
        .ok_or_else(|| format!("lifecycle_operation no soportada: {lifecycle}"))?;

    let mut origin_topology = optional_str(inputs, "origin_topology").unwrap_or_else(|| "core".into());
    if origin_topology != "core" && origin_topology != "local" {
        origin_topology = "core".into();
    }

    if let Some(ref uuid) = entity_uuid {
        if lifecycle != "delete" {
            if let Some(existing) = find_existing_domain_event(repo, uuid, &lifecycle, event_type)? {
                if existing.get("event_id").and_then(|v| v.as_str()).is_some() {
                    if let Some(hash_sig) = inputs.get("hash_signature_new") {
                        if is_sha256_hash(hash_sig) {
                            let _ = upsert_entity_coverage(
                                repo,
                                uuid,
                                event_type,
                                hash_sig.as_str().unwrap(),
                            );
                        }
                    }
                    return Ok(json!({
                        "success": true,
                        "idempotent": true,
                        "event_type": event_type,
                        "event_id": existing.get("event_id"),
                        "target_path": existing.get("target_path"),
                    }));
                }
            }
        }
    }

    if lifecycle == "delete" {
        if let Some(ref uuid) = entity_uuid {
            let _ = remove_entity_coverage(repo, uuid);
        }
    }

    let event_id = generate_uuid(repo)?;
    let changes_summary = inputs
        .get("changes_summary")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| format!("{lifecycle} {entity_class} {entity_name}"));

    let payload = json!({
        "entity_class": entity_class,
        "entity_type": entity_class,
        "entity_id": entity_uuid,
        "lifecycle_operation": lifecycle,
        "entity_uuid": entity_uuid,
        "entity_name": entity_name,
        "version": inputs.get("version"),
        "hash_signature_new": inputs.get("hash_signature_new"),
        "hash_signature_old": inputs.get("hash_signature_old"),
        "origin_topology": origin_topology,
        "changes_summary": changes_summary,
    });

    let mut event = json!({
        "event_id": event_id,
        "event_type": event_type,
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": inputs.get("emitter_agent").unwrap_or(&json!("entity-manager")),
        "payload": payload,
        "delivery_state": {},
    });

    if let Some(corr) = optional_str(inputs, "correlation_id") {
        event["correlation_id"] = json!(corr);
    }

    validate_domain_mutation_event(repo, &event)?;

    if lifecycle != "delete" {
        if let Some(ref uuid) = entity_uuid {
            let hash_new = inputs
                .get("hash_signature_new")
                .and_then(|v| v.as_str())
                .ok_or("hash_signature_new (sha256:) obligatorio para upsert SSOT")?;
            if !hash_new.starts_with("sha256:") {
                return Err("hash_signature_new (sha256:) obligatorio para upsert SSOT".into());
            }
            upsert_entity_coverage(repo, uuid, event_type, hash_new)?;
        }
    }

    let seal = write_pending_event(repo, &event)?;
    Ok(json!({
        "success": true,
        "event_type": event_type,
        "event_id": seal.get("event_id"),
        "target_path": seal.get("target_path"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use std::fs;
    use uuid::Uuid;

    #[test]
    fn emit_domain_mutation_create_writes_pending() {
        let repo = find_repo_root().unwrap();
        let entity_uuid = Uuid::new_v4().to_string();
        let inputs = json!({
            "entity_class": "tool",
            "entity_name": format!("lab-smoke-{}", &entity_uuid[..8]),
            "lifecycle_operation": "create",
            "entity_uuid": entity_uuid,
            "version": "1.0.0",
            "hash_signature_new": "sha256:deadbeef",
            "hash_signature_old": null,
            "changes_summary": "lab create smoke",
            "emitter_agent": "execute-process-test",
            "origin_topology": "core"
        });
        let result = run(&repo, &inputs).expect("emit");
        assert_eq!(result.get("success"), Some(&json!(true)));
        let target = result.get("target_path").and_then(|v| v.as_str()).unwrap();
        let path = repo.join(target);
        assert!(path.is_file(), "pending file missing: {target}");
        let body: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            body.get("event_type").and_then(|v| v.as_str()),
            Some("Domain_Entity_Created")
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn emit_domain_mutation_rejects_invalid_lifecycle_hash() {
        let repo = find_repo_root().unwrap();
        let inputs = json!({
            "entity_class": "tool",
            "entity_name": "bad",
            "lifecycle_operation": "create",
            "entity_uuid": Uuid::new_v4().to_string(),
            "hash_signature_new": null,
            "hash_signature_old": null,
            "changes_summary": "bad",
            "emitter_agent": "test"
        });
        assert!(run(&repo, &inputs).is_err());
    }
}
