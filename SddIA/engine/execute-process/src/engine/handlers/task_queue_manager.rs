//! Handler nativo: task-queue-manager — triaje Kalma2 + despacho de ciclo hijo.

use super::super::fractal::{load_fractal_dirs, write_fractal_event};
use super::super::invoke_orchestrator::invoke_process_full_with_env;
use super::super::persist_pec_correlation_proof;
use super::super::residual_runner;
use super::super::thermodynamic;
use super::super::workspace::{
    load_paths_config, resolve_documentation_features_path, resolve_documentation_fixes_path,
};
use crate::core::resolver::load_process_def;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use uuid::Uuid;

const LOCK_CONTENT_GRACE_SECS: u64 = 2;
const TQM_DISPATCH_DISCARDED_EVENT_TYPE: &str = "TQM_Dispatch_Discarded";

const DISPATCHABLE: &[&str] = &["bug-fix", "feature", "refactorization"];

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

struct SingleFlightGuard {
    path: PathBuf,
}

impl Drop for SingleFlightGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn single_flight_dir(repo: &Path) -> PathBuf {
    repo.join(".SddIA/daemons/state/tqm-single-flight")
}

fn normalize_rel(rel: &str) -> String {
    let p = rel.trim().replace('\\', "/");
    if let Some(stripped) = p.strip_prefix("./") {
        stripped.to_string()
    } else {
        p
    }
}

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

struct LockIdentity {
    lock_id: String,
    lock_hex: String,
    pbi_ref_normalized: String,
}

fn resolve_lock_identity(repo: &Path, pbi_ref: &str) -> Result<Option<LockIdentity>, String> {
    let trimmed = pbi_ref.trim();
    if trimmed.is_empty() || trimmed.contains("..") {
        return Ok(None);
    }
    let norm_pbi = normalize_rel(trimmed);
    if let Some(body) = load_pbi_body(repo, &norm_pbi) {
        if let Some(doc_id) = extract_fm_string(&body, "document_id")
            .or_else(|| extract_fm_string(&body, "uuid"))
        {
            let lock_id = format!("id:{doc_id}");
            return Ok(Some(LockIdentity {
                lock_hex: sha256_hex(&lock_id),
                lock_id,
                pbi_ref_normalized: norm_pbi,
            }));
        }
    }
    let lock_id = format!("path:{}", sha256_hex(&norm_pbi));
    Ok(Some(LockIdentity {
        lock_hex: sha256_hex(&lock_id),
        lock_id,
        pbi_ref_normalized: norm_pbi,
    }))
}

#[derive(Debug, Deserialize)]
struct LockPayload {
    pid: u32,
    #[serde(default)]
    starttime: Option<String>,
    #[serde(default)]
    holder_correlation_id: Option<String>,
}

enum LockOccupancy {
    Held,
    Stale,
}

fn lock_mtime_age_secs(path: &Path) -> Option<u64> {
    let modified = fs::metadata(path).ok()?.modified().ok()?;
    let now = SystemTime::now();
    now.duration_since(modified).ok().map(|d| d.as_secs())
}

#[cfg(target_os = "linux")]
fn proc_starttime(pid: u32) -> Option<String> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let close = stat.rfind(')')?;
    let after = stat.get(close + 2..)?;
    let fields: Vec<&str> = after.split_whitespace().collect();
    fields.get(19).map(|s| s.to_string())
}

#[cfg(not(target_os = "linux"))]
fn proc_starttime(_pid: u32) -> Option<String> {
    None
}

#[cfg(unix)]
fn signal_process_alive(pid: u32) -> bool {
    unsafe { libc::kill(pid as i32, 0) == 0 }
}

#[cfg(not(unix))]
fn signal_process_alive(_pid: u32) -> bool {
    false
}

fn liveness_supported() -> bool {
    cfg!(unix)
}

fn process_matches_lock(pid: u32, starttime: Option<&str>) -> Result<bool, String> {
    if !liveness_supported() {
        return Err("tqm-single-flight: liveness no soportada en esta plataforma".into());
    }
    #[cfg(target_os = "linux")]
    {
        let proc_path = format!("/proc/{pid}");
        if !Path::new(&proc_path).exists() {
            return Ok(false);
        }
        if let Some(expected) = starttime.filter(|s| !s.is_empty()) {
            return Ok(proc_starttime(pid).as_deref() == Some(expected));
        }
        return Ok(true);
    }
    #[cfg(all(unix, not(target_os = "linux")))]
    {
        eprintln!("[TQM-SF-LIVENESS] backend=kill0 pid={pid}");
        return Ok(signal_process_alive(pid));
    }
    #[cfg(not(unix))]
    {
        let _ = (pid, starttime);
        Err("tqm-single-flight: liveness no soportada en esta plataforma".into())
    }
}

fn parse_lock_payload(raw: &str) -> Option<LockPayload> {
    if let Ok(v) = serde_json::from_str::<LockPayload>(raw.trim()) {
        return Some(v);
    }
    let pid = raw.trim().parse::<u32>().ok()?;
    Some(LockPayload {
        pid,
        starttime: None,
        holder_correlation_id: None,
    })
}

fn lock_occupancy(path: &Path) -> Result<LockOccupancy, String> {
    let raw = fs::read_to_string(path).unwrap_or_default();
    if raw.trim().is_empty() {
        let age = lock_mtime_age_secs(path).unwrap_or(LOCK_CONTENT_GRACE_SECS + 1);
        return Ok(if age <= LOCK_CONTENT_GRACE_SECS {
            LockOccupancy::Held
        } else {
            LockOccupancy::Stale
        });
    }
    let Some(payload) = parse_lock_payload(&raw) else {
        let age = lock_mtime_age_secs(path).unwrap_or(LOCK_CONTENT_GRACE_SECS + 1);
        return Ok(if age <= LOCK_CONTENT_GRACE_SECS {
            LockOccupancy::Held
        } else {
            LockOccupancy::Stale
        });
    };
    if process_matches_lock(payload.pid, payload.starttime.as_deref())? {
        Ok(LockOccupancy::Held)
    } else {
        Ok(LockOccupancy::Stale)
    }
}

fn read_holder_correlation_id(path: &Path) -> Option<String> {
    let raw = fs::read_to_string(path).ok()?;
    parse_lock_payload(&raw)
        .and_then(|p| p.holder_correlation_id)
        .filter(|s| !s.is_empty())
}

fn write_lock_payload(
    file: &mut std::fs::File,
    holder_correlation_id: Option<&str>,
) -> Result<(), String> {
    let starttime = proc_starttime(std::process::id());
    let payload = json!({
        "pid": std::process::id(),
        "starttime": starttime,
        "holder_correlation_id": holder_correlation_id.filter(|s| !s.is_empty()),
    });
    let line = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    file.write_all(line.as_bytes())
        .map_err(|e| format!("tqm-single-flight write: {e}"))?;
    file.sync_all()
        .map_err(|e| format!("tqm-single-flight sync: {e}"))
}

fn try_acquire_single_flight(
    repo: &Path,
    identity: &LockIdentity,
    holder_correlation_id: Option<&str>,
) -> Result<Option<SingleFlightGuard>, String> {
    if !liveness_supported() {
        return Err("tqm-single-flight: liveness no soportada en esta plataforma".into());
    }
    let dir = single_flight_dir(repo);
    fs::create_dir_all(&dir).map_err(|e| format!("tqm-single-flight mkdir: {e}"))?;
    let path = dir.join(format!("{}.lock", identity.lock_hex));
    for _ in 0..3 {
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut f) => {
                write_lock_payload(&mut f, holder_correlation_id)?;
                return Ok(Some(SingleFlightGuard { path }));
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => match lock_occupancy(&path)? {
                LockOccupancy::Held => return Ok(None),
                LockOccupancy::Stale => {
                    let _ = fs::remove_file(&path);
                }
            },
            Err(e) => return Err(format!("tqm-single-flight lock: {e}")),
        }
    }
    Ok(None)
}

fn persist_single_flight_hit_proof(
    repo: &Path,
    identity: &LockIdentity,
    holder_correlation_id: Option<&str>,
    discarded_correlation_id: Option<&str>,
) -> Result<String, String> {
    let dir = persist_pec_correlation_proof::resolve_eda_proofs_dir(repo).join("tqm-single-flight");
    fs::create_dir_all(&dir).map_err(|e| format!("tqm-single-flight proof mkdir: {e}"))?;
    let path = dir.join(format!("{}.json", identity.lock_hex));
    let payload = json!({
        "kind": "tqm-single-flight-hit",
        "timestamp": Utc::now().to_rfc3339(),
        "pbi_ref": identity.pbi_ref_normalized,
        "lock_key": identity.lock_id,
        "lock_hex": identity.lock_hex,
        "holder_correlation_id": holder_correlation_id.filter(|s| !s.is_empty()),
        "discarded_correlation_id": discarded_correlation_id.filter(|s| !s.is_empty()),
        "reason": "single_flight_pbi",
    });
    fs::write(
        &path,
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("tqm-single-flight proof write: {e}"))?;
    Ok(path
        .strip_prefix(repo)
        .unwrap_or(&path)
        .to_string_lossy()
        .replace('\\', "/"))
}

fn emit_single_flight_discarded_event(
    repo: &Path,
    identity: &LockIdentity,
    holder_correlation_id: Option<&str>,
    discarded_correlation_id: Option<&str>,
) -> Result<Value, String> {
    let (_, orch_dir, _, _) = load_fractal_dirs(repo);
    let event_id = Uuid::new_v4().to_string();
    let payload = json!({
        "pbi_ref": identity.pbi_ref_normalized,
        "lock_key": identity.lock_id,
        "holder_correlation_id": holder_correlation_id.filter(|s| !s.is_empty()),
        "discarded_correlation_id": discarded_correlation_id.filter(|s| !s.is_empty()),
        "reason": "single_flight_pbi",
    });
    let event = json!({
        "event_id": event_id,
        "event_type": TQM_DISPATCH_DISCARDED_EVENT_TYPE,
        "event_family": "orchestration",
        "timestamp": Utc::now().to_rfc3339(),
        "emitter_agent": "task-queue-manager",
        "correlation_id": discarded_correlation_id.filter(|s| !s.is_empty()),
        "payload": payload,
        "delivery_state": {},
    });
    write_fractal_event(repo, &event, &orch_dir)
}

fn single_flight_hit_envelope(
    process: &str,
    discarded_correlation_id: Option<&str>,
    holder_correlation_id: Option<&str>,
    pbi: Option<String>,
    identity: &LockIdentity,
    proof_path: &str,
    orchestration: Option<Value>,
) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "dispatched_process": process,
            "correlation_id": discarded_correlation_id,
            "holder_correlation_id": holder_correlation_id,
            "pbi_ref": pbi,
            "lock_key": identity.lock_id,
            "lock_hex": identity.lock_hex,
            "single_flight_hit": true,
            "reason": "single_flight_pbi",
            "proof_path": proof_path,
            "orchestration": orchestration,
            "handler": "task-queue-manager-kalma2",
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": "task-queue-manager",
            "phases": [{
                "phase_name": "Triaje Kalma2",
                "status": "executed",
                "handler": "task-queue-manager-kalma2",
                "single_flight_hit": true,
            }, {
                "phase_name": "Despacho",
                "status": "skipped",
                "reason": "single_flight_pbi",
            }],
        })),
        exit_code: 0,
    }
}

/// Extrae `docs/todos/{pending|done}/….md` aunque el path contenga espacios.
pub fn extract_pbi_path(text: &str) -> Option<String> {
    for anchor in ["docs/todos/pending/", "docs/todos/done/"] {
        if let Some(start) = text.find(anchor) {
            let rest = &text[start..];
            if let Some(rel_end) = rest.find(".md") {
                return Some(rest[..=rel_end + 2].to_string());
            }
        }
    }
    None
}

fn sanitize_slug(raw: &str) -> String {
    let mut out = String::new();
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if matches!(c, '-' | '_' | '.') && !out.ends_with('-') {
            out.push('-');
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        format!("kalma2-{}", &Uuid::new_v4().to_string()[..8])
    } else {
        trimmed.chars().take(64).collect()
    }
}

fn derive_slug(pbi_ref: Option<&str>, task_text: &str) -> String {
    let source = pbi_ref.unwrap_or(task_text);
    if let Some(open) = source.rfind('(') {
        if let Some(close) = source[open + 1..].find(')') {
            let inner = source[open + 1..open + 1 + close].trim();
            if inner.len() >= 8 && inner.chars().all(|c| c.is_ascii_hexdigit()) {
                return sanitize_slug(inner);
            }
        }
    }
    let from_text = extract_pbi_path(task_text);
    if let Some(pbi) = pbi_ref.map(str::to_string).or(from_text) {
        let name = pbi
            .rsplit('/')
            .next()
            .unwrap_or(&pbi)
            .trim_end_matches(".md");
        let cleaned = name
            .trim_start_matches('[')
            .replace(']', " ")
            .replace("FIX", "")
            .replace("FEATURE", "")
            .replace("OPERATIVO", "");
        return sanitize_slug(&cleaned);
    }
    sanitize_slug(task_text)
}

/// Preferencia PBI: `suggested_branch` del frontmatter (p. ej. fix/execute-process-…).
fn extract_suggested_branch(pbi_body: &str) -> Option<String> {
    let trimmed = pbi_body.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let rest = trimmed.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    let fm = &rest[..end];
    for line in fm.lines() {
        let line = line.trim();
        let Some(raw) = line
            .strip_prefix("suggested_branch:")
            .map(str::trim)
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let cleaned = raw
            .trim_matches('"')
            .trim_matches('\'')
            .trim()
            .trim_matches('"')
            .trim_matches('\'');
        if cleaned.is_empty() || cleaned.contains("..") || cleaned.contains(' ') {
            return None;
        }
        if !(cleaned.starts_with("fix/")
            || cleaned.starts_with("feat/")
            || cleaned.starts_with("feature/")
            || cleaned.starts_with("refactor/"))
        {
            return None;
        }
        return Some(cleaned.to_string());
    }
    None
}

fn extract_fm_string(pbi_body: &str, key: &str) -> Option<String> {
    let trimmed = pbi_body.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let rest = trimmed.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    let prefix = format!("{key}:");
    for line in rest[..end].lines() {
        let line = line.trim();
        let Some(raw) = line
            .strip_prefix(&prefix)
            .map(str::trim)
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let cleaned = raw
            .trim_matches('"')
            .trim_matches('\'')
            .trim()
            .trim_matches('"')
            .trim_matches('\'');
        if cleaned.is_empty() || cleaned.contains("..") {
            return None;
        }
        return Some(cleaned.to_string());
    }
    None
}

fn resolve_child_persist_ref(
    repo: &Path,
    process: &str,
    slug: &str,
    pbi_body: Option<&str>,
) -> String {
    if let Some(body) = pbi_body {
        if let Some(p) = extract_fm_string(body, "persist_ref_suggested")
            .or_else(|| extract_fm_string(body, "persist_ref"))
        {
            return p;
        }
    }
    let cfg = load_paths_config(repo).unwrap_or_else(|_| json!({}));
    match process {
        "bug-fix" => format!("{}/{}", resolve_documentation_fixes_path(repo, &cfg), slug),
        _ => format!(
            "{}/{}",
            resolve_documentation_features_path(repo, &cfg),
            slug
        ),
    }
}

fn slug_from_branch(branch: &str) -> String {
    let stripped = branch
        .strip_prefix("fix/")
        .or_else(|| branch.strip_prefix("feat/"))
        .or_else(|| branch.strip_prefix("feature/"))
        .or_else(|| branch.strip_prefix("refactor/"))
        .unwrap_or(branch);
    sanitize_slug(stripped)
}

fn resolve_pbi_ref(inputs: &Value, task_text: &str) -> Option<String> {
    if let Some(p) = inputs
        .get("pbi_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Some(p.to_string());
    }
    extract_pbi_path(task_text)
}

/// Lee el cuerpo del PBI referenciado (slice C). Fallo FS → None (no tumba despacho).
pub fn load_pbi_body(repo: &Path, pbi_ref: &str) -> Option<String> {
    let rel = pbi_ref.trim();
    if rel.is_empty() || rel.contains("..") {
        return None;
    }
    let path = repo.join(rel);
    let raw = std::fs::read_to_string(&path).ok()?;
    let body = raw.trim();
    if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    }
}

fn build_child_inputs(
    repo: &Path,
    process: &str,
    task_text: &str,
    pbi_ref: Option<&str>,
    correlation_id: Option<&str>,
) -> Result<Value, String> {
    let mut map = Map::new();
    if let Some(c) = correlation_id.filter(|s| !s.is_empty()) {
        map.insert("correlation_id".into(), json!(c));
    }
    let mut suggested_branch: Option<String> = None;
    if let Some(p) = pbi_ref.filter(|s| !s.is_empty()) {
        map.insert("pbi_ref".into(), json!(p));
        if let Some(body) = load_pbi_body(repo, p) {
            suggested_branch = extract_suggested_branch(&body);
            map.insert("pbi_body".into(), json!(body));
        }
    }
    let slug = suggested_branch
        .as_deref()
        .map(slug_from_branch)
        .unwrap_or_else(|| derive_slug(pbi_ref, task_text));
    map.insert("base_branch".into(), json!("main"));

    // Semilla de ciclo: preferir cuerpo PBI (misión) y conservar prompt como contexto.
    let seed = match map.get("pbi_body").and_then(|v| v.as_str()) {
        Some(body) if !body.is_empty() => format!(
            "## Prompt operador\n{}\n\n## PBI adjunto (`{}`)\n{}",
            task_text,
            pbi_ref.unwrap_or(""),
            body
        ),
        _ => task_text.to_string(),
    };

    match process {
        "bug-fix" => {
            map.insert("bug_summary".into(), json!(seed));
            map.insert("fix_name".into(), json!(slug.clone()));
            let branch = suggested_branch
                .clone()
                .unwrap_or_else(|| format!("fix/{slug}"));
            map.insert("branch_name".into(), json!(branch));
        }
        "feature" => {
            map.insert("refined_requirements".into(), json!(seed));
            map.insert("feature_name".into(), json!(slug.clone()));
            let branch = suggested_branch
                .clone()
                .filter(|b| b.starts_with("feat/") || b.starts_with("feature/"))
                .unwrap_or_else(|| format!("feat/{slug}"));
            map.insert("branch_name".into(), json!(branch));
        }
        "refactorization" => {
            map.insert("refactor_goal".into(), json!(seed));
            map.insert(
                "refined_constraints".into(),
                json!("Despacho Kalma2 vía task-queue-manager; alcance acotado al goal."),
            );
            map.insert("refactor_name".into(), json!(slug.clone()));
            let branch = suggested_branch
                .clone()
                .unwrap_or_else(|| format!("refactor/{slug}"));
            map.insert("branch_name".into(), json!(branch));
        }
        _ => return Err(format!("proceso no despachable: {process}")),
    }
    let persist = resolve_child_persist_ref(
        repo,
        process,
        &slug,
        map.get("pbi_body").and_then(|v| v.as_str()),
    );
    map.insert("persist_ref".into(), json!(persist));
    Ok(Value::Object(map))
}

fn child_env_for_kalma2(
    correlation_id: Option<&str>,
    stop_after: Option<&str>,
) -> (Vec<(String, String)>, bool) {
    let mut env = Vec::new();
    let l2_skip = correlation_id.map(|s| !s.is_empty()).unwrap_or(false)
        && !env_truthy("SDDIA_TQM_FULL_CYCLE");
    if l2_skip {
        for key in [
            "SDDIA_LAB_SKIP_PBI_ARCHIVE",
            "SDDIA_LAB_SKIP_DELIVERY_CLOSE",
        ] {
            if std::env::var(key).is_err() {
                env.push((key.to_string(), "1".into()));
            }
        }
    }
    if let Some(sa) = stop_after.map(str::trim).filter(|s| !s.is_empty()) {
        if std::env::var("SDDIA_TQM_STOP_AFTER").is_err() {
            env.push(("SDDIA_TQM_STOP_AFTER".into(), sa.to_string()));
        }
    }
    (env, l2_skip)
}

fn dispatch_child(
    repo: &Path,
    process: &str,
    inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let task_text = inputs
        .get("task_text")
        .or_else(|| inputs.get("raw_text"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "task_text requerido para despacho Kalma2".to_string())?;
    let pbi = resolve_pbi_ref(inputs, task_text);
    let correlation_id = inputs
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let lock_identity = if let Some(ref pbi_ref) = pbi {
        resolve_lock_identity(repo, pbi_ref)?
    } else {
        None
    };
    let _sf_guard = if let Some(ref identity) = lock_identity {
        match try_acquire_single_flight(repo, identity, correlation_id)? {
            Some(g) => Some(g),
            None => {
                let lock_path = single_flight_dir(repo).join(format!("{}.lock", identity.lock_hex));
                let holder = read_holder_correlation_id(&lock_path);
                let proof_path =
                    persist_single_flight_hit_proof(repo, identity, holder.as_deref(), correlation_id)?;
                let orchestration = emit_single_flight_discarded_event(
                    repo,
                    identity,
                    holder.as_deref(),
                    correlation_id,
                )
                .ok();
                return Ok(single_flight_hit_envelope(
                    process,
                    correlation_id,
                    holder.as_deref(),
                    pbi.clone(),
                    identity,
                    &proof_path,
                    orchestration,
                ));
            }
        }
    } else {
        None
    };
    let child_inputs =
        build_child_inputs(repo, process, task_text, pbi.as_deref(), correlation_id)?;
    let pbi_loaded = child_inputs.get("pbi_body").is_some();
    // O2 Kaizen: PEC awaiting_agents antes del hijo — UI sondea sin cortar en initialized.
    let early_pec = if let Some(cid) = correlation_id {
        match thermodynamic::emit_initialized_pec(repo, process, cid) {
            Ok(seal) => Some(seal),
            Err(e) => {
                eprintln!("[TQM-EARLY-PEC] correlation={cid} process={process}: {e}");
                None
            }
        }
    } else {
        None
    };
    let extra_env = {
        let stop_after: Option<String> = inputs
            .get("stop_after")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .or_else(|| {
                std::env::var("SDDIA_TQM_STOP_AFTER")
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            });
        child_env_for_kalma2(correlation_id, stop_after.as_deref())
    };
    let (extra_env, l2_skip) = extra_env;
    let env_refs: Vec<(&str, &str)> = extra_env
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let child = invoke_process_full_with_env(repo, process, &child_inputs, &env_refs)?;
    let ok = child
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let status_code = child
        .get("status_code")
        .or_else(|| child.get("exit_code"))
        .and_then(|v| v.as_i64())
        .unwrap_or(if ok { 0 } else { 1 }) as i32;
    let mut child_data = child.get("data").cloned().unwrap_or(json!({}));
    if child_data
        .get("detached")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return Err(
            "tqm-single-flight: hijo detached con guard activo (invariante R1)".to_string(),
        );
    }
    if let Some(obj) = child_data.as_object_mut() {
        obj.insert("pbi_body_loaded".into(), json!(pbi_loaded));
    }
    let mut tqm_data = json!({
        "dispatched_process": process,
        "correlation_id": correlation_id,
        "pbi_ref": pbi,
        "early_pec": early_pec,
        "child": child_data,
        "handler": "task-queue-manager-kalma2",
    });
    if l2_skip {
        tqm_data["delivery_close"] = json!("skipped_l2");
    }
    Ok(OrchestratorEnvelope {
        success: ok && status_code == 0,
        status_code,
        data: Some(tqm_data),
        error: if ok && status_code == 0 {
            None
        } else {
            child
                .get("error")
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .or_else(|| Some("despacho hijo falló".into()))
        },
        execution_report: Some(json!({
            "process_name": "task-queue-manager",
            "phases": [{
                "phase_name": "Triaje Kalma2",
                "status": "executed",
                "handler": "task-queue-manager-kalma2",
                "dispatched_process": process,
            }, {
                "phase_name": "Despacho",
                "status": if ok && status_code == 0 { "executed" } else { "failed" },
                "child_process": process,
                "child_report": child.get("execution_report").cloned().unwrap_or(json!({})),
            }],
        })),
        exit_code: status_code,
    })
}

fn run_legacy(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let mut inputs = process_inputs.clone();
    let missing_tasks = inputs
        .get("tasks_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_none();
    if missing_tasks {
        if let Some(obj) = inputs.as_object_mut() {
            obj.insert("tasks_path".into(), json!("docs/todos"));
        }
    }
    let (canonical, process_def, phases) = load_process_def(repo, "task-queue-manager")?;
    residual_runner::run(repo, &canonical, &process_def, &phases, &inputs)
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let process = process_inputs
        .get("process")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("");
    if DISPATCHABLE.contains(&process) {
        return dispatch_child(repo, process, process_inputs);
    }
    run_legacy(repo, process_inputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::cli_detach;

    #[test]
    fn extract_pbi_with_spaces_and_emdash() {
        let text = "Inicia fix en docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (e6cbecb9032c).md por favor";
        let got = extract_pbi_path(text).expect("pbi");
        assert_eq!(
            got,
            "docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (e6cbecb9032c).md"
        );
    }

    #[test]
    fn extract_pbi_migrated_deuda_tecnica_paths() {
        let paths = [
            "docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md",
            "docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md",
            "docs/todos/pending/[DEUDA] Escaneo lineal de docs-todos en el resolutor de fractura — umbral de indexación.md",
        ];
        for path in paths {
            let text = format!("Despacha {path}");
            let got = extract_pbi_path(&text).expect("pending path must be extractable");
            assert_eq!(got, path);
        }
    }

    #[test]
    fn extract_pbi_ignores_inert_bucket_paths() {
        let inert_only = [
            "docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md",
            "docs/todos/kitchen/PBI-MULTI-LLM-ROUTER.md",
            "docs/todos/historias/[ARQUITECTURA] MVP.md",
        ];
        for path in inert_only {
            let text = format!("Ver {path}");
            assert!(
                extract_pbi_path(&text).is_none(),
                "inert bucket must not be dispatchable: {path}"
            );
        }
    }

    #[test]
    fn extract_pbi_prefers_pending_over_inert_when_both_present() {
        let text = "docs/todos/kitchen/x.md y docs/todos/pending/[DEUDA] Escaneo lineal de docs-todos en el resolutor de fractura — umbral de indexación.md";
        let got = extract_pbi_path(text).expect("pending wins");
        assert!(got.starts_with("docs/todos/pending/"));
    }

    #[test]
    fn pending_pbi_path_accepted_for_archive_gate() {
        // Paridad con phase_capsules feature-pbi-archive (rel.contains docs/todos/pending/)
        let rel = "docs/todos/pending/[DEUDA] Escaneo lineal de docs-todos en el resolutor de fractura — umbral de indexación.md";
        assert!(rel.contains("docs/todos/pending/"));
        let inert = "docs/todos/DeudaTecnica/[DEUDA] Escaneo lineal.md";
        assert!(!inert.contains("docs/todos/pending/"));
    }

    #[test]
    fn derive_slug_from_hash() {
        let pbi =
            "docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (e6cbecb9032c).md";
        assert_eq!(derive_slug(Some(pbi), "x"), "e6cbecb9032c");
    }

    #[test]
    fn build_bug_fix_inputs() {
        let repo = Path::new(".");
        let v = build_child_inputs(
            repo,
            "bug-fix",
            "semilla",
            Some("docs/todos/pending/[FIX] x (aabbccddeeff).md"),
            Some("corr-1"),
        )
        .unwrap();
        // Sin fichero real: seed = prompt; con fichero incluiría pbi_body.
        assert!(v["bug_summary"].as_str().unwrap().contains("semilla"));
        assert_eq!(v["branch_name"], "fix/aabbccddeeff");
        assert_eq!(v["correlation_id"], "corr-1");
    }

    #[test]
    fn load_pbi_body_reads_file() {
        let dir = std::env::temp_dir().join(format!("sddia-pbi-{}", Uuid::new_v4()));
        let rel = "docs/todos/pending/[FIX] demo (aabbccddeeff0011).md";
        let full = dir.join(rel);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(&full, "# PBI demo\n\nCuerpo del defecto.\n").unwrap();
        let body = load_pbi_body(&dir, rel).expect("body");
        assert!(body.contains("Cuerpo del defecto"));
        let v = build_child_inputs(&dir, "bug-fix", "inicia fix", Some(rel), None).unwrap();
        assert!(v.get("pbi_body").is_some());
        let summary = v["bug_summary"].as_str().unwrap();
        assert!(summary.contains("PBI adjunto"));
        assert!(summary.contains("Cuerpo del defecto"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn suggested_branch_from_pbi_frontmatter() {
        let dir = std::env::temp_dir().join(format!("sddia-pbi-sb-{}", Uuid::new_v4()));
        let rel = "docs/todos/pending/[FIX] execute-process — fallo (EV-AUD-005).md";
        let full = dir.join(rel);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(
            &full,
            "---\nsuggested_branch: fix/execute-process-phase-failure-propagation\ntype: bug-fix\n---\n\n# body\n",
        )
        .unwrap();
        let v = build_child_inputs(&dir, "bug-fix", "inicia proceso fix", Some(rel), None).unwrap();
        assert_eq!(
            v["branch_name"].as_str().unwrap(),
            "fix/execute-process-phase-failure-propagation"
        );
        assert_eq!(
            v["fix_name"].as_str().unwrap(),
            "execute-process-phase-failure-propagation"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn dispatchable_processes_not_in_cli_detach_allowlist() {
        let policy = cli_detach::DetachPolicy {
            foreground: false,
            force_detach: false,
            allowlist: vec!["pull-request-review".into()],
        };
        for process in DISPATCHABLE {
            assert!(
                !policy.should_detach(process),
                "{process} no debe estar en allowlist detach"
            );
        }
    }

    #[test]
    fn lock_key_stable_for_path_variants() {
        let dir = std::env::temp_dir().join(format!("sddia-sf-key-{}", Uuid::new_v4()));
        let rel = "docs/todos/pending/[FIX] demo.md";
        let full = dir.join(rel);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(&full, "# demo\n").unwrap();
        let a = resolve_lock_identity(&dir, rel)
            .unwrap()
            .expect("identity");
        let b = resolve_lock_identity(&dir, &format!("./{rel}"))
            .unwrap()
            .expect("identity");
        assert_eq!(a.lock_hex, b.lock_hex);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn lock_key_uses_document_id_across_pending_done() {
        let dir = std::env::temp_dir().join(format!("sddia-sf-docid-{}", Uuid::new_v4()));
        let pending = "docs/todos/pending/[KAIZEN] demo.md";
        let done = "docs/todos/done/[KAIZEN] demo.md";
        for rel in [pending, done] {
            let full = dir.join(rel);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(
                &full,
                "---\ndocument_id: PBI-DEMO-LOCK\n---\n\n# demo\n",
            )
            .unwrap();
        }
        let pending_id = resolve_lock_identity(&dir, pending)
            .unwrap()
            .expect("pending");
        let done_id = resolve_lock_identity(&dir, done).unwrap().expect("done");
        assert_eq!(pending_id.lock_hex, done_id.lock_hex);
        assert!(pending_id.lock_id.starts_with("id:"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn single_flight_same_pbi_different_correlation_ids() {
        let dir = std::env::temp_dir().join(format!("sddia-sf-pbi-{}", Uuid::new_v4()));
        let rel = "docs/todos/pending/[FIX] race.md";
        let full = dir.join(rel);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(&full, "---\ndocument_id: PBI-RACE\n---\n\n# race\n").unwrap();
        let identity = resolve_lock_identity(&dir, rel).unwrap().expect("identity");
        let g1 = try_acquire_single_flight(&dir, &identity, Some("cid-a"))
            .unwrap()
            .expect("first acquire");
        assert!(try_acquire_single_flight(&dir, &identity, Some("cid-b"))
            .unwrap()
            .is_none());
        drop(g1);
        assert!(try_acquire_single_flight(&dir, &identity, None)
            .unwrap()
            .is_some());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn single_flight_empty_recent_lock_not_purged() {
        let dir = std::env::temp_dir().join(format!("sddia-sf-toctou-{}", Uuid::new_v4()));
        let rel = "docs/todos/pending/[FIX] toctou.md";
        let full = dir.join(rel);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(&full, "# toctou\n").unwrap();
        let identity = resolve_lock_identity(&dir, rel).unwrap().expect("identity");
        let lock_dir = single_flight_dir(&dir);
        std::fs::create_dir_all(&lock_dir).unwrap();
        let lock_path = lock_dir.join(format!("{}.lock", identity.lock_hex));
        std::fs::write(&lock_path, "").unwrap();
        assert!(matches!(
            lock_occupancy(&lock_path).unwrap(),
            LockOccupancy::Held
        ));
        assert!(try_acquire_single_flight(&dir, &identity, Some("cid-b"))
            .unwrap()
            .is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn single_flight_second_acquire_hits_while_guard_lives() {
        let dir = std::env::temp_dir().join(format!("sddia-sf-{}", Uuid::new_v4()));
        let rel = "docs/todos/pending/[FIX] guard.md";
        let full = dir.join(rel);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(&full, "---\ndocument_id: PBI-GUARD\n---\n\n# guard\n").unwrap();
        let identity = resolve_lock_identity(&dir, rel).unwrap().expect("identity");
        let g1 = try_acquire_single_flight(&dir, &identity, Some("dcb9efed-2268-4298-8108-7a55cf4db323"))
            .unwrap()
            .expect("first acquire");
        assert!(try_acquire_single_flight(
            &dir,
            &identity,
            Some("cc6d6e2c-b84b-40f9-ac01-acff25ed252e")
        )
        .unwrap()
        .is_none());
        drop(g1);
        assert!(try_acquire_single_flight(&dir, &identity, Some("dcb9efed-2268-4298-8108-7a55cf4db323"))
            .unwrap()
            .is_some());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn refactor_child_keeps_suggested_branch_and_persist_ref() {
        let dir = std::env::temp_dir().join(format!("sddia-pbi-ref-{}", Uuid::new_v4()));
        std::fs::create_dir_all(dir.join("SddIA/core")).unwrap();
        std::fs::write(
            dir.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"featurePath":"docs/features","fixPath":"docs/fixes"}}"#,
        )
        .unwrap();
        let rel = "docs/todos/pending/[REFACTOR] Kalma2 (KALMA2-AUD).md";
        let full = dir.join(rel);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(
            &full,
            "---\nsuggested_branch: refactor/kalma2-phase-barrier-timeout-persist\npersist_ref_suggested: docs/features/kalma2-phase-barrier-timeout-persist\ntype: refactorization\n---\n\n# body\n",
        )
        .unwrap();
        let v =
            build_child_inputs(&dir, "refactorization", "inicia proceso", Some(rel), None).unwrap();
        assert_eq!(
            v["branch_name"].as_str().unwrap(),
            "refactor/kalma2-phase-barrier-timeout-persist"
        );
        assert_eq!(
            v["persist_ref"].as_str().unwrap(),
            "docs/features/kalma2-phase-barrier-timeout-persist"
        );
        assert!(!v["persist_ref"].as_str().unwrap().is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn refactor_child_persist_ref_fallback_from_cumulo() {
        let dir = std::env::temp_dir().join(format!("sddia-pbi-fb-{}", Uuid::new_v4()));
        std::fs::create_dir_all(dir.join("SddIA/core")).unwrap();
        std::fs::write(
            dir.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"featurePath":"docs/features","fixPath":"docs/fixes"}}"#,
        )
        .unwrap();
        let v = build_child_inputs(&dir, "refactorization", "semilla", None, None).unwrap();
        let persist = v["persist_ref"].as_str().unwrap();
        assert!(persist.starts_with("docs/features/"), "{persist}");
        assert!(!persist.is_empty());
        assert!(v["branch_name"].as_str().unwrap().starts_with("refactor/"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn child_env_l2_skip_declares_skipped_l2() {
        let prev_full = std::env::var("SDDIA_TQM_FULL_CYCLE").ok();
        let prev_a = std::env::var("SDDIA_LAB_SKIP_PBI_ARCHIVE").ok();
        let prev_d = std::env::var("SDDIA_LAB_SKIP_DELIVERY_CLOSE").ok();
        let prev_sa = std::env::var("SDDIA_TQM_STOP_AFTER").ok();
        std::env::remove_var("SDDIA_TQM_FULL_CYCLE");
        std::env::remove_var("SDDIA_LAB_SKIP_PBI_ARCHIVE");
        std::env::remove_var("SDDIA_LAB_SKIP_DELIVERY_CLOSE");
        std::env::remove_var("SDDIA_TQM_STOP_AFTER");
        let (vars, l2) = child_env_for_kalma2(Some("cid-1"), Some("design"));
        assert!(l2);
        assert!(vars
            .iter()
            .any(|(k, v)| k == "SDDIA_LAB_SKIP_DELIVERY_CLOSE" && v == "1"));
        assert!(vars
            .iter()
            .any(|(k, v)| k == "SDDIA_TQM_STOP_AFTER" && v == "design"));
        restore_env("SDDIA_TQM_FULL_CYCLE", prev_full);
        restore_env("SDDIA_LAB_SKIP_PBI_ARCHIVE", prev_a);
        restore_env("SDDIA_LAB_SKIP_DELIVERY_CLOSE", prev_d);
        restore_env("SDDIA_TQM_STOP_AFTER", prev_sa);
    }

    #[test]
    fn child_env_full_cycle_no_l2() {
        let prev_full = std::env::var("SDDIA_TQM_FULL_CYCLE").ok();
        std::env::set_var("SDDIA_TQM_FULL_CYCLE", "1");
        let (vars, l2) = child_env_for_kalma2(Some("cid-1"), None);
        assert!(!l2);
        assert!(!vars.iter().any(|(k, _)| k == "SDDIA_LAB_SKIP_DELIVERY_CLOSE"));
        restore_env("SDDIA_TQM_FULL_CYCLE", prev_full);
    }

    #[test]
    fn pr_in_task_text_does_not_imply_full_cycle() {
        let text = "Ejecuta planificación hasta haber forjado PR con test en verde.";
        assert!(text.contains("PR"));
        let prev_full = std::env::var("SDDIA_TQM_FULL_CYCLE").ok();
        std::env::remove_var("SDDIA_TQM_FULL_CYCLE");
        let (_, l2) = child_env_for_kalma2(Some("cid"), None);
        assert!(l2, "mencionar PR no deroga L2");
        restore_env("SDDIA_TQM_FULL_CYCLE", prev_full);
    }

    fn restore_env(key: &str, prev: Option<String>) {
        match prev {
            Some(v) => std::env::set_var(key, v),
            None => std::env::remove_var(key),
        }
    }
}
