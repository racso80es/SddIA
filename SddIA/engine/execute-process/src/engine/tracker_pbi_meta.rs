//! Metadatos de PBI/HU para cableado `tracker_ref` (HU Tracker Linear).

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub fn tracker_ref_from_pbi(repo: &Path, pbi_rel: &str) -> Option<String> {
    let path = resolve_repo_path(repo, pbi_rel);
    let text = fs::read_to_string(path).ok()?;
    yaml_field(&text, "tracker_ref")
}

pub fn expected_hu_tracker_ref_from_pbi(repo: &Path, pbi_rel: &str) -> Option<String> {
    let path = resolve_repo_path(repo, pbi_rel);
    let text = fs::read_to_string(path).ok()?;
    let historia = yaml_field(&text, "historia_ref")?;
    let hu_path = resolve_repo_path(repo, &historia);
    let hu_text = fs::read_to_string(hu_path).ok()?;
    yaml_field(&hu_text, "tracker_ref")
}

/// Resuelve ruta del PBI activo desde `persist_ref` (objectives `related_todo` o `pbi_ref`).
pub fn resolve_pbi_path_from_persist(repo: &Path, persist_ref: &str) -> Option<String> {
    let objectives = repo.join(persist_ref).join("objectives.md");
    if objectives.is_file() {
        if let Ok(text) = fs::read_to_string(&objectives) {
            if let Some(rel) = yaml_field(&text, "related_todo") {
                return Some(rel);
            }
            if let Some(rel) = yaml_field(&text, "pbi_ref") {
                return Some(rel);
            }
        }
    }
    None
}

pub fn attach_tracker_ref_from_persist(repo: &Path, persist_ref: &str, action_inputs: &mut Value) {
    if let Some(pbi) = resolve_pbi_path_from_persist(repo, persist_ref) {
        if let Some(tr) = tracker_ref_from_pbi(repo, &pbi) {
            if let Some(obj) = action_inputs.as_object_mut() {
                obj.insert("tracker_ref".into(), json!(tr));
            }
        }
        if let Some(hu) = expected_hu_tracker_ref_from_pbi(repo, &pbi) {
            if let Some(obj) = action_inputs.as_object_mut() {
                obj.insert("expected_hu_tracker_ref".into(), json!(hu));
            }
        }
    }
}

/// Hereda `tracker_ref` del `PullRequest_Presented` pendiente para la misma rama.
pub fn tracker_ref_from_presented_for_branch(repo: &Path, branch: &str) -> Option<String> {
    let pending = repo.join(".events/pending");
    if !pending.is_dir() {
        return None;
    }
    let Ok(entries) = fs::read_dir(&pending) else {
        return None;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(event) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        if event.get("event_type").and_then(|v| v.as_str()) != Some("PullRequest_Presented") {
            continue;
        }
        let payload = event.get("payload")?;
        if payload.get("branch").and_then(|v| v.as_str()) != Some(branch) {
            continue;
        }
        if let Some(tr) = payload
            .get("tracker_ref")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            return Some(tr.to_string());
        }
    }
    None
}

fn resolve_repo_path(repo: &Path, rel: &str) -> PathBuf {
    let rel = rel.trim();
    if Path::new(rel).is_absolute() {
        PathBuf::from(rel)
    } else {
        repo.join(rel)
    }
}

fn yaml_field(text: &str, key: &str) -> Option<String> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let rest = trimmed.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    for line in rest[..end].lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix(&format!("{key}:")) {
            let v = val.trim().trim_matches('"');
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

pub fn tracker_ref_from_pbi_frontmatter_path(path: &Path) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    yaml_field(&text, "tracker_ref")
}

pub fn related_todo_from_objectives(repo: &Path, persist_ref: &str) -> Option<String> {
    resolve_pbi_path_from_persist(repo, persist_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn tracker_ref_from_pbi_reads_frontmatter() {
        let dir = std::env::temp_dir().join(format!("pbi-meta-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let pbi = dir.join("pbi.md");
        let mut f = fs::File::create(&pbi).unwrap();
        writeln!(
            f,
            "---\ntracker_ref: OSC-99\nhistoria_ref: hu.md\n---\n# PBI\n"
        )
        .unwrap();
        let hu = dir.join("hu.md");
        fs::write(&hu, "---\ntracker_ref: OSC-5\n---\n# HU\n").unwrap();
        assert_eq!(
            tracker_ref_from_pbi(&dir, "pbi.md").as_deref(),
            Some("OSC-99")
        );
        assert_eq!(
            expected_hu_tracker_ref_from_pbi(&dir, "pbi.md").as_deref(),
            Some("OSC-5")
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
