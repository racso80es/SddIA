//! Handler nativo `sync-entity-index` — auditoría/purga de `index.md` (D-P6T.1).

use super::capsules::invoke_tool;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;

fn index_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("process", "SddIA/process/index.md"),
        ("agent", "SddIA/agents/index.md"),
        ("skill", "SddIA/skills/index.md"),
        ("tool", "SddIA/tools/index.md"),
        ("action", "SddIA/actions/index.md"),
        ("codex", "SddIA/library/codexes/index.md"),
        ("suite", "SddIA/suites/index.md"),
    ])
}

fn resolve_origin_topology(inputs: &Value) -> String {
    inputs
        .get("origin_topology")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| *s == "core" || *s == "local")
        .unwrap_or("core")
        .to_string()
}

fn build_tool_payload(rel_path: &str, entity_name: &str, lifecycle: &str) -> Result<Value, String> {
    let base = json!({
        "file_path": rel_path,
        "key_column": "name",
        "row_data": {"name": entity_name},
        "match_token": entity_name,
    });
    match lifecycle {
        "delete" => {
            let mut p = base;
            p["operation"] = json!("delete_row");
            Ok(p)
        }
        "create" | "update" => {
            let mut p = base;
            p["operation"] = json!("row_exists");
            Ok(p)
        }
        other => Err(format!("lifecycle_operation no soportada: {other}")),
    }
}

fn tool_result_fields(body: &Value) -> Value {
    if let Some(inner) = body.get("result") {
        if inner.get("exists").is_some()
            || inner.get("rows_removed").is_some()
            || inner.get("modified").is_some()
        {
            return inner.clone();
        }
    }
    body.clone()
}

/// Ejecuta `sync-entity-index` (paridad `execute-action.py::_run_sync_entity_index`).
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let entity_class = inputs
        .get("entity_class")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let entity_name = inputs
        .get("entity_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let lifecycle_operation = inputs
        .get("lifecycle_operation")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    if entity_class == "norm" {
        return Ok(json!({
            "success": true,
            "target_index_path": Value::Null,
            "message": "Indexación ignorada para norm.",
        }));
    }

    if resolve_origin_topology(inputs) == "local" {
        return Ok(json!({
            "success": true,
            "target_index_path": Value::Null,
            "message": "Indexación canónica omitida (origin_topology=local).",
        }));
    }

    let map = index_map();
    let Some(rel_path) = map.get(entity_class.as_str()).copied() else {
        return Ok(json!({
            "success": true,
            "target_index_path": Value::Null,
            "message": format!("entity_class desconocida ({entity_class}): no-op."),
        }));
    };

    if entity_name.is_empty() {
        return Err("entity_name requerido".into());
    }

    let target_file = repo.join(rel_path);
    if !target_file.is_file() {
        return Ok(json!({
            "success": false,
            "target_index_path": rel_path,
            "message": format!("Índice inexistente: {rel_path}"),
        }));
    }

    let tool_payload = build_tool_payload(rel_path, &entity_name, &lifecycle_operation)?;
    let tool_body = invoke_tool(repo, "markdown-table-editor", &tool_payload)?;
    let fields = tool_result_fields(&tool_body);

    if lifecycle_operation == "delete" {
        let rows_removed = fields
            .get("rows_removed")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let modified = fields.get("modified").and_then(|v| v.as_bool()).unwrap_or(false);
        let message = if rows_removed > 0 || modified {
            format!("Fila purgada para {entity_name}.")
        } else {
            format!("Sin fila para purgar (idempotente): {entity_name}.")
        };
        return Ok(json!({
            "success": true,
            "target_index_path": rel_path,
            "message": message,
        }));
    }

    if lifecycle_operation == "create" || lifecycle_operation == "update" {
        let exists = fields.get("exists").and_then(|v| v.as_bool()).unwrap_or(false);
        return Ok(json!({
            "success": exists,
            "target_index_path": rel_path,
            "message": if exists {
                format!("Auditoría OK: fila presente para {entity_name}.")
            } else {
                format!("ALERTA: Fila no encontrada para {entity_name}.")
            },
        }));
    }

    Err(format!(
        "lifecycle_operation no soportada: {lifecycle_operation}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;

    #[test]
    fn sync_entity_index_norm_noop() {
        let repo = find_repo_root().unwrap();
        let out = run(
            &repo,
            &json!({
                "entity_class": "norm",
                "entity_name": "execution-contexts",
                "lifecycle_operation": "create"
            }),
        )
        .unwrap();
        assert_eq!(out.get("success"), Some(&json!(true)));
    }

    #[test]
    fn sync_entity_index_audit_existing_action() {
        let repo = find_repo_root().unwrap();
        let out = run(
            &repo,
            &json!({
                "entity_class": "action",
                "entity_name": "policy-validator",
                "lifecycle_operation": "update",
                "entity_uuid": "00000000-0000-4000-8000-000000000001"
            }),
        )
        .unwrap();
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert!(out
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .contains("Auditoría OK"));
    }

    #[test]
    fn sync_entity_index_delete_idempotent() {
        let repo = find_repo_root().unwrap();
        let out = run(
            &repo,
            &json!({
                "entity_class": "action",
                "entity_name": "nonexistent-sync-entity-index-xyz",
                "lifecycle_operation": "delete"
            }),
        )
        .unwrap();
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert!(out
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .contains("idempotente"));
    }

    #[test]
    fn sync_entity_index_local_topology_skip() {
        let repo = find_repo_root().unwrap();
        let out = run(
            &repo,
            &json!({
                "entity_class": "action",
                "entity_name": "policy-validator",
                "lifecycle_operation": "update",
                "origin_topology": "local"
            }),
        )
        .unwrap();
        assert!(out
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .contains("origin_topology=local"));
    }
}
