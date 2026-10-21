//! Flags de inspección del store. En `filesystem` no abren la base ni barren carpetas.

use std::path::Path;

use sddia_event_store::{
    runtime_db_path, store_mode, writes_store, DeadLetter, EventStore, Family, StoreError,
    StoredEvent,
};
use serde_json::{json, Value};

pub const EVENT_STORE_DISABLED: &str = "EVENT_STORE_DISABLED";

pub fn run(repo: &Path, flag: &str, inputs: &Value) -> (i32, Value) {
    if !writes_store(store_mode()) {
        return (
            1,
            json!({
                "success": false,
                "status_code": 1,
                "exitCode": 1,
                "data": null,
                "error": EVENT_STORE_DISABLED,
            }),
        );
    }
    match flag {
        "--query-events" => query_events(repo, inputs),
        "--inspect-dead-letter" => inspect_dead_letter(repo),
        "--replay-dead-letter" => replay_dead_letter(repo, inputs),
        other => (
            1,
            json!({
                "success": false,
                "status_code": 1,
                "exitCode": 1,
                "data": null,
                "error": format!("flag desconocida: {other}"),
            }),
        ),
    }
}

fn open_store(repo: &Path) -> Result<EventStore, StoreError> {
    let path = runtime_db_path(repo);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    EventStore::open(path)
}

fn fail(error: impl Into<String>) -> (i32, Value) {
    let error = error.into();
    (
        1,
        json!({
            "success": false,
            "status_code": 1,
            "exitCode": 1,
            "data": null,
            "error": error,
        }),
    )
}

fn ok(data: Value) -> (i32, Value) {
    (
        0,
        json!({
            "success": true,
            "status_code": 0,
            "exitCode": 0,
            "data": data,
            "error": null,
        }),
    )
}

fn query_events(repo: &Path, inputs: &Value) -> (i32, Value) {
    let family = inputs.get("family").and_then(|v| v.as_str()).map(str::trim);
    let event_type = inputs
        .get("event_type")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let limit = inputs.get("limit").and_then(|v| v.as_u64());
    if family.unwrap_or("").is_empty() && event_type.is_none() {
        return fail("family o event_type requerido");
    }
    let Some(limit) = limit else {
        return fail("limit requerido");
    };
    let families = match family {
        Some("telemetry") => vec![Family::Telemetry],
        Some("orchestration") => vec![Family::Orchestration],
        Some("domain") => vec![Family::Domain],
        Some("") | None => vec![Family::Telemetry, Family::Orchestration, Family::Domain],
        Some(other) => return fail(format!("family desconocida: {other}")),
    };
    let store = match open_store(repo) {
        Ok(store) => store,
        Err(err) => return fail(err.to_string()),
    };
    let mut rows = Vec::new();
    for family in families {
        let listed = match store.list_family(family) {
            Ok(rows) => rows,
            Err(err) => return fail(err.to_string()),
        };
        for event in listed {
            if let Some(want) = event_type {
                if event.event_type != want {
                    continue;
                }
            }
            rows.push(event_row(event));
        }
    }
    rows.sort_by(|a, b| {
        b.get("created_at")
            .and_then(|v| v.as_i64())
            .cmp(&a.get("created_at").and_then(|v| v.as_i64()))
    });
    rows.truncate(limit as usize);
    ok(json!({ "events": rows }))
}

fn event_row(event: StoredEvent) -> Value {
    json!({
        "event_id": event.event_id,
        "family": event.family.as_str(),
        "event_type": event.event_type,
        "created_at": event.created_at,
        "ecst": event.ecst,
    })
}

fn inspect_dead_letter(repo: &Path) -> (i32, Value) {
    let store = match open_store(repo) {
        Ok(store) => store,
        Err(err) => return fail(err.to_string()),
    };
    let letters = match store.list_dead_letters() {
        Ok(rows) => rows,
        Err(err) => return fail(err.to_string()),
    };
    let rows: Vec<Value> = letters.into_iter().map(letter_row).collect();
    ok(json!({ "dead_letters": rows }))
}

fn letter_row((event_id, letter): (String, DeadLetter)) -> Value {
    json!({
        "event_id": event_id,
        "cause": letter.cause,
        "failed_subscriber_id": letter.failed_subscriber_id,
    })
}

fn replay_dead_letter(repo: &Path, inputs: &Value) -> (i32, Value) {
    let Some(event_id) = inputs
        .get("event_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    else {
        return fail("event_id requerido");
    };
    let store = match open_store(repo) {
        Ok(store) => store,
        Err(err) => return fail(err.to_string()),
    };
    match store.replay(event_id) {
        Ok(()) => ok(json!({ "event_id": event_id, "replayed": true })),
        Err(err) => fail(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sddia_event_store::{
        override_store_mode, DeliveryStatus, Family, StoreMode, StoredEvent,
    };
    use std::fs;

    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            override_store_mode(None);
        }
    }

    fn repo() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "sddia-q04-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn sample(id: &str, family: Family, event_type: &str) -> StoredEvent {
        StoredEvent {
            event_id: id.into(),
            family,
            event_type: event_type.into(),
            created_at: 10,
            ecst: json!({"event_id": id, "event_type": event_type}),
        }
    }

    #[test]
    fn filesystem_flags_are_disabled_and_do_not_open_the_store() {
        let _guard = Guard;
        override_store_mode(Some(StoreMode::Filesystem));
        let root = repo();
        for flag in ["--query-events", "--inspect-dead-letter", "--replay-dead-letter"] {
            let (code, body) = run(&root, flag, &json!({"family":"domain","limit":1,"event_id":"x"}));
            assert_ne!(code, 0);
            assert_eq!(body["error"], EVENT_STORE_DISABLED);
        }
        assert!(!runtime_db_path(&root).exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn replay_clears_dead_letter_and_leaves_the_witness_pending() {
        let _guard = Guard;
        override_store_mode(Some(StoreMode::Redb));
        let root = repo();
        fs::create_dir_all(runtime_db_path(&root).parent().unwrap()).unwrap();
        let store = EventStore::open(runtime_db_path(&root)).unwrap();
        store
            .insert_event(&sample("evt-r", Family::Domain, "Domain_Probe"), &["agent:argos"])
            .unwrap();
        store
            .dead_letter("evt-r", "boom", "agent:argos")
            .unwrap();
        drop(store);
        let (code, body) = run(&root, "--replay-dead-letter", &json!({"event_id":"evt-r"}));
        assert_eq!(code, 0, "{body}");
        let store = EventStore::open(runtime_db_path(&root)).unwrap();
        assert!(store.get_dead_letter("evt-r").unwrap().is_none());
        let witness = store.get_delivery("evt-r", "agent:argos").unwrap().unwrap();
        assert_eq!(witness.status, DeliveryStatus::Pending);
        drop(store);
        let (code, listed) = run(&root, "--inspect-dead-letter", &json!({}));
        assert_eq!(code, 0);
        assert!(listed["data"]["dead_letters"].as_array().unwrap().is_empty());
        let (code, queried) = run(
            &root,
            "--query-events",
            &json!({"family":"domain","event_type":"Domain_Probe","limit":5}),
        );
        assert_eq!(code, 0);
        assert_eq!(queried["data"]["events"][0]["event_id"], "evt-r");
        let _ = fs::remove_dir_all(&root);
    }
}
