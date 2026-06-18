//! Delegación al motor Python legacy (`execute_process_capsules.run_process`).
//! Deuda Fase C: sustituir progresivamente por handlers nativos Rust.

use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn python_bin() -> String {
    std::env::var("PYTHON").unwrap_or_else(|_| "python3".into())
}

pub fn run_process(
    repo: &Path,
    process_name: &str,
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let bridge = repo.join("SddIA/scripts/qa/_execute_process_engine_bridge.py");
    if !bridge.is_file() {
        return Err(format!("bridge engine ausente: {}", bridge.display()));
    }

    let payload = json!({
        "process": process_name,
        "inputs": process_inputs,
    });

    let mut child = Command::new(python_bin())
        .arg(&bridge)
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn engine bridge: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        stdin
            .write_all(body.as_bytes())
            .map_err(|e| format!("stdin bridge: {e}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("wait bridge: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let line = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .last()
        .ok_or_else(|| {
            if stderr.trim().is_empty() {
                "sin salida del motor Python".to_string()
            } else {
                stderr.trim().to_string()
            }
        })?;

    let body: Value = serde_json::from_str(line).map_err(|e| format!("JSON inválido del motor: {e}"))?;
    Ok(OrchestratorEnvelope::from_value(body))
}
