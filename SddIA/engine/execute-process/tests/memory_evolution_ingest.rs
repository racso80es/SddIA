//! Integración de ingesta `Domain_Entity_Telemetry_Captured` → LanceDB (vía store redb).

mod common;

use common::{lab_repo, with_mode, ModeGuard};
use execute_process::engine::memory_evolution_ingest_core::{
    ingest_domain_event_file, lancedb_uri, vector_store_root,
};
use execute_process::engine::route_domain_core::materialize_pending_domain_event;
use sddia_event_store::StoreMode;
use serde_json::json;
use std::fs;
use std::path::Path;

fn telemetry_payload() -> serde_json::Value {
    json!({
        "entity_type": "process",
        "entity_id": "feature",
        "execution_metrics": {
            "duration_ms": 42,
            "exit_code": 0,
            "success_status": true
        },
        "origin_stimulus": {
            "event_type": "Raw_Execution_Finished",
            "event_id": "11111111-2222-3333-4444-555555555555"
        }
    })
}

fn seed_store_pending_rel(repo: &Path) -> String {
    let rel = materialize_pending_domain_event(
        repo,
        "Domain_Entity_Telemetry_Captured",
        "tekton",
        telemetry_payload(),
    )
    .expect("materialize pending");
    assert!(
        !repo.join(&rel).is_file(),
        "cutover: pending mirror no esperado en {}",
        rel
    );
    rel
}

fn with_redb_lab<F: FnOnce(&Path, &str)>(f: F) {
    let _guard = ModeGuard;
    let repo = lab_repo();
    with_mode(StoreMode::Redb, || {
        let rel = seed_store_pending_rel(&repo);
        f(&repo, &rel);
    });
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn memory_evolution_ingest_persists_to_lancedb() {
    with_redb_lab(|repo, rel| {
        let result = ingest_domain_event_file(repo, rel);
        assert_eq!(result["ok"].as_bool(), Some(true), "{result}");
        let record_id = result["record_id"].as_str().unwrap();
        assert_eq!(result["store_backend"].as_str(), Some("lancedb"));
        let uri = lancedb_uri(repo);
        assert!(
            uri.exists() || uri.parent().is_some_and(|p| p.exists()),
            "lancedb uri missing: {}",
            uri.display()
        );
        assert!(!record_id.is_empty());
    });
}

#[test]
fn ingests_telemetry_captured_to_vector_store() {
    with_redb_lab(|repo, rel| {
        let result = ingest_domain_event_file(repo, rel);
        assert_eq!(result["ok"].as_bool(), Some(true), "{result}");
        assert!(result.get("record_id").and_then(|v| v.as_str()).is_some());
        assert_eq!(result["store_backend"].as_str(), Some("lancedb"));
    });
}

#[test]
fn json_fallback_is_not_used() {
    with_redb_lab(|repo, rel| {
        let result = ingest_domain_event_file(repo, rel);
        assert_eq!(result["ok"].as_bool(), Some(true), "{result}");
        let record_id = result["record_id"].as_str().unwrap();
        let json_path = vector_store_root(repo)
            .join("evolution")
            .join(format!("{record_id}.json"));
        assert!(
            !json_path.is_file(),
            "JSON fallback written at {}",
            json_path.display()
        );
    });
}
