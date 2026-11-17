//! Matriz fractal — telemetría, orquestación y dominio vía EventManagerPort.

mod common;

use common::{
    cleanup, lab_repo, orchestration_event_types, roundtrip_publish_load, telemetry_event_types,
    with_backend, LabBackend,
};
use execute_process::engine::eda_port;
use execute_process::engine::route_fractal_core::route_telemetry_event;
use sddia_core::ports::event_manager::ListEventsRequest;

#[test]
fn fractal_telemetry_matrix_redb() {
    let repo = lab_repo();
    for event_type in telemetry_event_types() {
        roundtrip_publish_load(&repo, "telemetry", LabBackend::Redb, &event_type);
    }
    cleanup(&repo);
}

#[test]
fn fractal_orchestration_matrix_redb() {
    let repo = lab_repo();
    for event_type in orchestration_event_types() {
        roundtrip_publish_load(&repo, "orchestration", LabBackend::Redb, &event_type);
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
        roundtrip_publish_load(&repo, "domain", LabBackend::Redb, event_type);
    }
    cleanup(&repo);
}

#[test]
fn route_telemetry_loads_from_store_without_json_file() {
    let repo = lab_repo();
    let event_type = "Raw_Execution_Finished";
    roundtrip_publish_load(&repo, "telemetry", LabBackend::Redb, event_type);
    with_backend(&repo, LabBackend::Redb, || {
        let row = eda_port::with_event_port(&repo, "matrix_list_telemetry", |port| {
            port.list_events(ListEventsRequest {
                family: Some("telemetry".into()),
                event_type: Some(event_type.into()),
                state: None,
                limit: Some(10),
                cursor: None,
            })
            .map_err(|e| e.to_string())
        })
        .expect("list_events")
        .events
        .into_iter()
        .find(|e| {
            e.ecst
                .get("event_type")
                .and_then(|v| v.as_str())
                == Some(event_type)
        })
        .expect("row");
        let rel = format!(".events/telemetry/{}.json", row.event_id);
        assert!(!repo.join(&rel).is_file());
        let out = route_telemetry_event(&repo, &rel);
        assert_ne!(
            out.get("error")
                .and_then(|v| v.as_str())
                .unwrap_or(""),
            "event file not found",
            "route debe cargar desde puerto: {out}"
        );
    });
    cleanup(&repo);
}

#[test]
fn fractal_telemetry_matrix_mock() {
    let repo = lab_repo();
    for event_type in ["Raw_Execution_Finished", "Daemon_Heartbeat"] {
        roundtrip_publish_load(&repo, "telemetry", LabBackend::Mock, event_type);
    }
    cleanup(&repo);
}
