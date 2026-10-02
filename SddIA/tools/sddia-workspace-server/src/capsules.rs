use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

pub fn resolve_native_capsule(sddia_repo: &Path, name: &str) -> Option<PathBuf> {
    for profile in ["release", "debug"] {
        let p = sddia_repo
            .join("SddIA/target")
            .join(profile)
            .join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

pub fn invoke_skill(
    sddia_repo: &Path,
    skill: &str,
    stdin_payload: &Value,
    child_env: &HashMap<String, String>,
) -> Result<(Value, i32, u128), String> {
    let bin = resolve_native_capsule(sddia_repo, skill)
        .ok_or_else(|| format!("capsule not found: {skill}"))?;
    let stdin_s = serde_json::to_string(stdin_payload).map_err(|e| e.to_string())?;
    let start = Instant::now();
    let mut cmd = Command::new(&bin);
    cmd.current_dir(sddia_repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env_clear();
    for (k, v) in child_env {
        cmd.env(k, v);
    }
    cmd.env("PATH", std::env::var("PATH").unwrap_or_default());
    let mut child = cmd.spawn().map_err(|e| format!("spawn {skill}: {e}"))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(stdin_s.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    let ms = start.elapsed().as_millis();
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let line = stdout.lines().last().unwrap_or("").trim();
    let body = if line.is_empty() {
        json!({"success": false, "error": "empty stdout"})
    } else {
        serde_json::from_str(line).unwrap_or_else(|_| {
            json!({"success": false, "error": "invalid json", "raw": line})
        })
    };
    let code = out.status.code().unwrap_or(1);
    let exit = body
        .get("exitCode")
        .and_then(|v| v.as_i64())
        .map(|n| n as i32)
        .unwrap_or(code);
    Ok((body, exit, ms))
}

pub fn git_invoke(
    sddia_repo: &Path,
    project_root: &Path,
    op: &str,
    payload: &Value,
    child_env: &HashMap<String, String>,
) -> Result<(Value, i32, u128), String> {
    let req = json!({
        "operation_type": op,
        "repository_path": project_root.to_string_lossy(),
        "operation_payload_json": payload,
    });
    invoke_skill(sddia_repo, "git-manager", &req, child_env)
}
