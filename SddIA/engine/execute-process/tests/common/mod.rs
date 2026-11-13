//! Laboratorio compartido — matriz I/O ECST × backend `EventManagerPort`.

#![allow(dead_code)]

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use execute_process::core::repo::find_repo_root;
use execute_process::engine::eda_port;
use sddia_core::ports::event_manager::InMemoryEventManagerMock;
use std::sync::{Mutex, MutexGuard, OnceLock};
use sddia_eda_catalog::stamp_published_event;
use sddia_event_manager_host::{open_lab_redb_port, productive_redb_file};
use serde_json::{json, Map, Value};
use uuid::Uuid;

const LAB_COPY: &[&str] = &[
    "SddIA/core/cumulo.paths.json",
    "SddIA/core/event-subscriptions.json",
    "SddIA/core/event-domain-subscriptions.json",
    "SddIA/core/event-telemetry-subscriptions.json",
    "SddIA/core/event-orchestration-subscriptions.json",
    "SddIA/library/codexes/codex-software-engineering/subscriptions.json",
    "SddIA/core/capability-bindings.md",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabBackend {
    Mock,
    Redb,
}

static PARALLEL_LAB: OnceLock<Mutex<()>> = OnceLock::new();

fn parallel_lab_mutex() -> MutexGuard<'static, ()> {
    PARALLEL_LAB
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub struct BackendGuard {
    _parallel: MutexGuard<'static, ()>,
}

pub fn install_backend(repo: &Path, backend: LabBackend) -> BackendGuard {
    let parallel = parallel_lab_mutex();
    eda_port::clear_port();
    match backend {
        LabBackend::Mock => {
            eda_port::install_port(Arc::new(InMemoryEventManagerMock::new()));
        }
        LabBackend::Redb => {
            let port = open_lab_redb_port(repo).expect("open lab redb port");
            eda_port::install_port(port);
        }
    }
    BackendGuard {
        _parallel: parallel,
    }
}

pub fn with_backend<F: FnOnce() -> R, R>(repo: &Path, backend: LabBackend, f: F) -> R {
    let _g = install_backend(repo, backend);
    f()
}

pub fn lab_repo() -> PathBuf {
    let root = find_repo_root().expect("repo root");
    let repo = std::env::temp_dir().join(format!(
        "sddia-store-matrix-{}-{}",
        std::process::id(),
        Uuid::new_v4()
    ));
    fs::create_dir_all(&repo).unwrap();
    for rel in LAB_COPY {
        let src = root.join(rel);
        let dst = repo.join(rel);
        if src.is_file() {
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(&src, &dst).unwrap_or_else(|e| panic!("copy {rel}: {e}"));
        }
    }
    for rel in [
        ".events/pending",
        ".events/domain",
        ".events/telemetry",
        ".events/orchestration",
        ".SddIA/runtime",
    ] {
        fs::create_dir_all(repo.join(rel)).unwrap();
    }
    repo
}

pub fn event_types_from_json(rel: &str) -> Vec<String> {
    let root = find_repo_root().expect("repo");
    let text = fs::read_to_string(root.join(rel)).expect("read subscriptions");
    let map: Map<String, Value> = serde_json::from_str(&text).expect("parse subscriptions");
    map.keys().cloned().collect()
}

pub fn pending_bus_event_types() -> Vec<String> {
    let mut set: BTreeSet<String> = event_types_from_json("SddIA/core/event-subscriptions.json")
        .into_iter()
        .collect();
    for t in event_types_from_json("SddIA/core/event-domain-subscriptions.json") {
        set.insert(t);
    }
    set.into_iter().collect()
}

pub fn telemetry_event_types() -> Vec<String> {
    let mut types = event_types_from_json("SddIA/core/event-telemetry-subscriptions.json");
    if !types.iter().any(|t| t == "System_Vitality_Probed") {
        types.push("System_Vitality_Probed".into());
    }
    types
}

pub fn orchestration_event_types() -> Vec<String> {
    let mut types = event_types_from_json("SddIA/core/event-orchestration-subscriptions.json");
    for extra in ["TQM_Dispatch_Discarded", "Local_QA_Requested"] {
        if !types.iter().any(|t| t == extra) {
            types.push(extra.into());
        }
    }
    types
}

pub fn minimal_payload(event_type: &str) -> Value {
    match event_type {
        "Domain_Entity_Created" | "Domain_Entity_Updated" | "Domain_Entity_Deleted" => json!({
            "entity_uuid": Uuid::new_v4().to_string(),
            "entity_name": "matrix-lab",
            "entity_class": "process",
            "lifecycle_operation": match event_type {
                "Domain_Entity_Created" => "create",
                "Domain_Entity_Updated" => "update",
                _ => "delete",
            },
            "origin_topology": "core",
        }),
        "Domain_Entity_Telemetry_Captured" => json!({
            "entity_id": "ent-matrix",
            "entity_type": "process",
            "execution_metrics": { "success_status": true, "duration_ms": 1, "exit_code": 0 },
        }),
        "System_Fracture_Detected" => json!({
            "process_name": "matrix-lab",
            "error_trace": "matrix roundtrip",
            "agent_emitter": "eda-matrix-lab",
        }),
        "PullRequest_Presented" | "PullRequest_Merged" | "PullRequest_Audited"
        | "PullRequest_Resealed" => json!({
            "branch": "feat/matrix",
            "tracker_ref": "AUT-MATRIX",
            "source_branch": "feat/matrix",
            "target_branch": "main",
        }),
        "Work_Initiated" => json!({
            "branch": "feat/matrix",
            "tracker_ref": "AUT-MATRIX",
            "persist_ref": "docs/features/eda-store-io-gate",
        }),
        "Tracker_Issue_Changed" => json!({
            "issue_ref": "AUT-MATRIX",
            "project_slug": "demo",
            "occurred_at": "2026-10-30T12:00:00Z",
            "changes": [],
        }),
        "PBI_Refined" | "HU_Refined" | "PBI_Forged" | "PBI_Cancelled" => {
            json!({ "pbi_ref": "AUT-MATRIX", "tracker_ref": "AUT-MATRIX" })
        }
        "Tracker_Sync_Failed" => json!({ "tracker_ref": "AUT-MATRIX", "reason": "lab" }),
        "Kaizen_Alert_Required" => json!({
            "review_id": "matrix",
            "justification": "lab",
            "implicated_processes": [],
        }),
        "CapabilityDi_Requested" => json!({
            "process_name": "feature",
            "phase_name": "lab",
            "requires_capability": [{"id": "doc:closure", "contract": "doc.closure"}],
        }),
        "Raw_Execution_Finished" => json!({
            "process_name": "feature",
            "success": true,
            "duration_ms": 1,
        }),
        "Daemon_Heartbeat" => json!({
            "daemon_name": "event-watcher",
            "status": "alive",
        }),
        "CI_Job_Failed" => json!({
            "workflow": "ci",
            "job": "matrix",
            "run_id": "1",
        }),
        "System_Vitality_Probed" => json!({
            "verdict": "ok",
            "probes": [],
        }),
        "Process_Execution_Completed" => json!({
            "process_name": "feature",
            "correlation_id": Uuid::new_v4().to_string(),
            "status": "success",
        }),
        "Fracture_Clarification_Requested" => json!({
            "fracture_event_id": Uuid::new_v4().to_string(),
        }),
        "Email_Received" | "Email_Triaged" => json!({
            "message_id": "msg-matrix",
            "mailbox": "lab",
        }),
        _ => json!({ "matrix": true }),
    }
}

pub fn build_event(event_type: &str) -> Value {
    json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": event_type,
        "timestamp": "2026-10-30T12:00:00Z",
        "emitter_agent": "eda-matrix-lab",
        "payload": minimal_payload(event_type),
    })
}

pub fn roundtrip_publish_load(
    repo: &Path,
    store_family: &str,
    backend: LabBackend,
    event_type: &str,
) {
    with_backend(repo, backend, || {
        let mut event = build_event(event_type);
        stamp_published_event(repo, &mut event)
            .unwrap_or_else(|e| panic!("stamp {event_type}: {e}"));
        let event_id = event["event_id"].as_str().unwrap().to_string();
        let _seal = eda_port::publish_ecst(repo, store_family, &event)
            .unwrap_or_else(|e| panic!("publish {event_type} backend={backend:?}: {e}"));
        let json_path = repo
            .join(".events")
            .join(store_family)
            .join(format!("{event_id}.json"));
        assert!(
            !json_path.is_file(),
            "{event_type}: puerto no debe dejar JSON en {}",
            json_path.display()
        );
        if backend == LabBackend::Redb {
            assert!(
                productive_redb_file(repo).unwrap().is_file(),
                "{event_type}: falta events.redb"
            );
        }
        let logical_path = repo.join(format!(".events/{store_family}/{event_id}.json"));
        let loaded = eda_port::load_dispatched(repo, &event_id, &logical_path)
            .unwrap_or_else(|e| panic!("load {event_type}: {e}"))
            .unwrap_or_else(|| panic!("load {event_type}: None"));
        assert_eq!(
            loaded.get("event_id").and_then(|v| v.as_str()),
            Some(event_id.as_str())
        );
        assert_eq!(
            loaded.get("event_type").and_then(|v| v.as_str()),
            Some(event_type)
        );
    });
}

/// TM-8: tras stamp + publish en Redb, el evento queda accesible en cola pending del puerto.
pub fn assert_pending_subscriber_deliveries(repo: &Path, event_type: &str, _min_rows: usize) {
    use sddia_core::ports::event_manager::ListPendingRequest;
    with_backend(repo, LabBackend::Redb, || {
        let mut event = build_event(event_type);
        stamp_published_event(repo, &mut event)
            .unwrap_or_else(|e| panic!("stamp {event_type}: {e}"));
        let event_id = event["event_id"].as_str().unwrap().to_string();
        eda_port::publish_ecst(repo, "pending", &event)
            .unwrap_or_else(|e| panic!("publish {event_type}: {e}"));
        eda_port::with_event_port(repo, "matrix_list_pending", |port| {
            let pending = port
                .list_pending(ListPendingRequest {
                    limit: Some(500),
                })
                .map_err(|e| e.to_string())?;
            assert!(
                pending
                    .events
                    .iter()
                    .any(|row| row.event_id == event_id),
                "{event_type}: pending no contiene {event_id}"
            );
            Ok(())
        })
        .expect("list_pending");
    });
}

pub fn cleanup(repo: &Path) {
    let _ = fs::remove_dir_all(repo);
}
