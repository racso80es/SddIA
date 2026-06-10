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

fn load_eda_bus(repo: &Path) -> Result<Value, String> {
    let cumulo_path = repo.join("SddIA/core/cumulo.paths.json");
    let content = fs::read_to_string(&cumulo_path).map_err(|e| format!("Failed to read cumulo: {}", e))?;
    let cumulo: Value = serde_json::from_str(&content).map_err(|e| format!("Failed to parse cumulo: {}", e))?;
    let mut default_bus = json!({
        "pending": ".events/pending",
        "processing": ".events/processing",
        "processed": ".events/processed",
        "dead_letter": ".events/dead_letter"
    });
    if let Some(obj) = cumulo.get("eda_bus").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            default_bus.as_object_mut().unwrap().insert(k.clone(), v.clone());
        }
    }

    // Check overrides
    if let Ok(p) = env::var("SDDIA_EDA_BUS_PENDING") { default_bus.as_object_mut().unwrap().insert("pending".to_string(), json!(p)); }
    if let Ok(p) = env::var("SDDIA_EDA_BUS_PROCESSING") { default_bus.as_object_mut().unwrap().insert("processing".to_string(), json!(p)); }
    if let Ok(p) = env::var("SDDIA_EDA_BUS_PROCESSED") { default_bus.as_object_mut().unwrap().insert("processed".to_string(), json!(p)); }
    if let Ok(p) = env::var("SDDIA_EDA_BUS_DEAD_LETTER") { default_bus.as_object_mut().unwrap().insert("dead_letter".to_string(), json!(p)); }

    Ok(default_bus)
}

fn ensure_event_bus_topology(repo: &Path) -> Result<Value, String> {
    let bus = load_eda_bus(repo)?;
    if let Some(obj) = bus.as_object() {
        for v in obj.values() {
            if let Some(rel) = v.as_str() {
                let _ = fs::create_dir_all(repo.join(rel));
            }
        }
    }
    Ok(bus)
}

fn main() {
    let req = read_stdin_json();

    let file_name = match req.get("file_name").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.trim(),
        _ => { emit_error("file_name requerido", 1); return; }
    };
    let from_bucket = match req.get("from_bucket").and_then(|v| v.as_str()) {
        Some(s) => s.trim(),
        None => { emit_error("from_bucket requerido", 1); return; }
    };
    let to_bucket = match req.get("to_bucket").and_then(|v| v.as_str()) {
        Some(s) => s.trim(),
        None => { emit_error("to_bucket requerido", 1); return; }
    };

    let valid_buckets = vec!["pending", "processing", "processed", "dead_letter"];
    if !valid_buckets.contains(&from_bucket) || !valid_buckets.contains(&to_bucket) {
        emit_error(&format!("from_bucket y to_bucket deben ser uno de {:?}", valid_buckets), 1);
        return;
    }

    let repo = match get_repo_root() {
        Ok(r) => r,
        Err(e) => { emit_error(&e, 1); return; }
    };

    let bus = match ensure_event_bus_topology(&repo) {
        Ok(b) => b,
        Err(e) => { emit_error(&e, 1); return; }
    };

    let from_rel = bus.get(from_bucket).and_then(|v| v.as_str()).unwrap_or("");
    let to_rel = bus.get(to_bucket).and_then(|v| v.as_str()).unwrap_or("");

    let source = repo.join(from_rel).join(file_name);
    let dest_dir = repo.join(to_rel);
    let dest = dest_dir.join(file_name);

    let canonical_repo = repo.canonicalize().unwrap_or(repo.clone());
    let canonical_source = source.canonicalize().unwrap_or(source.clone());

    // We can't canonicalize dest if it doesn't exist.
    if !canonical_source.starts_with(&canonical_repo) {
        emit_error("ruta fuera del workspace", 1);
        return;
    }
    if !canonical_source.is_file() {
        emit_error(&format!("origen inexistente: {}", source.display()), 1);
        return;
    }

    let _ = fs::create_dir_all(&dest_dir);

    if dest.is_file() {
        let dest_rel = dest.strip_prefix(&canonical_repo).unwrap_or(&dest).to_string_lossy().replace("\\", "/");
        emit_success(Some(json!({
            "message": "Destino ya existe (idempotente).",
            "target_path": dest_rel,
            "modified": false
        })));
        return;
    }

    if let Err(e) = fs::rename(&canonical_source, &dest) {
        emit_error(&format!("Failed to move: {}", e), 1);
        return;
    }

    let dest_rel = dest.strip_prefix(&canonical_repo).unwrap_or(&dest).to_string_lossy().replace("\\", "/");
    emit_success(Some(json!({
        "message": "Tránsito completado.",
        "target_path": dest_rel,
        "modified": true
    })));
}
