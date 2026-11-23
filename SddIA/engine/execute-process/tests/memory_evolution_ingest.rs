//! Tests unitarios/contrato de orquestación `memory_evolution_ingest_core` (vía puerto mock DI, sin LanceDB).

mod common;

use common::{install_backend, lab_repo, LabBackend};
use execute_process::engine::memory_evolution_ingest_core::{
    ingest_domain_event_file_with_port, lancedb_uri, vector_store_root, EvolutionIngestPort,
};
use execute_process::engine::route_domain_core::materialize_pending_domain_event;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Default)]
struct MockEvolutionIngestPort {
    records: RefCell<HashMap<String, (String, String, Value)>>,
}

impl EvolutionIngestPort for MockEvolutionIngestPort {
    fn ingest_telemetry(
        &self,
        _repo: &Path,
        record_id: &str,
        polarity: &str,
        payload: &str,
        operational_metadata: &Value,
    ) -> Result<Value, String> {
        let mut map = self.records.borrow_mut();
        if map.contains_key(record_id) {
            return Ok(json!({
                "skipped": "already_indexed",
                "record_id": record_id,
                "store_backend": "lancedb",
                "table": "evolution",
            }));
        }
        map.insert(
            record_id.to_string(),
            (
                polarity.to_string(),
                payload.to_string(),
                operational_metadata.clone(),
            ),
        );
        Ok(json!({
            "record_id": record_id,
            "store_backend": "lancedb",
            "table": "evolution",
        }))
    }
}

fn telemetry_payload() -> Value {
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
    let repo = lab_repo();
    let _g = install_backend(&repo, LabBackend::Redb);
    let rel = seed_store_pending_rel(&repo);
    f(&repo, &rel);
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn memory_evolution_ingest_persists_via_di_port_and_is_idempotent() {
    with_redb_lab(|repo, rel| {
        let port = MockEvolutionIngestPort::default();
        let result = ingest_domain_event_file_with_port(repo, rel, &port);
        assert_eq!(result["ok"].as_bool(), Some(true), "{result}");
        let record_id = result["record_id"].as_str().unwrap();
        assert_eq!(result["store_backend"].as_str(), Some("lancedb"));
        assert_eq!(result["polarity"].as_str(), Some("EfficientSymmetry"));
        assert!(!record_id.is_empty());
        assert_eq!(port.records.borrow().len(), 1);

        let second = ingest_domain_event_file_with_port(repo, rel, &port);
        assert_eq!(second["ok"].as_bool(), Some(true), "{second}");
        assert_eq!(second["skipped"].as_str(), Some("already_indexed"));
    });
}

#[test]
fn lancedb_uri_resolves_under_vector_store_root() {
    with_redb_lab(|repo, _rel| {
        let uri = lancedb_uri(repo);
        assert_eq!(uri, vector_store_root(repo).join("lancedb"));
    });
}

#[test]
fn json_fallback_is_not_used() {
    with_redb_lab(|repo, rel| {
        let port = MockEvolutionIngestPort::default();
        let result = ingest_domain_event_file_with_port(repo, rel, &port);
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
