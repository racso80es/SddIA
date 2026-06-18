use serde::Serialize;
use serde_json::{json, Value};
use std::io::{self, Write};
use std::process;

/// Envelope rico del orquestador (paridad con `execute_process_core.emit`).
#[derive(Debug, Clone, Serialize)]
pub struct OrchestratorEnvelope {
    pub success: bool,
    pub status_code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_report: Option<Value>,
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
}

impl OrchestratorEnvelope {
    pub fn from_value(v: Value) -> Self {
        let success = v.get("success").and_then(|x| x.as_bool()).unwrap_or(false);
        let status_code = v
            .get("status_code")
            .and_then(|x| x.as_i64())
            .unwrap_or(if success { 0 } else { 1 }) as i32;
        let exit_code = v
            .get("exitCode")
            .and_then(|x| x.as_i64())
            .unwrap_or(status_code as i64) as i32;
        Self {
            success,
            status_code,
            data: v.get("data").cloned(),
            error: v
                .get("error")
                .and_then(|x| x.as_str())
                .map(str::to_string),
            execution_report: v.get("execution_report").cloned(),
            exit_code,
        }
    }

    pub fn failure(error: impl Into<String>, status_code: i32) -> Self {
        Self {
            success: false,
            status_code,
            data: None,
            error: Some(error.into()),
            execution_report: None,
            exit_code: status_code,
        }
    }
}

pub fn emit(envelope: OrchestratorEnvelope) -> ! {
    let code = envelope.exit_code;
    let line = serde_json::to_string(&envelope).unwrap_or_else(|_| {
        json!({"success": false, "status_code": 1, "error": "serialize envelope", "exitCode": 1})
            .to_string()
    });
    let mut out = io::stdout();
    let _ = writeln!(out, "{line}");
    let _ = out.flush();
    process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_serializes_exit_code_field() {
        let env = OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({"ok": true})),
            error: None,
            execution_report: Some(json!({"process_name": "kalma2-interact", "phases": []})),
            exit_code: 0,
        };
        let s = serde_json::to_string(&env).unwrap();
        assert!(s.contains("\"exitCode\":0"));
        assert!(s.contains("\"execution_report\""));
    }
}
