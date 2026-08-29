//! Utilidades bus fractal EDA (paridad subset `eda_bus_utils.py`).

use super::eda_bus_topology::{safe_remove_path, subscriber_id, write_json_atomic};
use super::workspace::load_paths_config;
use crate::core::parser::parse_frontmatter;
use chrono::Utc;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub const RADAMANTO_BATCH_SUBSCRIBER_KEY: &str = "radamanto.radamanto-batch";
pub const COMPLIANCE_SUBSCRIBER_KEY: &str = "argos.telemetry-compliance-audit";
const DEFAULT_TELEMETRY_SCHEMA: &[&str] = &["prompt_tokens", "completion_tokens"];

pub fn lab_route_sync() -> bool {
    matches!(
        std::env::var("SDDIA_LAB_ROUTE_SYNC")
            .unwrap_or_default()
            .trim()
            .to_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

pub fn load_fractal_subscription_rel(repo: &Path, key: &str) -> Result<String, String> {
    let cfg = load_paths_config(repo)?;
    let default = match key {
        "telemetry_subscriptions" => "SddIA/core/event-telemetry-subscriptions.json",
        "orchestration_subscriptions" => "SddIA/core/event-orchestration-subscriptions.json",
        "domain_subscriptions" => "SddIA/core/event-domain-subscriptions.json",
        other => return Err(format!("unknown fractal subscription key: {other}")),
    };
    let rel = cfg
        .get("eda_fractal")
        .and_then(|f| f.get(key))
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(default);
    Ok(rel.trim().replace('\\', "/"))
}

pub fn load_radamanto_config(repo: &Path) -> Result<HashMap<String, Value>, String> {
    let mut defaults: HashMap<String, Value> = HashMap::from([
        ("stats".into(), json!(".SddIA/radamanto/stats.json")),
        ("consumed".into(), json!(".SddIA/radamanto/consumed.json")),
        (
            "thresholds".into(),
            json!("SddIA/agents/radamanto.thresholds.json"),
        ),
        ("sandbox_root".into(), json!(".SddIA/sandbox/")),
        (
            "revoked_entities".into(),
            json!(".SddIA/cerbero/revoked_entities.json"),
        ),
        (
            "cognitive_inbox".into(),
            json!(".SddIA/radamanto/inbox"),
        ),
    ]);
    if let Ok(cfg) = load_paths_config(repo) {
        if let Some(block) = cfg.get("radamanto").and_then(|v| v.as_object()) {
            for (key, value) in block {
                if let Some(s) = value.as_str() {
                    if !s.trim().is_empty() {
                        defaults.insert(key.clone(), json!(s.trim().replace('\\', "/")));
                    }
                }
            }
        }
    }
    let thresh_rel = defaults
        .get("thresholds")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/agents/radamanto.thresholds.json");
    let thresh_path = repo.join(thresh_rel);
    let mut thresholds = json!({
        "success_rate_min": 0.85,
        "success_rate_min_by_entity_type": {},
        "batch_min_events": 10,
        "latency_ms_p95_threshold": 30000,
        "redemption_success_count": 3,
        "max_recovery_attempts": 3,
        "abrupt_drop_min_samples": 3,
    });
    if thresh_path.is_file() {
        if let Ok(text) = fs::read_to_string(&thresh_path) {
            if let Ok(loaded) = serde_json::from_str::<Value>(&text) {
                if let Some(obj) = loaded.as_object() {
                    if let Some(t) = thresholds.as_object_mut() {
                        for (k, v) in obj {
                            t.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }
    }
    defaults.insert("thresholds".into(), thresholds);
    Ok(defaults)
}

pub fn load_telemetry_compliance_config(repo: &Path) -> Result<HashMap<String, String>, String> {
    let mut defaults = HashMap::from([(
        "emitted_registry".to_string(),
        ".SddIA/telemetry-compliance/emitted.json".to_string(),
    )]);
    if let Ok(cfg) = load_paths_config(repo) {
        if let Some(block) = cfg.get("telemetry_compliance").and_then(|v| v.as_object()) {
            for (key, value) in block {
                if let Some(s) = value.as_str() {
                    if !s.trim().is_empty() {
                        defaults.insert(key.clone(), s.trim().replace('\\', "/"));
                    }
                }
            }
        }
    }
    Ok(defaults)
}

pub fn stamp_fractal_delivery_state(event_path: &Path, subscriber_key: &str, status: &str) {
    if !event_path.is_file() {
        return;
    }
    let Ok(text) = fs::read_to_string(event_path) else {
        return;
    };
    let Ok(mut body) = serde_json::from_str::<Value>(&text) else {
        return;
    };
    let ds = body
        .as_object_mut()
        .and_then(|o| {
            if !o.contains_key("delivery_state") {
                o.insert("delivery_state".into(), json!({}));
            }
            o.get_mut("delivery_state")
        })
        .and_then(|v| v.as_object_mut());
    if let Some(ds) = ds {
        ds.insert(subscriber_key.to_string(), json!(status));
        let _ = write_json_atomic(event_path, &body);
    }
}

pub fn delivery_stamp_terminal_ok(status: &str) -> bool {
    status == "success" || status == "skipped" || status.starts_with("skipped")
}

pub fn delivery_stamp_terminal(status: &str) -> bool {
    delivery_stamp_terminal_ok(status) || status == "failed"
}

/// Stamps no vacíos, todos terminales, al menos un `failed` (laudo C2 → DLQ).
pub fn fractal_delivery_terminal_with_failure(body: &Value) -> bool {
    let Some(ds) = body.get("delivery_state").and_then(|v| v.as_object()) else {
        return false;
    };
    if ds.is_empty() {
        return false;
    }
    let all_terminal = ds
        .values()
        .all(|v| v.as_str().map(delivery_stamp_terminal).unwrap_or(false));
    all_terminal && ds.values().any(|v| v.as_str() == Some("failed"))
}

pub fn load_fractal_dead_letter_dir(repo: &Path) -> PathBuf {
    let default = repo.join(".events/dead-letter");
    let Ok(cfg) = load_paths_config(repo) else {
        return default;
    };
    if let Some(dl) = cfg
        .get("eda_fractal")
        .and_then(|f| f.get("dead_letter"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
    {
        let trimmed = dl.trim().trim_start_matches("./");
        return repo.join(trimmed);
    }
    if let Some(dl) = cfg
        .get("eda_bus")
        .and_then(|b| b.get("dead_letter"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
    {
        let trimmed = dl.trim().trim_start_matches("./");
        return repo.join(trimmed);
    }
    default
}

/// Move físico a `eda_fractal.dead_letter` (crea directorio si falta).
pub fn move_fractal_event_to_dead_letter(repo: &Path, event_path: &Path) -> bool {
    if !event_path.is_file() {
        return false;
    }
    let dead_letter = load_fractal_dead_letter_dir(repo);
    if let Err(e) = fs::create_dir_all(&dead_letter) {
        eprintln!("[fractal-dlq] mkdir {}: {e}", dead_letter.display());
        return false;
    }
    let Some(name) = event_path.file_name() else {
        return false;
    };
    let dest = dead_letter.join(name);
    if dest.exists() {
        let _ = fs::remove_file(&dest);
    }
    match fs::rename(event_path, &dest) {
        Ok(()) => true,
        Err(e) => {
            eprintln!(
                "[fractal-dlq] rename {} → {}: {e}",
                event_path.display(),
                dest.display()
            );
            false
        }
    }
}

pub fn required_subscriber_ids(registry: &Value, event_type: &str) -> Vec<String> {
    registry
        .get(event_type)
        .and_then(|v| v.as_array())
        .map(|subs| {
            subs.iter()
                .filter_map(|sub| sub.as_object().map(|_| subscriber_id(sub)))
                .collect()
        })
        .unwrap_or_default()
}

pub fn maybe_purge_fractal_telemetry_when_terminal(
    repo: &Path,
    event_path: &Path,
    registry: &Value,
    event_type: &str,
) -> bool {
    let required = required_subscriber_ids(registry, event_type);
    if required.is_empty() || !event_path.is_file() {
        return false;
    }
    let Ok(text) = fs::read_to_string(event_path) else {
        return false;
    };
    let Ok(body) = serde_json::from_str::<Value>(&text) else {
        return false;
    };
    let Some(ds) = body.get("delivery_state").and_then(|v| v.as_object()) else {
        return false;
    };
    for sid in &required {
        let st = ds.get(sid).and_then(|v| v.as_str()).unwrap_or("");
        if !delivery_stamp_terminal_ok(st) {
            return false;
        }
    }
    let _ = repo;
    safe_remove_path(event_path)
}

pub fn resolve_ed_telemetry_contract(repo: &Path, capsule_id: Option<&str>) -> Value {
    let Some(cid) = capsule_id.map(str::trim).filter(|s| !s.is_empty()) else {
        return json!({"telemetry_provided": false, "telemetry_schema": null, "entity_kind": null});
    };
    for (kind, subdir) in [("skill", "skills"), ("action", "actions"), ("tool", "tools")] {
        let rel = format!("SddIA/{subdir}/{cid}.md");
        let path = repo.join(&rel);
        if !path.is_file() {
            continue;
        }
        let Ok(fm) = parse_frontmatter(&path) else {
            continue;
        };
        let provided = fm
            .get("telemetry_provided")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let schema_yaml = fm.get("telemetry_schema");
        let schema_val = if provided
            && schema_yaml.map(|s| s.is_null()).unwrap_or(true)
        {
            json!(DEFAULT_TELEMETRY_SCHEMA)
        } else {
            schema_yaml
                .map(|v| serde_json::to_value(v).unwrap_or(Value::Null))
                .unwrap_or(Value::Null)
        };
        return json!({
            "telemetry_provided": provided,
            "telemetry_schema": schema_val,
            "entity_kind": kind,
        });
    }
    json!({"telemetry_provided": false, "telemetry_schema": null, "entity_kind": null})
}

pub fn receipt_satisfies_schema(receipt: &Value, schema: &[String]) -> bool {
    let Some(obj) = receipt.as_object() else {
        return false;
    };
    for key in schema {
        match obj.get(key) {
            Some(v) if v.is_i64() || v.is_u64() || v.is_f64() => {
                let n = v.as_f64().unwrap_or(-1.0);
                if n < 0.0 {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

pub fn build_telemetry_compliance_breached_event(
    asset_id: &str,
    capsule_id: &str,
    process_name: &str,
    breach_reason: &str,
    expected_schema: Option<&[String]>,
) -> Value {
    let mut payload = json!({
        "asset_id": asset_id,
        "capsule_id": capsule_id,
        "breach_reason": breach_reason,
        "process_name": process_name,
    });
    if let Some(schema) = expected_schema {
        payload["expected_schema"] = json!(schema);
    }
    json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "Telemetry_Compliance_Breached",
        "event_family": "domain",
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": "telemetry-compliance-audit",
        "payload": payload,
        "delivery_state": {},
    })
}

pub fn radamanto_path(repo: &Path, cfg: &HashMap<String, Value>, key: &str) -> PathBuf {
    let rel = cfg
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("");
    repo.join(rel.trim_start_matches("./"))
}
