//! Handler nativo: task-queue-manager — triaje Kalma2 + despacho de ciclo hijo.

use super::super::invoke_orchestrator::invoke_process_full_with_env;
use super::super::residual_runner;
use crate::core::resolver::load_process_def;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Map, Value};
use std::path::Path;
use uuid::Uuid;

const DISPATCHABLE: &[&str] = &["bug-fix", "feature", "refactorization"];

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
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

fn build_child_inputs(
    process: &str,
    task_text: &str,
    pbi_ref: Option<&str>,
    correlation_id: Option<&str>,
) -> Result<Value, String> {
    let slug = derive_slug(pbi_ref, task_text);
    let mut map = Map::new();
    if let Some(c) = correlation_id.filter(|s| !s.is_empty()) {
        map.insert("correlation_id".into(), json!(c));
    }
    if let Some(p) = pbi_ref.filter(|s| !s.is_empty()) {
        map.insert("pbi_ref".into(), json!(p));
    }
    map.insert("base_branch".into(), json!("main"));

    match process {
        "bug-fix" => {
            map.insert("bug_summary".into(), json!(task_text));
            map.insert("fix_name".into(), json!(slug.clone()));
            map.insert("branch_name".into(), json!(format!("fix/{slug}")));
        }
        "feature" => {
            map.insert("refined_requirements".into(), json!(task_text));
            map.insert("feature_name".into(), json!(slug.clone()));
            map.insert("branch_name".into(), json!(format!("feat/{slug}")));
        }
        "refactorization" => {
            map.insert("refactor_goal".into(), json!(task_text));
            map.insert(
                "refined_constraints".into(),
                json!("Despacho Kalma2 vía task-queue-manager; alcance acotado al goal."),
            );
            map.insert("refactor_name".into(), json!(slug.clone()));
            map.insert("branch_name".into(), json!(format!("feat/{slug}")));
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
    let child_inputs = build_child_inputs(
        process,
        task_text,
        pbi.as_deref(),
        correlation_id,
    )?;
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
    let child_data = child.get("data").cloned().unwrap_or(json!({}));
    Ok(OrchestratorEnvelope {
        success: ok && status_code == 0,
        status_code,
        data: Some(json!({
            "dispatched_process": process,
            "correlation_id": correlation_id,
            "pbi_ref": pbi,
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
        let v = build_child_inputs(
            "bug-fix",
            "semilla",
            Some("docs/todos/pending/[FIX] x (aabbccddeeff).md"),
            Some("corr-1"),
        )
        .unwrap();
        assert_eq!(v["bug_summary"], "semilla");
        assert_eq!(v["branch_name"], "fix/aabbccddeeff");
        assert_eq!(v["correlation_id"], "corr-1");
    }
}
