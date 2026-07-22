//! Handlers de fase con cápsulas skill/tool/action (P5).

use super::capsules::{invoke_action, invoke_git_manager, invoke_shell_executor, invoke_tool};
use regex::Regex;
use serde_json::{json, Value};
use std::path::Path;
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
    super::eda_coverage::scan_orphans(repo)
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
        .filter(|s| !s.is_empty())
        .ok_or(
            "pr_url obligatorio para Sello Presentación ECST (evita PullRequest_Presented incompleto → DLT Argos/IOTA)",
        )?;
    action_inputs["pr_url"] = json!(pr_url);
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

fn build_capsule_payload(kind: &str, name: &str, inputs: &Value) -> Value {
    if kind == "tool" && name == "io-choke" {
        return json!({
            "workspace_path": inputs.get("workspace_path"),
            "target_file": inputs.get("target_file").unwrap_or(&json!(".capsule-smoke-target")),
        });
    }
    inputs.clone()
}

/// Intenta invocar delegados skill:/tool: con cápsula resoluble (P5).
/// Si hay `di_bindings`, inyecta `di_binding` en el payload stdin (R2).
/// Post-invoke: valida payload real vs schema contrato (R8).
pub fn try_invoke_delegates(
    repo: &Path,
    delegates: &[Value],
    inputs: &Value,
    di_bindings: &[super::capability_di_resolver::ResolvedBinding],
    process_name: &str,
    phase_name: &str,
) -> Option<Value> {
    let mut capsule_missing = false;
    let mut last_err: Option<String> = None;
    for d in delegates {
        let s = d.as_str()?;
        if s == "skill:git-manager" {
            continue;
        }
        let Some((kind, name)) = delegate_prefix(s) else {
            continue;
        };
        let base = build_capsule_payload(kind, name, inputs);
        let payload =
            super::capability_di_resolver::merge_first_di_binding(&base, di_bindings);
        match invoke_tool(repo, name, &payload) {
            Ok(body) => {
                if let Some(b) = di_bindings.first() {
                    if let Err(out_err) =
                        super::capability_di_output_validator::validate_output_payload(
                            repo, b, &body,
                        )
                    {
                        super::capability_di_output_validator::write_output_dead_letter(
                            repo,
                            &out_err,
                            phase_name,
                            process_name,
                            &b.contract,
                        );
                        return Some(json!({
                            "status": "failed",
                            "handler": "capability-di-output-validator",
                            "error": out_err.message,
                            "output_validator_code": out_err.code.as_str(),
                        }));
                    }
                }
                let mut out = json!({
                    "status": "executed",
                    "handler": format!("capsule-{kind}-{name}"),
                    "capsule_result": body,
                });
                if let Some(b) = di_bindings.first() {
                    out["di_binding"] = super::capability_di_resolver::di_binding_object(b);
                }
                return Some(out);
            }
            Err(e) if e.contains("no encontrada") => {
                capsule_missing = true;
            }
            Err(e) => {
                last_err = Some(e);
            }
        }
    }
    if let Some(e) = last_err {
        return Some(json!({
            "status": "failed",
            "handler": "capsule-invoke",
            "error": e,
        }));
    }
    if capsule_missing {
        return None;
    }
    None
}

fn delivery_pr_title_feature(inputs: &Value) -> String {
    if let Some(title) = str_field(inputs, "pr_title") {
        return title;
    }
    if let Some(branch) = str_field(inputs, "branch_name") {
        return format!("feat: {branch}");
    }
    "feat: delivery-close-cycle".into()
}

pub fn capsule_feature_pbi_archive(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    if env_truthy("SDDIA_LAB_SKIP_PBI_ARCHIVE") {
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_PBI_ARCHIVE",
        }));
    }
    let Some(persist_ref) = str_field(inputs, "persist_ref") else {
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "skipped": true,
            "reason": "persist_ref ausente",
        }));
    };
    let val_path = repo.join(&persist_ref).join("validacion.md");
    if !val_path.is_file() {
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "skipped": true,
            "reason": "validacion.md ausente",
        }));
    }
    let fm = crate::core::parser::parse_frontmatter(&val_path)?;
    let global_v = fm
        .get("global")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_uppercase();
    if global_v != "APTO" {
        let g = fm.get("global").and_then(|v| v.as_str()).unwrap_or("?");
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "skipped": true,
            "reason": format!("global={g}"),
        }));
    }
    let archived = fm.get("pbi_archived");
    let ok_archived = matches!(archived, Some(v) if v.as_bool() == Some(true))
        || archived
            .and_then(|v| v.as_str())
            .map(|s| matches!(s.to_lowercase().as_str(), "true" | "1"))
            .unwrap_or(false);
    if !ok_archived {
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "skipped": true,
            "reason": "pbi_archived != true",
        }));
    }
    let pbi_path = if let Some(todo) = str_field(inputs, "related_todo") {
        let p = repo.join(todo.replace('\\', "/"));
        if p.is_file() { Some(p) } else { None }
    } else {
        let objectives = repo.join(&persist_ref).join("objectives.md");
        if objectives.is_file() {
            let ofm = crate::core::parser::parse_frontmatter(&objectives)?;
            if let Some(rel) = ofm.get("related_todo").and_then(|v| v.as_str()) {
                let p = repo.join(rel.replace('\\', "/"));
                if p.is_file() { Some(p) } else { None }
            } else {
                None
            }
        } else {
            None
        }
    };
    let Some(pbi_path) = pbi_path else {
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "skipped": true,
            "reason": "related_todo no resuelto",
        }));
    };
    let rel = pbi_path
        .strip_prefix(repo)
        .unwrap_or(&pbi_path)
        .to_string_lossy()
        .replace('\\', "/");
    if rel.starts_with("docs/todos/done/") {
        if let Some(obj) = state.as_object_mut() {
            obj.insert("pbi_archived_path".into(), json!(rel));
        }
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "already_archived": true,
            "pbi_path": rel,
        }));
    }
    if !rel.contains("docs/todos/pending/") {
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "skipped": true,
            "reason": format!("PBI fuera de pending/: {rel}"),
        }));
    }
    let done_dir = repo.join("docs/todos/done");
    std::fs::create_dir_all(&done_dir).map_err(|e| e.to_string())?;
    let dest = done_dir.join(pbi_path.file_name().ok_or("nombre PBI inválido")?);
    if dest.is_file() {
        let dest_rel = dest
            .strip_prefix(repo)
            .unwrap_or(&dest)
            .to_string_lossy()
            .replace('\\', "/");
        if let Some(obj) = state.as_object_mut() {
            obj.insert("pbi_archived_path".into(), json!(dest_rel));
        }
        return Ok(json!({
            "status": "executed",
            "handler": "feature-pbi-archive",
            "already_archived": true,
            "pbi_path": dest_rel,
        }));
    }
    std::fs::rename(&pbi_path, &dest).map_err(|e| e.to_string())?;
    let dest_rel = dest
        .strip_prefix(repo)
        .unwrap_or(&dest)
        .to_string_lossy()
        .replace('\\', "/");
    if let Some(obj) = state.as_object_mut() {
        obj.insert("pbi_archived_path".into(), json!(dest_rel));
    }
    Ok(json!({
        "status": "executed",
        "handler": "feature-pbi-archive",
        "archived": true,
        "pbi_path": dest_rel,
    }))
}

pub fn capsule_feature_invoke_delivery_close(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    if env_truthy("SDDIA_LAB_SKIP_DELIVERY_CLOSE") {
        return Ok(json!({
            "status": "executed",
            "handler": "feature-delivery-close",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_DELIVERY_CLOSE",
        }));
    }
    let branch = str_field(inputs, "branch_name").or_else(|| {
        state
            .get("workspace")
            .and_then(|w| w.get("branch_name"))
            .and_then(|v| v.as_str())
            .map(str::to_string)
    });
    let persist_ref = str_field(inputs, "persist_ref").or_else(|| {
        state
            .get("workspace")
            .and_then(|w| w.get("persist_ref"))
            .and_then(|v| v.as_str())
            .map(str::to_string)
    });
    let branch = branch.ok_or("branch_name es obligatorio para Cierre de entrega")?;
    let persist_ref = persist_ref.ok_or("persist_ref es obligatorio para Cierre de entrega")?;
    let mut child_inputs = json!({
        "source_process": "feature",
        "persist_ref": persist_ref,
        "branch_name": branch,
        "pr_title": delivery_pr_title_feature(inputs),
        "target_branch": inputs.get("target_branch").or(inputs.get("base_branch")).unwrap_or(&json!("main")),
    });
    if let Some(body) = str_field(inputs, "pr_body") {
        child_inputs["pr_body"] = json!(body);
    }
    if let Some(url) = str_field(inputs, "pr_url") {
        child_inputs["pr_url"] = json!(url);
    }
    let data =
        super::invoke_orchestrator::invoke_process(repo, "delivery-close-cycle", &child_inputs)?;
    if let Some(obj) = state.as_object_mut() {
        for key in [
            "pr_url",
            "event_id",
            "target_path",
            "closed_branch",
            "snapshot_commit_hash",
        ] {
            if let Some(v) = data.get(key) {
                obj.insert(key.into(), v.clone());
            }
        }
        obj.insert("delivery_close".into(), data.clone());
    }
    Ok(json!({
        "status": "executed",
        "handler": "feature-delivery-close",
        "child_process": "delivery-close-cycle",
        "delivery_close": data,
    }))
}

pub fn execute_feature_phase(
    repo: &Path,
    phase_name: &str,
    inputs: &Value,
    state: &mut Value,
) -> Option<Result<Value, String>> {
    match phase_name {
        "Cierre documental en rama" => Some(capsule_feature_pbi_archive(repo, inputs, state)),
        "Cierre de entrega" => Some(capsule_feature_invoke_delivery_close(repo, inputs, state)),
        _ => None,
    }
}

#[cfg(test)]
mod emit_presented_tests {
    use super::*;
    use serde_json::json;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        let mut here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        here.pop();
        here.pop();
        here.pop();
        here
    }

    #[test]
    fn emit_presented_rejects_missing_pr_url() {
        let repo = repo_root();
        let inputs = json!({"branch_name": "fix/demo"});
        let mut state = json!({});
        let err = capsule_delivery_emit_presented(&repo, &inputs, &mut state)
            .expect_err("must require pr_url");
        assert!(err.contains("pr_url obligatorio"), "got: {err}");
    }
}
