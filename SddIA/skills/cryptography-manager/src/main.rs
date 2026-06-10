use sddia_io::{read_stdin_json, emit_success, emit_error};
use serde_json::Value;
use sha2::{Sha256, Digest};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn safe_read_file(rel: &str) -> Result<Vec<u8>, String> {
    let p = Path::new(rel);
    if p.is_absolute() {
        return Err("FILE_PATH must be relative to workspace".to_string());
    }

    let cwd = env::current_dir().map_err(|_| "failed to get current dir".to_string())?;
    let resolved = cwd.join(p);

    let canonical_cwd = cwd.canonicalize().unwrap_or(cwd.clone());
    let canonical_resolved = resolved.canonicalize().map_err(|_| "FILE_PATH does not exist or is not a file".to_string())?;

    if !canonical_resolved.starts_with(&canonical_cwd) {
        return Err("path escapes workspace or is invalid".to_string());
    }
    if !canonical_resolved.is_file() {
        return Err("FILE_PATH does not exist or is not a file".to_string());
    }

    fs::read(canonical_resolved).map_err(|e| format!("failed to read file: {}", e))
}

fn main() {
    let data = read_stdin_json();

    let op = match data.get("operation").and_then(|v| v.as_str()) {
        Some(o) => o,
        None => {
            emit_error("operation must be a string", 1);
            return;
        }
    };

    let tt = data.get("target_type").and_then(|v| v.as_str());
    let payload = data.get("target_payload");

    let valid_ops = vec!["GENERATE_SHA256", "VALIDATE_HASH", "GENERATE_UUID"];
    let valid_types = vec!["STRING", "FILE_PATH"];

    if !valid_ops.contains(&op) {
        emit_error(&format!("operation must be one of {:?}", valid_ops), 1);
        return;
    }

    if op != "GENERATE_UUID" {
        if tt.is_none() || !valid_types.contains(&tt.unwrap()) {
            emit_error(&format!("target_type must be one of {:?}", valid_types), 1);
            return;
        }
    }

    if op == "GENERATE_UUID" {
        emit_success(Some(uuid::Uuid::new_v4().to_string()));
        return;
    }

    let tt = tt.unwrap();

    if op == "GENERATE_SHA256" {
        if tt == "STRING" {
            let s = match payload.and_then(|v| v.as_str()) {
                Some(s) => s,
                None => {
                    emit_error("target_payload must be a string for GENERATE_SHA256+STRING", 1);
                    return;
                }
            };
            emit_success(Some(sha256_bytes(s.as_bytes())));
            return;
        } else if tt == "FILE_PATH" {
            let path_str = match payload.and_then(|v| v.as_str()) {
                Some(s) => s,
                None => {
                    emit_error("target_payload must be a string path for GENERATE_SHA256+FILE_PATH", 1);
                    return;
                }
            };
            match safe_read_file(path_str) {
                Ok(bytes) => emit_success(Some(sha256_bytes(&bytes))),
                Err(e) => emit_error(&e, 1),
            }
            return;
        }
    }

    if op == "VALIDATE_HASH" {
        let payload_obj = match payload.and_then(|v| v.as_object()) {
            Some(obj) => obj,
            None => {
                emit_error("target_payload must be a JSON object for VALIDATE_HASH", 1);
                return;
            }
        };

        let expected_str = match payload_obj.get("expected_sha256").and_then(|v| v.as_str()) {
            Some(s) if s.len() == 64 => s,
            _ => {
                emit_error("expected_sha256 must be a 64-char hex string", 1);
                return;
            }
        };

        if hex::decode(expected_str).is_err() {
            emit_error("expected_sha256 is not valid hex", 1);
            return;
        }
        let expected = expected_str.to_lowercase();

        let actual = if tt == "STRING" {
            let subj = match payload_obj.get("subject").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => {
                    emit_error("subject must be a string for STRING validation", 1);
                    return;
                }
            };
            sha256_bytes(subj.as_bytes())
        } else {
            let path_str = match payload_obj.get("path").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => {
                    emit_error("path must be a string for FILE_PATH validation", 1);
                    return;
                }
            };
            match safe_read_file(path_str) {
                Ok(bytes) => sha256_bytes(&bytes),
                Err(e) => {
                    emit_error(&e, 1);
                    return;
                }
            }
        };

        emit_success(Some(actual == expected));
        return;
    }

    emit_error("unsupported operation", 1);
}
