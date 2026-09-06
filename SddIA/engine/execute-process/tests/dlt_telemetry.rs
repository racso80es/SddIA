//! Fan-out fractal `Domain_Entity_Telemetry_Captured` + skip DLT sin bóveda.

use execute_process::engine::dlt_telemetry_anchor::IOTA_ENV_LOCK;
use execute_process::engine::route_fractal_core::route_domain_fractal_event;
use serde_json::{json, Value};
use std::fs;
use tempfile::tempdir;

fn write_min_repo(repo: &std::path::Path) {
    fs::create_dir_all(repo.join("SddIA/core")).unwrap();
    fs::write(
        repo.join("SddIA/core/cumulo.paths.json"),
        r#"{
          "eda_fractal": {
            "domain": "./.events/domain",
            "dead_letter": "./.events/dead-letter",
            "domain_subscriptions": "SddIA/core/event-domain-subscriptions.json"
          },
          "eda_instance": {"proofs": ".SddIA/proofs"}
        }"#,
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
    fs::write(
        repo.join(".events/domain/telem.json"),
        r#"{
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
}"#,
    )
    .unwrap();
}

fn restore(key: &str, prev: Option<String>) {
    match prev {
        Some(v) => std::env::set_var(key, v),
        None => std::env::remove_var(key),
    }
}

#[test]
fn fractal_fanout_skip_config_does_not_dead_letter() {
    let _g = IOTA_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let prev_sim = std::env::var("SDDIA_LAB_SIMULATE_IOTA").ok();
    let prev_out = std::env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
    let prev_mock = std::env::var("SDDIA_LAB_MOCK_IOTA_URL").ok();
    let prev_sec = std::env::var("IOTA_WALLET_SECRET").ok();
    let prev_rel = std::env::var("IOTA_PUBLISH_RELAY_URL").ok();
    std::env::remove_var("SDDIA_LAB_SIMULATE_IOTA");
    std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
    std::env::remove_var("SDDIA_LAB_MOCK_IOTA_URL");
    std::env::remove_var("IOTA_WALLET_SECRET");
    std::env::remove_var("IOTA_PUBLISH_RELAY_URL");

    let dir = tempdir().unwrap();
    let repo = dir.path();
    write_min_repo(repo);

    let out = route_domain_fractal_event(repo, ".events/domain/telem.json");
    restore("SDDIA_LAB_SIMULATE_IOTA", prev_sim);
    restore("SDDIA_LAB_MOCK_OUTBOUND", prev_out);
    restore("SDDIA_LAB_MOCK_IOTA_URL", prev_mock);
    restore("IOTA_WALLET_SECRET", prev_sec);
    restore("IOTA_PUBLISH_RELAY_URL", prev_rel);

    assert_eq!(out["success"].as_bool(), Some(true), "{out}");
    let ds = &out["data"]["delivery_status"];
    assert_eq!(ds["cumulo.memory-evolution-ingest"], "success");
    assert_eq!(ds["cumulo.iota-immutable-publisher"], "skipped-config-missing");
    assert_eq!(out["data"]["purged"].as_bool(), Some(true));
    assert!(!repo.join(".events/dead-letter").exists() || fs::read_dir(repo.join(".events/dead-letter")).map(|i| i.count()).unwrap_or(0) == 0);
    assert!(!repo
        .join(".SddIA/proofs/dlt-telemetry/aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee.json")
        .is_file());
}

#[test]
fn fractal_fanout_simulate_persists_proof_then_purges() {
    let _g = IOTA_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let prev_sim = std::env::var("SDDIA_LAB_SIMULATE_IOTA").ok();
    std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "1");

    let dir = tempdir().unwrap();
    let repo = dir.path();
    write_min_repo(repo);

    let out = route_domain_fractal_event(repo, ".events/domain/telem.json");
    restore("SDDIA_LAB_SIMULATE_IOTA", prev_sim);

    assert_eq!(out["success"].as_bool(), Some(true), "{out}");
    let ds = &out["data"]["delivery_status"];
    assert_eq!(ds["cumulo.memory-evolution-ingest"], "success");
    assert_eq!(ds["cumulo.iota-immutable-publisher"], "success");
    assert_eq!(out["data"]["purged"].as_bool(), Some(true));
    let proof_path = repo.join(".SddIA/proofs/dlt-telemetry/aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee.json");
    assert!(proof_path.is_file(), "proof missing after purge");
    let proof: Value = serde_json::from_str(&fs::read_to_string(&proof_path).unwrap()).unwrap();
    let digest = proof["transaction_digest"].as_str().unwrap_or("");
    assert!(digest.starts_with("lab-sim-"), "{digest}");
    assert_ne!(digest, "batched-digest");
}
