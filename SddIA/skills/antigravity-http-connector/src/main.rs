use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::Value;
use std::env;

fn main() {
    let input = read_stdin_json();

    let request_payload = match input.get("request") {
        Some(req) => req.clone(),
        None => {
            emit_error("Missing 'request' in input JSON envelope", 1);
            return;
        }
    };

    let api_key = match env::var("ANTIGRAVITY_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            emit_error("ANTIGRAVITY_API_KEY environment variable not set", 1);
            return;
        }
    };

    let api_endpoint = match env::var("ANTIGRAVITY_API_ENDPOINT") {
        Ok(endpoint) => endpoint,
        Err(_) => {
            emit_error("ANTIGRAVITY_API_ENDPOINT environment variable not set", 1);
            return;
        }
    };

    let request_string = match serde_json::to_string(&request_payload) {
        Ok(s) => s,
        Err(e) => {
            emit_error(&format!("Failed to serialize request payload: {}", e), 1);
            return;
        }
    };

    // For WASI compatibility we use std::process::Command to invoke curl
    // instead of reqwest::blocking which is unavailable on wasm32-wasi
    let mut command = std::process::Command::new("curl");
    command.arg("-X").arg("POST")
           .arg("-H").arg(format!("Authorization: Bearer {}", api_key))
           .arg("-H").arg("Content-Type: application/json")
           .arg("-d").arg(&request_string)
           .arg(&api_endpoint);

    let output = match command.output() {
        Ok(o) => o,
        Err(e) => {
            emit_error(&format!("Failed to invoke curl: {}", e), 1);
            return;
        }
    };

    if output.status.success() {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let result_payload = serde_json::from_str::<Value>(&stdout_str)
            .unwrap_or_else(|_| serde_json::json!({ "output": stdout_str.to_string() }));
        emit_success(Some(result_payload));
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        emit_error(
            &format!("HTTP Request failed ({}): {}", output.status, stderr_str),
            output.status.code().unwrap_or(1),
        );
    }
}
