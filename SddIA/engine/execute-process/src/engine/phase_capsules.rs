//! Handlers de fase con cápsulas skill/tool/action (P5).

use super::capsules::{invoke_action, invoke_git_manager, invoke_shell_executor, invoke_tool};
use regex::Regex;
use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

static GH_PR_URL_RE: OnceLock<Regex> = OnceLock::new();

fn gh_pr_url_re() -> &'static Regex {
    GH_PR_URL_RE.get_or_init(|| {
        Regex::new(r"https://github\.com/[^\s/]+/[^\s/]+/pull/\d+").expect("gh pr url regex")
    })
}

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_gh_pr_url(stdout: &str) -> Option<String> {
    for line in stdout.lines().rev() {
        if let Some(m) = gh_pr_url_re().find(line) {
            return Some(m.as_str().to_string());
        }
    }
    gh_pr_url_re()
        .find(stdout)
        .map(|m| m.as_str().to_string())
}

fn delivery_pr_title(inputs: &Value) -> String {
    if let Some(title) = str_field(inputs, "pr_title") {
        return title;
    }
    if let Some(branch) = str_field(inputs, "branch_name") {
        return format!("feat: {branch}");
    }
    "feat: delivery-close-cycle".into()
}

fn run_eda_audit_scan(repo: &Path) -> Result<Value, String> {
    let script = repo.join("SddIA/scripts/qa/audit-entity-eda-coverage.py");
    if !script.is_file() {
        return Err(format!("audit script ausente: {}", script.display()));
    }
    let py = std::env::var("PYTHON").unwrap_or_else(|_| "python3".into());
    let output = Command::new(&py)
        .arg(&script)
        .args(["--scan", "--json"])
        .current_dir(repo)
        .output()
        .map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.trim();
    if line.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    serde_json::from_str(line).map_err(|e| format!("JSON audit EDA: {e}"))
}

fn backfill_manifest_active(repo: &Path, persist_ref: Option<&str>) -> bool {
    let Some(persist_ref) = persist_ref.filter(|s| !s.is_empty()) else {
        return false;
    };
    let manifest = repo.join(persist_ref).join("backfill-manifest.json");
    let Ok(text) = std::fs::read_to_string(&manifest) else {
        return false;
    };
    let Ok(data) = serde_json::from_str::<Value>(&text) else {
        return false;
    };
    data.get("correlation_id").is_some() && data.get("merkle_anchored") != Some(&json!(true))
}

pub fn capsule_eda_genomic_audit_gate(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    let report = run_eda_audit_scan(repo)?;
    let orphan_count = report
        .get("orphan_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as i64;
    if let Some(obj) = state.as_object_mut() {
        obj.insert("eda_audit".into(), report.clone());
    }
    let persist_ref = str_field(inputs, "persist_ref");
    let (verdict, noise) = if orphan_count > 0
        && backfill_manifest_active(repo, persist_ref.as_deref())
    {
        ("warn", Some("backfill Fase C en curso"))
    } else if orphan_count > 0 {
        ("block", Some("Ruido de Sistema"))
    } else {
        ("pass", None)
    };
    let mut entry = json!({
        "status": if verdict == "block" { "blocked" } else { "executed" },
        "handler": "eda-genomic-audit",
        "argos_verdict": verdict,
        "orphan_count": orphan_count,
    });
    if let Some(n) = noise {
        entry["argos_noise"] = json!(n);
    }
    Ok(entry)
}

pub fn capsule_delivery_snapshot_final_with_repo(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    if env_truthy("SDDIA_LAB_SKIP_SNAPSHOT") {
        return Ok(json!({
            "status": "executed",
            "handler": "delivery-snapshot-final",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_SNAPSHOT",
        }));
    }
    let branch = str_field(inputs, "branch_name")
        .ok_or("branch_name es obligatorio para Snapshot final")?;
    let data = invoke_git_manager(repo, "get_last_commit", &json!({"ref": branch}))?;
    let commit_hash = data
        .get("commitHash")
        .or_else(|| data.get("commit_hash"))
        .cloned();
    if let Some(obj) = state.as_object_mut() {
        if let Some(h) = &commit_hash {
            obj.insert("snapshot_commit_hash".into(), h.clone());
        }
    }
    Ok(json!({
        "status": "executed",
        "handler": "delivery-snapshot-final",
        "commit_hash": commit_hash,
        "branch": branch,
    }))
}

pub fn capsule_delivery_impact_assessment(inputs: &Value, _state: &mut Value) -> Value {
    if env_truthy("SDDIA_LAB_SKIP_IMPACT_ASSESSMENT") {
        return json!({
            "status": "skipped",
            "handler": "delivery-impact-assessment",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_IMPACT_ASSESSMENT",
        });
    }
    if inputs.get("source_process").and_then(|v| v.as_str()) != Some("feature") {
        return json!({
            "status": "skipped",
            "handler": "delivery-impact-assessment",
            "skipped": true,
            "reason": "source_process != feature",
        });
    }
    json!({
        "status": "executed",
        "handler": "delivery-impact-assessment",
        "impact": "none",
        "sddia_paths": [],
        "note": "git diff omitido en stub Rust; paridad lab vía skip",
    })
}

pub fn capsule_delivery_remote_push(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    let branch = str_field(inputs, "branch_name")
        .ok_or("branch_name es obligatorio para Publicación remota")?;
    if env_truthy("SDDIA_LAB_SKIP_GIT_PUSH") {
        return Ok(json!({
            "status": "executed",
            "handler": "delivery-remote-push",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_GIT_PUSH",
        }));
    }
    let data = invoke_git_manager(
        repo,
        "push",
        &json!({"remote": "origin", "branch": branch, "force": false}),
    )?;
    if let Some(obj) = state.as_object_mut() {
        obj.insert("delivery_push".into(), data.clone());
    }
    Ok(json!({
        "status": "executed",
        "handler": "delivery-remote-push",
        "result": data,
    }))
}

pub fn capsule_delivery_gh_pr(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    let branch = str_field(inputs, "branch_name")
        .ok_or("branch_name es obligatorio para Apertura en forja")?;
    if let Some(preset) = str_field(inputs, "pr_url") {
        if let Some(obj) = state.as_object_mut() {
            obj.insert("pr_url".into(), json!(preset));
        }
        return Ok(json!({
            "status": "executed",
            "handler": "delivery-gh-pr",
            "pr_url": preset,
            "simulated": true,
            "source": "inputs.pr_url",
        }));
    }
    if env_truthy("SDDIA_LAB_SIMULATE_GH_PR") {
        let pr_url = format!(
            "https://github.com/lab-simulated/SddIA/pull/0-{}",
            branch.replace('/', "-")
        );
        if let Some(obj) = state.as_object_mut() {
            obj.insert("pr_url".into(), json!(pr_url));
        }
        return Ok(json!({
            "status": "executed",
            "handler": "delivery-gh-pr",
            "pr_url": pr_url,
            "simulated": true,
        }));
    }
    let target = str_field(inputs, "target_branch").unwrap_or_else(|| "main".into());
    let title = delivery_pr_title(inputs);
    let mut args = vec![
        "pr".into(),
        "create".into(),
        "--title".into(),
        title,
        "--head".into(),
        branch.clone(),
        "--base".into(),
        target,
    ];
    if let Some(body) = str_field(inputs, "pr_body") {
        args.push("--body".into());
        args.push(body);
    } else {
        args.push("--fill".into());
    }
    let data = invoke_shell_executor(repo, "gh", &args)?;
    let stdout = data.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    let stderr = data.get("stderr").and_then(|v| v.as_str()).unwrap_or("");
    let mut pr_url = parse_gh_pr_url(stdout).or_else(|| parse_gh_pr_url(stderr));
    if pr_url.is_none() {
        let view = invoke_shell_executor(
            repo,
            "gh",
            &[
                "pr".into(),
                "view".into(),
                branch.clone(),
                "--json".into(),
                "url".into(),
                "-q".into(),
                ".url".into(),
            ],
        )?;
        pr_url = view
            .get("stdout")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
    }
    let pr_url = pr_url.ok_or("no se pudo resolver pr_url desde gh")?;
    if let Some(obj) = state.as_object_mut() {
        obj.insert("pr_url".into(), json!(pr_url));
    }
    Ok(json!({
        "status": "executed",
        "handler": "delivery-gh-pr",
        "pr_url": pr_url,
        "gh_stdout": stdout.chars().take(500).collect::<String>(),
    }))
}

pub fn capsule_delivery_emit_presented(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    let branch = str_field(inputs, "branch_name")
        .ok_or("branch_name es obligatorio para Sello Presentación ECST")?;
    let mut action_inputs = json!({
        "branch": branch,
        "status": inputs.get("status").unwrap_or(&json!("presented")),
        "emitter_agent": "delivery-close-cycle",
    });
    let pr_url = state
        .get("pr_url")
        .and_then(|v| v.as_str())
        .or_else(|| inputs.get("pr_url").and_then(|v| v.as_str()))
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if let Some(url) = pr_url {
        action_inputs["pr_url"] = json!(url);
    }
    if let Some(corr) = str_field(inputs, "correlation_id") {
        action_inputs["correlation_id"] = json!(corr);
    }
    let seal = invoke_action(repo, "emit-pr-presented-event", &action_inputs)?;
    let handoff = state.get("handoff").cloned().unwrap_or(json!({}));
    let mut merged_handoff = handoff;
    if let Some(s) = seal.as_object() {
        if let Some(m) = merged_handoff.as_object_mut() {
            for (k, v) in s {
                m.insert(k.clone(), v.clone());
            }
        }
    }
    if let Some(obj) = state.as_object_mut() {
        obj.insert("handoff".into(), merged_handoff);
        if let Some(eid) = seal.get("event_id") {
            obj.insert("event_id".into(), eid.clone());
        }
        if let Some(tp) = seal.get("target_path") {
            obj.insert("target_path".into(), tp.clone());
        }
    }
    let mut entry = json!({
        "status": "executed",
        "handler": "delivery-emit-pr-presented",
    });
    if let Some(obj) = seal.as_object() {
        for (k, v) in obj {
            entry[k.clone()] = v.clone();
        }
    }
    Ok(entry)
}

pub fn capsule_delivery_local_hygiene(_inputs: &Value, _state: &mut Value) -> Value {
    if env_truthy("SDDIA_LAB_SKIP_HIGIENE") {
        return json!({
            "status": "executed",
            "handler": "delivery-local-hygiene",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_HIGIENE",
            "closed_branch": null,
            "note": "higiene parcial en laboratorio; delete requiere SDDIA_LAB_DELETE_FEATURE_BRANCH",
        });
    }
    json!({
        "status": "executed",
        "handler": "delivery-local-hygiene",
        "closed_branch": null,
        "note": "higiene parcial en laboratorio; delete requiere SDDIA_LAB_DELETE_FEATURE_BRANCH",
    })
}

pub fn execute_delivery_close_phase(
    repo: &Path,
    phase_name: &str,
    inputs: &Value,
    state: &mut Value,
) -> Option<Result<Value, String>> {
    match phase_name {
        "Snapshot final" => Some(capsule_delivery_snapshot_final_with_repo(repo, inputs, state)),
        "Impacto SddIA condicional" => Some(Ok(capsule_delivery_impact_assessment(inputs, state))),
        "Publicación remota" => Some(capsule_delivery_remote_push(repo, inputs, state)),
        "Apertura en forja" => Some(capsule_delivery_gh_pr(repo, inputs, state)),
        "Sello Presentación ECST" => Some(capsule_delivery_emit_presented(repo, inputs, state)),
        "Higiene local" => Some(Ok(capsule_delivery_local_hygiene(inputs, state))),
        _ => None,
    }
}

fn delegate_prefix(d: &str) -> Option<(&str, &str)> {
    for prefix in ["skill:", "tool:"] {
        if let Some(name) = d.strip_prefix(prefix) {
            if !name.is_empty() {
                return Some((prefix.trim_end_matches(':'), name));
            }
        }
    }
    None
}

/// Intenta invocar delegados skill:/tool: con cápsula resoluble (P5).
pub fn try_invoke_delegates(repo: &Path, delegates: &[Value], inputs: &Value) -> Option<Value> {
    let mut last_err: Option<String> = None;
    for d in delegates {
        let s = d.as_str()?;
        if s == "skill:git-manager" {
            continue;
        }
        let Some((kind, name)) = delegate_prefix(s) else {
            continue;
        };
        match invoke_tool(repo, name, inputs) {
            Ok(body) => {
                return Some(json!({
                    "status": "executed",
                    "handler": format!("capsule-{kind}-{name}"),
                    "capsule_result": body,
                }));
            }
            Err(e) if e.contains("no encontrada") => continue,
            Err(e) => {
                last_err = Some(e);
            }
        }
    }
    last_err.map(|e| {
        json!({
            "status": "failed",
            "handler": "capsule-invoke",
            "error": e,
        })
    })
}
