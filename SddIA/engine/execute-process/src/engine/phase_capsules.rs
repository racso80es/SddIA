//! Handlers de fase con cápsulas skill/tool/action (P5).

use super::capsules::{
    invoke_action, invoke_git_manager, invoke_shell_executor, invoke_tool_capsule_json,
    unwrap_tool_body,
};
use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
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

/// Paridad con `assert_safe_token` de `shell-executor` (sin invocar la cápsula).
fn is_shell_token_safe(token: &str) -> bool {
    !token.chars().any(|c| "\n\r;|><`".contains(c))
        && !token.contains("&&")
        && !token.contains("$(")
        && !token.contains('&')
}

fn classify_delivery_error(err: &str) -> Option<&'static str> {
    if err.contains("forbidden shell metacharacters")
        || err.contains("no pasa preflight de token seguro")
    {
        Some("PR_BODY_METACHAR")
    } else {
        None
    }
}

fn format_delivery_error(error_code: &str, msg: &str) -> String {
    format!("[{error_code}] {msg}")
}

fn delivery_phase_failed(handler: &str, error_code: &str, msg: &str) -> Value {
    json!({
        "status": "failed",
        "handler": handler,
        "error_code": error_code,
        "error": format_delivery_error(error_code, msg),
    })
}

fn git_porcelain_stdout(data: &Value) -> String {
    data.get("gitStdout")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn git_commit_hash(data: &Value) -> Option<Value> {
    data.get("commitHash")
        .or_else(|| data.get("commit_hash"))
        .cloned()
}

use super::git_porcelain::unescape_git_cquoted_path;
fn parse_porcelain_paths(git_stdout: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in git_stdout.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let raw = if let Some(rest) = line.strip_prefix("?? ") {
            rest.trim()
        } else if line.len() >= 4 {
            let rest = line[3..].trim();
            if let Some(idx) = rest.find(" -> ") {
                rest[idx + 4..].trim()
            } else {
                rest
            }
        } else {
            continue;
        };
        let path = unescape_git_cquoted_path(raw);
        if !path.is_empty() {
            paths.push(path);
        }
    }
    let mut seen = HashSet::new();
    paths
        .into_iter()
        .filter(|p| seen.insert(p.clone()))
        .collect()
}

fn should_preserve_untracked_todos(path: &str, pbi_ref: Option<&str>) -> bool {
    let norm = path.trim_start_matches("./");
    if !norm.starts_with("docs/todos/") {
        return false;
    }
    if let Some(pbi) = pbi_ref {
        if norm == pbi.trim_start_matches("./") {
            return false;
        }
    }
    true
}

fn porcelain_untracked_paths(git_stdout: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    for line in git_stdout.lines() {
        if let Some(rest) = line.strip_prefix("?? ") {
            for p in parse_porcelain_paths(&format!("?? {rest}")) {
                set.insert(p);
            }
        }
    }
    set
}

fn filter_snapshot_commit_files(
    files: Vec<String>,
    porcelain: &str,
    pbi_ref: Option<&str>,
) -> Vec<String> {
    let untracked = porcelain_untracked_paths(porcelain);
    files
        .into_iter()
        .filter(|p| !untracked.contains(p) || !should_preserve_untracked_todos(p, pbi_ref))
        .collect()
}

fn porcelain_excluding_preserved_untracked_todos(porcelain: &str, pbi_ref: Option<&str>) -> String {
    porcelain
        .lines()
        .filter(|line| {
            if let Some(rest) = line.strip_prefix("?? ") {
                if let Some(path) = parse_porcelain_paths(&format!("?? {rest}")).first() {
                    if should_preserve_untracked_todos(path, pbi_ref) {
                        return false;
                    }
                }
            }
            true
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn resolve_pr_body_file_dir(repo: &Path, inputs: &Value, state: &Value) -> Result<PathBuf, String> {
    if let Some(persist_ref) = str_field(inputs, "persist_ref") {
        return Ok(repo.join(persist_ref).join(".tmp"));
    }
    let execution_id = str_field(inputs, "execution_id").or_else(|| {
        state
            .get("execution_id")
            .and_then(|v| v.as_str())
            .map(str::to_string)
    });
    if let Some(eid) = execution_id {
        return Ok(repo
            .join(".SddIA/workspaces/delivery-close-cycle")
            .join(eid));
    }
    Err("persist_ref o execution_id requeridos cuando pr_body está presente".into())
}

fn write_pr_body_file(dir: &Path, content: &str) -> Result<String, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let file_path = dir.join("pr-body.md");
    std::fs::write(&file_path, content).map_err(|e| e.to_string())?;
    let abs = file_path
        .canonicalize()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();
    if !is_shell_token_safe(&abs) {
        return Err(format!(
            "path --body-file no pasa preflight de token seguro: {abs}"
        ));
    }
    Ok(abs)
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

fn resolve_sddia_qa_bin(repo: &Path) -> Result<PathBuf, String> {
    for rel in [
        "SddIA/target/debug/sddia-qa",
        "SddIA/target/release/sddia-qa",
    ] {
        let p = repo.join(rel);
        if p.is_file() {
            return Ok(p);
        }
    }
    Err("sddia-qa no encontrado (compilar: cd SddIA && cargo build -p sddia-qa)".into())
}

pub fn capsule_evolution_audit_gate(
    repo: &Path,
    _inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    let bin = resolve_sddia_qa_bin(repo)?;
    let out = std::process::Command::new(&bin)
        .args(["gate-evolution", "--json", "--range"])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("gate-evolution spawn: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let body: Value = serde_json::from_str(stdout.trim())
        .map_err(|e| format!("gate-evolution JSON inválido: {e}"))?;
    if let Some(obj) = state.as_object_mut() {
        obj.insert("evolution_gate".into(), body.clone());
    }
    let success = body.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
    let exit_code = body
        .get("exitCode")
        .and_then(|v| v.as_i64())
        .unwrap_or(if success { 0 } else { 2 }) as i32;
    let skipped = body.pointer("/result/skipped").is_some();
    if success || skipped {
        return Ok(json!({
            "status": "executed",
            "handler": "evolution-audit",
            "exitCode": exit_code,
            "skipped": skipped,
            "base_resolution": body.pointer("/result/base_resolution").cloned(),
        }));
    }
    Ok(json!({
        "status": "blocked",
        "handler": "evolution-audit",
        "exitCode": exit_code,
        "message": body.get("message").cloned().unwrap_or(Value::Null),
        "reason_codes": body.pointer("/result/reason_codes").cloned().unwrap_or(json!([])),
    }))
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

    let hash_before_data =
        invoke_git_manager(repo, "get_last_commit", &json!({"ref": branch}))?;
    let hash_before = git_commit_hash(&hash_before_data);

    let status_data = invoke_git_manager(repo, "status", &json!({}))?;
    let porcelain = git_porcelain_stdout(&status_data);

    if porcelain.trim().is_empty() {
        if let Some(obj) = state.as_object_mut() {
            if let Some(h) = &hash_before {
                obj.insert("snapshot_commit_hash".into(), h.clone());
            }
        }
        return Ok(json!({
            "status": "executed",
            "handler": "delivery-snapshot-final",
            "commit_hash": hash_before,
            "branch": branch,
            "consolidated": false,
        }));
    }

    let pbi_ref = str_field(inputs, "pbi_ref");
    let all_files = parse_porcelain_paths(&porcelain);
    let files = filter_snapshot_commit_files(all_files.clone(), &porcelain, pbi_ref.as_deref());
    if files.is_empty() {
        let remaining =
            porcelain_excluding_preserved_untracked_todos(&porcelain, pbi_ref.as_deref());
        if remaining.trim().is_empty() && !all_files.is_empty() {
            if let Some(obj) = state.as_object_mut() {
                if let Some(h) = &hash_before {
                    obj.insert("snapshot_commit_hash".into(), h.clone());
                }
            }
            return Ok(json!({
                "status": "executed",
                "handler": "delivery-snapshot-final",
                "commit_hash": hash_before,
                "branch": branch,
                "consolidated": false,
                "preserved_untracked_todos": true,
            }));
        }
        return Ok(delivery_phase_failed(
            "delivery-snapshot-final",
            "SNAPSHOT_DIRTY_SKIPPED",
            "porcelain no vacío pero sin paths parseables",
        ));
    }
    let files_count = files.len();

    if let Err(e) = invoke_git_manager(
        repo,
        "commit",
        &json!({
            "message": "delivery-close: snapshot final consolidado",
            "files": files,
        }),
    ) {
        return Ok(delivery_phase_failed(
            "delivery-snapshot-final",
            "SNAPSHOT_DIRTY_SKIPPED",
            &e,
        ));
    }

    let status_after_data = invoke_git_manager(repo, "status", &json!({}))?;
    let porcelain_after = porcelain_excluding_preserved_untracked_todos(
        &git_porcelain_stdout(&status_after_data),
        pbi_ref.as_deref(),
    );
    if !porcelain_after.trim().is_empty() {
        return Ok(delivery_phase_failed(
            "delivery-snapshot-final",
            "SNAPSHOT_DIRTY_SKIPPED",
            "working tree sigue sucio tras commit de snapshot",
        ));
    }

    let hash_after_data =
        invoke_git_manager(repo, "get_last_commit", &json!({"ref": branch}))?;
    let hash_after = git_commit_hash(&hash_after_data);

    if hash_before == hash_after {
        return Ok(delivery_phase_failed(
            "delivery-snapshot-final",
            "SNAPSHOT_DIRTY_SKIPPED",
            "hash_after == hash_before; snapshot no consolidó WIP",
        ));
    }

    if let Some(obj) = state.as_object_mut() {
        if let Some(h) = &hash_after {
            obj.insert("snapshot_commit_hash".into(), h.clone());
        }
    }
    Ok(json!({
        "status": "executed",
        "handler": "delivery-snapshot-final",
        "commit_hash": hash_after,
        "branch": branch,
        "consolidated": true,
        "files_committed": files_count,
    }))
}

pub fn capsule_delivery_impact_assessment(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Value {
    if env_truthy("SDDIA_LAB_SKIP_IMPACT_ASSESSMENT") {
        return json!({
            "status": "skipped",
            "handler": "delivery-impact-assessment",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_IMPACT_ASSESSMENT",
        });
    }
    let source = inputs.get("source_process").and_then(|v| v.as_str()).unwrap_or("");
    if !matches!(source, "feature" | "bug-fix" | "refactorization") {
        return json!({
            "status": "skipped",
            "handler": "delivery-impact-assessment",
            "skipped": true,
            "reason": "source_process no elegible para impacto SddIA",
        });
    }
    let target = str_field(inputs, "target_branch").unwrap_or_else(|| "main".into());
    let origin_ref = format!("origin/{target}");
    let ref_spec = if std::process::Command::new("git")
        .args(["rev-parse", "--verify", "--quiet", &origin_ref])
        .current_dir(repo)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        format!("{origin_ref}...HEAD")
    } else {
        format!("{target}...HEAD")
    };
    let data = match invoke_git_manager(
        repo,
        "diff_name_only",
        &json!({"ref_spec": ref_spec}),
    ) {
        Ok(d) => d,
        Err(e) => {
            return json!({
                "status": "executed",
                "handler": "delivery-impact-assessment",
                "impact": "unknown",
                "sddia_paths": [],
                "error": e,
            });
        }
    };
    let files: Vec<String> = data
        .get("files")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let cfg = match crate::core::paths::load_paths_config(repo) {
        Ok(c) => c,
        Err(_) => {
            return json!({
                "status": "executed",
                "handler": "delivery-impact-assessment",
                "impact": "none",
                "sddia_paths": [],
            });
        }
    };
    let evo = cfg
        .pointer("/directories/evolution")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution");
    let mut prefixes = Vec::new();
    if let Some(dirs) = cfg.get("directories").and_then(|v| v.as_object()) {
        for val in dirs.values() {
            if let Some(s) = val.as_str() {
                let norm = s.trim().trim_end_matches('/');
                if norm.starts_with("SddIA/")
                    && norm != evo.trim_end_matches('/')
                    && !norm.starts_with(&format!("{}/", evo.trim_end_matches('/')))
                {
                    prefixes.push(format!("{norm}/"));
                }
            } else if let Some(arr) = val.as_array() {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        let norm = s.trim().trim_end_matches('/');
                        if norm.starts_with("SddIA/") {
                            prefixes.push(format!("{norm}/"));
                        }
                    }
                }
            }
        }
    }
    let sddia_paths: Vec<String> = files
        .into_iter()
        .filter(|p| {
            let p = p.replace('\\', "/");
            prefixes.iter().any(|prefix| {
                let base = prefix.trim_end_matches('/');
                p == base || p.starts_with(prefix) || p.starts_with(&format!("{base}/"))
            })
        })
        .collect();
    let impact = if sddia_paths.is_empty() { "none" } else { "material" };
    if let Some(obj) = state.as_object_mut() {
        obj.insert("sddia_impact".into(), json!(impact));
        obj.insert("sddia_paths".into(), json!(sddia_paths.clone()));
    }
    json!({
        "status": "executed",
        "handler": "delivery-impact-assessment",
        "impact": impact,
        "sddia_paths": sddia_paths,
        "ref_spec": ref_spec,
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
        let body_dir = resolve_pr_body_file_dir(repo, inputs, state)?;
        let body_path = match write_pr_body_file(&body_dir, &body) {
            Ok(p) => p,
            Err(e) => {
                return Ok(delivery_phase_failed(
                    "delivery-gh-pr",
                    "PR_BODY_METACHAR",
                    &e,
                ));
            }
        };
        args.push("--body-file".into());
        args.push(body_path);
    } else {
        args.push("--fill".into());
    }
    let data = match invoke_shell_executor(repo, "gh", &args) {
        Ok(d) => d,
        Err(e) => {
            if let Some(code) = classify_delivery_error(&e) {
                return Ok(delivery_phase_failed("delivery-gh-pr", code, &e));
            }
            return Err(e);
        }
    };
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
        "Impacto SddIA condicional" => Some(Ok(capsule_delivery_impact_assessment(repo, inputs, state))),
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
        // H9/qa.probe: validar envelope sddia-io (success/exitCode), no el `result` interno.
        match invoke_tool_capsule_json(repo, name, &payload, true) {
            Ok(cap) => {
                if cap.exit_code != 0 || cap.body.get("success") == Some(&json!(false)) {
                    let err = cap
                        .body
                        .get("error")
                        .and_then(|v| v.as_str())
                        .unwrap_or("tool failed")
                        .to_string();
                    last_err = Some(err);
                    continue;
                }
                if let Some(b) = di_bindings.first() {
                    if let Err(out_err) =
                        super::capability_di_output_validator::validate_output_payload(
                            repo, b, &cap.body,
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
                let body = unwrap_tool_body(&cap.body);
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

fn envelope_data(envelope: &Value) -> &Value {
    envelope.get("data").unwrap_or(envelope)
}

fn envelope_child_phases(envelope: &Value) -> Option<&Vec<Value>> {
    let report = envelope
        .get("execution_report")
        .or_else(|| envelope_data(envelope).get("execution_report"));
    report.and_then(|r| r.get("phases")).and_then(|v| v.as_array())
}

fn envelope_has_physical_threshold(envelope: &Value) -> bool {
    let data = envelope_data(envelope);
    let pr_ok = |v: Option<&Value>| {
        v.and_then(|x| x.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .is_some()
    };
    if pr_ok(data.get("pr_url")) {
        return true;
    }
    if data.get("delivery_push").is_some() {
        return true;
    }
    if pr_ok(data.get("delivery_close").and_then(|d| d.get("pr_url"))) {
        return true;
    }
    // Defensa: data.delivery_push aún no copiado; fase de publicación remota executed.
    envelope_child_phases(envelope)
        .map(|phases| {
            phases.iter().any(|p| {
                p.get("phase_name").and_then(|v| v.as_str()) == Some("Publicación remota")
                    && p.get("status").and_then(|v| v.as_str()) == Some("executed")
            })
        })
        .unwrap_or(false)
}

fn dcc_error_is_secondary(err: &str) -> bool {
    let el = err.to_lowercase();
    el.contains("timeout")
        || el.contains("telemetry")
        || el.contains("receipt")
        || el.contains("validaci")
        || el.contains("higiene")
        || el.contains("impacto")
        || el.contains("telemetry_io")
}

fn dcc_phase_is_secondary(phase_name: &str) -> bool {
    // Paridad con delivery_close::is_dcc_secondary_phase (no importar: ciclo de módulos).
    matches!(
        phase_name,
        "Impacto SddIA condicional" | "Higiene local"
    )
}

fn child_phases_fail_soft(envelope: &Value) -> bool {
    envelope_child_phases(envelope)
        .map(|phases| {
            phases
                .iter()
                .any(|p| p.get("fail_soft").and_then(|v| v.as_bool()) == Some(true))
        })
        .unwrap_or(false)
}

fn child_has_causal_hard_fail(envelope: &Value) -> bool {
    envelope_child_phases(envelope)
        .map(|phases| {
            phases.iter().any(|p| {
                let status = p.get("status").and_then(|v| v.as_str()).unwrap_or("");
                let hard = matches!(status, "failed" | "blocked")
                    && p.get("fail_soft").and_then(|v| v.as_bool()) != Some(true);
                if !hard {
                    return false;
                }
                let name = p.get("phase_name").and_then(|v| v.as_str()).unwrap_or("");
                !dcc_phase_is_secondary(name)
            })
        })
        .unwrap_or(false)
}

/// L-FAILSOFT-PADRE: DCC hijo cruzó umbral físico y el fallo es cola secundaria.
pub(crate) fn feature_dcc_parent_fail_soft(envelope: &Value) -> bool {
    if envelope.get("success") == Some(&json!(true)) {
        return false;
    }
    if !envelope_has_physical_threshold(envelope) {
        return false;
    }
    if child_has_causal_hard_fail(envelope) {
        return false;
    }
    if envelope_data(envelope)
        .get("telemetry_io_failed")
        .and_then(|v| v.as_bool())
        == Some(true)
        || envelope_data(envelope)
            .pointer("/thermodynamic_toll/telemetry_io_failed")
            .and_then(|v| v.as_bool())
            == Some(true)
    {
        return true;
    }
    let err = envelope
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    child_phases_fail_soft(envelope) || dcc_error_is_secondary(err)
}

fn copy_delivery_close_into_state(state: &mut Value, data: &Value) {
    if let Some(obj) = state.as_object_mut() {
        for key in [
            "pr_url",
            "event_id",
            "target_path",
            "closed_branch",
            "snapshot_commit_hash",
            "delivery_push",
        ] {
            if let Some(v) = data.get(key) {
                obj.insert(key.into(), v.clone());
            }
        }
        obj.insert("delivery_close".into(), data.clone());
    }
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
    let envelope =
        super::invoke_orchestrator::invoke_process_full(repo, "delivery-close-cycle", &child_inputs)?;
    let data = envelope.get("data").cloned().unwrap_or(envelope.clone());
    copy_delivery_close_into_state(state, &data);
    if envelope.get("success") == Some(&json!(true)) {
        return Ok(json!({
            "status": "executed",
            "handler": "feature-delivery-close",
            "child_process": "delivery-close-cycle",
            "delivery_close": data,
        }));
    }
    let err = envelope
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("subproceso falló")
        .to_string();
    if feature_dcc_parent_fail_soft(&envelope) {
        return Ok(json!({
            "status": "failed",
            "fail_soft": true,
            "handler": "feature-delivery-close",
            "child_process": "delivery-close-cycle",
            "error": err,
            "delivery_close": data,
        }));
    }
    Err(err)
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

#[cfg(test)]
mod delivery_close_kaizen_tests {
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
    fn parse_porcelain_paths_untracked_and_modified() {
        let stdout = "?? docs/fixes/new/file.md\n M SddIA/engine/foo.rs\n";
        let paths = parse_porcelain_paths(stdout);
        assert_eq!(
            paths,
            vec![
                "docs/fixes/new/file.md".to_string(),
                "SddIA/engine/foo.rs".to_string(),
            ]
        );
    }

    #[test]
    fn parse_porcelain_paths_rename() {
        let stdout = "R  old/path.md -> new/path.md\n";
        let paths = parse_porcelain_paths(stdout);
        assert_eq!(paths, vec!["new/path.md".to_string()]);
    }

    #[test]
    fn parse_porcelain_paths_cquoted_unicode_and_delete() {
        // Em-dash U+2014 = \342\200\224 ; ó U+00F3 = \303\255 in "vacío"
        let stdout = concat!(
            " D \"docs/todos/pending/[Kaizen] delivery-close \\342\\200\\224 snapshot vac\\303\\255o.md\"\n",
            "?? \"docs/todos/done/[Kaizen] delivery-close \\342\\200\\224 snapshot vac\\303\\255o.md\"\n",
            "?? docs/fixes/kaizen-delivery-close-snapshot-pr-body/\n",
        );
        let paths = parse_porcelain_paths(stdout);
        assert_eq!(
            paths,
            vec![
                "docs/todos/pending/[Kaizen] delivery-close — snapshot vacío.md".to_string(),
                "docs/todos/done/[Kaizen] delivery-close — snapshot vacío.md".to_string(),
                "docs/fixes/kaizen-delivery-close-snapshot-pr-body".to_string(),
            ]
        );
    }

    #[test]
    fn porcelain_excluding_preserved_untracked_todos_filters_ajeno() {
        let stdout = "?? docs/todos/pending/[KAIZEN] otro.md\n M docs/features/x/objectives.md\n";
        let out = porcelain_excluding_preserved_untracked_todos(stdout, None);
        assert!(!out.contains("docs/todos/pending"));
        assert!(out.contains("docs/features/x/objectives.md"));
    }

    #[test]
    fn filter_snapshot_commit_files_skips_untracked_todos_ajeno() {
        let porcelain = "?? docs/todos/pending/[KAIZEN] otro.md\n M docs/features/x/plan.md\n";
        let all = parse_porcelain_paths(porcelain);
        let files = filter_snapshot_commit_files(all, porcelain, None);
        assert_eq!(files, vec!["docs/features/x/plan.md".to_string()]);
    }

    #[test]
    fn pr_body_file_path_is_safe_token() {
        let repo = repo_root();
        let dir = repo.join("docs/fixes/kaizen-delivery-close-snapshot-pr-body/.tmp");
        let body = "## Summary\n- línea 1\n- línea 2\n";
        let path = write_pr_body_file(&dir, body).expect("write pr-body");
        assert!(is_shell_token_safe(&path));
        assert!(path.ends_with("pr-body.md"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn map_shell_metachar_error_to_pr_body_metachar() {
        let err = "arguments[9] contains forbidden shell metacharacters";
        assert_eq!(classify_delivery_error(err), Some("PR_BODY_METACHAR"));
    }

    #[test]
    fn snapshot_dirty_failure_sets_error_code() {
        let entry = delivery_phase_failed(
            "delivery-snapshot-final",
            "SNAPSHOT_DIRTY_SKIPPED",
            "hash_after == hash_before",
        );
        assert_eq!(entry.get("status").and_then(|v| v.as_str()), Some("failed"));
        assert_eq!(
            entry.get("error_code").and_then(|v| v.as_str()),
            Some("SNAPSHOT_DIRTY_SKIPPED")
        );
        assert!(
            entry
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .starts_with("[SNAPSHOT_DIRTY_SKIPPED]")
        );
    }

    #[test]
    fn resolve_pr_body_file_dir_from_persist_ref() {
        let repo = repo_root();
        let inputs = json!({
            "persist_ref": "docs/fixes/kaizen-delivery-close-snapshot-pr-body"
        });
        let state = json!({});
        let dir = resolve_pr_body_file_dir(&repo, &inputs, &state).expect("dir");
        assert!(dir.ends_with("docs/fixes/kaizen-delivery-close-snapshot-pr-body/.tmp"));
    }
}

#[cfg(test)]
mod evolution_audit_ca12_tests {
    use super::capsule_evolution_audit_gate;
    use serde_json::json;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use uuid::Uuid;

    fn repo_root() -> PathBuf {
        let mut here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        here.pop();
        here.pop();
        here.pop();
        here
    }

    #[cfg(not(unix))]
    fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let to = dst.join(entry.file_name());
            if ty.is_dir() {
                copy_dir_all(&entry.path(), &to)?;
            } else {
                fs::copy(entry.path(), to)?;
            }
        }
        Ok(())
    }

    struct Ca12Worktree {
        root: PathBuf,
        path: PathBuf,
    }

    impl Ca12Worktree {
        fn new() -> Self {
            let root = repo_root();
            let qa_src = root.join("SddIA/target/debug/sddia-qa");
            assert!(
                qa_src.is_file(),
                "compilar sddia-qa antes del test CA12: cd SddIA && cargo build -p sddia-qa"
            );
            let id = Uuid::new_v4().to_string();
            let path = root.join("target/ca12-worktrees").join(id);
            fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
            let status = Command::new("git")
                .args(["worktree", "add", "-q", path.to_str().expect("wt path"), "HEAD"])
                .current_dir(&root)
                .status()
                .expect("git worktree add");
            assert!(status.success(), "worktree add failed");
            let target_src = root.join("SddIA/target");
            let target_dst = path.join("SddIA/target");
            if target_dst.exists() {
                let _ = fs::remove_dir_all(&target_dst);
            }
            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&target_src, &target_dst).expect("symlink target");
            }
            #[cfg(not(unix))]
            {
                copy_dir_all(&target_src, &target_dst).expect("copy target");
            }
            Self { root, path }
        }

        fn commit_unregistered_probe(&self) {
            let probe = self.path.join("SddIA/tools/_ca12_smoke_probe.txt");
            fs::create_dir_all(probe.parent().expect("parent")).expect("mkdir probe");
            fs::write(&probe, "probe\n").expect("write probe");
            assert!(
                Command::new("git")
                    .args(["add", "SddIA/tools/_ca12_smoke_probe.txt"])
                    .current_dir(&self.path)
                    .status()
                    .expect("git add")
                    .success()
            );
            assert!(
                Command::new("git")
                    .args([
                        "-c",
                        "user.email=ca12@test",
                        "-c",
                        "user.name=ca12",
                        "commit",
                        "-m",
                        "test: ca12 unregistered material",
                        "--no-verify",
                    ])
                    .current_dir(&self.path)
                    .status()
                    .expect("git commit")
                    .success()
            );
        }
    }

    impl Drop for Ca12Worktree {
        fn drop(&mut self) {
            let _ = Command::new("git")
                .args(["worktree", "remove", "-f"])
                .arg(&self.path)
                .current_dir(&self.root)
                .status();
        }
    }

    #[test]
    fn evolution_audit_gate_blocks_unregistered_material_ca12() {
        let wt = Ca12Worktree::new();
        wt.commit_unregistered_probe();
        let mut state = json!({});
        let result = capsule_evolution_audit_gate(&wt.path, &json!({}), &mut state)
            .expect("gate debe retornar veredicto parseable");
        assert_eq!(
            result.get("status").and_then(|v| v.as_str()),
            Some("blocked"),
            "result={result}"
        );
        let codes = result
            .get("reason_codes")
            .and_then(|v| v.as_array())
            .expect("reason_codes");
        assert!(
            codes
                .iter()
                .any(|c| c.as_str() == Some("EVOL_MATERIAL_UNREGISTERED")),
            "codes={codes:?}"
        );
    }
}

#[cfg(test)]
mod feature_dcc_parent_fail_soft_tests {
    use super::feature_dcc_parent_fail_soft;
    use serde_json::json;

    #[test]
    fn soft_when_pr_url_and_hygiene_error() {
        let envelope = json!({
            "success": false,
            "error": "fase \"Higiene local\" failed",
            "data": {"pr_url": "https://github.com/racso80es/SddIA/pull/185"}
        });
        assert!(feature_dcc_parent_fail_soft(&envelope));
    }

    #[test]
    fn soft_when_telemetry_io_failed_after_push() {
        let envelope = json!({
            "success": false,
            "error": "subproceso falló",
            "data": {
                "delivery_push": {"ok": true},
                "thermodynamic_toll": {"telemetry_io_failed": true}
            }
        });
        assert!(feature_dcc_parent_fail_soft(&envelope));
    }

    #[test]
    fn soft_when_push_and_child_fail_soft_phase() {
        let envelope = json!({
            "success": false,
            "error": "subproceso falló",
            "data": {"delivery_push": {"ok": true}},
            "execution_report": {
                "phases": [
                    {"phase_name": "Higiene local", "status": "failed", "fail_soft": true}
                ]
            }
        });
        assert!(feature_dcc_parent_fail_soft(&envelope));
    }

    #[test]
    fn hard_when_snapshot_failed_without_physical_threshold() {
        let envelope = json!({
            "success": false,
            "error": "fase \"Snapshot final\" failed",
            "data": {}
        });
        assert!(!feature_dcc_parent_fail_soft(&envelope));
    }

    #[test]
    fn hard_when_child_succeeded() {
        let envelope = json!({
            "success": true,
            "data": {"pr_url": "https://github.com/racso80es/SddIA/pull/1"}
        });
        assert!(!feature_dcc_parent_fail_soft(&envelope));
    }

    #[test]
    fn hard_when_push_but_apertura_failed_without_fail_soft() {
        let envelope = json!({
            "success": false,
            "error": "fase \"Apertura en forja\" failed",
            "data": {"delivery_push": {"ok": true}},
            "execution_report": {
                "phases": [
                    {"phase_name": "Publicación remota", "status": "executed"},
                    {"phase_name": "Apertura en forja", "status": "failed"}
                ]
            }
        });
        assert!(!feature_dcc_parent_fail_soft(&envelope));
    }

    #[test]
    fn physical_fallback_from_remote_phase_executed() {
        let envelope = json!({
            "success": false,
            "error": "fase \"Higiene local\" failed",
            "data": {},
            "execution_report": {
                "phases": [
                    {"phase_name": "Publicación remota", "status": "executed"},
                    {"phase_name": "Higiene local", "status": "failed", "fail_soft": true}
                ]
            }
        });
        assert!(feature_dcc_parent_fail_soft(&envelope));
    }

    #[test]
    fn aggregator_treats_parent_fail_soft_as_success() {
        let reports = vec![json!({
            "phase_name": "Cierre de entrega",
            "status": "failed",
            "fail_soft": true,
            "handler": "feature-delivery-close"
        })];
        let v = crate::engine::phase_terminal::aggregate_execution_terminal(&reports, &json!({}));
        assert!(v.success);
        assert_eq!(v.status_code, 0);
    }
}
