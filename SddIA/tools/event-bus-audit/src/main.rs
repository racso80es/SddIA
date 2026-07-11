use chrono::{DateTime, Utc};
use regex::Regex;
use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashSet};
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

fn load_known_event_types(repo: &Path) -> HashSet<String> {
    let mut types = HashSet::new();
    let events_root = repo.join("SddIA/events");
    if !events_root.is_dir() {
        return types;
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
                if let Ok(text) = fs::read_to_string(&path) {
                    if let Some(fm) = text.split("---").nth(1) {
                        if let Some(cap) = re.captures(fm) {
                            types.insert(cap[1].trim().to_string());
                        }
                    }
                }
            }
        }
    }
    types
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
                            stale_pending.push(format!("{eid} ({age_hours}h)"));
                            state.anomalies.push(Anomaly {
                                kind: "stale_pending".into(),
                                path: rel_path(repo, &path),
                                detail: format!("pending age {age_hours}h > threshold {stale_threshold_hours}h"),
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

fn run_audit(repo: &Path, stale_threshold_hours: i64) -> Result<(ScanState, BusPaths), String> {
    let paths = resolve_bus_paths(repo)?;
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

    state.stale_pending = stale_pending;
    Ok((state, paths))
}

fn write_pending_kaizen(
    repo: &Path,
    paths: &BusPaths,
    review_id: &str,
    justification: &str,
    implicated: &[String],
) -> Result<Value, String> {
    let event_id = Uuid::new_v4().to_string();
    let event = json!({
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
    fs::create_dir_all(&paths.pending).map_err(|e| e.to_string())?;
    let target = paths.pending.join(format!("{event_id}.json"));
    let text = serde_json::to_string_pretty(&event).map_err(|e| e.to_string())?;
    fs::write(&target, format!("{text}\n")).map_err(|e| e.to_string())?;
    Ok(json!({
        "event_id": event_id,
        "target_path": rel_path(repo, &target)
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
        "## Conteos por estado".into(),
        String::new(),
        "| Estado | Cantidad |".into(),
        "|--------|----------|".into(),
    ];
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
    lines.push(format!("- Testigos huérfanos: {}", summary.orphan_witness_count));
    lines.push(format!("- Errores estructurales: {}", summary.structural_error_count));
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
                format!("{}…", &err[..120])
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

    let summary = AuditSummary {
        stale_pending_count: state.stale_pending.len() as u64,
        orphan_witness_count,
        structural_error_count,
        dead_letter_count: *state.counts.get("dead-letter").unwrap_or(&0),
        dead_letter_witness_count: *state
            .counts
            .get("dead-letter/subscribers")
            .unwrap_or(&0),
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

    let needs_kaizen = summary.dead_letter_count > 0
        || summary.dead_letter_witness_count > 0
        || structural_error_count > 0
        || orphan_witness_count > 0
        || summary.stale_pending_count > 0;

    let mut kaizen_event_id = None;
    let mut kaizen_target = None;

    if emit_kaizen_alert && needs_kaizen {
        let review_id = Uuid::new_v4().to_string();
        let justification = format!(
            "Auditoría event-bus-audit: {} dead-letter cabeceras, {} testigos KO, {} anomalías estructurales, {} huérfanos, {} pending estancados",
            summary.dead_letter_count,
            summary.dead_letter_witness_count,
            structural_error_count,
            orphan_witness_count,
            summary.stale_pending_count
        );
        let implicated: Vec<String> = state
            .anomalies
            .iter()
            .take(50)
            .map(|a| a.path.clone())
            .collect();
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
}
