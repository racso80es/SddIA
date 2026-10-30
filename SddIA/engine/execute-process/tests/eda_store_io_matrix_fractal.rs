//! Matriz fractal — telemetría, orquestación y dominio bajo `.events/{family}/`.

mod common;

use common::{
    cleanup, lab_repo, orchestration_event_types, roundtrip_publish_load, telemetry_event_types,
};
use execute_process::engine::route_fractal_core::route_telemetry_event;
use sddia_event_store::StoreMode;

#[test]
fn fractal_telemetry_matrix_redb() {
    let repo = lab_repo();
    for event_type in telemetry_event_types() {
        roundtrip_publish_load(
            &repo,
            "telemetry",
            &repo.join(".events/telemetry"),
            StoreMode::Redb,
            &event_type,
        );
    }
    cleanup(&repo);
}

#[test]
fn fractal_orchestration_matrix_redb() {
    let repo = lab_repo();
    for event_type in orchestration_event_types() {
        roundtrip_publish_load(
            &repo,
            "orchestration",
            &repo.join(".events/orchestration"),
            StoreMode::Redb,
            &event_type,
        );
    }
    cleanup(&repo);
}

#[test]
fn fractal_domain_matrix_redb_sample() {
    let repo = lab_repo();
    let sample = [
        "Domain_Entity_Telemetry_Captured",
        "Fracture_Pbi_Materialized",
        "Local_QA_Requested",
        "Kalma2_Process_Requested",
    ];
    for event_type in sample {
        roundtrip_publish_load(
            &repo,
            "domain",
            &repo.join(".events/domain"),
            StoreMode::Redb,
            event_type,
        );
    }
    cleanup(&repo);
}

#[test]
fn route_telemetry_loads_from_store_without_json_file() {
    let repo = lab_repo();
    common::with_mode(StoreMode::Redb, || {
        let event_type = "Raw_Execution_Finished";
        common::roundtrip_publish_load(
            &repo,
            "telemetry",
            &repo.join(".events/telemetry"),
            StoreMode::Redb,
            event_type,
        );
        let store = sddia_event_store::EventStore::open(sddia_event_store::runtime_db_path(&repo))
            .expect("open");
        let row = store
            .list_family(sddia_event_store::Family::Telemetry)
            .expect("list")
            .into_iter()
            .find(|r| r.event_type == event_type)
            .expect("row");
        let rel = format!(".events/telemetry/{}.json", row.event_id);
        assert!(!repo.join(&rel).is_file());
        let out = route_telemetry_event(&repo, &rel);
        assert_ne!(
            out.get("error")
                .and_then(|v| v.as_str())
                .unwrap_or(""),
            "event file not found",
            "route debe cargar desde store: {out}"
        );
    });
    cleanup(&repo);
}

#[test]
fn fractal_telemetry_matrix_filesystem() {
    let repo = lab_repo();
    for event_type in ["Raw_Execution_Finished", "Daemon_Heartbeat"] {
        roundtrip_publish_load(
            &repo,
            "telemetry",
            &repo.join(".events/telemetry"),
            StoreMode::Filesystem,
            event_type,
        );
    }
    cleanup(&repo);
}
