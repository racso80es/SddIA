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

pub fn is_entity_covered(repo: &Path, entity_uuid: &str) -> bool {
    load_coverage(repo)
        .ok()
        .and_then(|data| {
            data.get("coverage_matrix")
                .and_then(|m| m.get(entity_uuid))
                .and_then(|entry| entry.get("is_covered"))
                .and_then(|v| v.as_bool())
        })
        .unwrap_or(false)
}

const ENTITY_DIRS: &[(&str, &str)] = &[
    ("skill", "SddIA/skills"),
    ("event", "SddIA/events"),
    ("process", "SddIA/process"),
    ("agent", "SddIA/agents"),
    ("tool", "SddIA/tools"),
    ("action", "SddIA/actions"),
    ("norm", "SddIA/library/norms"),
    ("codex", "SddIA/library/codexes"),
    ("suite", "SddIA/suites"),
];

fn parse_index_uuids(index_path: &Path) -> std::collections::HashMap<String, String> {
    use regex::Regex;
    use std::sync::OnceLock;
    static UUID_RE: OnceLock<Regex> = OnceLock::new();
    static NAME_RE: OnceLock<Regex> = OnceLock::new();
    let uuid_re = UUID_RE.get_or_init(|| {
        Regex::new(r"(?i)[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}")
            .expect("uuid regex")
    });
    let name_re = NAME_RE.get_or_init(|| Regex::new(r"`([^`]+\.md)`").expect("name regex"));

    let Ok(text) = fs::read_to_string(index_path) else {
        return std::collections::HashMap::new();
    };
    let mut out = std::collections::HashMap::new();
    for line in text.lines() {
        if !line.starts_with('|') || !line.contains('`') {
            continue;
        }
        let uuids: Vec<_> = uuid_re.find_iter(line).map(|m| m.as_str().to_string()).collect();
        if uuids.is_empty() {
            continue;
        }
        if let Some(caps) = name_re.captures(line) {
            let name = caps[1].replace(".md", "");
            out.insert(name, uuids[0].clone());
        } else {
            let cols: Vec<_> = line.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
            if let Some(first) = cols.first() {
                out.insert(first.replace(".md", ""), uuids[0].clone());
            }
        }
    }
    out
}

fn valid_sha256_str(value: &str) -> bool {
    value.starts_with("sha256:") && value.len() > 15
}

fn hash_from_artifact(artifact: &Path) -> Result<String, String> {
    use sha2::{Digest, Sha256};
    let text = fs::read_to_string(artifact).map_err(|e| e.to_string())?;
    let digest = Sha256::digest(text.as_bytes());
    Ok(format!("sha256:{digest:x}"))
}

fn resolve_hash_signature(
    artifact: &Path,
    fm: &std::collections::HashMap<String, serde_yaml::Value>,
) -> Result<String, String> {
    if let Some(hs) = fm.get("hash_signature").and_then(|v| v.as_str()) {
        if valid_sha256_str(hs) {
            return Ok(hs.to_string());
        }
    }
    hash_from_artifact(artifact)
}

pub fn scan_orphans(repo: &Path) -> Result<Value, String> {
    use crate::core::parser::parse_frontmatter;
    let mut orphans = Vec::new();
    let mut indexed_count = 0usize;
    let mut dirs: Vec<(&str, String)> = ENTITY_DIRS
        .iter()
        .map(|(c, d)| (*c, (*d).to_string()))
        .collect();
    if let Ok(cfg) = load_paths_config(repo) {
        if let Some(arr) = cfg
            .get("directories")
            .and_then(|d| d.get("process_domain_roots"))
            .and_then(|v| v.as_array())
        {
            for item in arr {
                if let Some(rel) = item.as_str() {
                    let rel = rel.trim().trim_end_matches('/').replace('\\', "/");
                    if !rel.is_empty() {
                        dirs.push(("process", rel));
                    }
                }
            }
        }
    }
    for (entity_class, rel_dir) in &dirs {
        let base = repo.join(rel_dir);
        let index_map = parse_index_uuids(&base.join("index.md"));
        for (name, uuid) in index_map {
            indexed_count += 1;
            let artifact = base.join(format!("{name}.md"));
            let artifact_exists = artifact.is_file();
            let is_covered = is_entity_covered(repo, &uuid);
            if artifact_exists && !is_covered {
                let fm = parse_frontmatter(&artifact).unwrap_or_default();
                let hash_signature = resolve_hash_signature(&artifact, &fm)?;
                orphans.push(json!({
                    "entity_class": entity_class,
                    "entity_name": name,
                    "entity_uuid": uuid,
                    "artifact_path": artifact.strip_prefix(repo).unwrap_or(&artifact).to_string_lossy().replace('\\', "/"),
                    "artifact_exists": artifact_exists,
                    "is_covered_ssot": is_covered,
                    "hash_signature": hash_signature,
                }));
            }
        }
    }
    Ok(json!({
        "scanned_at": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "scan_source": "eda-coverage.json",
        "orphan_count": orphans.len(),
        "orphans": orphans,
        "indexed_entities": indexed_count,
    }))
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
