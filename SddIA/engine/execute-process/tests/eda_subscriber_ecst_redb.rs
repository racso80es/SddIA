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

struct ModeGuard {
    restore_agent_runtime: Option<String>,
}

impl ModeGuard {
    fn new() -> Self {
        let restore_agent_runtime = std::env::var("SDDIA_AGENT_RUNTIME_COMMAND").ok();
        std::env::remove_var("SDDIA_AGENT_RUNTIME_COMMAND");
        Self {
            restore_agent_runtime,
        }
    }
}

impl Drop for ModeGuard {
    fn drop(&mut self) {
        override_store_mode(None);
        std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        std::env::remove_var("SDDIA_LAB_SKIP_CAPABILITY_DI");
        std::env::remove_var("SDDIA_EXECUTE_PROCESS_BIN");
        std::env::remove_var("SDDIA_EVENT_STORE");
        std::env::remove_var("SDDIA_LAB_ROUTE_SYNC");
        std::env::remove_var("SDDIA_CLI_FOREGROUND");
        std::env::remove_var("SDDIA_TQM_STOP_AFTER");
        std::env::remove_var("SDDIA_LAB_ALLOW_DIRTY");
        if let Some(v) = self.restore_agent_runtime.take() {
            std::env::set_var("SDDIA_AGENT_RUNTIME_COMMAND", v);
        }
    }
}

fn set_route_domain_lab_env() {
    override_store_mode(Some(StoreMode::Redb));
    std::env::set_var("SDDIA_EVENT_STORE", "redb");
    std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
    std::env::set_var("SDDIA_CLI_FOREGROUND", "1");
    std::env::set_var("SDDIA_LAB_ALLOW_DIRTY", "1");
    std::env::set_var(
        "SDDIA_EXECUTE_PROCESS_BIN",
        lab_execute_process_bin().to_string_lossy().as_ref(),
    );
}

fn assert_subscriber_success(
    ds: &serde_json::Map<String, serde_json::Value>,
    sid: &str,
    out: &serde_json::Value,
    case_label: &str,
) {
    let status = ds.get(sid).and_then(|v| v.as_str()).unwrap_or("");
    assert_eq!(
        status, "success",
        "case={case_label} sid={sid} ds={ds:?} out={out}"
    );
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

fn link_lab_tree_if_missing(repo: &Path, rel: &str) {
    let root = find_repo_root().expect("repo root");
    let src = root.join(rel);
    if !src.is_dir() {
        return;
    }
    let dst = repo.join(rel);
    if dst.exists() {
        return;
    }
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).ok();
    }
    #[cfg(unix)]
    let _ = std::os::unix::fs::symlink(&src, &dst);
}

fn link_lab_file_if_missing(repo: &Path, rel: &str) {
    let root = find_repo_root().expect("repo root");
    let src = root.join(rel);
    if !src.is_file() {
        return;
    }
    let dst = repo.join(rel);
    if dst.exists() {
        return;
    }
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).ok();
    }
    #[cfg(unix)]
    let _ = std::os::unix::fs::symlink(&src, &dst);
}

fn temp_repo_with_bus_skeleton() -> PathBuf {
    let root = find_repo_root().expect("repo root");
    let scratch = root.join(".tmp");
    fs::create_dir_all(&scratch).unwrap();
    let repo = scratch.join(format!(
        "sddia-ecst-redb-{}-{}",
        std::process::id(),
        Uuid::new_v4()
    ));
    fs::create_dir_all(&repo).unwrap();
    for rel in [
        ".SddIA/project.md",
        ".SddIA/projects/core.md",
        ".SddIA/active-domain-profile.json",
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
    link_lab_tree_if_missing(&repo, "SddIA/events");
    link_lab_tree_if_missing(&repo, "SddIA/process");
    link_lab_tree_if_missing(
        &repo,
        "SddIA/library/codexes/codex-software-engineering/process",
    );
    link_lab_tree_if_missing(
        &repo,
        "SddIA/library/codexes/codex-kalma2-assistant/process",
    );
    link_lab_di_skeleton(&repo);
    link_lab_target_tree(&repo);
    seed_lab_execute_suite(&repo);
    fs::create_dir_all(repo.join(".events/pending")).unwrap();
    repo
}

fn seed_lab_execute_suite(repo: &Path) {
    let dir = repo.join("SddIA/suites");
    fs::create_dir_all(&dir).ok();
    fs::write(
        dir.join("smoke.md"),
        "---\nname: smoke\nversion: \"1.0.0\"\nexecution_strategy: run_all\natomic_nodes: []\n---\n\n# smoke (lab route effect)\n",
    )
    .ok();
}

#[cfg(unix)]
fn link_lab_target_tree(repo: &Path) {
    let root = find_repo_root().expect("repo root");
    let src = root.join("SddIA/target");
    let dst = repo.join("SddIA/target");
    if dst.exists() {
        return;
    }
    if src.is_dir() {
        let _ = std::os::unix::fs::symlink(&src, &dst);
    }
}

#[cfg(not(unix))]
fn link_lab_target_tree(_repo: &Path) {}

fn link_lab_di_skeleton(repo: &Path) {
    link_lab_file_if_missing(repo, "SddIA/library/norms/capability-taxonomy.md");
    link_lab_tree_if_missing(repo, "SddIA/library/norms/capability-contracts");
    link_lab_file_if_missing(repo, "SddIA/library/codexes/codex-software-engineering.md");
    link_lab_file_if_missing(repo, "SddIA/skills/git-manager.md");
    link_lab_file_if_missing(repo, "SddIA/tools/linear-tracker-adapter.md");
}

fn init_lab_git(repo: &Path) {
    seed_lab_ppr_docs(repo);
    let _ = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(repo)
        .status();
    let _ = std::process::Command::new("git")
        .args(["config", "user.email", "lab@sddia.local"])
        .current_dir(repo)
        .status();
    let _ = std::process::Command::new("git")
        .args(["config", "user.name", "lab"])
        .current_dir(repo)
        .status();
    let readme = repo.join("README.md");
    if !readme.is_file() {
        fs::write(&readme, "lab\n").ok();
    }
    let _ = std::process::Command::new("git")
        .args(["add", "-A"])
        .current_dir(repo)
        .status();
    let _ = std::process::Command::new("git")
        .args(["commit", "-q", "-m", "lab init"])
        .current_dir(repo)
        .status();
    let _ = std::process::Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(repo)
        .status();
    let bare = repo.join(".lab-origin.git");
    let _ = fs::remove_dir_all(&bare);
    let _ = std::process::Command::new("git")
        .args(["init", "--bare", "-q"])
        .arg(&bare)
        .status();
    let _ = std::process::Command::new("git")
        .args(["remote", "add", "origin"])
        .arg(&bare)
        .current_dir(repo)
        .status();
    let _ = std::process::Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(repo)
        .status();
    let _ = std::process::Command::new("git")
        .args(["checkout", "-q", "-b", "feat/lab"])
        .current_dir(repo)
        .status();
    let _ = std::process::Command::new("git")
        .args(["push", "-q", "-u", "origin", "feat/lab"])
        .current_dir(repo)
        .status();
}

fn seed_lab_tqm_pbi(repo: &Path) {
    let p = repo.join("docs/todos/pending/lab-tqm-route-effect.md");
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(
        &p,
        "---\ndocument_id: LAB-TQM-ROUTE\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\n---\n\n# lab tqm\n",
    )
    .unwrap();
}

fn tracker_issue_changed_lab_payload() -> serde_json::Value {
    json!({
        "issue_ref": "LAB-PBI-OK",
        "project_slug": "core",
        "occurred_at": "2026-10-29T12:00:00Z",
        "changes": [],
        "correlation_id": "corr-lab-tracker-md",
        "source": "linear-bridge-watcher",
    })
}

fn telemetry_captured_lab_payload() -> serde_json::Value {
    json!({
        "entity_id": "ent-route-redb",
        "entity_type": "process",
        "execution_metrics": { "success_status": true, "duration_ms": 1, "exit_code": 0 },
        "origin_stimulus": "lab-route-effect",
    })
}

fn seed_lab_ppr_docs(repo: &Path) {
    let root = find_repo_root().expect("repo root");
    let src = root.join("docs/features/route-domain-process-effect");
    for dst_rel in [
        "docs/features/route-domain-process-effect",
        "docs/features/lab",
    ] {
        let dst = repo.join(dst_rel);
        if src.is_dir() {
            copy_tree(&src, &dst);
        } else {
            fs::create_dir_all(&dst).ok();
            for name in [
                "objectives.md",
                "spec.md",
                "plan.md",
                "implementation.md",
                "validacion.md",
            ] {
                fs::write(
                    dst.join(name),
                    format!("---\nglobal: APTO\n---\n\n# {name}\n"),
                )
                .ok();
            }
        }
        for name in ["objectives.md", "spec.md", "plan.md", "implementation.md"] {
            let f = dst.join(name);
            if !f.is_file() {
                fs::write(
                    &f,
                    format!("---\nglobal: APTO\n---\n\n# {name}\n"),
                )
                .ok();
            }
        }
    }
}

fn kalma2_tqm_payload() -> serde_json::Value {
    json!({
        "process": "feature",
        "project_slug": "core",
        "raw_text": "docs/todos/pending/lab-tqm-route-effect.md",
        "process_inputs": { "stop_after": "design" },
    })
}

fn aiua_tqm_payload() -> serde_json::Value {
    json!({
        "process": "feature",
        "raw_text": "docs/todos/pending/lab-tqm-route-effect.md",
        "process_inputs": { "stop_after": "design" },
    })
}

fn domain_entity_created_payload() -> serde_json::Value {
    json!({
        "entity_class": "tool",
        "entity_type": "tool",
        "entity_id": "00000000-0000-4000-8000-000000000099",
        "lifecycle_operation": "create",
        "entity_uuid": "00000000-0000-4000-8000-000000000099",
        "entity_name": "route-effect-lab",
        "version": "1.0.0",
        "hash_signature_new": "sha256:abc",
        "changes_summary": "lab create",
        "origin_topology": "core",
    })
}

fn run_route_domain_effect(repo: &Path, event_type: &str, payload: serde_json::Value) -> serde_json::Value {
    let rel = seed_store_only_pending_rel(repo, event_type, payload);
    route_domain_test_progress_mark("enter", event_type, &rel);
    let out = route_domain_event(repo, &rel, false);
    route_domain_test_progress_mark("leave", event_type, &rel);
    out
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
    let _guard = ModeGuard::new();
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
    let _guard = ModeGuard::new();
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
    let _guard = ModeGuard::new();
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
    let _guard = ModeGuard::new();
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
    let _guard = ModeGuard::new();
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
    let _guard = ModeGuard::new();
    set_route_domain_lab_env();
    let repo = temp_repo_with_bus_skeleton();
    let out = run_route_domain_effect(
        &repo,
        "Tracker_Issue_Changed",
        tracker_issue_changed_lab_payload(),
    );
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    assert_delivery_not_ecst_gate_only(&ds, &out);
    assert_subscriber_success(&ds, "tekton.tracker-markdown-apply", &out, "tracker-markdown");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn route_domain_telegram_responder_receives_ecst_path() {
    let _guard = ModeGuard::new();
    set_route_domain_lab_env();
    let repo = temp_repo_with_bus_skeleton();
    let out = run_route_domain_effect(
        &repo,
        "TelegramMessage_Received",
        telegram_route_test_payload("triaje lab"),
    );
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    assert_delivery_not_ecst_gate_only(&ds, &out);
    assert_subscriber_success(
        &ds,
        "mayeuta.telegram-fallback-responder",
        &out,
        "telegram-text",
    );
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn route_domain_kalma2_task_queue_not_blocked_before_handler() {
    let _guard = ModeGuard::new();
    set_route_domain_lab_env();
    std::env::set_var("SDDIA_TQM_STOP_AFTER", "design");
    let repo = temp_repo_with_bus_skeleton();
    seed_lab_tqm_pbi(&repo);
    init_lab_git(&repo);
    let out = run_route_domain_effect(&repo, "Kalma2_Process_Requested", kalma2_tqm_payload());
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    assert_delivery_not_ecst_gate_only(&ds, &out);
    assert_subscriber_success(&ds, "tekton.task-queue-manager", &out, "kalma2-tqm");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn route_domain_aiua_task_queue_not_blocked_before_handler() {
    let _guard = ModeGuard::new();
    set_route_domain_lab_env();
    std::env::set_var("SDDIA_TQM_STOP_AFTER", "design");
    let repo = temp_repo_with_bus_skeleton();
    seed_lab_tqm_pbi(&repo);
    init_lab_git(&repo);
    let out = run_route_domain_effect(&repo, "Aiua_Process_Requested", aiua_tqm_payload());
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    assert_delivery_not_ecst_gate_only(&ds, &out);
    assert_subscriber_success(&ds, "tekton.task-queue-manager", &out, "aiua-tqm");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn mode_guard_isolates_host_agent_runtime_command() {
    std::env::set_var("SDDIA_AGENT_RUNTIME_COMMAND", "/tmp/nonexistent-agent-runtime");
    let _guard = ModeGuard::new();
    assert!(
        std::env::var("SDDIA_AGENT_RUNTIME_COMMAND").is_err(),
        "route lab tests must not inherit host SDDIA_AGENT_RUNTIME_COMMAND"
    );
}

#[test]
fn route_domain_telegram_empty_text_still_skipped() {
    let _guard = ModeGuard::new();
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
    let _guard = ModeGuard::new();
    set_route_domain_lab_env();
    let repo = temp_repo_with_bus_skeleton();
    let out = run_route_domain_effect(
        &repo,
        "Domain_Entity_Telemetry_Captured",
        telemetry_captured_lab_payload(),
    );
    let ds = out
        .pointer("/data/delivery_status")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    assert_delivery_not_ecst_gate_only(&ds, &out);
    assert_subscriber_success(
        &ds,
        "cumulo.memory-evolution-ingest",
        &out,
        "memory-evolution",
    );
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn route_domain_process_effect_catalog_success() {
    let _guard = ModeGuard::new();
    set_route_domain_lab_env();
    std::env::set_var("SDDIA_TQM_STOP_AFTER", "design");
    let repo = temp_repo_with_bus_skeleton();
    seed_lab_tqm_pbi(&repo);
    init_lab_git(&repo);

    struct Case {
        label: &'static str,
        event_type: &'static str,
        payload: serde_json::Value,
        sids: &'static [&'static str],
    }

    let cases = [
        Case {
            label: "local-qa-ppr",
            event_type: "Local_QA_Requested",
            payload: json!({ "branch": "feat/lab", "qa_profile": "docs-only" }),
            sids: &["argos.pull-request-review"],
        },
        Case {
            label: "work-initiated-stamp",
            event_type: "Work_Initiated",
            payload: json!({
                "branch": "feat/lab",
                "persist_ref": "docs/features/route-domain-process-effect",
                "source_process": "feature",
                "occurred_at": "2026-11-01T12:00:00Z",
                "tracker_ref": "LAB-PBI-OK",
                "project_slug": "core",
            }),
            sids: &["tekton.tracker-stamp"],
        },
        Case {
            label: "entity-created-compile",
            event_type: "Domain_Entity_Created",
            payload: domain_entity_created_payload(),
            sids: &["cumulo.compile-ecosystem-map-snapshot"],
        },
        Case {
            label: "entity-degraded-cerbero-fix",
            event_type: "Domain_Entity_Degraded",
            payload: json!({
                "entity_type": "tool",
                "entity_id": "lab-tool-1",
                "reason": "lab degraded",
                "success_rate": 0.42,
                "recovery_attempt": 1,
            }),
            sids: &[
                "cerbero.cerbero-governance-react",
                "dedalo.fix-tool-process",
            ],
        },
        Case {
            label: "suite-execute",
            event_type: "Suite_Execution_Requested",
            payload: json!({ "suite_id": "smoke" }),
            sids: &["tekton.execute-suite"],
        },
        Case {
            label: "email-triage",
            event_type: "Email_Received",
            payload: json!({
                "message_uid": "uid-lab-1",
                "mailbox": "INBOX",
                "from": "lab@example.com",
                "subject": "lab",
                "received_at": "2026-11-01T12:00:00Z",
                "snippet": "lab snippet",
            }),
            sids: &["cumulo.email-triage-gateway"],
        },
        Case {
            label: "email-quick-action",
            event_type: "Email_Quick_Action_Requested",
            payload: json!({
                "message_uid": "uid-lab-2",
                "action": "archive",
            }),
            sids: &["cumulo.email-quick-action-ingest"],
        },
        Case {
            label: "user-preference",
            event_type: "User_Preference_Change_Requested",
            payload: json!({
                "operation": "propose",
                "channel": "telegram",
                "subject_hint": "lab-route-effect",
                "predicate": "priority",
            }),
            sids: &["cumulo.user-preference-ingest"],
        },
        Case {
            label: "telegram-callback-digest",
            event_type: "TelegramCallback_Received",
            payload: json!({
                "callback_data": "digest:ok",
                "chat_id": "lab-chat-1",
                "source": "telegram-gateway",
            }),
            sids: &["cumulo.email-digest-preference-reply"],
        },
        Case {
            label: "tracker-sync-replay",
            event_type: "Tracker_Sync_Failed",
            payload: json!({
                "issue_ref": "LAB-PBI-OK",
                "operation": "update_issue_state",
                "error_code": "LINEAR_RATE_LIMIT",
                "occurred_at": "2026-11-01T12:00:00Z",
                "source_process": "tracker-stamp",
                "target_state": "in_progress",
                "project_slug": "core",
            }),
            sids: &["tekton.tracker-sync-replay"],
        },
        Case {
            label: "aiua-tqm",
            event_type: "Aiua_Process_Requested",
            payload: aiua_tqm_payload(),
            sids: &["tekton.task-queue-manager"],
        },
    ];

    for case in cases {
        let out = run_route_domain_effect(&repo, case.event_type, case.payload);
        let err = out.get("error").and_then(|v| v.as_str()).unwrap_or("");
        assert!(
            !err.contains("event_type not cataloged"),
            "case={} catalog: {err} out={out}",
            case.label
        );
        let ds = out
            .pointer("/data/delivery_status")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();
        assert_delivery_not_ecst_gate_only(&ds, &out);
        for sid in case.sids {
            assert_subscriber_success(&ds, sid, &out, case.label);
        }
    }
    let _ = fs::remove_dir_all(&repo);
}
