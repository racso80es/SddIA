//! Materialización ECST PullRequest_Presented (DEBT-K2 — sin Python).

use crate::eda_bus::{ensure_event_bus_topology, write_json_atomic, EdBusPaths};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

const STATE_REL: &str = ".SddIA/.dev/github_bridge_state.json";
const SIGNER_RBAC: &str = "Vertice_Biologico_Relay";
const FALLBACK_FLAG: &str = "FALLBACK_LOCAL_SIGNATURE";
const IOTA_RETRIES: usize = 3;
const IOTA_BACKOFF_SECONDS: [u64; 3] = [1, 2, 4];

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BridgeState {
    #[serde(default)]
    pub processed_pr_urls: Vec<String>,
}

pub fn load_bridge_state(repo: &Path) -> BridgeState {
    let path = repo.join(STATE_REL);
    if !path.is_file() {
        return BridgeState::default();
    }
    let Ok(raw) = fs::read_to_string(&path) else {
        return BridgeState::default();
    };
    serde_json::from_str(&raw).unwrap_or_default()
}

pub fn save_bridge_state(repo: &Path, state: &BridgeState) -> Result<(), String> {
    let path = repo.join(STATE_REL);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir state: {e}"))?;
    }
    write_json_atomic(&path, &serde_json::to_value(state).map_err(|e| e.to_string())?)
}

pub fn load_wallet_secret(repo: &Path) {
    if !std::env::var("IOTA_WALLET_SECRET")
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        return;
    }
    let wallet_path = repo.join(".SddIA/.dev/wallet.key");
    if !wallet_path.is_file() {
        return;
    }
    if let Ok(secret) = fs::read_to_string(&wallet_path) {
        let secret = secret.trim();
        if !secret.is_empty() {
            std::env::set_var("IOTA_WALLET_SECRET", secret);
        }
    }
}

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn compose_pre_anchor_event(pr: &Value) -> Value {
    json!({
        "event_type": "PullRequest_Presented",
        "timestamp": iso_now(),
        "emitter_agent": "github-bridge-watcher",
        "payload": {
            "repository": pr.get("repository").and_then(|v| v.as_str()).unwrap_or(""),
            "branch": pr.get("branch").and_then(|v| v.as_str()).unwrap_or(""),
            "pr_url": pr.get("pr_url").and_then(|v| v.as_str()).unwrap_or(""),
            "status": "presented",
            "origin_agent": pr.get("origin_agent").and_then(|v| v.as_str()).unwrap_or("jules"),
            "signer_identity_rbac": SIGNER_RBAC,
        }
    })
}

pub fn resolve_execute_process_bin(repo: &Path) -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        let p = p.trim();
        if !p.is_empty() {
            return Ok(PathBuf::from(p));
        }
    }
    for rel in ["SddIA/target/debug/execute-process", "SddIA/target/release/execute-process"] {
        let candidate = repo.join(rel);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err("binario execute-process no encontrado (compilar: cd SddIA && cargo build -p execute-process)".into())
}

fn simulate_iota_enabled() -> bool {
    matches!(
        std::env::var("SDDIA_LAB_SIMULATE_IOTA")
            .unwrap_or_default()
            .trim()
            .to_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

fn invoke_tool_capsule(repo: &Path, tool: &str, payload: &Value) -> Result<(i32, Value), String> {
    let bin = resolve_execute_process_bin(repo)?;
    let inputs = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    let output = Command::new(&bin)
        .args(["--tool", tool, "--prefer-native", "--inputs", &inputs])
        .current_dir(repo)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("spawn execute-process --tool {tool}: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.trim();
    if line.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(if stderr.trim().is_empty() {
            format!("{tool} sin salida")
        } else {
            stderr.trim().to_string()
        });
    }
    let body: Value = serde_json::from_str(line).map_err(|e| format!("JSON {tool}: {e}"))?;
    Ok((output.status.code().unwrap_or(1), body))
}

fn invoke_iota_publisher(repo: &Path, event: &Value) -> (bool, String, Option<String>) {
    if simulate_iota_enabled() {
        let digest = format!("lab-sim-{}", &Uuid::new_v4().simple().to_string()[..24]);
        return (true, "lab-simulated".into(), Some(digest));
    }
    let payload = json!({
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": serde_json::to_string(event).unwrap_or_default(),
    });
    match invoke_tool_capsule(repo, "iota-immutable-publisher", &payload) {
        Ok((_, body)) if body.get("success") == Some(&json!(true)) => {
            let digest = body
                .get("result")
                .and_then(|r| r.get("transaction_digest"))
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);
            let feedback = body
                .get("feedback")
                .or_else(|| body.get("message"))
                .and_then(|v| v.as_str())
                .unwrap_or("ok")
                .to_string();
            (true, feedback, digest)
        }
        Ok((code, body)) => {
            let err = body
                .get("error")
                .or_else(|| body.get("message"))
                .or_else(|| body.get("feedback"))
                .and_then(|v| v.as_str())
                .unwrap_or("iota publish failed");
            (false, format!("{err} (exit {code})"), None)
        }
        Err(e) => (false, e, None),
    }
}

pub fn publish_with_retries(repo: &Path, event: &Value) -> (Option<String>, Option<String>, Option<String>) {
    let mut last_error: Option<String> = None;
    for (attempt, delay) in IOTA_BACKOFF_SECONDS.iter().enumerate() {
        let (ok, feedback, digest) = invoke_iota_publisher(repo, event);
        if ok {
            if let Some(d) = digest {
                return (Some(d), None, None);
            }
        }
        last_error = Some(feedback);
        if attempt + 1 < IOTA_RETRIES {
            thread::sleep(Duration::from_secs(*delay));
        }
    }
    (None, None, last_error)
}

pub fn build_bus_event(pre_event: &Value, transaction_digest: &str, object_id: Option<&str>) -> Value {
    let mut payload = pre_event
        .get("payload")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    payload.insert(
        "dlt_anchor_address".into(),
        json!(object_id.unwrap_or(transaction_digest)),
    );
    json!({
        "event_id": transaction_digest,
        "event_type": pre_event.get("event_type").and_then(|v| v.as_str()).unwrap_or("PullRequest_Presented"),
        "timestamp": pre_event.get("timestamp").and_then(|v| v.as_str()).unwrap_or(iso_now().as_str()),
        "emitter_agent": pre_event.get("emitter_agent").and_then(|v| v.as_str()).unwrap_or("github-bridge-watcher"),
        "payload": Value::Object(payload),
        "delivery_state": {
            "argos": "pending",
            "cumulo": "success",
        }
    })
}

pub fn materialize_to_bus(repo: &Path, bus_event: &Value) -> Result<Option<PathBuf>, String> {
    let event_id = bus_event
        .get("event_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "event_id inválido para materialización".to_string())?;
    let bus = ensure_event_bus_topology(repo)?;
    let target = bus.pending.join(format!("{event_id}.json"));
    if target.is_file() {
        return Ok(None);
    }
    write_json_atomic(&target, bus_event)?;
    Ok(Some(target))
}

pub fn write_fallback_dead_letter(repo: &Path, pr: &Value, error: &str) -> Result<PathBuf, String> {
    let bus: EdBusPaths = ensure_event_bus_topology(repo)?;
    let stamp = Utc::now().format("%Y%m%dT%H%M%SZ");
    let name = format!("github-bridge-{stamp}-{}.json", &Uuid::new_v4().simple().to_string()[..8]);
    let target = bus.dead_letter.join(name);
    let record = json!({
        "flag": FALLBACK_FLAG,
        "timestamp": iso_now(),
        "source": "github-bridge-watcher",
        "error": error,
        "pr": {
            "repository": pr.get("repository"),
            "branch": pr.get("branch"),
            "pr_url": pr.get("pr_url"),
            "origin_agent": pr.get("origin_agent"),
        }
    });
    write_json_atomic(&target, &record)?;
    Ok(target)
}

pub fn process_pr(repo: &Path, pr: &Value, state: &mut BridgeState) -> Result<bool, String> {
    let pr_url = pr.get("pr_url").and_then(|v| v.as_str()).unwrap_or("");
    if pr_url.is_empty() {
        return Ok(false);
    }
    if state.processed_pr_urls.iter().any(|u| u == pr_url) {
        return Ok(false);
    }

    eprintln!("[GITHUB-BRIDGE] Procesando PR remoto: {pr_url}");
    load_wallet_secret(repo);
    let pre_event = compose_pre_anchor_event(pr);
    let (digest, _object_id, err) = publish_with_retries(repo, &pre_event);
    let Some(digest) = digest else {
        let dl = write_fallback_dead_letter(repo, pr, err.as_deref().unwrap_or("iota publish failed"))?;
        if let Ok(rel) = dl.strip_prefix(repo) {
            eprintln!("[GITHUB-BRIDGE] Fallback dead-letter: {}", rel.display());
        }
        return Ok(false);
    };

    let bus_event = build_bus_event(&pre_event, &digest, None);
    match materialize_to_bus(repo, &bus_event)? {
        Some(target) => {
            if let Ok(rel) = target.strip_prefix(repo) {
                eprintln!("[GITHUB-BRIDGE] Materializado: {}", rel.display());
            }
        }
        None => eprintln!("[GITHUB-BRIDGE] Idempotente: evento {digest} ya en pending"),
    }

    state.processed_pr_urls.push(pr_url.to_string());
    save_bridge_state(repo, state)?;
    Ok(true)
}
