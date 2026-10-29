use std::fs;
use std::path::{Path, PathBuf};

use execute_process::engine::eda_bus_topology::{
    load_event_bus_topology, write_processing_witness,
};
use execute_process::engine::progress_trace::emit_progress_trace;
use execute_process::engine::route_domain_core::materialize_pending_domain_event;
use sddia_event_store::{override_store_mode, runtime_db_path, EventStore, StoreMode};
use serde_json::json;

struct ModeGuard;

impl Drop for ModeGuard {
    fn drop(&mut self) {
        override_store_mode(None);
    }
}

fn temp_repo() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "sddia-writers-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    fs::create_dir_all(&path).unwrap();
    path
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
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

#[test]
fn redb_domain_event_has_no_new_json() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    let repo = temp_repo();
    materialize_pending_domain_event(
        &repo,
        "Domain_Entity_Created",
        "tekton",
        json!({"name": "a"}),
    )
    .expect("materialize");
    assert!(json_files(&repo.join(".events")).is_empty());
    let store = EventStore::open(runtime_db_path(&repo)).expect("open");
    let rows = store
        .list_family(sddia_event_store::Family::Domain)
        .expect("list");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].event_type, "Domain_Entity_Created");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn filesystem_still_writes_the_pending_json() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Filesystem));
    let repo = temp_repo();
    let rel = materialize_pending_domain_event(
        &repo,
        "Domain_Entity_Created",
        "tekton",
        json!({"name": "b"}),
    )
    .expect("materialize");
    let path = repo.join(&rel);
    assert!(path.is_file(), "{rel}");
    assert!(!runtime_db_path(&repo).is_file());
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn dual_file_and_two_subscriber_rows() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Dual));
    let repo = temp_repo();
    let rel = materialize_pending_domain_event(
        &repo,
        "Domain_Entity_Created",
        "tekton",
        json!({"name": "c"}),
    )
    .expect("materialize");
    let path = repo.join(&rel);
    let body: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    let event_id = body["event_id"].as_str().unwrap();
    let bus = load_event_bus_topology(&repo).unwrap();
    write_processing_witness(
        &repo,
        &bus,
        event_id,
        "argos",
        "Domain_Entity_Created",
        "async",
    )
    .unwrap();
    write_processing_witness(
        &repo,
        &bus,
        event_id,
        "cumulo",
        "Domain_Entity_Created",
        "async",
    )
    .unwrap();
    let store = EventStore::open(runtime_db_path(&repo)).unwrap();
    let stored = store.get_event(event_id).unwrap().unwrap();
    assert_eq!(stored.event_id, event_id);
    assert!(store.get_delivery(event_id, "argos").unwrap().is_some());
    assert!(store.get_delivery(event_id, "cumulo").unwrap().is_some());
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn progress_frames_land_in_the_store() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    let repo = temp_repo();
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
    let store = EventStore::open(runtime_db_path(&repo)).unwrap();
    let frames = store.list_progress(cid).unwrap();
    assert_eq!(frames.len(), 2);
    assert_eq!(frames[0].0, 0);
    assert_eq!(frames[1].0, 1);
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn terminal_witness_exists_redb_without_fs_witness() {
    use execute_process::engine::eda_bus_topology::{
        load_event_bus_topology, terminal_witness_exists,
    };

    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    let repo = temp_repo();
    materialize_pending_domain_event(
        &repo,
        "PBI_Refined",
        "refine-item",
        json!({"pbi_ref": "AUT-45"}),
    )
    .expect("materialize");
    let store = EventStore::open(runtime_db_path(&repo)).unwrap();
    let rows = store
        .list_family(sddia_event_store::Family::Domain)
        .unwrap();
    assert_eq!(rows.len(), 1);
    let event_id = rows[0].event_id.clone();
    store
        .upsert_delivery(
            &event_id,
            "tracker-stamp",
            sddia_event_store::DeliveryStatus::Success,
            json!({"ok": true}),
        )
        .expect("upsert");
    drop(store);
    let bus = load_event_bus_topology(&repo).unwrap();
    assert!(
        terminal_witness_exists(&repo, &bus, &event_id, "tracker-stamp"),
        "store Success debe contar como terminal sin testigo FS"
    );
    assert!(json_files(&repo.join(".events")).is_empty());
    let _ = fs::remove_dir_all(&repo);
}
