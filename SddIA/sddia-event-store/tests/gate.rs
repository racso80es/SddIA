use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use sddia_event_store::{
    append_progress_monotonic, mode_from_name, override_store_mode, path_is_heartbeat_sidechannel,
    record_ecst, runtime_db_path, DeliveryStatus, EventStore, Family, StoreMode,
};

static TEMP_SEQ: AtomicU64 = AtomicU64::new(1);

struct TempRepo {
    path: PathBuf,
}

impl TempRepo {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("reloj")
            .as_nanos();
        let seq = TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "sddia-gate-{}-{}-{}",
            std::process::id(),
            seq,
            nanos
        ));
        fs::create_dir_all(&path).expect("tmpdir");
        Self { path }
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        override_store_mode(None);
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn mode_names_default_to_redb() {
    assert_eq!(mode_from_name(None), StoreMode::Redb);
    assert_eq!(mode_from_name(Some("")), StoreMode::Redb);
    assert_eq!(mode_from_name(Some("unknown")), StoreMode::Redb);
    assert_eq!(mode_from_name(Some("filesystem")), StoreMode::Filesystem);
    assert_eq!(mode_from_name(Some("dual")), StoreMode::Dual);
    assert_eq!(mode_from_name(Some("redb")), StoreMode::Redb);
}

#[test]
fn progress_seq_is_monotonic_and_heartbeat_path_is_not_a_row() {
    let repo = TempRepo::new();
    override_store_mode(Some(StoreMode::Redb));
    let frame = serde_json::json!({"trace_id": "t1"});
    let first = append_progress_monotonic(&repo.path, "corr-1", &frame).expect("seq");
    let second = append_progress_monotonic(&repo.path, "corr-1", &frame).expect("seq");
    assert_eq!((first, second), (0, 1));
    let store = EventStore::open(runtime_db_path(&repo.path)).expect("open");
    let frames = store.list_progress("corr-1").expect("list");
    assert_eq!(frames.len(), 2);

    let beat = repo
        .path
        .join(".SddIA/daemons/state/heartbeats/event-watcher.json");
    assert!(path_is_heartbeat_sidechannel(&beat));
    fs::create_dir_all(beat.parent().unwrap()).unwrap();
    fs::write(&beat, "{\"source\":\"side-channel\"}\n").unwrap();
    assert!(store.list_family(Family::Telemetry).unwrap().is_empty());
}

#[test]
fn dual_keeps_two_subscriber_rows() {
    let repo = TempRepo::new();
    override_store_mode(Some(StoreMode::Dual));
    let event = serde_json::json!({
        "event_id": "evt-1",
        "event_type": "Domain_Entity_Created",
        "delivery_state": {"argos": "pending", "cumulo": "pending"}
    });
    record_ecst(&repo.path, "domain", &event).expect("record");
    let store = EventStore::open(runtime_db_path(&repo.path)).expect("open");
    let stored = store.get_event("evt-1").expect("get").expect("row");
    assert_eq!(stored.family, Family::Domain);
    assert_eq!(
        store
            .get_delivery("evt-1", "argos")
            .unwrap()
            .unwrap()
            .status,
        DeliveryStatus::Pending
    );
    assert_eq!(
        store
            .get_delivery("evt-1", "cumulo")
            .unwrap()
            .unwrap()
            .status,
        DeliveryStatus::Pending
    );
    assert!(store.get_delivery("evt-1", "otro").unwrap().is_none());
    let mut pending = store.list_pending_deliveries().unwrap();
    pending.sort();
    assert_eq!(
        pending,
        vec![
            ("evt-1".into(), "argos".into()),
            ("evt-1".into(), "cumulo".into()),
        ]
    );
}

#[test]
fn domain_without_plan_is_rejected() {
    let repo = TempRepo::new();
    override_store_mode(Some(StoreMode::Redb));
    let event = serde_json::json!({
        "event_id": "evt-empty",
        "event_type": "PBI_Refined",
        "delivery_state": {}
    });
    let err = record_ecst(&repo.path, "domain", &event).unwrap_err();
    assert!(err.to_string().contains("EDA_DELIVERY_PLAN_MISSING"));
}

#[test]
fn success_label_is_not_pending() {
    let repo = TempRepo::new();
    override_store_mode(Some(StoreMode::Redb));
    let event = serde_json::json!({
        "event_id": "evt-ok",
        "event_type": "PBI_Refined",
        "delivery_state": {"tekton.tracker-stamp": "success"}
    });
    record_ecst(&repo.path, "domain", &event).unwrap();
    let store = EventStore::open(runtime_db_path(&repo.path)).unwrap();
    assert_eq!(
        store.get_delivery("evt-ok", "tekton.tracker-stamp").unwrap().unwrap().status,
        DeliveryStatus::Success
    );
    assert!(store.list_pending_deliveries().unwrap().is_empty());
}
