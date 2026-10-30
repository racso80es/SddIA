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

fn tracker_ref_already(action_inputs: &Value) -> bool {
    action_inputs
        .get("tracker_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .is_some_and(|s| !s.is_empty())
}

pub fn attach_tracker_ref_from_persist(repo: &Path, persist_ref: &str, action_inputs: &mut Value) {
    if !tracker_ref_already(action_inputs) {
        if let Some(tr) = tracker_ref_on_objectives(repo, persist_ref) {
            if let Some(obj) = action_inputs.as_object_mut() {
                obj.insert("tracker_ref".into(), json!(tr));
            }
        }
    }
    if let Some(pbi) = resolve_pbi_path_from_persist(repo, persist_ref) {
        if !tracker_ref_already(action_inputs) {
            if let Some(tr) = tracker_ref_from_pbi(repo, &pbi) {
                if let Some(obj) = action_inputs.as_object_mut() {
                    obj.insert("tracker_ref".into(), json!(tr));
                }
            }
        }
        if let Some(hu) = expected_hu_tracker_ref_from_pbi(repo, &pbi) {
            if let Some(obj) = action_inputs.as_object_mut() {
                obj.insert("expected_hu_tracker_ref".into(), json!(hu));
            }
        }
    }
}

fn tracker_ref_on_objectives(repo: &Path, persist_ref: &str) -> Option<String> {
    let path = resolve_repo_path(repo, persist_ref).join("objectives.md");
    let text = fs::read_to_string(path).ok()?;
    yaml_field(&text, "tracker_ref")
}

/// Hereda `tracker_ref` del `PullRequest_Presented` de la misma rama.
/// Mira pending, processed y la fila del store: el sello de merge llega cuando el presentado ya no está en pending.
pub fn tracker_ref_from_presented_for_branch(repo: &Path, branch: &str) -> Option<String> {
    for rel in [".events/pending", ".events/processed"] {
        if let Some(tr) = tracker_ref_in_dir(&repo.join(rel), branch) {
            return Some(tr);
        }
    }
    tracker_ref_in_store(repo, branch)
}

fn tracker_ref_in_dir(dir: &Path, branch: &str) -> Option<String> {
    if !dir.is_dir() {
        return None;
    }
    let entries = fs::read_dir(dir).ok()?;
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
        if let Some(tr) = tracker_ref_of_presented(&event, branch) {
            return Some(tr);
        }
    }
    None
}

/// Presented en `Family::Domain` del store para `branch` (modo con escritura a store).
pub(crate) fn presented_in_store(repo: &Path, branch: &str) -> bool {
    visit_presented_in_store(repo, |event| {
        event
            .get("payload")
            .and_then(|p| p.get("branch"))
            .and_then(|v| v.as_str())
            == Some(branch)
    })
}

fn tracker_ref_in_store(repo: &Path, branch: &str) -> Option<String> {
    let mut found = None;
    visit_presented_in_store(repo, |event| {
        if let Some(tr) = tracker_ref_of_presented(event, branch) {
            found = Some(tr);
            true
        } else {
            false
        }
    });
    found
}

/// Recorre `PullRequest_Presented` en `Family::Domain`. `pred` true → corta.
fn visit_presented_in_store(repo: &Path, mut pred: impl FnMut(&Value) -> bool) -> bool {
    if !sddia_event_store::writes_store(sddia_event_store::store_mode()) {
        return false;
    }
    let db = sddia_event_store::runtime_db_path(repo);
    if !db.is_file() {
        return false;
    }
    let Ok(store) = sddia_event_store::EventStore::open(db) else {
        return false;
    };
    let Ok(rows) = store.list_family(sddia_event_store::Family::Domain) else {
        return false;
    };
    for row in rows {
        if row.event_type != "PullRequest_Presented" {
            continue;
        }
        if pred(&row.ecst) {
            return true;
        }
    }
    false
}

fn tracker_ref_of_presented(event: &Value, branch: &str) -> Option<String> {
    if event.get("event_type").and_then(|v| v.as_str()) != Some("PullRequest_Presented") {
        return None;
    }
    let payload = event.get("payload")?;
    if payload.get("branch").and_then(|v| v.as_str()) != Some(branch) {
        return None;
    }
    payload
        .get("tracker_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// `event_id` del `PullRequest_Presented` más reciente para `branch` (pending, processed, store).
pub fn presented_event_id_for_branch(repo: &Path, branch: &str) -> Option<String> {
    let mut best: Option<(String, String)> = None;
    for rel in [".events/pending", ".events/processed"] {
        presented_ids_in_dir(&repo.join(rel), branch, &mut best);
    }
    presented_ids_in_store(repo, branch, &mut best);
    best.map(|(_, id)| id)
}

fn presented_ids_in_dir(dir: &Path, branch: &str, best: &mut Option<(String, String)>) {
    if !dir.is_dir() {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
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
        consider_presented_event_id(&event, branch, best);
    }
}

fn presented_ids_in_store(repo: &Path, branch: &str, best: &mut Option<(String, String)>) {
    if !sddia_event_store::writes_store(sddia_event_store::store_mode()) {
        return;
    }
    let db = sddia_event_store::runtime_db_path(repo);
    if !db.is_file() {
        return;
    }
    let Ok(store) = sddia_event_store::EventStore::open(db) else {
        return;
    };
    let Ok(rows) = store.list_family(sddia_event_store::Family::Domain) else {
        return;
    };
    for row in rows {
        if row.event_type != "PullRequest_Presented" {
            continue;
        }
        consider_presented_event_id(&row.ecst, branch, best);
    }
}

fn consider_presented_event_id(event: &Value, branch: &str, best: &mut Option<(String, String)>) {
    if event.get("event_type").and_then(|v| v.as_str()) != Some("PullRequest_Presented") {
        return;
    }
    let payload = match event.get("payload") {
        Some(p) => p,
        None => return,
    };
    if payload.get("branch").and_then(|v| v.as_str()) != Some(branch) {
        return;
    }
    let Some(event_id) = event.get("event_id").and_then(|v| v.as_str()) else {
        return;
    };
    let ts = event
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    match best {
        None => *best = Some((ts, event_id.to_string())),
        Some((prev_ts, _)) if ts >= *prev_ts => *best = Some((ts, event_id.to_string())),
        _ => {}
    }
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

/// Identificador de issue (`AUT-45`), no ruta de fichero PBI.
pub fn looks_like_issue_identifier(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() || s.contains('/') || s.contains('\\') || s.ends_with(".md") {
        return false;
    }
    let Some((prefix, rest)) = s.split_once('-') else {
        return false;
    };
    !prefix.is_empty()
        && prefix.chars().all(|c| c.is_ascii_alphabetic())
        && !rest.is_empty()
        && rest.chars().all(|c| c.is_ascii_digit())
}

/// Resolución de `tracker_ref` para peaje lifecycle (inputs + objectives, AUT-47).
pub fn resolve_tracker_ref_for_lifecycle(repo: &Path, process_inputs: &Value) -> Option<String> {
    for key in ["tracker_ref", "pbi_ref", "issue_ref"] {
        if let Some(s) = process_inputs.get(key).and_then(|v| v.as_str()) {
            let t = s.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    let persist = process_inputs.get("persist_ref")?.as_str()?.trim();
    if persist.is_empty() {
        return None;
    }
    if let Some(tr) = tracker_ref_on_objectives(repo, persist) {
        return Some(tr);
    }
    if let Some(pbi_rel) = resolve_pbi_path_from_persist(repo, persist) {
        if looks_like_issue_identifier(&pbi_rel) {
            return Some(pbi_rel);
        }
        return tracker_ref_from_pbi(repo, &pbi_rel);
    }
    None
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

    #[test]
    fn looks_like_issue_identifier_rejects_paths() {
        assert!(looks_like_issue_identifier("AUT-46"));
        assert!(!looks_like_issue_identifier("docs/todos/pending/x.md"));
        assert!(!looks_like_issue_identifier("AUT-46-extra"));
    }

    #[test]
    fn resolve_lifecycle_from_objectives_tracker_ref_only() {
        let dir = std::env::temp_dir().join(format!("pbi-lc-{}", std::process::id()));
        let persist = dir.join("docs/fixes/demo");
        fs::create_dir_all(&persist).unwrap();
        fs::write(
            persist.join("objectives.md"),
            "---\ntracker_ref: AUT-12\n---\n# obj\n",
        )
        .unwrap();
        let inputs = json!({"persist_ref": "docs/fixes/demo"});
        assert_eq!(
            resolve_tracker_ref_for_lifecycle(&dir, &inputs).as_deref(),
            Some("AUT-12")
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn resolve_lifecycle_pbi_ref_as_identifier_not_path() {
        let dir = std::env::temp_dir().join(format!("pbi-id-{}", std::process::id()));
        let persist = dir.join("docs/features/demo");
        fs::create_dir_all(&persist).unwrap();
        fs::write(
            persist.join("objectives.md"),
            "---\npbi_ref: AUT-46\n---\n# obj\n",
        )
        .unwrap();
        let inputs = json!({"persist_ref": "docs/features/demo"});
        assert_eq!(
            resolve_tracker_ref_for_lifecycle(&dir, &inputs).as_deref(),
            Some("AUT-46")
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn objectives_tracker_ref_feeds_the_seal() {
        let dir = std::env::temp_dir().join(format!("pbi-obj-{}", std::process::id()));
        let persist = dir.join("docs/fixes/demo");
        fs::create_dir_all(&persist).unwrap();
        fs::write(
            persist.join("objectives.md"),
            "---\ntracker_ref: MAN-23\n---\n# obj\n",
        )
        .unwrap();
        let mut inputs = json!({});
        attach_tracker_ref_from_persist(&dir, "docs/fixes/demo", &mut inputs);
        assert_eq!(inputs["tracker_ref"], "MAN-23");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn presented_in_processed_still_yields_tracker_ref() {
        let dir = std::env::temp_dir().join(format!("pbi-proc-{}", std::process::id()));
        let processed = dir.join(".events/processed");
        fs::create_dir_all(&processed).unwrap();
        fs::write(
            processed.join("presented.json"),
            r#"{"event_type":"PullRequest_Presented","payload":{"branch":"fix/pr-merged-witness-store","tracker_ref":"MAN-23"}}"#,
        )
        .unwrap();
        assert_eq!(
            tracker_ref_from_presented_for_branch(&dir, "fix/pr-merged-witness-store").as_deref(),
            Some("MAN-23")
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn presented_event_id_from_pending_json() {
        let dir = std::env::temp_dir().join(format!("pbi-pres-id-{}", std::process::id()));
        let pending = dir.join(".events/pending");
        fs::create_dir_all(&pending).unwrap();
        fs::write(
            pending.join("p1.json"),
            r#"{"event_id":"11111111-1111-4111-8111-111111111111","event_type":"PullRequest_Presented","timestamp":"2026-10-30T06:00:00Z","payload":{"branch":"feat/x"}}"#,
        )
        .unwrap();
        fs::write(
            pending.join("p2.json"),
            r#"{"event_id":"22222222-2222-4222-8222-222222222222","event_type":"PullRequest_Presented","timestamp":"2026-10-30T07:00:00Z","payload":{"branch":"feat/x"}}"#,
        )
        .unwrap();
        assert_eq!(
            presented_event_id_for_branch(&dir, "feat/x").as_deref(),
            Some("22222222-2222-4222-8222-222222222222")
        );
        assert!(presented_event_id_for_branch(&dir, "feat/other").is_none());
        let _ = fs::remove_dir_all(&dir);
    }
}
