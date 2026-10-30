//! Materialización ECST PullRequest_Presented (DEBT-K2 — sin Python).

use crate::eda_bus::{ensure_event_bus_topology, write_json_atomic, EdBusPaths};
use crate::{write_fractal_event, BusTopology};
use chrono::Utc;
use sddia_event_store::{record_ecst, runtime_db_path, store_mode, writes_filesystem, writes_store};
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

fn frontmatter_field(text: &str, key: &str) -> Option<String> {
    let rest = text.trim_start().strip_prefix("---")?;
    let end = rest.find("\n---")?;
    let prefix = format!("{key}:");
    for line in rest[..end].lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix(&prefix) {
            let v = val.trim().trim_matches('"').trim_matches('\'');
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

fn valid_git_sha(sha: &str) -> bool {
    (7..=40).contains(&sha.len()) && sha.bytes().all(|b| b.is_ascii_hexdigit())
}

fn git_stdout(repo: &Path, args: &[String]) -> Option<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

/// `objectives.md` del commit cabeza cuyo `branch_name` coincide con la rama del PR.
fn objectives_refs_at_head(repo: &Path, sha: &str, branch: &str) -> Option<(Option<String>, Option<String>)> {
    if !valid_git_sha(sha) || branch.is_empty() {
        return None;
    }
    let listing = git_stdout(
        repo,
        &["ls-tree".into(), "-r".into(), "--name-only".into(), sha.into()],
    )?;
    for path in listing.lines() {
        let path = path.trim();
        if path.is_empty() || path.contains("..") || !path.ends_with("objectives.md") {
            continue;
        }
        let spec = format!("{sha}:{path}");
        let Some(text) = git_stdout(repo, &["show".into(), spec]) else {
            continue;
        };
        if frontmatter_field(&text, "branch_name").as_deref() != Some(branch) {
            continue;
        }
        let tracker = frontmatter_field(&text, "tracker_ref");
        let pbi = frontmatter_field(&text, "pbi_ref");
        if tracker.is_some() || pbi.is_some() {
            return Some((tracker, pbi));
        }
    }
    None
}

/// Copia `tracker_ref` / `pbi_ref` desde el blob del head. Si el commit no está o no declara la rama, el PR sale igual.
pub fn attach_tracker_from_head(repo: &Path, pr: &Value) -> Value {
    let mut out = pr.clone();
    let sha = pr.get("head_sha").and_then(|v| v.as_str()).unwrap_or("");
    let branch = pr.get("branch").and_then(|v| v.as_str()).unwrap_or("");
    let Some((tracker, pbi)) = objectives_refs_at_head(repo, sha, branch) else {
        return out;
    };
    if let Some(obj) = out.as_object_mut() {
        if let Some(tr) = tracker {
            obj.insert("tracker_ref".into(), json!(tr));
        }
        if let Some(pbi) = pbi {
            obj.insert("pbi_ref".into(), json!(pbi));
        }
    }
    out
}

pub fn compose_pre_anchor_event(pr: &Value) -> Value {
    let mut payload = json!({
        "repository": pr.get("repository").and_then(|v| v.as_str()).unwrap_or(""),
        "branch": pr.get("branch").and_then(|v| v.as_str()).unwrap_or(""),
        "pr_url": pr.get("pr_url").and_then(|v| v.as_str()).unwrap_or(""),
        "status": "presented",
        "origin_agent": pr.get("origin_agent").and_then(|v| v.as_str()).unwrap_or("jules"),
        "signer_identity_rbac": SIGNER_RBAC,
    });
    for key in ["tracker_ref", "pbi_ref"] {
        if let Some(v) = pr.get(key).and_then(|x| x.as_str()).filter(|s| !s.is_empty()) {
            payload[key] = json!(v);
        }
    }
    json!({
        "event_type": "PullRequest_Presented",
        "timestamp": iso_now(),
        "emitter_agent": "github-bridge-watcher",
        "payload": payload,
    })
}

pub fn resolve_execute_process_bin(repo: &Path) -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        let p = p.trim();
        if !p.is_empty() {
            return Ok(PathBuf::from(p));
        }
    }
    crate::resolve_execute_process_elf(repo).ok_or_else(|| {
        "binario execute-process no encontrado (compilar: cd SddIA && cargo build -p execute-process)"
            .into()
    })
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
        "delivery_state": {},
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
    let mode = store_mode();
    if writes_filesystem(mode) && target.is_file() {
        return Ok(None);
    }
    if writes_store(mode) {
        let db = runtime_db_path(repo);
        if !writes_filesystem(mode) && db.is_file() {
            let store = sddia_event_store::EventStore::open(&db).map_err(|err| err.to_string())?;
            if store
                .get_event(event_id)
                .map_err(|err| err.to_string())?
                .is_some()
            {
                return Ok(None);
            }
        }
        let mut stamped = bus_event.clone();
        sddia_eda_catalog::stamp_published_event(repo, &mut stamped)?;
        record_ecst(repo, "domain", &stamped).map_err(|err| err.to_string())?;
    }
    if !writes_filesystem(mode) {
        return Ok(Some(runtime_db_path(repo)));
    }
    let mut stamped = bus_event.clone();
    sddia_eda_catalog::stamp_published_event(repo, &mut stamped)?;
    write_json_atomic(&target, &stamped)?;
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
    let enriched = attach_tracker_from_head(repo, pr);
    let pre_event = compose_pre_anchor_event(&enriched);
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
            base.join("SddIA/core/event-domain-subscriptions.json"),
            r#"{"CI_Job_Failed":[{"agent":"argos","process":"event-bus-audit"}],"PullRequest_Presented":[{"agent":"tekton","process":"tracker-stamp"}]}"#,
        )
        .unwrap();
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
        if out.is_empty() {
            let db = sddia_event_store::runtime_db_path(repo);
            if db.is_file() {
                if let Ok(store) = sddia_event_store::EventStore::open(db) {
                    if let Ok(rows) = store.list_family(sddia_event_store::Family::Telemetry) {
                        out.extend(rows.into_iter().map(|row| row.ecst));
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

    #[test]
    fn build_bus_event_leaves_delivery_state_empty() {
        let pre = compose_pre_anchor_event(&json!({
            "repository": "racso80es/SddIA",
            "branch": "fix/x",
            "pr_url": "https://github.com/racso80es/SddIA/pull/1",
            "tracker_ref": "AUT-25",
            "pbi_ref": "docs/fixes/demo/objectives.md",
        }));
        assert_eq!(pre["payload"]["tracker_ref"], "AUT-25");
        assert_eq!(pre["payload"]["pbi_ref"], "docs/fixes/demo/objectives.md");
        let bus = build_bus_event(&pre, "digest-1", None);
        assert_eq!(bus["delivery_state"], json!({}));
        assert_eq!(bus["payload"]["dlt_anchor_address"], "digest-1");
        assert_eq!(bus["payload"]["tracker_ref"], "AUT-25");
    }

    #[test]
    fn materialize_stamps_catalog_plan() {
        let (base, _) = fixture_repo();
        fs::write(
            base.join("SddIA/core/event-domain-subscriptions.json"),
            r#"{"PullRequest_Presented":[
                {"agent":"argos","process":"pull-request-review"},
                {"agent":"tekton","process":"tracker-stamp"},
                {"agent":"argos","tool":"send-telegram-notification"}
            ]}"#,
        )
        .unwrap();
        let pre = compose_pre_anchor_event(&json!({
            "repository": "racso80es/SddIA",
            "branch": "fix/x",
            "pr_url": "https://github.com/racso80es/SddIA/pull/9",
        }));
        let bus = build_bus_event(&pre, "digest-catalog", None);
        materialize_to_bus(&base, &bus).unwrap();
        let store = sddia_event_store::EventStore::open(sddia_event_store::runtime_db_path(&base)).unwrap();
        let row = store.get_event("digest-catalog").unwrap().expect("event");
        let ds = row.ecst.get("delivery_state").unwrap();
        assert_eq!(ds["tekton.tracker-stamp"], "pending");
        assert_eq!(ds["argos.pull-request-review"], "pending");
        assert_eq!(ds["argos.send-telegram-notification"], "pending");
        assert!(ds.get("argos").is_none());
        assert!(ds.get("cumulo").is_none());
        let _ = fs::remove_dir_all(base);
    }

    fn git_ok(repo: &Path, args: &[&str]) {
        let status = Command::new("git")
            .args(args)
            .current_dir(repo)
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@example.com")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@example.com")
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?}");
    }

    #[test]
    fn attach_tracker_reads_objectives_at_head() {
        let base = std::env::temp_dir().join(format!("sddia-gh-bridge-git-{}", Uuid::new_v4()));
        fs::create_dir_all(base.join("docs/fixes/other")).unwrap();
        fs::create_dir_all(base.join("docs/fixes/demo")).unwrap();
        git_ok(&base, &["init"]);
        fs::write(
            base.join("docs/fixes/other/objectives.md"),
            "---\nbranch_name: fix/other\ntracker_ref: AUT-1\n---\n",
        )
        .unwrap();
        fs::write(
            base.join("docs/fixes/demo/objectives.md"),
            "---\nbranch_name: fix/demo\ntracker_ref: AUT-25\npbi_ref: docs/fixes/demo/item.md\n---\n",
        )
        .unwrap();
        git_ok(&base, &["add", "."]);
        git_ok(&base, &["commit", "-m", "obj"]);
        let sha = git_stdout(
            &base,
            &["rev-parse".into(), "HEAD".into()],
        )
        .unwrap();
        let sha = sha.trim();
        let pr = json!({
            "repository": "racso80es/SddIA",
            "branch": "fix/demo",
            "head_sha": sha,
            "pr_url": "https://github.com/racso80es/SddIA/pull/2",
        });
        let enriched = attach_tracker_from_head(&base, &pr);
        assert_eq!(enriched["tracker_ref"], "AUT-25");
        assert_eq!(enriched["pbi_ref"], "docs/fixes/demo/item.md");
        let miss = json!({
            "branch": "fix/absent",
            "head_sha": sha,
        });
        let plain = attach_tracker_from_head(&base, &miss);
        assert!(plain.get("tracker_ref").is_none());
        let _ = fs::remove_dir_all(base);
    }
}
