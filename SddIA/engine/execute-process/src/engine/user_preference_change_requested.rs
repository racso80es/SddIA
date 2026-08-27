//! Handler nativo `emit-user-preference-change-requested`.

use super::crypto_broker;
use super::eda_bus::write_fractal_event;
use chrono::Utc;
use serde_json::{json, Value};
use std::path::Path;
use uuid::Uuid;

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn mint_event_id(repo: &Path) -> Result<String, String> {
    let out = crypto_broker::run(
        repo,
        &json!({"operation": "GENERATE_UUID", "target_payload": null}),
    )?;
    if let Some(s) = out.get("crypto_response").and_then(|v| v.as_str()) {
        return Ok(s.to_string());
    }
    if let Some(nested) = out
        .get("crypto_response")
        .and_then(|v| v.get("result"))
        .and_then(|v| v.as_str())
    {
        return Ok(nested.to_string());
    }
    Ok(Uuid::new_v4().to_string())
}

pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let operation = str_field(inputs, "operation").ok_or("operation requerido")?;
    let channel = str_field(inputs, "channel").ok_or("channel requerido")?;
    let mut payload = inputs.get("payload").cloned().unwrap_or(json!({}));
    if let Some(obj) = payload.as_object_mut() {
        obj.entry("operation".to_string())
            .or_insert(json!(operation));
        obj.entry("channel".to_string()).or_insert(json!(channel));
    }

    let event_id = mint_event_id(repo)?;
    let event = json!({
        "event_id": event_id,
        "event_type": "User_Preference_Change_Requested",
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": "emit-user-preference-change-requested",
        "payload": payload,
    });

    let seal = write_fractal_event(repo, &event, "domain")?;
    Ok(json!({
        "success": true,
        "exitCode": 0,
        "event_id": event_id,
        "target_path": seal.get("target_path"),
    }))
}
