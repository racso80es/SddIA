//! Invocación puntual de módulos Python core (EDA fan-out residual).
//! No sustituye al bridge capsules; solo cores aislados hasta porte full nativo.

use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;

fn python_bin() -> String {
    std::env::var("PYTHON").unwrap_or_else(|_| "python3".into())
}

pub fn invoke_route_fractal(
    repo: &Path,
    fn_name: &str,
    event_rel: &str,
) -> Result<Value, String> {
    let code = format!(
        r#"
import json, sys
from pathlib import Path
repo = Path({repo:?}).resolve()
rel = sys.argv[1]
from route_fractal_event_core import {fn_name}
out = {fn_name}(repo, rel)
print(json.dumps(out, ensure_ascii=False))
"#,
        repo = repo.display().to_string(),
        fn_name = fn_name,
    );
    let qa = repo.join("SddIA/scripts/qa");
    let output = Command::new(python_bin())
        .arg("-c")
        .arg(&code)
        .arg(event_rel)
        .current_dir(&qa)
        .env("PYTHONPATH", &qa)
        .output()
        .map_err(|e| format!("spawn route fractal core: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let line = stdout.lines().filter(|l| !l.trim().is_empty()).last().unwrap_or("");
    if line.is_empty() {
        return Err(if stderr.trim().is_empty() {
            "sin salida route fractal core".into()
        } else {
            stderr.trim().to_string()
        });
    }
    serde_json::from_str(line).map_err(|e| format!("JSON route fractal: {e}"))
}

pub fn invoke_radamanto_batch(repo: &Path, event_rel: &str) -> Result<Value, String> {
    let code = r#"
import json, sys
from pathlib import Path
repo = Path(sys.argv[1]).resolve()
rel = sys.argv[2]
from radamanto_batch_core import process_telemetry_file
print(json.dumps(process_telemetry_file(repo, rel), ensure_ascii=False))
"#;
    let qa = repo.join("SddIA/scripts/qa");
    let output = Command::new(python_bin())
        .arg("-c")
        .arg(code)
        .arg(repo)
        .arg(event_rel)
        .current_dir(&qa)
        .env("PYTHONPATH", &qa)
        .output()
        .map_err(|e| format!("spawn radamanto core: {e}"))?;
    parse_json_stdout(&output.stdout, &output.stderr)
}

pub fn invoke_telemetry_compliance(repo: &Path, event_rel: &str) -> Result<Value, String> {
    let code = r#"
import json, sys
from pathlib import Path
repo = Path(sys.argv[1]).resolve()
rel = sys.argv[2]
from telemetry_compliance_audit_core import audit_telemetry_compliance
print(json.dumps(audit_telemetry_compliance(repo, rel), ensure_ascii=False))
"#;
    let qa = repo.join("SddIA/scripts/qa");
    let output = Command::new(python_bin())
        .arg("-c")
        .arg(code)
        .arg(repo)
        .arg(event_rel)
        .current_dir(&qa)
        .env("PYTHONPATH", &qa)
        .output()
        .map_err(|e| format!("spawn telemetry compliance core: {e}"))?;
    parse_json_stdout(&output.stdout, &output.stderr)
}

fn parse_json_stdout(stdout: &[u8], stderr: &[u8]) -> Result<Value, String> {
    let stdout = String::from_utf8_lossy(stdout);
    let stderr = String::from_utf8_lossy(stderr);
    let line = stdout.lines().filter(|l| !l.trim().is_empty()).last().unwrap_or("");
    if line.is_empty() {
        return Err(if stderr.trim().is_empty() {
            "sin salida python core".into()
        } else {
            stderr.trim().to_string()
        });
    }
    serde_json::from_str(line).map_err(|e| format!("JSON python core: {e}"))
}

pub fn run_telemetry_batch_stub(repo: &Path, event_rel: &str) -> Result<Value, String> {
    let event_path = repo.join(event_rel.trim());
    if !event_path.is_file() {
        return Err(format!("no existe: {event_rel}"));
    }
    let text = std::fs::read_to_string(&event_path).map_err(|e| e.to_string())?;
    let body: Value = serde_json::from_str(&text).map_err(|e| format!("JSON inválido: {e}"))?;
    std::fs::remove_file(&event_path).ok();
    Ok(json!({
        "ok": true,
        "event_id": body.get("event_id"),
        "event_type": body.get("event_type"),
        "purged": true,
    }))
}
