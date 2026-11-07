//! Carga ECST: precedencia `event` en línea → `event_id` (store) → `event_file_path` (lab).

use crate::engine::eda_port;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn load_from_store(repo: &Path, event_id: &str, logical_path: &Path) -> Result<Value, String> {
    eda_port::install_productive_port(repo)?;
    eda_port::load_dispatched(repo, event_id, logical_path)?
        .ok_or_else(|| format!("event not found in store: {event_id}"))
}

fn resolve_logical_path(repo: &Path, rel: &str) -> PathBuf {
    let trimmed = rel.trim_start_matches("./");
    let raw = PathBuf::from(trimmed);
    if raw.is_absolute() {
        raw
    } else {
        repo.join(trimmed)
    }
}

fn load_from_event_file_path(
    repo: &Path,
    rel: &str,
    explicit_event_id: Option<&str>,
) -> Result<Value, String> {
    let path = resolve_logical_path(repo, rel);
    if path.extension().and_then(|e| e.to_str()) == Some("json") && path.is_file() {
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        return serde_json::from_str(&text).map_err(|e| e.to_string());
    }
    if let Some(eid) = explicit_event_id.filter(|s| !s.is_empty()) {
        return load_from_store(repo, eid, &path);
    }
    if path.extension().and_then(|e| e.to_str()) == Some("json") {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| format!("event file not found: {}", path.display()))?;
        return load_from_store(repo, stem, &path);
    }
    Err(format!(
        "event_id requerido para cargar desde store (path={})",
        path.display()
    ))
}

/// Resuelve ruta relativa al repo y carga el ECST (FS o store).
pub fn load_ecst_from_rel(repo: &Path, rel: &str) -> Result<Value, String> {
    load_ecst_from_inputs(repo, &json!({ "event_file_path": rel }))
}

/// Precedencia: objeto `event` → `event_id` → `event_file_path`.
pub fn load_ecst_from_inputs(repo: &Path, inputs: &Value) -> Result<Value, String> {
    if let Some(inline) = inputs.get("event") {
        if inline.is_object() {
            if str_field(inline, "event_id").is_some() {
                return Ok(inline.clone());
            }
        }
    }
    if let Some(event_id) = str_field(inputs, "event_id") {
        let synthetic = repo
            .join(".events/pending")
            .join(format!("{event_id}.json"));
        return load_from_store(repo, &event_id, &synthetic);
    }
    if let Some(rel) = str_field(inputs, "event_file_path") {
        return load_from_event_file_path(repo, &rel, None);
    }
    Err("event, event_id o event_file_path requerido".into())
}

/// Carga para handlers con tracker remoto: recarga desde bus si el inline no coincide.
pub fn load_ecst_for_handler(repo: &Path, inputs: &Value) -> Result<Value, String> {
    if inputs.get("event").is_some_and(|v| v.is_object()) {
        return load_ecst_from_inputs(repo, inputs);
    }
    if let Some(rel) = str_field(inputs, "event_file_path") {
        let path = resolve_logical_path(repo, &rel);
        if path.is_file() {
            return load_ecst_from_inputs(repo, inputs);
        }
    }
    eda_port::install_productive_port(repo)?;
    let event = load_ecst_from_inputs(repo, inputs)?;
    if verify_ecst_matches_bus(repo, &event).is_ok() {
        return Ok(event);
    }
    let eid = str_field(&event, "event_id").ok_or("event_id ausente")?;
    let synthetic = repo.join(".events/pending").join(format!("{eid}.json"));
    let stored = load_from_store(repo, &eid, &synthetic)?;
    verify_ecst_matches_bus(repo, &stored)?;
    Ok(stored)
}

/// Antes de efecto remoto: el ECST debe existir en bus y coincidir.
pub fn verify_ecst_matches_bus(repo: &Path, event: &Value) -> Result<(), String> {
    eda_port::install_productive_port(repo)?;
    let event_id = str_field(event, "event_id").ok_or("event_id ausente")?;
    let synthetic = repo
        .join(".events/pending")
        .join(format!("{event_id}.json"));
    let stored = eda_port::load_dispatched(repo, &event_id, &synthetic)?;
    let Some(stored) = stored else {
        return Err("ecst_not_in_bus".into());
    };
    if stored != *event {
        return Err("ecst_mismatch".into());
    }
    Ok(())
}
