use execute_process::core::repo::find_repo_root;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn resolve_execute_process(repo: &Path) -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        let p = p.trim();
        if !p.is_empty() {
            return Ok(PathBuf::from(p));
        }
    }
    for rel in ["SddIA/target/debug/execute-process", "SddIA/target/release/execute-process"] {
        let c = repo.join(rel);
        if c.is_file() {
            return Ok(c);
        }
    }
    Err("execute-process no encontrado (cd SddIA && cargo build -p execute-process)".into())
}

pub fn resolve_event_watcher(repo: &Path) -> Result<PathBuf, String> {
    for rel in ["SddIA/target/debug/event-watcher", "SddIA/target/release/event-watcher"] {
        let c = repo.join(rel);
        if c.is_file() {
            return Ok(c);
        }
    }
    let sh = repo.join("SddIA/daemons/event-watcher.sh");
    if sh.is_file() {
        return Ok(sh);
    }
    Err("event-watcher no encontrado".into())
}

pub fn repo_from_cwd() -> Result<PathBuf, String> {
    find_repo_root()
}

pub fn run_json_cmd(cmd: &mut Command) -> Result<serde_json::Value, String> {
    let output = cmd.output().map_err(|e| format!("spawn: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.trim().lines().last().unwrap_or("");
    if line.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(if stderr.trim().is_empty() {
            "sin salida JSON".into()
        } else {
            stderr.trim().to_string()
        });
    }
    let body: serde_json::Value =
        serde_json::from_str(line).map_err(|e| format!("JSON: {e}"))?;
    if body.get("success") == Some(&serde_json::json!(false)) {
        return Err(body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("comando falló")
            .to_string());
    }
    Ok(body)
}

pub fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

pub fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

pub fn print_json_report(report: &serde_json::Value, json_pretty: bool) {
    if json_pretty {
        println!(
            "{}",
            serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".into())
        );
    } else {
        println!("{}", serde_json::to_string(report).unwrap_or_else(|_| "{}".into()));
    }
}
