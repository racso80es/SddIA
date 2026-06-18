//! Utilidades del bus EDA (paridad parcial `eda_bus_utils.py`).

use super::workspace::load_paths_config;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct EdaBusPaths {
    pub pending: PathBuf,
    pub processing: PathBuf,
    pub processed: PathBuf,
}

fn normalize_rel(rel: &str) -> String {
    rel.trim().replace('\\', "/")
}

fn resolve_bus_path(repo: &Path, rel: &str) -> PathBuf {
    let rel = normalize_rel(rel);
    if rel.starts_with("./") {
        repo.join(rel.trim_start_matches("./"))
    } else {
        repo.join(rel)
    }
}

pub fn load_eda_bus(repo: &Path) -> Result<EdaBusPaths, String> {
    let cfg = load_paths_config(repo)?;
    let bus = cfg.get("eda_bus");
    let pending = bus
        .and_then(|b| b.get("pending"))
        .and_then(|v| v.as_str())
        .unwrap_or("./.events/pending");
    let processing = bus
        .and_then(|b| b.get("processing"))
        .and_then(|v| v.as_str())
        .unwrap_or("./.events/processing");
    let processed = bus
        .and_then(|b| b.get("processed"))
        .and_then(|v| v.as_str())
        .unwrap_or("./.events/processed");
    Ok(EdaBusPaths {
        pending: resolve_bus_path(repo, pending),
        processing: resolve_bus_path(repo, processing),
        processed: resolve_bus_path(repo, processed),
    })
}

fn collect_json_headers(dir: &Path, seen: &mut HashSet<PathBuf>, out: &mut Vec<PathBuf>) {
    if !dir.is_dir() {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();
    paths.sort();
    for path in paths {
        let resolved = path.canonicalize().unwrap_or_else(|_| path.clone());
        if seen.insert(resolved) {
            out.push(path);
        }
    }
}

pub fn iter_bus_event_files(repo: &Path) -> Result<Vec<PathBuf>, String> {
    let bus = load_eda_bus(repo)?;
    let mut seen = HashSet::new();
    let mut files = Vec::new();
    collect_json_headers(&bus.pending, &mut seen, &mut files);
    collect_json_headers(&bus.processing, &mut seen, &mut files);
    collect_json_headers(&bus.processed, &mut seen, &mut files);
    files.sort_by_key(|p| {
        p.strip_prefix(repo)
            .unwrap_or(p)
            .to_string_lossy()
            .replace('\\', "/")
    });
    Ok(files)
}

pub fn find_existing_domain_event(
    repo: &Path,
    entity_uuid: &str,
    lifecycle_operation: &str,
    event_type: &str,
) -> Result<Option<Value>, String> {
    for path in iter_bus_event_files(repo)? {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(body) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        if body.get("event_type").and_then(|v| v.as_str()) != Some(event_type) {
            continue;
        }
        let Some(payload) = body.get("payload").and_then(|v| v.as_object()) else {
            continue;
        };
        if payload.get("entity_uuid").and_then(|v| v.as_str()) == Some(entity_uuid)
            && payload.get("lifecycle_operation").and_then(|v| v.as_str()) == Some(lifecycle_operation)
        {
            return Ok(Some(serde_json::json!({
                "event_id": body.get("event_id"),
                "target_path": path
                    .strip_prefix(repo)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/"),
                "event_type": body.get("event_type"),
            })));
        }
    }
    Ok(None)
}
