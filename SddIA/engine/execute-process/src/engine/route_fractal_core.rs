//! Enrutador bus EDA fractal (paridad `route_fractal_event_core.py`).

use super::cerbero_governance_react_core::react_to_domain_event;
use super::eda_bus_topology::{rel_event_path, safe_remove_path, subscriber_id};
use super::fix_tool_process_core::process_fix_tool;
use super::fractal_bus::{
    delivery_stamp_terminal_ok, fractal_delivery_terminal_with_failure,
    load_fractal_subscription_rel, maybe_purge_fractal_telemetry_when_terminal,
    move_fractal_event_to_dead_letter, stamp_fractal_delivery_state,
};
use super::invoke_orchestrator::invoke_process_full;
use super::memory_evolution_ingest_core::ingest_domain_event_file;
use super::radamanto_batch_core::process_telemetry_file;
use super::route_domain_core::dispatch_subscriber;
use super::telemetry_compliance_core::audit_telemetry_compliance;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const OK_STATUSES: &[&str] = &[
    "success",
    "skipped-topology",
    "skipped-backfill",
    "skipped-pre-anchored",
    "skipped-dlt-threshold",
    "skipped-empty-text",
    "skipped-empty-message",
    "skipped-no-correlation",
    "skipped-lab-simulated",
    "skipped-already-delivered",
];

fn dispatch_fractal_subscriber(
    repo: &Path,
    subscriber: &Value,
    event: &Value,
    rel_path: &str,
) -> (String, String, Option<String>, i32) {
    let sid = subscriber_id(subscriber);
    if let Some(process_name) = subscriber.get("process").and_then(|v| v.as_str()) {
        let key = process_name.trim();
        match key {
            "radamanto-batch" => {
                let result = process_telemetry_file(repo, rel_path);
                if result.get("ok").and_then(|v| v.as_bool()) == Some(true) {
                    return (sid, "success".into(), None, 0);
                }
                return (
                    sid,
                    "failed".into(),
                    result
                        .get("error")
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                        .or_else(|| Some("radamanto-batch failed".into())),
                    1,
                );
            }
            "telemetry-compliance-audit" => {
                let result = audit_telemetry_compliance(repo, rel_path);
                if result.get("ok").and_then(|v| v.as_bool()) == Some(true) {
                    return (sid, "success".into(), None, 0);
                }
                return (
                    sid,
                    "failed".into(),
                    result
                        .get("error")
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                        .or_else(|| Some("telemetry-compliance-audit failed".into())),
                    1,
                );
            }
            "daemon-heartbeat-audit" => match super::handlers::daemon_heartbeat::audit_telemetry_file(repo, rel_path) {
                Ok(result) if result.get("ok").and_then(|v| v.as_bool()) == Some(true) => {
                    (sid, "success".into(), None, 0)
                }
                Ok(result) => (
                    sid,
                    "failed".into(),
                    result
                        .get("error")
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                        .or_else(|| Some("daemon-heartbeat-audit failed".into())),
                    1,
                ),
                Err(e) => (sid, "failed".into(), Some(e), 1),
            },
            "telemetry-batch-stub" => {
                let process_inputs = json!({
                    "event_file_path": rel_path,
                    "correlation_id": event.get("event_id").and_then(|v| v.as_str()).unwrap_or(""),
                });
                match invoke_process_full(repo, "telemetry-batch-stub", &process_inputs) {
                    Ok(envelope) => {
                        let exit_code = envelope
                            .get("status_code")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(if envelope.get("success").and_then(|v| v.as_bool()) == Some(true) { 0 } else { 1 })
                            as i32;
                        let ok = envelope.get("success").and_then(|v| v.as_bool()) == Some(true) && exit_code == 0;
                        if ok {
                            (sid, "success".into(), None, 0)
                        } else {
                            (
                                sid,
                                "failed".into(),
                                envelope
                                    .get("error")
                                    .and_then(|v| v.as_str())
                                    .map(str::to_string)
                                    .or_else(|| Some("telemetry-batch-stub failed".into())),
                                exit_code,
                            )
                        }
                    }
                    Err(e) => (sid, "failed".into(), Some(e), 1),
                }
            }
            "cerbero-governance-react" => {
                let result = react_to_domain_event(repo, event);
                if result.get("ok").and_then(|v| v.as_bool()) == Some(true) {
                    (sid, "success".into(), None, 0)
                } else {
                    (
                        sid,
                        "failed".into(),
                        result
                            .get("error")
                            .and_then(|v| v.as_str())
                            .map(str::to_string)
                            .or_else(|| Some("cerbero-governance-react failed".into())),
                        1,
                    )
                }
            }
            "fix-tool-process" => {
                let result = process_fix_tool(repo, rel_path);
                if result.get("ok").and_then(|v| v.as_bool()) == Some(true) {
                    (sid, "success".into(), None, 0)
                } else {
                    (
                        sid,
                        "failed".into(),
                        result
                            .get("error")
                            .and_then(|v| v.as_str())
                            .map(str::to_string)
                            .or_else(|| Some("fix-tool-process failed".into())),
                        1,
                    )
                }
            }
            "memory-evolution-ingest" => {
                let result = ingest_domain_event_file(repo, rel_path);
                if result.get("ok").and_then(|v| v.as_bool()) == Some(true) {
                    (sid, "success".into(), None, 0)
                } else {
                    (
                        sid,
                        "failed".into(),
                        result
                            .get("error")
                            .and_then(|v| v.as_str())
                            .map(str::to_string)
                            .or_else(|| Some("memory-evolution-ingest failed".into())),
                        1,
                    )
                }
            }
            "execute-suite" => {
                let pl = event.get("payload");
                let Some(payload) = pl.and_then(|v| v.as_object()) else {
                    return (sid, "failed".into(), Some("payload must be object".into()), 1);
                };
                let suite_id = payload.get("suite_id").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty());
                let Some(suite_id) = suite_id else {
                    return (sid, "failed".into(), Some("suite_id missing in payload".into()), 1);
                };
                let mut process_inputs = json!({"suite_id": suite_id});
                if let Some(strategy) = payload.get("execution_strategy").and_then(|v| v.as_str()) {
                    if strategy == "fail_fast" || strategy == "run_all" {
                        process_inputs["execution_strategy"] = json!(strategy);
                    }
                }
                if let Some(asset_id) = payload.get("asset_id").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
                    process_inputs["asset_id"] = json!(asset_id);
                }
                match invoke_process_full(repo, "execute-suite", &process_inputs) {
                    Ok(envelope) => {
                        let exit_code = envelope
                            .get("status_code")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(if envelope.get("success").and_then(|v| v.as_bool()) == Some(true) { 0 } else { 1 })
                            as i32;
                        let ok = envelope.get("success").and_then(|v| v.as_bool()) == Some(true) && exit_code == 0;
                        if ok {
                            (sid, "success".into(), None, 0)
                        } else {
                            (
                                sid,
                                "failed".into(),
                                envelope
                                    .get("error")
                                    .and_then(|v| v.as_str())
                                    .map(str::to_string)
                                    .or_else(|| Some("execute-suite failed".into())),
                                exit_code,
                            )
                        }
                    }
                    Err(e) => (sid, "failed".into(), Some(e), 1),
                }
            }
            _ => {
                let mut event_mut = event.clone();
                dispatch_subscriber(repo, subscriber, &mut event_mut, false)
            }
        }
    } else {
        let mut event_mut = event.clone();
        dispatch_subscriber(repo, subscriber, &mut event_mut, false)
    }
}

fn route_fractal_event(
    repo: &Path,
    event_file_path: &str,
    subscriptions_rel: &str,
    purge_after: bool,
    skip_ecst_gate: bool,
) -> Value {
    let raw_path = Path::new(event_file_path);
    let event_path = if raw_path.is_absolute() {
        raw_path.to_path_buf()
    } else {
        repo.join(raw_path)
    };
    let event_path = event_path.canonicalize().unwrap_or(event_path);

    if !event_path.is_file() {
        return json!({
            "success": false,
            "exitCode": 1,
            "data": null,
            "error": format!("event file not found: {}", event_path.display()),
        });
    }

    let event: Value = match fs::read_to_string(&event_path)
        .map_err(|e| e.to_string())
        .and_then(|t| serde_json::from_str(&t).map_err(|e| e.to_string()))
    {
        Ok(v) => v,
        Err(e) => {
            return json!({
                "success": false,
                "exitCode": 1,
                "data": null,
                "error": format!("invalid event JSON: {e}"),
            });
        }
    };

    let event_type = match event.get("event_type").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        Some(et) => et,
        None => {
            return json!({
                "success": false,
                "exitCode": 1,
                "data": null,
                "error": "event_type missing",
            });
        }
    };

    let subs_path = repo.join(subscriptions_rel);
    let registry: Value = match fs::read_to_string(&subs_path)
        .map_err(|e| e.to_string())
        .and_then(|t| serde_json::from_str(&t).map_err(|e| e.to_string()))
    {
        Ok(v) => v,
        Err(e) => {
            return json!({
                "success": false,
                "exitCode": 1,
                "data": null,
                "error": format!("cannot read subscriptions: {e}"),
            });
        }
    };

    let subscribers: Vec<Value> = registry
        .get(event_type)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter(|sub| sub.is_object()).cloned().collect())
        .unwrap_or_default();

    let rel_path = rel_event_path(repo, &event_path);
    let mut delivery_status = serde_json::Map::new();
    let mut all_ok = true;

    if subscribers.is_empty() {
        let purged_empty = if purge_after {
            safe_remove_path(&event_path)
        } else {
            false
        };
        let purge_failed_empty = purge_after && !purged_empty && event_path.is_file();
        return json!({
            "success": true,
            "exitCode": 0,
            "data": {
                "success": true,
                "delivery_status": {},
                "parent_path": rel_path,
                "purged": purged_empty,
                "purge_failed": purge_failed_empty,
                "skip_ecst_gate": skip_ecst_gate,
            },
        });
    }

    let prior_delivery = event
        .get("delivery_state")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    for sub in &subscribers {
        let sid = subscriber_id(sub);
        if let Some(prev) = prior_delivery.get(&sid).and_then(|v| v.as_str()) {
            if delivery_stamp_terminal_ok(prev) {
                delivery_status.insert(sid.clone(), json!("skipped-already-delivered"));
                stamp_fractal_delivery_state(&event_path, &sid, "skipped-already-delivered");
                continue;
            }
        }
        let (sid, status, _err, _code) =
            dispatch_fractal_subscriber(repo, sub, &event, &rel_path);
        stamp_fractal_delivery_state(&event_path, &sid, &status);
        if !OK_STATUSES.contains(&status.as_str()) {
            all_ok = false;
        }
        delivery_status.insert(sid, json!(status));
    }

    // Releer stamps tras despacho: all_ok si todos los requeridos están terminales OK
    if event_path.is_file() {
        if let Ok(raw) = fs::read_to_string(&event_path) {
            if let Ok(body) = serde_json::from_str::<Value>(&raw) {
                if let Some(ds) = body.get("delivery_state").and_then(|v| v.as_object()) {
                    let required: Vec<String> = subscribers.iter().map(subscriber_id).collect();
                    all_ok = required.iter().all(|sid| {
                        ds.get(sid)
                            .and_then(|v| v.as_str())
                            .map(delivery_stamp_terminal_ok)
                            .unwrap_or(false)
                    });
                }
            }
        }
    }

    let purged_purge_after = if all_ok && purge_after && event_path.is_file() {
        safe_remove_path(&event_path)
    } else {
        false
    };

    // Laudo C2: terminal-with-failure → move a eda_fractal.dead_letter (no unlink).
    let mut dead_lettered = false;
    if !all_ok && purge_after && event_path.is_file() {
        if let Ok(raw) = fs::read_to_string(&event_path) {
            if let Ok(body) = serde_json::from_str::<Value>(&raw) {
                if fractal_delivery_terminal_with_failure(&body) {
                    dead_lettered = move_fractal_event_to_dead_letter(repo, &event_path);
                }
            }
        }
    }

    let subs_norm = subscriptions_rel.replace('\\', "/");
    let is_telemetry_bus = subs_norm.contains("telemetry");
    let purged_telemetry = if all_ok && is_telemetry_bus && event_path.is_file() {
        maybe_purge_fractal_telemetry_when_terminal(repo, &event_path, &registry, event_type)
    } else {
        false
    };

    let purged = purged_purge_after || purged_telemetry;
    let purge_failed = all_ok
        && event_path.is_file()
        && ((purge_after && !purged_purge_after) || (is_telemetry_bus && !purged_telemetry));
    let dead_letter_failed = !all_ok
        && purge_after
        && !dead_lettered
        && event_path.is_file()
        && {
            fs::read_to_string(&event_path)
                .ok()
                .and_then(|r| serde_json::from_str::<Value>(&r).ok())
                .map(|b| fractal_delivery_terminal_with_failure(&b))
                .unwrap_or(false)
        };

    json!({
        "success": all_ok,
        "exitCode": if all_ok { 0 } else { 1 },
        "data": {
            "success": all_ok,
            "delivery_status": delivery_status,
            "parent_path": rel_path,
            "purged": purged,
            "purge_failed": purge_failed,
            "dead_lettered": dead_lettered,
            "dead_letter_failed": dead_letter_failed,
            "skip_ecst_gate": skip_ecst_gate,
        },
        "error": if all_ok { Value::Null } else { json!("one or more subscribers failed") },
    })
}

pub fn route_telemetry_event(repo: &Path, event_file_path: &str) -> Value {
    let subs = load_fractal_subscription_rel(repo, "telemetry_subscriptions")
        .unwrap_or_else(|_| "SddIA/core/event-telemetry-subscriptions.json".into());
    route_fractal_event(repo, event_file_path, &subs, false, true)
}

pub fn route_orchestration_event(repo: &Path, event_file_path: &str) -> Value {
    let subs = load_fractal_subscription_rel(repo, "orchestration_subscriptions")
        .unwrap_or_else(|_| "SddIA/core/event-orchestration-subscriptions.json".into());
    route_fractal_event(repo, event_file_path, &subs, true, false)
}

pub fn route_domain_fractal_event(repo: &Path, event_file_path: &str) -> Value {
    let subs = load_fractal_subscription_rel(repo, "domain_subscriptions")
        .unwrap_or_else(|_| "SddIA/core/event-domain-subscriptions.json".into());
    // Opción B: purga física del JSON domain tras consenso de suscriptores (purge_after=true).
    route_fractal_event(repo, event_file_path, &subs, true, false)
}

pub fn invoke_route_fractal(repo: &Path, fn_name: &str, event_rel: &str) -> Result<Value, String> {
    let out = match fn_name {
        "route_telemetry_event" => route_telemetry_event(repo, event_rel),
        "route_orchestration_event" => route_orchestration_event(repo, event_rel),
        "route_domain_fractal_event" => route_domain_fractal_event(repo, event_rel),
        other => return Err(format!("route fractal desconocido: {other}")),
    };
    Ok(out)
}

pub fn invoke_radamanto_batch(repo: &Path, event_rel: &str) -> Result<Value, String> {
    Ok(process_telemetry_file(repo, event_rel))
}

pub fn invoke_telemetry_compliance(repo: &Path, event_rel: &str) -> Result<Value, String> {
    Ok(audit_telemetry_compliance(repo, event_rel))
}
