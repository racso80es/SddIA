//! Matriz pending bus — publish_ecst + load_dispatched_event por `event_type`.

mod common;

use common::{
    cleanup, lab_repo, pending_bus_event_types, roundtrip_publish_load, with_mode,
};
use sddia_event_store::{load_domain_ecsts, StoreMode};

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
        roundtrip_publish_load(
            &repo,
            "pending",
            &repo.join(".events/pending"),
            StoreMode::Redb,
            event_type,
        );
    }
    with_mode(StoreMode::Redb, || {
        let rows = load_domain_ecsts(&repo, &[repo.join(".events/pending")]).expect("load_domain");
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
fn pending_bus_matrix_filesystem_all_catalog_types() {
    let repo = lab_repo();
    for event_type in pending_bus_event_types() {
        roundtrip_publish_load(
            &repo,
            "pending",
            &repo.join(".events/pending"),
            StoreMode::Filesystem,
            &event_type,
        );
    }
    cleanup(&repo);
}

#[test]
fn pending_bus_matrix_dual_stratified() {
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
        roundtrip_publish_load(
            &repo,
            "pending",
            &repo.join(".events/pending"),
            StoreMode::Dual,
            event_type,
        );
    }
    cleanup(&repo);
}
