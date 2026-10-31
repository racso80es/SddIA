//! Carga ECST: precedencia `event` en línea → `event_id` (store) → `event_file_path` (lab).

use serde_json::{json, Value};
use sddia_event_store::{load_dispatched_event, store_mode, writes_store};
use std::fs;
use std::path::{Path, PathBuf};

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn load_from_store(repo: &Path, event_id: &str, logical_path: &Path) -> Result<Value, String> {
    load_dispatched_event(repo, event_id, logical_path)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("event not found in store: {event_id}"))
}

fn resolve_logical_path(repo: &Path, rel: &str) -> PathBuf {
    let trimmed = rel.trim_start_matches("./");
    let raw = PathBuf::from(trimmed);
    if raw.is_absolute() {
        raw
    } else {
        repo.join(trimmed)
    }
}

fn load_from_event_file_path(
    repo: &Path,
    rel: &str,
    explicit_event_id: Option<&str>,
) -> Result<Value, String> {
    let path = resolve_logical_path(repo, rel);
    if path.extension().and_then(|e| e.to_str()) == Some("json") && path.is_file() {
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        return serde_json::from_str(&text).map_err(|e| e.to_string());
    }
    if writes_store(store_mode()) {
        if let Some(eid) = explicit_event_id.filter(|s| !s.is_empty()) {
            return load_from_store(repo, eid, &path);
        }
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| format!("event file not found: {}", path.display()))?;
            return load_from_store(repo, stem, &path);
        }
        return Err(format!(
            "event_id requerido para cargar desde store (path={})",
            path.display()
        ));
    }
    Err(format!("event file not found: {}", path.display()))
}

/// Resuelve ruta relativa al repo y carga el ECST (FS o store).
pub fn load_ecst_from_rel(repo: &Path, rel: &str) -> Result<Value, String> {
    load_ecst_from_inputs(repo, &json!({ "event_file_path": rel }))
}

/// Precedencia: objeto `event` → `event_id` → `event_file_path`.
pub fn load_ecst_from_inputs(repo: &Path, inputs: &Value) -> Result<Value, String> {
    if let Some(inline) = inputs.get("event") {
        if inline.is_object() {
            if str_field(inline, "event_id").is_some() {
                return Ok(inline.clone());
            }
        }
    }
    if let Some(event_id) = str_field(inputs, "event_id") {
        let synthetic = repo
            .join(".events/pending")
            .join(format!("{event_id}.json"));
        return load_from_store(repo, &event_id, &synthetic);
    }
    if let Some(rel) = str_field(inputs, "event_file_path") {
        return load_from_event_file_path(repo, &rel, None);
    }
    Err("event, event_id o event_file_path requerido".into())
}

/// Carga para handlers con tracker remoto: recarga desde bus si el inline no coincide.
pub fn load_ecst_for_handler(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let event = load_ecst_from_inputs(repo, inputs)?;
    if !writes_store(store_mode()) {
        return Ok(event);
    }
    if verify_ecst_matches_bus(repo, &event).is_ok() {
        return Ok(event);
    }
    let eid = str_field(&event, "event_id").ok_or("event_id ausente")?;
    let synthetic = repo.join(".events/pending").join(format!("{eid}.json"));
    let stored = load_from_store(repo, &eid, &synthetic)?;
    verify_ecst_matches_bus(repo, &stored)?;
    Ok(stored)
}

/// Antes de efecto remoto: el ECST debe existir en bus y coincidir (modo redb).
pub fn verify_ecst_matches_bus(repo: &Path, event: &Value) -> Result<(), String> {
    if !writes_store(store_mode()) {
        return Ok(());
    }
    let event_id = str_field(event, "event_id").ok_or("event_id ausente")?;
    let synthetic = repo
        .join(".events/pending")
        .join(format!("{event_id}.json"));
    let stored = load_dispatched_event(repo, &event_id, &synthetic).map_err(|e| e.to_string())?;
    let Some(stored) = stored else {
        return Err("ecst_not_in_bus".into());
    };
    if stored != *event {
        return Err("ecst_mismatch".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::route_domain_core::materialize_pending_domain_event;
    use sddia_event_store::{override_store_mode, StoreMode};
    use serde_json::json;

    struct ModeGuard;

    impl Drop for ModeGuard {
        fn drop(&mut self) {
            override_store_mode(None);
        }
    }

    fn temp_repo_with_bus_skeleton() -> tempfile::TempDir {
        let root = crate::core::repo::find_repo_root().expect("repo root");
        let repo = tempfile::tempdir().unwrap();
        for rel in [
            "SddIA/core/cumulo.paths.json",
            "SddIA/core/event-domain-subscriptions.json",
        ] {
            let src = root.join(rel);
            let dst = repo.path().join(rel);
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(&src, &dst).unwrap_or_else(|e| panic!("copy {rel}: {e}"));
        }
        fs::create_dir_all(repo.path().join(".events/pending")).unwrap();
        repo
    }

    #[test]
    fn load_inline_event_without_disk() {
        let repo = tempfile::tempdir().unwrap();
        let event = json!({
            "event_id": "inline-1",
            "event_type": "Work_Initiated",
            "payload": { "branch": "feat/x" }
        });
        let got = load_ecst_from_inputs(repo.path(), &json!({ "event": event.clone() })).unwrap();
        assert_eq!(got, event);
    }

    fn seed_store_only_pending_rel(
        repo: &Path,
        event_type: &str,
        payload: Value,
    ) -> String {
        materialize_pending_domain_event(repo, event_type, "test", payload).expect("materialize");
        let store = sddia_event_store::EventStore::open(sddia_event_store::runtime_db_path(repo))
            .expect("open store");
        let rows = store
            .list_family(sddia_event_store::Family::Domain)
            .expect("list");
        let event_id = rows.last().expect("event row").event_id.clone();
        drop(store);
        let pending = repo.join(".events/pending").join(format!("{event_id}.json"));
        if pending.is_file() {
            fs::remove_file(&pending).expect("remove pending mirror");
        }
        format!(".events/pending/{event_id}.json")
    }

    #[test]
    fn load_by_event_id_redb_without_pending_file() {
        let _guard = ModeGuard;
        override_store_mode(Some(StoreMode::Redb));
        let repo = temp_repo_with_bus_skeleton();
        let rel = seed_store_only_pending_rel(
            repo.path(),
            "Domain_Entity_Telemetry_Captured",
            json!({"entity_id": "e1"}),
        );
        let body = load_ecst_from_rel(repo.path(), &rel).expect("load rel");
        let eid = body.get("event_id").and_then(|v| v.as_str()).unwrap();
        let pending = repo.path().join(".events/pending").join(format!("{eid}.json"));
        if pending.is_file() {
            fs::remove_file(&pending).unwrap();
        }
        let by_id = load_ecst_from_inputs(
            repo.path(),
            &json!({ "event_id": eid }),
        )
        .expect("by id");
        assert_eq!(by_id.get("event_id").and_then(|v| v.as_str()), Some(eid));
    }

    #[test]
    fn load_ecst_from_rel_redb_without_pending_file() {
        let _guard = ModeGuard;
        override_store_mode(Some(StoreMode::Redb));
        let repo = temp_repo_with_bus_skeleton();
        let rel = seed_store_only_pending_rel(
            repo.path(),
            "Domain_Entity_Telemetry_Captured",
            json!({"entity_id": "e1"}),
        );
        let body = load_ecst_from_rel(repo.path(), &rel).expect("load");
        assert_eq!(
            body.get("event_type").and_then(|v| v.as_str()),
            Some("Domain_Entity_Telemetry_Captured")
        );
    }

    #[test]
    fn load_rejects_without_sources() {
        let repo = tempfile::tempdir().unwrap();
        assert!(load_ecst_from_inputs(repo.path(), &json!({})).is_err());
    }

    #[test]
    fn verify_rejects_tampered_ecst() {
        let _guard = ModeGuard;
        override_store_mode(Some(StoreMode::Redb));
        let repo = temp_repo_with_bus_skeleton();
        let rel = seed_store_only_pending_rel(
            repo.path(),
            "Work_Initiated",
            json!({"tracker_ref": "AUT-1", "branch": "feat/x"}),
        );
        let mut tampered = load_ecst_from_rel(repo.path(), &rel).unwrap();
        tampered["payload"]["branch"] = json!("other");
        assert!(verify_ecst_matches_bus(repo.path(), &tampered).is_err());
    }

    #[test]
    fn verify_rejects_inline_not_in_bus() {
        let _guard = ModeGuard;
        override_store_mode(Some(StoreMode::Redb));
        let repo = temp_repo_with_bus_skeleton();
        let inline = json!({
            "event_id": "ghost-id",
            "event_type": "PBI_Refined",
            "payload": {}
        });
        assert!(verify_ecst_matches_bus(repo.path(), &inline).is_err());
    }

    #[test]
    fn handler_reload_from_bus_when_inline_tampered() {
        let _guard = ModeGuard;
        override_store_mode(Some(StoreMode::Redb));
        let repo = temp_repo_with_bus_skeleton();
        let rel = seed_store_only_pending_rel(
            repo.path(),
            "Work_Initiated",
            json!({"tracker_ref": "AUT-1", "branch": "feat/x"}),
        );
        let mut inline = load_ecst_from_rel(repo.path(), &rel).unwrap();
        inline["payload"]["branch"] = json!("tampered");
        let restored = load_ecst_for_handler(
            repo.path(),
            &json!({ "event": inline }),
        )
        .expect("reload");
        assert_eq!(
            restored.get("payload").and_then(|p| p.get("branch")),
            Some(&json!("feat/x"))
        );
    }
}
