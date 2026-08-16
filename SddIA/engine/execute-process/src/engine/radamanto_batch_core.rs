//! Radamanto batch — acumulador telemetría (paridad `radamanto_batch_core.py`).

use super::eda_bus::write_fractal_event;
use super::fractal_bus::{
    lab_route_sync, load_radamanto_config, radamanto_path, stamp_fractal_delivery_state,
    RADAMANTO_BATCH_SUBSCRIBER_KEY,
};
use super::invoke_orchestrator::invoke_process_full;
use crate::core::resolver::resolve_process_path;
use chrono::Utc;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use uuid::Uuid;

const VALID_ENTITY_TYPES: &[&str] = &[
    "process", "agent", "skill", "tool", "action", "norm", "codex", "event",
];

/// Allowlist aditiva (overlay). Tipología `process` también exime latency (L-LATENCY-PROCESS).
const LATENCY_THRESHOLD_EXEMPT: &[&str] = &["pull-request-review"];

fn is_latency_threshold_exempt(repo: &Path, entity_id: &str) -> bool {
    LATENCY_THRESHOLD_EXEMPT.contains(&entity_id)
        || resolve_entity_type(repo, entity_id) == "process"
}

fn success_rate_min_for(thresholds: &Value, etype: &str) -> f64 {
    thresholds
        .get("success_rate_min_by_entity_type")
        .and_then(|m| m.get(etype))
        .and_then(|v| v.as_f64())
        .or_else(|| thresholds.get("success_rate_min").and_then(|v| v.as_f64()))
        .unwrap_or(0.85)
}

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn write_json_atomic(path: &Path, data: &Value) -> Result<(), String> {
    super::eda_bus_topology::write_json_atomic(path, data)
}

fn load_stats(repo: &Path, cfg: &HashMap<String, Value>) -> Value {
    let path = radamanto_path(repo, cfg, "stats");
    if !path.is_file() {
        return json!({"entities": {}});
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .map(|mut data| {
            if !data.get("entities").and_then(|v| v.as_object()).is_some() {
                data["entities"] = json!({});
            }
            data
        })
        .unwrap_or(json!({"entities": {}}))
}

fn save_stats(repo: &Path, cfg: &HashMap<String, Value>, stats: &Value) -> Result<(), String> {
    let path = radamanto_path(repo, cfg, "stats");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_json_atomic(&path, stats)
}

fn load_consumed(repo: &Path, cfg: &HashMap<String, Value>) -> HashSet<String> {
    let path = radamanto_path(repo, cfg, "consumed");
    if !path.is_file() {
        return HashSet::new();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|data| {
            data.get("asset_ids")
                .and_then(|v| v.as_array())
                .map(|ids| ids.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        })
        .unwrap_or_default()
}

fn mark_consumed(repo: &Path, cfg: &HashMap<String, Value>, asset_id: &str) -> Result<(), String> {
    let mut consumed = load_consumed(repo, cfg);
    consumed.insert(asset_id.to_string());
    let mut ids: Vec<_> = consumed.into_iter().collect();
    ids.sort();
    let path = radamanto_path(repo, cfg, "consumed");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_json_atomic(&path, &json!({"asset_ids": ids}))
}

fn target_entity_from_payload(payload: &Value) -> String {
    if let Some(c) = payload.get("capsule_id").and_then(|v| v.as_str()) {
        if !c.trim().is_empty() {
            return c.trim().to_string();
        }
    }
    if let Some(p) = payload.get("process_name").and_then(|v| v.as_str()) {
        if !p.trim().is_empty() {
            return p.trim().to_string();
        }
    }
    "unknown-entity".into()
}

/// Precedencia L-TYPE-RESOLVE: prefijo `type:id` válido → catálogo process → default `tool`.
fn resolve_entity_type(repo: &Path, entity_id: &str) -> &'static str {
    if let Some((prefix, _)) = entity_id.split_once(':') {
        let p = prefix.trim().to_lowercase();
        if VALID_ENTITY_TYPES.contains(&p.as_str()) {
            return match p.as_str() {
                "process" => "process",
                "agent" => "agent",
                "skill" => "skill",
                "tool" => "tool",
                "action" => "action",
                "norm" => "norm",
                "codex" => "codex",
                "event" => "event",
                _ => "tool",
            };
        }
    }
    if resolve_process_path(repo, entity_id).is_ok() {
        return "process";
    }
    "tool"
}

fn governance_payload(repo: &Path, entity_id: &str, extra: Value) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert(
        "entity_type".into(),
        json!(resolve_entity_type(repo, entity_id)),
    );
    obj.insert("entity_id".into(), json!(entity_id));
    if let Some(map) = extra.as_object() {
        for (k, v) in map {
            obj.insert(k.clone(), v.clone());
        }
    }
    Value::Object(obj)
}

fn entity_bucket<'a>(stats: &'a mut Value, entity_id: &str) -> &'a mut Value {
    let entities = stats
        .as_object_mut()
        .expect("stats object");
    if !entities.get(entity_id).and_then(|v| v.as_object()).is_some() {
        entities.insert(
            entity_id.to_string(),
            json!({
                "samples": [],
                "status": "healthy",
                "recovery_attempts": 0,
                "degraded_at": null,
                "structure_valid": false,
                "consecutive_success_count": 0,
            }),
        );
    }
    entities.get_mut(entity_id).unwrap()
}

pub fn set_structure_valid(repo: &Path, entity_id: &str, valid: bool) -> Result<(), String> {
    let cfg = load_radamanto_config(repo)?;
    let mut stats = load_stats(repo, &cfg);
    {
        let bucket = entity_bucket(&mut stats, entity_id);
        bucket["structure_valid"] = json!(valid);
        if valid && bucket.get("status").and_then(|v| v.as_str()) == Some("degraded") {
            bucket["status"] = json!("pending_redemption");
            bucket["consecutive_success_count"] = json!(0);
        }
    }
    save_stats(repo, &cfg, &stats)
}

fn success_rate(samples: &[Value]) -> f64 {
    if samples.is_empty() {
        return 1.0;
    }
    let ok = samples
        .iter()
        .filter(|s| s.get("exit_code").and_then(|v| v.as_i64()).unwrap_or(1) == 0)
        .count();
    ok as f64 / samples.len() as f64
}

fn avg_duration(samples: &[Value]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum: i64 = samples
        .iter()
        .map(|s| s.get("duration_ms").and_then(|v| v.as_i64()).unwrap_or(0))
        .sum();
    sum as f64 / samples.len() as f64
}

fn build_domain_event(event_type: &str, payload: Value) -> Value {
    json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": event_type,
        "event_family": "domain",
        "timestamp": iso_now(),
        "emitter_agent": "radamanto",
        "payload": payload,
        "delivery_state": {},
    })
}

/// Snapshot de ejecución → Domain_Entity_Telemetry_Captured (fail-soft).
fn emit_telemetry_captured_failsoft(
    repo: &Path,
    entity_id: &str,
    asset_id: &str,
    sample: &Value,
    origin_event_id: &str,
) -> Value {
    let exit_code = sample.get("exit_code").and_then(|v| v.as_i64()).unwrap_or(1);
    let duration_ms = sample.get("duration_ms").and_then(|v| v.as_i64()).unwrap_or(0);
    let payload = json!({
        "entity_type": resolve_entity_type(repo, entity_id),
        "entity_id": entity_id,
        "asset_id": asset_id,
        "execution_metrics": {
            "duration_ms": duration_ms,
            "exit_code": exit_code,
            "success_status": exit_code == 0
        },
        "origin_stimulus": {
            "event_type": "Raw_Execution_Finished",
            "event_id": origin_event_id
        },
        "evolution_footprint": null,
        "state_after": {
            "last_execution_ms": duration_ms,
            "last_exit_code": exit_code
        }
    });
    let ev = build_domain_event("Domain_Entity_Telemetry_Captured", payload);
    match emit_domain_and_route(repo, &ev) {
        Ok(result) => json!({
            "type": "Domain_Entity_Telemetry_Captured",
            "result": result,
        }),
        Err(e) => json!({
            "type": "Domain_Entity_Telemetry_Captured",
            "error": e,
        }),
    }
}

fn emit_domain_and_route(repo: &Path, event: &Value) -> Result<Value, String> {
    let seal = write_fractal_event(repo, event, "domain")?;
    let route_out = if lab_route_sync() {
        if let Some(target) = seal.get("target_path").and_then(|v| v.as_str()) {
            invoke_process_full(
                repo,
                "route-domain",
                &json!({"event_file_path": target}),
            )
            .ok()
        } else {
            None
        }
    } else {
        None
    };
    Ok(json!({"seal": seal, "route": route_out}))
}

pub fn process_telemetry_file(repo: &Path, rel_path: &str) -> Value {
    match process_telemetry_file_inner(repo, rel_path) {
        Ok(v) => v,
        Err(e) => json!({"ok": false, "error": e}),
    }
}

fn process_telemetry_file_inner(repo: &Path, rel_path: &str) -> Result<Value, String> {
    let cfg = load_radamanto_config(repo)?;
    let thresholds = cfg
        .get("thresholds")
        .cloned()
        .unwrap_or(json!({}));
    let event_path = repo.join(rel_path.trim());
    if !event_path.is_file() {
        return Err(format!("no existe: {rel_path}"));
    }
    let body: Value =
        serde_json::from_str(&fs::read_to_string(&event_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let payload = body.get("payload").cloned().unwrap_or(json!({}));
    if !payload.is_object() {
        return Err("payload invalido".into());
    }
    let origin_event_id = body
        .get("event_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let asset_id = payload
        .get("asset_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("asset_id requerido")?;

    if load_consumed(repo, &cfg).contains(asset_id) {
        stamp_fractal_delivery_state(&event_path, RADAMANTO_BATCH_SUBSCRIBER_KEY, "skipped");
        return Ok(json!({"ok": true, "skipped": "duplicate_asset_id", "asset_id": asset_id}));
    }

    let entity_id = target_entity_from_payload(&payload);
    let mut stats = load_stats(repo, &cfg);
    let sample = json!({
        "asset_id": asset_id,
        "exit_code": payload.get("exit_code").and_then(|v| v.as_i64()).unwrap_or(1),
        "duration_ms": payload.get("duration_ms").and_then(|v| v.as_i64()).unwrap_or(0),
    });
    let max_keep = thresholds
        .get("batch_min_events")
        .and_then(|v| v.as_i64())
        .unwrap_or(10)
        .max(20) as usize;

    let mut samples: Vec<Value> = {
        let bucket = entity_bucket(&mut stats, &entity_id);
        let mut s = bucket
            .get("samples")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        s.push(sample.clone());
        if s.len() > max_keep {
            s = s[s.len() - max_keep..].to_vec();
        }
        bucket["samples"] = json!(s);
        s
    };

    let mut actions: Vec<Value> = Vec::new();
    let mut status = entity_bucket(&mut stats, &entity_id)
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("healthy")
        .to_string();
    let exit_code = sample.get("exit_code").and_then(|v| v.as_i64()).unwrap_or(1);

    if status == "degraded" || status == "pending_redemption" {
        let bucket = entity_bucket(&mut stats, &entity_id);
        if exit_code == 0 {
            let c = bucket
                .get("consecutive_success_count")
                .and_then(|v| v.as_i64())
                .unwrap_or(0)
                + 1;
            bucket["consecutive_success_count"] = json!(c);
        } else {
            bucket["consecutive_success_count"] = json!(0);
            let attempts = bucket
                .get("recovery_attempts")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let max_attempts = thresholds
                .get("max_recovery_attempts")
                .and_then(|v| v.as_i64())
                .unwrap_or(3);
            if attempts >= max_attempts {
                bucket["status"] = json!("deprecated");
                save_stats(repo, &cfg, &stats)?;
                let ev = build_domain_event(
                    "Domain_Entity_Deprecated",
                    governance_payload(
                        repo,
                        &entity_id,
                        json!({
                            "recovery_attempts": attempts,
                            "reason": "max_recovery_attempts_exceeded",
                        }),
                    ),
                );
                actions.push(json!({
                    "type": "Domain_Entity_Deprecated",
                    "result": emit_domain_and_route(repo, &ev)?,
                }));
                stats = load_stats(repo, &cfg);
                status = entity_bucket(&mut stats, &entity_id)
                    .get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("deprecated")
                    .to_string();
            }
        }
    }

    if status == "pending_redemption" {
        let structure_valid = entity_bucket(&mut stats, &entity_id)
            .get("structure_valid")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if structure_valid {
            let need = thresholds
                .get("redemption_success_count")
                .and_then(|v| v.as_i64())
                .unwrap_or(3);
            let count = entity_bucket(&mut stats, &entity_id)
                .get("consecutive_success_count")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            if count >= need {
                let rate = success_rate(&samples);
                {
                    let bucket = entity_bucket(&mut stats, &entity_id);
                    bucket["status"] = json!("healthy");
                    bucket["structure_valid"] = json!(false);
                    bucket["consecutive_success_count"] = json!(0);
                    bucket["degraded_at"] = Value::Null;
                }
                save_stats(repo, &cfg, &stats)?;
                let ev = build_domain_event(
                    "Domain_Entity_Restored",
                    governance_payload(
                        repo,
                        &entity_id,
                        json!({
                            "success_rate": (rate * 10000.0).round() / 10000.0,
                            "consecutive_success_count": need,
                        }),
                    ),
                );
                actions.push(json!({
                    "type": "Domain_Entity_Restored",
                    "result": emit_domain_and_route(repo, &ev)?,
                }));
                stats = load_stats(repo, &cfg);
                let keep = need as usize;
                samples = samples[samples.len().saturating_sub(keep)..].to_vec();
                entity_bucket(&mut stats, &entity_id)["samples"] = json!(samples);
                save_stats(repo, &cfg, &stats)?;
                mark_consumed(repo, &cfg, asset_id)?;
                actions.push(emit_telemetry_captured_failsoft(
                    repo,
                    &entity_id,
                    asset_id,
                    &sample,
                    &origin_event_id,
                ));
                stamp_fractal_delivery_state(
                    &event_path,
                    RADAMANTO_BATCH_SUBSCRIBER_KEY,
                    "success",
                );
                return Ok(json!({
                    "ok": true,
                    "asset_id": asset_id,
                    "entity_id": entity_id,
                    "status": "healthy",
                    "actions": actions,
                    "purged": false,
                }));
            }
        }
    }

    if status == "healthy" {
        let min_batch = thresholds
            .get("batch_min_events")
            .and_then(|v| v.as_i64())
            .unwrap_or(10);
        let abrupt_min = thresholds
            .get("abrupt_drop_min_samples")
            .and_then(|v| v.as_i64())
            .unwrap_or(3);
        let etype = resolve_entity_type(repo, &entity_id);
        let rate_min = success_rate_min_for(&thresholds, etype);
        let latency_thresh = thresholds
            .get("latency_ms_p95_threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(30000.0);
        let rate = success_rate(&samples);
        let avg_ms = avg_duration(&samples);
        let mut degraded = false;
        let mut reason = "";
        // Procesos multi-fase: umbral por tipo + latency exempt (L-LATENCY-PROCESS).
        if samples.len() as i64 >= min_batch && rate < rate_min {
            degraded = true;
            reason = "success_rate_below_threshold";
        } else if samples.len() as i64 >= abrupt_min && rate < rate_min {
            degraded = true;
            reason = "abrupt_success_rate_drop";
        } else if samples.len() >= 5
            && avg_ms > latency_thresh
            && !is_latency_threshold_exempt(repo, &entity_id)
        {
            degraded = true;
            reason = "latency_threshold";
        }
        if degraded {
            let attempts = entity_bucket(&mut stats, &entity_id)
                .get("recovery_attempts")
                .and_then(|v| v.as_i64())
                .unwrap_or(0)
                + 1;
            {
                let bucket = entity_bucket(&mut stats, &entity_id);
                bucket["recovery_attempts"] = json!(attempts);
                bucket["status"] = json!("degraded");
                bucket["degraded_at"] = json!(iso_now());
                bucket["structure_valid"] = json!(false);
                bucket["consecutive_success_count"] = json!(0);
            }
            save_stats(repo, &cfg, &stats)?;
            let ev = build_domain_event(
                "Domain_Entity_Degraded",
                governance_payload(
                    repo,
                    &entity_id,
                    json!({
                        "reason": reason,
                        "success_rate": (rate * 10000.0).round() / 10000.0,
                        "recovery_attempt": attempts,
                        "avg_duration_ms": (avg_ms * 100.0).round() / 100.0,
                    }),
                ),
            );
            actions.push(json!({
                "type": "Domain_Entity_Degraded",
                "result": emit_domain_and_route(repo, &ev)?,
            }));
            stats = load_stats(repo, &cfg);
        }
    }

    entity_bucket(&mut stats, &entity_id)["samples"] = json!(samples);
    save_stats(repo, &cfg, &stats)?;
    mark_consumed(repo, &cfg, asset_id)?;
    actions.push(emit_telemetry_captured_failsoft(
        repo,
        &entity_id,
        asset_id,
        &sample,
        &origin_event_id,
    ));
    stamp_fractal_delivery_state(&event_path, RADAMANTO_BATCH_SUBSCRIBER_KEY, "success");
    let final_status = entity_bucket(&mut load_stats(repo, &cfg), &entity_id)
        .get("status")
        .cloned()
        .unwrap_or(json!("healthy"));
    Ok(json!({
        "ok": true,
        "asset_id": asset_id,
        "entity_id": entity_id,
        "status": final_status,
        "actions": actions,
        "purged": false,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_rate_min_lookup_by_entity_type() {
        let t = json!({
            "success_rate_min": 0.85,
            "success_rate_min_by_entity_type": {
                "process": 0.70,
                "tool": 0.85,
                "agent": 0.75
            }
        });
        assert!((success_rate_min_for(&t, "process") - 0.70).abs() < 1e-9);
        assert!((success_rate_min_for(&t, "tool") - 0.85).abs() < 1e-9);
        assert!((success_rate_min_for(&t, "agent") - 0.75).abs() < 1e-9);
        assert!((success_rate_min_for(&t, "unknown") - 0.85).abs() < 1e-9);
        let plain = json!({"success_rate_min": 0.85});
        assert!((success_rate_min_for(&plain, "process") - 0.85).abs() < 1e-9);
    }

    #[test]
    fn allowlist_latency_exempt_without_catalog() {
        let fake = Path::new("/tmp/sddia-no-such-repo-radamanto-thresh");
        assert!(is_latency_threshold_exempt(fake, "pull-request-review"));
        assert!(!is_latency_threshold_exempt(fake, "delivery-close-cycle"));
        assert!(!is_latency_threshold_exempt(fake, "some-atomic-tool"));
    }

    #[test]
    fn bare_process_names_resolve_as_process() {
        let repo = crate::core::repo::find_repo_root().expect("repo root");
        assert_eq!(resolve_entity_type(&repo, "delivery-close-cycle"), "process");
        assert_eq!(resolve_entity_type(&repo, "pull-request-review"), "process");
        assert_eq!(resolve_entity_type(&repo, "tool:lab-x"), "tool");
        assert_eq!(resolve_entity_type(&repo, "process:feature"), "process");
        assert_eq!(resolve_entity_type(&repo, "no-such-entity-xyz"), "tool");
    }

    #[test]
    fn process_entity_is_latency_exempt_via_type() {
        let repo = crate::core::repo::find_repo_root().expect("repo root");
        assert!(is_latency_threshold_exempt(&repo, "delivery-close-cycle"));
        assert!(is_latency_threshold_exempt(&repo, "pull-request-review"));
        assert!(!is_latency_threshold_exempt(&repo, "no-such-entity-xyz"));
    }
}
