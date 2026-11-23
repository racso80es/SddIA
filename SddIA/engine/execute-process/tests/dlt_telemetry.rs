//! Fan-out fractal `Domain_Entity_Telemetry_Captured` + skip DLT sin bóveda.

use execute_process::engine::dlt_telemetry_anchor::IOTA_ENV_LOCK;
use execute_process::engine::route_fractal_core::route_domain_fractal_event;
use serde_json::{json, Value};
use std::fs;
use std::sync::{Mutex, MutexGuard, OnceLock};
use tempfile::tempdir;

const TELEM_REL: &str = ".events/domain/aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee.json";

static DLT_LAB: OnceLock<Mutex<()>> = OnceLock::new();

fn dlt_lab_lock() -> MutexGuard<'static, ()> {
    DLT_LAB
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

struct DltLabGuard {
    _lock: MutexGuard<'static, ()>,
}

impl DltLabGuard {
    fn acquire() -> Self {
        let _lock = dlt_lab_lock();
        execute_process::engine::eda_port::clear_port();
        Self { _lock }
    }
}

impl Drop for DltLabGuard {
    fn drop(&mut self) {
        execute_process::engine::eda_port::clear_port();
    }
}

fn write_min_repo(repo: &std::path::Path) {
    let real_target = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target")
        .canonicalize()
        .unwrap();
    fs::create_dir_all(repo.join("SddIA/core")).unwrap();
    fs::write(
        repo.join("SddIA/core/cumulo.paths.json"),
        serde_json::to_string_pretty(&json!({
          "compiled_capsules": {
            "native_root": real_target.display().to_string(),
            "wasm_root": real_target.join("wasm32-wasip1").display().to_string(),
            "profiles": ["release", "debug"]
          },
          "eda_bus": {
            "store": ".SddIA/runtime/events.redb"
          },
          "eda_fractal": {
            "domain": "./.events/domain",
            "dead_letter": "./.events/dead-letter",
            "domain_subscriptions": "SddIA/core/event-domain-subscriptions.json"
          },
          "eda_instance": {"proofs": ".SddIA/proofs"},
          "paths": {"vectorStore": ".SddIA/vector_store"}
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        repo.join("SddIA/core/event-domain-subscriptions.json"),
        serde_json::to_string_pretty(&json!({
            "Domain_Entity_Telemetry_Captured": [
                {
                    "agent": "cumulo",
                    "process": "memory-evolution-ingest",
                    "intent": "ingest"
                },
                {
                    "agent": "cumulo",
                    "tool": "iota-immutable-publisher",
                    "intent": "dlt"
                }
            ]
        }))
        .unwrap(),
    )
    .unwrap();
    fs::create_dir_all(repo.join(".events/domain")).unwrap();
    fs::create_dir_all(repo.join(".SddIA/vector_store")).unwrap();
}

fn telemetry_ecst() -> Value {
    json!({
        "event_id": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "event_type": "Domain_Entity_Telemetry_Captured",
        "payload": {
            "entity_type": "process",
            "entity_id": "feature",
            "execution_metrics": {
                "duration_ms": 42,
                "exit_code": 0,
                "success_status": true
            },
            "origin_stimulus": {
                "event_type": "Raw_Execution_Finished",
                "event_id": "11111111-2222-3333-4444-555555555555"
            }
        },
        "delivery_state": {}
    })
}

fn seed_telemetry_in_port(repo: &std::path::Path) {
    let mut ecst = telemetry_ecst();
    let _ = sddia_eda_catalog::stamp_published_event(repo, &mut ecst);
    fs::create_dir_all(repo.join(".SddIA/runtime")).unwrap();
    execute_process::engine::eda_port::install_port(
        sddia_event_manager_host::open_lab_redb_port(repo).expect("redb adapter"),
    );
    execute_process::engine::eda_port::record_ecst(repo, "domain", &ecst).expect("seed port");
}

fn restore(key: &str, prev: Option<String>) {
    match prev {
        Some(v) => std::env::set_var(key, v),
        None => std::env::remove_var(key),
    }
}

#[test]
fn fractal_fanout_skip_config_does_not_dead_letter() {
    let _lab = DltLabGuard::acquire();
    let _g = IOTA_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let prev_sim = std::env::var("SDDIA_LAB_SIMULATE_IOTA").ok();
    let prev_out = std::env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
    let prev_mock = std::env::var("SDDIA_LAB_MOCK_IOTA_URL").ok();
    let prev_sec = std::env::var("IOTA_WALLET_SECRET").ok();
    let prev_rel = std::env::var("IOTA_PUBLISH_RELAY_URL").ok();
    for key in [
        "SDDIA_LAB_SIMULATE_IOTA",
        "SDDIA_LAB_MOCK_OUTBOUND",
        "SDDIA_LAB_MOCK_IOTA_URL",
        "IOTA_WALLET_SECRET",
        "IOTA_PUBLISH_RELAY_URL",
    ] {
        std::env::remove_var(key);
    }
    std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "0");
    std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "0");
    let dir = tempdir().unwrap();
    let repo = dir.path();
    write_min_repo(repo);
    seed_telemetry_in_port(repo);

    let out = route_domain_fractal_event(repo, TELEM_REL);
    restore("SDDIA_LAB_SIMULATE_IOTA", prev_sim);
    restore("SDDIA_LAB_MOCK_OUTBOUND", prev_out);
    restore("SDDIA_LAB_MOCK_IOTA_URL", prev_mock);
    restore("IOTA_WALLET_SECRET", prev_sec);
    restore("IOTA_PUBLISH_RELAY_URL", prev_rel);

    assert_eq!(out["success"].as_bool(), Some(true), "{out}");
    let ds = &out["data"]["delivery_status"];
    assert_eq!(ds["cumulo.memory-evolution-ingest"], "success");
    assert_eq!(ds["cumulo.iota-immutable-publisher"], "success");
    assert_eq!(out["data"]["dead_lettered"].as_bool(), Some(false));
    assert!(!repo.join(".events/dead-letter").exists() || fs::read_dir(repo.join(".events/dead-letter")).map(|i| i.count()).unwrap_or(0) == 0);
}

#[test]
fn fractal_fanout_simulate_persists_proof_then_purges() {
    let _lab = DltLabGuard::acquire();
    let _g = IOTA_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let prev_sim = std::env::var("SDDIA_LAB_SIMULATE_IOTA").ok();
    std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "1");

    let dir = tempdir().unwrap();
    let repo = dir.path();
    write_min_repo(repo);
    seed_telemetry_in_port(repo);

    let out = route_domain_fractal_event(repo, TELEM_REL);
    restore("SDDIA_LAB_SIMULATE_IOTA", prev_sim);

    assert_eq!(out["success"].as_bool(), Some(true), "{out}");
    let ds = &out["data"]["delivery_status"];
    assert_eq!(ds["cumulo.memory-evolution-ingest"], "success");
    assert_eq!(ds["cumulo.iota-immutable-publisher"], "success");
    let proof_path = repo.join(".SddIA/proofs/dlt-telemetry/aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee.json");
    assert!(proof_path.is_file(), "proof missing after purge");
    let proof: Value = serde_json::from_str(&fs::read_to_string(&proof_path).unwrap()).unwrap();
    let digest = proof["transaction_digest"].as_str().unwrap_or("");
    assert!(
        digest.starts_with("local-") || digest.starts_with("lab-sim-"),
        "{digest}"
    );
    assert_ne!(digest, "batched-digest");
}
