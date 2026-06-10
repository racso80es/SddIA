use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::json;

fn main() {
    let req = read_stdin_json();

    let mode = match req.get("corruption_mode").and_then(|v| v.as_str()) {
        Some(s) => s.trim().to_lowercase(),
        None => "empty".to_string(),
    };

    if mode != "empty" && mode != "invalid_json" && mode != "partial" {
        emit_error("corruption_mode invalido: empty | invalid_json | partial", 1);
        return;
    }

    let mut result = json!({
        "corruption_mode": mode
    });

    let mut envelope = json!({
        "success": true,
        "exitCode": 0,
        "message": format!("recibo corrupto modo={}", mode),
        "result": result
    });

    if mode == "invalid_json" {
        envelope.as_object_mut().unwrap().insert("telemetry_receipt".to_string(), json!("not-valid-json{{"));
    } else if mode == "partial" {
        envelope.as_object_mut().unwrap().insert("telemetry_receipt".to_string(), json!({"prompt_tokens": 1}));
    }

    emit_success(Some(envelope));
}
