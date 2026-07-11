//! Núcleo orquestador `route-domain-event` nativo (paridad `route_domain_event_core.py`).

use super::actions;
use super::capsules::{invoke_capsule_json, invoke_tool};
use super::ecst_validation::{load_event_class_schemas, validate_ecst_instance};
use super::eda_bus_topology::{
    delegation_meta, dlt_threshold_ok, ensure_event_bus_topology, ensure_processing_header,
    github_pr_merged, inject_domain_entity_topology_defaults, infer_persist_ref_from_branch,
    is_backfill_emitter, maybe_purge_processing_header, promote_witness, rel_event_path,
    resolve_origin_topology, resolve_pull_request_lifecycle, subscriber_applies_to_topology,
    subscriber_id, terminal_witness_exists, try_sweep_event, write_json_atomic,
    write_processing_witness, ECST_GATE_SUBSCRIBER, EventBusTopology,
};
use super::invoke_orchestrator::{invoke_process_full, resolve_orchestrator_bin};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

const ALLOWLIST_KALMA2: &[&str] = &["bug-fix", "feature", "refactorization", "task-queue-manager"];

fn python_bin() -> String {
    std::env::var("PYTHON").unwrap_or_else(|_| "python3".into())
}

fn sync_dispatch_mode() -> bool {
    matches!(
        std::env::var("SDDIA_LAB_ROUTE_SYNC")
            .unwrap_or_default()
            .trim()
            .to_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

fn dispatch_mode_label() -> &'static str {
    if sync_dispatch_mode() {
        "sync"
    } else {
        "async"
    }
}

fn iota_timeout_seconds() -> u64 {
    std::env::var("SDDIA_IOTA_TIMEOUT_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(45)
}

fn status_is_terminal_ok(status: &str) -> bool {
    status == "success" || status.starts_with("skipped")
}

fn build_telegram_message_from_event(event: &Value) -> Option<String> {
    let event_type = event.get("event_type").and_then(|v| v.as_str())?;
    let payload = event.get("payload").and_then(|v| v.as_object());
    match event_type {
        "PullRequest_Presented" => {
            let branch = payload
                .and_then(|p| p.get("branch"))
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let mut lines = vec![format!("PR presentado: {branch}")];
            if let Some(url) = payload
                .and_then(|p| p.get("pr_url"))
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                lines.push(url.to_string());
            }
            Some(lines.join("\n"))
        }
        "System_Fracture_Detected" => {
            let proc = payload
                .and_then(|p| p.get("process_name"))
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let trace = payload
                .and_then(|p| p.get("trace_hash"))
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .or_else(|| {
                    payload
                        .and_then(|p| p.get("error_trace"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.trim().chars().take(120).collect::<String>())
                        .filter(|s| !s.is_empty())
                })
                .unwrap_or_else(|| "sin-traza".into());
            Some(format!("Fractura detectada: {proc}\n{trace}"))
        }
        _ => None,
    }
}

fn invoke_send_telegram_notification(repo: &Path, message: &str) -> Result<(bool, Value), String> {
    let req = json!({"message": message});
    match invoke_tool(repo, "send-telegram-notification", &req) {
        Ok(body) => Ok((true, body)),
        Err(e) => Ok((false, json!({"error": e}))),
    }
}

fn invoke_iota_publisher(repo: &Path, event: &Value) -> (bool, String, i32, Option<String>) {
    let payload = json!({
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": serde_json::to_string(event).unwrap_or_default(),
    });
    let _timeout = iota_timeout_seconds();
    let repo = repo.to_path_buf();
    let handle = thread::spawn(move || invoke_capsule_json(&repo, "iota-immutable-publisher", &payload, true));
    let result = match handle.join() {
        Ok(Ok(r)) => r,
        Ok(Err(e)) => {
            return (false, e, 1, None);
        }
        Err(_) => return (false, "iota thread panicked".into(), 1, None),
    };
    let body = &result.body;
    let ok = body.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
    let digest = body
        .get("result")
        .and_then(|r| r.get("transaction_digest"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let feedback = body
        .get("feedback")
        .or_else(|| body.get("message"))
        .and_then(|v| v.as_str())
        .unwrap_or(if ok { "ok" } else { "iota publish failed" })
        .to_string();
    (ok, feedback, if ok { 0 } else { result.exit_code }, digest)
}

fn invoke_execute_action(repo: &Path, action: &str, payload: &Value) -> Result<(bool, String, i32), String> {
    let runner = repo.join("SddIA/scripts/qa/execute-action.py");
    if !runner.is_file() {
        return Ok((false, "execute-action.py not found".into(), 1));
    }
    let inputs_json = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    let output = Command::new(python_bin())
        .args([
            runner.to_string_lossy().as_ref(),
            "--action",
            action,
            "--inputs",
            &inputs_json,
        ])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("spawn execute-action: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let line = stdout.lines().filter(|l| !l.trim().is_empty()).last().unwrap_or("");
    if line.is_empty() {
        return Ok((
            false,
            if stderr.trim().is_empty() {
                "empty stdout".into()
            } else {
                stderr.trim().to_string()
            },
            output.status.code().unwrap_or(1),
        ));
    }
    let envelope: Value = serde_json::from_str(line).map_err(|e| format!("invalid JSON: {e}"))?;
    let data = envelope.get("data").cloned().unwrap_or(json!({}));
    let exit_code = envelope
        .get("status_code")
        .and_then(|v| v.as_i64())
        .unwrap_or(if envelope.get("success").and_then(|v| v.as_bool()) == Some(true) {
            0
        } else {
            1
        }) as i32;
    let ok = envelope.get("success").and_then(|v| v.as_bool()) == Some(true)
        && data.get("success").and_then(|v| v.as_bool()).unwrap_or(true);
    let err = envelope
        .get("error")
        .or_else(|| data.get("error"))
        .and_then(|v| v.as_str())
        .unwrap_or("action failed")
        .to_string();
    Ok((ok, err, exit_code))
}

fn pull_request_review_precheck(
    repo: &Path,
    branch: &str,
    pr_url: Option<&str>,
    payload: &Value,
) -> (bool, Option<String>, Value) {
    let target = payload
        .get("target_branch")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("main");
    let lifecycle = resolve_pull_request_lifecycle(repo, branch, pr_url, target);
    let merged = lifecycle.get("merged");
    let on_remote = lifecycle.get("branch_on_remote").and_then(|v| v.as_bool()) == Some(true);
    if merged == Some(&json!(true)) {
        return (true, None, lifecycle);
    }
    if merged == Some(&json!(false)) && !on_remote {
        let pr_num = lifecycle.get("pr_number");
        return (
            false,
            Some(format!(
                "pull-request-review: rama ausente en origin y PR no mergeado (branch={branch}, pr={pr_num:?})"
            )),
            lifecycle,
        );
    }
    if merged.is_none() && !on_remote {
        let diag = lifecycle.get("diagnostics").cloned().unwrap_or(json!([]));
        return (
            false,
            Some(format!(
                "pull-request-review: no se pudo resolver ciclo de vida del PR (branch={branch}; diagnostics={diag})"
            )),
            lifecycle,
        );
    }
    (true, None, lifecycle)
}

fn dispatch_process_subscriber(
    repo: &Path,
    process_key: &str,
    process_inputs: Value,
) -> (String, i32) {
    match invoke_process_full(repo, process_key, &process_inputs) {
        Ok(envelope) => {
            let exit_code = envelope
                .get("status_code")
                .or_else(|| envelope.get("exit_code"))
                .and_then(|v| v.as_i64())
                .unwrap_or(if envelope.get("success").and_then(|v| v.as_bool()) == Some(true) {
                    0
                } else {
                    1
                }) as i32;
            let ok = envelope.get("success").and_then(|v| v.as_bool()) == Some(true) && exit_code == 0;
            if ok {
                ("success".into(), 0)
            } else {
                let err = envelope
                    .get("error")
                    .or_else(|| envelope.get("message"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("process failed")
                    .to_string();
                (err, exit_code)
            }
        }
        Err(e) => (e, 1),
    }
}

fn dispatch_subscriber(
    repo: &Path,
    subscriber: &Value,
    event: &mut Value,
) -> (String, String, Option<String>, i32) {
    let sid = subscriber_id(subscriber);
    let agent = subscriber.get("agent").and_then(|v| v.as_str()).map(str::trim);
    if agent.is_none() || agent == Some("") {
        return (sid, "failed".into(), Some("missing agent".into()), 1);
    }

    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    let origin_topology = resolve_origin_topology(&payload);
    if event
        .get("event_type")
        .and_then(|v| v.as_str())
        .map(|et| et.starts_with("Domain_Entity_"))
        == Some(true)
    {
        if !subscriber_applies_to_topology(subscriber, &origin_topology) {
            return (sid, "skipped-topology".into(), None, 0);
        }
    }

    if let Some(process_name) = subscriber.get("process").and_then(|v| v.as_str()) {
        let process_key = process_name.trim();
        if process_key.is_empty() {
            return (sid, "failed".into(), Some("empty process".into()), 1);
        }
        if resolve_orchestrator_bin(repo).is_err() {
            return (
                sid,
                "failed".into(),
                Some("orquestador execute-process no encontrado".into()),
                1,
            );
        }
        let Some(payload_obj) = payload.as_object() else {
            return (sid, "failed".into(), Some("payload must be object".into()), 1);
        };

        if event.get("event_type").and_then(|v| v.as_str()) == Some("Kalma2_Process_Requested") {
            let proc = payload_obj
                .get("process")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .unwrap_or("");
            if !ALLOWLIST_KALMA2.contains(&proc) {
                return (
                    sid,
                    "failed".into(),
                    Some(format!("proceso no permitido: {proc}")),
                    1,
                );
            }
            let mut process_inputs = Map::new();
            process_inputs.insert(
                "correlation_id".into(),
                json!(event.get("event_id").and_then(|v| v.as_str()).unwrap_or("")),
            );
            process_inputs.insert("process".into(), json!(proc));
            if let Some(pbi) = payload_obj.get("pbi_ref").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
                process_inputs.insert("pbi_ref".into(), json!(pbi));
            }
            if let Some(raw) = payload_obj.get("raw_text").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
                process_inputs.insert("task_text".into(), json!(raw));
            }
            if let Some(extra) = payload_obj.get("process_inputs").and_then(|v| v.as_object()) {
                for (k, v) in extra {
                    if !process_inputs.contains_key(k) {
                        process_inputs.insert(k.clone(), v.clone());
                    }
                }
            }
            let (status, exit_code) =
                dispatch_process_subscriber(repo, process_key, Value::Object(process_inputs));
            if status == "success" {
                return (sid, status, None, exit_code);
            }
            return (sid, "failed".into(), Some(status), exit_code);
        }

        if process_key == "telegram-fallback-responder" {
            let text = payload_obj.get("text").and_then(|v| v.as_str()).map(str::trim);
            if text.is_none() || text == Some("") {
                return (sid, "skipped-empty-text".into(), None, 0);
            }
            let mut process_inputs = Map::new();
            process_inputs.insert("text".into(), json!(text.unwrap()));
            if let Some(chat) = payload_obj.get("chat_id").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
                process_inputs.insert("chat_id".into(), json!(chat));
            }
            let (status, exit_code) =
                dispatch_process_subscriber(repo, process_key, Value::Object(process_inputs));
            if status == "success" {
                return (sid, status, None, exit_code);
            }
            return (sid, "failed".into(), Some(status), exit_code);
        }

        let branch = payload_obj.get("branch").and_then(|v| v.as_str()).map(str::trim);
        let Some(branch) = branch.filter(|s| !s.is_empty()) else {
            return (sid, "failed".into(), Some("branch missing in payload".into()), 1);
        };

        let mut process_inputs = Map::new();
        process_inputs.insert("pr_branch".into(), json!(branch));
        process_inputs.insert(
            "pr_id_or_path".into(),
            json!(payload_obj
                .get("pr_url")
                .and_then(|v| v.as_str())
                .unwrap_or(branch)),
        );
        process_inputs.insert(
            "correlation_id".into(),
            json!(event.get("event_id").and_then(|v| v.as_str()).unwrap_or("")),
        );
        process_inputs.insert("author".into(), json!("eda-bus-watcher"));

        let pr_url = payload_obj.get("pr_url").and_then(|v| v.as_str());
        if let Some(url) = pr_url.map(str::trim).filter(|s| !s.is_empty()) {
            process_inputs.insert("pr_url".into(), json!(url));
        }

        if process_key == "pull-request-review" {
            let (ok, err, lifecycle) =
                pull_request_review_precheck(repo, branch, pr_url, &payload);
            if !ok {
                return (sid, "failed".into(), err, 1);
            }
            if lifecycle.get("merged") == Some(&json!(true)) {
                process_inputs.insert("merge_already_done".into(), json!(true));
            }
        } else if let Some(url) = pr_url.map(str::trim).filter(|s| !s.is_empty()) {
            if github_pr_merged(url) {
                process_inputs.insert("merge_already_done".into(), json!(true));
            }
        }

        if let Some(inferred) = infer_persist_ref_from_branch(repo, branch) {
            process_inputs.insert("persist_ref".into(), json!(inferred));
        }

        if process_key == "pull-request-review" {
            process_inputs.insert("code_diff".into(), json!("origin/main...HEAD"));
            process_inputs.insert("tasks_path".into(), json!("docs/todos"));
            if let Some(pr) = process_inputs.get("persist_ref").and_then(|v| v.as_str()) {
                process_inputs.insert("document_context".into(), json!(pr));
            }
        }

        std::env::set_var("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF", "0");
        let (status, exit_code) =
            dispatch_process_subscriber(repo, process_key, Value::Object(process_inputs));
        if status == "success" {
            return (sid, status, None, exit_code);
        }
        return (sid, "failed".into(), Some(status), exit_code);
    }

    if subscriber.get("tool").and_then(|v| v.as_str()) == Some("send-telegram-notification") {
        let message = build_telegram_message_from_event(event);
        let Some(message) = message else {
            return (sid, "skipped-empty-message".into(), None, 0);
        };
        match invoke_send_telegram_notification(repo, &message) {
            Ok((true, _)) => return (sid, "success".into(), None, 0),
            Ok((false, body)) => {
                let err = body
                    .get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("send-telegram-notification failed")
                    .to_string();
                return (sid, "failed".into(), Some(err), 1);
            }
            Err(e) => return (sid, "failed".into(), Some(e), 1),
        }
    }

    if subscriber.get("tool").and_then(|v| v.as_str()) == Some("iota-immutable-publisher") {
        let emitter = event.get("emitter_agent").and_then(|v| v.as_str());
        if is_backfill_emitter(emitter) {
            return (sid, "skipped-backfill".into(), None, 0);
        }
        if payload
            .get("dlt_anchor_address")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .is_some()
        {
            return (sid, "skipped-pre-anchored".into(), None, 0);
        }
        let (ok_thresh, reason) = dlt_threshold_ok(event);
        if !ok_thresh {
            return (sid, "skipped-dlt-threshold".into(), Some(reason), 0);
        }
        let (ok, feedback, code, digest) = invoke_iota_publisher(repo, event);
        if ok {
            if let Some(d) = digest {
                if let Some(ds) = event.get_mut("delivery_state").and_then(|v| v.as_object_mut()) {
                    ds.insert("cumulo".to_string(), json!("success"));
                    ds.insert("transaction_digest".to_string(), json!(d));
                }
            }
            return (sid, "success".into(), None, code);
        }
        return (sid, "failed".into(), Some(feedback), code);
    }

    if let Some(action) = subscriber.get("action").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
        if action == "sync-entity-index"
            && matches!(
                std::env::var("SDDIA_LAB_SIMULATE_SYNC_INDEX")
                    .unwrap_or_default()
                    .trim()
                    .to_lowercase()
                    .as_str(),
                "1" | "true" | "yes"
            )
        {
            return (sid, "success".into(), None, 0);
        }
        let Some(payload_obj) = payload.as_object() else {
            return (sid, "failed".into(), Some("payload must be object".into()), 1);
        };
        let inputs = Value::Object(payload_obj.clone());
        if let Ok(Some(data)) = actions::try_run_native(repo, action, &inputs) {
            let ok = data.get("success").and_then(|v| v.as_bool()).unwrap_or(true);
            if ok {
                return (sid, "success".into(), None, 0);
            }
            let err = data
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("action failed")
                .to_string();
            return (sid, "failed".into(), Some(err), 1);
        }
        match invoke_execute_action(repo, action, &inputs) {
            Ok((true, _, _)) => return (sid, "success".into(), None, 0),
            Ok((false, err, code)) => return (sid, "failed".into(), Some(err), code),
            Err(e) => return (sid, "failed".into(), Some(e), 1),
        }
    }

    (
        sid,
        "failed".into(),
        Some("no process/action/tool configured".into()),
        1,
    )
}

fn write_dead_letter_fallback(
    path: &Path,
    event_uuid: &str,
    sid: &str,
    event_type: &str,
    error_trace: &str,
) {
    let _ = write_json_atomic(
        path,
        &json!({
            "event_uuid": event_uuid,
            "subscriber": sid,
            "state": "dead-letter",
            "started_at": super::eda_bus_topology::iso_now(),
            "failed_at": super::eda_bus_topology::iso_now(),
            "error_trace": error_trace,
            "event_type": event_type,
        }),
    );
}

fn handle_subscriber(
    repo: &Path,
    bus: &EventBusTopology,
    subscriber: &Value,
    event: &mut Value,
    event_uuid: &str,
    event_type: &str,
    pending_path: &Path,
    registry: &Value,
    origin_topology: &str,
    dispatch_mode: &str,
) -> (String, String) {
    let sid = subscriber_id(subscriber);
    if terminal_witness_exists(repo, bus, event_uuid, &sid) {
        let existing = super::eda_bus_topology::list_witnesses(
            repo,
            bus,
            "processed_subscribers",
            event_uuid,
        );
        for p in &existing {
            if p.file_name().and_then(|n| n.to_str()) == Some(&format!("{event_uuid}.{sid}.json")) {
                return (sid, "skipped-already-processed".into());
            }
        }
        return (sid, "skipped-already-terminal".into());
    }

    let _ = write_processing_witness(
        repo,
        bus,
        event_uuid,
        &sid,
        event_type,
        dispatch_mode,
    );

    let (_, status, err, exit_code) = dispatch_subscriber(repo, subscriber, event);
    let delegation = delegation_meta(subscriber, exit_code);

    let promote_result = if status_is_terminal_ok(&status) {
        promote_witness(
            repo,
            bus,
            event_uuid,
            &sid,
            "processed",
            Some(&json!({
                "result_status": status,
                "delegation": delegation,
            })),
            Some(pending_path),
        )
    } else {
        promote_witness(
            repo,
            bus,
            event_uuid,
            &sid,
            "dead-letter",
            Some(&json!({
                "error_trace": err.as_deref().unwrap_or(&status),
                "delegation": delegation,
            })),
            Some(pending_path),
        )
    };

    let (sid, status) = if promote_result.is_err() {
        let dead = repo
            .join(&bus.dead_letter_subscribers)
            .join(format!("{event_uuid}.{sid}.json"));
        write_dead_letter_fallback(
            &dead,
            event_uuid,
            &sid,
            event_type,
            &promote_result.err().unwrap_or_default(),
        );
        (sid, "failed".into())
    } else {
        (sid, status)
    };

    let _ = maybe_purge_processing_header(
        repo,
        bus,
        event_uuid,
        registry,
        event_type,
        origin_topology,
    );
    (sid, status)
}

/// Ejecuta enrutamiento ECST/fan-out de un evento de dominio.
pub fn route_domain_event(repo: &Path, event_file_path: &str) -> Value {
    let bus = match ensure_event_bus_topology(repo) {
        Ok(b) => b,
        Err(e) => {
            return json!({
                "success": false,
                "exitCode": 1,
                "data": null,
                "error": e,
            });
        }
    };

    let raw_path = PathBuf::from(event_file_path);
    let event_path = if raw_path.is_absolute() {
        raw_path
    } else {
        repo.join(&raw_path)
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

    let mut event: Value = match fs::read_to_string(&event_path) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                return json!({
                    "success": false,
                    "exitCode": 1,
                    "data": null,
                    "error": format!("invalid event JSON: {e}"),
                });
            }
        },
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
        Some(et) => et.to_string(),
        None => {
            return json!({
                "success": false,
                "exitCode": 1,
                "data": null,
                "error": "event_type missing",
            });
        }
    };

    let event_uuid = match event.get("event_id").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        Some(id) => id.to_string(),
        None => {
            return json!({
                "success": false,
                "exitCode": 1,
                "data": null,
                "error": "event_id missing",
            });
        }
    };

    inject_domain_entity_topology_defaults(&mut event);
    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    let origin_topology = resolve_origin_topology(&payload);

    let schemas = load_event_class_schemas(repo);
    let schema = schemas.get(&event_type);
    let (ecst_ok, ecst_errors) = validate_ecst_instance(&event, schema);
    if !ecst_ok {
        let _ = write_processing_witness(
            repo,
            &bus,
            &event_uuid,
            ECST_GATE_SUBSCRIBER,
            &event_type,
            "sync",
        );
        let _ = promote_witness(
            repo,
            &bus,
            &event_uuid,
            ECST_GATE_SUBSCRIBER,
            "dead-letter",
            Some(&json!({
                "error_trace": ecst_errors.join("; "),
                "ecst_errors": ecst_errors,
            })),
            Some(&event_path),
        );
        return json!({
            "success": false,
            "exitCode": 1,
            "data": {
                "success": false,
                "delivery_status": { ECST_GATE_SUBSCRIBER: "failed" },
                "parent_path": rel_event_path(repo, &event_path),
            },
            "error": ecst_errors.join("; "),
        });
    }

    let processing_header = match ensure_processing_header(repo, &bus, &event_uuid, &event_path) {
        Ok(p) => p,
        Err(e) => {
            return json!({
                "success": false,
                "exitCode": 1,
                "data": null,
                "error": e,
            });
        }
    };

    let subs_path = repo.join(&bus.subscriptions);
    let registry: Value = match fs::read_to_string(&subs_path) {
        Ok(text) => {
            let trimmed = text.strip_prefix('\u{feff}').unwrap_or(&text);
            match serde_json::from_str(trimmed) {
                Ok(v) => v,
                Err(e) => {
                    return json!({
                        "success": false,
                        "exitCode": 1,
                        "data": null,
                        "error": format!("cannot read event-subscriptions.json: {e}"),
                    });
                }
            }
        }
        Err(e) => {
            return json!({
                "success": false,
                "exitCode": 1,
                "data": null,
                "error": format!("cannot read event-subscriptions.json: {e}"),
            });
        }
    };

    let subscribers: Vec<Value> = registry
        .get(&event_type)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter(|sub| subscriber_applies_to_topology(sub, &origin_topology))
                .cloned()
                .collect()
        })
        .unwrap_or_default();

    let dispatch_mode = dispatch_mode_label();
    let mut delivery_status: HashMap<String, String> = HashMap::new();

    if subscribers.is_empty() {
        let sweep = try_sweep_event(repo, &bus, &event_uuid, Some(&registry));
        return json!({
            "success": true,
            "exitCode": 0,
            "data": {
                "success": true,
                "delivery_status": {},
                "parent_path": rel_event_path(repo, &event_path),
                "processing_header_path": rel_event_path(repo, &processing_header),
                "dispatch_mode": dispatch_mode,
                "sweep": sweep,
            },
        });
    }

    let event_arc = Arc::new(Mutex::new(event));
    let repo_path = repo.to_path_buf();
    let bus_clone = EventBusTopology {
        pending: bus.pending.clone(),
        processing: bus.processing.clone(),
        processing_subscribers: bus.processing_subscribers.clone(),
        processed: bus.processed.clone(),
        processed_subscribers: bus.processed_subscribers.clone(),
        dead_letter: bus.dead_letter.clone(),
        dead_letter_subscribers: bus.dead_letter_subscribers.clone(),
        subscriptions: bus.subscriptions.clone(),
    };

    if sync_dispatch_mode() {
        for sub in &subscribers {
            let (sid, status) = handle_subscriber(
                repo,
                &bus,
                sub,
                &mut event_arc.lock().unwrap(),
                &event_uuid,
                &event_type,
                &event_path,
                &registry,
                &origin_topology,
                dispatch_mode,
            );
            delivery_status.insert(sid, status);
        }
    } else {
        let results: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let handles: Vec<_> = subscribers
            .iter()
            .map(|sub| {
                let repo = repo_path.clone();
                let bus = EventBusTopology {
                    pending: bus_clone.pending.clone(),
                    processing: bus_clone.processing.clone(),
                    processing_subscribers: bus_clone.processing_subscribers.clone(),
                    processed: bus_clone.processed.clone(),
                    processed_subscribers: bus_clone.processed_subscribers.clone(),
                    dead_letter: bus_clone.dead_letter.clone(),
                    dead_letter_subscribers: bus_clone.dead_letter_subscribers.clone(),
                    subscriptions: bus_clone.subscriptions.clone(),
                };
                let sub = sub.clone();
                let event = Arc::clone(&event_arc);
                let event_uuid = event_uuid.clone();
                let event_type = event_type.clone();
                let pending = event_path.clone();
                let registry = registry.clone();
                let origin = origin_topology.clone();
                let results = Arc::clone(&results);
                thread::spawn(move || {
                    let (sid, status) = handle_subscriber(
                        &repo,
                        &bus,
                        &sub,
                        &mut event.lock().unwrap(),
                        &event_uuid,
                        &event_type,
                        &pending,
                        &registry,
                        &origin,
                        "async",
                    );
                    results.lock().unwrap().insert(sid, status);
                })
            })
            .collect();
        for h in handles {
            let _ = h.join();
        }
        delivery_status = results.lock().unwrap().clone();
    }

    let skip_only = !delivery_status.is_empty()
        && delivery_status.values().all(|v| v.starts_with("skipped"));
    let all_success = delivery_status.is_empty()
        || delivery_status.values().all(|v| status_is_terminal_ok(v));

    let mut result_data = json!({
        "success": all_success || skip_only,
        "delivery_status": delivery_status,
        "parent_path": rel_event_path(repo, &event_path),
        "processing_header_path": rel_event_path(repo, &processing_header),
        "dispatch_mode": dispatch_mode,
    });

    if let Some(ds) = event_arc.lock().unwrap().get("delivery_state").and_then(|v| v.as_object()) {
        if let Some(d) = ds.get("transaction_digest").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
            result_data["transaction_digest"] = json!(d);
        }
    }

    let mut result = json!({
        "success": all_success || skip_only,
        "exitCode": if all_success || skip_only { 0 } else { 1 },
        "data": result_data,
    });

    if !all_success && !skip_only {
        result["error"] = json!("one or more subscribers failed");
    }

    let sweep = try_sweep_event(repo, &bus, &event_uuid, Some(&registry));
    if let Some(data) = result.get_mut("data").and_then(|v| v.as_object_mut()) {
        data.insert("sweep".to_string(), sweep);
    }

    result
}
