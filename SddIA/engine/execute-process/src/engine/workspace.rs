//! Workspace dinámico (paridad `workspace_utils`).

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub use crate::core::paths::load_paths_config;

fn documentation_root(cfg: &Value) -> String {
    cfg.get("directories")
        .and_then(|d| d.get("documentation"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_end_matches('/').replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "docs".to_string())
}

pub fn resolve_documentation_features_path(_repo: &Path, cfg: &Value) -> String {
    cfg.get("paths")
        .and_then(|p| p.get("featurePath"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_end_matches('/').replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("{}/features", documentation_root(cfg)))
}

pub fn resolve_documentation_fixes_path(_repo: &Path, cfg: &Value) -> String {
    cfg.get("paths")
        .and_then(|p| p.get("fixPath"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_end_matches('/').replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("{}/fixes", documentation_root(cfg)))
}

pub fn resolve_workspaces_root(repo: &Path, cfg: &Value) -> PathBuf {
    if let Some(root) = cfg
        .get("paths")
        .and_then(|p| p.get("workspacesRoot"))
        .and_then(|v| v.as_str())
    {
        let rel = root.trim().replace('\\', "/");
        return repo.join(rel);
    }
    repo.join(".SddIA/workspaces")
}

pub fn bootstrap_workspace(
    repo: &Path,
    process_name: &str,
    workspace_template: &str,
    process_inputs: &mut Value,
    state: &mut Value,
) -> Result<Value, String> {
    let execution_id = process_inputs
        .get("execution_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| {
            std::env::var("SDDIA_DETACHED_EXECUTION_ID")
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let ws_str = if let Some(existing) = process_inputs.get("workspace_path").and_then(|v| v.as_str()) {
        if !existing.trim().is_empty() {
            let p = PathBuf::from(existing);
            fs::create_dir_all(&p).map_err(|e| e.to_string())?;
            p.to_string_lossy().into_owned()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let ws_str = if ws_str.is_empty() {
        let rel = workspace_template
            .replace("{process_name}", process_name)
            .replace("{execution_id}", &execution_id)
            .replace('\\', "/");
        let ws_path = if rel.starts_with(".SddIA/") || rel.starts_with("SddIA/") {
            repo.join(&rel)
        } else {
            resolve_workspaces_root(repo, &load_paths_config(repo)?).join(&rel)
        };
        fs::create_dir_all(&ws_path).map_err(|e| e.to_string())?;
        ws_path.to_string_lossy().into_owned()
    } else {
        ws_str
    };

    if let Some(obj) = state.as_object_mut() {
        obj.insert("execution_id".into(), Value::String(execution_id.clone()));
        obj.insert("workspace_path".into(), Value::String(ws_str.clone()));
    }
    if let Some(obj) = process_inputs.as_object_mut() {
        obj.insert("execution_id".into(), Value::String(execution_id.clone()));
        obj.insert("workspace_path".into(), Value::String(ws_str.clone()));
    }

    Ok(json!({
        "execution_id": execution_id,
        "workspace_path": ws_str,
        "workspace_template": workspace_template,
    }))
}
