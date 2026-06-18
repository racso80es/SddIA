//! Invocación recursiva al binario orquestador (subprocesos internos).

use serde_json::Value;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn resolve_orchestrator_bin(repo: &Path) -> Result<std::path::PathBuf, String> {
    if let Ok(p) = std::env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        let p = p.trim();
        if !p.is_empty() {
            return Ok(std::path::PathBuf::from(p));
        }
    }
    for rel in ["SddIA/target/debug/execute-process", "SddIA/target/release/execute-process"] {
        let candidate = repo.join(rel);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err("binario execute-process no encontrado (compilar: cd SddIA && cargo build -p execute-process)".into())
}

pub fn invoke_process(repo: &Path, process_name: &str, inputs: &Value) -> Result<Value, String> {
    let bin = resolve_orchestrator_bin(repo)?;
    let inputs_json = serde_json::to_string(inputs).map_err(|e| e.to_string())?;
    let output = Command::new(&bin)
        .args(["--process", process_name, "--inputs", &inputs_json])
        .current_dir(repo)
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
    let bin = resolve_orchestrator_bin(repo)?;
    let inputs_json = serde_json::to_string(inputs).map_err(|e| e.to_string())?;
    let output = Command::new(&bin)
        .args(["--process", process_name, "--inputs", &inputs_json])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("spawn orquestador: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().filter(|l| !l.trim().is_empty()).last().unwrap_or("");
    if line.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    serde_json::from_str(line).map_err(|e| format!("JSON envelope: {e}"))
}

/// Invoca bridge Python legacy (solo procesos residuales no portados).
pub fn invoke_capsules_bridge(repo: &Path, process_name: &str, inputs: &Value) -> Result<Value, String> {
    let bridge = repo.join("SddIA/scripts/qa/_execute_process_capsules_bridge.py");
    if !bridge.is_file() {
        return Err(format!("bridge capsules ausente: {}", bridge.display()));
    }
    let payload = serde_json::json!({
        "process": process_name,
        "inputs": inputs,
    });
    let py = std::env::var("PYTHON").unwrap_or_else(|_| "python3".into());
    let mut child = Command::new(&py)
        .arg(&bridge)
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn capsules bridge: {e}"))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(serde_json::to_string(&payload).unwrap().as_bytes())
            .map_err(|e| e.to_string())?;
    }
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().filter(|l| !l.trim().is_empty()).last().unwrap_or("");
    if line.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    let body: Value = serde_json::from_str(line).map_err(|e| format!("JSON bridge: {e}"))?;
    Ok(body)
}
