//! Inicialización de espacio de trabajo (git-manager + objectives.md).

use super::capsules::invoke_git_manager;
use super::domain_profile::resolve_execution_profile;
use super::git_porcelain;
use super::workspace::{
    load_paths_config, resolve_documentation_features_path, resolve_documentation_fixes_path,
};
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

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

fn path_in_scope(path: &str, persist_ref: &str, pbi_ref: Option<&str>) -> bool {
    let norm = path.trim_start_matches("./");
    if norm == persist_ref.trim_start_matches("./")
        || norm.starts_with(&format!("{}/", persist_ref.trim_start_matches("./")))
    {
        return true;
    }
    if let Some(pbi) = pbi_ref {
        if norm == pbi.trim_start_matches("./") {
            return true;
        }
    }
    false
}

fn dirty_paths_outside_scope(
    repo: &Path,
    persist_ref: &str,
    pbi_ref: Option<&str>,
) -> Result<Vec<String>, String> {
    if env_truthy("SDDIA_LAB_ALLOW_DIRTY") {
        return Ok(vec![]);
    }
    let status = super::capsules::invoke_git_manager(repo, "status", &json!({}))?;
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
    let branch_name = match branch_name {
        Some(b) => canonicalize_branch_name(b, &process_label, &task_name),
        None => format!("{}/{task_name}", default_branch_prefix(&process_label)),
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
        let dirty = dirty_paths_outside_scope(repo, &persist_ref, pbi_ref_meta)?;
        if !dirty.is_empty() {
            let msg = format!(
                "dirty-worktree: cambios fuera de persist_ref/pbi_ref: {}",
                dirty.join(", ")
            );
            return Err(msg);
        }

        let fetch = invoke_git_manager(repo, "fetch", &json!({"remote": "origin", "prune": true}))?;
        git_steps.push(json!({"op": "fetch", "result": fetch}));

        let checkout_base = invoke_git_manager(
            repo,
            "checkout",
            &json!({"branch_name": base_branch, "create_if_not_exists": false}),
        )?;
        git_steps.push(json!({"op": "checkout_base", "result": checkout_base}));

        let offline = fetch.get("offline").and_then(|v| v.as_bool()) == Some(true);
        if !offline {
            let pull = invoke_git_manager(
                repo,
                "pull",
                &json!({"remote": "origin", "branch": base_branch}),
            )?;
            git_steps.push(json!({"op": "pull_base", "result": pull}));
        } else {
            git_steps.push(json!({
                "op": "pull_base",
                "result": {"skipped": true, "reason": "offline_fetch", "offline": true}
            }));
        }

        let checkout_feature = invoke_git_manager(
            repo,
            "checkout",
            &json!({"branch_name": branch_name, "create_if_not_exists": true}),
        );
        match checkout_feature {
            Ok(r) => git_steps.push(json!({"op": "checkout_feature", "result": r})),
            Err(_) => {
                let r = invoke_git_manager(
                    repo,
                    "checkout",
                    &json!({"branch_name": branch_name, "create_if_not_exists": false}),
                )?;
                git_steps.push(json!({"op": "checkout_feature_existing", "result": r}));
            }
        }
    }

    let persist_dir = repo.join(&persist_ref);
    fs::create_dir_all(&persist_dir).map_err(|e| e.to_string())?;
    let objectives_path = persist_dir.join("objectives.md");
    if !objectives_path.is_file() {
        let created = Utc::now().format("%Y-%m-%d").to_string();
        let summary = if refined.trim().is_empty() {
            format!("{process_label} {task_name}")
        } else {
            refined.trim().to_string()
        };
        let pbi_line = pbi_ref_meta
            .map(|p| format!("pbi_ref: {p}\n"))
            .unwrap_or_default();
        let execution_id_line = inputs
            .get("execution_id")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|eid| format!("execution_id: \"{eid}\"\n"))
            .unwrap_or_default();
        let body = format!(
            "---\nfeature_name: {task_name}\ncreated: \"{created}\"\nprocess: {process_label}\nbranch_name: {branch_name}\npersist_ref: {persist_ref}\n{pbi_line}{execution_id_line}---\n\n# Objetivos — {task_name}\n\n## Misión\n\n{summary}\n\n## Alcance (manifiesto)\n\nInicialización de contexto vía orquestador nativo `execute-process` (laboratorio).\n\n## Ley aplicada\n\n- Git exclusivamente vía `skill:git-manager`.\n- Jerarquía: Acción → Agente → Skill → Tools.\n"
        );
        fs::write(&objectives_path, body).map_err(|e| e.to_string())?;
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
        "execution_profile": profile.to_json(),
    }))
}

/// Detector de fase Inicialización: acepta delegates post-DI, `requires_capability: proc:git-sync`
/// o `resolved_provider` con `skill:git-manager` (I7 / L-SPLIT-A D4).
pub fn is_workspace_init_phase(phase: &Value, inputs: &Value, process_name: &str) -> bool {
    if !matches!(process_name, "feature" | "bug-fix" | "refactorization") {
        return false;
    }
    if phase.get("name").and_then(|v| v.as_str()) != Some("Inicialización de Espacio de Trabajo") {
        return false;
    }
    if !phase_has_git_manager_delegate(phase) && !phase_requires_git_sync(phase) {
        return false;
    }
    if workspace_task_name(inputs).is_some() {
        return true;
    }
    if process_name == "bug-fix" {
        let branch = inputs.get("branch_name").and_then(|v| v.as_str());
        let persist = inputs.get("persist_ref").and_then(|v| v.as_str());
        if branch.map(|s| !s.trim().is_empty()) == Some(true)
            && persist.map(|s| !s.trim().is_empty()) == Some(true)
        {
            return true;
        }
    }
    false
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
