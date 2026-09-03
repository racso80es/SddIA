use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::Value;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let input = read_stdin_json();

    let request_payload = match input.get("request") {
        Some(req) => req.clone(),
        None => {
            emit_error("Missing 'request' in input JSON envelope", 1);
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

    let cli_path = match env::var("ANTIGRAVITY_CLI_PATH") {
        Ok(path) => path,
        Err(_) => {
            emit_error("ANTIGRAVITY_CLI_PATH environment variable not set", 1);
            return;
        }
    };

    let mut child = match Command::new(&cli_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            emit_error(&format!("Failed to spawn CLI process: {}", e), 1);
            return;
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(request_string.as_bytes()) {
            emit_error(&format!("Failed to write to CLI stdin: {}", e), 1);
            return;
        }
    }

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            emit_error(&format!("Failed to wait on CLI process: {}", e), 1);
            return;
        }
    };

    if output.status.success() {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        // Try parsing the CLI stdout as JSON, else return it as a string payload
        let result_payload = serde_json::from_str::<Value>(&stdout_str)
            .unwrap_or_else(|_| serde_json::json!({ "output": stdout_str.to_string() }));
        emit_success(Some(result_payload));
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        emit_error(
            &format!("CLI process failed ({}): {}", output.status, stderr_str),
            output.status.code().unwrap_or(1),
        );
    }
}
