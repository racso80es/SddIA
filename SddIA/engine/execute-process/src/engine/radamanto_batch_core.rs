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
        return json!({"entities": {}, "cognitive": default_cognitive_block()});
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .map(|mut data| {
            if !data.get("entities").and_then(|v| v.as_object()).is_some() {
                data["entities"] = json!({});
            }
            if !data.get("cognitive").and_then(|v| v.as_object()).is_some() {
                data["cognitive"] = default_cognitive_block();
            }
            data
        })
        .unwrap_or(json!({"entities": {}, "cognitive": default_cognitive_block()}))
}

fn default_cognitive_block() -> Value {
    json!({
        "tokens_prompt_total": 0,
        "tokens_completion_total": 0,
        "by_model": {},
        "last_model": null,
        "latency_ms_avg": 0,
        "window": [],
        "quota_alert": false,
        "quota_critical": false
    })
}

fn cognitive_thresholds(thresholds: &Value) -> (u64, u64, usize) {
    let block = thresholds.get("cognitive").and_then(|v| v.as_object());
    let max_tpm = block
        .and_then(|o| o.get("max_tokens_per_minute"))
        .and_then(|v| v.as_u64())
        .unwrap_or(120_000);
    let critical = block
        .and_then(|o| o.get("critical_tokens_per_minute"))
        .and_then(|v| v.as_u64())
        .unwrap_or(500_000);
    let cap = block
        .and_then(|o| o.get("window_max_samples"))
        .and_then(|v| v.as_u64())
        .unwrap_or(120) as usize;
    (max_tpm, critical, cap)
}

fn apply_cognitive_receipt(stats: &mut Value, receipt: &Value, thresholds: &Value) -> bool {
    let prompt = receipt
        .get("prompt_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let completion = receipt
        .get("completion_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let latency = receipt
        .get("provider_latency_ms")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let model = receipt
        .get("llm_model")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let tokens = prompt + completion;

    let cog = stats
        .as_object_mut()
        .and_then(|o| {
            if !o.contains_key("cognitive") {
                o.insert("cognitive".into(), default_cognitive_block());
            }
            o.get_mut("cognitive")
        })
        .and_then(|v| v.as_object_mut());
    let Some(cog) = cog else {
        return false;
    };

    let prompt_total = cog
        .get("tokens_prompt_total")
        .and_then(|v| v.as_u64())
        .unwrap_or(0)
        + prompt;
    let completion_total = cog
        .get("tokens_completion_total")
        .and_then(|v| v.as_u64())
        .unwrap_or(0)
        + completion;
    cog.insert("tokens_prompt_total".into(), json!(prompt_total));
    cog.insert("tokens_completion_total".into(), json!(completion_total));
    if model != "unknown" {
        cog.insert("last_model".into(), json!(model));
        if !cog.contains_key("by_model") {
            cog.insert("by_model".into(), json!({}));
        }
        if let Some(bm) = cog.get_mut("by_model").and_then(|v| v.as_object_mut()) {
            let prev = bm.get(&model).and_then(|v| v.as_u64()).unwrap_or(0);
            bm.insert(model.clone(), json!(prev + tokens));
        }
    }
    let samples_n = cog
        .get("latency_samples")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let avg_prev = cog
        .get("latency_ms_avg")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let new_avg = if samples_n == 0 {
        latency as f64
    } else {
        (avg_prev * samples_n as f64 + latency as f64) / (samples_n + 1) as f64
    };
    cog.insert("latency_ms_avg".into(), json!(new_avg));
    cog.insert("latency_samples".into(), json!(samples_n + 1));

    let (max_tpm, critical_tpm, cap) = cognitive_thresholds(thresholds);
    let mut window: Vec<Value> = cog
        .get("window")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    window.push(json!({
        "ts": iso_now(),
        "tokens": tokens,
        "model": model,
    }));
    if window.len() > cap {
        window = window[window.len() - cap..].to_vec();
    }
    cog.insert("window".into(), json!(window));

    let now = Utc::now();
    let minute_ago = now - chrono::Duration::seconds(60);
    let mut tpm: u64 = 0;
    for entry in &window {
        let ts = entry.get("ts").and_then(|v| v.as_str()).unwrap_or("");
        if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(ts) {
            if parsed.with_timezone(&Utc) >= minute_ago {
                tpm += entry.get("tokens").and_then(|v| v.as_u64()).unwrap_or(0);
            }
        } else {
            tpm += entry.get("tokens").and_then(|v| v.as_u64()).unwrap_or(0);
        }
    }
    let alert = tpm > max_tpm;
    let critical = tpm > critical_tpm;
    cog.insert("quota_alert".into(), json!(alert));
    cog.insert("quota_critical".into(), json!(critical));
    cog.insert("tokens_per_minute".into(), json!(tpm));
    critical
}

fn drain_cognitive_inbox(
    repo: &Path,
    cfg: &HashMap<String, Value>,
    stats: &mut Value,
    thresholds: &Value,
) -> Result<Vec<Value>, String> {
    let inbox = radamanto_path(repo, cfg, "cognitive_inbox");
    if !inbox.is_dir() {
        return Ok(vec![]);
    }
    let mut critical_hits = Vec::new();
    let entries: Vec<_> = fs::read_dir(&inbox)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .collect();
    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(receipt) = serde_json::from_str::<Value>(&text) else {
            let _ = fs::remove_file(&path);
            continue;
        };
        if apply_cognitive_receipt(stats, &receipt, thresholds) {
            critical_hits.push(receipt);
        }
        let _ = fs::remove_file(&path);
    }
    Ok(critical_hits)
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

/// Provider revocado en mensaje Cerbero: `proveedor '{id}' revocado en revoked_entities`.
fn revoked_provider_from_phase_error(error: &str) -> Option<String> {
    const PREFIX: &str = "proveedor '";
    let start = error.find(PREFIX)? + PREFIX.len();
    let rest = &error[start..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

/// Poda auto-referencial: entidad revocada que aborta por su propia revocación no degrada ratio.
fn is_governance_self_revoked_hollow(payload: &Value) -> bool {
    if payload.get("failed_phase_code").and_then(|v| v.as_str()) != Some("CERBERO_ENTITY_REVOKED")
    {
        return false;
    }
    let error = payload
        .get("failed_phase_error")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let Some(revoked) = revoked_provider_from_phase_error(error) else {
        return false;
    };
    revoked == target_entity_from_payload(payload)
}

/// Poda de supervivencia: lab hueco no alimenta success_rate ni recovery_attempts.
pub(crate) fn is_survival_hollow(payload: &Value) -> bool {
    if payload.get("lab_hollow").and_then(|v| v.as_bool()) == Some(true) {
        return true;
    }
    if payload.get("detach").and_then(|v| v.as_bool()) == Some(true) {
        return true;
    }
    // Hijo PPR post-detach: el acuse CLI ya fue success; el KO tardío no es supervivencia.
    if payload.get("detached_child").and_then(|v| v.as_bool()) == Some(true)
        && payload.get("exit_code").and_then(|v| v.as_i64()) != Some(0)
    {
        return true;
    }
    matches!(
        payload.get("cycle_phase").and_then(|v| v.as_str()),
        Some("initialized") | Some("awaiting_agents")
    ) || is_governance_self_revoked_hollow(payload)
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

fn payload_check_run_id(payload: &Value) -> Option<i64> {
    if let Some(n) = payload.get("check_run_id").and_then(|v| v.as_i64()) {
        return Some(n);
    }
    payload
        .get("check_run_id")
        .and_then(|v| v.as_u64())
        .and_then(|n| i64::try_from(n).ok())
}

fn load_ci_failures(repo: &Path, cfg: &HashMap<String, Value>) -> Value {
    let path = radamanto_path(repo, cfg, "ci_failures");
    if !path.is_file() {
        return json!({ "failures": [] });
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .map(|mut data| {
            if !data.get("failures").and_then(|v| v.as_array()).is_some() {
                data["failures"] = json!([]);
            }
            data
        })
        .unwrap_or(json!({ "failures": [] }))
}

fn save_ci_failures(repo: &Path, cfg: &HashMap<String, Value>, data: &Value) -> Result<(), String> {
    let path = radamanto_path(repo, cfg, "ci_failures");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_json_atomic(&path, data)
}

fn process_ci_job_failed(
    repo: &Path,
    cfg: &HashMap<String, Value>,
    event_path: &Path,
    payload: &Value,
    origin_event_id: &str,
) -> Result<Value, String> {
    let check_run_id = payload_check_run_id(payload).ok_or("check_run_id requerido")?;
    let mut ledger = load_ci_failures(repo, cfg);
    let existing = ledger
        .get("failures")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if existing.iter().any(|row| payload_check_run_id(row) == Some(check_run_id)) {
        stamp_fractal_delivery_state(event_path, RADAMANTO_BATCH_SUBSCRIBER_KEY, "skipped");
        return Ok(json!({
            "ok": true,
            "kind": "ci_job_failed",
            "skipped": "duplicate_check_run_id",
            "check_run_id": check_run_id,
        }));
    }
    let mut failures = existing;
    failures.push(json!({
        "check_run_id": check_run_id,
        "job_name": payload.get("job_name"),
        "workflow_name": payload.get("workflow_name"),
        "head_sha": payload.get("head_sha"),
        "html_url": payload.get("html_url"),
        "repository": payload.get("repository"),
        "timestamp": iso_now(),
        "event_id": origin_event_id,
    }));
    ledger["failures"] = json!(failures);
    save_ci_failures(repo, cfg, &ledger)?;
    stamp_fractal_delivery_state(event_path, RADAMANTO_BATCH_SUBSCRIBER_KEY, "success");
    Ok(json!({
        "ok": true,
        "kind": "ci_job_failed",
        "check_run_id": check_run_id,
    }))
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
    let event_type = body
        .get("event_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if event_type == "CI_Job_Failed" {
        return process_ci_job_failed(repo, &cfg, &event_path, &payload, &origin_event_id);
    }
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

    if is_survival_hollow(&payload) {
        mark_consumed(repo, &cfg, asset_id)?;
        stamp_fractal_delivery_state(&event_path, RADAMANTO_BATCH_SUBSCRIBER_KEY, "skipped");
        return Ok(json!({
            "ok": true,
            "skipped": "survival_hollow",
            "asset_id": asset_id,
        }));
    }

    let entity_id = target_entity_from_payload(&payload);
    let mut stats = load_stats(repo, &cfg);
    let _inbox_critical = drain_cognitive_inbox(repo, &cfg, &mut stats, &thresholds)?;
    if let Some(receipt) = payload.get("telemetry_receipt").filter(|v| v.is_object()) {
        let critical = apply_cognitive_receipt(&mut stats, receipt, &thresholds);
        if critical {
            let capsule = payload
                .get("capsule_id")
                .and_then(|v| v.as_str())
                .unwrap_or("cognitive-system");
            let ev = build_domain_event(
                "Domain_Entity_Degraded",
                governance_payload(
                    repo,
                    capsule,
                    json!({
                        "reason": "cognitive_critical_quota",
                        "tokens_per_minute": stats.pointer("/cognitive/tokens_per_minute").cloned(),
                    }),
                ),
            );
            let _ = emit_domain_and_route(repo, &ev);
        }
    }
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
    fn cognitive_quota_alert_without_degraded_emit() {
        let mut stats = json!({"entities": {}, "cognitive": default_cognitive_block()});
        let thresholds = json!({
            "cognitive": {
                "max_tokens_per_minute": 10,
                "critical_tokens_per_minute": 1000,
                "window_max_samples": 10
            }
        });
        let receipt = json!({
            "prompt_tokens": 8,
            "completion_tokens": 5,
            "provider_latency_ms": 10,
            "llm_model": "lab"
        });
        let critical = apply_cognitive_receipt(&mut stats, &receipt, &thresholds);
        assert!(!critical);
        assert_eq!(stats["cognitive"]["quota_alert"], true);
    }

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

    #[test]
    fn hollow_by_cycle_phase() {
        assert!(is_survival_hollow(&json!({"cycle_phase": "initialized"})));
        assert!(is_survival_hollow(&json!({"cycle_phase": "awaiting_agents"})));
        assert!(is_survival_hollow(&json!({"lab_hollow": true, "cycle_phase": "completed"})));
    }

    #[test]
    fn fire_completed_and_failed_are_not_hollow() {
        assert!(!is_survival_hollow(&json!({"cycle_phase": "completed"})));
        assert!(!is_survival_hollow(&json!({"cycle_phase": "failed", "exit_code": 1})));
        assert!(!is_survival_hollow(&json!({"exit_code": 0})));
    }

    #[test]
    fn ppr_detached_child_failure_is_hollow() {
        assert!(is_survival_hollow(&json!({
            "process_name": "pull-request-review",
            "detached_child": true,
            "exit_code": 1,
            "duration_ms": 2_500_000
        })));
        assert!(!is_survival_hollow(&json!({
            "process_name": "pull-request-review",
            "detached_child": true,
            "exit_code": 0
        })));
        assert!(is_survival_hollow(&json!({
            "process_name": "pull-request-review",
            "detach": true,
            "cycle_phase": "awaiting_agents"
        })));
    }

    #[test]
    fn t_a2_hollow_entity_revoked_self() {
        assert!(is_survival_hollow(&json!({
            "capsule_id": "skill:git-manager",
            "failed_phase_code": "CERBERO_ENTITY_REVOKED",
            "failed_phase_error": "proveedor 'skill:git-manager' revocado en revoked_entities",
            "exit_code": 1
        })));
    }

    #[test]
    fn t_a2_hollow_rbac_denied_not_podado() {
        assert!(!is_survival_hollow(&json!({
            "process_name": "feature",
            "failed_phase_code": "CERBERO_RBAC_DENIED",
            "failed_phase_error": "RBAC deny: provider 'skill:filesystem-manager' context=[\"filesystem-ops\"] ∉ requester=[\"knowledge-management\"]",
            "exit_code": 1
        })));
    }

    #[test]
    fn t_a2_hollow_revoked_other_provider_not_podado() {
        assert!(!is_survival_hollow(&json!({
            "process_name": "pull-request-review",
            "failed_phase_code": "CERBERO_ENTITY_REVOKED",
            "failed_phase_error": "proveedor 'skill:git-manager' revocado en revoked_entities",
            "exit_code": 1
        })));
    }

    #[test]
    fn t_a2_hollow_config_error_not_podado() {
        assert!(!is_survival_hollow(&json!({
            "process_name": "feature",
            "failed_phase_code": "CERBERO_CONFIG_ERROR",
            "failed_phase_error": "políticas solicitante vacías (process=feature, phase=lab)",
            "exit_code": 1
        })));
    }

    #[test]
    fn thresholds_110_process_intact() {
        let repo = crate::core::repo::find_repo_root().expect("repo root");
        let raw = std::fs::read_to_string(repo.join("SddIA/agents/radamanto.thresholds.json"))
            .expect("thresholds");
        let t: Value = serde_json::from_str(&raw).expect("json");
        assert_eq!(t["version"], "1.2.0");
        assert_eq!(t["success_rate_min_by_entity_type"]["process"], 0.70);
        assert_eq!(t["max_recovery_attempts"], 3);
        assert!(t.get("cognitive").is_some());
    }

    #[test]
    fn ci_job_failed_writes_ledger_not_stats() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
  "radamanto": {
    "stats": ".SddIA/radamanto/stats.json",
    "ci_failures": ".SddIA/radamanto/ci_failures.json",
    "consumed": ".SddIA/radamanto/consumed.json",
    "thresholds": "SddIA/agents/radamanto.thresholds.json"
  }
}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join(".events/telemetry")).unwrap();
        fs::create_dir_all(repo.join(".SddIA/radamanto")).unwrap();
        let event_id = "11111111-1111-4111-8111-111111111111";
        let rel = format!(".events/telemetry/{event_id}.json");
        let event = json!({
            "event_id": event_id,
            "event_type": "CI_Job_Failed",
            "timestamp": "2026-09-01T00:00:00Z",
            "emitter_agent": "github-bridge-watcher",
            "payload": {
                "repository": "racso80es/SddIA",
                "head_sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "workflow_name": "sddia-index-qa",
                "job_name": "sddia-index-integrity",
                "conclusion": "failure",
                "html_url": "https://github.com/racso80es/SddIA/runs/4242",
                "check_run_id": 4242
            }
        });
        fs::write(repo.join(&rel), serde_json::to_string_pretty(&event).unwrap()).unwrap();

        let r1 = process_telemetry_file(repo, &rel);
        assert_eq!(r1["ok"], true);
        assert_eq!(r1["kind"], "ci_job_failed");
        assert!(!repo.join(".SddIA/radamanto/stats.json").is_file());
        let ledger: Value = serde_json::from_str(
            &fs::read_to_string(repo.join(".SddIA/radamanto/ci_failures.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(ledger["failures"].as_array().unwrap().len(), 1);

        let r2 = process_telemetry_file(repo, &rel);
        assert_eq!(r2["skipped"], "duplicate_check_run_id");
        let ledger2: Value = serde_json::from_str(
            &fs::read_to_string(repo.join(".SddIA/radamanto/ci_failures.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(ledger2["failures"].as_array().unwrap().len(), 1);
    }
}
