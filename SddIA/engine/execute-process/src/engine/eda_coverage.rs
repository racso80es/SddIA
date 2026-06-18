//! SSOT `eda-coverage.json` (paridad `eda_coverage_utils.py`).

use super::workspace::load_paths_config;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const DEFAULT_COVERAGE: &str = "SddIA/core/eda-coverage.json";

fn coverage_path(repo: &Path) -> Result<std::path::PathBuf, String> {
    let cfg = load_paths_config(repo)?;
    let rel = cfg
        .get("eda_coverage")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(DEFAULT_COVERAGE);
    Ok(repo.join(rel.trim().replace('\\', "/")))
}

fn load_coverage(repo: &Path) -> Result<Value, String> {
    let path = coverage_path(repo)?;
    if !path.is_file() {
        return Ok(json!({"version": "1.0.0", "coverage_matrix": {}}));
    }
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut data: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if !data.get("coverage_matrix").map(|v| v.is_object()).unwrap_or(false) {
        data["coverage_matrix"] = json!({});
    }
    if !data.get("version").and_then(|v| v.as_str()).is_some() {
        data["version"] = json!("1.0.0");
    }
    Ok(data)
}

fn save_coverage(repo: &Path, data: &Value) -> Result<(), String> {
    let path = coverage_path(repo)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let tmp = path.with_extension("json.tmp");
    let text = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&tmp, format!("{text}\n")).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn upsert_entity_coverage(
    repo: &Path,
    entity_uuid: &str,
    event_type: &str,
    last_hash: &str,
) -> Result<(), String> {
    let mut data = load_coverage(repo)?;
    let matrix = data
        .as_object_mut()
        .and_then(|o| o.get_mut("coverage_matrix"))
        .and_then(|v| v.as_object_mut())
        .ok_or("coverage_matrix invalid")?;
    matrix.insert(
        entity_uuid.to_string(),
        json!({
            "is_covered": true,
            "last_emitted_event": event_type,
            "last_hash": last_hash,
            "correlation_timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        }),
    );
    save_coverage(repo, &data)
}

pub fn remove_entity_coverage(repo: &Path, entity_uuid: &str) -> Result<(), String> {
    let mut data = load_coverage(repo)?;
    if let Some(matrix) = data
        .as_object_mut()
        .and_then(|o| o.get_mut("coverage_matrix"))
        .and_then(|v| v.as_object_mut())
    {
        matrix.remove(entity_uuid);
    }
    save_coverage(repo, &data)
}
