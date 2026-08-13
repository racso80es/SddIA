//! Handler nativo: task-queue-manager — triaje Kalma2 + despacho de ciclo hijo.

use super::super::invoke_orchestrator::invoke_process_full_with_env;
use super::super::residual_runner;
use super::super::thermodynamic;
use crate::core::resolver::load_process_def;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Map, Value};
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

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

fn lock_pid_alive(path: &Path) -> bool {
    let Ok(raw) = fs::read_to_string(path) else {
        return false;
    };
    let Ok(pid) = raw.trim().parse::<u32>() else {
        return false;
    };
    Path::new(&format!("/proc/{pid}")).exists()
}

fn try_acquire_single_flight(
    repo: &Path,
    correlation_id: &str,
) -> Result<Option<SingleFlightGuard>, String> {
    let dir = single_flight_dir(repo);
    fs::create_dir_all(&dir).map_err(|e| format!("tqm-single-flight mkdir: {e}"))?;
    let path = dir.join(format!("{correlation_id}.lock"));
    for _ in 0..3 {
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut f) => {
                let _ = writeln!(f, "{}", std::process::id());
                return Ok(Some(SingleFlightGuard { path }));
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                if lock_pid_alive(&path) {
                    return Ok(None);
                }
                let _ = fs::remove_file(&path);
            }
            Err(e) => return Err(format!("tqm-single-flight lock: {e}")),
        }
    }
    Ok(None)
}

fn single_flight_hit_envelope(
    process: &str,
    correlation_id: &str,
    pbi: Option<String>,
) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "dispatched_process": process,
            "correlation_id": correlation_id,
            "pbi_ref": pbi,
            "single_flight_hit": true,
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
                "reason": "single-flight correlation_id",
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
                .unwrap_or_else(|| format!("feat/{slug}"));
            map.insert("branch_name".into(), json!(branch));
        }
        _ => return Err(format!("proceso no despachable: {process}")),
    }
    Ok(Value::Object(map))
}

fn child_env_for_kalma2(correlation_id: Option<&str>) -> Vec<(String, String)> {
    let mut env = Vec::new();
    if correlation_id.map(|s| !s.is_empty()).unwrap_or(false) && !env_truthy("SDDIA_TQM_FULL_CYCLE") {
        for key in ["SDDIA_LAB_SKIP_PBI_ARCHIVE", "SDDIA_LAB_SKIP_DELIVERY_CLOSE"] {
            if std::env::var(key).is_err() {
                env.push((key.to_string(), "1".into()));
            }
        }
    }
    env
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
    let _sf_guard = if let Some(cid) = correlation_id {
        match try_acquire_single_flight(repo, cid)? {
            Some(g) => Some(g),
            None => return Ok(single_flight_hit_envelope(process, cid, pbi.clone())),
        }
    } else {
        None
    };
    let child_inputs = build_child_inputs(
        repo,
        process,
        task_text,
        pbi.as_deref(),
        correlation_id,
    )?;
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
    let extra_env = child_env_for_kalma2(correlation_id);
    let env_refs: Vec<(&str, &str)> = extra_env
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let child = invoke_process_full_with_env(repo, process, &child_inputs, &env_refs)?;
    let ok = child.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
    let status_code = child
        .get("status_code")
        .or_else(|| child.get("exit_code"))
        .and_then(|v| v.as_i64())
        .unwrap_or(if ok { 0 } else { 1 }) as i32;
    let mut child_data = child.get("data").cloned().unwrap_or(json!({}));
    if let Some(obj) = child_data.as_object_mut() {
        obj.insert("pbi_body_loaded".into(), json!(pbi_loaded));
    }
    Ok(OrchestratorEnvelope {
        success: ok && status_code == 0,
        status_code,
        data: Some(json!({
            "dispatched_process": process,
            "correlation_id": correlation_id,
            "pbi_ref": pbi,
            "early_pec": early_pec,
            "child": child_data,
            "handler": "task-queue-manager-kalma2",
        })),
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
    fn derive_slug_from_hash() {
        let pbi = "docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (e6cbecb9032c).md";
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
    fn single_flight_second_acquire_hits_while_guard_lives() {
        let dir = std::env::temp_dir().join(format!("sddia-sf-{}", Uuid::new_v4()));
        let cid = "dcb9efed-2268-4298-8108-7a55cf4db323";
        let g1 = try_acquire_single_flight(&dir, cid)
            .unwrap()
            .expect("first acquire");
        assert!(try_acquire_single_flight(&dir, cid).unwrap().is_none());
        drop(g1);
        assert!(try_acquire_single_flight(&dir, cid).unwrap().is_some());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
