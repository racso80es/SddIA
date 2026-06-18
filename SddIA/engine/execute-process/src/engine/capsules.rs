//! Invocación de cápsulas skill/tool vía wasmtime o binario nativo.

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn target_dir(repo: &Path) -> PathBuf {
    repo.join("SddIA/target")
}

pub fn resolve_capsule_wasm(repo: &Path, name: &str) -> Option<PathBuf> {
    let base = target_dir(repo).join("wasm32-wasip1");
    for profile in ["release", "debug"] {
        let p = base.join(profile).join(format!("{name}.wasm"));
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

pub fn resolve_capsule_native(repo: &Path, name: &str) -> Option<PathBuf> {
    let base = target_dir(repo);
    for profile in ["release", "debug"] {
        let p = base.join(profile).join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn resolve_skill(repo: &Path, name: &str, prefer_wasm: bool) -> Result<(String, PathBuf), String> {
    let wasm = resolve_capsule_wasm(repo, name);
    let native = resolve_capsule_native(repo, name);
    let has_wasmtime = Command::new("wasmtime")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if prefer_wasm {
        if let Some(w) = wasm.as_ref() {
            if has_wasmtime {
                return Ok(("wasm".into(), w.clone()));
            }
        }
    }
    if let Some(n) = native {
        return Ok(("native".into(), n));
    }
    if let Some(w) = wasm {
        return Ok(("wasm".into(), w));
    }
    Err(format!("cápsula skill '{name}' no encontrada bajo SddIA/target"))
}

fn invoke_capsule(
    repo: &Path,
    kind: &str,
    path: &Path,
    stdin_payload: &str,
) -> Result<(String, String, i32), String> {
    let mut cmd = if kind == "wasm" {
        let mut c = Command::new("wasmtime");
        c.args(["run", "--dir=.", &path.to_string_lossy()]);
        c
    } else {
        Command::new(path)
    };
    cmd.current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("spawn capsule: {e}"))?;
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin
            .write_all(stdin_payload.as_bytes())
            .map_err(|e| format!("stdin capsule: {e}"))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("wait capsule: {e}"))?;
    Ok((
        String::from_utf8_lossy(&out.stdout).trim().to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
        out.status.code().unwrap_or(1),
    ))
}

fn parse_capsule_stdout(stdout: &str) -> Result<Value, String> {
    let line = stdout.lines().last().unwrap_or("").trim();
    if line.is_empty() {
        return Err("cápsula sin salida".into());
    }
    serde_json::from_str(line).map_err(|e| format!("JSON cápsula inválido: {e}"))
}

fn unwrap_git_manager_body(body: &Value) -> Result<Value, String> {
    let inner = body.get("result").unwrap_or(body);
    if inner.get("success").and_then(|v| v.as_bool()) == Some(true) {
        return Ok(inner.get("data").cloned().unwrap_or(json!({})));
    }
    if inner
        .get("data")
        .and_then(|d| d.get("offline"))
        .and_then(|v| v.as_bool())
        == Some(true)
    {
        return Ok(inner.get("data").cloned().unwrap_or(json!({})));
    }
    Err(inner
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("git-manager failed")
        .to_string())
}

fn unwrap_tool_body(body: &Value) -> Value {
    if let Some(result) = body.get("result") {
        return result.clone();
    }
    body.clone()
}

pub fn invoke_tool(repo: &Path, tool_name: &str, payload: &Value) -> Result<Value, String> {
    let stdin_payload = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    let (kind, path) = resolve_skill(repo, tool_name, true)?;
    let (stdout, stderr, _code) = invoke_capsule(repo, &kind, &path, &stdin_payload)?;
    if stdout.is_empty() {
        return Err(if stderr.is_empty() {
            format!("tool '{tool_name}' sin salida")
        } else {
            stderr
        });
    }
    Ok(unwrap_tool_body(&parse_capsule_stdout(&stdout)?))
}

pub fn invoke_git_manager(
    repo: &Path,
    operation_type: &str,
    payload: &Value,
) -> Result<Value, String> {
    let req = json!({
        "operation_type": operation_type,
        "repository_path": repo.canonicalize().unwrap_or_else(|_| repo.to_path_buf()).to_string_lossy(),
        "operation_payload_json": payload,
    });
    let stdin_payload = serde_json::to_string(&req).map_err(|e| e.to_string())?;
    let (kind, path) = resolve_skill(repo, "git-manager", true)?;
    let (stdout, stderr, _code) = invoke_capsule(repo, &kind, &path, &stdin_payload)?;
    if stdout.is_empty() {
        return Err(if stderr.is_empty() {
            "git-manager sin salida".into()
        } else {
            stderr
        });
    }
    let body = parse_capsule_stdout(&stdout)?;
    unwrap_git_manager_body(&body)
}
