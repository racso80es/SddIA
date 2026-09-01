//! Núcleo orquestador `route-domain-event` nativo (paridad `route_domain_event_core.py`).

use super::actions;
use super::capsules::{invoke_capsule_json, invoke_tool_capsule_json};
use super::ecst_validation::{load_event_class_schemas, validate_ecst_instance};
use super::eda_bus_topology::{
    delegation_meta, dlt_threshold_ok, ensure_event_bus_topology, ensure_processing_header,
    github_pr_merged, inject_domain_entity_topology_defaults, infer_persist_ref_from_branch,
    is_backfill_emitter, is_lab_simulated_pr_url, maybe_purge_processing_header, promote_witness, rel_event_path,
    resolve_origin_topology, resolve_pull_request_lifecycle, subscriber_applies_to_topology,
    subscriber_id, terminal_witness_exists, try_sweep_event, write_json_atomic,
    write_processing_witness, ECST_GATE_SUBSCRIBER, EventBusTopology,
};
use super::invoke_orchestrator::{invoke_process_full, resolve_orchestrator_bin};
use chrono::Utc;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use uuid::Uuid;

const ALLOWLIST_KALMA2: &[&str] = &["bug-fix", "feature", "refactorization", "task-queue-manager"];

/// Acciones de forja documental (Filtro C — L-FRACTURE). No vaciar JSON de suscripciones.
const CONSUMER_SKIP_FORGE_ACTIONS: &[&str] = &[
    "materialize-fracture-pbi",
    "enrich-fracture-pbi-kaizen",
    "materialize-kaizen-alert-doc",
];

fn runtime_profile_is_consumer() -> bool {
    matches!(
        std::env::var("SDDIA_RUNTIME_PROFILE")
            .unwrap_or_default()
            .trim()
            .to_lowercase()
            .as_str(),
        "consumer" | "consumidor"
    )
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

fn recover_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn dispatch_mode_label() -> &'static str {
    if sync_dispatch_mode() {
        "sync"
    } else {
        "async"
    }
}

pub fn input_truthy(inputs: &Value, key: &str) -> bool {
    inputs.get(key).map_or(false, |v| {
        v.as_bool().unwrap_or_else(|| {
            v.as_str()
                .map(|s| matches!(s.trim().to_lowercase().as_str(), "1" | "true" | "yes"))
                .unwrap_or(false)
        })
    })
}

/// Activa `SDDIA_LAB_ROUTE_SYNC=1` durante el scope (modo `--blocking`).
pub struct SyncRouteGuard {
    previous: Option<String>,
}

impl SyncRouteGuard {
    pub fn activate() -> Self {
        let previous = std::env::var("SDDIA_LAB_ROUTE_SYNC").ok();
        std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
        Self { previous }
    }
}

impl Drop for SyncRouteGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(v) => std::env::set_var("SDDIA_LAB_ROUTE_SYNC", v),
            None => std::env::remove_var("SDDIA_LAB_ROUTE_SYNC"),
        }
    }
}

pub fn git_abbrev_ref_head(repo: &Path) -> String {
    Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(repo)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

pub fn subscribers_for_event_type(repo: &Path, event_type: &str) -> Result<Vec<Value>, String> {
    let bus = ensure_event_bus_topology(repo)?;
    let subs_path = repo.join(&bus.subscriptions);
    let text = fs::read_to_string(&subs_path)
        .map_err(|e| format!("cannot read event-subscriptions.json: {e}"))?;
    let trimmed = text.strip_prefix('\u{feff}').unwrap_or(&text);
    let registry: Value = serde_json::from_str(trimmed)
        .map_err(|e| format!("cannot read event-subscriptions.json: {e}"))?;
    Ok(registry
        .get(event_type)
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default())
}

fn fracture_event_content_hash(event_type: &str, payload: &Value) -> String {
    let canonical = json!({
        "event_type": event_type,
        "payload": payload,
    });
    let text = serde_json::to_string(&canonical).unwrap_or_else(|_| "{}".into());
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())[..12].to_string()
}

fn write_pending_domain_event_file(
    _repo: &Path,
    pending_dir: &Path,
    event_type: &str,
    emitter_agent: &str,
    payload: Value,
) -> Result<PathBuf, String> {
    let content_hash = if event_type == "System_Fracture_Detected" {
        Some(fracture_event_content_hash(event_type, &payload))
    } else {
        None
    };
    let event_uuid = Uuid::new_v4().to_string();
    let event = json!({
        "event_id": event_uuid,
        "event_type": event_type,
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": emitter_agent,
        "payload": payload,
        "delivery_state": {},
    });
    let text = serde_json::to_string_pretty(&event).map_err(|e| e.to_string())?;

    if let Some(hash12) = content_hash {
        let target = pending_dir.join(format!("{hash12}.json"));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&target)
        {
            Ok(mut f) => {
                writeln!(f, "{text}").map_err(|e| e.to_string())?;
                f.sync_all().map_err(|e| e.to_string())?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e.to_string()),
        }
        return Ok(target);
    }

    let target = pending_dir.join(format!("{event_uuid}.json"));
    write_json_atomic(&target, &event)?;
    Ok(target)
}

pub fn materialize_pending_domain_event(
    repo: &Path,
    event_type: &str,
    emitter_agent: &str,
    payload: Value,
) -> Result<String, String> {
    let bus = ensure_event_bus_topology(repo)?;
    let pending_dir = repo.join(&bus.pending);
    fs::create_dir_all(&pending_dir).map_err(|e| e.to_string())?;
    let event_path = write_pending_domain_event_file(
        repo,
        &pending_dir,
        event_type,
        emitter_agent,
        payload,
    )?;
    Ok(rel_event_path(repo, &event_path))
}

fn validate_blocking_subscribers(
    repo: &Path,
    event_type: &str,
    target_agent: Option<&str>,
) -> Result<(), String> {
    let subscribers = subscribers_for_event_type(repo, event_type)?;
    if subscribers.is_empty() {
        return Err(format!(
            "blocking: event_type '{event_type}' sin suscriptor válido en event-domain-subscriptions.json"
        ));
    }
    if let Some(target) = target_agent.map(str::trim).filter(|s| !s.is_empty()) {
        let has_agent = subscribers.iter().any(|sub| {
            sub.get("agent")
                .and_then(|v| v.as_str())
                .map(str::trim)
                == Some(target)
        });
        if !has_agent {
            return Err(format!(
                "blocking: agente destino '{target}' no suscripto a '{event_type}'"
            ));
        }
    }
    Ok(())
}

/// Resuelve `event_file_path` desde inputs del orquestador (modo blocking / CLI hook).
pub fn resolve_route_event_path(
    repo: &Path,
    process_inputs: &Value,
    blocking: bool,
) -> Result<String, String> {
    if let Some(rel) = process_inputs
        .get("event_file_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Ok(rel.to_string());
    }

    let event_type = process_inputs
        .get("event_type")
        .or_else(|| process_inputs.get("event"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_file_path o event_type requerido")?;

    if blocking {
        let target = process_inputs
            .get("target")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty());
        validate_blocking_subscribers(repo, event_type, target)?;
    }

    let emitter = process_inputs
        .get("emitter_agent")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("git-hook-pre-push");

    let mut payload = process_inputs
        .get("payload")
        .cloned()
        .unwrap_or_else(|| json!({}));
    if !payload.is_object() {
        payload = json!({});
    }
    if let Some(obj) = payload.as_object_mut() {
        if !obj.contains_key("branch") {
            obj.insert("branch".into(), json!(git_abbrev_ref_head(repo)));
        }
    }

    materialize_pending_domain_event(repo, event_type, emitter, payload)
}

pub fn is_local_qa_event(event: &Value) -> bool {
    event.get("event_type").and_then(|v| v.as_str()) == Some("Local_QA_Requested")
        || event.get("emitter_agent").and_then(|v| v.as_str()) == Some("git-hook-pre-push")
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
        "Process_Execution_Completed" => {
            let pname = payload
                .and_then(|p| p.get("process_name"))
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let cid = payload
                .and_then(|p| p.get("correlation_id"))
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .unwrap_or("sin-correlation_id");
            let st = payload
                .and_then(|p| p.get("status"))
                .and_then(|v| v.as_str())
                .unwrap_or("success");
            let cycle = payload
                .and_then(|p| p.get("cycle_phase"))
                .and_then(|v| v.as_str())
                .unwrap_or("completed");
            Some(format!(
                "PEC {st}/{cycle}: {pname}\ncorrelation_id={cid}"
            ))
        }
        "Email_Triaged" => {
            let verdict = payload
                .and_then(|p| p.get("verdict"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if verdict != "actionable" {
                return None;
            }
            let uid = payload
                .and_then(|p| p.get("message_uid"))
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let from = payload
                .and_then(|p| p.get("from"))
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or("?");
            let subject = payload
                .and_then(|p| p.get("subject"))
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or("(sin asunto)");
            Some(format!(
                "Correo accionable\nfrom={from}\nsubject={subject}\nuid={uid}"
            ))
        }
        _ => None,
    }
}

fn invoke_send_telegram_notification(repo: &Path, message: &str) -> Result<(bool, Value), String> {
    let req = json!({"message": message});
    match invoke_capsule_json(repo, "send-telegram-notification", &req, false) {
        Ok(result)
            if result.exit_code == 0 && result.body.get("success") == Some(&json!(true)) =>
        {
            Ok((true, result.body))
        }
        Ok(result) => {
            let err = result
                .body
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("send-telegram-notification failed");
            Ok((false, json!({"error": err})))
        }
        Err(e) => Ok((false, json!({"error": e}))),
    }
}

fn simulate_iota_enabled() -> bool {
    matches!(
        std::env::var("SDDIA_LAB_SIMULATE_IOTA")
            .unwrap_or_default()
            .trim()
            .to_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

/// Extrae traza humana del envelope cápsula (`error` | `feedback` | `message`).
/// Paridad capsule-json-io / sddia-io::emit_error (campo `error`).
fn capsule_error_trace(body: &Value, ok: bool) -> String {
    body.get("error")
        .or_else(|| body.get("feedback"))
        .or_else(|| body.get("message"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(if ok { "ok" } else { "iota publish failed" })
        .to_string()
}

fn is_valid_iota_anchor(ds: &Map<String, Value>) -> bool {
    if ds.get("merkle_anchored").and_then(|v| v.as_bool()) == Some(true) {
        if let Some(d) = ds.get("transaction_digest").and_then(|v| v.as_str()).map(str::trim) {
            return !d.is_empty() && d != "batched-digest";
        }
        return true;
    }
    if let Some(d) = ds.get("transaction_digest").and_then(|v| v.as_str()).map(str::trim) {
        if !d.is_empty() && d != "batched-digest" {
            return true;
        }
        if d == "batched-digest" {
            return false;
        }
    }
    ds.contains_key("cumulo.iota-immutable-publisher") || ds.contains_key("cumulo")
}

fn resolve_eda_proofs_dir(repo: &Path) -> PathBuf {
    if let Ok(cfg) = super::workspace::load_paths_config(repo) {
        if let Some(p) = cfg
            .get("eda_instance")
            .and_then(|e| e.get("proofs"))
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let pb = PathBuf::from(p);
            return if pb.is_absolute() {
                pb
            } else {
                repo.join(pb)
            };
        }
    }
    repo.join(".SddIA").join("proofs")
}

fn resolve_dlt_reanchor_dir(repo: &Path) -> PathBuf {
    if let Ok(cfg) = super::workspace::load_paths_config(repo) {
        if let Some(p) = cfg
            .get("eda_instance")
            .and_then(|e| e.get("dlt_reanchor"))
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let pb = PathBuf::from(p);
            return if pb.is_absolute() {
                pb
            } else {
                repo.join(pb)
            };
        }
    }
    repo.join(".SddIA").join("dlt").join("reanchor-queue")
}

fn iota_relay_health_url() -> String {
    if let Ok(url) = std::env::var("IOTA_PUBLISH_RELAY_URL") {
        let t = url.trim();
        if !t.is_empty() {
            if let Some(base) = t.strip_suffix("/v1/publish") {
                return format!("{base}/health");
            }
        }
    }
    let host = std::env::var("IOTA_PUBLISH_RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("IOTA_PUBLISH_RELAY_PORT").unwrap_or_else(|_| "8787".into());
    format!("http://{host}:{port}/health")
}

fn iota_relay_health_ok() -> bool {
    let url = iota_relay_health_url();
    let out = Command::new("curl")
        .args(["-sf", "--max-time", "2", &url])
        .output();
    matches!(out, Ok(o) if o.status.success())
}

fn enqueue_dlt_reanchor(repo: &Path, event_uuid: &str, path: &Path, error_trace: &str) {
    let dir = resolve_dlt_reanchor_dir(repo);
    let _ = fs::create_dir_all(&dir);
    let entry = json!({
        "event_uuid": event_uuid,
        "path": path.to_string_lossy(),
        "error_trace": error_trace,
        "queued_at": Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    });
    let target = dir.join(format!("{event_uuid}.json"));
    let _ = write_json_atomic(&target, &entry);
}

fn classify_batch_anchor_friction(causa: &str) -> (&'static str, String) {
    let lower = causa.to_lowercase();
    if causa.contains("capsule-stale-hash") || causa.contains("capsule-stale:") {
        (
            "F-CAPSULA-BINARIO-FOSIL",
            format!("merkle-batch-preseal failed: {causa}"),
        )
    } else if lower.contains("iota-relay-publish-error") {
        (
            "F-DLT-PUBLISH-ERROR",
            format!("merkle-batch-preseal failed: {causa}"),
        )
    } else if lower.contains("iota-relay-unreachable")
        || lower.contains("relay-unreachable")
        || (lower.contains("relay") && lower.contains("health"))
    {
        (
            "F-DLT-RELAY-SIN-SUPERVISOR",
            format!("merkle-batch-preseal failed: {causa}"),
        )
    } else if lower.contains("campo obligatorio") || lower.contains("payload") {
        (
            "F-CAPSULA-CONTRATO-ENTRADA",
            format!("merkle-batch-preseal failed: {causa}"),
        )
    } else {
        (
            "F-DLT-BATCH-ANCHOR",
            format!("merkle-batch-preseal failed: {causa}"),
        )
    }
}

fn emit_dlt_batch_fracture(repo: &Path, causa: &str) {
    let pending_rel = if let Ok(cfg) = super::workspace::load_paths_config(repo) {
        cfg.get("eda_bus")
            .and_then(|b| b.get("pending"))
            .and_then(|v| v.as_str())
            .map(|s| s.trim().trim_start_matches("./").to_string())
            .unwrap_or_else(|| ".events/pending".into())
    } else {
        ".events/pending".into()
    };
    let pending = repo.join(&pending_rel);
    let _ = fs::create_dir_all(&pending);
    let (friction_id, error_trace) = classify_batch_anchor_friction(causa);
    let payload = json!({
        "process_name": "route-domain-event",
        "error_trace": error_trace,
        "agent_emitter": "execute-process",
        "attempted_action": "merkle-batch-preseal",
        "friction_id": friction_id,
    });
    let _ = write_pending_domain_event_file(
        repo,
        &pending,
        "System_Fracture_Detected",
        "execute-process",
        payload,
    );
}

fn stamp_batch_anchor_error(
    repo: &Path,
    bus: &EventBusTopology,
    event_paths_by_uuid: &HashMap<String, PathBuf>,
    uuids: &[String],
    causa: &str,
) {
    for uuid in uuids {
        if let Some(event_path) = event_paths_by_uuid.get(uuid) {
            let event_path = if event_path.is_absolute() {
                event_path.clone()
            } else {
                repo.join(event_path)
            };
            if let Ok(text) = fs::read_to_string(&event_path) {
                if let Ok(mut event) = serde_json::from_str::<Value>(&text) {
                    if let Some(obj) = event.as_object_mut() {
                        if !obj.contains_key("delivery_state") {
                            obj.insert("delivery_state".to_string(), json!({}));
                        }
                        if let Some(ds) =
                            obj.get_mut("delivery_state").and_then(|v| v.as_object_mut())
                        {
                            ds.insert(
                                "last_batch_anchor_error".to_string(),
                                json!(causa),
                            );
                        }
                    }
                    let _ = write_json_atomic(&event_path, &event);
                }
            }
            enqueue_dlt_reanchor(repo, uuid, &event_path, causa);
        } else {
            let fallback = PathBuf::from(&bus.pending).join(format!("{uuid}.json"));
            enqueue_dlt_reanchor(repo, uuid, &fallback, causa);
        }
    }
}

fn bus_dir_abs(repo: &Path, dir: &str) -> PathBuf {
    let p = Path::new(dir);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        repo.join(dir)
    }
}

/// Localiza el JSON del evento: path grabado, luego pending → processed → dead-letter por UUID.
fn resolve_reanchor_event_path(
    repo: &Path,
    bus: &EventBusTopology,
    uuid: &str,
    stored_path: &Path,
) -> Option<PathBuf> {
    let stored = if stored_path.is_absolute() {
        stored_path.to_path_buf()
    } else {
        repo.join(stored_path)
    };
    if stored.is_file() {
        return Some(stored);
    }
    let fname = format!("{uuid}.json");
    for dir in [&bus.pending, &bus.processed, &bus.dead_letter] {
        let p = bus_dir_abs(repo, dir).join(&fname);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn try_drain_dlt_reanchor_queue(repo: &Path, bus: &EventBusTopology) {
    if simulate_iota_enabled() || !iota_relay_health_ok() {
        return;
    }
    let dir = resolve_dlt_reanchor_dir(repo);
    if !dir.is_dir() {
        return;
    }
    let mut queue_files = vec![];
    let mut payloads = vec![];
    let mut uuids = vec![];
    let mut paths_by_uuid: HashMap<String, PathBuf> = HashMap::new();

    let Ok(entries) = fs::read_dir(&dir) else {
        return;
    };
    for ent in entries.flatten() {
        let p = ent.path();
        if p.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&p) else {
            continue;
        };
        let Ok(body) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        let Some(uuid) = body.get("event_uuid").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(path_str) = body.get("path").and_then(|v| v.as_str()) else {
            continue;
        };
        let stored = PathBuf::from(path_str);
        let Some(event_path) = resolve_reanchor_event_path(repo, bus, uuid, &stored) else {
            continue;
        };
        let Ok(evt_text) = fs::read_to_string(&event_path) else {
            continue;
        };
        let Ok(event) = serde_json::from_str::<Value>(&evt_text) else {
            continue;
        };
        if let Some(ds) = event.get("delivery_state").and_then(|v| v.as_object()) {
            if is_valid_iota_anchor(ds) {
                let _ = fs::remove_file(&p);
                continue;
            }
        }
        let payload = event.get("payload").cloned().unwrap_or(json!({}));
        let mut buf = Vec::new();
        let formatter = serde_json::ser::CompactFormatter;
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        if serde::Serialize::serialize(&payload, &mut ser).is_err() {
            continue;
        }
        payloads.push(String::from_utf8(buf).unwrap_or_default());
        uuids.push(uuid.to_string());
        paths_by_uuid.insert(uuid.to_string(), event_path);
        queue_files.push(p);
    }

    if payloads.is_empty() {
        return;
    }

    let payload = json!({
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": payloads,
    });
    let repo_buf = repo.to_path_buf();
    let handle = thread::spawn(move || {
        invoke_tool_capsule_json(&repo_buf, "iota-immutable-publisher", &payload, false)
    });

    let ok = match handle.join() {
        Ok(Ok(result)) => {
            let body = result.body;
            if body.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                apply_merkle_batch_success(repo, bus, &uuids, &paths_by_uuid, &body);
                true
            } else {
                let causa = capsule_error_trace(&body, false);
                stamp_batch_anchor_error(repo, bus, &paths_by_uuid, &uuids, &causa);
                emit_dlt_batch_fracture(repo, &causa);
                false
            }
        }
        Ok(Err(e)) => {
            let causa = format!("iota-invoke: {e}");
            stamp_batch_anchor_error(repo, bus, &paths_by_uuid, &uuids, &causa);
            emit_dlt_batch_fracture(repo, &causa);
            false
        }
        Err(_) => {
            let causa = "iota-thread-panicked";
            stamp_batch_anchor_error(repo, bus, &paths_by_uuid, &uuids, causa);
            emit_dlt_batch_fracture(repo, causa);
            false
        }
    };

    if ok {
        for q in queue_files {
            let _ = fs::remove_file(&q);
        }
    }
}

fn apply_merkle_batch_success(
    repo: &Path,
    _bus: &EventBusTopology,
    uuids: &[String],
    paths_by_uuid: &HashMap<String, PathBuf>,
    body: &Value,
) {
    let digest = body
        .get("result")
        .and_then(|r| r.get("transaction_digest"))
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let merkle_root = body
        .get("result")
        .and_then(|r| r.get("merkle_root"))
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let proofs = body
        .get("result")
        .and_then(|r| r.get("merkle_proofs"))
        .and_then(|v| v.as_array());
    let proofs_dir = resolve_eda_proofs_dir(repo);
    let _ = fs::create_dir_all(&proofs_dir);

    for (i, uuid) in uuids.iter().enumerate() {
        if let Some(proofs) = proofs {
            if let Some(proof) = proofs.get(i) {
                let proof_path = proofs_dir.join(format!("{uuid}.json"));
                let _ = fs::write(
                    &proof_path,
                    serde_json::to_string_pretty(proof).unwrap_or_default(),
                );
            }
        }
        if let Some(event_path) = paths_by_uuid.get(uuid) {
            if let Ok(text) = fs::read_to_string(event_path) {
                if let Ok(mut event) = serde_json::from_str::<Value>(&text) {
                    if let Some(obj) = event.as_object_mut() {
                        if !obj.contains_key("delivery_state") {
                            obj.insert("delivery_state".to_string(), json!({}));
                        }
                        if let Some(ds) = obj.get_mut("delivery_state").and_then(|v| v.as_object_mut()) {
                            ds.insert("cumulo".to_string(), json!("success"));
                            ds.insert("merkle_anchored".to_string(), json!(true));
                            ds.remove("last_batch_anchor_error");
                            if let Some(ref d) = digest {
                                ds.insert("transaction_digest".to_string(), json!(d));
                            }
                            if let Some(ref root) = merkle_root {
                                ds.insert("merkle_root".to_string(), json!(root));
                            }
                        }
                    }
                    let _ = write_json_atomic(event_path, &event);
                }
            }
        }
    }
}

fn batch_anchor_error_trace(event: &Value) -> String {
    let causa = event
        .get("delivery_state")
        .and_then(|ds| ds.get("last_batch_anchor_error"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("batch-missing-merkle-anchor");
    format!("batch-anchor-failed: {causa}")
}

fn lab_sim_iota_digest() -> String {
    format!("lab-sim-{}", &Uuid::new_v4().simple().to_string()[..24])
}

fn stamp_batch_iota_preseal(
    repo: &Path,
    bus: &EventBusTopology,
    event_paths_by_uuid: &HashMap<String, PathBuf>,
    uuids: &[String],
    digest: Option<&str>,
    merkle_root: Option<&str>,
    proofs: Option<&[Value]>,
) {
    let proofs_dir = resolve_eda_proofs_dir(repo);
    if proofs.is_some() {
        let _ = fs::create_dir_all(&proofs_dir);
    }
    for (i, uuid) in uuids.iter().enumerate() {
        if let Some(proofs) = proofs {
            if let Some(proof) = proofs.get(i) {
                let proof_path = proofs_dir.join(format!("{uuid}.json"));
                let _ = fs::write(
                    &proof_path,
                    serde_json::to_string_pretty(proof).unwrap_or_default(),
                );
            }
        }
        let event_path = event_paths_by_uuid
            .get(uuid)
            .cloned()
            .unwrap_or_else(|| PathBuf::from(&bus.pending).join(format!("{uuid}.json")));
        let event_path = if event_path.is_absolute() {
            event_path
        } else {
            repo.join(&event_path)
        };
        if let Ok(text) = fs::read_to_string(&event_path) {
            if let Ok(mut event) = serde_json::from_str::<Value>(&text) {
                if let Some(obj) = event.as_object_mut() {
                    if !obj.contains_key("delivery_state") {
                        obj.insert("delivery_state".to_string(), json!({}));
                    }
                    if let Some(ds) = obj
                        .get_mut("delivery_state")
                        .and_then(|v| v.as_object_mut())
                    {
                        ds.insert("cumulo".to_string(), json!("success"));
                        ds.insert("merkle_anchored".to_string(), json!(true));
                        ds.remove("last_batch_anchor_error");
                        if let Some(d) = digest {
                            ds.insert("transaction_digest".to_string(), json!(d));
                        }
                        if let Some(root) = merkle_root {
                            ds.insert("merkle_root".to_string(), json!(root));
                        }
                    }
                }
                let _ = write_json_atomic(&event_path, &event);
            }
        }
    }
}

/// Pre-sellado Merkle del lote. En lab (`SDDIA_LAB_SIMULATE_IOTA`) no invoca la cápsula.
fn apply_batch_iota_preseal(
    repo: &Path,
    bus: &EventBusTopology,
    payloads_to_anchor: Vec<String>,
    uuids_to_anchor: Vec<String>,
    event_paths_by_uuid: &HashMap<String, PathBuf>,
) {
    if payloads_to_anchor.is_empty() {
        return;
    }
    if simulate_iota_enabled() {
        let digest = lab_sim_iota_digest();
        stamp_batch_iota_preseal(
            repo,
            bus,
            event_paths_by_uuid,
            &uuids_to_anchor,
            Some(&digest),
            None,
            None,
        );
        return;
    }

    let payload = json!({
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": payloads_to_anchor,
    });

    let repo_buf = repo.to_path_buf();
    let handle = std::thread::spawn(move || {
        invoke_tool_capsule_json(&repo_buf, "iota-immutable-publisher", &payload, false)
    });

    match handle.join() {
        Ok(Ok(result)) => {
            let body = result.body;
            if body
                .get("success")
                .and_then(|v: &Value| v.as_bool())
                .unwrap_or(false)
            {
                let digest = body
                    .get("result")
                    .and_then(|r| r.get("transaction_digest"))
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                let merkle_root = body
                    .get("result")
                    .and_then(|r| r.get("merkle_root"))
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                let proofs = body
                    .get("result")
                    .and_then(|r| r.get("merkle_proofs"))
                    .and_then(|v| v.as_array())
                    .cloned();
                stamp_batch_iota_preseal(
                    repo,
                    bus,
                    event_paths_by_uuid,
                    &uuids_to_anchor,
                    digest.as_deref(),
                    merkle_root.as_deref(),
                    proofs.as_deref(),
                );
            } else {
                let causa = capsule_error_trace(&body, false);
                eprintln!("[route_domain_batch] pre-sellado Merkle falló: {causa}");
                stamp_batch_anchor_error(
                    repo,
                    bus,
                    event_paths_by_uuid,
                    &uuids_to_anchor,
                    &causa,
                );
                emit_dlt_batch_fracture(repo, &causa);
            }
        }
        Ok(Err(e)) => {
            let causa = e;
            eprintln!("[route_domain_batch] invoke cápsula: {causa}");
            stamp_batch_anchor_error(
                repo,
                bus,
                event_paths_by_uuid,
                &uuids_to_anchor,
                &causa,
            );
            emit_dlt_batch_fracture(repo, &causa);
        }
        Err(_) => {
            let causa = "iota-thread-panicked".to_string();
            eprintln!("[route_domain_batch] {causa}");
            stamp_batch_anchor_error(
                repo,
                bus,
                event_paths_by_uuid,
                &uuids_to_anchor,
                &causa,
            );
            emit_dlt_batch_fracture(repo, &causa);
        }
    }
}

fn invoke_iota_publisher(repo: &Path, event: &Value) -> (bool, String, i32, Option<String>) {
    if simulate_iota_enabled() {
        return (true, "lab-simulated".into(), 0, Some(lab_sim_iota_digest()));
    }
    let payload = json!({
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": serde_json::to_string(event).unwrap_or_default(),
    });
    let _timeout = iota_timeout_seconds();
    let repo = repo.to_path_buf();
    let handle = thread::spawn(move || {
        invoke_tool_capsule_json(&repo, "iota-immutable-publisher", &payload, false)
    });
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
    let feedback = capsule_error_trace(body, ok);
    (ok, feedback, if ok { 0 } else { result.exit_code }, digest)
}

fn invoke_execute_action(repo: &Path, action: &str, payload: &Value) -> Result<(bool, String, i32), String> {
    match super::capsules::invoke_action(repo, action, payload) {
        Ok(data) => {
            let ok = data.get("success").and_then(|v| v.as_bool()).unwrap_or(true);
            let exit_code = data
                .get("exitCode")
                .and_then(|v| v.as_i64())
                .unwrap_or(if ok { 0 } else { 1 }) as i32;
            let err = data
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("action failed")
                .to_string();
            Ok((ok, err, exit_code))
        }
        Err(e) => Ok((false, e, 1)),
    }
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
        if is_lab_simulated_pr_url(pr_url) {
            let mut lifecycle_obj = lifecycle.clone();
            if let Some(obj) = lifecycle_obj.as_object_mut() {
                obj.insert("lab_simulated".into(), json!(true));
                let diag = obj
                    .entry("diagnostics")
                    .or_insert_with(|| json!([]));
                if let Some(arr) = diag.as_array_mut() {
                    if !arr.iter().any(|v| v.as_str() == Some("lab:simulated-url")) {
                        arr.push(json!("lab:simulated-url"));
                    }
                }
            }
            return (true, None, lifecycle_obj);
        }
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

pub(crate) fn dispatch_subscriber(
    repo: &Path,
    subscriber: &Value,
    event: &mut Value,
    batch_mode_iota: bool,
    event_path: Option<&Path>,
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

        if process_key == "email-triage-gateway"
            || process_key == "email-quick-action-ingest"
            || process_key == "user-preference-ingest"
        {
            let Some(path) = event_path else {
                return (
                    sid,
                    "failed".into(),
                    Some(format!("event_file_path required for {process_key}")),
                    1,
                );
            };
            let rel = super::eda_bus_topology::rel_event_path(repo, path);
            let mut process_inputs = Map::new();
            process_inputs.insert("event_file_path".into(), json!(rel));
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
        // G4: alias runtime Kalma2 / agent_runtime (branch_name).
        process_inputs.insert("branch_name".into(), json!(branch));
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
            if !is_local_qa_event(event) {
                let (ok, err, lifecycle) =
                    pull_request_review_precheck(repo, branch, pr_url, &payload);
                if !ok {
                    if is_lab_simulated_pr_url(pr_url) {
                        return (sid, "skipped-lab-simulated".into(), None, 0);
                    }
                    return (sid, "failed".into(), err, 1);
                }
                if lifecycle.get("merged") == Some(&json!(true)) {
                    process_inputs.insert("merge_already_done".into(), json!(true));
                }
            } else {
                process_inputs.insert("local_qa".into(), json!(true));
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
            let doc_ctx = process_inputs
                .get("persist_ref")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or("docs/features");
            process_inputs.insert("document_context".into(), json!(doc_ctx));
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
        let (ok, feedback, code, digest) = if batch_mode_iota {
            if let Some(ds) = event.get("delivery_state").and_then(|v| v.as_object()) {
                if is_valid_iota_anchor(ds) {
                    let d = ds
                        .get("transaction_digest")
                        .and_then(|v| v.as_str())
                        .map(str::trim)
                        .filter(|s| !s.is_empty() && *s != "batched-digest")
                        .map(str::to_string);
                    (true, "batched-preanchored".into(), 0, d)
                } else {
                    (
                        false,
                        batch_anchor_error_trace(event),
                        1,
                        None,
                    )
                }
            } else {
                (
                    false,
                    batch_anchor_error_trace(event),
                    1,
                    None,
                )
            }
        } else {
            invoke_iota_publisher(repo, event)
        };
        if ok {
            if let Some(ds) = event.get_mut("delivery_state").and_then(|v| v.as_object_mut()) {
                ds.insert("cumulo".to_string(), json!("success"));
                if let Some(d) = digest {
                    let existing = ds
                        .get("transaction_digest")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if existing.is_empty() || existing == "batched-digest" {
                        ds.insert("transaction_digest".to_string(), json!(d));
                    }
                }
            }
            return (sid, "success".into(), None, code);
        }
        return (sid, "failed".into(), Some(feedback), code);
    }

    if subscriber.get("action").and_then(|v| v.as_str()).map(str::trim)
        == Some("persist-pec-correlation-proof")
    {
        match super::persist_pec_correlation_proof::run_from_event(repo, event) {
            Ok(super::persist_pec_correlation_proof::PersistOutcome::Wrote(_)) => {
                return (sid, "success".into(), None, 0);
            }
            Ok(super::persist_pec_correlation_proof::PersistOutcome::SkippedNoCid) => {
                return (sid, "skipped-no-correlation".into(), None, 0);
            }
            Err(e) => return (sid, "failed".into(), Some(e), 1),
        }
    }

    if let Some(action) = subscriber.get("action").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
        if runtime_profile_is_consumer() && CONSUMER_SKIP_FORGE_ACTIONS.contains(&action) {
            return (sid, "skipped-consumer-profile".into(), None, 0);
        }
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
    batch_mode_iota: bool,
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

    let (_, status, err, exit_code) =
        dispatch_subscriber(repo, subscriber, event, batch_mode_iota, Some(pending_path));
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
pub fn route_domain_event(repo: &Path, event_file_path: &str, batch_mode_iota: bool) -> Value {
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
                &mut recover_lock(&event_arc),
                &event_uuid,
                &event_type,
                &event_path,
                &registry,
                &origin_topology,
                dispatch_mode,
                batch_mode_iota,
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
                let sid_fallback = subscriber_id(&sub);
                thread::spawn(move || {
                    let outcome = catch_unwind(AssertUnwindSafe(|| {
                        let mut ev = recover_lock(&event);
                        handle_subscriber(
                            &repo,
                            &bus,
                            &sub,
                            &mut ev,
                            &event_uuid,
                            &event_type,
                            &pending,
                            &registry,
                            &origin,
                            "async",
                            batch_mode_iota,
                        )
                    }));
                    let mut results = recover_lock(&results);
                    match outcome {
                        Ok((sid, status)) => {
                            results.insert(sid, status);
                        }
                        Err(_) => {
                            results.insert(sid_fallback, "failed: subscriber panicked".into());
                        }
                    }
                })
            })
            .collect();
        for h in handles {
            let _ = h.join();
        }
        delivery_status = recover_lock(&results).clone();
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

    {
        let ev = recover_lock(&event_arc);
        if let Some(ds) = ev.get("delivery_state").and_then(|v| v.as_object()) {
            if let Some(d) = ds.get("transaction_digest").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
                result_data["transaction_digest"] = json!(d);
            }
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

#[cfg(test)]
mod blocking_tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        let mut here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        here.pop();
        here.pop();
        here.pop();
        here
    }

    #[test]
    fn capsule_error_trace_prefers_error_field() {
        let body = json!({
            "success": false,
            "error": "iota-publish-unavailable: configure relay",
            "feedback": "ignored-when-error-present"
        });
        let trace = capsule_error_trace(&body, false);
        assert!(trace.starts_with("iota-publish-unavailable"));
        assert_ne!(trace, "iota publish failed");
    }

    #[test]
    fn capsule_error_trace_falls_back_to_feedback() {
        let body = json!({"success": false, "feedback": "config-missing: IOTA_WALLET_SECRET"});
        assert_eq!(
            capsule_error_trace(&body, false),
            "config-missing: IOTA_WALLET_SECRET"
        );
    }

    #[test]
    fn capsule_error_trace_opaque_only_when_empty() {
        let body = json!({"success": false});
        assert_eq!(capsule_error_trace(&body, false), "iota publish failed");
    }

    #[test]
    fn batch_anchor_error_prefers_last_causa() {
        let ev = json!({
            "delivery_state": {
                "last_batch_anchor_error": "iota-relay-unreachable: Connection refused"
            }
        });
        let trace = batch_anchor_error_trace(&ev);
        assert!(trace.starts_with("batch-anchor-failed:"));
        assert!(trace.contains("iota-relay-unreachable"));
        assert!(!trace.ends_with("batch-missing-merkle-anchor"));
    }

    #[test]
    fn batch_anchor_error_fallback_prefixed() {
        let ev = json!({});
        assert_eq!(
            batch_anchor_error_trace(&ev),
            "batch-anchor-failed: batch-missing-merkle-anchor"
        );
    }

    #[test]
    fn simulate_batch_preseal_stamps_lab_sim_without_capsule() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let uuid = "eeeeeeee-5555-4555-8555-eeeeeeeeeeee";
        let pending = repo.join(".events/pending");
        fs::create_dir_all(&pending).unwrap();
        let event_path = pending.join(format!("{uuid}.json"));
        fs::write(
            &event_path,
            r#"{"event_id":"eeeeeeee-5555-4555-8555-eeeeeeeeeeee","event_type":"PullRequest_Merged","delivery_state":{}}"#,
        )
        .unwrap();
        let bus = test_reanchor_bus();
        let mut paths = HashMap::new();
        paths.insert(uuid.to_string(), event_path.clone());
        let prev = std::env::var("SDDIA_LAB_SIMULATE_IOTA").ok();
        std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "1");
        apply_batch_iota_preseal(
            repo,
            &bus,
            vec!["{}".into()],
            vec![uuid.to_string()],
            &paths,
        );
        match prev {
            Some(v) => std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", v),
            None => std::env::remove_var("SDDIA_LAB_SIMULATE_IOTA"),
        }
        let body: Value = serde_json::from_str(&fs::read_to_string(&event_path).unwrap()).unwrap();
        let ds = body["delivery_state"].as_object().expect("delivery_state");
        assert_eq!(ds.get("merkle_anchored"), Some(&json!(true)));
        assert_eq!(ds.get("cumulo"), Some(&json!("success")));
        let digest = ds
            .get("transaction_digest")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        assert!(digest.starts_with("lab-sim-"), "digest={digest}");
        assert!(is_valid_iota_anchor(ds));
    }

    #[test]
    fn resolve_dlt_reanchor_respects_cumulo() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_instance":{"dlt_reanchor":".SddIA/dlt/reanchor-queue"}}"#,
        )
        .unwrap();
        let dir = resolve_dlt_reanchor_dir(repo);
        assert!(dir.ends_with(".SddIA/dlt/reanchor-queue"));
    }

    #[test]
    fn enqueue_dlt_reanchor_writes_queue_file() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_instance":{"dlt_reanchor":".SddIA/dlt/reanchor-queue"}}"#,
        )
        .unwrap();
        let ev_path = repo.join("evt.json");
        fs::write(&ev_path, "{}").unwrap();
        enqueue_dlt_reanchor(repo, "uuid-1", &ev_path, "iota-relay-unreachable: x");
        let q = repo.join(".SddIA/dlt/reanchor-queue/uuid-1.json");
        assert!(q.is_file());
        let body: Value = serde_json::from_str(&fs::read_to_string(&q).unwrap()).unwrap();
        assert_eq!(body["event_uuid"], "uuid-1");
        assert!(body["error_trace"]
            .as_str()
            .unwrap()
            .contains("iota-relay-unreachable"));
    }

    fn test_reanchor_bus() -> EventBusTopology {
        EventBusTopology {
            pending: ".events/pending".into(),
            processing: ".events/processing".into(),
            processing_subscribers: ".events/processing/subscribers".into(),
            processed: ".events/processed".into(),
            processed_subscribers: ".events/processed/subscribers".into(),
            dead_letter: ".events/dead-letter".into(),
            dead_letter_subscribers: ".events/dead-letter/subscribers".into(),
            subscriptions: "SddIA/core/event-domain-subscriptions.json".into(),
        }
    }

    #[test]
    fn resolve_reanchor_keeps_live_pending_path() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let uuid = "aaaaaaaa-1111-4111-8111-aaaaaaaaaaaa";
        let pending = repo.join(".events/pending");
        fs::create_dir_all(&pending).unwrap();
        let live = pending.join(format!("{uuid}.json"));
        fs::write(&live, "{}").unwrap();
        let bus = test_reanchor_bus();
        let got = resolve_reanchor_event_path(repo, &bus, uuid, &live).unwrap();
        assert_eq!(got, live);
    }

    #[test]
    fn resolve_reanchor_finds_processed_when_pending_gone() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let uuid = "bbbbbbbb-2222-4222-8222-bbbbbbbbbbbb";
        let pending = repo.join(".events/pending");
        let processed = repo.join(".events/processed");
        fs::create_dir_all(&pending).unwrap();
        fs::create_dir_all(&processed).unwrap();
        let ghost = pending.join(format!("{uuid}.json"));
        let live = processed.join(format!("{uuid}.json"));
        fs::write(&live, r#"{"payload":{"k":1}}"#).unwrap();
        let bus = test_reanchor_bus();
        let got = resolve_reanchor_event_path(repo, &bus, uuid, &ghost).unwrap();
        assert_eq!(got, live);
    }

    #[test]
    fn resolve_reanchor_finds_dead_letter_when_pending_gone() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let uuid = "cccccccc-3333-4333-8333-cccccccccccc";
        let pending = repo.join(".events/pending");
        let dl = repo.join(".events/dead-letter");
        fs::create_dir_all(&pending).unwrap();
        fs::create_dir_all(&dl).unwrap();
        let ghost = pending.join(format!("{uuid}.json"));
        let live = dl.join(format!("{uuid}.json"));
        fs::write(&live, r#"{"payload":{"k":2}}"#).unwrap();
        let bus = test_reanchor_bus();
        let got = resolve_reanchor_event_path(repo, &bus, uuid, &ghost).unwrap();
        assert_eq!(got, live);
    }

    #[test]
    fn resolve_reanchor_none_when_all_buckets_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let uuid = "dddddddd-4444-4444-8444-dddddddddddd";
        let ghost = repo.join(".events/pending").join(format!("{uuid}.json"));
        let bus = test_reanchor_bus();
        assert!(resolve_reanchor_event_path(repo, &bus, uuid, &ghost).is_none());
    }

    #[test]
    fn classify_publish_error_vs_unreachable() {
        let (publish, _) = classify_batch_anchor_friction(
            "iota-relay-publish-error: status=500 config-missing: IOTA_WALLET_SECRET",
        );
        assert_eq!(publish, "F-DLT-PUBLISH-ERROR");
        let (unreachable, _) =
            classify_batch_anchor_friction("iota-relay-unreachable: Connection refused");
        assert_eq!(unreachable, "F-DLT-RELAY-SIN-SUPERVISOR");
    }

    #[test]
    fn emit_dlt_batch_fracture_publish_error_friction() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_bus":{"pending":".events/pending"}}"#,
        )
        .unwrap();
        emit_dlt_batch_fracture(
            repo,
            "iota-relay-publish-error: status=500 config-missing: IOTA_WALLET_SECRET",
        );
        let pending = repo.join(".events/pending");
        let mut found = false;
        for ent in fs::read_dir(&pending).unwrap() {
            let p = ent.unwrap().path();
            let body: Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
            if body.get("event_type").and_then(|v| v.as_str()) == Some("System_Fracture_Detected")
            {
                assert_eq!(body["payload"]["friction_id"], "F-DLT-PUBLISH-ERROR");
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn emit_dlt_batch_fracture_writes_pending() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_bus":{"pending":".events/pending"}}"#,
        )
        .unwrap();
        emit_dlt_batch_fracture(repo, "iota-relay-unreachable: Connection refused");
        let pending = repo.join(".events/pending");
        let mut found = false;
        for ent in fs::read_dir(&pending).unwrap() {
            let p = ent.unwrap().path();
            let body: Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
            if body.get("event_type").and_then(|v| v.as_str()) == Some("System_Fracture_Detected")
            {
                assert_eq!(
                    body["payload"]["friction_id"],
                    "F-DLT-RELAY-SIN-SUPERVISOR"
                );
                assert_eq!(body["payload"]["attempted_action"], "merkle-batch-preseal");
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn sync_route_guard_sets_and_restores_env() {
        std::env::remove_var("SDDIA_LAB_ROUTE_SYNC");
        {
            let _g = SyncRouteGuard::activate();
            assert_eq!(
                std::env::var("SDDIA_LAB_ROUTE_SYNC").unwrap_or_default(),
                "1"
            );
        }
        assert!(std::env::var("SDDIA_LAB_ROUTE_SYNC").is_err());
    }

    #[test]
    fn validate_blocking_rejects_unknown_event() {
        let repo = repo_root();
        let err = validate_blocking_subscribers(&repo, "Evento_Inexistente_XYZ", None)
            .expect_err("must fail");
        assert!(err.contains("sin suscriptor"));
    }

    #[test]
    fn validate_blocking_rejects_wrong_target_agent() {
        let repo = repo_root();
        let err = validate_blocking_subscribers(&repo, "Local_QA_Requested", Some("agente-fantasma"))
            .expect_err("must fail");
        assert!(err.contains("agente destino"));
    }

    #[test]
    fn local_qa_event_detection() {
        let ev = json!({
            "event_type": "Local_QA_Requested",
            "emitter_agent": "git-hook-pre-push",
        });
        assert!(is_local_qa_event(&ev));
    }

    #[test]
    fn telegram_message_for_pec_includes_correlation() {
        let ev = json!({
            "event_type": "Process_Execution_Completed",
            "payload": {
                "process_name": "feature",
                "correlation_id": "e273713c-dd91-487b-8716-1bdc8c5da741",
                "status": "success",
                "cycle_phase": "completed"
            }
        });
        let msg = build_telegram_message_from_event(&ev).expect("msg");
        assert!(msg.contains("feature"));
        assert!(msg.contains("e273713c-dd91-487b-8716-1bdc8c5da741"));
        assert!(msg.contains("success"));
    }

    #[test]
    fn email_triaged_noise_yields_no_telegram() {
        let ev = json!({
            "event_type": "Email_Triaged",
            "payload": {
                "message_uid": "9",
                "verdict": "noise",
                "from": "list@ex",
                "subject": "newsletter"
            }
        });
        assert!(build_telegram_message_from_event(&ev).is_none());
    }

    #[test]
    fn email_triaged_actionable_builds_poke() {
        let ev = json!({
            "event_type": "Email_Triaged",
            "payload": {
                "message_uid": "42",
                "verdict": "actionable",
                "from": "jefe@corp",
                "subject": "Firma hoy"
            }
        });
        let msg = build_telegram_message_from_event(&ev).expect("msg");
        assert!(msg.contains("Correo accionable"));
        assert!(msg.contains("jefe@corp"));
        assert!(msg.contains("Firma hoy"));
        assert!(msg.contains("uid=42"));
        assert!(!msg.contains("newsletter"));
    }

    #[test]
    fn dispatch_persist_pec_proof_writes_file() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        let cid = "e273713c-dd91-487b-8716-1bdc8c5da741";
        let mut event = json!({
            "event_id": "9ff24776-26c7-4596-8b08-7b6fc4531641",
            "event_type": "Process_Execution_Completed",
            "timestamp": "2026-08-15T12:00:00Z",
            "payload": {
                "correlation_id": cid,
                "process_name": "feature",
                "status": "success",
                "cycle_phase": "completed"
            }
        });
        let sub = json!({
            "agent": "cumulo",
            "action": "persist-pec-correlation-proof"
        });
        let (sid, status, err, code) = dispatch_subscriber(repo, &sub, &mut event, false, None);
        assert_eq!(sid, "cumulo.persist-pec-correlation-proof");
        assert_eq!(status, "success");
        assert!(err.is_none());
        assert_eq!(code, 0);
        assert!(repo
            .join(".SddIA/proofs/pec-correlation")
            .join(format!("{cid}.json"))
            .is_file());
    }

    #[test]
    fn consumer_profile_skips_forge_actions() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        std::env::set_var("SDDIA_RUNTIME_PROFILE", "consumer");
        let mut event = json!({
            "event_id": "00000000-0000-4000-8000-000000000001",
            "event_type": "System_Fracture_Detected",
            "payload": { "process_name": "x", "error_trace": "t" }
        });
        let sub = json!({
            "agent": "cumulo",
            "action": "materialize-fracture-pbi"
        });
        let (_sid, status, err, code) = dispatch_subscriber(repo, &sub, &mut event, false, None);
        std::env::remove_var("SDDIA_RUNTIME_PROFILE");
        assert_eq!(status, "skipped-consumer-profile");
        assert!(err.is_none());
        assert_eq!(code, 0);
    }
}


pub fn route_domain_batch(repo: &std::path::Path, event_file_paths: Vec<String>) -> Value {
    let bus = match ensure_event_bus_topology(repo) {
        Ok(b) => b,
        Err(e) => return json!({ "success": false, "exitCode": 1, "error": e }),
    };

    // L-QUEUE: re-anclaje de cola si /health OK (sin reinyectar a pending/).
    try_drain_dlt_reanchor_queue(repo, &bus);

    let subs_path = repo.join(&bus.subscriptions);
    let registry: Value = fs::read_to_string(&subs_path)
        .ok()
        .and_then(|t| serde_json::from_str(t.strip_prefix('\u{feff}').unwrap_or(&t)).ok())
        .unwrap_or(json!({}));

    let mut payloads_to_anchor = vec![];
    let mut uuids_to_anchor = vec![];
    let mut event_paths_by_uuid: HashMap<String, PathBuf> = HashMap::new();
    let mut results = vec![];

    for path_str in &event_file_paths {
        let raw_path = PathBuf::from(path_str);
        let event_path = if raw_path.is_absolute() {
            raw_path
        } else {
            repo.join(&raw_path)
        };

        let Ok(text) = fs::read_to_string(&event_path) else {
            continue;
        };
        let Ok(event) = serde_json::from_str::<Value>(&text) else {
            continue;
        };

        let Some(event_type) = event.get("event_type").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(event_uuid) = event.get("event_id").and_then(|v| v.as_str()) else {
            continue;
        };
        event_paths_by_uuid.insert(event_uuid.to_string(), event_path.clone());
        let payload = event.get("payload").cloned().unwrap_or(json!({}));
        let origin_topology = resolve_origin_topology(&payload);

        let mut needs_anchor = false;
        let subscribers = registry
            .get(event_type)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for sub in subscribers {
            if subscriber_applies_to_topology(&sub, &origin_topology)
                && sub.get("tool").and_then(|v| v.as_str()) == Some("iota-immutable-publisher")
            {
                needs_anchor = true;
                break;
            }
        }

        if needs_anchor {
            if let Some(ds) = event.get("delivery_state").and_then(|v| v.as_object()) {
                if is_valid_iota_anchor(ds) {
                    needs_anchor = false;
                }
            }
        }

        if needs_anchor {
            let mut buf = Vec::new();
            let formatter = serde_json::ser::CompactFormatter;
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            let _ = serde::Serialize::serialize(&payload, &mut ser);
            let payload_str = String::from_utf8(buf).unwrap_or_default();
            payloads_to_anchor.push(payload_str);
            uuids_to_anchor.push(event_uuid.to_string());
        }
    }

    apply_batch_iota_preseal(
        repo,
        &bus,
        payloads_to_anchor,
        uuids_to_anchor,
        &event_paths_by_uuid,
    );

    for path_str in &event_file_paths {
        let out = route_domain_event(repo, path_str, true);
        results.push(out);

        let raw_path = PathBuf::from(path_str);
        let event_uuid = raw_path.file_stem().unwrap_or_default().to_string_lossy();
        let _sweep = try_sweep_event(repo, &bus, &event_uuid, Some(&registry));
    }

    json!({ "success": true, "exitCode": 0, "data": results })
}
