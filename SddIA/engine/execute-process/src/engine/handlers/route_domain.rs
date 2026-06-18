//! Handler `route-domain-event` (P4) — entry nativo; core EDA vía bridge Python.

use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn python_bin() -> String {
    std::env::var("PYTHON").unwrap_or_else(|_| "python3".into())
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let bridge = repo.join("SddIA/scripts/qa/_execute_process_route_bridge.py");
    if !bridge.is_file() {
        return Err(format!("route bridge ausente: {}", bridge.display()));
    }
    let payload = json!({
        "process": "route-domain-event",
        "inputs": process_inputs,
    });
    let mut child = Command::new(python_bin())
        .arg(&bridge)
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn route bridge: {e}"))?;
    if let Some(mut stdin) = child.stdin.take() {
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        stdin
            .write_all(body.as_bytes())
            .map_err(|e| format!("stdin route bridge: {e}"))?;
    }
    let output = child
        .wait_with_output()
        .map_err(|e| format!("wait route bridge: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let line = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .last()
        .ok_or_else(|| {
            if stderr.trim().is_empty() {
                "sin salida del route bridge".to_string()
            } else {
                stderr.trim().to_string()
            }
        })?;
    let body: Value = serde_json::from_str(line).map_err(|e| format!("JSON route bridge: {e}"))?;
    Ok(OrchestratorEnvelope::from_value(body))
}
