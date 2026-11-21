//! Registro incremental write-ahead + índice active-executions (MAN-34 / KA-STOP-4).

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

const ACTIVE_EXECUTIONS_REL: &str = ".SddIA/state/active-executions.json";

/// Normaliza `stop_after` (lista o string legacy) → `Vec<String>`.
pub fn normalize_stop_after(value: &Value) -> Result<Vec<String>, String> {
    match value {
        Value::Null => Ok(Vec::new()),
        Value::String(s) => {
            let t = s.trim();
            if t.is_empty() {
                Ok(Vec::new())
            } else {
                Ok(vec![t.to_string()])
            }
        }
        Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for (i, item) in arr.iter().enumerate() {
                let s = item
                    .as_str()
                    .ok_or_else(|| format!("stop_after[{i}] debe ser string"))?
                    .trim();
                if !s.is_empty() {
                    out.push(s.to_string());
                }
            }
            Ok(out)
        }
        _ => Err("stop_after debe ser lista de strings o string legacy".into()),
    }
}

/// Lee `stop_after` desde inputs (ausente → `[]`).
pub fn stop_after_from_inputs(inputs: &Value) -> Result<Vec<String>, String> {
    match inputs.get("stop_after") {
        None => Ok(Vec::new()),
        Some(v) => normalize_stop_after(v),
    }
}

pub fn resume_requested(inputs: &Value) -> bool {
    inputs
        .get("resume")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

fn atomic_write_json(path: &Path, value: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let tmp = path.with_extension("json.tmp");
    let body = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    fs::write(&tmp, &body).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    Ok(())
}

/// Persistencia atómica de `{workspace}/phase_reports.json`.
pub fn persist_phase_reports_atomic(workspace_path: &str, reports: &[Value]) -> Result<(), String> {
    let ws = workspace_path.trim();
    if ws.is_empty() {
        return Ok(());
    }
    let path = Path::new(ws).join("phase_reports.json");
    atomic_write_json(&path, &Value::Array(reports.to_vec()))
}

pub fn load_phase_reports(workspace_path: &str) -> Result<Option<Vec<Value>>, String> {
    let ws = workspace_path.trim();
    if ws.is_empty() {
        return Ok(None);
    }
    let path = Path::new(ws).join("phase_reports.json");
    if !path.is_file() {
        return Ok(None);
    }
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    match value {
        Value::Array(arr) => Ok(Some(arr)),
        _ => Err("phase_reports.json no es un array".into()),
    }
}

fn active_executions_path(repo: &Path) -> PathBuf {
    repo.join(ACTIVE_EXECUTIONS_REL)
}

fn load_active_index(repo: &Path) -> Result<Value, String> {
    let path = active_executions_path(repo);
    if !path.is_file() {
        return Ok(json!({"version": 1, "executions": {}}));
    }
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut value: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    if value.get("version").is_none() {
        value["version"] = json!(1);
    }
    if value.get("executions").is_none() {
        value["executions"] = json!({});
    }
    Ok(value)
}

pub fn upsert_active_execution(
    repo: &Path,
    execution_id: &str,
    entry: Value,
) -> Result<(), String> {
    let eid = execution_id.trim();
    if eid.is_empty() {
        return Ok(());
    }
    let mut index = load_active_index(repo)?;
    let executions = index
        .get_mut("executions")
        .and_then(|v| v.as_object_mut())
        .ok_or_else(|| "active-executions.json inválido".to_string())?;
    executions.insert(eid.to_string(), entry);
    atomic_write_json(&active_executions_path(repo), &index)
}

pub fn lookup_active_execution(repo: &Path, execution_id: &str) -> Result<Option<Value>, String> {
    let eid = execution_id.trim();
    if eid.is_empty() {
        return Ok(None);
    }
    let index = load_active_index(repo)?;
    Ok(index
        .pointer(&format!("/executions/{eid}"))
        .cloned())
}

pub fn active_execution_entry(
    process_name: &str,
    workspace_path: &str,
    persist_ref: Option<&str>,
    status: &str,
    stop_after: &[String],
) -> Value {
    let updated_at = chrono_like_now();
    json!({
        "process_name": process_name,
        "workspace_path": workspace_path,
        "persist_ref": persist_ref,
        "status": status,
        "stop_after": stop_after,
        "updated_at": updated_at,
    })
}

fn chrono_like_now() -> String {
    // ISO-8601 aproximado sin dependencia chrono: SystemTime → millis.
    use std::time::{SystemTime, UNIX_EPOCH};
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{ms}")
}

/// Verifica artefactos de diseño bajo persist_ref (spec.md + plan.md si aplica).
pub fn design_artifacts_ready(repo: &Path, inputs: &Value, require_plan: bool) -> bool {
    let Some(pref) = inputs.get("persist_ref").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) else {
        return false;
    };
    let base = repo.join(pref);
    if !base.join("spec.md").is_file() {
        return false;
    }
    if require_plan && !base.join("plan.md").is_file() {
        return false;
    }
    true
}

pub fn phase_id_of(phase: &Value) -> Option<&str> {
    phase
        .get("id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

pub fn is_valid_phase_id(id: &str) -> bool {
    let mut chars = id.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    #[test]
    fn normalize_stop_after_legacy_string() {
        assert_eq!(
            normalize_stop_after(&json!("design")).unwrap(),
            vec!["design".to_string()]
        );
        assert_eq!(normalize_stop_after(&json!(null)).unwrap(), Vec::<String>::new());
        assert_eq!(
            normalize_stop_after(&json!(["design", "execution"])).unwrap(),
            vec!["design".to_string(), "execution".to_string()]
        );
        assert!(normalize_stop_after(&json!(42)).is_err());
    }

    #[test]
    fn persist_phase_reports_atomic_roundtrip() {
        let dir = std::env::temp_dir().join(format!("sddia-reg-{}", Uuid::new_v4()));
        let reports = vec![json!({"phase_id": "design", "status": "executed"})];
        persist_phase_reports_atomic(dir.to_str().unwrap(), &reports).unwrap();
        let loaded = load_phase_reports(dir.to_str().unwrap()).unwrap().unwrap();
        assert_eq!(loaded[0]["phase_id"], "design");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn active_executions_not_under_docs() {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path();
        upsert_active_execution(
            repo,
            "eid-1",
            active_execution_entry("feature", ".SddIA/workspaces/feature/eid-1/", Some("docs/features/x"), "running", &["design".into()]),
        )
        .unwrap();
        let path = repo.join(ACTIVE_EXECUTIONS_REL);
        assert!(path.is_file());
        assert!(!path.starts_with(repo.join("docs")));
        let looked = lookup_active_execution(repo, "eid-1").unwrap().unwrap();
        assert_eq!(looked["process_name"], "feature");
    }

    #[test]
    fn phase_id_snake_case() {
        assert!(is_valid_phase_id("design"));
        assert!(is_valid_phase_id("workspace_init"));
        assert!(!is_valid_phase_id("Diseño"));
        assert!(!is_valid_phase_id(""));
        assert!(!is_valid_phase_id("1design"));
    }
}
