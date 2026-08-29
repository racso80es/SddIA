//! Handler `phagocyte-recovered-fracture-pbis` — auto-poda documental de fracturas recuperadas.

use super::super::daemons::{iso_now, parse_iso, pid_alive, read_lock, state_dir, write_json_atomic};
use crate::core::fracture_pbi::{resolve_todos_done_rel, resolve_todos_pending_rel, scan_fracture_ledger};
use crate::envelope::OrchestratorEnvelope;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PhagocyteCandidate {
    pub rel_path: String,
    pub document_id: String,
    pub fracture_process: String,
    pub trace_last_heartbeat: String,
    pub lock_started_at: String,
    pub reason: &'static str,
}

fn phagocyte_ledger_path(repo: &Path) -> Result<PathBuf, String> {
    Ok(state_dir(repo)?.join("phagocytosed-fractures.json"))
}

fn load_ledger(repo: &Path) -> Value {
    let path = phagocyte_ledger_path(repo).ok();
    let Some(path) = path.filter(|p| p.is_file()) else {
        return json!({"entries": []});
    };
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or(json!({"entries": []}))
}

fn save_ledger(repo: &Path, ledger: &Value) -> Result<(), String> {
    write_json_atomic(&phagocyte_ledger_path(repo)?, ledger)
}

pub fn parse_last_heartbeat_from_text(text: &str) -> Option<DateTime<Utc>> {
    static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"last_heartbeat=([0-9T:\-+Z]+)").expect("regex"));
    let cap = re.captures(text)?;
    parse_iso(cap.get(1)?.as_str())
}

pub fn find_phagocyte_candidates(repo: &Path) -> Result<Vec<PhagocyteCandidate>, String> {
    let scan = scan_fracture_ledger(repo)?;
    let mut out = Vec::new();
    for rec in scan.pending {
        if !rec.document_id.starts_with("PBI-FIX-FRACTURE-") {
            continue;
        }
        let path = repo.join(&rec.rel_path);
        let text = fs::read_to_string(&path).unwrap_or_default();
        let trace_hb = match parse_last_heartbeat_from_text(&text) {
            Some(dt) => dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            None => continue,
        };
        let lock = match read_lock(repo, &rec.fracture_process) {
            Some(l) => l,
            None => continue,
        };
        let pid = lock.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        if !pid_alive(pid) {
            continue;
        }
        let started = lock
            .get("started_at")
            .and_then(|v| v.as_str())
            .and_then(parse_iso);
        let trace_dt = parse_iso(&trace_hb);
        let (Some(started), Some(trace_dt)) = (started, trace_dt) else {
            continue;
        };
        if started <= trace_dt {
            continue;
        }
        out.push(PhagocyteCandidate {
            rel_path: rec.rel_path,
            document_id: rec.document_id,
            fracture_process: rec.fracture_process,
            trace_last_heartbeat: trace_hb,
            lock_started_at: started.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            reason: "trace_before_lock",
        });
    }
    Ok(out)
}

fn append_ledger_entries(repo: &Path, candidates: &[PhagocyteCandidate]) -> Result<(), String> {
    if candidates.is_empty() {
        return Ok(());
    }
    let mut ledger = load_ledger(repo);
    let entries = ledger
        .as_object_mut()
        .and_then(|o| o.get_mut("entries"))
        .and_then(|e| e.as_array_mut())
        .ok_or("ledger entries invalid")?;
    let now = iso_now();
    for c in candidates {
        entries.push(json!({
            "document_id": c.document_id,
            "fracture_process": c.fracture_process,
            "trace_last_heartbeat": c.trace_last_heartbeat,
            "lock_started_at": c.lock_started_at,
            "phagocytosed_at": now,
            "reason": c.reason,
        }));
    }
    save_ledger(repo, &ledger)
}

fn today_ola_suffix() -> String {
    Utc::now().format("%Y%m%d").to_string()
}

fn apply_documental(
    repo: &Path,
    candidates: &[PhagocyteCandidate],
) -> Result<Vec<String>, String> {
    let pending_rel = resolve_todos_pending_rel(repo)?;
    let done_rel = resolve_todos_done_rel(repo)?;
    let fix_ola = format!("docs/fixes/centinelas-fracture-ola-{}", today_ola_suffix());
    let mut applied = Vec::new();
    fs::create_dir_all(repo.join(&fix_ola)).map_err(|e| e.to_string())?;
    let closed = Utc::now().format("%Y-%m-%d").to_string();

    for c in candidates {
        let src = repo.join(&c.rel_path);
        if !src.is_file() {
            continue;
        }
        let fname = src.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let dst = repo.join(&done_rel).join(fname);
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut body = fs::read_to_string(&src).map_err(|e| e.to_string())?;
        if body.starts_with("---") {
            if let Some(end) = body[3..].find("---") {
                let fm_end = 3 + end + 3;
                let fm = &body[..fm_end];
                let rest = &body[fm_end..];
                let mut lines: Vec<String> = fm
                    .lines()
                    .map(str::to_string)
                    .filter(|l| {
                        let t = l.trim();
                        !t.starts_with("status:")
                            && !t.starts_with("closed:")
                            && !t.starts_with("laudo:")
                            && !t.starts_with("fix_ref:")
                    })
                    .collect();
                lines.push(format!("status: cerrado"));
                lines.push(format!("closed: \"{closed}\""));
                lines.push("laudo: B-automatic-phagocyte".to_string());
                lines.push(format!("fix_ref: {fix_ola}"));
                body = format!("{}\n{}", lines.join("\n"), rest);
            }
        }
        fs::write(&dst, &body).map_err(|e| e.to_string())?;
        fs::remove_file(&src).map_err(|e| e.to_string())?;
        applied.push(c.rel_path.clone());
    }

    if !applied.is_empty() {
        let manifest = repo.join(&fix_ola).join("phagocyte-manifest.md");
        let list = applied
            .iter()
            .map(|p| format!("- `{p}`"))
            .collect::<Vec<_>>()
            .join("\n");
        let manifest_body = format!(
            "---\nfix_ola: centinelas-fracture-ola-{}\ncreated: \"{}\"\nlaudo: B-automatic-phagocyte\n---\n\n# Ola fagocitosis automática\n\n{}\n",
            today_ola_suffix(),
            closed,
            list
        );
        fs::write(&manifest, manifest_body).map_err(|e| e.to_string())?;
    }
    Ok(applied)
}

pub fn run_phagocyte(
    repo: &Path,
    apply: bool,
) -> Result<Value, String> {
    let candidates = find_phagocyte_candidates(repo)?;
    append_ledger_entries(repo, &candidates)?;
    let applied = if apply {
        apply_documental(repo, &candidates)?
    } else {
        Vec::new()
    };
    let candidate_paths: Vec<String> = candidates.iter().map(|c| c.rel_path.clone()).collect();
    Ok(json!({
        "candidates": candidate_paths,
        "applied": applied,
        "skipped_count": 0,
        "apply": apply,
    }))
}

pub fn env_apply_enabled() -> bool {
    matches!(
        env::var("SDDIA_PHAGOCYTE_APPLY").ok().as_deref(),
        Some("1") | Some("true") | Some("yes") | Some("on")
    )
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let apply = process_inputs
        .get("apply")
        .and_then(|v| v.as_bool())
        .unwrap_or_else(env_apply_enabled);
    let result = run_phagocyte(repo, apply)?;
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(result),
        error: None,
        execution_report: Some(json!({
            "process_name": "phagocyte-recovered-fracture-pbis",
            "phases": [{
                "phase_name": "Fagocitosis",
                "status": "executed",
                "handler": "phagocyte-recovered-fracture-pbis",
            }],
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_last_heartbeat_from_trace() {
        let text = "Centinela email-watcher omitió 1532 ciclos. last_heartbeat=2026-08-19T16:26:27Z";
        let dt = parse_last_heartbeat_from_text(text).unwrap();
        assert_eq!(
            dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            "2026-08-19T16:26:27Z"
        );
    }

    #[test]
    fn phagocyte_skip_unparseable_trace() {
        let dir = tempfile::TempDir::new().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"todos":{"pending":"docs/todos/pending","done":"docs/todos/done"}},"daemons_instance":{"state":".SddIA/daemons/state","status":".SddIA/daemons/status"}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join(".SddIA/daemons/state")).unwrap();
        fs::write(
            repo.join("docs/todos/pending/fracture.md"),
            "---\ndocument_id: PBI-FIX-FRACTURE-abc123\nfracture_process: event-watcher\nfracture_hash: abc123\nstatus: pending\n---\n\nsin traza\n",
        )
        .unwrap();
        let c = find_phagocyte_candidates(repo).unwrap();
        assert!(c.is_empty());
    }

    #[test]
    fn phagocyte_predicate_trace_before_lock() {
        let dir = tempfile::TempDir::new().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();
        fs::create_dir_all(repo.join("docs/todos/done")).unwrap();
        fs::create_dir_all(repo.join(".SddIA/daemons/status")).unwrap();
        fs::create_dir_all(repo.join(".SddIA/daemons/state")).unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"todos":{"pending":"docs/todos/pending","done":"docs/todos/done"}},"daemons_instance":{"state":".SddIA/daemons/state","status":".SddIA/daemons/status"}}"#,
        )
        .unwrap();
        fs::write(
            repo.join("docs/todos/pending/[FIX] event-watcher — fractura.md"),
            "---\ndocument_id: PBI-FIX-FRACTURE-deadbeef1234\nfracture_process: event-watcher\nfracture_hash: deadbeef1234\nstatus: pending\n---\n\n```\nCentinela event-watcher omitió 10 ciclos. last_heartbeat=2026-08-19T08:00:00Z\n```\n",
        )
        .unwrap();
        fs::write(
            repo.join(".SddIA/daemons/status/event-watcher.lock"),
            r#"{"pid":99999,"started_at":"2026-08-26T06:19:32Z"}"#,
        )
        .unwrap();
        // PID 99999 unlikely alive — mock by testing parse + started_at logic only
        // Use current process pid for alive check
        let pid = std::process::id() as i32;
        fs::write(
            repo.join(".SddIA/daemons/status/event-watcher.lock"),
            format!(r#"{{"pid":{pid},"started_at":"2026-08-26T06:19:32Z"}}"#),
        )
        .unwrap();
        let c = find_phagocyte_candidates(repo).unwrap();
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].fracture_process, "event-watcher");
    }
}
