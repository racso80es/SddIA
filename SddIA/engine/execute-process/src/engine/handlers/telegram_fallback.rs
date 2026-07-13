//! Handler nativo `telegram-fallback-responder` (P4).

use super::mayeuta::{filter_c_should_abort, synthesize_mayeuta_response};
use super::super::capsules::{resolve_capsule_native, resolve_capsule_wasm};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn resolve_telegram_tool(repo: &Path) -> Option<(String, std::path::PathBuf)> {
    if let Some(native) = resolve_capsule_native(repo, "send-telegram-notification") {
        return Some(("native".into(), native));
    }
    if let Some(wasm) = resolve_capsule_wasm(repo, "send-telegram-notification") {
        if Command::new("wasmtime")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some(("wasm".into(), wasm));
        }
    }
    None
}

fn invoke_send_telegram_notification(
    repo: &Path,
    message: &str,
) -> Result<(bool, Value), String> {
    let req = json!({"message": message});
    let payload = serde_json::to_string(&req).map_err(|e| e.to_string())?;
    let Some((kind, path)) = resolve_telegram_tool(repo) else {
        return Ok((
            false,
            json!({"error": "cápsula send-telegram-notification no encontrada"}),
        ));
    };

    let output = match kind.as_str() {
        "wasm" => {
            let mut cmd = Command::new("wasmtime");
            cmd.args(["run", "--dir=.", &path.to_string_lossy()]);
            cmd.current_dir(repo)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            let mut child = cmd.spawn().map_err(|e| format!("spawn wasm telegram: {e}"))?;
            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(payload.as_bytes())
                    .map_err(|e| format!("stdin wasm telegram: {e}"))?;
            }
            child
                .wait_with_output()
                .map_err(|e| format!("wait wasm telegram: {e}"))?
        }
        _ => {
            let mut child = Command::new(&path)
                .current_dir(repo)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("spawn native telegram: {e}"))?;
            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(payload.as_bytes())
                    .map_err(|e| format!("stdin native telegram: {e}"))?;
            }
            child
                .wait_with_output()
                .map_err(|e| format!("wait native telegram: {e}"))?
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().last().unwrap_or("").trim();
    let body: Value = if line.is_empty() {
        json!({})
    } else {
        serde_json::from_str(line).unwrap_or(json!({}))
    };
    let ok = output.status.success() && body.get("success").and_then(|v| v.as_bool()) != Some(false);
    Ok((ok, body))
}

fn build_phases(filtered: bool, _synthesized: bool, notified: bool) -> Value {
    json!([
        {
            "phase_name": "Filtro C",
            "status": "executed",
            "handler": "telegram-fallback-responder-core",
            "filtered": filtered,
        },
        {
            "phase_name": "Síntesis",
            "status": if filtered { "skipped" } else { "executed" },
            "handler": "telegram-fallback-responder-core",
        },
        {
            "phase_name": "Materialización",
            "status": if filtered {
                "skipped"
            } else if notified {
                "executed"
            } else {
                "failed"
            },
            "handler": "telegram-fallback-responder-core",
            "notified": notified,
        }
    ])
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let text = process_inputs
        .get("text")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "text requerido".to_string())?;

    if filter_c_should_abort(text) {
        return Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({
                "ok": true,
                "filtered": true,
                "synthesized": false,
                "notified": false,
                "reason": "filtro_c_abort",
            })),
            error: None,
            execution_report: Some(json!({
                "process_name": "telegram-fallback-responder",
                "phases": build_phases(true, false, false),
            })),
            exit_code: 0,
        });
    }

    let synthesized = synthesize_mayeuta_response(text);
    let chat_id = process_inputs
        .get("chat_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| {
            std::env::var("TELEGRAM_ALLOWED_CHAT_ID")
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        });

    if chat_id.is_none() {
        return Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            data: Some(json!({
                "ok": false,
                "filtered": false,
                "synthesized": true,
                "notified": false,
                "error": "chat_id ausente",
            })),
            error: Some("chat_id ausente".into()),
            execution_report: Some(json!({
                "process_name": "telegram-fallback-responder",
                "phases": build_phases(false, true, false),
            })),
            exit_code: 1,
        });
    }

    let (notified, tool_result) = invoke_send_telegram_notification(repo, &synthesized)?;
    let preview = synthesized
        .lines()
        .next()
        .unwrap_or("")
        .chars()
        .take(80)
        .collect::<String>();
    let ok = notified;

    Ok(OrchestratorEnvelope {
        success: ok,
        status_code: if ok { 0 } else { 1 },
        data: Some(json!({
            "ok": ok,
            "filtered": false,
            "synthesized": true,
            "notified": notified,
            "chat_id": chat_id,
            "message_preview": preview,
            "tool_result": tool_result,
            "error": if ok { Value::Null } else { tool_result.get("error").cloned().unwrap_or(json!("notify failed")) },
        })),
        error: if ok {
            None
        } else {
            tool_result
                .get("error")
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .or_else(|| Some("notify failed".into()))
        },
        execution_report: Some(json!({
            "process_name": "telegram-fallback-responder",
            "phases": build_phases(false, true, notified),
        })),
        exit_code: if ok { 0 } else { 1 },
    })
}
