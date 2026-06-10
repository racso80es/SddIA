use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_repo_root() -> Result<PathBuf, String> {
    let current_exe = env::current_exe().map_err(|e| format!("Failed to get current exe: {}", e))?;
    let mut current_dir = current_exe.parent().unwrap_or(Path::new(""));
    loop {
        if current_dir.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(current_dir.to_path_buf());
        }
        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            return Err("No se encontró raíz del workspace".to_string());
        }
    }
}

fn invoke_tool(repo: &Path, tool_key: &str, payload: &Value) -> Result<Value, String> {
    let rel = match tool_key {
        "read-event-subscriptions" => "SddIA/tools/read-event-subscriptions/read-event-subscriptions.wasm",
        "manage-event-receipt" => "SddIA/tools/manage-event-receipt/manage-event-receipt.wasm",
        "transit-event-payload" => "SddIA/tools/transit-event-payload/transit-event-payload.wasm",
        "markdown-table-editor" => "SddIA/tools/markdown-table-editor/markdown-table-editor.wasm",
        _ => return Err(format!("tool no mapeada: {}", tool_key)),
    };
    let script = repo.join(rel);

    let payload_str = serde_json::to_string(payload).unwrap_or_default();

    let mut child = Command::new("wasmtime")
        .arg("run")
        .arg("--dir=.")
        .arg(script)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .current_dir(repo)
        .spawn()
        .map_err(|e| format!("Failed to spawn tool: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(payload_str.as_bytes());
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait on tool: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let body: Value = serde_json::from_str(&stdout).unwrap_or(json!({}));
    if body.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
        Ok(body.get("result").cloned().unwrap_or(json!({})))
    } else {
        let err_msg = body
            .get("error")
            .and_then(|v| v.as_str())
            .or_else(|| body.get("message").and_then(|v| v.as_str()))
            .unwrap_or("tool failed");
        Err(err_msg.to_string())
    }
}

fn sync_entity_index_payload(payload: &Value) -> Result<Value, String> {
    let entity_class = payload.get("entity_class").and_then(|v| v.as_str()).unwrap_or("");
    let entity_name = payload.get("entity_name").and_then(|v| v.as_str()).unwrap_or("");
    let lifecycle = payload.get("lifecycle_operation").and_then(|v| v.as_str()).unwrap_or("");

    let rel_path = match entity_class {
        "process" => "SddIA/process/index.md",
        "agent" => "SddIA/agents/index.md",
        "skill" => "SddIA/skills/index.md",
        "tool" => "SddIA/tools/index.md",
        "action" => "SddIA/actions/index.md",
        "codex" => "SddIA/library/codexes/index.md",
        _ => return Ok(json!({"noop": true, "entity_class": entity_class})),
    };

    if lifecycle == "delete" {
        Ok(json!({
            "file_path": rel_path,
            "operation": "delete_row",
            "key_column": "name",
            "row_data": {"name": entity_name},
            "match_token": entity_name,
        }))
    } else if lifecycle == "create" || lifecycle == "update" {
        Ok(json!({
            "file_path": rel_path,
            "operation": "row_exists",
            "key_column": "name",
            "row_data": {"name": entity_name},
            "match_token": entity_name,
        }))
    } else {
        Err(format!("lifecycle_operation no soportada: {}", lifecycle))
    }
}

fn main() {
    let doc = read_stdin_json();

    let op = match doc.get("operation").and_then(|v| v.as_str()) {
        Some(op) => op,
        None => {
            emit_error("operation must be a string", 1);
            return;
        }
    };

    let allowed_ops = vec!["resolve_subscribers", "transit_payload", "apply_receipt", "sync_entity_index"];
    if !allowed_ops.contains(&op) {
        emit_error(&format!("operation debe ser uno de {:?}", allowed_ops), 1);
        return;
    }

    let payload = match doc.get("operation_payload") {
        Some(p) if p.is_object() => p,
        _ => {
            emit_error("operation_payload debe ser objeto", 1);
            return;
        }
    };

    let repo = match get_repo_root() {
        Ok(r) => r,
        Err(e) => {
            emit_error(&e, 1);
            return;
        }
    };

    let result = match op {
        "resolve_subscribers" => invoke_tool(&repo, "read-event-subscriptions", payload),
        "transit_payload" => invoke_tool(&repo, "transit-event-payload", payload),
        "apply_receipt" => invoke_tool(&repo, "manage-event-receipt", payload),
        "sync_entity_index" => {
            if payload.get("entity_class").and_then(|v| v.as_str()) == Some("norm") {
                Ok(json!({"skipped": true, "message": "norm no indexada"}))
            } else {
                match sync_entity_index_payload(payload) {
                    Ok(tool_payload) => {
                        if tool_payload.get("noop").and_then(|v| v.as_bool()).unwrap_or(false) {
                            Ok(tool_payload)
                        } else {
                            invoke_tool(&repo, "markdown-table-editor", &tool_payload)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        }
        _ => Err(format!("operation no implementada: {}", op)),
    };

    match result {
        Ok(res) => emit_success(Some(res)),
        Err(err) => emit_error(&err, 1),
    }
}
