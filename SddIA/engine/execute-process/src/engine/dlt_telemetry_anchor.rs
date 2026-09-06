//! Anclaje DLT de `Domain_Entity_Telemetry_Captured` (proof durable + skip config).

use super::eda_bus_topology::write_json_atomic;
use super::persist_pec_correlation_proof::resolve_eda_proofs_dir;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use sddia_io::outbound_lab::{lab_mock_iota_url, lab_simulate_iota_enabled, load_iota_wallet_secret};

pub static IOTA_ENV_LOCK: Mutex<()> = Mutex::new(());

pub const EVENT_TYPE: &str = "Domain_Entity_Telemetry_Captured";
pub const PROOF_NAMESPACE: &str = "dlt-telemetry";
pub const SKIP_STATUS: &str = "skipped-config-missing";

pub fn is_telemetry_captured(event: &Value) -> bool {
    event.get("event_type").and_then(|v| v.as_str()) == Some(EVENT_TYPE)
}

pub fn is_iota_config_error(feedback: &str) -> bool {
    let f = feedback.to_lowercase();
    f.contains("config-missing") || f.contains("iota-publish-unavailable")
}

pub fn digest_is_valid(digest: &str) -> bool {
    let d = digest.trim();
    !d.is_empty() && d != "batched-digest"
}

/// None → no skip (simulado, mock o bóveda+relay presentes).
pub fn telemetry_iota_skip_config(repo: &Path) -> Option<String> {
    if lab_simulate_iota_enabled() || lab_mock_iota_url().is_some() {
        return None;
    }
    if load_iota_wallet_secret(Some(repo)).is_none() {
        return Some("config-missing: IOTA_WALLET_SECRET".into());
    }
    let relay = std::env::var("IOTA_PUBLISH_RELAY_URL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    if relay.is_none() {
        return Some(
            "iota-publish-unavailable: configure SDDIA_LAB_SIMULATE_IOTA, SDDIA_LAB_MOCK_IOTA_URL o IOTA_PUBLISH_RELAY_URL"
                .into(),
        );
    }
    None
}

pub fn proof_mode() -> &'static str {
    if lab_simulate_iota_enabled() {
        "lab-simulated"
    } else {
        "relay"
    }
}

pub fn apply_digest_to_delivery_state(event: &mut Value, digest: &str) {
    let obj = match event.as_object_mut() {
        Some(o) => o,
        None => return,
    };
    if !obj.contains_key("delivery_state") {
        obj.insert("delivery_state".into(), json!({}));
    }
    if let Some(ds) = obj.get_mut("delivery_state").and_then(|v| v.as_object_mut()) {
        ds.insert("transaction_digest".into(), json!(digest));
        ds.insert("cumulo".into(), json!("success"));
    }
}

pub fn stamp_digest_on_event_file(path: &Path, digest: &str) {
    if !path.is_file() {
        return;
    }
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    let Ok(mut body) = serde_json::from_str::<Value>(&text) else {
        return;
    };
    apply_digest_to_delivery_state(&mut body, digest);
    let _ = write_json_atomic(path, &body);
}

pub fn persist_dlt_telemetry_proof(
    repo: &Path,
    event: &Value,
    digest: &str,
) -> Result<String, String> {
    if !digest_is_valid(digest) {
        return Err("invalid-digest".into());
    }
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_id missing")?;
    let entity_id = event
        .get("payload")
        .and_then(|p| p.get("entity_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let dir = resolve_eda_proofs_dir(repo).join(PROOF_NAMESPACE);
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir dlt-telemetry proof: {e}"))?;
    let path = dir.join(format!("{event_id}.json"));
    let body = json!({
        "kind": "dlt-telemetry-proof",
        "event_id": event_id,
        "event_type": EVENT_TYPE,
        "entity_id": entity_id,
        "network": "testnet",
        "transaction_digest": digest,
        "mode": proof_mode(),
        "anchored_at": Utc::now().to_rfc3339(),
    });
    write_json_atomic(&path, &body)?;
    Ok(path
        .strip_prefix(repo)
        .unwrap_or(&path)
        .to_string_lossy()
        .replace('\\', "/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn restore(key: &str, prev: Option<String>) {
        match prev {
            Some(v) => std::env::set_var(key, v),
            None => std::env::remove_var(key),
        }
    }

    #[test]
    fn digest_rejects_batched_and_empty() {
        assert!(!digest_is_valid(""));
        assert!(!digest_is_valid("batched-digest"));
        assert!(digest_is_valid("lab-sim-abc"));
        assert!(digest_is_valid("8TaP3rFM27J76NDzKoGRPd72ysAr7TWSkJdKnk5HpAdG"));
    }

    #[test]
    fn config_error_detects_capsule_traces() {
        assert!(is_iota_config_error("config-missing: IOTA_WALLET_SECRET"));
        assert!(is_iota_config_error(
            "iota-publish-unavailable: configure SDDIA_LAB_SIMULATE_IOTA"
        ));
        assert!(!is_iota_config_error("iota-relay-unreachable: Connection refused"));
    }

    #[test]
    fn skip_when_no_wallet_no_simulate() {
        let _g = IOTA_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let sim = std::env::var("SDDIA_LAB_SIMULATE_IOTA").ok();
        let mock_out = std::env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
        let mock_url = std::env::var("SDDIA_LAB_MOCK_IOTA_URL").ok();
        let secret = std::env::var("IOTA_WALLET_SECRET").ok();
        let relay = std::env::var("IOTA_PUBLISH_RELAY_URL").ok();
        std::env::remove_var("SDDIA_LAB_SIMULATE_IOTA");
        std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        std::env::remove_var("SDDIA_LAB_MOCK_IOTA_URL");
        std::env::remove_var("IOTA_WALLET_SECRET");
        std::env::remove_var("IOTA_PUBLISH_RELAY_URL");
        let tmp = tempfile::tempdir().unwrap();
        let reason = telemetry_iota_skip_config(tmp.path());
        restore("SDDIA_LAB_SIMULATE_IOTA", sim);
        restore("SDDIA_LAB_MOCK_OUTBOUND", mock_out);
        restore("SDDIA_LAB_MOCK_IOTA_URL", mock_url);
        restore("IOTA_WALLET_SECRET", secret);
        restore("IOTA_PUBLISH_RELAY_URL", relay);
        assert_eq!(reason.as_deref(), Some("config-missing: IOTA_WALLET_SECRET"));
    }

    #[test]
    fn no_skip_when_simulate() {
        let _g = IOTA_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let sim = std::env::var("SDDIA_LAB_SIMULATE_IOTA").ok();
        let secret = std::env::var("IOTA_WALLET_SECRET").ok();
        std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "1");
        std::env::remove_var("IOTA_WALLET_SECRET");
        let tmp = tempfile::tempdir().unwrap();
        let reason = telemetry_iota_skip_config(tmp.path());
        restore("SDDIA_LAB_SIMULATE_IOTA", sim);
        restore("IOTA_WALLET_SECRET", secret);
        assert_eq!(reason, None);
    }

    #[test]
    fn persist_proof_writes_digest() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        let event = json!({
            "event_id": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
            "event_type": EVENT_TYPE,
            "payload": {"entity_id": "mayeuta-llm"}
        });
        let rel = persist_dlt_telemetry_proof(repo, &event, "lab-sim-testdigest00000001").unwrap();
        assert!(rel.contains("dlt-telemetry"));
        let body: Value = serde_json::from_str(
            &fs::read_to_string(repo.join(".SddIA/proofs/dlt-telemetry/aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee.json"))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(body["transaction_digest"], "lab-sim-testdigest00000001");
        assert_eq!(body["entity_id"], "mayeuta-llm");
        assert_ne!(body["transaction_digest"], "batched-digest");
    }

    #[test]
    fn persist_proof_rejects_batched() {
        let tmp = tempfile::tempdir().unwrap();
        let event = json!({"event_id": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"});
        assert!(persist_dlt_telemetry_proof(tmp.path(), &event, "batched-digest").is_err());
    }
}
