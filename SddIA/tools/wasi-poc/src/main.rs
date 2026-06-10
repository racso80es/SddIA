use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, Read};
use std::process;
use std::time::Instant;

const SCHEMA_VERSION: &str = "2.0";
const ENTITY_KIND: &str = "tool";
const ENTITY_ID: &str = "wasi-poc";

#[derive(Debug, Deserialize)]
struct CapsuleRequest {
    meta: RequestMeta,
    request: Value,
}

#[derive(Debug, Deserialize)]
struct RequestMeta {
    #[serde(rename = "schemaVersion", alias = "schema_version")]
    schema_version: String,
    #[serde(rename = "entityKind", alias = "entity_kind")]
    entity_kind: String,
    #[serde(rename = "entityId", alias = "entity_id")]
    entity_id: String,
}

#[derive(Debug, Serialize)]
struct CapsuleResponse {
    meta: ResponseMeta,
    success: bool,
    #[serde(rename = "exitCode")]
    exit_code: u8,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    feedback: Option<String>,
    result: Value,
    #[serde(skip_serializing_if = "Option::is_none", rename = "durationMs")]
    duration_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
struct ResponseMeta {
    #[serde(rename = "schemaVersion")]
    schema_version: String,
    #[serde(rename = "entityKind")]
    entity_kind: String,
    #[serde(rename = "entityId")]
    entity_id: String,
}

fn emit_response(resp: CapsuleResponse) -> ! {
    let line = serde_json::to_string(&resp).unwrap_or_else(|_| "{}".to_string());
    println!("{line}");
    process::exit(resp.exit_code as i32);
}

fn reject(message: &str) -> ! {
    emit_response(CapsuleResponse {
        meta: ResponseMeta {
            schema_version: SCHEMA_VERSION.to_string(),
            entity_kind: ENTITY_KIND.to_string(),
            entity_id: ENTITY_ID.to_string(),
        },
        success: false,
        exit_code: 1,
        message: message.to_string(),
        feedback: None,
        result: json!({}),
        duration_ms: None,
    });
}

fn read_stdin_payload() -> String {
    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).is_err() {
        reject("stdin unreadable");
    }
    buffer
}

fn parse_request(buffer: &str) -> CapsuleRequest {
    let trimmed = buffer.trim();
    if trimmed.is_empty() {
        reject("empty stdin payload");
    }
    match serde_json::from_str::<CapsuleRequest>(trimmed) {
        Ok(req) => req,
        Err(e) => reject(&format!("invalid JSON envelope: {e}")),
    }
}

fn validate_meta(meta: &RequestMeta) -> Result<(), String> {
    if meta.schema_version != SCHEMA_VERSION {
        return Err(format!(
            "unsupported schemaVersion: {}",
            meta.schema_version
        ));
    }
    if meta.entity_kind != ENTITY_KIND {
        return Err(format!("unsupported entityKind: {}", meta.entity_kind));
    }
    if meta.entity_id != ENTITY_ID {
        return Err(format!("unsupported entityId: {}", meta.entity_id));
    }
    Ok(())
}

fn main() {
    let started = Instant::now();
    let buffer = read_stdin_payload();
    let req = parse_request(&buffer);

    if let Err(detail) = validate_meta(&req.meta) {
        reject(&detail);
    }

    let echo = req.request.clone();
    let duration_ms = started.elapsed().as_millis() as u64;

    emit_response(CapsuleResponse {
        meta: ResponseMeta {
            schema_version: SCHEMA_VERSION.to_string(),
            entity_kind: ENTITY_KIND.to_string(),
            entity_id: ENTITY_ID.to_string(),
        },
        success: true,
        exit_code: 0,
        message: "WASI capsule executed in sandbox".to_string(),
        feedback: Some("I/O limited to stdin/stdout per capsule-json-io".to_string()),
        result: json!({
            "echo": echo,
            "wasi_status": "S+ Grade_Sealed",
            "sandbox": "wasm32-wasip1"
        }),
        duration_ms: Some(duration_ms),
    });
}
