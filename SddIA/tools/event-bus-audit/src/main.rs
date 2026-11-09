//! FS-legacy helpers retained for `#[cfg(test)]`; store path is default in release.
#![cfg_attr(not(test), allow(dead_code, unused_variables))]

use chrono::{DateTime, Utc};
use regex::Regex;
use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
struct Anomaly {
    kind: String,
    path: String,
    detail: String,
}

#[derive(Debug, Serialize)]
struct AuditSummary {
    counts: BTreeMap<String, u64>,
    counts_by_type: BTreeMap<String, u64>,
    stale_pending_count: u64,
    orphan_witness_count: u64,
    structural_error_count: u64,
    dead_letter_count: u64,
    dead_letter_witness_count: u64,
    non_ecst_sink_count: u64,
    actionable_stale_pending_count: u64,
    fracture_stale_pending_count: u64,
}

#[derive(Debug)]
struct BusPaths {
    pending: PathBuf,
    processing: PathBuf,
    processed: PathBuf,
    dead_letter: PathBuf,
    processing_subscribers: PathBuf,
    processed_subscribers: PathBuf,
    dead_letter_subscribers: PathBuf,
    telemetry: PathBuf,
    orchestration: PathBuf,
    domain: PathBuf,
}

#[derive(Debug, Default)]
struct ScanState {
    counts: BTreeMap<String, u64>,
    counts_by_type: BTreeMap<String, u64>,
    anomalies: Vec<Anomaly>,
    dead_letter_errors: BTreeMap<String, u64>,
    stale_pending: Vec<String>,
    event_ids: HashSet<String>,
    dlt_event_ids: HashSet<String>,
    witness_parent_ids: HashSet<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AuditFamily {
    Domain,
    Orchestration,
    Telemetry,
}

#[derive(Clone)]
struct AuditEventRow {
    event_id: String,
    event_type: String,
    ecst: Value,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AuditDeliveryStatus {
    Success,
    Failed,
}

fn get_repo_root() -> Result<PathBuf, String> {
    let mut current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current dir: {e}"))?;
    loop {
        if current_dir.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(current_dir);
        }
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            return Err("No se encontró raíz del workspace".into());
        }
    }
}

fn resolve_bus_paths(repo: &Path) -> Result<BusPaths, String> {
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let text = fs::read_to_string(&cfg_path).map_err(|e| e.to_string())?;
    let cfg: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let rel = |section: &str, key: &str, default: &str| -> PathBuf {
        cfg.get(section)
            .and_then(|s| s.get(key))
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .trim()
            .trim_start_matches("./")
            .into()
    };

    let pending = repo.join(rel("eda_bus", "pending", ".events/pending"));
    let processing = repo.join(rel("eda_bus", "processing", ".events/processing"));
    let processed = repo.join(rel("eda_bus", "processed", ".events/processed"));
    let dead_letter = repo.join(rel("eda_bus", "dead_letter", ".events/dead-letter"));

    Ok(BusPaths {
        processing_subscribers: processing.join("subscribers"),
        processed_subscribers: processed.join("subscribers"),
        dead_letter_subscribers: dead_letter.join("subscribers"),
        telemetry: repo.join(
            cfg.get("eda_fractal")
                .and_then(|f| f.get("telemetry"))
                .and_then(|v| v.as_str())
                .unwrap_or("./.events/telemetry")
                .trim()
                .trim_start_matches("./"),
        ),
        orchestration: repo.join(
            cfg.get("eda_fractal")
                .and_then(|f| f.get("orchestration"))
                .and_then(|v| v.as_str())
                .unwrap_or("./.events/orchestration")
                .trim()
                .trim_start_matches("./"),
        ),
        domain: repo.join(
            cfg.get("eda_fractal")
                .and_then(|f| f.get("domain"))
                .and_then(|v| v.as_str())
                .unwrap_or("./.events/domain")
                .trim()
                .trim_start_matches("./"),
        ),
        pending,
        processing,
        processed,
        dead_letter,
    })
}

fn load_catalog(repo: &Path) -> HashMap<String, String> {
    let mut catalog = HashMap::new();
    let events_root = repo.join("SddIA/events");
    if !events_root.is_dir() {
        return catalog;
    }
    let re = Regex::new(r#"event_type:\s*"?([^"\n]+)"?"#).expect("regex");
    for family in ["telemetry", "orchestration", "domain"] {
        let dir = events_root.join(family);
        if !dir.is_dir() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("md") {
                    continue;
                }
                if path.file_name().and_then(|n| n.to_str()) == Some("index.md") {
                    continue;
                }
                if let Ok(text) = fs::read_to_string(&path) {
                    if let Some(fm) = text.split("---").nth(1) {
                        if let Some(cap) = re.captures(fm) {
                            catalog.insert(cap[1].trim().to_string(), family.to_string());
                        }
                    }
                }
            }
        }
    }
    catalog
}

fn load_known_event_types(repo: &Path) -> HashSet<String> {
    load_catalog(repo).into_keys().collect()
}

fn load_registry_counts(path: &Path) -> HashMap<String, usize> {
    let mut out = HashMap::new();
    let Ok(text) = fs::read_to_string(path) else {
        return out;
    };
    let Ok(val) = serde_json::from_str::<Value>(&text) else {
        return out;
    };
    let Some(obj) = val.as_object() else {
        return out;
    };
    for (k, v) in obj {
        let n = v
            .as_array()
            .map(|a| a.iter().filter(|s| s.is_object()).count())
            .unwrap_or(0);
        out.insert(k.clone(), n);
    }
    out
}

const PURGE_AFTER_FAMILIES: &[&str] = &["orchestration", "domain"];

fn audit_subscription_coverage(repo: &Path, state: &mut ScanState) {
    let catalog = load_catalog(repo);
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let cfg: Value = fs::read_to_string(&cfg_path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or(json!({}));
    let fractal = cfg.get("eda_fractal");
    let rel_for = |key: &str, default: &str| -> String {
        fractal
            .and_then(|f| f.get(key))
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .trim()
            .trim_start_matches("./")
            .replace('\\', "/")
    };
    let regs = [
        (
            "telemetry",
            rel_for(
                "telemetry_subscriptions",
                "SddIA/core/event-telemetry-subscriptions.json",
            ),
        ),
        (
            "orchestration",
            rel_for(
                "orchestration_subscriptions",
                "SddIA/core/event-orchestration-subscriptions.json",
            ),
        ),
        (
            "domain",
            rel_for(
                "domain_subscriptions",
                "SddIA/core/event-domain-subscriptions.json",
            ),
        ),
    ];
    let mut by_family: HashMap<&str, (String, HashMap<String, usize>)> = HashMap::new();
    for (family, rel) in &regs {
        let counts = load_registry_counts(&repo.join(rel));
        by_family.insert(*family, (rel.clone(), counts));
    }

    for (event_type, family) in &catalog {
        let (rel, counts) = match by_family.get(family.as_str()) {
            Some(v) => v,
            None => continue,
        };
        let n = counts.get(event_type).copied();
        if n.unwrap_or(0) == 0 {
            state.anomalies.push(Anomaly {
                kind: "EMPTY_SUBSCRIBERS".into(),
                path: rel.clone(),
                detail: event_type.clone(),
            });
            if PURGE_AFTER_FAMILIES.contains(&family.as_str()) {
                state.anomalies.push(Anomaly {
                    kind: "PURGE_BLACKHOLE".into(),
                    path: rel.clone(),
                    detail: format!("{event_type} family={family} purge_after=true"),
                });
            }
        }
    }

    for (family, (rel, counts)) in &by_family {
        for key in counts.keys() {
            match catalog.get(key) {
                None => state.anomalies.push(Anomaly {
                    kind: "ORPHAN_REGISTRY_KEY".into(),
                    path: rel.clone(),
                    detail: key.clone(),
                }),
                Some(cat_fam) if cat_fam != *family => state.anomalies.push(Anomaly {
                    kind: "FAMILY_MISMATCH".into(),
                    path: rel.clone(),
                    detail: format!("{key} registry={family} class_family={cat_fam}"),
                }),
                Some(_) => {}
            }
        }
    }
}

fn utf8_prefix(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn rel_path(repo: &Path, path: &Path) -> String {
    path.strip_prefix(repo)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn parse_timestamp(raw: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|| {
            chrono::NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%SZ")
                .ok()
                .map(|ndt| ndt.and_utc())
        })
}

fn is_non_ecst_sink(value: &Value) -> bool {
    value.get("event_id").and_then(|v| v.as_str()).is_none()
        && value.get("event_type").and_then(|v| v.as_str()).is_none()
        && value.get("emitter_agent").and_then(|v| v.as_str()).is_none()
}

fn is_fracture_stale_detail(detail: &str) -> bool {
    detail.contains("event_type=System_Fracture_Detected;")
}

fn needs_kaizen_actionable(circuit_alert: bool, actionable_stale_pending_count: u64) -> bool {
    circuit_alert || actionable_stale_pending_count > 0
}

fn validate_ecst_event(
    repo: &Path,
    bucket: &str,
    path: &Path,
    value: &Value,
    known_types: &HashSet<String>,
    state: &mut ScanState,
    track_dlt: bool,
) {
    let rel = rel_path(repo, path);
    state.counts.entry(bucket.to_string()).or_insert(0);
    *state.counts.get_mut(bucket).unwrap() += 1;

    if is_non_ecst_sink(value) {
        state.anomalies.push(Anomaly {
            kind: "non_ecst_sink".into(),
            path: rel,
            detail: "missing event_id, event_type, emitter_agent".into(),
        });
        return;
    }

    let mut structural_errors = Vec::new();

    let event_id = value.get("event_id").and_then(|v| v.as_str());
    let event_type = value.get("event_type").and_then(|v| v.as_str());
    let timestamp = value.get("timestamp").and_then(|v| v.as_str());
    let emitter = value.get("emitter_agent").and_then(|v| v.as_str());

    if event_id.is_none() {
        structural_errors.push("missing event_id");
    }
    if event_type.is_none() {
        structural_errors.push("missing event_type");
    }
    if timestamp.is_none() {
        structural_errors.push("missing timestamp");
    }
    if emitter.is_none() {
        structural_errors.push("missing emitter_agent");
    }

    if let (Some(eid), Some(stem)) = (event_id, path.file_stem().and_then(|s| s.to_str())) {
        if stem != eid {
            structural_errors.push("event_id mismatch filename");
        }
        state.event_ids.insert(eid.to_string());
        if track_dlt {
            state.dlt_event_ids.insert(eid.to_string());
        }
    }

    if let Some(et) = event_type {
        state
            .counts_by_type
            .entry(et.to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
        if !known_types.is_empty() && !known_types.contains(et) {
            structural_errors.push("unknown event_type (no Clase ECST)");
        }
    }

    if !structural_errors.is_empty() {
        state.anomalies.push(Anomaly {
            kind: "structural".into(),
            path: rel.clone(),
            detail: structural_errors.join("; "),
        });
    }
}

fn scan_event_dir(
    repo: &Path,
    dir: &Path,
    bucket: &str,
    known_types: &HashSet<String>,
    state: &mut ScanState,
    track_dlt: bool,
    stale_threshold_hours: i64,
    stale_pending: &mut Vec<String>,
) {
    if !dir.is_dir() {
        return;
    }
    let now = Utc::now();
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let rel = rel_path(repo, &path);
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                state.anomalies.push(Anomaly {
                    kind: "io".into(),
                    path: rel,
                    detail: format!("read error: {e}"),
                });
                continue;
            }
        };
        let value: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                state.anomalies.push(Anomaly {
                    kind: "parse".into(),
                    path: rel,
                    detail: format!("invalid JSON: {e}"),
                });
                continue;
            }
        };
        validate_ecst_event(repo, bucket, &path, &value, known_types, state, track_dlt);

        if bucket == "pending" {
            if let Some(ts) = value.get("timestamp").and_then(|v| v.as_str()) {
                if let Some(parsed) = parse_timestamp(ts) {
                    let age_hours = (now - parsed).num_hours();
                    if age_hours > stale_threshold_hours {
                        if let Some(eid) = value.get("event_id").and_then(|v| v.as_str()) {
                            let et = value
                                .get("event_type")
                                .and_then(|v| v.as_str())
                                .unwrap_or("?");
                            stale_pending.push(format!("{eid} ({age_hours}h) {et}"));
                            state.anomalies.push(Anomaly {
                                kind: "stale_pending".into(),
                                path: rel_path(repo, &path),
                                detail: format!(
                                    "event_type={et}; pending age {age_hours}h > threshold {stale_threshold_hours}h"
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
}

fn scan_witness_dir(
    repo: &Path,
    dir: &Path,
    bucket: &str,
    state: &mut ScanState,
) {
    if !dir.is_dir() {
        return;
    }
    let witness_re =
        Regex::new(r"^([0-9a-fA-F-]{36})\.(.+)\.json$").expect("witness regex");
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(fname) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if !fname.ends_with(".json") {
            continue;
        }
        let rel = rel_path(repo, &path);
        state.counts.entry(bucket.to_string()).or_insert(0);
        *state.counts.get_mut(bucket).unwrap() += 1;

        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                state.anomalies.push(Anomaly {
                    kind: "io".into(),
                    path: rel.clone(),
                    detail: format!("read error: {e}"),
                });
                continue;
            }
        };
        let value: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                state.anomalies.push(Anomaly {
                    kind: "parse".into(),
                    path: rel.clone(),
                    detail: format!("invalid JSON: {e}"),
                });
                continue;
            }
        };

        let parent_id = value
            .get("event_uuid")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .or_else(|| {
                witness_re
                    .captures(fname)
                    .map(|c| c[1].to_string())
            });

        if let Some(pid) = parent_id {
            state.witness_parent_ids.insert(pid.clone());
            if !state.dlt_event_ids.contains(&pid) {
                state.anomalies.push(Anomaly {
                    kind: "orphan_witness".into(),
                    path: rel.clone(),
                    detail: format!("witness parent {pid} not found in DLT headers"),
                });
            }
        } else {
            state.anomalies.push(Anomaly {
                kind: "structural".into(),
                path: rel.clone(),
                detail: "witness missing event_uuid".into(),
            });
        }

        if bucket == "dead-letter/subscribers" {
            if let Some(err) = value
                .get("error_trace")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
            {
                *state
                    .dead_letter_errors
                    .entry(err.to_string())
                    .or_insert(0) += 1;
            }
        }
    }
}

const CORE_DOMAIN_REGISTRY: &str = "SddIA/core/event-domain-subscriptions.json";
const CODEX_REGISTRY: &str = "SddIA/library/codexes/codex-software-engineering/subscriptions.json";

fn software_codex_authority(repo: &Path) -> bool {
    let Ok(text) = fs::read_to_string(repo.join(".SddIA/project.md")) else {
        return false;
    };
    text.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("codex_slug:") && trimmed.contains("codex-software-engineering")
    })
}

fn load_json_file(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(text.trim_start_matches('\u{feff}')).map_err(|e| e.to_string())
}

fn load_domain_subscription_registries(
    repo: &Path,
) -> Result<(Value, Option<Value>, bool), String> {
    let core = load_json_file(&repo.join(CORE_DOMAIN_REGISTRY))?;
    let authority = software_codex_authority(repo);
    let codex = if authority {
        let path = repo.join(CODEX_REGISTRY);
        if path.is_file() {
            Some(load_json_file(&path)?)
        } else {
            None
        }
    } else {
        None
    };
    Ok((core, codex, authority))
}

fn agent_llm_tier(repo: &Path, agent: &str) -> Option<String> {
    let path = repo.join("SddIA/agents").join(format!("{agent}.md"));
    let text = fs::read_to_string(&path).ok()?;
    let fm = text.split("---").nth(1)?;
    let re = Regex::new(r"(?m)^\s*tier:\s*([a-z]+)\s*$").ok()?;
    re.captures(fm).map(|c| c[1].to_string())
}

fn tier_is_cognitive(tier: &str) -> bool {
    matches!(tier, "high" | "medium" | "low")
}

fn split_subscriber_id(id: &str) -> (&str, Option<&str>) {
    match id.split_once('.') {
        Some((agent, capsule)) if !capsule.is_empty() => (agent, Some(capsule)),
        _ => (id, None),
    }
}

fn eda_dispatch_workspace_empty(repo: &Path, event_id: &str, agent: &str) -> bool {
    let dir = repo
        .join(".SddIA/workspaces/eda-dispatch")
        .join(event_id)
        .join(agent);
    if !dir.is_dir() {
        return true;
    }
    fs::read_dir(&dir)
        .map(|mut e| e.next().is_none())
        .unwrap_or(true)
}

fn tracker_ref_from_ecst(ecst: &Value) -> Option<&str> {
    ecst.pointer("/payload/tracker_ref")
        .or_else(|| ecst.get("tracker_ref"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

fn tracker_outbound_exists(repo: &Path, tracker_ref: &str) -> bool {
    repo.join(".SddIA/state/tracker-outbound")
        .join(format!("{tracker_ref}.jsonl"))
        .is_file()
}

fn collect_residual_fractures(rows: &[AuditEventRow]) -> Vec<(String, String)> {
    rows
        .iter()
        .filter(|r| r.event_type == "System_Fracture_Detected")
        .filter_map(|r| {
            let friction = r
                .ecst
                .pointer("/payload/friction_id")
                .and_then(|v| v.as_str());
            if friction != Some("F-EDA-RESIDUAL-FILESYSTEM") {
                return None;
            }
            let trace = r
                .ecst
                .pointer("/payload/error_trace")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            Some((r.event_id.clone(), trace))
        })
        .collect()
}

fn audit_residual_pending_files(
    repo: &Path,
    pending: &Path,
    fractures: &[(String, String)],
    state: &mut ScanState,
) {
    if !pending.is_dir() {
        return;
    }
    let Ok(entries) = fs::read_dir(pending) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let rel = rel_path(repo, &path);
        let fname = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let mut detail = rel.clone();
        for (event_id, trace) in fractures {
            if trace.contains(fname) || trace.contains(&rel) {
                detail = format!(
                    "{rel}; fracture_event_id={event_id}; friction_id=F-EDA-RESIDUAL-FILESYSTEM"
                );
                break;
            }
        }
        state.anomalies.push(Anomaly {
            kind: "residual_filesystem".into(),
            path: rel.clone(),
            detail,
        });
    }
}

fn audit_domain_delivery_signals(
    repo: &Path,
    domain_rows: &[AuditEventRow],
    deliveries: &[(String, String, AuditDeliveryStatus)],
    state: &mut ScanState,
) {
    let Ok((core, codex, authority)) = load_domain_subscription_registries(repo) else {
        return;
    };
    let mut by_event: HashMap<String, Vec<(String, AuditDeliveryStatus)>> = HashMap::new();
    for (event_id, subscriber_id, status) in deliveries {
        if subscriber_id.starts_with('_') || subscriber_id.trim().is_empty() {
            continue;
        }
        by_event
            .entry(event_id.clone())
            .or_default()
            .push((subscriber_id.clone(), *status));
    }

    for row in domain_rows {
        let payload = row.ecst.get("payload").unwrap_or(&row.ecst);
        let origin = sddia_eda_catalog::resolve_origin_topology(payload);
        let plan =
            sddia_eda_catalog::plan_route(&core, codex.as_ref(), authority, &row.event_type);
        let expected: HashSet<String> = plan
            .subscribers
            .iter()
            .filter(|s| sddia_eda_catalog::subscriber_applies_to_topology(s, &origin))
            .map(sddia_eda_catalog::subscriber_id)
            .collect();

        let actual_rows = by_event.get(&row.event_id).cloned().unwrap_or_default();
        let actual: HashSet<String> = actual_rows.iter().map(|(sid, _)| sid.clone()).collect();

        for sid in actual.difference(&expected) {
            state.anomalies.push(Anomaly {
                kind: "delivery_fuera_de_catalogo".into(),
                path: row.event_id.clone(),
                detail: sid.clone(),
            });
        }
        for sid in expected.difference(&actual) {
            state.anomalies.push(Anomaly {
                kind: "suscriptor_sin_fila".into(),
                path: row.event_id.clone(),
                detail: sid.clone(),
            });
        }

        for (subscriber_id, status) in &actual_rows {
            if *status != AuditDeliveryStatus::Success {
                continue;
            }
            let (agent, capsule) = split_subscriber_id(subscriber_id);
            let Some(tier) = agent_llm_tier(repo, agent) else {
                continue;
            };
            if !tier_is_cognitive(&tier) {
                continue;
            }
            if !eda_dispatch_workspace_empty(repo, &row.event_id, agent) {
                continue;
            }
            if capsule == Some("tracker-stamp") {
                if let Some(tr) = tracker_ref_from_ecst(&row.ecst) {
                    if tracker_outbound_exists(repo, tr) {
                        continue;
                    }
                }
            }
            state.anomalies.push(Anomaly {
                kind: "success_sin_efecto".into(),
                path: row.event_id.clone(),
                detail: format!("{subscriber_id}; tier={tier}"),
            });
        }
    }
}

fn ingest_audit_event(
    state: &mut ScanState,
    buckets: &mut BTreeMap<&str, u64>,
    domain_rows: &mut Vec<AuditEventRow>,
    domain_ids: &mut HashSet<String>,
    event_id: String,
    ecst: Value,
) {
    let family = store_family_for_ecst(&ecst);
    let bucket = match family {
        AuditFamily::Orchestration => "orchestration",
        AuditFamily::Telemetry => "telemetry",
        AuditFamily::Domain => "domain",
    };
    *buckets.entry(bucket).or_insert(0) += 1;
    let event_type = ecst
        .get("event_type")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    *state.counts_by_type.entry(event_type.clone()).or_insert(0) += 1;
    state.event_ids.insert(event_id.clone());
    if family == AuditFamily::Domain && domain_ids.insert(event_id.clone()) {
        domain_rows.push(AuditEventRow {
            event_id,
            event_type,
            ecst,
        });
    }
}

fn store_family_for_ecst(ecst: &Value) -> AuditFamily {
    let t = ecst
        .get("event_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if t.contains("Process_Execution") || t.starts_with("Orchestration") {
        AuditFamily::Orchestration
    } else if t.contains("Telemetry") || ecst.pointer("/payload/telemetry_receipt").is_some() {
        AuditFamily::Telemetry
    } else {
        AuditFamily::Domain
    }
}

fn audit_state_from_store(repo: &Path, bus_paths: &BusPaths) -> ScanState {
    use sddia_core::ports::event_manager::{
        DeliveryStatus as PortDeliveryStatus, GetEventRequest, ListDeliveriesRequest,
        ListDlqRequest, ListPendingRequest,
    };
    use sddia_event_manager_host::{productive_store_present, with_productive_port};

    if !productive_store_present(repo) {
        return ScanState::default();
    }
    match with_productive_port(repo, "audit_scan", |port| {
        let mut state = ScanState::default();
        let mut domain_rows = Vec::new();
    let mut domain_ids = HashSet::new();
    let mut buckets = BTreeMap::<&str, u64>::new();

    if let Ok(listed) = port.list_pending(ListPendingRequest {
        limit: Some(10_000),
    }) {
        for e in listed.events {
            ingest_audit_event(
                &mut state,
                &mut buckets,
                &mut domain_rows,
                &mut domain_ids,
                e.event_id,
                e.ecst,
            );
        }
    }

    if let Ok(dlq) = port.list_dlq(ListDlqRequest {
        limit: Some(10_000),
    }) {
        state
            .counts
            .insert("dead-letter".into(), dlq.events.len() as u64);
        for e in dlq.events {
            *state
                .dead_letter_errors
                .entry(e.reason)
                .or_insert(0) += 1;
            ingest_audit_event(
                &mut state,
                &mut buckets,
                &mut domain_rows,
                &mut domain_ids,
                e.event_id,
                e.ecst,
            );
        }
    }

    let mut all_deliveries = Vec::new();
    if let Ok(rows) = port.list_deliveries(ListDeliveriesRequest {
        limit: Some(10_000),
        ..Default::default()
    }) {
        let mut failed = 0u64;
        let mut success = 0u64;
        for d in &rows.deliveries {
            let mapped = match d.status {
                PortDeliveryStatus::Ack => AuditDeliveryStatus::Success,
                PortDeliveryStatus::Nack => AuditDeliveryStatus::Failed,
            };
            match mapped {
                AuditDeliveryStatus::Failed => failed += 1,
                AuditDeliveryStatus::Success => success += 1,
            }
            all_deliveries.push((d.event_id.clone(), d.delivery_id.clone(), mapped));
        }
        state
            .counts
            .insert("dead-letter/subscribers".into(), failed);
        state.counts.insert("processed/subscribers".into(), success);
        for (event_id, _, _) in &all_deliveries {
            if domain_ids.contains(event_id) {
                continue;
            }
            if let Ok(ev) = port.get_event(GetEventRequest {
                event_id: event_id.clone(),
            }) {
                ingest_audit_event(
                    &mut state,
                    &mut buckets,
                    &mut domain_rows,
                    &mut domain_ids,
                    ev.event_id,
                    ev.ecst,
                );
            }
        }
    }
    for (k, v) in buckets {
        state.counts.insert(k.into(), v);
    }

        audit_domain_delivery_signals(repo, &domain_rows, &all_deliveries, &mut state);
        #[cfg(test)]
        {
            let fractures = collect_residual_fractures(&domain_rows);
            audit_residual_pending_files(repo, &bus_paths.pending, &fractures, &mut state);
        }

        Ok(state)
    }) {
        Ok(s) => s,
        Err(err) => {
            let mut state = ScanState::default();
            let path = match sddia_event_manager_host::resolved_redb_path(repo) {
                Ok(p) => rel_path(repo, &p),
                Err(_) => "(unresolved)".into(),
            };
            state.anomalies.push(Anomaly {
                kind: "store_unavailable".into(),
                path,
                detail: err,
            });
            state
        }
    }
}

fn run_audit(repo: &Path, stale_threshold_hours: i64) -> Result<(ScanState, BusPaths), String> {
    let paths = resolve_bus_paths(repo)?;
    if sddia_event_manager_host::productive_store_present(repo) {
        return Ok((audit_state_from_store(repo, &paths), paths));
    }
    #[cfg(not(test))]
    {
        let mut state = ScanState::default();
        let path = match sddia_event_manager_host::resolved_redb_path(repo) {
            Ok(p) => rel_path(repo, &p),
            Err(_) => "(unresolved)".into(),
        };
        state.anomalies.push(Anomaly {
            kind: "store_missing".into(),
            path,
            detail: "events.redb ausente; escaneo FS deshabilitado (cutover puerto)".into(),
        });
        return Ok((state, paths));
    }
    #[cfg(test)]
    {
    let known_types = load_known_event_types(repo);
    let mut state = ScanState::default();
    let mut stale_pending = Vec::new();

    scan_event_dir(
        repo,
        &paths.pending,
        "pending",
        &known_types,
        &mut state,
        true,
        stale_threshold_hours,
        &mut stale_pending,
    );
    scan_event_dir(
        repo,
        &paths.processing,
        "processing",
        &known_types,
        &mut state,
        true,
        stale_threshold_hours,
        &mut stale_pending,
    );
    scan_event_dir(
        repo,
        &paths.processed,
        "processed",
        &known_types,
        &mut state,
        true,
        stale_threshold_hours,
        &mut stale_pending,
    );
    scan_event_dir(
        repo,
        &paths.dead_letter,
        "dead-letter",
        &known_types,
        &mut state,
        true,
        stale_threshold_hours,
        &mut stale_pending,
    );
    scan_event_dir(
        repo,
        &paths.telemetry,
        "telemetry",
        &known_types,
        &mut state,
        false,
        stale_threshold_hours,
        &mut stale_pending,
    );
    scan_event_dir(
        repo,
        &paths.orchestration,
        "orchestration",
        &known_types,
        &mut state,
        false,
        stale_threshold_hours,
        &mut stale_pending,
    );
    scan_event_dir(
        repo,
        &paths.domain,
        "domain",
        &known_types,
        &mut state,
        false,
        stale_threshold_hours,
        &mut stale_pending,
    );

    scan_witness_dir(
        repo,
        &paths.processing_subscribers,
        "processing/subscribers",
        &mut state,
    );
    scan_witness_dir(
        repo,
        &paths.processed_subscribers,
        "processed/subscribers",
        &mut state,
    );
    scan_witness_dir(
        repo,
        &paths.dead_letter_subscribers,
        "dead-letter/subscribers",
        &mut state,
    );

    audit_subscription_coverage(repo, &mut state);

    state.stale_pending = stale_pending;
    Ok((state, paths))
    }
}

fn write_pending_kaizen(
    repo: &Path,
    _paths: &BusPaths,
    review_id: &str,
    justification: &str,
    implicated: &[String],
) -> Result<Value, String> {
    let event_id = Uuid::new_v4().to_string();
    let mut event = json!({
        "event_id": event_id,
        "event_type": "Kaizen_Alert_Required",
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": "event-bus-audit",
        "correlation_id": review_id,
        "payload": {
            "review_id": review_id,
            "alert_justification": justification,
            "implicated_files": implicated,
            "alert_kind": "event-bus-audit"
        },
        "delivery_state": {}
    });
    sddia_eda_catalog::stamp_published_event(repo, &mut event).map_err(|e| e.to_string())?;
    let seal = sddia_event_manager_host::with_productive_port(repo, "emit_kaizen", |port| {
        sddia_core::ports::event_manager::publish_ecst(port, repo, "domain", &event)
            .map_err(|e| e.to_string())
    })?;
    let target_path = seal.logical_rel;
    Ok(json!({
        "event_id": event_id,
        "target_path": target_path
    }))
}

fn build_report_md(
    summary: &AuditSummary,
    anomalies: &[Anomaly],
    dead_letter_errors: &BTreeMap<String, u64>,
    stale_pending: &[String],
) -> String {
    let mut lines = vec![
        "# Informe de auditoría — event-bus-audit".into(),
        String::new(),
        format!("Generado: {}", Utc::now().format("%Y-%m-%dT%H:%M:%SZ")),
        String::new(),
    ];
    if anomalies
        .iter()
        .any(|a| a.kind == "store_unavailable")
    {
        lines.push("## Store no disponible".into());
        lines.push(String::new());
        for a in anomalies
            .iter()
            .filter(|a| a.kind == "store_unavailable")
        {
            lines.push(format!("- `{}`: {}", a.path, a.detail));
        }
        lines.push(String::new());
        lines.push(
            "Los conteos siguientes no representan un bus vacío; la base no pudo abrirse.".into(),
        );
        lines.push(String::new());
    }
    lines.extend([
        "## Conteos por estado".into(),
        String::new(),
        "| Estado | Cantidad |".into(),
        "|--------|----------|".into(),
    ]);
    for (k, v) in &summary.counts {
        lines.push(format!("| `{k}` | {v} |"));
    }
    lines.push(String::new());
    lines.push("## Conteos por event_type".into());
    lines.push(String::new());
    lines.push("| event_type | Cantidad |".into());
    lines.push("|------------|----------|".into());
    for (k, v) in &summary.counts_by_type {
        lines.push(format!("| `{k}` | {v} |"));
    }
    lines.push(String::new());
    lines.push("## Resumen de anomalías".into());
    lines.push(String::new());
    lines.push(format!("- Pending estancados: {}", summary.stale_pending_count));
    lines.push(format!(
        "- Pending estancados accionables (no fractura): {}",
        summary.actionable_stale_pending_count
    ));
    lines.push(format!(
        "- Pending estancados System_Fracture_Detected: {}",
        summary.fracture_stale_pending_count
    ));
    lines.push(format!("- Testigos huérfanos: {}", summary.orphan_witness_count));
    lines.push(format!("- Errores estructurales: {}", summary.structural_error_count));
    lines.push(format!("- Sink no-ECST: {}", summary.non_ecst_sink_count));
    lines.push(format!("- Dead-letter (cabeceras): {}", summary.dead_letter_count));
    lines.push(format!(
        "- Dead-letter (testigos): {}",
        summary.dead_letter_witness_count
    ));

    if !stale_pending.is_empty() {
        lines.push(String::new());
        lines.push("## Pending estancados".into());
        for item in stale_pending.iter().take(20) {
            lines.push(format!("- {item}"));
        }
        if stale_pending.len() > 20 {
            lines.push(format!("- … y {} más", stale_pending.len() - 20));
        }
    }

    if !dead_letter_errors.is_empty() {
        lines.push(String::new());
        lines.push("## Top causas dead-letter".into());
        let mut ranked: Vec<_> = dead_letter_errors.iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(a.1));
        for (err, count) in ranked.into_iter().take(10) {
            let short = if err.len() > 120 {
                format!("{}…", utf8_prefix(err, 120))
            } else {
                err.clone()
            };
            lines.push(format!("- ({count}) {short}"));
        }
    }

    if !anomalies.is_empty() {
        lines.push(String::new());
        lines.push("## Detalle de anomalías (muestra)".into());
        for a in anomalies.iter().take(30) {
            lines.push(format!("- **[{}]** `{}` — {}", a.kind, a.path, a.detail));
        }
        if anomalies.len() > 30 {
            lines.push(format!("- … y {} más", anomalies.len() - 30));
        }
    }

    lines.join("\n")
}

fn bool_input(req: &Value, key: &str, default: bool) -> bool {
    req.get(key)
        .and_then(|v| {
            if let Some(b) = v.as_bool() {
                Some(b)
            } else if let Some(s) = v.as_str() {
                match s.to_lowercase().as_str() {
                    "1" | "true" | "yes" | "on" => Some(true),
                    "0" | "false" | "no" | "off" => Some(false),
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or(default)
}

fn num_input(req: &Value, key: &str, default: i64) -> i64 {
    req.get(key)
        .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(default)
        .max(1)
}

fn main() {
    let req = read_stdin_json();
    let stale_threshold_hours = num_input(&req, "stale_threshold_hours", 24);
    let emit_kaizen_alert = bool_input(&req, "emit_kaizen_alert", true);

    let repo = match get_repo_root() {
        Ok(r) => r,
        Err(e) => {
            emit_error(&e, 1);
            return;
        }
    };

    let (state, paths) = match run_audit(&repo, stale_threshold_hours) {
        Ok(v) => v,
        Err(e) => {
            emit_error(&e, 1);
            return;
        }
    };

    let orphan_witness_count = state
        .anomalies
        .iter()
        .filter(|a| a.kind == "orphan_witness")
        .count() as u64;
    let structural_error_count = state
        .anomalies
        .iter()
        .filter(|a| a.kind == "structural" || a.kind == "parse")
        .count() as u64;
    let non_ecst_sink_count = state
        .anomalies
        .iter()
        .filter(|a| a.kind == "non_ecst_sink")
        .count() as u64;
    let actionable_stale_pending_count = state
        .anomalies
        .iter()
        .filter(|a| a.kind == "stale_pending" && !is_fracture_stale_detail(&a.detail))
        .count() as u64;
    let fracture_stale_pending_count = state
        .anomalies
        .iter()
        .filter(|a| a.kind == "stale_pending" && is_fracture_stale_detail(&a.detail))
        .count() as u64;
    let circuit_alert = state.anomalies.iter().any(|a| {
        a.kind == "PURGE_BLACKHOLE"
            || (a.kind == "EMPTY_SUBSCRIBERS"
                && a.path.contains("orchestration"))
    });

    let summary = AuditSummary {
        stale_pending_count: state.stale_pending.len() as u64,
        orphan_witness_count,
        structural_error_count,
        dead_letter_count: *state.counts.get("dead-letter").unwrap_or(&0),
        dead_letter_witness_count: *state
            .counts
            .get("dead-letter/subscribers")
            .unwrap_or(&0),
        non_ecst_sink_count,
        actionable_stale_pending_count,
        fracture_stale_pending_count,
        counts: state.counts.clone(),
        counts_by_type: state.counts_by_type.clone(),
    };

    let workspace = req
        .get("workspace_path")
        .and_then(|v| v.as_str())
        .map(PathBuf::from);
    let report_path = workspace.as_ref().map(|ws| {
        if let Err(e) = fs::create_dir_all(ws) {
            eprintln!("warn: workspace create: {e}");
        }
        ws.join("audit-report.md")
    });

    let report_md = build_report_md(
        &summary,
        &state.anomalies,
        &state.dead_letter_errors,
        &state.stale_pending,
    );

    let mut report_rel = None;
    if let Some(ref rp) = report_path {
        if fs::write(rp, &report_md).is_ok() {
            report_rel = Some(rel_path(&repo, rp));
        }
    }

    let needs_kaizen =
        needs_kaizen_actionable(circuit_alert, summary.actionable_stale_pending_count);

    let mut kaizen_event_id = None;
    let mut kaizen_target = None;

    if emit_kaizen_alert && needs_kaizen {
        let review_id = Uuid::new_v4().to_string();
        let justification = format!(
            "Auditoría event-bus-audit: circuit_alert={} actionable_stale={} (total_stale={}, fracture_stale={}) non_ecst_sink={} structural={} dead-letter={}",
            circuit_alert,
            summary.actionable_stale_pending_count,
            summary.stale_pending_count,
            summary.fracture_stale_pending_count,
            summary.non_ecst_sink_count,
            structural_error_count,
            summary.dead_letter_count
        );
        let implicated: Vec<String> = state
            .anomalies
            .iter()
            .filter(|a| {
                a.kind == "PURGE_BLACKHOLE"
                    || (a.kind == "EMPTY_SUBSCRIBERS" && a.path.contains("orchestration"))
                    || (a.kind == "stale_pending" && !is_fracture_stale_detail(&a.detail))
            })
            .take(50)
            .map(|a| a.path.clone())
            .collect();
        if implicated.is_empty() {
            eprintln!("warn: needs_kaizen sin implicated_files; se omite emisión Kaizen");
        } else {
        match write_pending_kaizen(&repo, &paths, &review_id, &justification, &implicated) {
            Ok(seal) => {
                kaizen_event_id = seal.get("event_id").and_then(|v| v.as_str()).map(str::to_string);
                kaizen_target = seal.get("target_path").and_then(|v| v.as_str()).map(str::to_string);
            }
            Err(e) => {
                emit_error(&format!("Kaizen emit failed: {e}"), 1);
                return;
            }
        }
        }
    }

    emit_success(Some(json!({
        "audit_summary": summary,
        "anomalies": state.anomalies,
        "dead_letter_top_errors": state.dead_letter_errors,
        "stale_pending": state.stale_pending,
        "report_path": report_rel,
        "kaizen_event_id": kaizen_event_id,
        "kaizen_target_path": kaizen_target,
        "emit_kaizen_alert": emit_kaizen_alert,
        "needs_kaizen": needs_kaizen
    })));
}

#[cfg(test)]
mod tests {
    use super::*;
    use sddia_core::ports::event_manager::{
        DeliveryStatus as PortDeliveryStatus, GetEventRequest, MoveToDlqRequest,
        PublishEcstRequest, RecordDeliveryRequest,
    };
    fn port_publish(repo: &Path, event_id: &str, ecst: Value) {
        sddia_event_manager_host::with_productive_port(repo, "test_publish", |port| {
            port.publish_ecst(PublishEcstRequest {
                event_id: event_id.into(),
                ecst,
                trace_id: None,
            })
            .map_err(|e| e.to_string())
        })
        .expect("publish");
    }

    fn port_record_ack(repo: &Path, event_id: &str, subscriber_id: &str) {
        sddia_event_manager_host::with_productive_port(repo, "test_record_delivery", |port| {
            port.record_delivery(RecordDeliveryRequest {
                event_id: event_id.into(),
                delivery_id: format!("{event_id}:{subscriber_id}"),
                status: PortDeliveryStatus::Ack,
                attempt: None,
            })
            .map_err(|e| e.to_string())
        })
        .expect("record_delivery");
    }

    fn port_move_to_dlq(repo: &Path, event_id: &str, reason: &str) {
        sddia_event_manager_host::with_productive_port(repo, "test_move_dlq", |port| {
            port.move_to_dlq(MoveToDlqRequest {
                event_id: event_id.into(),
                reason: reason.into(),
            })
            .map_err(|e| e.to_string())
        })
        .expect("move_to_dlq");
    }

    fn port_get_event(
        repo: &Path,
        event_id: &str,
    ) -> sddia_core::ports::event_manager::GetEventResult {
        sddia_event_manager_host::with_productive_port(repo, "test_get_event", |port| {
            port.get_event(GetEventRequest {
                event_id: event_id.into(),
            })
            .map_err(|e| e.to_string())
        })
        .expect("get_event")
    }

    #[test]
    fn utf8_prefix_does_not_split_emdash() {
        let s = format!("{}—resto", "x".repeat(119));
        let p = utf8_prefix(&s, 120);
        assert!(p.is_char_boundary(p.len()));
        assert!(!p.contains('—'));
        assert_eq!(p.len(), 119);
    }

    #[test]
    fn parse_timestamp_accepts_z_suffix() {
        assert!(parse_timestamp("2026-06-12T10:05:42Z").is_some());
    }

    #[test]
    fn witness_regex_extracts_uuid() {
        let re = Regex::new(r"^([0-9a-fA-F-]{36})\.(.+)\.json$").unwrap();
        let cap = re
            .captures("f47cb2ff-a6b8-4b45-90af-6292b5c3393a.cumulo.materialize-fracture-pbi.json")
            .unwrap();
        assert_eq!(&cap[1], "f47cb2ff-a6b8-4b45-90af-6292b5c3393a");
    }

    #[test]
    fn circuit_coverage_emits_four_codes() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/events/orchestration")).unwrap();
        fs::create_dir_all(repo.join("SddIA/events/domain")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
              "eda_fractal": {
                "telemetry_subscriptions": "SddIA/core/event-telemetry-subscriptions.json",
                "orchestration_subscriptions": "SddIA/core/event-orchestration-subscriptions.json",
                "domain_subscriptions": "SddIA/core/event-domain-subscriptions.json"
              }
            }"#,
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/events/orchestration/pec.md"),
            "---\nevent_type: Process_Execution_Completed\nevent_family: orchestration\n---\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/events/orchestration/lqa.md"),
            "---\nevent_type: Local_QA_Requested\nevent_family: orchestration\n---\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/events/domain/pra.md"),
            "---\nevent_type: PullRequest_Audited\nevent_family: domain\n---\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/core/event-telemetry-subscriptions.json"),
            "{}",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/core/event-orchestration-subscriptions.json"),
            r#"{"Process_Execution_Completed": []}"#,
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/core/event-domain-subscriptions.json"),
            r#"{
              "Local_QA_Requested": [{"agent":"argos","process":"pull-request-review"}],
              "CapabilityDi_Requested": [{"agent":"cumulo","process":"x"}]
            }"#,
        )
        .unwrap();
        let mut state = ScanState::default();
        audit_subscription_coverage(repo, &mut state);
        let kinds: Vec<&str> = state.anomalies.iter().map(|a| a.kind.as_str()).collect();
        assert!(kinds.contains(&"EMPTY_SUBSCRIBERS"), "{kinds:?}");
        assert!(kinds.contains(&"PURGE_BLACKHOLE"), "{kinds:?}");
        assert!(kinds.contains(&"FAMILY_MISMATCH"), "{kinds:?}");
        assert!(kinds.contains(&"ORPHAN_REGISTRY_KEY"), "{kinds:?}");
        assert!(state
            .anomalies
            .iter()
            .any(|a| a.kind == "ORPHAN_REGISTRY_KEY" && a.detail == "CapabilityDi_Requested"));
        assert!(state
            .anomalies
            .iter()
            .any(|a| a.kind == "FAMILY_MISMATCH" && a.detail.contains("Local_QA_Requested")));
    }

    #[test]
    fn github_bridge_dump_is_non_ecst_sink_not_structural() {
        let dump = json!({
            "error": "iota-relay-unreachable",
            "flag": "FALLBACK_LOCAL_SIGNATURE",
            "source": "github-bridge-watcher",
            "timestamp": "2026-07-23T14:59:37Z"
        });
        assert!(is_non_ecst_sink(&dump));

        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let path = repo.join("github-bridge-20260723T145937Z-9a7687d8.json");
        fs::write(&path, "{}").unwrap();
        let mut state = ScanState::default();
        validate_ecst_event(
            repo,
            "dead-letter",
            &path,
            &dump,
            &HashSet::new(),
            &mut state,
            true,
        );
        assert_eq!(state.anomalies.len(), 1);
        assert_eq!(state.anomalies[0].kind, "non_ecst_sink");
        let structural = state
            .anomalies
            .iter()
            .filter(|a| a.kind == "structural" || a.kind == "parse")
            .count();
        assert_eq!(structural, 0);
    }

    #[test]
    fn needs_kaizen_ignores_historical_dl_and_fracture_stale() {
        assert!(!needs_kaizen_actionable(false, 0));
        assert!(needs_kaizen_actionable(true, 0));
        assert!(needs_kaizen_actionable(false, 1));
        assert!(is_fracture_stale_detail(
            "event_type=System_Fracture_Detected; pending age 26h > threshold 24h"
        ));
        assert!(!is_fracture_stale_detail(
            "event_type=PullRequest_Presented; pending age 26h > threshold 24h"
        ));
    }

    #[test]
    fn store_audit_counts_dead_letter_without_json_files() {
        let dir = std::env::temp_dir().join(format!(
            "eba-04-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("SddIA/core")).unwrap();
        fs::write(
            dir.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_bus":{"pending":".events/pending","processing":".events/processing","processed":".events/processed","dead_letter":".events/dead-letter"}}"#,
        )
        .unwrap();
        port_publish(
            &dir,
            "evt-dl",
            json!({"event_id": "evt-dl", "event_type": "Domain_Probe"}),
        );
        port_move_to_dlq(&dir, "evt-dl", "boom");
        let (state, _) = run_audit(&dir, 24).unwrap();
        assert_eq!(state.counts.get("dead-letter").copied(), Some(1));
        assert_eq!(state.dead_letter_errors.get("boom").copied(), Some(1));
        assert!(!dir.join(".events").exists());
        let _ = fs::remove_dir_all(&dir);
    }

    fn test_repo_bus_paths(repo: &Path) -> BusPaths {
        BusPaths {
            pending: repo.join(".events/pending"),
            processing: repo.join(".events/processing"),
            processed: repo.join(".events/processed"),
            dead_letter: repo.join(".events/dead-letter"),
            processing_subscribers: repo.join(".events/processing/subscribers"),
            processed_subscribers: repo.join(".events/processed/subscribers"),
            dead_letter_subscribers: repo.join(".events/dead-letter/subscribers"),
            telemetry: repo.join(".events/telemetry"),
            orchestration: repo.join(".events/orchestration"),
            domain: repo.join(".events/domain"),
        }
    }

    fn seed_kaizen_test_repo(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        let subs = include_str!("../../../core/event-subscriptions.json");
        fs::write(repo.join("SddIA/core/event-subscriptions.json"), subs).unwrap();
    }

    /// EBA-REDB-01: Kaizen en redb no crea JSON pending.
    #[test]
    fn kaizen_emit_redb_uses_store_not_pending_file() {
        let dir = std::env::temp_dir().join(format!(
            "eba-kaizen-redb-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let _ = fs::remove_dir_all(&dir);
        seed_kaizen_test_repo(&dir);
        let paths = test_repo_bus_paths(&dir);
        let seal = write_pending_kaizen(
            &dir,
            &paths,
            "rev-test",
            "justification",
            &[".events/foo.json".into()],
        )
        .unwrap();
        let event_id = seal.get("event_id").and_then(|v| v.as_str()).unwrap();
        assert!(
            !dir.join(".events/pending").join(format!("{event_id}.json")).is_file(),
            "puerto no debe escribir pending FS"
        );
        let row = port_get_event(&dir, event_id);
        assert_eq!(
            row.ecst.get("event_type").and_then(|v| v.as_str()),
            Some("Kaizen_Alert_Required")
        );
        let _ = fs::remove_dir_all(&dir);
    }

    fn seed_aut27_minimal_repo(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/agents")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_bus":{"pending":".events/pending","processing":".events/processing","processed":".events/processed","dead_letter":".events/dead-letter"}}"#,
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/core/event-domain-subscriptions.json"),
            include_str!("../../../core/event-domain-subscriptions.json"),
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/agents/tekton.md"),
            "---\nname: tekton\nllm_profile:\n  tier: low\n---\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/agents/argos.md"),
            "---\nname: argos\nllm_profile:\n  tier: medium\n---\n",
        )
        .unwrap();
    }

    #[test]
    fn store_unavailable_emits_anomaly_not_silent_empty_scan() {
        let dir = std::env::temp_dir().join(format!(
            "eba-store-unavail-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let _ = fs::remove_dir_all(&dir);
        seed_aut27_minimal_repo(&dir);
        let db = sddia_event_store::runtime_db_path(&dir);
        fs::create_dir_all(db.parent().unwrap()).unwrap();
        fs::write(&db, b"not-a-redb-database").unwrap();
        let paths = test_repo_bus_paths(&dir);
        let state = audit_state_from_store(&dir, &paths);
        assert!(
            state
                .anomalies
                .iter()
                .any(|a| a.kind == "store_unavailable"),
            "{:?}",
            state.anomalies
        );
        let md = build_report_md(
            &AuditSummary {
                counts: BTreeMap::new(),
                counts_by_type: BTreeMap::new(),
                stale_pending_count: 0,
                orphan_witness_count: 0,
                structural_error_count: 0,
                dead_letter_count: 0,
                dead_letter_witness_count: 0,
                non_ecst_sink_count: 0,
                actionable_stale_pending_count: 0,
                fracture_stale_pending_count: 0,
            },
            &state.anomalies,
            &BTreeMap::new(),
            &[],
        );
        assert!(md.contains("Store no disponible"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn redb_audit_marks_success_sin_efecto_and_tracker_outbound_shield() {
        let dir = std::env::temp_dir().join(format!(
            "eba-success-efecto-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let _ = fs::remove_dir_all(&dir);
        seed_aut27_minimal_repo(&dir);
        let event_id = "3803aeae-7a1e-4c15-b02f-8c72d16fd8ff";
        let ecst = json!({
            "event_id": event_id,
            "event_type": "PullRequest_Merged",
            "payload": { "tracker_ref": "AUT-8" }
        });
        let row = AuditEventRow {
            event_id: event_id.into(),
            event_type: "PullRequest_Merged".into(),
            ecst: ecst.clone(),
        };
        let deliveries = vec![
            (
                event_id.into(),
                "argos.notify-humanized-pr-merged".into(),
                AuditDeliveryStatus::Success,
            ),
            (
                event_id.into(),
                "tekton.tracker-stamp".into(),
                AuditDeliveryStatus::Success,
            ),
        ];
        let mut state = ScanState::default();
        audit_domain_delivery_signals(&dir, &[row.clone()], &deliveries, &mut state);
        let efecto: Vec<_> = state
            .anomalies
            .iter()
            .filter(|a| a.kind == "success_sin_efecto")
            .collect();
        assert_eq!(efecto.len(), 2, "{:?}", state.anomalies);

        fs::create_dir_all(dir.join(".SddIA/state/tracker-outbound")).unwrap();
        fs::write(dir.join(".SddIA/state/tracker-outbound/AUT-8.jsonl"), "{}\n").unwrap();
        let mut state2 = ScanState::default();
        audit_domain_delivery_signals(&dir, &[row], &deliveries, &mut state2);
        let tekton_marked = state2.anomalies.iter().any(|a| {
            a.kind == "success_sin_efecto" && a.detail.starts_with("tekton.tracker-stamp")
        });
        let argos_marked = state2.anomalies.iter().any(|a| {
            a.kind == "success_sin_efecto" && a.detail.starts_with("argos.notify-humanized")
        });
        assert!(!tekton_marked);
        assert!(argos_marked);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn redb_audit_catalog_mismatch_on_presented() {
        let dir = std::env::temp_dir().join(format!(
            "eba-catalog-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let _ = fs::remove_dir_all(&dir);
        seed_aut27_minimal_repo(&dir);
        let event_id = "BVLgK63NjKvYhH1Zt4LGPE2KbrSCAhQaBTnJmfng8xCU";
        port_publish(
            &dir,
            event_id,
            json!({
                "event_id": event_id,
                "event_type": "PullRequest_Presented",
                "payload": {}
            }),
        );
        port_record_ack(&dir, event_id, "argos");
        port_record_ack(&dir, event_id, "cumulo");

        let row = AuditEventRow {
            event_id: event_id.into(),
            event_type: "PullRequest_Presented".into(),
            ecst: json!({
                "event_id": event_id,
                "event_type": "PullRequest_Presented",
                "payload": {}
            }),
        };
        let deliveries = vec![
            (
                event_id.into(),
                "argos".into(),
                AuditDeliveryStatus::Success,
            ),
            (
                event_id.into(),
                "cumulo".into(),
                AuditDeliveryStatus::Success,
            ),
        ];
        let mut catalog_state = ScanState::default();
        audit_domain_delivery_signals(&dir, &[row], &deliveries, &mut catalog_state);
        assert!(catalog_state.anomalies.iter().any(|a| {
            a.kind == "delivery_fuera_de_catalogo" && a.detail == "argos"
        }));
        assert!(catalog_state.anomalies.iter().any(|a| {
            a.kind == "delivery_fuera_de_catalogo" && a.detail == "cumulo"
        }));
        for expected in [
            "argos.pull-request-review",
            "tekton.tracker-stamp",
            "argos.send-telegram-notification",
        ] {
            assert!(
                catalog_state.anomalies.iter().any(|a| {
                    a.kind == "suscriptor_sin_fila" && a.detail == expected
                }),
                "falta suscriptor_sin_fila {expected}: {:?}",
                catalog_state.anomalies
            );
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn redb_audit_residual_pending_with_fracture_trace() {
        let dir = std::env::temp_dir().join(format!(
            "eba-residual-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let _ = fs::remove_dir_all(&dir);
        seed_aut27_minimal_repo(&dir);
        fs::create_dir_all(dir.join(".events/pending")).unwrap();
        let pending_name = "orphan-pending.json";
        fs::write(
            dir.join(".events/pending").join(pending_name),
            r#"{"event_id":"x"}"#,
        )
        .unwrap();

        let fracture_id = "residual-fracture-1";
        let trace = format!("residual .events/pending/{pending_name} con store redb");
        port_publish(
            &dir,
            fracture_id,
            json!({
                "event_id": fracture_id,
                "event_type": "System_Fracture_Detected",
                "payload": {
                    "friction_id": "F-EDA-RESIDUAL-FILESYSTEM",
                    "error_trace": trace
                }
            }),
        );

        let paths = test_repo_bus_paths(&dir);
        let state = audit_state_from_store(&dir, &paths);
        let residual = state
            .anomalies
            .iter()
            .find(|a| a.kind == "residual_filesystem")
            .expect("residual_filesystem");
        assert!(residual.detail.contains("F-EDA-RESIDUAL-FILESYSTEM"));
        assert!(residual.detail.contains(fracture_id));
        let _ = fs::remove_dir_all(&dir);
    }
}
