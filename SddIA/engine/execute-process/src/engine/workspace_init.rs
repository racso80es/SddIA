//! Inicialización de espacio de trabajo (git-manager + objectives.md).

use super::capsules::invoke_git_manager;
use super::workspace::{load_paths_config, resolve_documentation_features_path, resolve_documentation_fixes_path};
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
            if (prefix == "feat" || prefix == "fix") && !slug.trim().is_empty() {
                return Some(slug.trim().to_string());
            }
        }
    }
    None
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

pub fn run(repo: &Path, inputs: &Value, process_name: &str) -> Result<Value, String> {
    let cfg = load_paths_config(repo)?;
    let task_name = workspace_task_name(inputs);
    let mut branch_name = inputs
        .get("branch_name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);

    if branch_name.is_none() {
        branch_name = task_name.as_ref().map(|t| format!("feat/{t}"));
    }
    let branch_name = branch_name.ok_or("branch_name inválido")?;
    let process_label = workspace_process_label(inputs, &branch_name, process_name);
    let task_name = task_name.unwrap_or_else(|| {
        branch_name
            .split_once('/')
            .map(|(_, s)| s.to_string())
            .unwrap_or_else(|| branch_name.clone())
    });

    let default_prefix = if process_label == "bug-fix" { "fix" } else { "feat" };
    let branch_name = if branch_name.starts_with(&format!("{default_prefix}/")) {
        branch_name
    } else {
        format!("{default_prefix}/{task_name}")
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
        .get("refined_requirements")
        .or_else(|| inputs.get("refactor_goal"))
        .or_else(|| inputs.get("bug_summary"))
        .or_else(|| inputs.get("description"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let mut git_steps: Vec<Value> = Vec::new();

    if env_truthy("SDDIA_LAB_SKIP_GIT") {
        git_steps.push(json!({
            "op": "git",
            "result": {"skipped": true, "reason": "SDDIA_LAB_SKIP_GIT"},
        }));
    } else {
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
            git_steps.push(json!({"op": "pull_base", "result": {"skipped": true, "reason": "offline_fetch", "offline": true}}));
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
        let body = format!(
            "---\nfeature_name: {task_name}\ncreated: \"{created}\"\nprocess: {process_label}\nbranch_name: {branch_name}\npersist_ref: {persist_ref}\n---\n\n# Objetivos — {task_name}\n\n## Misión\n\n{summary}\n\n## Alcance (manifiesto)\n\nInicialización de contexto vía orquestador nativo `execute-process` (laboratorio).\n\n## Ley aplicada\n\n- Git exclusivamente vía `skill:git-manager`.\n- Jerarquía: Acción → Agente → Skill → Tools.\n"
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
    }))
}

pub fn is_workspace_init_phase(phase: &Value, inputs: &Value, process_name: &str) -> bool {
    let delegates = phase.get("delegates_to").and_then(|v| v.as_array());
    let Some(delegates) = delegates else {
        return false;
    };
    let has_git = delegates
        .iter()
        .any(|d| d.as_str() == Some("skill:git-manager"));
    if !has_git {
        return false;
    }
    if !matches!(process_name, "feature" | "bug-fix" | "refactorization") {
        return false;
    }
    if phase.get("name").and_then(|v| v.as_str()) != Some("Inicialización de Espacio de Trabajo") {
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
