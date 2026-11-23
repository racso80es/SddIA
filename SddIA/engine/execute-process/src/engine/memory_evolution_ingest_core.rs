//! Ingesta `Domain_Entity_Telemetry_Captured` → proveedor DI `lancedb-memory-adapter`.

use super::capsules::invoke_tool_capsule_json;
use super::fractal_bus::stamp_fractal_delivery_state_by_id;
use super::workspace::load_paths_config;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

pub const MEMORY_EVOLUTION_INGEST_SUBSCRIBER_KEY: &str = "cumulo.memory-evolution-ingest";
const VECTOR_STORE_DEFAULT: &str = ".SddIA/vector_store";
const LANCEDB_SUBDIR: &str = "lancedb";
const TABLE_EVOLUTION: &str = "evolution";
const LANCEDB_ADAPTER_TOOL: &str = "lancedb-memory-adapter";

pub trait EvolutionIngestPort {
    fn ingest_telemetry(
        &self,
        repo: &Path,
        record_id: &str,
        polarity: &str,
        payload: &str,
        operational_metadata: &Value,
    ) -> Result<Value, String>;
}

pub struct CapsuleEvolutionIngestPort;

impl EvolutionIngestPort for CapsuleEvolutionIngestPort {
    fn ingest_telemetry(
        &self,
        repo: &Path,
        record_id: &str,
        polarity: &str,
        payload: &str,
        operational_metadata: &Value,
    ) -> Result<Value, String> {
        let req = json!({
            "request": {
                "operation": "evolution.ingest_telemetry",
                "repository_path": repo.display().to_string(),
                "record_id": record_id,
                "polarity": polarity,
                "payload": payload,
                "operational_metadata": operational_metadata,
            }
        });
        let cap = invoke_tool_capsule_json(repo, LANCEDB_ADAPTER_TOOL, &req, false)?;
        if cap.exit_code != 0 || cap.body.get("success") == Some(&json!(false)) {
            let msg = cap
                .body
                .get("error")
                .and_then(|v| v.as_str())
                .or_else(|| cap.body.get("message").and_then(|v| v.as_str()))
                .unwrap_or("lancedb-memory-adapter failed");
            return Err(msg.to_string());
        }
        Ok(cap.body.get("result").cloned().unwrap_or_else(|| json!({})))
    }
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

pub fn vector_store_root(repo: &Path) -> PathBuf {
    load_paths_config(repo)
        .ok()
        .and_then(|cfg| {
            cfg.get("paths")
                .and_then(|p| p.get("vectorStore"))
                .and_then(|v| v.as_str())
                .map(|s| {
                    let rel = s.trim().trim_start_matches("./");
                    repo.join(rel)
                })
        })
        .unwrap_or_else(|| repo.join(VECTOR_STORE_DEFAULT))
}

pub fn lancedb_uri(repo: &Path) -> PathBuf {
    if let Ok(cfg) = load_paths_config(repo) {
        if let Some(rel) = cfg
            .pointer("/paths/lancedbStore")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            return repo.join(rel.trim_start_matches("./"));
        }
    }
    vector_store_root(repo).join(LANCEDB_SUBDIR)
}

/// Procesa un evento domain en ruta relativa; fail-soft hacia el caller vía `ok`.
pub fn ingest_domain_event_file(repo: &Path, rel_path: &str) -> Value {
    ingest_domain_event_file_with_port(repo, rel_path, &CapsuleEvolutionIngestPort)
}

pub fn ingest_domain_event_file_with_port(
    repo: &Path,
    rel_path: &str,
    port: &dyn EvolutionIngestPort,
) -> Value {
    match ingest_inner(repo, rel_path, port) {
        Ok(v) => v,
        Err(e) => json!({"ok": false, "error": e}),
    }
}

/// Entrada de proceso (`event` / `event_id` / `event_uri`); AUT-57.
pub fn ingest_from_process_inputs(repo: &Path, inputs: &Value) -> Value {
    ingest_from_process_inputs_with_port(repo, inputs, &CapsuleEvolutionIngestPort)
}

pub fn ingest_from_process_inputs_with_port(
    repo: &Path,
    inputs: &Value,
    port: &dyn EvolutionIngestPort,
) -> Value {
    match ingest_from_inputs(repo, inputs, port) {
        Ok(v) => v,
        Err(e) => json!({"ok": false, "error": e}),
    }
}

fn ingest_from_inputs(
    repo: &Path,
    inputs: &Value,
    port: &dyn EvolutionIngestPort,
) -> Result<Value, String> {
    let body = super::ecst_input_loader::load_ecst_from_inputs(repo, inputs)?;
    let event_ref = inputs
        .get("event_uri")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| {
            body.get("event_id")
                .and_then(|v| v.as_str())
                .map(|id| format!("eda://domain/{id}"))
                .unwrap_or_else(|| "unknown".into())
        });
    ingest_ecst_body_with_port(repo, &body, &event_ref, port)
}

fn ingest_inner(
    repo: &Path,
    rel_path: &str,
    port: &dyn EvolutionIngestPort,
) -> Result<Value, String> {
    let body = super::ecst_input_loader::load_ecst_from_rel(repo, rel_path)?;
    ingest_ecst_body_with_port(repo, &body, rel_path, port)
}

fn resolve_event_id(body: &Value, event_ref: &str) -> String {
    if let Some(id) = body
        .get("event_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return id.to_string();
    }
    if let Ok(uri) = sddia_core::ports::event_manager::EventUri::parse(event_ref) {
        return uri.event_id;
    }
    Path::new(event_ref.trim())
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn ingest_ecst_body_with_port(
    repo: &Path,
    body: &Value,
    event_ref: &str,
    port: &dyn EvolutionIngestPort,
) -> Result<Value, String> {
    let event_id = resolve_event_id(body, event_ref);

    let event_type = body
        .get("event_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if event_type != "Domain_Entity_Telemetry_Captured" {
        stamp_fractal_delivery_state_by_id(
            repo,
            &event_id,
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
    let polarity_label = if success {
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

    let op_meta = json!({
        "success": success,
        "entity_id": entity_id,
        "entity_type": payload.get("entity_type"),
        "origin_stimulus": payload.get("origin_stimulus"),
        "duration_ms": metrics.get("duration_ms"),
        "exit_code": metrics.get("exit_code"),
        "source_event_id": body.get("event_id"),
    });

    let res = port.ingest_telemetry(repo, &record_id, polarity_label, &payload_str, &op_meta)?;
    if res.get("skipped").and_then(|v| v.as_str()) == Some("already_indexed") {
        stamp_fractal_delivery_state_by_id(
            repo,
            &event_id,
            MEMORY_EVOLUTION_INGEST_SUBSCRIBER_KEY,
            "skipped",
        );
        return Ok(json!({
            "ok": true,
            "skipped": "already_indexed",
            "record_id": record_id,
            "store_backend": "lancedb",
            "table": TABLE_EVOLUTION,
        }));
    }

    stamp_fractal_delivery_state_by_id(
        repo,
        &event_id,
        MEMORY_EVOLUTION_INGEST_SUBSCRIBER_KEY,
        "success",
    );

    Ok(json!({
        "ok": true,
        "record_id": record_id,
        "entity_id": entity_id,
        "polarity": polarity_label,
        "store_backend": "lancedb",
        "table": TABLE_EVOLUTION,
        "purged": false,
    }))
}
