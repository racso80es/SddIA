//! Invocación recursiva al binario orquestador (subprocesos internos).

use serde_json::Value;
use std::path::Path;
use std::process::Command;

fn elf_mtime(path: &Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

/// F-DEP-07: debug solo si su mtime es estrictamente posterior al de release.
pub fn resolve_orchestrator_bin(repo: &Path) -> Result<std::path::PathBuf, String> {
    if let Ok(p) = std::env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        let p = p.trim();
        if !p.is_empty() {
            return Ok(std::path::PathBuf::from(p));
        }
    }
    let debug = repo.join("SddIA/target/debug/execute-process");
    let release = repo.join("SddIA/target/release/execute-process");
    let chosen = match (debug.is_file(), release.is_file()) {
        (true, true) => {
            let debug_newer = elf_mtime(&debug)
                .zip(elf_mtime(&release))
                .is_some_and(|(d, r)| d > r);
            if debug_newer { debug } else { release }
        }
        (true, false) => debug,
        (false, true) => release,
        (false, false) => {
            return Err(
                "binario execute-process no encontrado (compilar: cd SddIA && cargo build -p execute-process)"
                    .into(),
            );
        }
    };
    Ok(chosen)
}

pub fn invoke_process(repo: &Path, process_name: &str, inputs: &Value) -> Result<Value, String> {
    let bin = resolve_orchestrator_bin(repo)?;
    let inputs_json = serde_json::to_string(inputs).map_err(|e| e.to_string())?;
    let output = Command::new(&bin)
        .args(["--process", process_name, "--inputs", &inputs_json])
        .current_dir(repo)
        .env("SDDIA_CLI_FOREGROUND", "1")
        .output()
        .map_err(|e| format!("spawn orquestador: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let line = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .last()
        .ok_or_else(|| {
            if stderr.trim().is_empty() {
                format!("subproceso {process_name} sin salida")
            } else {
                stderr.trim().to_string()
            }
        })?;
    let body: Value = serde_json::from_str(line).map_err(|e| format!("JSON subproceso: {e}"))?;
    if body.get("success") != Some(&serde_json::json!(true)) {
        return Err(body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("subproceso falló")
            .to_string());
    }
    Ok(body.get("data").cloned().unwrap_or(body))
}

pub fn invoke_process_full(repo: &Path, process_name: &str, inputs: &Value) -> Result<Value, String> {
    invoke_process_full_with_env(repo, process_name, inputs, &[])
}

pub fn invoke_process_full_with_env(
    repo: &Path,
    process_name: &str,
    inputs: &Value,
    extra_env: &[(&str, &str)],
) -> Result<Value, String> {
    let bin = resolve_orchestrator_bin(repo)?;
    let inputs_json = serde_json::to_string(inputs).map_err(|e| e.to_string())?;
    let mut cmd = Command::new(&bin);
    cmd.args(["--process", process_name, "--inputs", &inputs_json])
        .current_dir(repo)
        .env("SDDIA_CLI_FOREGROUND", "1");
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    let output = cmd
        .output()
        .map_err(|e| format!("spawn orquestador: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().filter(|l| !l.trim().is_empty()).last().unwrap_or("");
    if line.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    serde_json::from_str(line).map_err(|e| format!("JSON envelope: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Mutex;
    use std::time::{Duration, SystemTime};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn touch(path: &std::path::Path, when: SystemTime) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, b"elf").unwrap();
        let file = fs::File::options().write(true).open(path).unwrap();
        file.set_modified(when).unwrap();
    }

    #[test]
    fn prefers_release_when_debug_is_older() {
        let _guard = ENV_LOCK.lock().unwrap();
        let prev = std::env::var("SDDIA_EXECUTE_PROCESS_BIN").ok();
        std::env::remove_var("SDDIA_EXECUTE_PROCESS_BIN");
        let root = std::env::temp_dir().join(format!("orch-rel-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let now = SystemTime::now();
        touch(
            &root.join("SddIA/target/debug/execute-process"),
            now - Duration::from_secs(120),
        );
        touch(
            &root.join("SddIA/target/release/execute-process"),
            now - Duration::from_secs(10),
        );
        let got = resolve_orchestrator_bin(&root).unwrap();
        assert!(got.ends_with("release/execute-process"), "{got:?}");
        let _ = fs::remove_dir_all(&root);
        if let Some(v) = prev {
            std::env::set_var("SDDIA_EXECUTE_PROCESS_BIN", v);
        }
    }

    #[test]
    fn prefers_debug_when_strictly_newer() {
        let _guard = ENV_LOCK.lock().unwrap();
        let prev = std::env::var("SDDIA_EXECUTE_PROCESS_BIN").ok();
        std::env::remove_var("SDDIA_EXECUTE_PROCESS_BIN");
        let root = std::env::temp_dir().join(format!("orch-dbg-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let now = SystemTime::now();
        touch(
            &root.join("SddIA/target/release/execute-process"),
            now - Duration::from_secs(120),
        );
        touch(
            &root.join("SddIA/target/debug/execute-process"),
            now - Duration::from_secs(10),
        );
        let got = resolve_orchestrator_bin(&root).unwrap();
        assert!(got.ends_with("debug/execute-process"), "{got:?}");
        let _ = fs::remove_dir_all(&root);
        if let Some(v) = prev {
            std::env::set_var("SDDIA_EXECUTE_PROCESS_BIN", v);
        }
    }
}
