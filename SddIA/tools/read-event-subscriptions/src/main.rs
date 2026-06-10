use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn get_repo_root() -> Result<PathBuf, String> {
    let current_exe = env::current_exe().map_err(|e| format!("Failed to get current exe: {}", e))?;
    let mut current_dir = current_exe.parent().unwrap_or(Path::new(""));
    loop {
        if current_dir.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(current_dir.to_path_buf());
        }
        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            return Err("No se encontró raíz del workspace".to_string());
        }
    }
}

fn main() {
    let req = read_stdin_json();
    let event_type = req.get("event_type").and_then(|v| v.as_str());

    let repo = match get_repo_root() {
        Ok(r) => r,
        Err(e) => {
            emit_error(&e, 1);
            return;
        }
    };

    let cumulo_path = repo.join("SddIA/core/cumulo.paths.json");
    let cumulo_str = match fs::read_to_string(&cumulo_path) {
        Ok(s) => s,
        Err(e) => {
            emit_error(&format!("Failed to read cumulo.paths.json: {}", e), 1);
            return;
        }
    };

    let cumulo: Value = match serde_json::from_str(&cumulo_str) {
        Ok(v) => v,
        Err(e) => {
            emit_error(&format!("Failed to parse cumulo.paths.json: {}", e), 1);
            return;
        }
    };

    let rel = cumulo.get("eda_bus")
        .and_then(|v| v.get("subscriptions"))
        .and_then(|v| v.as_str())
        .or_else(|| cumulo.get("normative_documents").and_then(|v| v.get("event_subscriptions")).and_then(|v| v.as_str()))
        .unwrap_or("SddIA/core/event-subscriptions.json");

    let subs_path = repo.join(rel);
    if !subs_path.is_file() {
        emit_error(&format!("SSOT inexistente: {}", rel), 1);
        return;
    }

    let subs_str = match fs::read_to_string(&subs_path) {
        Ok(s) => s,
        Err(e) => {
            emit_error(&format!("No se pudo leer suscripciones: {}", e), 1);
            return;
        }
    };

    let registry: Value = match serde_json::from_str(subs_str.trim_start_matches('\u{feff}')) {
        Ok(v) => v,
        Err(e) => {
            emit_error(&format!("No se pudo parsear suscripciones: {}", e), 1);
            return;
        }
    };

    if event_type.is_none() {
        emit_success(Some(json!({
            "message": "Registro completo cargado.",
            "subscriptions_path": rel.replace("\\", "/"),
            "registry": registry
        })));
        return;
    }

    let event_type_str = event_type.unwrap().trim();
    if event_type_str.is_empty() {
        emit_error("event_type debe ser string no vacío si se proporciona", 1);
        return;
    }

    let subscribers = match registry.get(event_type_str) {
        Some(Value::Array(arr)) => arr.clone(),
        Some(_) => {
            emit_error(&format!("Entrada para {} no es array", event_type_str), 1);
            return;
        }
        None => Vec::new(),
    };

    emit_success(Some(json!({
        "message": format!("Suscriptores para {}.", event_type_str),
        "subscriptions_path": rel.replace("\\", "/"),
        "event_type": event_type_str,
        "subscribers": subscribers
    })));
}
