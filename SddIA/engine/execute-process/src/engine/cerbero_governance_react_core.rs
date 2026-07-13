//! Cerbero reacción RBAC (paridad `cerbero_governance_react_core.py`).

use super::fractal_bus::load_radamanto_config;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

fn write_json_atomic(path: &Path, data: &Value) -> Result<(), String> {
    super::eda_bus_topology::write_json_atomic(path, data)
}

fn revoked_path(repo: &Path) -> Result<std::path::PathBuf, String> {
    let cfg = load_radamanto_config(repo)?;
    let rel = cfg
        .get("revoked_entities")
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/cerbero/revoked_entities.json");
    Ok(repo.join(rel.trim_start_matches("./")))
}

fn load_revoked(repo: &Path) -> Value {
    let path = revoked_path(repo).ok();
    let Some(path) = path.filter(|p| p.is_file()) else {
        return json!({"revoked": {}, "permanent": {}});
    };
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .map(|mut data| {
            if !data.get("revoked").and_then(|v| v.as_object()).is_some() {
                data["revoked"] = json!({});
            }
            if !data.get("permanent").and_then(|v| v.as_object()).is_some() {
                data["permanent"] = json!({});
            }
            data
        })
        .unwrap_or(json!({"revoked": {}, "permanent": {}}))
}

fn save_revoked(repo: &Path, data: &Value) -> Result<(), String> {
    let path = revoked_path(repo)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_json_atomic(&path, data)
}

fn resolve_entity_id(payload: &Value) -> Option<String> {
    payload
        .get("entity_id")
        .or_else(|| payload.get("target_entity_id"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

pub fn react_to_domain_event(repo: &Path, event: &Value) -> Value {
    match react_to_domain_event_inner(repo, event) {
        Ok(v) => v,
        Err(e) => json!({"ok": false, "error": e}),
    }
}

fn react_to_domain_event_inner(repo: &Path, event: &Value) -> Result<Value, String> {
    let event_type = event
        .get("event_type")
        .and_then(|v| v.as_str())
        .ok_or("event_type requerido")?;
    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    if !payload.is_object() {
        return Err("payload invalido".into());
    }
    let entity_id = resolve_entity_id(&payload).ok_or("entity_id requerido")?;
    let mut data = load_revoked(repo);

    match event_type {
        "Domain_Entity_Degraded" => {
            if let Some(revoked) = data.get_mut("revoked").and_then(|v| v.as_object_mut()) {
                revoked.insert(
                    entity_id.clone(),
                    json!({
                        "since": event.get("timestamp"),
                        "reason": payload.get("reason"),
                        "entity_type": payload.get("entity_type"),
                    }),
                );
            }
            save_revoked(repo, &data)?;
            Ok(json!({"ok": true, "action": "revoked", "entity_id": entity_id}))
        }
        "Domain_Entity_Restored" => {
            if event.get("emitter_agent").and_then(|v| v.as_str()) != Some("radamanto") {
                return Err("Domain_Entity_Restored solo desde radamanto".into());
            }
            if let Some(revoked) = data.get_mut("revoked").and_then(|v| v.as_object_mut()) {
                revoked.remove(&entity_id);
            }
            save_revoked(repo, &data)?;
            Ok(json!({"ok": true, "action": "restored", "entity_id": entity_id}))
        }
        "Domain_Entity_Deprecated" => {
            if let Some(revoked) = data.get_mut("revoked").and_then(|v| v.as_object_mut()) {
                revoked.remove(&entity_id);
            }
            if let Some(permanent) = data.get_mut("permanent").and_then(|v| v.as_object_mut()) {
                permanent.insert(
                    entity_id.clone(),
                    json!({
                        "since": event.get("timestamp"),
                        "reason": payload.get("reason"),
                        "entity_type": payload.get("entity_type"),
                    }),
                );
            }
            save_revoked(repo, &data)?;
            Ok(json!({"ok": true, "action": "permanent_block", "entity_id": entity_id}))
        }
        other => Err(format!("event_type no soportado: {other}")),
    }
}
