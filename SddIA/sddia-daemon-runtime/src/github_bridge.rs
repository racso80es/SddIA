//! Materialización ECST PullRequest_Presented (DEBT-K2 — sin Python).

use crate::eda_bus::{ensure_event_bus_topology, write_json_atomic, EdBusPaths};
use crate::{write_fractal_event, BusTopology};
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
const CI_FAILURE_FIXTURE_REL: &str = ".SddIA/.dev/remote_ci_failure_simulation.json";
const SIGNER_RBAC: &str = "Vertice_Biologico_Relay";
const FALLBACK_FLAG: &str = "FALLBACK_LOCAL_SIGNATURE";
const IOTA_RETRIES: usize = 3;
const IOTA_BACKOFF_SECONDS: [u64; 3] = [1, 2, 4];
const KNOWN_SDDIA_QA_JOBS: &[&str] = &[
    "sddia-index-integrity",
    "eda-iota-smoke-simulate",
    "wasi-runtime-smoke",
    "eda-bus-e2e-smoke",
    "eda-iota-physical",
];

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BridgeState {
    #[serde(default)]
    pub processed_pr_urls: Vec<String>,
    #[serde(default)]
    pub processed_check_run_ids: Vec<i64>,
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

pub fn workflow_name_for_job(job_name: &str) -> &'static str {
    if KNOWN_SDDIA_QA_JOBS.contains(&job_name) {
        "sddia-index-qa"
    } else {
        "github-actions"
    }
}

pub fn check_run_id(item: &Value) -> Option<i64> {
    if let Some(n) = item.get("id").and_then(|v| v.as_i64()) {
        return Some(n);
    }
    item.get("id")
        .and_then(|v| v.as_u64())
        .and_then(|n| i64::try_from(n).ok())
}

pub fn is_failure_check_run(item: &Value) -> bool {
    item.get("conclusion").and_then(|v| v.as_str()) == Some("failure")
}

pub fn failed_check_runs_from_payload(data: &Value) -> Vec<Value> {
    let runs = data
        .get("check_runs")
        .and_then(|v| v.as_array())
        .or_else(|| data.as_array());
    let Some(runs) = runs else {
        return vec![];
    };
    runs.iter()
        .filter(|item| is_failure_check_run(item))
        .cloned()
        .collect()
}

pub fn load_lab_ci_failure_payload(repo: &Path) -> Option<Value> {
    let path = repo.join(CI_FAILURE_FIXTURE_REL);
    if !path.is_file() {
        return None;
    }
    let raw = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&raw).ok()
}

pub fn compose_ci_job_failed_event(
    check: &Value,
    repository: &str,
    pr_url: Option<&str>,
    head_sha_fallback: &str,
) -> Option<Value> {
    if !is_failure_check_run(check) {
        return None;
    }
    let id = check_run_id(check)?;
    let job_name = check.get("name").and_then(|v| v.as_str()).unwrap_or("");
    if job_name.is_empty() {
        return None;
    }
    let head_sha = check
        .get("head_sha")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or(head_sha_fallback);
    let html_url = check.get("html_url").and_then(|v| v.as_str()).unwrap_or("");
    let mut payload = json!({
        "repository": repository,
        "head_sha": head_sha,
        "workflow_name": workflow_name_for_job(job_name),
        "job_name": job_name,
        "conclusion": "failure",
        "html_url": html_url,
        "check_run_id": id,
    });
    if let Some(pr) = pr_url.filter(|s| !s.is_empty()) {
        payload["pr_url"] = json!(pr);
    }
    if let Some(run_id) = check.get("run_id") {
        if !run_id.is_null() {
            payload["run_id"] = run_id.clone();
        }
    }
    if let Some(step) = check.get("step_name").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        payload["step_name"] = json!(step);
    }
    Some(json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "CI_Job_Failed",
        "timestamp": iso_now(),
        "emitter_agent": "github-bridge-watcher",
        "payload": payload,
    }))
}

pub fn assimilate_failed_check_runs(
    repo: &Path,
    top: &BusTopology,
    state: &mut BridgeState,
    checks: &[Value],
    repository: &str,
    pr_url: Option<&str>,
    head_sha: &str,
) -> Result<u32, String> {
    let mut emitted = 0u32;
    for check in checks {
        let Some(id) = check_run_id(check) else {
            continue;
        };
        if state.processed_check_run_ids.contains(&id) {
            continue;
        }
        if !is_failure_check_run(check) {
            continue;
        }
        let Some(event) = compose_ci_job_failed_event(check, repository, pr_url, head_sha) else {
            continue;
        };
        write_fractal_event(repo, top, &event, "telemetry")?;
        state.processed_check_run_ids.push(id);
        emitted += 1;
    }
    if emitted > 0 {
        save_bridge_state(repo, state)?;
    }
    Ok(emitted)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_bus_topology;
    use std::fs;

    fn fixture_repo() -> (PathBuf, BusTopology) {
        let base = std::env::temp_dir().join(format!("sddia-gh-bridge-ci-{}", Uuid::new_v4()));
        fs::create_dir_all(base.join("SddIA/core")).unwrap();
        fs::write(
            base.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_fractal":{"telemetry":".events/telemetry"}}"#,
        )
        .unwrap();
        fs::create_dir_all(base.join(".events/telemetry")).unwrap();
        let top = load_bus_topology(&base);
        (base, top)
    }

    fn telemetry_files(repo: &Path) -> Vec<Value> {
        let dir = repo.join(".events/telemetry");
        let mut out = Vec::new();
        if let Ok(rd) = fs::read_dir(&dir) {
            for ent in rd.flatten() {
                let p = ent.path();
                if p.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                if let Ok(raw) = fs::read_to_string(&p) {
                    if let Ok(v) = serde_json::from_str::<Value>(&raw) {
                        out.push(v);
                    }
                }
            }
        }
        out
    }

    #[test]
    fn failure_emits_once_cancelled_skipped() {
        let (base, top) = fixture_repo();
        let payload = json!({
            "check_runs": [
                {
                    "id": 4242,
                    "name": "sddia-index-integrity",
                    "head_sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "conclusion": "failure",
                    "html_url": "https://github.com/racso80es/SddIA/runs/4242"
                },
                {
                    "id": 4243,
                    "name": "wasi-runtime-smoke",
                    "head_sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "conclusion": "cancelled",
                    "html_url": "https://github.com/racso80es/SddIA/runs/4243"
                },
                {
                    "id": 4244,
                    "name": "eda-bus-e2e-smoke",
                    "head_sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "conclusion": "skipped",
                    "html_url": "https://github.com/racso80es/SddIA/runs/4244"
                }
            ]
        });
        let checks = failed_check_runs_from_payload(&payload);
        let mut state = BridgeState::default();
        let n = assimilate_failed_check_runs(
            &base,
            &top,
            &mut state,
            &checks,
            "racso80es/SddIA",
            Some("https://github.com/racso80es/SddIA/pull/1"),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .unwrap();
        assert_eq!(n, 1);
        let events = telemetry_files(&base);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0]["event_type"], "CI_Job_Failed");
        assert_eq!(events[0]["emitter_agent"], "github-bridge-watcher");
        assert_eq!(events[0]["payload"]["workflow_name"], "sddia-index-qa");
        assert_eq!(events[0]["payload"]["job_name"], "sddia-index-integrity");
        assert_eq!(events[0]["payload"]["check_run_id"], 4242);
        assert!(events[0]["payload"].get("entity_id").is_none());
        assert!(events[0]["payload"].get("asset_id").is_none());

        let n2 = assimilate_failed_check_runs(
            &base,
            &top,
            &mut state,
            &checks,
            "racso80es/SddIA",
            Some("https://github.com/racso80es/SddIA/pull/1"),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .unwrap();
        assert_eq!(n2, 0);
        assert_eq!(telemetry_files(&base).len(), 1);
        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn unknown_job_maps_workflow_github_actions() {
        assert_eq!(workflow_name_for_job("sddia-index-integrity"), "sddia-index-qa");
        assert_eq!(workflow_name_for_job("custom-job"), "github-actions");
    }
}
