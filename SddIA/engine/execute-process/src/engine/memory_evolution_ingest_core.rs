//! Ingesta `Domain_Entity_Telemetry_Captured` → store evolution (mínimo durable).

use super::fractal_bus::stamp_fractal_delivery_state;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub const MEMORY_EVOLUTION_INGEST_SUBSCRIBER_KEY: &str = "cumulo.memory-evolution-ingest";
const STORE_REL: &str = ".SddIA/vector_store/evolution";

fn store_dir(repo: &Path) -> PathBuf {
    repo.join(STORE_REL)
}

fn sha256_hex(payload: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    hex_encode(hasher.finalize().as_slice())
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0xf) as usize] as char);
    }
    out
}

fn origin_stimulus_id(payload: &Value) -> Option<String> {
    payload
        .get("origin_stimulus")
        .and_then(|o| o.get("event_id"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn record_path_for_id(repo: &Path, id: &str) -> PathBuf {
    store_dir(repo).join(format!("{id}.json"))
}

/// Procesa un evento domain en ruta relativa; fail-soft hacia el caller vía `ok`.
pub fn ingest_domain_event_file(repo: &Path, rel_path: &str) -> Value {
    match ingest_inner(repo, rel_path) {
        Ok(v) => v,
        Err(e) => json!({"ok": false, "error": e}),
    }
}

fn ingest_inner(repo: &Path, rel_path: &str) -> Result<Value, String> {
    let event_path = repo.join(rel_path.trim());
    if !event_path.is_file() {
        return Err(format!("no existe: {rel_path}"));
    }
    let body: Value =
        serde_json::from_str(&fs::read_to_string(&event_path).map_err(|e| e.to_string())?)
            .map_err(|e| format!("JSON inválido: {e}"))?;

    let event_type = body
        .get("event_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if event_type != "Domain_Entity_Telemetry_Captured" {
        stamp_fractal_delivery_state(
            &event_path,
            MEMORY_EVOLUTION_INGEST_SUBSCRIBER_KEY,
            "skipped",
        );
        return Ok(json!({
            "ok": true,
            "skipped": "event_type_mismatch",
            "event_type": event_type,
        }));
    }

    let payload = body
        .get("payload")
        .cloned()
        .ok_or_else(|| "payload ausente".to_string())?;
    if !payload.is_object() {
        return Err("payload debe ser objeto".into());
    }

    let entity_id = payload
        .get("entity_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("entity_id requerido")?;
    let metrics = payload
        .get("execution_metrics")
        .cloned()
        .unwrap_or(json!({}));
    let success = metrics
        .get("success_status")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let polarity = if success {
        "EfficientSymmetry"
    } else {
        "StructuralFracture"
    };

    let payload_str = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let record_id = if let Some(origin_id) = origin_stimulus_id(&payload) {
        let h = sha256_hex(&origin_id);
        format!("stim-{}", &h[..16.min(h.len())])
    } else {
        sha256_hex(&payload_str)
    };

    let out_path = record_path_for_id(repo, &record_id);
    if out_path.is_file() {
        stamp_fractal_delivery_state(
            &event_path,
            MEMORY_EVOLUTION_INGEST_SUBSCRIBER_KEY,
            "skipped",
        );
        return Ok(json!({
            "ok": true,
            "skipped": "already_indexed",
            "record_id": record_id,
            "store_path": format!("{STORE_REL}/{record_id}.json"),
        }));
    }

    let record = json!({
        "id": record_id,
        "polarity": polarity,
        "payload": payload_str,
        "operational_metadata": {
            "success": success,
            "entity_id": entity_id,
            "entity_type": payload.get("entity_type"),
            "origin_stimulus": payload.get("origin_stimulus"),
            "duration_ms": metrics.get("duration_ms"),
            "exit_code": metrics.get("exit_code"),
            "source_event_id": body.get("event_id"),
        },
        "embedding": null,
    });

    fs::create_dir_all(store_dir(repo)).map_err(|e| e.to_string())?;
    super::eda_bus_topology::write_json_atomic(&out_path, &record)?;

    stamp_fractal_delivery_state(
        &event_path,
        MEMORY_EVOLUTION_INGEST_SUBSCRIBER_KEY,
        "success",
    );

    Ok(json!({
        "ok": true,
        "record_id": record_id,
        "entity_id": entity_id,
        "polarity": polarity,
        "store_path": format!("{STORE_REL}/{record_id}.json"),
        "purged": false,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn ingests_telemetry_captured_to_vector_store() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        let event_rel = ".events/domain/test-telem.json";
        let event_path = repo.join(event_rel);
        fs::create_dir_all(event_path.parent().unwrap()).unwrap();
        let event = json!({
            "event_id": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
            "event_type": "Domain_Entity_Telemetry_Captured",
            "payload": {
                "entity_type": "process",
                "entity_id": "feature",
                "execution_metrics": {
                    "duration_ms": 42,
                    "exit_code": 0,
                    "success_status": true
                },
                "origin_stimulus": {
                    "event_type": "Raw_Execution_Finished",
                    "event_id": "11111111-2222-3333-4444-555555555555"
                }
            },
            "delivery_state": {}
        });
        fs::write(&event_path, serde_json::to_string_pretty(&event).unwrap()).unwrap();

        let result = ingest_domain_event_file(repo, event_rel);
        assert_eq!(result["ok"], json!(true), "{result}");
        assert!(result.get("record_id").and_then(|v| v.as_str()).is_some());
        let store = repo.join(result["store_path"].as_str().unwrap());
        assert!(store.is_file());

        let again = ingest_domain_event_file(repo, event_rel);
        assert_eq!(again["skipped"], json!("already_indexed"));
    }
}
