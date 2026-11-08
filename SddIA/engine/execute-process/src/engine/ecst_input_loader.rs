//! Carga ECST: precedencia `event` en línea → `event_id` (store) → `event_file_path` (id lógico).

use crate::engine::eda_port;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn load_from_store(repo: &Path, event_id: &str, logical_path: &Path) -> Result<Value, String> {
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

fn event_id_from_path(path: &Path) -> Option<String> {
    if path.extension().and_then(|e| e.to_str()) != Some("json") {
        return None;
    }
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn load_from_event_file_path(
    repo: &Path,
    rel: &str,
    explicit_event_id: Option<&str>,
) -> Result<Value, String> {
    let path = resolve_logical_path(repo, rel);
    let event_id = explicit_event_id
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| event_id_from_path(&path))
        .ok_or_else(|| format!("event_id requerido (path={})", path.display()))?;
    load_from_store(repo, &event_id, &path)
}

/// Resuelve ruta relativa al repo y carga el ECST vía store.
pub fn load_ecst_from_rel(repo: &Path, rel: &str) -> Result<Value, String> {
    load_ecst_from_inputs(repo, &json!({ "event_file_path": rel }))
}

/// Precedencia: objeto `event` → `event_id` → `event_file_path`.
pub fn load_ecst_from_inputs(repo: &Path, inputs: &Value) -> Result<Value, String> {
    if let Some(inline) = inputs.get("event") {
        if inline.is_object() {
            if str_field(inline, "event_id").is_some() {
                verify_ecst_matches_bus(repo, inline)?;
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
