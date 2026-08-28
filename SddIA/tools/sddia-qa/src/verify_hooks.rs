//! Verificación de `core.hooksPath` contra el directorio versionado de hooks SddIA.

use serde_json::json;
use std::path::Path;
use std::process::Command;

const EXPECTED_HOOKS_REL: &str = "SddIA/scripts/qa/git-hooks";

fn git_config(repo: &Path, key: &str) -> Option<String> {
    let out = Command::new("git")
        .args(["config", "--get", key])
        .current_dir(repo)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub fn run(repo: &Path, json_out: bool) -> i32 {
    let configured = git_config(repo, "core.hooksPath");
    let mut findings: Vec<String> = Vec::new();

    if configured.as_deref() != Some(EXPECTED_HOOKS_REL) {
        findings.push(format!(
            "core.hooksPath debe ser '{EXPECTED_HOOKS_REL}' (actual: {:?}). Remedio: git config core.hooksPath {EXPECTED_HOOKS_REL}",
            configured
        ));
    }

    let hooks_dir = repo.join(EXPECTED_HOOKS_REL);
    for name in ["pre-commit", "pre-push", "post-merge"] {
        if !hooks_dir.join(name).is_file() {
            findings.push(format!(
                "Falta dispatcher versionado: {EXPECTED_HOOKS_REL}/{name}"
            ));
        }
    }

    let success = findings.is_empty();
    let body = json!({
        "success": success,
        "exitCode": if success { 0 } else { 1 },
        "message": if success {
            "hooks OK".into()
        } else {
            findings.join("; ")
        },
        "result": {
            "core.hooksPath": configured,
            "expected": EXPECTED_HOOKS_REL,
            "findings": findings,
        }
    });

    if json_out {
        println!("{}", serde_json::to_string(&body).unwrap_or_else(|_| "{}".into()));
    } else {
        eprintln!("{}", body["message"]);
    }
    if success {
        0
    } else {
        1
    }
}
