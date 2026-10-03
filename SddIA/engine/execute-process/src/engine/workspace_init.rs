//! Inicialización de espacio de trabajo (git-manager + objectives.md).

use super::capsules::invoke_git_manager_for;
use super::domain_profile::resolve_execution_profile;
use super::project_binding::{self, DeliveryMode};
use super::git_porcelain;
use super::workspace::{
    load_paths_config, resolve_documentation_features_path, resolve_documentation_fixes_path,
};
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn workspace_task_name(inputs: &Value) -> Option<String> {
    for key in ["feature_name", "fix_name", "refactor_name"] {
        if let Some(v) = inputs.get(key).and_then(|x| x.as_str()) {
            let t = v.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    if let Some(branch) = inputs.get("branch_name").and_then(|v| v.as_str()) {
        let b = branch.trim();
        if let Some((prefix, slug)) = b.split_once('/') {
            if matches!(prefix, "feat" | "feature" | "fix" | "refactor") && !slug.trim().is_empty()
            {
                return Some(slug.trim().to_string());
            }
        }
    }
    None
}

fn has_work_branch_prefix(branch: &str) -> bool {
    ["feat/", "feature/", "fix/", "refactor/"]
        .iter()
        .any(|p| branch.starts_with(p))
}

fn default_branch_prefix(process_label: &str) -> &'static str {
    match process_label {
        "bug-fix" => "fix",
        "refactorization" => "refactor",
        _ => "feat",
    }
}

/// Conserva feat/fix/refactor si ya vienen; si no, aplica el default del proceso.
fn canonicalize_branch_name(branch_name: String, process_label: &str, task_name: &str) -> String {
    if has_work_branch_prefix(&branch_name) {
        return branch_name;
    }
    format!("{}/{task_name}", default_branch_prefix(process_label))
}

fn workspace_process_label(inputs: &Value, branch_name: &str, process_name: &str) -> String {
    if let Some(l) = inputs.get("process_label").and_then(|v| v.as_str()) {
        if !l.trim().is_empty() {
            return l.trim().to_string();
        }
    }
    if process_name == "refactorization"
        || inputs.get("source_process").and_then(|v| v.as_str()) == Some("refactorization")
    {
        return "refactorization".into();
    }
    if process_name == "bug-fix"
        || inputs.get("source_process").and_then(|v| v.as_str()) == Some("bug-fix")
        || branch_name.starts_with("fix/")
    {
        return "bug-fix".into();
    }
    "feature".into()
}

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn git_pull_divergence_soft_fail(err: &str) -> bool {
    let e = err.to_lowercase();
    e.contains("reconciliar")
        || e.contains("diverg")
        || e.contains("diverged")
        || e.contains("non-fast-forward")
}

fn pull_base_step(
    repo: &Path,
    git_root: &Path,
    base_branch: &str,
    git_steps: &mut Vec<Value>,
) -> Result<(), String> {
    let pull = invoke_git_manager_for(
        repo,
        git_root,
        "pull",
        &json!({"remote": "origin", "branch": base_branch}),
    );
    match pull {
        Ok(r) => git_steps.push(json!({"op": "pull_base", "result": r})),
        Err(e) if env_truthy("SDDIA_LAB_ALLOW_DIRTY") && git_pull_divergence_soft_fail(&e) => {
            git_steps.push(json!({
                "op": "pull_base",
                "result": {
                    "skipped": true,
                    "reason": "pull_diverged_lab_allow_dirty",
                    "error": e,
                }
            }));
        }
        Err(e) => return Err(e),
    }
    Ok(())
}

/// Misión destilada: no volcar YAML+cuerpo del PBI (F7).
fn distill_mission(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let (fm, after) = if trimmed.starts_with("---") {
        let rest = trimmed.strip_prefix("---").unwrap_or(trimmed);
        if let Some(idx) = rest.find("\n---") {
            let fm = rest[..idx].trim();
            let after = rest[idx + 4..].trim();
            (Some(fm), after)
        } else {
            (None, trimmed)
        }
    } else {
        (None, trimmed)
    };
    let doc_id = fm.and_then(|block| {
        block.lines().find_map(|l| {
            l.trim()
                .strip_prefix("document_id:")
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty())
        })
    });
    let title = after.lines().find_map(|l| {
        l.trim()
            .strip_prefix("# ")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    });
    let para: String = after
        .lines()
        .skip_while(|l| {
            let t = l.trim();
            t.is_empty()
                || t.starts_with('#')
                || t.starts_with('|')
                || t.starts_with('>')
                || t.starts_with('-')
        })
        .take_while(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let para_cut: String = para.chars().take(400).collect();
    let mut out = String::new();
    if let Some(t) = title {
        out.push_str(&t);
        out.push('\n');
    }
    if let Some(d) = doc_id {
        out.push_str("document_id: ");
        out.push_str(&d);
        out.push('\n');
    }
    if !para_cut.is_empty() {
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&para_cut);
    }
    let distilled = out.trim().to_string();
    if distilled.is_empty() {
        trimmed.chars().take(400).collect()
    } else {
        distilled
    }
}

fn path_in_scope(path: &str, persist_ref: &str, pbi_ref: Option<&str>) -> bool {
    let norm = path.trim_start_matches("./");
    let persist = persist_ref.trim_start_matches("./");
    if norm == persist || norm.starts_with(&format!("{}/", persist)) {
        return true;
    }
    // `git status` puede listar un directorio padre (`docs`) al crear `docs/fixes/…`.
    if persist.starts_with(&format!("{}/", norm)) || persist == norm {
        return true;
    }
    if let Some(pbi) = pbi_ref {
        if norm == pbi.trim_start_matches("./") {
            return true;
        }
    }
    false
}

fn current_branch_from_branch_list(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        let t = line.trim();
        let Some(rest) = t.strip_prefix("* ") else {
            continue;
        };
        let name = rest.split_whitespace().next().unwrap_or("");
        if name.is_empty() || name.starts_with('(') {
            continue;
        }
        return Some(name.to_string());
    }
    None
}

fn git_current_branch(capsule_repo: &Path, git_repo: &Path) -> Result<Option<String>, String> {
    let status = invoke_git_manager_for(capsule_repo, git_repo, "status", &json!({}))?;
    let stdout = status
        .get("gitStdout")
        .or_else(|| status.get("stdout"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    for line in stdout.lines() {
        if let Some(b) = line.strip_prefix("On branch ") {
            return Ok(Some(b.trim().to_string()));
        }
        if let Some(b) = line.strip_prefix("En la rama ") {
            return Ok(Some(b.trim().to_string()));
        }
    }
    let list = invoke_git_manager_for(capsule_repo, git_repo, "branch_list", &json!({}))?;
    let list_out = list
        .get("gitStdout")
        .or_else(|| list.get("stdout"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    Ok(current_branch_from_branch_list(list_out))
}

fn dirty_paths_outside_scope(
    capsule_repo: &Path,
    git_repo: &Path,
    persist_ref: &str,
    pbi_ref: Option<&str>,
) -> Result<Vec<String>, String> {
    if env_truthy("SDDIA_LAB_ALLOW_DIRTY") {
        return Ok(vec![]);
    }
    let status =
        super::capsules::invoke_git_manager_for(capsule_repo, git_repo, "status", &json!({}))?;
    let stdout = status
        .get("gitStdout")
        .or_else(|| status.get("stdout"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mut dirty = Vec::new();
    for line in stdout.lines() {
        if let Some(path) = git_porcelain::porcelain_path_from_line(line) {
            if !path_in_scope(&path, persist_ref, pbi_ref) {
                dirty.push(path);
            }
        }
    }
    dirty.sort();
    dirty.dedup();
    Ok(dirty)
}

fn emit_scope_fracture(repo: &Path, detail: &str) {
    let _ = super::route_domain_core::materialize_pending_domain_event(
        repo,
        "System_Fracture_Detected",
        "workspace-init",
        json!({
            "friction_id": "F-PROJECT-SCOPE-ESCAPE",
            "detail": detail,
        }),
    );
}

fn phase_requires_git_sync(phase: &Value) -> bool {
    if let Some(caps) = phase.get("requires_capability").and_then(|v| v.as_array()) {
        for c in caps {
            let id = c.get("id").and_then(|x| x.as_str()).or_else(|| c.as_str());
            if id == Some("proc:git-sync") {
                return true;
            }
        }
    }
    false
}

fn phase_has_git_manager_delegate(phase: &Value) -> bool {
    if let Some(delegates) = phase.get("delegates_to").and_then(|v| v.as_array()) {
        if delegates
            .iter()
            .any(|d| d.as_str() == Some("skill:git-manager"))
        {
            return true;
        }
    }
    if let Some(providers) = phase.get("resolved_provider").and_then(|v| v.as_array()) {
        if providers
            .iter()
            .any(|d| d.as_str() == Some("skill:git-manager"))
        {
            return true;
        }
    }
    if let Some(p) = phase.get("resolved_provider").and_then(|v| v.as_str()) {
        if p == "skill:git-manager" {
            return true;
        }
    }
    false
}

pub fn run(repo: &Path, inputs: &Value, process_name: &str) -> Result<Value, String> {
    let cfg = load_paths_config(repo)?;
    let profile = resolve_execution_profile(repo, inputs);
    let bound = match project_binding::bind(repo, inputs) {
        Ok(b) => b,
        Err(e) => {
            if e.starts_with("PROJECT_SCOPE_ESCAPE") {
                emit_scope_fracture(repo, &e);
            }
            return Err(e);
        }
    };
    let task_name = workspace_task_name(inputs);
    let branch_name = inputs
        .get("branch_name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);

    let process_label =
        workspace_process_label(inputs, branch_name.as_deref().unwrap_or(""), process_name);
    let task_name = task_name.unwrap_or_else(|| {
        branch_name
            .as_deref()
            .and_then(|b| b.split_once('/').map(|(_, s)| s.to_string()))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| branch_name.clone().unwrap_or_default())
    });
    if task_name.is_empty() && branch_name.is_none() {
        return Err("branch_name inválido".into());
    }
    let trunk = bound
        .as_ref()
        .map(|b| b.delivery_mode == DeliveryMode::TrunkDirect)
        .unwrap_or(false);
    let branch_name = if trunk {
        bound
            .as_ref()
            .map(|b| b.default_branch.clone())
            .unwrap_or_else(|| "main".into())
    } else {
        match branch_name {
            Some(b) => canonicalize_branch_name(b, &process_label, &task_name),
            None => format!("{}/{task_name}", default_branch_prefix(&process_label)),
        }
    };

    let base_branch = inputs
        .get("base_branch")
        .and_then(|v| v.as_str())
        .unwrap_or("main")
        .trim()
        .to_string();

    let default_docs = if process_label == "bug-fix" {
        resolve_documentation_fixes_path(repo, &cfg)
    } else {
        resolve_documentation_features_path(repo, &cfg)
    };
    let persist_ref = inputs
        .get("persist_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("{default_docs}/{task_name}"));

    let persist_ref = if let Some(project) = &bound {
        match project_binding::anchor_persist(&project.project_root, &persist_ref) {
            Ok(path) => path.to_string_lossy().replace('\\', "/"),
            Err(e) => {
                if e.starts_with("PROJECT_SCOPE_ESCAPE") {
                    emit_scope_fracture(repo, &e);
                }
                return Err(e);
            }
        }
    } else {
        persist_ref
    };

    let git_root = bound
        .as_ref()
        .map(|b| b.project_root.as_path())
        .unwrap_or(repo);

    let persist_scope = bound
        .as_ref()
        .and_then(|b| {
            Path::new(&persist_ref)
                .strip_prefix(&b.project_root)
                .ok()
                .map(|p| p.to_string_lossy().replace('\\', "/"))
        })
        .unwrap_or_else(|| persist_ref.clone());

    let refined = inputs
        .get("pbi_body")
        .or_else(|| inputs.get("refined_requirements"))
        .or_else(|| inputs.get("refactor_goal"))
        .or_else(|| inputs.get("bug_summary"))
        .or_else(|| inputs.get("description"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let pbi_ref_meta = inputs
        .get("pbi_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let mut git_steps: Vec<Value> = Vec::new();
    let skip_lab = env_truthy("SDDIA_LAB_SKIP_GIT");
    let skip_profile = !profile.git_required;

    if skip_lab {
        git_steps.push(json!({
            "op": "git",
            "result": {"skipped": true, "reason": "SDDIA_LAB_SKIP_GIT"},
        }));
    } else if skip_profile {
        git_steps.push(json!({
            "op": "git",
            "result": {
                "skipped": true,
                "reason": "profile_git_not_required",
                "profile_source": profile.source,
                "codex_slug": profile.codex_slug,
            },
        }));
    } else {
        let dirty =
            dirty_paths_outside_scope(repo, git_root, &persist_scope, pbi_ref_meta)?;
        if !dirty.is_empty() {
            let msg = format!(
                "dirty-worktree: cambios fuera de persist_ref/pbi_ref: {}",
                dirty.join(", ")
            );
            return Err(msg);
        }

        let on_feature_branch = git_current_branch(repo, git_root)?
            .map(|b| b == branch_name)
            .unwrap_or(false);
        if on_feature_branch {
            git_steps.push(json!({
                "op": "git_reentry",
                "result": {
                    "skipped": true,
                    "reason": "already_on_branch_no_dirty_outside_scope",
                    "branch_name": branch_name,
                },
            }));
        } else {
        let fetch = invoke_git_manager_for(
            repo,
            git_root,
            "fetch",
            &json!({"remote": "origin", "prune": true}),
        )?;
        git_steps.push(json!({"op": "fetch", "result": fetch}));

        let checkout_base = invoke_git_manager_for(
            repo,
            git_root,
            "checkout",
            &json!({"branch_name": base_branch, "create_if_not_exists": false}),
        )?;
        git_steps.push(json!({"op": "checkout_base", "result": checkout_base}));

        let offline = fetch.get("offline").and_then(|v| v.as_bool()) == Some(true);
        if !offline {
            pull_base_step(repo, git_root, &base_branch, &mut git_steps)?;
        } else {
            git_steps.push(json!({
                "op": "pull_base",
                "result": {"skipped": true, "reason": "offline_fetch", "offline": true}
            }));
        }

        let checkout_feature = invoke_git_manager_for(
            repo,
            git_root,
            "checkout",
            &json!({"branch_name": branch_name, "create_if_not_exists": true}),
        );
        match checkout_feature {
            Ok(r) => git_steps.push(json!({"op": "checkout_feature", "result": r})),
            Err(_) => {
                let r = invoke_git_manager_for(
                    repo,
                    git_root,
                    "checkout",
                    &json!({"branch_name": branch_name, "create_if_not_exists": false}),
                )?;
                git_steps.push(json!({"op": "checkout_feature_existing", "result": r}));
            }
        }
        }
    }

    let persist_dir = if Path::new(&persist_ref).is_absolute() {
        PathBuf::from(&persist_ref)
    } else {
        repo.join(&persist_ref)
    };
    fs::create_dir_all(&persist_dir).map_err(|e| e.to_string())?;
    let objectives_path = persist_dir.join("objectives.md");
    let execution_id_meta = inputs
        .get("execution_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if !objectives_path.is_file() {
        let created = Utc::now().format("%Y-%m-%d").to_string();
        let summary = if refined.trim().is_empty() {
            format!("{process_label} {task_name}")
        } else {
            distill_mission(refined)
        };
        let pbi_line = pbi_ref_meta
            .map(|p| format!("pbi_ref: {p}\n"))
            .unwrap_or_default();
        let execution_id_line = execution_id_meta
            .map(|eid| format!("execution_id: \"{eid}\"\n"))
            .unwrap_or_default();
        let body = format!(
            "---\nfeature_name: {task_name}\ncreated: \"{created}\"\nprocess: {process_label}\nbranch_name: {branch_name}\npersist_ref: {persist_ref}\n{pbi_line}{execution_id_line}---\n\n# Objetivos — {task_name}\n\n## Misión\n\n{summary}\n\n## Alcance (manifiesto)\n\nInicialización de contexto vía orquestador nativo `execute-process` (laboratorio).\n\n## Ley aplicada\n\n- Git exclusivamente vía `skill:git-manager`.\n- Jerarquía: Acción → Agente → Skill → Tools.\n"
        );
        fs::write(&objectives_path, body).map_err(|e| e.to_string())?;
    } else if let Some(eid) = execution_id_meta {
        let text = fs::read_to_string(&objectives_path).map_err(|e| e.to_string())?;
        let trimmed = text.trim_start();
        if trimmed.starts_with("---") {
            let rest = trimmed.strip_prefix("---").unwrap_or(trimmed);
            if let Some(end) = rest.find("\n---") {
                let fm = rest[..end].trim_end();
                let body = rest[end + 4..].trim_start();
                let mut lines: Vec<String> = fm
                    .lines()
                    .filter(|l| !l.trim_start().starts_with("execution_id:"))
                    .map(|l| l.to_string())
                    .collect();
                lines.push(format!("execution_id: \"{eid}\""));
                let updated = format!("---\n{}\n---\n{}", lines.join("\n"), body);
                fs::write(&objectives_path, updated).map_err(|e| e.to_string())?;
            }
        }
    }

    let objectives_rel = objectives_path
        .strip_prefix(repo)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| objectives_path.to_string_lossy().into_owned());

    Ok(json!({
        "feature_name": task_name,
        "task_name": task_name,
        "process_label": process_label,
        "branch_name": branch_name,
        "persist_ref": persist_ref,
        "objectives_path": objectives_rel,
        "git_steps": git_steps,
        "work_initiated": json!(null),
        "execution_profile": profile.to_json(),
        "delivery_mode": bound.as_ref().map(|b| b.delivery_mode.as_str()),
        "delivery_mode_source": bound.as_ref().map(|b| b.delivery_mode_source),
        "project_root": bound.as_ref().map(|b| b.project_root.to_string_lossy().replace('\\', "/")),
    }))
}

fn input_non_empty_str(inputs: &Value, key: &str) -> bool {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .is_some_and(|s| !s.is_empty())
}

/// Primera fase de ejecución Tekton (`agent:tekton` en `delegates_to`).
pub fn phase_delegates_to_tekton(delegates: &[Value]) -> bool {
    delegates
        .iter()
        .any(|d| d.as_str() == Some("agent:tekton"))
}

fn pbi_ref_from_inputs(inputs: &Value) -> Option<&str> {
    inputs
        .get("pbi_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

/// Emisión fail-soft de `Work_Initiated` (D5). Payload estable respecto al cierre de init previo.
pub fn emit_work_initiated_fail_soft(
    repo: &Path,
    process_name: &str,
    inputs: &Value,
) -> Value {
    if !matches!(process_name, "feature" | "bug-fix" | "refactorization")
        || env_truthy("SDDIA_LAB_SKIP_WORK_INITIATED")
    {
        return json!(null);
    }
    let branch = inputs
        .get("branch_name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("");
    let persist = inputs
        .get("persist_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("");
    if branch.is_empty() || persist.is_empty() {
        return json!({"warn": "work_initiated: branch_name o persist_ref ausentes"});
    }
    let mut emit_inputs = json!({
        "branch": branch,
        "persist_ref": persist,
        "source_process": process_name,
    });
    if let Some(slug) = inputs
        .get("project_slug")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        emit_inputs["project_slug"] = json!(slug);
    }
    if let Some(pbi) = pbi_ref_from_inputs(inputs) {
        emit_inputs["pbi_ref"] = json!(pbi);
        if let Some(tr) = super::tracker_pbi_meta::tracker_ref_from_pbi(repo, pbi) {
            emit_inputs["tracker_ref"] = json!(tr);
        }
    }
    match super::actions::try_run_native(repo, "emit-work-initiated-event", &emit_inputs) {
        Ok(Some(v)) => v,
        Ok(None) => json!({"warn": "emit-work-initiated-event no nativo"}),
        Err(e) => json!({"warn": e}),
    }
}

/// Una sola emisión al entrar en la primera fase `agent:tekton` (HU-A 05 / D-B).
pub fn maybe_emit_work_initiated_on_tekton_entry(
    repo: &Path,
    process_name: &str,
    delegates: &[Value],
    inputs: &Value,
    state: &mut Value,
    entry: &mut Value,
) {
    if !phase_delegates_to_tekton(delegates) {
        return;
    }
    if state
        .get("work_initiated_emitted")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return;
    }
    let wi = emit_work_initiated_fail_soft(repo, process_name, inputs);
    if let Some(obj) = state.as_object_mut() {
        obj.insert("work_initiated_emitted".into(), json!(true));
        obj.insert("work_initiated".into(), wi.clone());
    }
    entry["work_initiated"] = wi;
}

/// Detector de fase Inicialización (I7 / L-SPLIT-A D4).
/// Identidad de tarea primero; no exigir `delegates_to` post-DI para interceptar `workspace-init`.
pub fn is_workspace_init_phase(phase: &Value, inputs: &Value, process_name: &str) -> bool {
    if !matches!(process_name, "feature" | "bug-fix" | "refactorization") {
        return false;
    }
    if phase.get("name").and_then(|v| v.as_str()) != Some("Inicialización de Espacio de Trabajo") {
        return false;
    }
    let has_identity = workspace_task_name(inputs).is_some()
        || input_non_empty_str(inputs, "project_slug")
        || (process_name == "bug-fix"
            && input_non_empty_str(inputs, "branch_name")
            && input_non_empty_str(inputs, "persist_ref"));
    has_identity
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_cumulo(root: &Path) {
        fs::create_dir_all(root.join("SddIA/core")).unwrap();
        fs::write(
            root.join("SddIA/core/cumulo.paths.json"),
            r#"{
  "directories": { "documentation": "docs" },
  "paths": { "featurePath": "docs/features", "fixPath": "docs/fixes" }
}"#,
        )
        .unwrap();
    }

    #[test]
    fn detector_accepts_requires_capability_without_delegates() {
        let phase = json!({
            "name": "Inicialización de Espacio de Trabajo",
            "requires_capability": [
                { "id": "proc:git-sync", "contract": "proc.git_sync", "version": ">=1.0.0" }
            ]
        });
        let inputs = json!({ "feature_name": "demo-task" });
        assert!(is_workspace_init_phase(&phase, &inputs, "feature"));
    }

    #[test]
    fn detector_accepts_delegates_git_manager() {
        let phase = json!({
            "name": "Inicialización de Espacio de Trabajo",
            "delegates_to": ["skill:git-manager"]
        });
        let inputs = json!({ "feature_name": "demo-task" });
        assert!(is_workspace_init_phase(&phase, &inputs, "feature"));
    }

    #[test]
    fn detector_accepts_bug_fix_with_project_slug_only() {
        let phase = json!({
            "name": "Inicialización de Espacio de Trabajo",
            "requires_capability": [
                { "id": "proc:git-sync", "contract": "proc.git_sync", "version": ">=1.0.0" }
            ]
        });
        let inputs = json!({
            "project_slug": "barcelonaxplorer",
            "branch_name": "fix/demo",
            "persist_ref": "docs/fixes/demo"
        });
        assert!(is_workspace_init_phase(&phase, &inputs, "bug-fix"));
    }

    #[test]
    fn workspace_init_does_not_emit_work_initiated() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        std::env::set_var("SDDIA_LAB_SKIP_GIT", "1");
        let inputs = json!({
            "feature_name": "wi-defer",
            "branch_name": "feat/wi-defer",
            "persist_ref": "docs/features/wi-defer",
            "refined_requirements": "AC-4c defer"
        });
        let out = run(root, &inputs, "feature").expect("run ok");
        assert_eq!(out["work_initiated"], json!(null));
        std::env::remove_var("SDDIA_LAB_SKIP_GIT");
    }

    #[test]
    fn tekton_delegate_detector() {
        assert!(phase_delegates_to_tekton(&[json!("agent:tekton")]));
        assert!(!phase_delegates_to_tekton(&[json!("agent:argos")]));
    }

    #[test]
    fn maybe_emit_work_initiated_only_once() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        std::env::set_var("SDDIA_LAB_SKIP_WORK_INITIATED", "1");
        let inputs = json!({
            "branch_name": "feat/wi-once",
            "persist_ref": "docs/features/wi-once"
        });
        let mut state = json!({});
        let mut entry = json!({});
        let delegates = vec![json!("agent:tekton")];
        maybe_emit_work_initiated_on_tekton_entry(
            root,
            "feature",
            &delegates,
            &inputs,
            &mut state,
            &mut entry,
        );
        assert_eq!(state["work_initiated_emitted"], json!(true));
        entry = json!({});
        maybe_emit_work_initiated_on_tekton_entry(
            root,
            "feature",
            &delegates,
            &inputs,
            &mut state,
            &mut entry,
        );
        assert!(entry.get("work_initiated").is_none());
        std::env::remove_var("SDDIA_LAB_SKIP_WORK_INITIATED");
    }

    #[test]
    fn detector_rejects_unrelated_phase() {
        let phase = json!({
            "name": "Estabilización de Requisitos",
            "requires_capability": [{ "id": "proc:git-sync" }]
        });
        let inputs = json!({ "feature_name": "demo-task" });
        assert!(!is_workspace_init_phase(&phase, &inputs, "feature"));
    }

    #[test]
    fn run_skips_git_when_profile_git_not_required() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        // Ensure no SDDIA_LAB_SKIP_GIT interference in this process.
        std::env::remove_var("SDDIA_LAB_SKIP_GIT");
        let inputs = json!({
            "feature_name": "no-git-boot",
            "branch_name": "feat/no-git-boot",
            "persist_ref": "docs/features/no-git-boot",
            "refined_requirements": "smoke AC-WSINIT",
            "execution_profile": { "git_required": false }
        });
        let out = run(root, &inputs, "feature").expect("run ok");
        let steps = out["git_steps"].as_array().unwrap();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0]["result"]["skipped"], json!(true));
        assert_eq!(
            steps[0]["result"]["reason"],
            json!("profile_git_not_required")
        );
        assert!(root
            .join("docs/features/no-git-boot/objectives.md")
            .is_file());
        assert_eq!(out["execution_profile"]["git_required"], json!(false));
    }

    #[test]
    fn refactor_prefix_is_not_rewritten_to_feat() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        std::env::remove_var("SDDIA_LAB_SKIP_GIT");
        let inputs = json!({
            "refactor_name": "kalma2-phase-barrier-timeout-persist",
            "branch_name": "refactor/kalma2-phase-barrier-timeout-persist",
            "persist_ref": "docs/features/kalma2-phase-barrier-timeout-persist",
            "refactor_goal": "AC-BRANCH",
            "execution_profile": { "git_required": false }
        });
        let out = run(root, &inputs, "refactorization").expect("run ok");
        assert_eq!(
            out["branch_name"].as_str().unwrap(),
            "refactor/kalma2-phase-barrier-timeout-persist"
        );
        assert_eq!(out["process_label"].as_str().unwrap(), "refactorization");
        assert!(root
            .join("docs/features/kalma2-phase-barrier-timeout-persist/objectives.md")
            .is_file());
    }

    #[test]
    fn refactorization_default_prefix_is_refactor() {
        assert_eq!(
            canonicalize_branch_name("kalma2-x".into(), "refactorization", "kalma2-x"),
            "refactor/kalma2-x"
        );
        assert_eq!(
            canonicalize_branch_name("feat/keep".into(), "refactorization", "keep"),
            "feat/keep"
        );
        assert_eq!(
            canonicalize_branch_name("fix/keep".into(), "feature", "keep"),
            "fix/keep"
        );
    }

    #[test]
    fn path_in_scope_accepts_pbi_ref_with_unicode_after_unescape() {
        use crate::engine::git_porcelain;
        let pbi = "docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md";
        let line = r#" M "docs/todos/pending/[REGRESI\303\223N] route-domain-event \342\200\224 fractura sist\303\251mica (6a49e0ad310e)-R1.md""#;
        let path = git_porcelain::porcelain_path_from_line(line).expect("path");
        assert!(path_in_scope(&path, "docs/fixes/other", Some(pbi)));
    }

    #[test]
    fn distill_mission_strips_pbi_yaml_dump() {
        let raw = "---\ndocument_id: PBI-KAIZEN-X\ntitle: dump\n---\n\n# Título visible\n\nPárrafo corto de misión.\n\n## Alcance\n\nNo debe entrar el YAML.\n";
        let got = distill_mission(raw);
        assert!(got.contains("Título visible"));
        assert!(got.contains("document_id: PBI-KAIZEN-X"));
        assert!(got.contains("Párrafo corto"));
        assert!(!got.contains("title: dump"));
        assert!(!got.contains("## Alcance"));
    }

    fn init_git_repo(root: &Path) {
        use std::process::Command;
        for args in [
            &["init"][..],
            &["config", "user.email", "wsinit@test.local"],
            &["config", "user.name", "wsinit-test"],
        ] {
            let status = Command::new("git").args(args).current_dir(root).status();
            assert!(status.is_ok_and(|s| s.success()), "git {:?}", args);
        }
        fs::write(root.join("README.md"), "seed\n").unwrap();
        for args in [&["add", "."][..], &["commit", "-m", "init"][..]] {
            let status = Command::new("git").args(args).current_dir(root).status();
            assert!(status.is_ok_and(|s| s.success()), "git {:?}", args);
        }
    }

    fn count_system_fracture_pending(repo: &Path) -> usize {
        let pending = repo.join(".events/pending");
        if !pending.is_dir() {
            return 0;
        }
        let Ok(entries) = fs::read_dir(&pending) else {
            return 0;
        };
        entries
            .filter_map(Result::ok)
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|x| x.to_str())
                    .is_some_and(|x| x == "json")
            })
            .filter(|e| {
                fs::read_to_string(e.path())
                    .map(|t| t.contains("System_Fracture_Detected"))
                    .unwrap_or(false)
            })
            .count()
    }

    fn link_capsule_target(root: &Path) {
        let sddia_ws = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("SddIA cargo workspace");
        let target_parent = root.join("SddIA");
        fs::create_dir_all(&target_parent).unwrap();
        let link = target_parent.join("target");
        if !link.exists() {
            std::os::unix::fs::symlink(sddia_ws.join("target"), &link).expect("symlink target");
        }
    }

    #[test]
    fn run_reentry_on_branch_skips_git_sync_when_dirty_only_in_persist() {
        use std::process::Command;
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        link_capsule_target(root);
        init_git_repo(root);
        let branch = "fix/reentry-persist";
        let persist = "docs/fixes/reentry-persist";
        let status = Command::new("git")
            .args(["checkout", "-b", branch])
            .current_dir(root)
            .status();
        assert!(status.is_ok_and(|s| s.success()));
        fs::create_dir_all(root.join(persist)).unwrap();
        fs::write(root.join(persist).join("validacion.md"), "draft reentry\n").unwrap();

        std::env::remove_var("SDDIA_LAB_ALLOW_DIRTY");
        std::env::remove_var("SDDIA_LAB_SKIP_GIT");

        let inputs = json!({
            "fix_name": "reentry-persist",
            "branch_name": branch,
            "persist_ref": persist,
            "execution_profile": { "git_required": true }
        });
        let out = run(root, &inputs, "bug-fix").expect("reentry init");
        let steps = out["git_steps"].as_array().unwrap();
        assert!(
            steps
                .iter()
                .any(|s| s.get("op") == Some(&json!("git_reentry"))),
            "steps: {steps:?}"
        );
    }

    #[test]
    fn run_dirty_outside_scope_aborts_without_system_fracture() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        link_capsule_target(root);
        fs::create_dir_all(root.join(".events/pending")).unwrap();
        init_git_repo(root);
        fs::write(root.join("outside-dirty.txt"), "dirty\n").unwrap();

        std::env::remove_var("SDDIA_LAB_ALLOW_DIRTY");
        std::env::remove_var("SDDIA_LAB_SKIP_GIT");

        let inputs = json!({
            "fix_name": "dirty-scope-test",
            "branch_name": "fix/dirty-scope-test",
            "persist_ref": "docs/fixes/dirty-scope-test",
            "execution_profile": { "git_required": true }
        });
        let err = run(root, &inputs, "bug-fix").expect_err("dirty abort");
        assert!(
            err.starts_with("dirty-worktree:"),
            "unexpected err: {err}"
        );
        assert!(err.contains("outside-dirty.txt"));
        assert_eq!(
            count_system_fracture_pending(root),
            0,
            "F-DIRTY-WORKTREE must not emit System_Fracture_Detected"
        );
    }
}
