mod common;

use std::fs;
use std::path::PathBuf;

use common::{cleanup, install_backend, lab_repo, LabBackend};
use execute_process::engine::eda_bus_topology::{
    load_event_bus_topology, write_processing_witness,
};
use execute_process::engine::eda_port;
use execute_process::engine::progress_trace::emit_progress_trace;
use execute_process::engine::route_domain_core::materialize_pending_domain_event;
use sddia_core::ports::event_manager::{DeliveryStatus, ListDeliveriesRequest, ListEventsRequest};
use sddia_event_manager_host::productive_redb_file;
use serde_json::json;

fn json_files(dir: &std::path::Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = fs::read_dir(&current) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out
}

fn event_id_from_pending_rel(rel: &str) -> String {
    std::path::Path::new(rel)
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("pending rel con event_id")
        .to_string()
}

#[test]
fn redb_domain_event_has_no_new_json() {
    let repo = lab_repo();
    let _g = install_backend(&repo, LabBackend::Redb);
    materialize_pending_domain_event(
        &repo,
        "Domain_Entity_Created",
        "tekton",
        json!({"name": "a"}),
    )
    .expect("materialize");
    assert!(json_files(&repo.join(".events")).is_empty());
    eda_port::with_event_port(&repo, "list_events", |port| {
        let listed = port
            .list_events(ListEventsRequest {
                family: Some("domain".into()),
                event_type: Some("Domain_Entity_Created".into()),
                state: None,
                limit: Some(10),
                cursor: None,
            })
            .map_err(|e| e.to_string())?;
        assert_eq!(listed.events.len(), 1);
        Ok(())
    })
    .expect("port list");
    cleanup(&repo);
}

#[test]
fn mock_publish_does_not_materialize_redb_file() {
    let repo = lab_repo();
  common::roundtrip_publish_load(
        &repo,
        "pending",
        LabBackend::Mock,
        "Domain_Entity_Created",
    );
    assert!(!productive_redb_file(&repo).unwrap().is_file());
    cleanup(&repo);
}

#[test]
fn redb_two_subscriber_processing_rows() {
    let repo = lab_repo();
    let _g = install_backend(&repo, LabBackend::Redb);
    let rel = materialize_pending_domain_event(
        &repo,
        "Domain_Entity_Created",
        "tekton",
        json!({"name": "c"}),
    )
    .expect("materialize");
    let event_id = event_id_from_pending_rel(&rel);
    let bus = load_event_bus_topology(&repo).unwrap();
    write_processing_witness(
        &repo,
        &bus,
        &event_id,
        "argos",
        "Domain_Entity_Created",
        "async",
    )
    .unwrap();
    write_processing_witness(
        &repo,
        &bus,
        &event_id,
        "cumulo",
        "Domain_Entity_Created",
        "async",
    )
    .unwrap();
    eda_port::with_event_port(&repo, "list_processing", |port| {
        let rows = port
            .list_deliveries(ListDeliveriesRequest {
                event_id: Some(event_id.to_string()),
                subscriber_id: None,
                status: None,
                limit: Some(100),
                cursor: None,
            })
            .map_err(|e| e.to_string())?;
        let subs: Vec<_> = rows
            .deliveries
            .iter()
            .filter(|d| d.status == DeliveryStatus::Processing)
            .filter_map(|d| d.subscriber_id.as_deref())
            .collect();
        assert!(subs.contains(&"argos"));
        assert!(subs.contains(&"cumulo"));
        Ok(())
    })
    .expect("port deliveries");
    cleanup(&repo);
}

#[test]
fn progress_frames_land_in_the_store() {
    let repo = lab_repo();
    let _g = install_backend(&repo, LabBackend::Redb);
    let cid = "corr-writers";
    emit_progress_trace(
        &repo,
        cid,
        "Ejecución",
        &[],
        "start",
        None,
        Some("feature"),
        None,
    );
    emit_progress_trace(
        &repo,
        cid,
        "Ejecución",
        &[],
        "end",
        Some("executed"),
        Some("feature"),
        None,
    );
    assert!(json_files(&repo.join(".events")).is_empty());
    eda_port::with_event_port(&repo, "list_progress", |port| {
        use sddia_core::ports::event_manager::ListProgressRequest;
        let frames = port
            .list_progress(ListProgressRequest {
                trace_id: None,
                correlation_id: Some(cid.to_string()),
            })
            .map_err(|e| e.to_string())?;
        assert_eq!(frames.frames.len(), 2);
        Ok(())
    })
    .expect("port progress");
    cleanup(&repo);
}

#[test]
fn terminal_witness_exists_redb_skipped_without_fs_witness() {
    use execute_process::engine::eda_bus_topology::{promote_witness, terminal_witness_exists};

    let repo = lab_repo();
    let _g = install_backend(&repo, LabBackend::Redb);
    let rel = materialize_pending_domain_event(
        &repo,
        "Work_Initiated",
        "feature",
        json!({"branch": "feat/x"}),
    )
    .expect("materialize");
    let event_id = event_id_from_pending_rel(&rel);
    let bus = load_event_bus_topology(&repo).unwrap();
    write_processing_witness(
        &repo,
        &bus,
        &event_id,
        "tracker-stamp",
        "Work_Initiated",
        "async",
    )
    .unwrap();
    promote_witness(
        &repo,
        &bus,
        &event_id,
        "tracker-stamp",
        "processed",
        Some(&json!({
            "result_status": "skipped-no-tracker-ref",
            "skip_reason": "skipped-no-tracker-ref",
        })),
        None,
    )
    .unwrap();
    assert!(
        terminal_witness_exists(&repo, &bus, &event_id, "tracker-stamp"),
        "Skipped en store debe ser terminal"
    );
    eda_port::with_event_port(&repo, "delivery_ack", |port| {
        let rows = port
            .list_deliveries(ListDeliveriesRequest {
                event_id: Some(event_id.clone()),
                subscriber_id: Some("tracker-stamp".into()),
                status: None,
                limit: Some(10),
                cursor: None,
            })
            .map_err(|e| e.to_string())?;
        assert!(
            rows.deliveries
                .iter()
                .any(|d| d.status == DeliveryStatus::Ack),
            "promote processed debe materializar ACK en puerto"
        );
        Ok(())
    })
    .expect("port ack");
    cleanup(&repo);
}

#[test]
fn terminal_witness_exists_redb_without_fs_witness() {
    use execute_process::engine::eda_bus_topology::{load_event_bus_topology, terminal_witness_exists};

    let repo = lab_repo();
    let _g = install_backend(&repo, LabBackend::Redb);
    let rel = materialize_pending_domain_event(
        &repo,
        "PBI_Refined",
        "refine-item",
        json!({"pbi_ref": "AUT-45"}),
    )
    .expect("materialize");
    let event_id = event_id_from_pending_rel(&rel);
    eda_port::record_delivery_ack(&repo, &event_id, "tracker-stamp").expect("ack");
    let bus = load_event_bus_topology(&repo).unwrap();
    assert!(
        terminal_witness_exists(&repo, &bus, &event_id, "tracker-stamp"),
        "store Success debe contar como terminal sin testigo FS"
    );
    assert!(json_files(&repo.join(".events")).is_empty());
    cleanup(&repo);
}
