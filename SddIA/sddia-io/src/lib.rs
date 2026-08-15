pub mod outbound_lab;

use std::io::{self, Read};
use std::process;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct SddiaResponse<T> {
    pub success: bool,
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub fn read_stdin_json() -> Value {
    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).is_err() {
        emit_error("Failed to read stdin", 1);
        process::exit(1);
    }

    let buffer = buffer.trim();
    if buffer.is_empty() {
        return serde_json::json!({});
    }

    match serde_json::from_str(buffer) {
        Ok(val) => val,
        Err(e) => {
            emit_error(&format!("invalid JSON stdin: {}", e), 1);
            process::exit(1);
        }
    }
}

pub fn emit_success<T: Serialize>(result: Option<T>) {
    let response = SddiaResponse {
        success: true,
        exit_code: 0,
        feedback: None,
        result,
        error: None,
    };
    emit(&response);
}

pub fn emit_error(msg: &str, exit_code: i32) {
    // `error` = contrato sddia-io; `feedback` = alias legible para route-domain / DLT.
    let response: SddiaResponse<Value> = SddiaResponse {
        success: false,
        exit_code,
        feedback: Some(msg.to_string()),
        result: None,
        error: Some(msg.to_string()),
    };
    emit(&response);
}

fn emit<T: Serialize>(response: &SddiaResponse<T>) {
    match serde_json::to_string(response) {
        Ok(json) => {
            println!("{}", json);
            process::exit(response.exit_code);
        }
        Err(_) => process::exit(1),
    }
}
