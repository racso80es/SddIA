use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

static TEMP_SEQ: AtomicU64 = AtomicU64::new(1);

use sddia_event_store::{
    DeadLetter, DeliveryStatus, DltSeal, EventStore, Family, StoreError, StoredEvent,
};

struct TempDb {
    path: PathBuf,
}

impl TempDb {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("reloj")
            .as_nanos();
        let seq = TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "sddia-event-store-{}-{}-{}.redb",
            std::process::id(),
            seq,
            nanos
        ));
        Self { path }
    }

    fn open(&self) -> EventStore {
        EventStore::open(&self.path).expect("open")
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn event(id: &str, family: Family, created_at: i64) -> StoredEvent {
    StoredEvent {
        event_id: id.to_string(),
        family,
        event_type: "Domain_Sample".to_string(),
        created_at,
        ecst: serde_json::json!({"event_id": id, "payload": "ecst"}),
    }
}

#[test]
fn ac2_two_subscribers_survive_reopen() {
    let db = TempDb::new();
    let id = "11111111-1111-4111-8111-111111111111";
    {
        let store = db.open();
        store
            .insert_event(&event(id, Family::Domain, 10), &["alpha", "beta"])
            .unwrap();
        store
            .set_delivery(
                id,
                "alpha",
                DeliveryStatus::Success,
                serde_json::json!({"n": 1}),
            )
            .unwrap();
        store
            .set_delivery(
                id,
                "beta",
                DeliveryStatus::Failed,
                serde_json::json!({"n": 2}),
            )
            .unwrap();
    }
    let store = db.open();
    let alpha = store.get_delivery(id, "alpha").unwrap().unwrap();
    let beta = store.get_delivery(id, "beta").unwrap().unwrap();
    assert_eq!(alpha.status, DeliveryStatus::Success);
    assert_eq!(alpha.witness, serde_json::json!({"n": 1}));
    assert_eq!(beta.status, DeliveryStatus::Failed);
    assert_eq!(beta.witness, serde_json::json!({"n": 2}));
}

#[test]
fn key_dimension_does_not_corrupt_neighbor() {
    let db = TempDb::new();
    let store = db.open();
    let id = "22222222-2222-4222-8222-222222222222";
    store
        .insert_event(&event(id, Family::Telemetry, 5), &["watcher"])
        .unwrap();
    store
        .append_progress(id, 7, serde_json::json!({"frame": "progress"}))
        .unwrap();
    store.dead_letter(id, "timeout", "watcher").unwrap();
    store
        .seal_dlt(
            id,
            &DltSeal {
                tx_hash: "tx-1".into(),
                digest: "dig-1".into(),
                scope: "governance".into(),
            },
        )
        .unwrap();
    store
        .set_delivery(
            id,
            "watcher",
            DeliveryStatus::Skipped,
            serde_json::json!({"why": "s"}),
        )
        .unwrap();

    let stored = store.get_event(id).unwrap().unwrap();
    assert_eq!(stored.ecst["payload"], "ecst");
    assert_eq!(stored.family, Family::Telemetry);
    let listed = store.list_family(Family::Telemetry).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].event_id, id);
    assert!(store.list_family(Family::Domain).unwrap().is_empty());
    assert_eq!(
        store.list_progress(id).unwrap(),
        vec![(7, serde_json::json!({"frame": "progress"}))]
    );
    assert_eq!(
        store.get_dead_letter(id).unwrap().unwrap(),
        DeadLetter {
            cause: "timeout".into(),
            failed_subscriber_id: "watcher".into(),
        }
    );
    assert_eq!(store.get_seal(id).unwrap().unwrap().tx_hash, "tx-1");
    let witness = store.get_delivery(id, "watcher").unwrap().unwrap();
    assert_eq!(witness.status, DeliveryStatus::Skipped);
    assert_eq!(witness.witness["why"], "s");
}

#[test]
fn insert_duplicate_does_not_replace_witnesses() {
    let db = TempDb::new();
    let store = db.open();
    let id = "33333333-3333-4333-8333-333333333333";
    let mut first = event(id, Family::Orchestration, 1);
    store.insert_event(&first, &["a", "b"]).unwrap();
    store
        .set_delivery(
            id,
            "a",
            DeliveryStatus::Success,
            serde_json::json!({"kept": true}),
        )
        .unwrap();
    first.ecst = serde_json::json!({"replaced": true});
    let err = store.insert_event(&first, &["c"]).unwrap_err();
    assert!(matches!(err, StoreError::Conflict(_)));
    assert!(store.get_delivery(id, "c").unwrap().is_none());
    assert_eq!(
        store.get_delivery(id, "a").unwrap().unwrap().status,
        DeliveryStatus::Success
    );
    assert_eq!(
        store.get_event(id).unwrap().unwrap().ecst["payload"],
        "ecst"
    );
}

#[test]
fn replay_resets_only_failed_subscriber() {
    let db = TempDb::new();
    let store = db.open();
    let id = "44444444-4444-4444-8444-444444444444";
    store
        .insert_event(&event(id, Family::Domain, 9), &["alpha", "beta"])
        .unwrap();
    store
        .set_delivery(
            id,
            "alpha",
            DeliveryStatus::Success,
            serde_json::json!({"keep": true}),
        )
        .unwrap();
    store
        .set_delivery(
            id,
            "beta",
            DeliveryStatus::Failed,
            serde_json::json!({"err": "x"}),
        )
        .unwrap();
    store.dead_letter(id, "timeout", "beta").unwrap();
    store.replay(id).unwrap();

    assert!(store.get_dead_letter(id).unwrap().is_none());
    let beta = store.get_delivery(id, "beta").unwrap().unwrap();
    assert_eq!(beta.status, DeliveryStatus::Pending);
    assert_eq!(beta.witness, serde_json::json!({"err": "x"}));
    let alpha = store.get_delivery(id, "alpha").unwrap().unwrap();
    assert_eq!(alpha.status, DeliveryStatus::Success);
    assert_eq!(alpha.witness, serde_json::json!({"keep": true}));
    assert!(store.get_event(id).unwrap().is_some());
    assert!(matches!(
        store.replay(id).unwrap_err(),
        StoreError::NotFound(_)
    ));
}

#[test]
fn list_family_orders_by_created_at() {
    let db = TempDb::new();
    let store = db.open();
    store
        .insert_event(&event("late", Family::Domain, 30), &[])
        .unwrap();
    store
        .insert_event(&event("early", Family::Domain, 10), &[])
        .unwrap();
    store
        .insert_event(&event("other", Family::Orchestration, 1), &[])
        .unwrap();
    let ids: Vec<_> = store
        .list_family(Family::Domain)
        .unwrap()
        .into_iter()
        .map(|row| row.event_id)
        .collect();
    assert_eq!(ids, vec!["early".to_string(), "late".to_string()]);
}

#[test]
fn set_delivery_does_not_invent_subscriber() {
    let db = TempDb::new();
    let store = db.open();
    let id = "55555555-5555-4555-8555-555555555555";
    store
        .insert_event(&event(id, Family::Domain, 1), &["known"])
        .unwrap();
    let err = store
        .set_delivery(
            id,
            "stranger",
            DeliveryStatus::Success,
            serde_json::json!({}),
        )
        .unwrap_err();
    assert!(matches!(err, StoreError::NotFound(_)));
    assert!(store.get_delivery(id, "stranger").unwrap().is_none());
    assert_eq!(
        store.get_delivery(id, "known").unwrap().unwrap().status,
        DeliveryStatus::Pending
    );
}

#[test]
fn fresh_reads_are_empty_and_duplicate_progress_conflicts() {
    let db = TempDb::new();
    let store = db.open();
    assert!(store.get_event("missing").unwrap().is_none());
    assert!(store.list_family(Family::Domain).unwrap().is_empty());
    assert!(store.list_progress("corr").unwrap().is_empty());
    store
        .append_progress("corr", 1, serde_json::json!({"a": 1}))
        .unwrap();
    let err = store
        .append_progress("corr", 1, serde_json::json!({"a": 2}))
        .unwrap_err();
    assert!(matches!(err, StoreError::Conflict(_)));
    assert_eq!(store.list_progress("corr").unwrap().len(), 1);
}

#[test]
fn concurrent_readers_and_distinct_writers() {
    let db = TempDb::new();
    let store = db.open();
    let id = "66666666-6666-4666-8666-666666666666";
    store
        .insert_event(&event(id, Family::Domain, 4), &["alpha", "beta"])
        .unwrap();

    thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..20 {
                    assert!(store.get_event(id).unwrap().is_some());
                }
            });
        }
        scope.spawn(|| {
            store
                .set_delivery(
                    id,
                    "alpha",
                    DeliveryStatus::Success,
                    serde_json::json!({"w": 1}),
                )
                .unwrap();
        });
        scope.spawn(|| {
            store
                .set_delivery(
                    id,
                    "beta",
                    DeliveryStatus::Failed,
                    serde_json::json!({"w": 2}),
                )
                .unwrap();
        });
    });

    let alpha = store.get_delivery(id, "alpha").unwrap().unwrap();
    let beta = store.get_delivery(id, "beta").unwrap().unwrap();
    assert_eq!(alpha.witness, serde_json::json!({"w": 1}));
    assert_eq!(beta.witness, serde_json::json!({"w": 2}));
}

#[test]
fn workspace_member_and_no_crates_dir() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(!manifest_dir
        .components()
        .any(|component| component.as_os_str() == "crates"));
    let workspace = manifest_dir.parent().expect("SddIA").join("Cargo.toml");
    let text = fs::read_to_string(&workspace).unwrap();
    assert!(text.contains("\"sddia-event-store\""));
    assert!(!manifest_dir.join("../crates").exists());
}
