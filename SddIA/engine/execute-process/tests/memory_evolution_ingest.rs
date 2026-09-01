//! Integración de ingesta `Domain_Entity_Telemetry_Captured` → LanceDB.
//! Compila el lib de `execute-process` sin `cfg(test)` del orquestador entero.

use execute_process::engine::memory_evolution_ingest_core::{
    ingest_domain_event_file, lancedb_uri, vector_store_root,
};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn write_event(repo: &Path, rel: &str) {
    let event_path = repo.join(rel);
    fs::create_dir_all(event_path.parent().unwrap()).unwrap();
    fs::write(
        &event_path,
        r#"{
  "event_id": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
  "event_type": "Domain_Entity_Telemetry_Captured",
  "payload": {
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
  },
  "delivery_state": {}
}"#,
    )
    .unwrap();
}

#[test]
fn memory_evolution_ingest_persists_to_lancedb() {
    let dir = tempdir().unwrap();
    let repo = dir.path();
    let event_rel = ".events/domain/test-telem.json";
    write_event(repo, event_rel);

    let result = ingest_domain_event_file(repo, event_rel);
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
}

#[test]
fn ingests_telemetry_captured_to_vector_store() {
    let dir = tempdir().unwrap();
    let repo = dir.path();
    let event_rel = ".events/domain/test-telem.json";
    write_event(repo, event_rel);

    let result = ingest_domain_event_file(repo, event_rel);
    assert_eq!(result["ok"].as_bool(), Some(true), "{result}");
    assert!(result.get("record_id").and_then(|v| v.as_str()).is_some());

    let again = ingest_domain_event_file(repo, event_rel);
    assert_eq!(again["skipped"].as_str(), Some("already_indexed"));
}

#[test]
fn json_fallback_is_not_used() {
    let dir = tempdir().unwrap();
    let repo = dir.path();
    let event_rel = ".events/domain/test-telem.json";
    write_event(repo, event_rel);
    let result = ingest_domain_event_file(repo, event_rel);
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
}
