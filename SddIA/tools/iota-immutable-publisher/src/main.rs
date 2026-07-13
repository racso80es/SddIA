use sddia_io::outbound_lab::{
    find_repo_root_from_cwd, lab_mock_iota_url, lab_sim_digest, lab_simulate_iota_enabled,
    load_iota_wallet_secret,
};
use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
fn main() {
    let _req = read_stdin_json();
    emit_error(
        "WASI environment cannot natively execute IOTA SDK without host capability delegation.",
        1,
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let req = read_stdin_json();
    match run(&req) {
        Ok(result) => {
            emit_success(Some(result));
        }
        Err(msg) => emit_error(&msg, 1),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run(req: &Value) -> Result<Value, String> {
    let action = req
        .get("action")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Campo obligatorio ausente o inválido: action".to_string())?;
    if action != "publish_immutable_data" {
        return Err(format!("Acción no soportada: {action}"));
    }

    let network = req
        .get("network")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Campo obligatorio ausente o inválido: network".to_string())?;

    let payload = req
        .get("payload")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Campo obligatorio ausente o inválido: payload".to_string())?;

    if lab_simulate_iota_enabled() {
        return Ok(simulated_result(network));
    }

    if let Some(mock_url) = lab_mock_iota_url() {
        return post_mock(&mock_url, network, payload);
    }

    let repo = find_repo_root_from_cwd();
    if load_iota_wallet_secret(repo.as_deref()).is_none() {
        return Err("config-missing: IOTA_WALLET_SECRET".into());
    }

    if let Ok(relay) = std::env::var("IOTA_PUBLISH_RELAY_URL") {
        let relay = relay.trim().to_string();
        if !relay.is_empty() {
            return publish_via_relay(&relay, network, payload);
        }
    }

    Err(
        "iota-publish-unavailable: configure SDDIA_LAB_SIMULATE_IOTA, SDDIA_LAB_MOCK_IOTA_URL o IOTA_PUBLISH_RELAY_URL"
            .into(),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn simulated_result(network: &str) -> Value {
    json!({
        "transaction_digest": lab_sim_digest("lab-sim"),
        "object_id": null,
        "network": network,
        "mode": "lab-simulated",
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn post_mock(mock_url: &str, network: &str, payload: &str) -> Result<Value, String> {
    let agent = ureq::agent();
    let body = json!({
        "action": "publish_immutable_data",
        "network": network,
        "payload": payload,
    });
    let resp = agent
        .post(mock_url)
        .set("Content-Type", "application/json")
        .timeout(Duration::from_secs(15))
        .send_string(&body.to_string())
        .map_err(|e| format!("mock-iota-unreachable: {e}"))?;
    let parsed: Value = resp
        .into_json()
        .map_err(|e| format!("mock-iota-invalid-json: {e}"))?;
    if parsed.get("success").and_then(|v| v.as_bool()) != Some(true) {
        let err = parsed
            .get("error")
            .or_else(|| parsed.get("feedback"))
            .and_then(|v| v.as_str())
            .unwrap_or("mock-iota-failed");
        return Err(err.to_string());
    }
    let digest = parsed
        .get("result")
        .and_then(|r| r.get("transaction_digest"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "mock-iota-missing-digest".to_string())?;
    Ok(json!({
        "transaction_digest": digest,
        "object_id": parsed.get("result").and_then(|r| r.get("object_id")).cloned().unwrap_or(Value::Null),
        "mode": "lab-mock-http",
    }))
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_via_relay(relay_url: &str, network: &str, payload: &str) -> Result<Value, String> {
    let agent = ureq::agent();
    let body = json!({
        "action": "publish_immutable_data",
        "network": network,
        "payload": payload,
    });
    let resp = agent
        .post(relay_url)
        .set("Content-Type", "application/json")
        .timeout(Duration::from_secs(30))
        .send_string(&body.to_string())
        .map_err(|e| format!("iota-relay-unreachable: {e}"))?;
    let parsed: Value = resp
        .into_json()
        .map_err(|e| format!("iota-relay-invalid-json: {e}"))?;
    if parsed.get("success").and_then(|v| v.as_bool()) != Some(true) {
        let err = parsed
            .get("error")
            .or_else(|| parsed.get("feedback"))
            .and_then(|v| v.as_str())
            .unwrap_or("iota publish failed");
        return Err(err.to_string());
    }
    let digest = parsed
        .get("result")
        .and_then(|r| r.get("transaction_digest"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "iota-relay-missing-digest".to_string())?;
    Ok(json!({
        "transaction_digest": digest,
        "object_id": parsed.get("result").and_then(|r| r.get("object_id")).cloned().unwrap_or(Value::Null),
        "mode": "relay",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_action() {
        let err = run(&json!({"network": "testnet", "payload": "x"})).unwrap_err();
        assert!(err.contains("action"));
    }

    #[test]
    fn lab_simulate_returns_digest() {
        std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "1");
        let out = run(&json!({
            "action": "publish_immutable_data",
            "network": "testnet",
            "payload": "{}"
        }))
        .expect("simulate");
        let digest = out
            .get("transaction_digest")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        assert!(digest.starts_with("lab-sim-"));
        std::env::remove_var("SDDIA_LAB_SIMULATE_IOTA");
    }
}
