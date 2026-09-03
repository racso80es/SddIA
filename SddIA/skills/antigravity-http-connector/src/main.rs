use reqwest::blocking::Client;
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

    let client = Client::new();
    let response_result = client
        .post(&api_endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_payload)
        .send();

    match response_result {
        Ok(response) => {
            let status = response.status();
            match response.json::<Value>() {
                Ok(json_response) => {
                    if status.is_success() {
                        emit_success(Some(json_response));
                    } else {
                        emit_error(&format!("HTTP Error {}: {}", status, json_response), 1);
                    }
                }
                Err(e) => {
                    emit_error(&format!("Failed to parse response JSON: {}", e), 1);
                }
            }
        }
        Err(e) => {
            emit_error(&format!("HTTP Request failed: {}", e), 1);
        }
    }
}
