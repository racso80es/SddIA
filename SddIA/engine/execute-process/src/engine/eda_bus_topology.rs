//! Topología bus EDA V3+ y testigos (paridad `eda_bus_utils.py` subset route-domain).

use super::workspace::{
    load_paths_config, resolve_documentation_features_path, resolve_documentation_fixes_path,
};
use chrono::Utc;
use regex::Regex;
use crate::core::store_lock_contention;
use crate::engine::eda_port;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

pub const ECST_GATE_SUBSCRIBER: &str = "ecst-gate";

const DOMAIN_ENTITY_TYPES: &[&str] = &[
    "Domain_Entity_Created",
    "Domain_Entity_Updated",
    "Domain_Entity_Deleted",
];

const BACKFILL_EMITTERS: &[&str] = &["cumulo-eda-backfill"];

const DLT_PLACEHOLDER_HASHES: &[&str] = &["sha256:pending-forge", "sha256:pending", ""];

static UUID4_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")
        .expect("uuid regex")
});

static BRANCH_NUMERIC_SUFFIX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<base>.+)-\d{10,}$").expect("branch suffix regex"));

static GH_PR_URL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)github\.com/[^/]+/[^/]+/pull/(\d+)").expect("gh pr regex"));

#[derive(Debug, Clone)]
pub struct EventBusTopology {
    pub pending: String,
    pub processing: String,
    pub processing_subscribers: String,
    pub processed: String,
    pub processed_subscribers: String,
    pub dead_letter: String,
    pub dead_letter_subscribers: String,
    pub subscriptions: String,
}

fn normalize_rel(path: &str) -> String {
    let p = path.replace('\\', "/");
    if let Some(stripped) = p.strip_prefix("./") {
        stripped.to_string()
    } else {
        p
    }
}

fn bus_defaults_from_root(event_bus: &str) -> EventBusTopology {
    let event_bus = normalize_rel(event_bus.trim_end_matches('/'));
    EventBusTopology {
        pending: format!("{event_bus}/pending"),
        processing: format!("{event_bus}/processing"),
        processing_subscribers: format!("{event_bus}/processing/subscribers"),
        processed: format!("{event_bus}/processed"),
        processed_subscribers: format!("{event_bus}/processed/subscribers"),
        dead_letter: format!("{event_bus}/dead-letter"),
        dead_letter_subscribers: format!("{event_bus}/dead-letter/subscribers"),
        subscriptions: "SddIA/core/event-domain-subscriptions.json".to_string(),
    }
}

pub fn load_event_bus_topology(repo: &Path) -> Result<EventBusTopology, String> {
    let env_raw = std::env::var("EVENT_BUS_PATH").unwrap_or_default();
    let env_bus = if crate::engine::eda_port::is_filesystem_rel(&env_raw) {
        env_raw.trim().to_string()
    } else {
        String::new()
    };
    let mut defaults = if env_bus.is_empty() {
        bus_defaults_from_root(".events")
    } else {
        bus_defaults_from_root(&env_bus)
    };

    let cfg = load_paths_config(repo).unwrap_or(json!({}));
    if env_bus.is_empty() {
        if let Some(eb) = cfg.get("event_bus").and_then(|v| v.as_str()) {
            if crate::engine::eda_port::is_filesystem_rel(eb) {
                defaults = bus_defaults_from_root(eb);
            }
        }
        if let Some(bus) = cfg.get("eda_bus").and_then(|v| v.as_object()) {
            if let Some(p) = bus.get("pending").and_then(|v| v.as_str()) {
                if crate::engine::eda_port::is_filesystem_rel(p) {
                    defaults.pending = normalize_rel(p);
                }
            }
            for key in ["processing", "processed", "dead_letter"] {
                if let Some(rel) = bus.get(key).and_then(|v| v.as_str()) {
                    if crate::engine::eda_port::is_filesystem_rel(rel) {
                        let norm = normalize_rel(rel);
                        let subs = format!("{}/subscribers", norm.trim_end_matches('/'));
                        match key {
                            "processing" => {
                                defaults.processing = norm;
                                defaults.processing_subscribers = subs;
                            }
                            "processed" => {
                                defaults.processed = norm;
                                defaults.processed_subscribers = subs;
                            }
                            "dead_letter" => {
                                defaults.dead_letter = norm;
                                defaults.dead_letter_subscribers = subs;
                            }
                            _ => {}
                        }
                    }
                }
            }
            if let Some(subs) = bus.get("subscribers").and_then(|v| v.as_object()) {
                for (legacy_key, flat_key) in [
                    ("processing", "processing_subscribers"),
                    ("processed", "processed_subscribers"),
                    ("dead_letter", "dead_letter_subscribers"),
                ] {
                    if let Some(rel) = subs.get(legacy_key).and_then(|v| v.as_str()) {
                        if !rel.is_empty() {
                            match flat_key {
                                "processing_subscribers" => {
                                    defaults.processing_subscribers = normalize_rel(rel)
                                }
                                "processed_subscribers" => {
                                    defaults.processed_subscribers = normalize_rel(rel)
                                }
                                "dead_letter_subscribers" => {
                                    defaults.dead_letter_subscribers = normalize_rel(rel)
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    if let Some(subs) = cfg
        .get("eda_bus")
        .and_then(|b| b.get("subscriptions"))
        .and_then(|v| v.as_str())
    {
        if !subs.is_empty() {
            defaults.subscriptions = subs.to_string();
        }
    }
    Ok(defaults)
}

pub fn ensure_event_bus_topology(repo: &Path) -> Result<EventBusTopology, String> {
    let bus = load_event_bus_topology(repo)?;
    for key in [
        &bus.pending,
        &bus.processing,
        &bus.processing_subscribers,
        &bus.processed,
        &bus.processed_subscribers,
        &bus.dead_letter,
        &bus.dead_letter_subscribers,
    ] {
        fs::create_dir_all(repo.join(key)).map_err(|e| e.to_string())?;
    }
    Ok(bus)
}

fn resolve_bus_path(repo: &Path, rel: &str) -> PathBuf {
    let rel = normalize_rel(rel);
    if rel.starts_with("./") {
        repo.join(rel.trim_start_matches("./"))
    } else {
        repo.join(rel)
    }
}

pub fn header_path(bus: &EventBusTopology, state: &str, event_uuid: &str) -> PathBuf {
    let base = match state {
        "processing" => &bus.processing,
        "processed" => &bus.processed,
        "dead_letter" => &bus.dead_letter,
        _ => &bus.processing,
    };
    resolve_bus_path(Path::new(""), base).join(format!("{event_uuid}.json"))
}

fn header_path_repo(repo: &Path, bus: &EventBusTopology, state: &str, event_uuid: &str) -> PathBuf {
    let base = match state {
        "processing" => &bus.processing,
        "processed" => &bus.processed,
        "dead_letter" => &bus.dead_letter,
        _ => &bus.processing,
    };
    resolve_bus_path(repo, base).join(format!("{event_uuid}.json"))
}

pub fn subscriber_id(subscriber: &Value) -> String {
    sddia_eda_catalog::subscriber_id(subscriber)
}

pub fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

pub fn write_json_atomic(path: &Path, payload: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(payload).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, format!("{text}\n")).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}

/// Token lógico de cabecera; MAN-43 O3: sin copia FS (SSOT = puerto).
pub fn ensure_state_header(
    repo: &Path,
    bus: &EventBusTopology,
    state: &str,
    event_uuid: &str,
    _source_path: &Path,
) -> Result<PathBuf, String> {
    Ok(header_path_repo(repo, bus, state, event_uuid))
}

pub fn ensure_processing_header(
    repo: &Path,
    bus: &EventBusTopology,
    event_uuid: &str,
    _pending_path: &Path,
) -> Result<PathBuf, String> {
    Ok(header_path_repo(repo, bus, "processing", event_uuid))
}

fn resolve_witness_key(state_key: &str) -> &str {
    match state_key {
        "subscriber_processing" | "processing" => "processing_subscribers",
        "subscriber_processed" | "processed" => "processed_subscribers",
        "subscriber_dead_letter" | "dead_letter" => "dead_letter_subscribers",
        other => other,
    }
}

fn list_port_witnesses(repo: &Path, state_key: &str, event_uuid: &str) -> Vec<PathBuf> {
    use sddia_core::ports::event_manager::{
        DeliveryStatus as PortDeliveryStatus, ListDeliveriesRequest,
    };
    let folder_key = resolve_witness_key(state_key);
    let Ok(rows) = eda_port::with_event_port(repo, "list_deliveries", |port| {
        port.list_deliveries(ListDeliveriesRequest {
            event_id: Some(event_uuid.to_string()),
            subscriber_id: None,
            status: None,
            cursor: None,
            limit: Some(10_000),
        })
        .map_err(|e| e.to_string())
    }) else {
        return Vec::new();
    };
    let mut paths = Vec::new();
    for row in rows.deliveries {
        if row.event_id != event_uuid {
            continue;
        }
        let Some(sub) = row
            .subscriber_id
            .as_deref()
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let include = matches!(
            (folder_key, row.status),
            (
                "processing_subscribers",
                PortDeliveryStatus::Processing
            ) | ("dead_letter_subscribers", PortDeliveryStatus::Nack)
                | ("processed_subscribers", PortDeliveryStatus::Ack)
        );
        if include {
            paths.push(PathBuf::from(format!("{event_uuid}.{sub}.json")));
        }
    }
    paths.sort();
    paths
}

pub fn list_witnesses(
    repo: &Path,
    _bus: &EventBusTopology,
    state_key: &str,
    event_uuid: &str,
) -> Vec<PathBuf> {
    list_port_witnesses(repo, state_key, event_uuid)
}

pub fn purge_store_event(repo: &Path, event_id: &str) {
    let _ = eda_port::purge_event(repo, event_id);
}

fn delivery_status_from_label(status: &str) -> &'static str {
    let label = status.trim().to_ascii_lowercase();
    if label == "success" || label == "processed" {
        "success"
    } else if label == "failed" || label == "dead-letter" || label == "dead_letter" {
        "failed"
    } else {
        "pending"
    }
}

pub fn terminal_witness_exists(
    repo: &Path,
    _bus: &EventBusTopology,
    event_uuid: &str,
    subscriber_name: &str,
) -> bool {
    eda_port::delivery_terminal_for_subscriber(repo, event_uuid, subscriber_name)
}

pub fn write_processing_witness(
    repo: &Path,
    _bus: &EventBusTopology,
    event_uuid: &str,
    subscriber_name: &str,
    _event_type: &str,
    _dispatch_mode: &str,
) -> Result<PathBuf, String> {
    eda_port::record_delivery_processing(repo, event_uuid, subscriber_name)?;
    Ok(PathBuf::from(format!("{event_uuid}.{subscriber_name}.json")))
}

pub fn delegation_meta(subscriber: &Value, exit_code: i32) -> Value {
    let mut kind = "unknown";
    let mut target = "unknown";
    if let Some(p) = subscriber.get("process").and_then(|v| v.as_str()) {
        if !p.trim().is_empty() {
            kind = "process";
            target = p.trim();
        }
    } else if let Some(a) = subscriber.get("action").and_then(|v| v.as_str()) {
        if !a.trim().is_empty() {
            kind = "action";
            target = a.trim();
        }
    } else if let Some(t) = subscriber.get("tool").and_then(|v| v.as_str()) {
        if !t.trim().is_empty() {
            kind = "tool";
            target = t.trim();
        }
    }
    json!({"kind": kind, "target": target, "exit_code": exit_code})
}

pub fn promote_witness(
    repo: &Path,
    _bus: &EventBusTopology,
    event_uuid: &str,
    subscriber_name: &str,
    to_state: &str,
    extra: Option<&Value>,
    _pending_header: Option<&Path>,
) -> Result<PathBuf, String> {
    match to_state {
        "processed" | "dead-letter" => {}
        other => return Err(format!("unknown witness state: {other}")),
    };
    let label = extra
        .and_then(|e| e.get("result_status").and_then(|v| v.as_str()))
        .unwrap_or(if to_state == "processed" {
            "success"
        } else {
            "dead-letter"
        });
    if delivery_status_from_label(label) == "failed" {
        let cause = extra
            .and_then(|e| e.get("error_trace"))
            .and_then(|v| v.as_str())
            .unwrap_or("dead-letter");
        if store_lock_contention::is_store_lock_contention_message(cause) {
            // MAN-42: contención pura → sin DLQ; entrega sigue pending para el siguiente tick.
        } else {
            eda_port::record_delivery_nack(repo, event_uuid, subscriber_name)?;
            eda_port::move_to_dlq(repo, event_uuid, cause)?;
        }
    } else {
        eda_port::record_delivery_ack(repo, event_uuid, subscriber_name)?;
    }
    Ok(PathBuf::from(format!("{event_uuid}.{subscriber_name}.json")))
}

fn terminal_subscriber_names(
    repo: &Path,
    bus: &EventBusTopology,
    event_uuid: &str,
) -> HashSet<String> {
    let mut names = HashSet::new();
    for key in ["processed_subscribers", "dead_letter_subscribers"] {
        for path in list_witnesses(repo, bus, key, event_uuid) {
            if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                let suffix = fname
                    .strip_prefix(&format!("{event_uuid}."))
                    .and_then(|s| s.strip_suffix(".json"));
                if let Some(s) = suffix {
                    names.insert(s.to_string());
                }
            }
        }
    }
    names
}

fn in_flight_subscriber_names(
    repo: &Path,
    bus: &EventBusTopology,
    event_uuid: &str,
) -> HashSet<String> {
    let mut names = HashSet::new();
    for path in list_witnesses(repo, bus, "processing_subscribers", event_uuid) {
        if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
            let suffix = fname
                .strip_prefix(&format!("{event_uuid}."))
                .and_then(|s| s.strip_suffix(".json"));
            if let Some(s) = suffix {
                names.insert(s.to_string());
            }
        }
    }
    names
}

pub fn resolve_origin_topology(payload: &Value) -> String {
    sddia_eda_catalog::resolve_origin_topology(payload)
}

pub fn subscriber_applies_to_topology(subscriber: &Value, origin_topology: &str) -> bool {
    sddia_eda_catalog::subscriber_applies_to_topology(subscriber, origin_topology)
}

pub fn inject_domain_entity_topology_defaults(event: &mut Value) {
    let event_type = event
        .get("event_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !DOMAIN_ENTITY_TYPES.contains(&event_type) {
        return;
    }
    if let Some(payload) = event.get_mut("payload").and_then(|v| v.as_object_mut()) {
        if !payload.contains_key("origin_topology") {
            payload.insert("origin_topology".to_string(), json!("core"));
        }
    }
}

pub fn is_backfill_emitter(emitter_agent: Option<&str>) -> bool {
    emitter_agent
        .map(|e| BACKFILL_EMITTERS.contains(&e))
        .unwrap_or(false)
}

pub fn dlt_threshold_ok(event: &Value) -> (bool, String) {
    if event.get("event_type").and_then(|v| v.as_str()) != Some("Domain_Entity_Created") {
        return (true, "not-create".into());
    }
    let Some(payload) = event.get("payload").and_then(|v| v.as_object()) else {
        return (false, "payload-missing".into());
    };
    if resolve_origin_topology(&Value::Object(payload.clone())) != "core" {
        return (false, "topology-local".into());
    }
    let entity_uuid = payload
        .get("entity_uuid")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !UUID4_RE.is_match(entity_uuid) {
        return (false, "invalid-uuid".into());
    }
    let hnew = payload
        .get("hash_signature_new")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !hnew.starts_with("sha256:") {
        return (false, "invalid-hash-prefix".into());
    }
    if DLT_PLACEHOLDER_HASHES
        .iter()
        .any(|p| hnew.eq_ignore_ascii_case(p))
    {
        return (false, "placeholder-hash".into());
    }
    let allowed = [
        "process", "agent", "skill", "tool", "action", "norm", "codex", "event", "suite",
    ];
    let entity_class = payload
        .get("entity_class")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !allowed.contains(&entity_class) {
        return (false, "invalid-entity-class".into());
    }
    (true, "ok".into())
}

pub fn infer_persist_ref_from_branch(repo: &Path, branch: &str) -> Option<String> {
    let cfg = load_paths_config(repo).ok()?;
    let features_prefix = resolve_documentation_features_path(repo, &cfg);
    let fixes_prefix = resolve_documentation_fixes_path(repo, &cfg);
    let b = branch.trim();
    let mut candidates = Vec::new();
    if let Some(slug) = b.strip_prefix("feat/") {
        candidates.push(format!("{features_prefix}/{slug}"));
        if let Some(caps) = BRANCH_NUMERIC_SUFFIX_RE.captures(slug) {
            candidates.push(format!("{features_prefix}/{}", &caps["base"]));
        }
    } else if let Some(slug) = b.strip_prefix("refactor/") {
        candidates.push(format!("{features_prefix}/{slug}"));
        if let Some(caps) = BRANCH_NUMERIC_SUFFIX_RE.captures(slug) {
            candidates.push(format!("{features_prefix}/{}", &caps["base"]));
        }
    } else if let Some(slug) = b.strip_prefix("fix/") {
        candidates.push(format!("{fixes_prefix}/{slug}"));
        if let Some(caps) = BRANCH_NUMERIC_SUFFIX_RE.captures(slug) {
            candidates.push(format!("{fixes_prefix}/{}", &caps["base"]));
        }
    } else if let Some(slug) = b.strip_prefix("docs/") {
        // ADU-CA5: docs/{slug} → features primero, fixes en defecto; sin dir → None.
        candidates.push(format!("{features_prefix}/{slug}"));
        candidates.push(format!("{fixes_prefix}/{slug}"));
        if let Some(caps) = BRANCH_NUMERIC_SUFFIX_RE.captures(slug) {
            candidates.push(format!("{features_prefix}/{}", &caps["base"]));
            candidates.push(format!("{fixes_prefix}/{}", &caps["base"]));
        }
    }
    let mut seen = HashSet::new();
    for ref_path in candidates {
        if !seen.insert(ref_path.clone()) {
            continue;
        }
        if repo.join(&ref_path).is_dir() {
            return Some(ref_path);
        }
    }
    None
}

fn gh_executable() -> Option<String> {
    if let Ok(override_path) = std::env::var("SDDIA_GH_EXECUTABLE") {
        let p = PathBuf::from(override_path.trim());
        if p.is_file() {
            return Some(p.to_string_lossy().to_string());
        }
        return None;
    }
    Command::new("which")
        .arg("gh")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let path = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if path.is_empty() {
                None
            } else {
                Some(path)
            }
        })
}

fn parse_pr_number(pr_url: Option<&str>) -> Option<u64> {
    let url = pr_url?.trim();
    GH_PR_URL_RE
        .captures(url)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

pub fn is_lab_simulated_pr_url(pr_url: Option<&str>) -> bool {
    let Some(url) = pr_url.map(str::trim).filter(|s| !s.is_empty()) else {
        return false;
    };
    if url.to_ascii_lowercase().contains("lab-simulated") {
        return true;
    }
    matches!(parse_pr_number(Some(url)), Some(0))
}

fn run_git(repo: &Path, args: &[&str]) -> (i32, String, String) {
    let output = Command::new("git").args(args).current_dir(repo).output();
    match output {
        Ok(o) => (
            o.status.code().unwrap_or(1),
            String::from_utf8_lossy(&o.stdout).trim().to_string(),
            String::from_utf8_lossy(&o.stderr).trim().to_string(),
        ),
        Err(_) => (1, String::new(), String::new()),
    }
}

fn gh_pr_state(pr_url: &str) -> Option<String> {
    let gh = gh_executable()?;
    let output = Command::new(&gh)
        .args(["pr", "view", pr_url.trim(), "--json", "state"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let body: Value = serde_json::from_slice(&output.stdout).ok()?;
    body.get("state")
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

pub fn github_pr_merged(pr_url: &str) -> bool {
    gh_pr_state(pr_url).as_deref() == Some("MERGED")
}

fn branch_exists_on_remote(repo: &Path, branch: &str, fetch: bool) -> bool {
    if fetch {
        run_git(repo, &["fetch", "origin", "--prune"]);
    }
    let (code, _, _) = run_git(
        repo,
        &["rev-parse", "--verify", &format!("origin/{branch}")],
    );
    code == 0
}

fn merged_via_pull_ref(repo: &Path, pr_number: u64, target_branch: &str) -> bool {
    let remote_ref = format!("refs/remotes/origin/.sddia/pr-{pr_number}-head");
    let fetch_spec = format!("pull/{pr_number}/head:{remote_ref}");
    let (code, _, _) = run_git(repo, &["fetch", "origin", &fetch_spec]);
    if code != 0 {
        return false;
    }
    let local_ref = format!("origin/.sddia/pr-{pr_number}-head");
    let target = format!("origin/{}", target_branch.trim());
    let (code_tgt, _, _) = run_git(repo, &["rev-parse", "--verify", &target]);
    if code_tgt != 0 {
        return false;
    }
    let (code_anc, _, _) = run_git(repo, &["merge-base", "--is-ancestor", &local_ref, &target]);
    code_anc == 0
}

pub fn resolve_pull_request_lifecycle(
    repo: &Path,
    branch: &str,
    pr_url: Option<&str>,
    target_branch: &str,
) -> Value {
    let branch = branch.trim();
    let target = if target_branch.trim().is_empty() {
        "main"
    } else {
        target_branch.trim()
    };
    let mut diagnostics: Vec<String> = Vec::new();
    let pr_number = parse_pr_number(pr_url);

    if let Some(url) = pr_url.map(str::trim).filter(|s| !s.is_empty()) {
        match gh_pr_state(url) {
            Some(state) if state == "MERGED" => {
                diagnostics.push("gh:MERGED".into());
                return json!({
                    "merged": true,
                    "source": "gh",
                    "branch_on_remote": branch_exists_on_remote(repo, branch, false),
                    "pr_number": pr_number,
                    "diagnostics": diagnostics,
                });
            }
            Some(state) if state == "OPEN" => diagnostics.push("gh:OPEN".into()),
            Some(state) if state == "CLOSED" => {
                diagnostics.push("gh:CLOSED".into());
                return json!({
                    "merged": false,
                    "source": "gh",
                    "branch_on_remote": branch_exists_on_remote(repo, branch, true),
                    "pr_number": pr_number,
                    "diagnostics": diagnostics,
                });
            }
            Some(_) => diagnostics.push("gh:ERROR".into()),
            None => {
                diagnostics.push(
                    if gh_executable().is_none() {
                        "gh:UNAVAILABLE"
                    } else {
                        "gh:ERROR"
                    }
                    .into(),
                );
            }
        }
    } else {
        diagnostics.push("pr_url:absent".into());
    }

    if branch_exists_on_remote(repo, branch, true) {
        diagnostics.push("branch:remote-present".into());
        return json!({
            "merged": false,
            "source": "branch-remote",
            "branch_on_remote": true,
            "pr_number": pr_number,
            "diagnostics": diagnostics,
        });
    }

    diagnostics.push("branch:remote-absent".into());
    if let Some(num) = pr_number {
        if merged_via_pull_ref(repo, num, target) {
            diagnostics.push("git-pull-ref:ancestor".into());
            return json!({
                "merged": true,
                "source": "git-pull-ref",
                "branch_on_remote": false,
                "pr_number": pr_number,
                "diagnostics": diagnostics,
            });
        }
    }

    json!({
        "merged": Value::Null,
        "source": "unknown",
        "branch_on_remote": false,
        "pr_number": pr_number,
        "diagnostics": diagnostics,
    })
}

pub fn maybe_purge_processing_header(
    repo: &Path,
    bus: &EventBusTopology,
    event_uuid: &str,
    registry: &Value,
    event_type: &str,
    origin_topology: &str,
) -> bool {
    let mut required = Vec::new();
    if let Some(subs) = registry.get(event_type).and_then(|v| v.as_array()) {
        for sub in subs {
            if subscriber_applies_to_topology(sub, origin_topology) {
                required.push(subscriber_id(sub));
            }
        }
    }
    if required.is_empty() {
        return false;
    }
    let terminals = terminal_subscriber_names(repo, bus, event_uuid);
    let in_flight = in_flight_subscriber_names(repo, bus, event_uuid);
    if !required.iter().all(|r| terminals.contains(r)) {
        return false;
    }
    if required.iter().any(|r| in_flight.contains(r)) {
        return false;
    }
    let header = header_path_repo(repo, bus, "processing", event_uuid);
    if header.is_file() {
        fs::remove_file(&header).ok();
        return true;
    }
    false
}

pub fn safe_remove_path(path: &Path) -> bool {
    if !path.is_file() {
        return true;
    }
    for attempt in 0..3 {
        if fs::remove_file(path).is_ok() {
            return true;
        }
        if attempt + 1 < 3 {
            thread::sleep(Duration::from_millis(50));
        }
    }
    !path.is_file()
}

fn processed_subscriber_names(
    repo: &Path,
    bus: &EventBusTopology,
    event_uuid: &str,
) -> HashSet<String> {
    let mut names = HashSet::new();
    for path in list_witnesses(repo, bus, "processed_subscribers", event_uuid) {
        if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
            let suffix = fname
                .strip_prefix(&format!("{event_uuid}."))
                .and_then(|s| s.strip_suffix(".json"));
            if let Some(s) = suffix {
                names.insert(s.to_string());
            }
        }
    }
    names
}

pub fn applicable_subscriber_ids_for_event(
    registry: &Value,
    event_type: &str,
    payload: &Value,
) -> Vec<String> {
    let origin = resolve_origin_topology(payload);
    registry
        .get(event_type)
        .and_then(|v| v.as_array())
        .map(|subs| {
            subs.iter()
                .filter(|sub| subscriber_applies_to_topology(sub, &origin))
                .map(subscriber_id)
                .collect()
        })
        .unwrap_or_default()
}

fn finalize_kaizen_terminal(
    repo: &Path,
    bus: &EventBusTopology,
    event_uuid: &str,
    pending_path: &Path,
    registry: &Value,
    event_type: &str,
    origin_topology: &str,
) -> Value {
    let mut counts = json!({"pending": 0, "headers": 0});
    if pending_path.is_file() && safe_remove_path(pending_path) {
        counts["pending"] = json!(1);
    }
    if eda_port::purge_event(repo, event_uuid).is_ok() {
        counts["headers"] = json!(1);
    }
    if maybe_purge_processing_header(repo, bus, event_uuid, registry, event_type, origin_topology) {
        counts["headers"] = json!(counts["headers"].as_i64().unwrap_or(0) + 1);
    }
    counts
}

fn archive_event_after_sweep(
    repo: &Path,
    bus: &EventBusTopology,
    event_uuid: &str,
    event_type: Option<&str>,
) -> Value {
    let mut counts = json!({"witnesses": 0, "headers": 0, "pending": 0});
    let _ = event_type;
    let pending = resolve_bus_path(repo, &bus.pending).join(format!("{event_uuid}.json"));
    if pending.is_file() && safe_remove_path(&pending) {
        counts["pending"] = json!(1);
    }
    if eda_port::purge_event(repo, event_uuid).is_ok() {
        counts["headers"] = json!(1);
    }
    for state in ["processing", "processed", "dead_letter"] {
        let header = header_path_repo(repo, bus, state, event_uuid);
        if header.is_file() && safe_remove_path(&header) {
            counts["headers"] = json!(counts["headers"].as_i64().unwrap_or(0) + 1);
        }
    }
    counts
}

fn merge_sweep(base: Value, extra: Value) -> Value {
    let mut out = base;
    if let (Some(obj), Some(ex)) = (out.as_object_mut(), extra.as_object()) {
        for (k, v) in ex {
            obj.insert(k.clone(), v.clone());
        }
    }
    out
}

const TRACKER_STAMP_SUBSCRIBER: &str = "tekton.tracker-stamp";

fn payload_str_field(payload: &Value, key: &str) -> Option<String> {
    payload
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn payload_tracker_ref(repo: &Path, payload: &Value) -> Option<String> {
    if let Some(tr) = payload_str_field(payload, "tracker_ref") {
        return Some(tr);
    }
    if let Some(pbi) = payload_str_field(payload, "pbi_ref") {
        if let Some(tr) = super::tracker_pbi_meta::resolve_tracker_ref_from_pbi_ref(repo, &pbi) {
            return Some(tr);
        }
        return Some(pbi);
    }
    payload_str_field(payload, "item_ref")
}

fn event_requires_tracker_done_seal(event_type: &str, payload: &Value) -> bool {
    match event_type {
        "PullRequest_Merged" => true,
        "Delivery_Committed" => {
            payload.get("delivery_mode").and_then(|v| v.as_str()) == Some("trunk_direct")
        }
        _ => false,
    }
}

fn dead_letter_includes_tracker_stamp(dead: &[PathBuf], event_uuid: &str) -> bool {
    let needle = format!("{event_uuid}.{TRACKER_STAMP_SUBSCRIBER}.json");
    dead.iter().any(|p| {
        p.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == needle.as_str())
            .unwrap_or(false)
    })
}

fn pending_tracker_sync_failed_for_ref(
    repo: &Path,
    _bus: &EventBusTopology,
    tracker_ref: &str,
) -> bool {
    let Ok(rows) = eda_port::load_domain_ecsts(repo, 5000) else {
        return false;
    };
    for event in rows {
        if event.get("event_type").and_then(|v| v.as_str()) != Some("Tracker_Sync_Failed") {
            continue;
        }
        let payload = event.get("payload").cloned().unwrap_or(json!({}));
        if payload_tracker_ref(repo, &payload).as_deref() == Some(tracker_ref) {
            return true;
        }
        if payload_str_field(&payload, "issue_ref").as_deref() == Some(tracker_ref) {
            return true;
        }
    }
    false
}

/// `true` cuando el barrido puede tratar el sello Done como reconciliado (o no aplica).
fn tracker_stamp_done_reconciled(
    repo: &Path,
    bus: &EventBusTopology,
    event_type: &str,
    payload: &Value,
    dead: &[PathBuf],
    event_uuid: &str,
) -> bool {
    if !event_requires_tracker_done_seal(event_type, payload) {
        return true;
    }
    if !dead_letter_includes_tracker_stamp(dead, event_uuid) {
        return true;
    }
    let Some(tracker_ref) = payload_tracker_ref(repo, payload) else {
        return false;
    };
    if super::done_gate::read_tracker_done_proof(repo, &tracker_ref).is_some() {
        return true;
    }
    if pending_tracker_sync_failed_for_ref(repo, bus, &tracker_ref) {
        return false;
    }
    false
}

pub fn try_sweep_event(
    repo: &Path,
    bus: &EventBusTopology,
    event_uuid: &str,
    registry: Option<&Value>,
) -> Value {
    let base = json!({"event_uuid": event_uuid, "purged": false});
    let pending_path = resolve_bus_path(repo, &bus.pending).join(format!("{event_uuid}.json"));
    let event: Value = match eda_port::load_dispatched(repo, event_uuid, &pending_path) {
        Ok(Some(event)) => event,
        Ok(None) => return merge_sweep(base, json!({"status": "absent"})),
        Err(_) => return merge_sweep(base, json!({"status": "invalid-json"})),
    };

    let event_type = match event.get("event_type").and_then(|v| v.as_str()) {
        Some(et) if !et.is_empty() => et.to_string(),
        _ => return merge_sweep(base, json!({"status": "missing-event_type"})),
    };

    let payload = event.get("payload").cloned().unwrap_or(json!({}));

    let registry_val: Value;
    let registry_ref: &Value = match registry {
        Some(r) => r,
        None => {
            let subs_path = repo.join(&bus.subscriptions);
            registry_val = match fs::read_to_string(&subs_path) {
                Ok(text) => {
                    let trimmed = text.strip_prefix('\u{feff}').unwrap_or(&text);
                    serde_json::from_str(trimmed).unwrap_or(json!({}))
                }
                Err(_) => {
                    return merge_sweep(
                        base,
                        json!({"status": "invalid-registry", "event_type": event_type}),
                    );
                }
            };
            &registry_val
        }
    };

    let applicable = applicable_subscriber_ids_for_event(registry_ref, &event_type, &payload);
    let dead = list_witnesses(repo, bus, "dead_letter_subscribers", event_uuid);

    if !dead.is_empty() {
        let origin = resolve_origin_topology(&payload);
        let in_flight = in_flight_subscriber_names(repo, bus, event_uuid);
        let terminals = terminal_subscriber_names(repo, bus, event_uuid);
        let applicable_set: HashSet<_> = applicable.iter().cloned().collect();
        if !applicable.is_empty()
            && applicable_set.is_subset(&terminals)
            && applicable_set.is_disjoint(&in_flight)
        {
            if event_type == "System_Fracture_Detected" {
                let materialize = "cumulo.materialize-fracture-pbi";
                let processed = processed_subscriber_names(repo, bus, event_uuid);
                if !processed.contains(materialize) {
                    let dead_names: Vec<_> = dead
                        .iter()
                        .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string))
                        .collect();
                    return merge_sweep(
                        base,
                        json!({
                            "status": "fracture-unmaterialized",
                            "purged": false,
                            "event_type": event_type,
                            "dead_letter_witnesses": dead_names,
                        }),
                    );
                }
            }
            if !tracker_stamp_done_reconciled(
                repo,
                bus,
                &event_type,
                &payload,
                &dead,
                event_uuid,
            ) {
                let dead_names: Vec<_> = dead
                    .iter()
                    .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string))
                    .collect();
                return merge_sweep(
                    base,
                    json!({
                        "status": "tracker-stamp-unreconciled",
                        "purged": false,
                        "event_type": event_type,
                        "dead_letter_witnesses": dead_names,
                    }),
                );
            }
            let finalized = finalize_kaizen_terminal(
                repo,
                bus,
                event_uuid,
                &pending_path,
                registry_ref,
                &event_type,
                &origin,
            );
            let dead_names: Vec<_> = dead
                .iter()
                .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string))
                .collect();
            return merge_sweep(
                base,
                json!({
                    "status": "kaizen-finalized",
                    "purged": true,
                    "finalized": true,
                    "event_type": event_type,
                    "dead_letter_witnesses": dead_names,
                    "pending": finalized.get("pending"),
                    "headers": finalized.get("headers"),
                }),
            );
        }
        let dead_names: Vec<_> = dead
            .iter()
            .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string))
            .collect();
        return merge_sweep(
            base,
            json!({
                "status": "kaizen",
                "event_type": event_type,
                "dead_letter_witnesses": dead_names,
            }),
        );
    }

    if applicable.is_empty() {
        let archived = archive_event_after_sweep(repo, bus, event_uuid, Some(&event_type));
        return merge_sweep(
            base,
            json!({
                "status": "purged",
                "purged": true,
                "event_type": event_type,
                "witnesses": archived.get("witnesses"),
                "headers": archived.get("headers"),
                "pending": archived.get("pending"),
            }),
        );
    }

    let in_flight = in_flight_subscriber_names(repo, bus, event_uuid);
    let applicable_set: HashSet<_> = applicable.iter().cloned().collect();
    let overlap: Vec<_> = applicable_set.intersection(&in_flight).cloned().collect();
    if !overlap.is_empty() {
        return merge_sweep(
            base,
            json!({
                "status": "in-flight",
                "event_type": event_type,
                "in_flight": overlap,
            }),
        );
    }

    let done = processed_subscriber_names(repo, bus, event_uuid);
    if applicable_set.is_subset(&done) {
        let archived = archive_event_after_sweep(repo, bus, event_uuid, Some(&event_type));
        return merge_sweep(
            base,
            json!({
                "status": "purged",
                "purged": true,
                "event_type": event_type,
                "witnesses": archived.get("witnesses"),
                "headers": archived.get("headers"),
                "pending": archived.get("pending"),
            }),
        );
    }

    let pending_subscribers: Vec<_> = applicable_set.difference(&done).cloned().collect();
    merge_sweep(
        base,
        json!({
            "status": "awaiting",
            "event_type": event_type,
            "pending_subscribers": pending_subscribers,
        }),
    )
}

pub fn rel_event_path(repo: &Path, event_path: &Path) -> String {
    event_path
        .canonicalize()
        .ok()
        .and_then(|p| {
            p.strip_prefix(repo.canonicalize().ok()?)
                .ok()
                .map(|r| r.to_string_lossy().replace('\\', "/"))
        })
        .unwrap_or_else(|| event_path.to_string_lossy().replace('\\', "/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_minimal_paths(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"directories":{"documentation":"docs"}}"#,
        )
        .unwrap();
    }

    fn fs_lab_tempdir() -> (
        crate::engine::eda_port::test_lab::FilesystemEdaGuard,
        tempfile::TempDir,
    ) {
        crate::engine::eda_port::test_lab::fs_lab_tempdir()
    }

    fn seed_pending_ecst(repo: &Path, event_id: &str, event_type: &str, payload: Value) {
        let mut event = json!({
            "event_id": event_id,
            "event_type": event_type,
            "payload": payload,
        });
        sddia_eda_catalog::stamp_published_event(repo, &mut event).unwrap();
        crate::engine::eda_port::record_ecst(repo, "pending", &event).unwrap();
    }

    fn seed_subscriber_witness(repo: &Path, event_id: &str, subscriber: &str, success: bool) {
        if success {
            crate::engine::eda_port::record_delivery_ack(repo, event_id, subscriber).unwrap();
        } else {
            crate::engine::eda_port::record_delivery_nack(repo, event_id, subscriber).unwrap();
        }
    }

    #[test]
    fn promote_witness_does_not_materialize_subscriber_json() {
        use crate::engine::eda_port::test_harness::EventManagerTestHarness;
        use sddia_core::ports::event_manager::{publish_request_domain, EventManagerPort};

        let h = EventManagerTestHarness::new();
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        write_minimal_paths(repo);
        let bus = ensure_event_bus_topology(repo).unwrap();
        let event_id = "evt-o2-promote-witness";
        h.mock()
            .publish_ecst(publish_request_domain(
                event_id,
                json!({"event_type": "T"}),
                None,
            ))
            .unwrap();
        let fs_path = resolve_bus_path(repo, &bus.processed_subscribers)
            .join(format!("{event_id}.sub-a.json"));
        assert!(!fs_path.is_file());
        promote_witness(
            repo,
            &bus,
            event_id,
            "sub-a",
            "processed",
            Some(&json!({"result_status": "success"})),
            None,
        )
        .unwrap();
        assert!(!fs_path.is_file(), "no debe existir testigo FS de suscriptor");
        assert!(terminal_witness_exists(repo, &bus, event_id, "sub-a"));
    }

    #[test]
    fn adu_ca5_docs_prefix_features_then_fixes() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        write_minimal_paths(repo);
        fs::create_dir_all(repo.join("docs/features/merge-thermo-hu-f2-close")).unwrap();
        assert_eq!(
            infer_persist_ref_from_branch(repo, "docs/merge-thermo-hu-f2-close").as_deref(),
            Some("docs/features/merge-thermo-hu-f2-close")
        );

        let tmp2 = tempfile::tempdir().unwrap();
        let repo2 = tmp2.path();
        write_minimal_paths(repo2);
        fs::create_dir_all(repo2.join("docs/fixes/kaizen-pr-aduana-friction-20261012")).unwrap();
        assert_eq!(
            infer_persist_ref_from_branch(repo2, "docs/kaizen-pr-aduana-friction-20261012")
                .as_deref(),
            Some("docs/fixes/kaizen-pr-aduana-friction-20261012")
        );

        let tmp3 = tempfile::tempdir().unwrap();
        let repo3 = tmp3.path();
        write_minimal_paths(repo3);
        assert!(infer_persist_ref_from_branch(repo3, "docs/missing-slug").is_none());
    }

    #[test]
    fn adu_ca6_feat_fix_refactor_unchanged() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        write_minimal_paths(repo);
        fs::create_dir_all(repo.join("docs/features/alpha")).unwrap();
        fs::create_dir_all(repo.join("docs/fixes/beta")).unwrap();
        assert_eq!(
            infer_persist_ref_from_branch(repo, "feat/alpha").as_deref(),
            Some("docs/features/alpha")
        );
        assert_eq!(
            infer_persist_ref_from_branch(repo, "refactor/alpha").as_deref(),
            Some("docs/features/alpha")
        );
        assert_eq!(
            infer_persist_ref_from_branch(repo, "fix/beta").as_deref(),
            Some("docs/fixes/beta")
        );
    }

    #[test]
    fn aut70_fracture_dead_letter_not_kaizen_finalized() {
        let (_fs, tmp) = fs_lab_tempdir();
        let repo = tmp.path();
        write_minimal_paths(repo);
        let event_id = "aaaaaaaa-bbbb-4ccc-8ddd-000000000001";
        let bus = ensure_event_bus_topology(repo).unwrap();
        seed_pending_ecst(
            repo,
            event_id,
            "System_Fracture_Detected",
            json!({ "origin_topology": "core" }),
        );
        seed_subscriber_witness(repo, event_id, "cumulo.materialize-fracture-pbi", false);
        seed_subscriber_witness(repo, event_id, "argos.send-telegram-notification", true);
        // ACK terminal puede expulsar el ECST del bucket pending; restaurar cabecera para el barrido.
        seed_pending_ecst(
            repo,
            event_id,
            "System_Fracture_Detected",
            json!({ "origin_topology": "core" }),
        );
        let registry = json!({
            "System_Fracture_Detected": [
                {"agent": "cumulo", "action": "materialize-fracture-pbi"},
                {"agent": "argos", "tool": "send-telegram-notification"}
            ]
        });
        let sweep = try_sweep_event(repo, &bus, event_id, Some(&registry));
        assert_eq!(
            sweep.get("status").and_then(|v| v.as_str()),
            Some("fracture-unmaterialized")
        );
        assert_eq!(sweep.get("purged").and_then(|v| v.as_bool()), Some(false));
        assert!(crate::engine::eda_port::test_lab::event_exists(repo, event_id));
    }

    fn write_pr_merged_pending(repo: &Path, _bus: &EventBusTopology, event_id: &str, tracker_ref: &str) {
        seed_pending_ecst(
            repo,
            event_id,
            "PullRequest_Merged",
            json!({
                "origin_topology": "core",
                "tracker_ref": tracker_ref,
                "merge_commit_hash": "abc123"
            }),
        );
    }

    fn pr_merged_registry() -> Value {
        json!({
            "PullRequest_Merged": [
                {"agent": "tekton", "process": "tracker-stamp"},
                {"agent": "cumulo", "tool": "iota-immutable-publisher"},
                {"agent": "argos", "action": "notify-humanized-pr-merged"}
            ]
        })
    }

    #[test]
    fn aut72_tracker_stamp_dead_letter_not_kaizen_finalized() {
        let (_fs, tmp) = fs_lab_tempdir();
        let repo = tmp.path();
        write_minimal_paths(repo);
        let event_id = "bbbbbbbb-bbbb-4ccc-8ddd-000000000002";
        let bus = ensure_event_bus_topology(repo).unwrap();
        write_pr_merged_pending(repo, &bus, event_id, "AUT-72");
        seed_subscriber_witness(repo, event_id, TRACKER_STAMP_SUBSCRIBER, false);
        for sub in ["cumulo.iota-immutable-publisher", "argos.notify-humanized-pr-merged"] {
            seed_subscriber_witness(repo, event_id, sub, true);
        }
        seed_pending_ecst(
            repo,
            event_id,
            "PullRequest_Merged",
            json!({
                "origin_topology": "core",
                "tracker_ref": "AUT-72",
                "merge_commit_hash": "abc123"
            }),
        );
        let sweep = try_sweep_event(repo, &bus, event_id, Some(&pr_merged_registry()));
        assert_eq!(
            sweep.get("status").and_then(|v| v.as_str()),
            Some("tracker-stamp-unreconciled")
        );
        assert_eq!(sweep.get("purged").and_then(|v| v.as_bool()), Some(false));
        assert!(crate::engine::eda_port::test_lab::event_exists(repo, event_id));
    }

    #[test]
    fn aut72_tracker_stamp_dead_letter_kaizen_finalized_with_proof() {
        let (_fs, tmp) = fs_lab_tempdir();
        let repo = tmp.path();
        write_minimal_paths(repo);
        let event_id = "cccccccc-bbbb-4ccc-8ddd-000000000003";
        let bus = ensure_event_bus_topology(repo).unwrap();
        write_pr_merged_pending(repo, &bus, event_id, "AUT-72");
        seed_subscriber_witness(repo, event_id, TRACKER_STAMP_SUBSCRIBER, false);
        for sub in ["cumulo.iota-immutable-publisher", "argos.notify-humanized-pr-merged"] {
            seed_subscriber_witness(repo, event_id, sub, true);
        }
        seed_pending_ecst(
            repo,
            event_id,
            "PullRequest_Merged",
            json!({
                "origin_topology": "core",
                "tracker_ref": "AUT-72",
                "merge_commit_hash": "abc123"
            }),
        );
        crate::engine::done_gate::write_tracker_done_proof(
            repo,
            "AUT-72",
            "abc123",
            &json!({"state": "Done"}),
        )
        .unwrap();
        let sweep = try_sweep_event(repo, &bus, event_id, Some(&pr_merged_registry()));
        assert_eq!(
            sweep.get("status").and_then(|v| v.as_str()),
            Some("kaizen-finalized")
        );
        assert_eq!(sweep.get("purged").and_then(|v| v.as_bool()), Some(true));
        assert!(!crate::engine::eda_port::test_lab::event_exists(repo, event_id));
    }
}
