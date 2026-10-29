//! Carga ECST desde fichero lógico `.events/...` o desde `events.redb` cuando no hay FS.

use serde_json::Value;
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

/// Resuelve ruta relativa al repo y carga el ECST (FS o store).
pub fn load_ecst_from_rel(repo: &Path, rel: &str) -> Result<Value, String> {
    load_ecst_from_inputs(repo, &serde_json::json!({ "event_file_path": rel }))
}

pub fn load_ecst_from_inputs(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let rel = str_field(inputs, "event_file_path").ok_or("event_file_path requerido")?;
    let trimmed = rel.trim_start_matches("./");
    let raw = PathBuf::from(trimmed);
    let path = if raw.is_absolute() {
        raw
    } else {
        repo.join(trimmed)
    };
    if path.is_file() {
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        return serde_json::from_str(&text).map_err(|e| e.to_string());
    }
    let event_id = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("event file not found: {}", path.display()))?;
    load_dispatched_event(repo, event_id, &path)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("event file not found: {}", path.display()))
}

/// Antes de efecto remoto: el ECST debe coincidir con el registro del bus (mismo id y contenido).
pub fn verify_ecst_matches_bus(repo: &Path, event: &Value) -> Result<(), String> {
    if !writes_store(store_mode()) {
        return Ok(());
    }
    let event_id = str_field(event, "event_id").ok_or("event_id ausente")?;
    let synthetic = repo
        .join(".events/pending")
        .join(format!("{event_id}.json"));
    let stored = load_dispatched_event(repo, &event_id, &synthetic)
        .map_err(|e| e.to_string())?;
    let Some(stored) = stored else {
        // Lab / filesystem transitorio: sin fila en bus no hay SSOT remoto que contradecir.
        return Ok(());
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

    #[test]
    fn load_ecst_from_rel_redb_without_pending_file() {
        let _guard = ModeGuard;
        override_store_mode(Some(StoreMode::Redb));
        let repo = tempfile::tempdir().unwrap();
        let rel = materialize_pending_domain_event(
            repo.path(),
            "Domain_Entity_Telemetry_Captured",
            "test",
            json!({"entity_id": "e1"}),
        )
        .expect("materialize");
        let body = load_ecst_from_rel(repo.path(), &rel).expect("load");
        assert_eq!(
            body.get("event_type").and_then(|v| v.as_str()),
            Some("Domain_Entity_Telemetry_Captured")
        );
    }

    #[test]
    fn verify_rejects_tampered_ecst() {
        let _guard = ModeGuard;
        override_store_mode(Some(StoreMode::Redb));
        let repo = tempfile::tempdir().unwrap();
        let rel = materialize_pending_domain_event(
            repo.path(),
            "Work_Initiated",
            "feature",
            json!({"tracker_ref": "AUT-1", "branch": "feat/x"}),
        )
        .unwrap();
        let mut tampered = load_ecst_from_rel(repo.path(), &rel).unwrap();
        tampered["payload"]["branch"] = json!("other");
        assert!(verify_ecst_matches_bus(repo.path(), &tampered).is_err());
    }
}
