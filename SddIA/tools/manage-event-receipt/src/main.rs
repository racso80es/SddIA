use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::json;
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

fn strip_known_suffixes(path: &Path) -> PathBuf {
    let allowed_suffixes = [".notificado", ".procesado", ".error"];
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    for suffix in allowed_suffixes.iter() {
        if name.ends_with(suffix) {
            let new_name = &name[..name.len() - suffix.len()];
            return path.with_file_name(new_name);
        }
    }
    path.to_path_buf()
}

fn main() {
    let req = read_stdin_json();

    let file_path = match req.get("file_path").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.trim(),
        _ => {
            emit_error("file_path requerido", 1);
            return;
        }
    };

    let suffix = match req.get("suffix").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            emit_error("suffix requerido", 1);
            return;
        }
    };

    let allowed_suffixes = vec![".notificado", ".procesado", ".error"];
    if !allowed_suffixes.contains(&suffix) {
        emit_error(&format!("suffix debe ser uno de {:?}", allowed_suffixes), 1);
        return;
    }

    let repo = match get_repo_root() {
        Ok(r) => r,
        Err(e) => {
            emit_error(&e, 1);
            return;
        }
    };

    let source = repo.join(file_path);
    let canonical_repo = repo.canonicalize().unwrap_or(repo.clone());
    let canonical_source = source.canonicalize().unwrap_or(source.clone());

    if !canonical_source.starts_with(&canonical_repo) {
        emit_error("file_path fuera del workspace", 1);
        return;
    }
    if !canonical_source.is_file() {
        emit_error(&format!("archivo inexistente: {}", file_path), 1);
        return;
    }

    let base = strip_known_suffixes(&canonical_source);
    let target_name = format!("{}{}", base.file_name().and_then(|n| n.to_str()).unwrap_or(""), suffix);
    let target = base.with_file_name(target_name);

    if target.exists() {
        let source_rel = canonical_source.strip_prefix(&canonical_repo).unwrap_or(&canonical_source).to_string_lossy().replace("\\", "/");
        let target_rel = target.strip_prefix(&canonical_repo).unwrap_or(&target).to_string_lossy().replace("\\", "/");
        emit_success(Some(json!({
            "source_path": source_rel,
            "target_path": target_rel,
            "modified": false,
            "message": "Sufijo ya aplicado (idempotente)."
        })));
        return;
    }

    if let Err(e) = fs::rename(&canonical_source, &target) {
        emit_error(&format!("Failed to rename: {}", e), 1);
        return;
    }

    let source_rel = canonical_source.strip_prefix(&canonical_repo).unwrap_or(&canonical_source).to_string_lossy().replace("\\", "/");
    let target_rel = target.strip_prefix(&canonical_repo).unwrap_or(&target).to_string_lossy().replace("\\", "/");

    emit_success(Some(json!({
        "source_path": source_rel,
        "target_path": target_rel,
        "modified": true,
        "message": "Sufijo aplicado."
    })));
}
