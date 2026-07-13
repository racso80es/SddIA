//! Stub consumo batch telemetría (paridad `python_core::run_telemetry_batch_stub`).

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub fn run_telemetry_batch_stub(repo: &Path, event_rel: &str) -> Result<Value, String> {
    let event_path = repo.join(event_rel.trim());
    if !event_path.is_file() {
        return Err(format!("no existe: {event_rel}"));
    }
    let text = fs::read_to_string(&event_path).map_err(|e| e.to_string())?;
    let body: Value = serde_json::from_str(&text).map_err(|e| format!("JSON inválido: {e}"))?;
    fs::remove_file(&event_path).ok();
    Ok(json!({
        "ok": true,
        "event_id": body.get("event_id"),
        "event_type": body.get("event_type"),
        "purged": true,
    }))
}
