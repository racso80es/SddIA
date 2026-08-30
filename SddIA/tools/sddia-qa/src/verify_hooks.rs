//! Verificación y armado de `core.hooksPath` + bit ejecutable de dispatchers SddIA.

use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::Command;

const EXPECTED_HOOKS_REL: &str = "SddIA/scripts/qa/git-hooks";
const DISPATCHERS: &[&str] = &["pre-commit", "pre-push", "post-merge"];

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

pub fn is_executable(path: &Path) -> bool {
    path.metadata()
        .map(|m| {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                m.permissions().mode() & 0o111 != 0
            }
            #[cfg(not(unix))]
            {
                let _ = m;
                true
            }
        })
        .unwrap_or(false)
}

fn hooks_dir(repo: &Path) -> PathBuf {
    repo.join(EXPECTED_HOOKS_REL)
}

fn collect_findings(repo: &Path) -> Vec<String> {
    let mut findings: Vec<String> = Vec::new();
    let configured = git_config(repo, "core.hooksPath");
    if configured.as_deref() != Some(EXPECTED_HOOKS_REL) {
        findings.push(format!(
            "core.hooksPath debe ser '{EXPECTED_HOOKS_REL}' (actual: {:?}). Remedio: sddia-qa verify-hooks --fix",
            configured
        ));
    }

    let hooks = hooks_dir(repo);
    for name in DISPATCHERS {
        let path = hooks.join(name);
        if !path.is_file() {
            findings.push(format!(
                "Falta dispatcher versionado: {EXPECTED_HOOKS_REL}/{name}"
            ));
            continue;
        }
        if !is_executable(&path) {
            findings.push(format!(
                "Dispatcher sin bit ejecutable: {EXPECTED_HOOKS_REL}/{name}. Remedio: sddia-qa verify-hooks --fix"
            ));
        }
    }
    findings
}

fn apply_fix(repo: &Path) -> Result<(), String> {
    if git_config(repo, "core.hooksPath").as_deref() != Some(EXPECTED_HOOKS_REL) {
        let status = Command::new("git")
            .args(["config", "core.hooksPath", EXPECTED_HOOKS_REL])
            .current_dir(repo)
            .status()
            .map_err(|e| format!("git config core.hooksPath: {e}"))?;
        if !status.success() {
            return Err("git config core.hooksPath falló".into());
        }
    }

    let hooks = hooks_dir(repo);
    for name in DISPATCHERS {
        let path = hooks.join(name);
        if !path.is_file() {
            return Err(format!(
                "dispatcher ausente: {EXPECTED_HOOKS_REL}/{name}"
            ));
        }
        if is_executable(&path) {
            continue;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let meta = path
                .metadata()
                .map_err(|e| format!("metadata {name}: {e}"))?;
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&path, perms)
                .map_err(|e| format!("chmod {name}: {e}"))?;
        }
        #[cfg(not(unix))]
        {
            return Err(format!("chmod no soportado en esta plataforma: {name}"));
        }
    }
    Ok(())
}

pub fn run(repo: &Path, json_out: bool, fix: bool) -> i32 {
    if fix {
        match apply_fix(repo) {
            Ok(()) => {}
            Err(e) => {
                let body = json!({
                    "success": false,
                    "exitCode": 1,
                    "message": e,
                    "result": {
                        "fixed": false,
                        "findings": [e.clone()],
                    }
                });
                if json_out {
                    println!("{}", serde_json::to_string(&body).unwrap_or_else(|_| "{}".into()));
                } else {
                    eprintln!("{e}");
                }
                return 1;
            }
        }
    }

    let configured = git_config(repo, "core.hooksPath");
    let findings = collect_findings(repo);
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
            "fixed": fix,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write_dispatcher(dir: &Path, name: &str, executable: bool) {
        let path = dir.join(name);
        fs::write(&path, "#!/usr/bin/env sh\nexit 0\n").expect("write dispatcher");
        if executable {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = path.metadata().expect("meta").permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&path, perms).expect("chmod");
            }
        } else {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = path.metadata().expect("meta").permissions();
                perms.set_mode(0o644);
                fs::set_permissions(&path, perms).expect("chmod");
            }
        }
    }

    fn init_git(repo: &Path) {
        assert!(Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo)
            .status()
            .expect("git init")
            .success());
        Command::new("git")
            .args(["config", "user.email", "verify-hooks@test"])
            .current_dir(repo)
            .status()
            .expect("email");
        Command::new("git")
            .args(["config", "user.name", "verify-hooks"])
            .current_dir(repo)
            .status()
            .expect("name");
    }

    #[test]
    fn detects_missing_executable_bit() {
        let tmp = tempdir().expect("tempdir");
        let repo = tmp.path();
        let hooks = repo.join(EXPECTED_HOOKS_REL);
        fs::create_dir_all(&hooks).expect("mkdir");
        for name in DISPATCHERS {
            write_dispatcher(&hooks, name, false);
        }
        init_git(repo);
        Command::new("git")
            .args(["config", "core.hooksPath", EXPECTED_HOOKS_REL])
            .current_dir(repo)
            .status()
            .expect("hooksPath");

        let findings = collect_findings(repo);
        assert!(
            findings
                .iter()
                .any(|f| f.contains("sin bit ejecutable")),
            "findings: {findings:?}"
        );
        assert_eq!(run(repo, false, false), 1);
    }

    #[test]
    fn fix_is_idempotent() {
        let tmp = tempdir().expect("tempdir");
        let repo = tmp.path();
        let hooks = repo.join(EXPECTED_HOOKS_REL);
        fs::create_dir_all(&hooks).expect("mkdir");
        for name in DISPATCHERS {
            write_dispatcher(&hooks, name, false);
        }
        init_git(repo);

        assert_eq!(run(repo, false, true), 0);
        for name in DISPATCHERS {
            assert!(
                is_executable(&hooks.join(name)),
                "expected +x on {name}"
            );
        }
        assert_eq!(run(repo, false, false), 0);
        assert_eq!(run(repo, false, true), 0);
    }
}
