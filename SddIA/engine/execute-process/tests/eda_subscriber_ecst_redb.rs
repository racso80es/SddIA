//! AUT-46 — suscriptores leen ECST desde redb sin fichero pending; concurrencia sin Locked.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier};
use std::thread;

use execute_process::core::repo::find_repo_root;
use execute_process::engine::ecst_input_loader::load_ecst_from_rel;
use execute_process::engine::handlers::tracker_markdown_apply;
use execute_process::engine::handlers::tracker_stamp;
use execute_process::engine::memory_evolution_ingest_core::ingest_domain_event_file;
use execute_process::engine::route_domain_core::{materialize_pending_domain_event, route_domain_event};
use sddia_event_store::{override_store_mode, record_ecst, runtime_db_path, EventStore, StoreMode};
use serde_json::json;
use uuid::Uuid;

struct ModeGuard;

impl Drop for ModeGuard {
    fn drop(&mut self) {
        override_store_mode(None);
        std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        std::env::remove_var("SDDIA_LAB_SKIP_CAPABILITY_DI");
        std::env::remove_var("SDDIA_EXECUTE_PROCESS_BIN");
    }
}

fn lab_execute_process_bin() -> PathBuf {
    let root = find_repo_root().expect("repo root");
    let bin = root.join("SddIA/target/debug/execute-process");
    assert!(
        bin.is_file(),
        "compile execute-process: cd SddIA && cargo build -p execute-process"
    );
    bin
}

fn copy_tree(src: &Path, dst: &Path) {
    if !src.is_dir() {
        return;
    }
    fs::create_dir_all(dst).expect("create_dir_all");
    for entry in fs::read_dir(src).expect("read_dir") {
        let entry = entry.expect("dir entry");
        let target = dst.join(entry.file_name());
        if entry.file_type().expect("file_type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).ok();
            }
            fs::copy(entry.path(), &target).unwrap_or_else(|e| {
                panic!("copy {} -> {}: {e}", entry.path().display(), target.display())
            });
        }
    }
}

fn temp_repo_with_bus_skeleton() -> PathBuf {
    let root = find_repo_root().expect("repo root");
    let repo = std::env::temp_dir().join(format!(
        "sddia-ecst-redb-{}-{}",
        std::process::id(),
        Uuid::new_v4()
    ));
    fs::create_dir_all(&repo).unwrap();
    for rel in [
        ".SddIA/project.md",
        "SddIA/core/cumulo.paths.json",
        "SddIA/core/event-domain-subscriptions.json",
        "SddIA/core/capability-bindings.md",
    ] {
        let src = root.join(rel);
        let dst = repo.join(rel);
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::copy(&src, &dst).unwrap_or_else(|e| panic!("copy {rel}: {e}"));
    }
    let events_src = root.join("SddIA/events");
    if events_src.is_dir() {
        copy_tree(&events_src, &repo.join("SddIA/events"));
    }
    fs::create_dir_all(repo.join(".events/pending")).unwrap();
    repo
}

fn telegram_route_test_payload(text: &str) -> serde_json::Value {
    json!({
        "text": text,
        "chat_id": "lab-chat-99",
        "source": "telegram-gateway",
    })
}

fn assert_delivery_not_ecst_gate_only(
    ds: &serde_json::Map<String, serde_json::Value>,
    out: &serde_json::Value,
) {
    let ecst_only = ds.len() == 1
        && ds
            .get("ecst-gate")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s == "failed");
    assert!(
        !ecst_only,
        "router stopped at ECST gate only: {ds:?} out={out}"
    );
}

fn seed_store_only_pending_rel(
    repo: &Path,
    event_type: &str,
    payload: serde_json::Value,
) -> String {
    materialize_pending_domain_event(repo, event_type, "tekton", payload).expect("materialize");
    let store = EventStore::open(runtime_db_path(repo)).expect("open store");
    let rows = store
        .list_family(sddia_event_store::Family::Domain)
        .expect("list");
    let event_id = rows.last().expect("event row").event_id.clone();
    drop(store);
    let pending = repo.join(".events/pending").join(format!("{event_id}.json"));
    if pending.is_file() {
        fs::remove_file(&pending).expect("remove pending mirror");
    }
    format!(".events/pending/{event_id}.json")
}

/// Marcas de avance AUT-59: stderr (flush) + append en checkout `.tmp/route-domain-test-progress.log`.
fn route_domain_test_progress_mark(phase: &str, event_type: &str, rel: &str) {
    let line = format!("route_domain_test phase={phase} event_type={event_type} rel={rel}\n");
    let _ = io::stderr().write_all(line.as_bytes());
    let _ = io::stderr().flush();
    let checkout = find_repo_root().expect("repo root");
    let log_path = checkout.join(".tmp/route-domain-test-progress.log");
    if let Some(parent) = log_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut f) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = f.write_all(line.as_bytes());
        let _ = f.flush();
    }
}

#[test]
fn tracker_stamp_loads_ecst_from_store_without_pending_file() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
    std::env::remove_var("SDDIA_LAB_SKIP_CAPABILITY_DI");
    let repo = find_repo_root().expect("repo");
    let rel = seed_store_only_pending_rel(
        &repo,
        "Work_Initiated",
        json!({
            "tracker_ref": "LAB-PBI-OK",
            "branch": "feat/x",
            "persist_ref": "docs/features/x",
        }),
    );
    assert!(!repo.join(&rel).is_file());
    let env = tracker_stamp::run(&repo, &json!({ "event_file_path": rel })).expect("run");
    assert_eq!(
        env.data
            .as_ref()
            .and_then(|d| d.get("delivery_status"))
            .and_then(|v| v.as_str()),
        Some("success")
    );
}

#[test]
fn tracker_markdown_apply_loads_ecst_from_store_without_pending_file() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    let repo = temp_repo_with_bus_skeleton();
    let rel = seed_store_only_pending_rel(
        &repo,
        "Tracker_Issue_Changed",
        json!({
            "issue_ref": "LAB-PBI-OK",
            "project_slug": "demo",
            "occurred_at": "2026-10-29T12:00:00Z",
            "changes": [],
        }),
    );
    assert!(!repo.join(&rel).is_file());
    load_ecst_from_rel(&repo, &rel).expect("load ecst");
    let err = tracker_markdown_apply::run(&repo, &json!({ "event_file_path": rel }))
        .err()
        .unwrap_or_default();
    assert!(
        !err.contains("event file not found") && !err.contains("no existe"),
        "fallo debe ser downstream, no FS: {err}"
    );
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn memory_evolution_ingest_loads_ecst_from_store_without_pending_file() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    let repo = temp_repo_with_bus_skeleton();
    let rel = seed_store_only_pending_rel(
        &repo,
        "Domain_Entity_Telemetry_Captured",
        json!({
            "entity_id": "ent-ecst-redb",
            "entity_type": "process",
            "execution_metrics": { "success_status": true, "duration_ms": 1, "exit_code": 0 },
        }),
    );
    assert!(!repo.join(&rel).is_file());
    let result = ingest_domain_event_file(&repo, &rel);
    let err = result.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !err.contains("no existe"),
        "no debe fallar por fichero ausente: {result}"
    );
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn four_concurrent_route_domain_on_same_redb_without_locked() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
    let repo = Arc::new(temp_repo_with_bus_skeleton());
    let mut rels = Vec::new();
    for i in 0..4 {
        let rel = seed_store_only_pending_rel(
            &repo,
            "PullRequest_Audited",
            json!({
                "tracker_ref": "LAB-PBI-OK",
                "resolution": format!("OK-{i}"),
            }),
        );
        rels.push(rel);
    }
    let barrier = Arc::new(Barrier::new(5));
    let mut handles = Vec::new();
    for rel in rels {
        let repo = Arc::clone(&repo);
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            b.wait();
            route_domain_event(&repo, &rel, false)
        }));
    }
    barrier.wait();
    for h in handles {
        let out = h.join().expect("join");
        let err = out.get("error").and_then(|v| v.as_str()).unwrap_or("");
        assert!(
            !err.contains("Locked") && !err.to_lowercase().contains("already open"),
            "route no debe agotar lock: {out}"
        );
    }
    let _ = fs::remove_dir_all(repo.as_ref());
}

#[test]
fn record_ecst_concurrent_writes_same_db_without_locked() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    let repo = Arc::new(temp_repo_with_bus_skeleton());
    let barrier = Arc::new(Barrier::new(5));
    let mut handles = Vec::new();
    for i in 0..4 {
        let repo = Arc::clone(&repo);
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            b.wait();
            let event = json!({
                "event_id": format!("evt-conc-{i}-{}", Uuid::new_v4()),
                "event_type": "Domain_Entity_Created",
                "timestamp": "2026-10-29T12:00:00Z",
                "emitter_agent": "test",
                "payload": { "n": i },
                "delivery_state": {},
            });
            record_ecst(&repo, "domain", &event)
        }));
    }
    barrier.wait();
    for h in handles {
        let r = h.join().expect("join");
        if let Err(e) = r {
            let msg = e.to_string();
            assert!(
                !msg.contains("Locked"),
                "record_ecst concurrente: {msg}"
            );
        }
    }
    let _ = fs::remove_dir_all(repo.as_ref());
}

#[test]
fn route_domain_tracker_markdown_apply_not_blocked_by_missing_branch() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
    let repo = temp_repo_with_bus_skeleton();
    let rel = seed_store_only_pending_rel(
        &repo,
        "Tracker_Issue_Changed",
        json!({
            "issue_ref": "LAB-PBI-OK",
            "project_slug": "core",
            "occurred_at": "2026-10-29T12:00:00Z",
            "changes": [],
        }),
    );
    route_domain_test_progress_mark("enter", "Tracker_Issue_Changed", &rel);
    let out = route_domain_event(&repo, &rel, false);
    route_domain_test_progress_mark("leave", "Tracker_Issue_Changed", &rel);
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    for (sid, status) in &ds {
        let s = status.as_str().unwrap_or("");
        assert!(
            !s.contains("branch missing in payload"),
            "sid={sid} status={s} out={out}"
        );
    }
    std::env::remove_var("SDDIA_LAB_ROUTE_SYNC");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn route_domain_telegram_responder_receives_ecst_path() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
    std::env::set_var(
        "SDDIA_EXECUTE_PROCESS_BIN",
        lab_execute_process_bin().to_string_lossy().as_ref(),
    );
    let repo = temp_repo_with_bus_skeleton();
    let rel = seed_store_only_pending_rel(
        &repo,
        "TelegramMessage_Received",
        telegram_route_test_payload("triaje lab"),
    );
    route_domain_test_progress_mark("enter", "TelegramMessage_Received", &rel);
    let out = route_domain_event(&repo, &rel, false);
    route_domain_test_progress_mark("leave", "TelegramMessage_Received", &rel);
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    assert_delivery_not_ecst_gate_only(&ds, &out);
    for (sid, status) in &ds {
        let s = status.as_str().unwrap_or("");
        assert!(
            s != "skipped-empty-text",
            "sid={sid} status={s} out={out}"
        );
        assert!(
            !s.contains("process sin despacho ECST"),
            "sid={sid} status={s} out={out}"
        );
    }
    std::env::remove_var("SDDIA_LAB_ROUTE_SYNC");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn route_domain_kalma2_task_queue_not_blocked_before_handler() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
    let repo = temp_repo_with_bus_skeleton();
    let rel = seed_store_only_pending_rel(
        &repo,
        "Kalma2_Process_Requested",
        json!({
            "process": "bug-fix",
            "project_slug": "core",
            "raw_text": "lab task",
        }),
    );
    route_domain_test_progress_mark("enter", "Kalma2_Process_Requested", &rel);
    let out = route_domain_event(&repo, &rel, false);
    route_domain_test_progress_mark("leave", "Kalma2_Process_Requested", &rel);
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    for (sid, status) in &ds {
        let s = status.as_str().unwrap_or("");
        assert!(
            !s.contains("proceso no permitido"),
            "sid={sid} status={s} out={out}"
        );
        assert!(
            !s.contains("process sin despacho ECST"),
            "sid={sid} status={s} out={out}"
        );
    }
    std::env::remove_var("SDDIA_LAB_ROUTE_SYNC");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn route_domain_telegram_empty_text_still_skipped() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
    let repo = temp_repo_with_bus_skeleton();
    let rel = seed_store_only_pending_rel(
        &repo,
        "TelegramMessage_Received",
        telegram_route_test_payload("   "),
    );
    route_domain_test_progress_mark("enter", "TelegramMessage_Received", &rel);
    let out = route_domain_event(&repo, &rel, false);
    route_domain_test_progress_mark("leave", "TelegramMessage_Received", &rel);
    let err = out.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !err.contains("event_type not cataloged"),
        "ECST catalog missing in temp repo: {err} out={out}"
    );
    assert!(
        !err.contains("missing required payload"),
        "payload should satisfy class: {err} out={out}"
    );
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    assert_delivery_not_ecst_gate_only(&ds, &out);
    let mut saw_skip = false;
    for (sid, status) in &ds {
        let s = status.as_str().unwrap_or("");
        if s == "skipped-empty-text" {
            saw_skip = true;
        }
        assert!(
            !s.contains("process sin despacho ECST"),
            "sid={sid} status={s} out={out}"
        );
    }
    assert!(saw_skip, "expected skipped-empty-text in {ds:?}");
    std::env::remove_var("SDDIA_LAB_ROUTE_SYNC");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn route_domain_memory_evolution_not_blocked_by_missing_branch() {
    let _guard = ModeGuard;
    override_store_mode(Some(StoreMode::Redb));
    std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
    let repo = temp_repo_with_bus_skeleton();
    let rel = seed_store_only_pending_rel(
        &repo,
        "Domain_Entity_Telemetry_Captured",
        json!({
            "entity_id": "ent-route-redb",
            "entity_type": "process",
            "execution_metrics": { "success_status": true, "duration_ms": 1, "exit_code": 0 },
        }),
    );
    route_domain_test_progress_mark("enter", "Domain_Entity_Telemetry_Captured", &rel);
    let out = route_domain_event(&repo, &rel, false);
    route_domain_test_progress_mark("leave", "Domain_Entity_Telemetry_Captured", &rel);
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    for (sid, status) in &ds {
        let s = status.as_str().unwrap_or("");
        assert!(
            !s.contains("branch missing in payload"),
            "sid={sid} status={s} out={out}"
        );
    }
    std::env::remove_var("SDDIA_LAB_ROUTE_SYNC");
    let _ = fs::remove_dir_all(&repo);
}
