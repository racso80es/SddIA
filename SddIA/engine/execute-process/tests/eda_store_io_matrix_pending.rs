//! Matriz pending bus — publish_ecst + load_dispatched vía EventManagerPort.

mod common;

use common::{
    assert_pending_subscriber_deliveries, cleanup, lab_repo, pending_bus_event_types,
    roundtrip_publish_load, with_backend, LabBackend,
};
use execute_process::engine::eda_port;

#[test]
fn pending_bus_matrix_redb_all_catalog_types() {
    let repo = lab_repo();
    let types = pending_bus_event_types();
    assert!(
        types.len() >= 20,
        "catálogo pending demasiado pequeño: {}",
        types.len()
    );
    for event_type in &types {
        roundtrip_publish_load(&repo, "pending", LabBackend::Redb, event_type);
    }
    with_backend(&repo, LabBackend::Redb, || {
        let rows = eda_port::load_domain_ecsts(&repo, types.len() as u32).expect("load_domain");
        assert!(
            rows.len() >= types.len(),
            "store debe contener al menos {} filas, got {}",
            types.len(),
            rows.len()
        );
    });
    cleanup(&repo);
}

#[test]
fn pending_bus_matrix_mock_all_catalog_types() {
    let repo = lab_repo();
    for event_type in pending_bus_event_types() {
        roundtrip_publish_load(&repo, "pending", LabBackend::Mock, &event_type);
    }
    cleanup(&repo);
}

#[test]
fn pending_bus_matrix_redb_stratified_sample() {
    let repo = lab_repo();
    let sample = [
        "Domain_Entity_Created",
        "System_Fracture_Detected",
        "PullRequest_Presented",
        "Kaizen_Alert_Required",
        "Thought_Persisted",
        "Work_Initiated",
    ];
    for event_type in sample {
        roundtrip_publish_load(&repo, "pending", LabBackend::Redb, event_type);
    }
    cleanup(&repo);
}

#[test]
fn tm8_presented_has_pending_subscriber_rows_after_stamp() {
    let repo = lab_repo();
    assert_pending_subscriber_deliveries(&repo, "PullRequest_Presented", 1);
    cleanup(&repo);
}
