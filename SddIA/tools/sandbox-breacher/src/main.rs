use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

fn get_repo_root() -> Result<PathBuf, String> {
    let mut current_dir = std::env::current_dir().map_err(|e| format!("Failed to get current dir: {}", e))?;
    loop {
        if current_dir.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(current_dir);
        }
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            return Err("No se encontró raíz del workspace".to_string());
        }
    }
}

fn assert_workspace_bound(repo: &Path, target: &Path, workspace: &Path) -> Result<(), String> {
    let canonical_repo = repo.canonicalize().unwrap_or(repo.to_path_buf());
    let canonical_workspace = workspace.canonicalize().unwrap_or(workspace.to_path_buf());

    if !canonical_workspace.starts_with(&canonical_repo) {
        return Err("workspace_path out of bounds from repo".to_string());
    }

    let target_norm = target.components().collect::<PathBuf>();
    let workspace_norm = workspace.components().collect::<PathBuf>();
    if !target_norm.starts_with(&workspace_norm) {
        return Err("target fuera de workspace_path".to_string());
    }
    Ok(())
}

fn main() {
    let req = read_stdin_json();

    let ws_raw = match req.get("workspace_path").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.trim(),
        _ => {
            let mut response = json!({
                "success": false,
                "exitCode": 1,
                "message": "workspace_path obligatorio",
                "error": "workspace_path obligatorio",
                "result": {"breach_blocked": true}
            });
            emit_success(Some(response));
            return;
        }
    };

    let escape = match req.get("escape_target").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.trim(),
        _ => "../breach-marker.txt",
    };

    let repo = match get_repo_root() {
        Ok(r) => r,
        Err(e) => {
            let response = json!({
                "success": false,
                "exitCode": 1,
                "message": e.clone(),
                "error": e,
                "result": {"breach_blocked": true}
            });
            emit_success(Some(response));
            return;
        }
    };

    let workspace_path = PathBuf::from(ws_raw);

    // Simplistic resolution for `../` since we can't easily use canonicalize if paths don't exist
    // However Rust's canonicalize fails if path does not exist.
    // Instead we can push the path and canonicalize its parent, or just do a string-based normalization
    // A simplified approach that matches WASI sandboxing inherently anyway.

    let mut target = workspace_path.clone();
    for component in Path::new(escape).components() {
        match component {
            std::path::Component::ParentDir => { target.pop(); }
            std::path::Component::Normal(c) => { target.push(c); }
            _ => {}
        }
    }

    if let Err(e) = assert_workspace_bound(&repo, &target, &workspace_path) {
        let response = json!({
            "success": false,
            "exitCode": 1,
            "message": e.clone(),
            "error": e,
            "result": {"breach_blocked": true}
        });
        emit_success(Some(response));
        return;
    }

    if let Some(parent) = target.parent() {
        let _ = fs::create_dir_all(parent);
    }

    match fs::write(&target, "breach\n") {
        Ok(_) => {
            emit_success(Some(json!({
                "message": "escritura dentro del workspace",
                "breach_blocked": false,
                "target": target.to_string_lossy()
            })));
        }
        Err(e) => {
            let response = json!({
                "success": false,
                "exitCode": 1,
                "message": format!("escritura fallida: {}", e),
                "error": format!("escritura fallida: {}", e),
                "result": {"breach_blocked": true}
            });
            emit_success(Some(response));
        }
    }
}
