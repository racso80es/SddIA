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

fn assert_workspace_bound(repo: &Path, target: &Path, workspace: &Path) -> Result<(), String> {
    let canonical_repo = repo.canonicalize().unwrap_or(repo.to_path_buf());
    let canonical_workspace = workspace.canonicalize().unwrap_or(workspace.to_path_buf());

    if !canonical_workspace.starts_with(&canonical_repo) {
        return Err("workspace_path out of bounds from repo".to_string());
    }

    // In WASI we might not be able to canonicalize target if it doesn't exist yet,
    // but we can check the path prefix.
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
            emit_error("workspace_path obligatorio", 1);
            return;
        }
    };

    let target_name = match req.get("target_file").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.trim(),
        _ => ".io-choke-target",
    };

    let repo = match get_repo_root() {
        Ok(r) => r,
        Err(e) => {
            emit_error(&e, 1);
            return;
        }
    };

    let workspace_path = PathBuf::from(ws_raw);
    let target = workspace_path.join(target_name);

    if let Err(e) = assert_workspace_bound(&repo, &target, &workspace_path) {
        emit_error(&e, 1);
        return;
    }

    if let Some(parent) = target.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if !target.exists() {
        if let Err(_) = fs::write(&target, "io-choke-seed\n") {
            // Ignore error here, we might not have permission anyway
        }
    }

    let mut perms = match fs::metadata(&target) {
        Ok(m) => m.permissions(),
        Err(_) => {
            emit_error("no se pudo leer metadata de archivo", 1);
            return;
        }
    };
    perms.set_readonly(true);
    if let Err(e) = fs::set_permissions(&target, perms) {
        emit_error(&format!("no se pudo marcar read-only: {}", e), 1);
        return;
    }

    let write_result = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&target)
        .and_then(|mut f| {
            use std::io::Write;
            f.write_all(b"choke-attempt\n")
        });

    if write_result.is_ok() {
        emit_error("escritura no bloqueada — io-choke no aplicó asfixia", 2);
    } else {
        let mut out = json!({
            "message": "asfixia E/S simulada",
            "io_choked": true,
            "target": target.to_string_lossy()
        });
        emit_success(Some(out));
    }
}
