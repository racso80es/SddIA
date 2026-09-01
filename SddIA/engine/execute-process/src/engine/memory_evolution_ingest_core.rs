//! Ingesta `Domain_Entity_Telemetry_Captured` → LanceDB (puerto `EvolutionStore`).

use super::fractal_bus::stamp_fractal_delivery_state;
use super::workspace::load_paths_config;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sddia_core_memory::models::evolution_node::{EvolutionEvent, SpatialPolarity};
use sddia_core_memory::ports::EvolutionStore;
use sddia_core_memory::services::inference_binding::LocalHashingEmbedder;
use sddia_core_memory::services::inference_binding::SemanticInference;
use sddia_infrastructure_lancedb_evolution::{LanceDbEvolutionAdapter, TABLE_EVOLUTION};
use std::fs;
use std::path::{Path, PathBuf};

pub const MEMORY_EVOLUTION_INGEST_SUBSCRIBER_KEY: &str = "cumulo.memory-evolution-ingest";
const VECTOR_STORE_DEFAULT: &str = ".SddIA/vector_store";
const LANCEDB_SUBDIR: &str = "lancedb";

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
    vector_store_root(repo).join(LANCEDB_SUBDIR)
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
        SpatialPolarity::EfficientSymmetry
    } else {
        SpatialPolarity::StructuralFracture
    };
    let polarity_label = match polarity {
        SpatialPolarity::EfficientSymmetry => "EfficientSymmetry",
        SpatialPolarity::StructuralFracture => "StructuralFracture",
    };

    let payload_str = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let record_id = if let Some(origin_id) = origin_stimulus_id(&payload) {
        let h = sha256_hex(&origin_id);
        format!("stim-{}", &h[..16.min(h.len())])
    } else {
        sha256_hex(&payload_str)
    };

    let uri = lancedb_uri(repo);
    let adapter = LanceDbEvolutionAdapter::open(&uri).map_err(|e| e.to_string())?;
    if adapter
        .get_event_by_id(&record_id)
        .map_err(|e| e.to_string())?
        .is_some()
    {
        stamp_fractal_delivery_state(
            &event_path,
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

    let mut event = EvolutionEvent {
        id: record_id.clone(),
        polarity,
        payload: payload_str,
        operational_metadata: json!({
            "success": success,
            "entity_id": entity_id,
            "entity_type": payload.get("entity_type"),
            "origin_stimulus": payload.get("origin_stimulus"),
            "duration_ms": metrics.get("duration_ms"),
            "exit_code": metrics.get("exit_code"),
            "source_event_id": body.get("event_id"),
        }),
        embedding: None,
    };
    LocalHashingEmbedder
        .embed_event(&mut event)
        .map_err(|e| e.to_string())?;
    adapter.store_event(event).map_err(|e| e.to_string())?;

    stamp_fractal_delivery_state(
        &event_path,
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
