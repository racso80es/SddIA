//! AUT-21 — puerto ECST por parámetros, aduana handlers, no-op no success.

mod common;

use std::fs;
use std::path::Path;

use common::{lab_repo, with_mode, ModeGuard};
use execute_process::engine::ecst_input_loader::load_ecst_from_inputs;
use execute_process::engine::handlers::tracker_stamp;
use execute_process::engine::route_domain_core::materialize_pending_domain_event;
use sddia_event_store::{EventStore, Family, StoreMode, runtime_db_path};
use serde_json::json;

fn seed_store_only_pending_rel(
    repo: &Path,
    event_type: &str,
    payload: serde_json::Value,
) -> String {
    materialize_pending_domain_event(repo, event_type, "test", payload).expect("materialize");
    let store = EventStore::open(runtime_db_path(repo)).expect("open store");
    let rows = store.list_family(Family::Domain).expect("list");
    let event_id = rows.last().expect("event row").event_id.clone();
    drop(store);
    let pending = repo.join(".events/pending").join(format!("{event_id}.json"));
    if pending.is_file() {
        fs::remove_file(&pending).expect("remove pending");
    }
    format!(".events/pending/{event_id}.json")
}

#[test]
fn handlers_must_not_fs_read_subscriber_event_path() {
    let handlers = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/engine/handlers");
    let forbidden = "fs::read_to_string(&path)";
    let watch = ["tracker_sync_replay.rs"];
    for name in watch {
        let path = handlers.join(name);
        let text = fs::read_to_string(&path).expect("read handler");
        assert!(
            !text.contains(forbidden),
            "{name} no debe leer ECST con {forbidden}; usar ecst_input_loader"
        );
    }
}

#[test]
fn load_ecst_accepts_inline_event_without_pending_file() {
    let _guard = ModeGuard;
    with_mode(StoreMode::Redb, || {
        let repo = lab_repo();
        let rel = seed_store_only_pending_rel(
            &repo,
            "Work_Initiated",
            json!({
                "tracker_ref": "LAB-PBI-OK",
                "branch": "feat/x",
                "persist_ref": "docs/features/x",
            }),
        );
        let event = load_ecst_from_inputs(&repo, &json!({ "event_file_path": rel })).expect("load");
        let eid = event.get("event_id").and_then(|v| v.as_str()).unwrap().to_string();
        let inline = load_ecst_from_inputs(
            &repo,
            &json!({ "event": event, "event_id": eid }),
        )
        .expect("inline load");
        assert_eq!(
            inline.get("event_id").and_then(|v| v.as_str()),
            Some(eid.as_str())
        );
        let _ = fs::remove_dir_all(&repo);
    });
}

#[test]
fn tracker_stamp_noop_remote_is_not_success() {
    let _guard = ModeGuard;
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
    with_mode(StoreMode::Redb, || {
        let repo = lab_repo();
        let rel = seed_store_only_pending_rel(
            &repo,
            "PBI_Refined",
            json!({
                "item_ref": "AUT-99",
            }),
        );
        let event = load_ecst_from_inputs(&repo, &json!({ "event_file_path": rel })).expect("load");
        let env = tracker_stamp::run(
            &repo,
            &json!({
                "event": event,
                "event_id": event.get("event_id").cloned().unwrap_or(json!("")),
            }),
        )
        .expect("run");
        let status = env
            .data
            .as_ref()
            .and_then(|d| d.get("delivery_status"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        assert_ne!(status, "success");
        assert!(
            status.starts_with("skipped-") || status == "failed",
            "no-op remoto: {status}"
        );
        let _ = fs::remove_dir_all(&repo);
    });
    std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
}
