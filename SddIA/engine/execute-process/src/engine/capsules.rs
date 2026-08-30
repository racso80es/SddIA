//! Invocación de cápsulas skill/tool vía wasmtime o binario nativo (P5).

use super::actions;
use super::capsule_paths;
use serde_json::{json, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub use capsule_paths::{
    anchor_enabled, compute_capsule_source_digest, resolve_capsule_native,
    resolve_capsule_native_checked, resolve_capsule_wasm, write_capsule_witness,
    CapsuleResolveError,
};

const WASM_NATIVE_FALLBACK_MARKERS: &[&str] = &[
    "function not implemented",
    "operation not supported",
    "failed to get current exe",
    "read-only",
    "no se pudo marcar read-only",
    "wasi environment cannot establish outbound",
    "outbound http connections",
];

const GIT_MANAGER_NATIVE_FALLBACK_MARKERS: &[&str] = &[
    "failed to execute git",
    "operation not supported",
];

const SHELL_EXECUTOR_NATIVE_FALLBACK_MARKERS: &[&str] = &[
    "working_directory invalid",
    "executable not found on path",
    "failed to execute",
    "no such file or directory (os error 44)",
];

pub struct CapsuleInvokeResult {
    pub exit_code: i32,
    pub body: Value,
}

fn wasmtime_candidate_paths() -> Vec<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(p) = std::env::var_os("WASMTIME") {
        candidates.push(PathBuf::from(p));
    }
    candidates.push(PathBuf::from("wasmtime"));
    if let Ok(home) = std::env::var("HOME") {
        candidates.push(PathBuf::from(format!("{home}/.wasmtime/bin/wasmtime")));
        candidates.push(PathBuf::from(format!("{home}/.cargo/bin/wasmtime")));
        candidates.push(PathBuf::from(format!("{home}/.local/bin/wasmtime")));
    }
    candidates.push(PathBuf::from("/usr/local/bin/wasmtime"));
    candidates
}

fn wasmtime_executable() -> Option<PathBuf> {
    for c in wasmtime_candidate_paths() {
        let looks_ok = if c.as_os_str() == "wasmtime" {
            Command::new("wasmtime")
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        } else {
            c.is_file()
                && Command::new(&c)
                    .arg("--version")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
        };
        if looks_ok {
            return Some(if c.as_os_str() == "wasmtime" {
                PathBuf::from("wasmtime")
            } else {
                c
            });
        }
    }
    None
}

fn has_wasmtime() -> bool {
    wasmtime_executable().is_some()
}

fn resolve_capsule(
    repo: &Path,
    name: &str,
    prefer_wasm: bool,
    entity_label: &str,
) -> Result<(String, PathBuf), String> {
    let wasm = capsule_paths::resolve_capsule_wasm(repo, name);
    let native = match capsule_paths::resolve_capsule_native_checked(repo, name) {
        Ok(p) => Some(p),
        Err(capsule_paths::CapsuleResolveError::NotFound) => None,
        Err(capsule_paths::CapsuleResolveError::StaleHash { message }) => return Err(message),
    };
    if prefer_wasm {
        if let Some(w) = wasm.as_ref() {
            if has_wasmtime() {
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
    Err(format!(
        "cápsula {entity_label} '{name}' no encontrada bajo SddIA/target"
    ))
}

fn blob_contains_markers(blob: &str, markers: &[&str]) -> bool {
    let lower = blob.to_lowercase();
    markers.iter().any(|m| lower.contains(m))
}

pub fn invoke_capsule_subprocess(
    repo: &Path,
    kind: &str,
    path: &Path,
    stdin_payload: &str,
) -> Result<(String, String, i32), String> {
    let wt = wasmtime_executable();
    let (kind, bin): (&str, PathBuf) = if kind == "wasm" {
        if wt.is_some() {
            ("wasm", path.to_path_buf())
        } else if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
            match capsule_paths::resolve_capsule_native(repo, name) {
                Some(n) => ("native", n),
                None => {
                    return Err(
                        "wasmtime not in PATH; install wasmtime to run WASI capsules".into(),
                    );
                }
            }
        } else {
            return Err("wasmtime not in PATH; install wasmtime to run WASI capsules".into());
        }
    } else {
        (kind, path.to_path_buf())
    };
    let mut cmd = if kind == "wasm" {
        let wt = wt.expect("wasm kind implies wasmtime_executable");
        let mut c = Command::new(wt);
        c.args(["run", "--dir=.", &bin.to_string_lossy()]);
        c
    } else {
        Command::new(&bin)
    };
    cmd.current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("spawn capsule: {e}"))?;
    if let Some(mut stdin) = child.stdin.take() {
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

pub fn parse_capsule_stdout(stdout: &str) -> Result<Value, String> {
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

fn unwrap_shell_executor_body(body: &Value) -> Result<Value, String> {
    let inner = body.get("result").unwrap_or(body);
    let data = inner.get("data").cloned().unwrap_or(json!({}));
    if inner.get("success").and_then(|v| v.as_bool()) == Some(true) {
        return Ok(if data.is_object() {
            data
        } else {
            json!({})
        });
    }
    let err = inner
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("shell-executor failed");
    if data.is_object() && !data.as_object().unwrap().is_empty() {
        let mut enriched = data;
        if let Some(obj) = enriched.as_object_mut() {
            obj.insert("_shell_error".into(), json!(err));
            if let Some(code) = inner.get("exitCode") {
                obj.insert("_shell_exit_code".into(), code.clone());
            }
        }
        return Ok(enriched);
    }
    Err(err.to_string())
}

pub fn unwrap_tool_body(body: &Value) -> Value {
    if let Some(result) = body.get("result") {
        return result.clone();
    }
    body.clone()
}

fn finalize_capsule_body(mut body: Value, rc: i32) -> CapsuleInvokeResult {
    if let Some(exit_code) = body.get("exitCode").and_then(|v| v.as_i64()) {
        let code = exit_code as i32;
        if body.get("success").is_none() {
            body["success"] = json!(code == 0);
        }
        return CapsuleInvokeResult {
            exit_code: code,
            body,
        };
    }
    if body.get("success").is_none() {
        body["success"] = json!(rc == 0);
    } else if body.get("success") == Some(&json!(false)) && rc == 0 {
        return CapsuleInvokeResult {
            exit_code: 1,
            body,
        };
    }
    CapsuleInvokeResult {
        exit_code: if body.get("success") == Some(&json!(true)) {
            0
        } else {
            rc.max(1)
        },
        body,
    }
}

pub fn invoke_capsule_json(
    repo: &Path,
    name: &str,
    payload: &Value,
    prefer_wasm: bool,
) -> Result<CapsuleInvokeResult, String> {
    invoke_capsule_json_labeled(repo, name, payload, prefer_wasm, "skill")
}

pub fn invoke_tool_capsule_json(
    repo: &Path,
    name: &str,
    payload: &Value,
    prefer_wasm: bool,
) -> Result<CapsuleInvokeResult, String> {
    invoke_capsule_json_labeled(repo, name, payload, prefer_wasm, "tool")
}

fn invoke_capsule_json_labeled(
    repo: &Path,
    name: &str,
    payload: &Value,
    prefer_wasm: bool,
    entity_label: &str,
) -> Result<CapsuleInvokeResult, String> {
    let stdin_payload = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    let (kind, path) = resolve_capsule(repo, name, prefer_wasm, entity_label)?;
    let (stdout, stderr, rc) = invoke_capsule_subprocess(repo, &kind, &path, &stdin_payload)?;
    let mut body = if stdout.is_empty() {
        json!({
            "success": false,
            "error": if stderr.is_empty() { "cápsula sin salida".to_string() } else { stderr.clone() },
        })
    } else {
        parse_capsule_stdout(&stdout).unwrap_or_else(|e| {
            json!({"success": false, "error": e, "parse_error": stdout.chars().take(200).collect::<String>()})
        })
    };
    body = unwrap_tool_envelope(body);
    let result = finalize_capsule_body(body, rc);

    if prefer_wasm
        && kind.as_str() == "wasm"
        && capsule_paths::resolve_capsule_native(repo, name).is_some()
        && (result.body.get("success") != Some(&json!(true)) || result.exit_code != 0)
    {
        let err_blob = format!(
            "{} {}",
            result.body.get("error").and_then(|v| v.as_str()).unwrap_or(""),
            result.body.get("message").and_then(|v| v.as_str()).unwrap_or("")
        );
        if blob_contains_markers(&err_blob, WASM_NATIVE_FALLBACK_MARKERS) {
            return invoke_capsule_json_labeled(repo, name, payload, false, entity_label);
        }
    }
    Ok(result)
}

fn unwrap_tool_envelope(body: Value) -> Value {
    let Some(inner) = body.get("result").cloned() else {
        return body;
    };
    let Some(inner_obj) = inner.as_object() else {
        return body;
    };
    if !inner_obj.keys().any(|k| {
        matches!(
            k.as_str(),
            "emitted" | "telemetry_receipt" | "event" | "message" | "error"
        )
    }) {
        return body;
    }
    let mut merged = inner;
    if let Some(nested) = body.get("result").and_then(|r| r.get("result")).cloned() {
        if let Some(obj) = merged.as_object_mut() {
            obj.insert("result".into(), nested);
        }
    }
    merged
}

pub fn invoke_tool(repo: &Path, tool_name: &str, payload: &Value) -> Result<Value, String> {
    let result = invoke_tool_capsule_json(repo, tool_name, payload, true)?;
    if result.exit_code != 0 || result.body.get("success") == Some(&json!(false)) {
        return Err(result
            .body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("tool failed")
            .to_string());
    }
    Ok(unwrap_tool_body(&result.body))
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

    fn run_git(
        repo: &Path,
        kind: &str,
        path: &Path,
        stdin_payload: &str,
    ) -> Result<Value, String> {
        let (stdout, stderr, _) = invoke_capsule_subprocess(repo, kind, path, stdin_payload)?;
        if stdout.is_empty() {
            return Err(if stderr.is_empty() {
                "git-manager sin salida".into()
            } else {
                stderr
            });
        }
        unwrap_git_manager_body(&parse_capsule_stdout(&stdout)?)
    }

    let (kind, path) = resolve_capsule(repo, "git-manager", true, "skill")?;
    if kind.as_str() == "wasm" {
        let (stdout, stderr, _) =
            invoke_capsule_subprocess(repo, "wasm", &path, &stdin_payload)?;
        if blob_contains_markers(&format!("{stderr}\n{stdout}"), GIT_MANAGER_NATIVE_FALLBACK_MARKERS)
        {
            if let Some(native) = capsule_paths::resolve_capsule_native(repo, "git-manager") {
                return run_git(repo, "native", &native, &stdin_payload);
            }
        }
        if stdout.is_empty() {
            if blob_contains_markers(&stderr, GIT_MANAGER_NATIVE_FALLBACK_MARKERS) {
                if let Some(native) = capsule_paths::resolve_capsule_native(repo, "git-manager") {
                    return run_git(repo, "native", &native, &stdin_payload);
                }
            }
            return Err(if stderr.is_empty() {
                "git-manager sin salida".into()
            } else {
                stderr
            });
        }
        return unwrap_git_manager_body(&parse_capsule_stdout(&stdout)?);
    }
    run_git(repo, "native", &path, &stdin_payload)
}

pub fn invoke_shell_executor(
    repo: &Path,
    executable: &str,
    arguments: &[String],
) -> Result<Value, String> {
    let req = json!({
        "executable": executable,
        "arguments": arguments,
        "working_directory": repo.canonicalize().unwrap_or_else(|_| repo.to_path_buf()).to_string_lossy(),
        "environment_vars": {},
    });
    let stdin_payload = serde_json::to_string(&req).map_err(|e| e.to_string())?;

    fn run_shell(
        repo: &Path,
        kind: &str,
        path: &Path,
        stdin_payload: &str,
    ) -> Result<Value, String> {
        let (stdout, stderr, _) = invoke_capsule_subprocess(repo, kind, path, stdin_payload)?;
        if stdout.is_empty() {
            return Err(if stderr.is_empty() {
                "shell-executor sin salida".into()
            } else {
                stderr
            });
        }
        let body = parse_capsule_stdout(&stdout)?;
        if body.get("success") != Some(&json!(true)) {
            let err = body
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("shell-executor failed");
            if blob_contains_markers(
                &format!("{stderr}\n{stdout}\n{err}"),
                SHELL_EXECUTOR_NATIVE_FALLBACK_MARKERS,
            ) {
                return Err("shell-executor wasm fallback marker".into());
            }
        }
        unwrap_shell_executor_body(&body)
    }

    let (kind, path) = resolve_capsule(repo, "shell-executor", true, "skill")?;
    if kind.as_str() == "wasm" {
        match run_shell(repo, "wasm", &path, &stdin_payload) {
            Ok(v) => return Ok(v),
            Err(_) => {
                if let Some(native) = capsule_paths::resolve_capsule_native(repo, "shell-executor") {
                    return run_shell(repo, "native", &native, &stdin_payload);
                }
            }
        }
    }
    run_shell(repo, &kind, &path, &stdin_payload)
}

pub fn invoke_action(repo: &Path, action_name: &str, action_inputs: &Value) -> Result<Value, String> {
    if let Some(data) = actions::try_run_native(repo, action_name, action_inputs)? {
        return Ok(data);
    }
    if let Ok(result) = invoke_capsule_json(repo, action_name, action_inputs, true) {
        if result.exit_code == 0 && result.body.get("success") != Some(&json!(false)) {
            let inner = result.body.get("data").cloned().unwrap_or(result.body);
            return Ok(inner);
        }
    }
    Err(format!(
        "acción '{action_name}' sin handler nativo ni cápsula resuelta"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasmtime_candidates_include_home_dot_wasmtime() {
        let Ok(home) = std::env::var("HOME") else {
            return;
        };
        let want = PathBuf::from(format!("{home}/.wasmtime/bin/wasmtime"));
        assert!(
            wasmtime_candidate_paths().contains(&want),
            "GitHub Desktop PATH no incluye ~/.wasmtime/bin"
        );
    }

    #[test]
    fn parse_capsule_last_line_json() {
        let stdout = "log line\n{\"success\":true,\"data\":{}}\n";
        let v = parse_capsule_stdout(stdout).unwrap();
        assert_eq!(v.get("success"), Some(&json!(true)));
    }

    #[test]
    fn finalize_capsule_respects_exit_code_field() {
        let body = json!({"success": true, "exitCode": 0});
        let r = finalize_capsule_body(body, 1);
        assert_eq!(r.exit_code, 0);
    }
}
