//! Handler nativo `materialize-kaizen-alert-doc` — cicatriz Kaizen DIA (D-P6T.1).

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

fn required_str(inputs: &Value, key: &str) -> Result<String, String> {
    match inputs.get(key).and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => Ok(s.trim().to_string()),
        _ => Err(format!("{key} es obligatorio (string)")),
    }
}

fn parse_implicated_files(inputs: &Value) -> Result<Vec<String>, String> {
    let Some(arr) = inputs.get("implicated_files").and_then(|v| v.as_array()) else {
        return Err("implicated_files es obligatorio (array no vacío)".into());
    };
    if arr.is_empty() {
        return Err("implicated_files es obligatorio (array no vacío)".into());
    }
    let files: Vec<String> = arr
        .iter()
        .map(|v| v.as_str().unwrap_or("").trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if files.is_empty() {
        return Err("implicated_files no contiene rutas válidas".into());
    }
    Ok(files)
}

/// Paridad `execute-action.py::_kaizen_alert_doc_hash`.
pub fn kaizen_alert_doc_hash(review_id: &str, implicated_files: &[String]) -> String {
    let mut sorted = implicated_files.to_vec();
    sorted.sort();
    let key = format!("{}{}", review_id.trim(), sorted.join(""));
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())[..8].to_string()
}

fn kaizen_todo_path(repo: &Path, hash8: &str) -> PathBuf {
    repo.join("docs/todos/pending")
        .join(format!("PENDING_AUDIT_DOC_{hash8}.md"))
}

fn optional_cell(inputs: &Value, key: &str) -> String {
    match inputs.get(key).and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => format!("`{}`", s.trim()),
        _ => "—".to_string(),
    }
}

fn impacts_doc_cell(inputs: &Value) -> String {
    match inputs.get("impacts_doc") {
        Some(v) if v.is_boolean() => format!("`{v}`"),
        _ => "—".to_string(),
    }
}

fn build_todo_body(
    todo_name: &str,
    review_id: &str,
    alert_justification: &str,
    alert_kind: &str,
    persist_cell: &str,
    branch_cell: &str,
    impacts_cell: &str,
    hits_md: &str,
) -> String {
    format!(
        r#"# {todo_name}

> Origen: `Kaizen_Alert_Required` / sensor DIA / evento EDA v2

**Alerta:** posible fuga de conocimiento documental.

| Campo | Valor |
|-------|-------|
| `review_id` | `{review_id}` |
| `alert_justification` | `{alert_justification}` |
| `alert_kind` | `{alert_kind}` |
| `persist_ref` | {persist_cell} |
| `pr_branch` | {branch_cell} |
| `impacts_doc` | {impacts_cell} |
| `implicated_files` | {hits_md} |

## Checklist DIA

- [ ] Revisar `spec.md` § Impacto en Documentación
- [ ] Actualizar README/manuales afectados o corregir `impacts_doc`
"#
    )
}

/// Ejecuta `materialize-kaizen-alert-doc` (paridad `execute-action.py::_run_materialize_kaizen_alert_doc`).
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let review_id = required_str(inputs, "review_id")?;
    let alert_justification = required_str(inputs, "alert_justification")?;
    let files = parse_implicated_files(inputs)?;

    let hash8 = kaizen_alert_doc_hash(&review_id, &files);
    let todo_name = format!("PENDING_AUDIT_DOC_{hash8}.md");
    let pending_dir = repo.join("docs/todos/pending");
    fs::create_dir_all(&pending_dir).map_err(|e| e.to_string())?;
    let target = kaizen_todo_path(repo, &hash8);
    let rel_path = target
        .strip_prefix(repo)
        .unwrap_or(&target)
        .to_string_lossy()
        .replace('\\', "/");

    if target.is_file() {
        return Ok(json!({
            "success": true,
            "target_path": rel_path,
            "message": "TODO Kaizen ya existente (idempotente)",
            "hash8": hash8,
        }));
    }

    let alert_kind = inputs
        .get("alert_kind")
        .and_then(|v| v.as_str())
        .unwrap_or("doc_parity");
    let persist_cell = optional_cell(inputs, "persist_ref");
    let branch_cell = optional_cell(inputs, "pr_branch");
    let impacts_cell = impacts_doc_cell(inputs);
    let hits_md = files
        .iter()
        .map(|h| format!("`{h}`"))
        .collect::<Vec<_>>()
        .join(", ");

    let body = build_todo_body(
        &todo_name,
        &review_id,
        &alert_justification,
        alert_kind,
        &persist_cell,
        &branch_cell,
        &impacts_cell,
        &hits_md,
    );
    fs::write(&target, body).map_err(|e| e.to_string())?;

    Ok(json!({
        "success": true,
        "target_path": rel_path,
        "message": "TODO Kaizen materializado",
        "hash8": hash8,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn kaizen_alert_doc_hash_deterministic_and_order_invariant() {
        let h1 = kaizen_alert_doc_hash(
            "rev-1",
            &["b.md".to_string(), "a.md".to_string()],
        );
        let h2 = kaizen_alert_doc_hash(
            "rev-1",
            &["a.md".to_string(), "b.md".to_string()],
        );
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 8);
        assert_ne!(
            kaizen_alert_doc_hash("rev-2", &["a.md".to_string()]),
            h1
        );
    }

    #[test]
    fn materialize_kaizen_alert_doc_creates_and_idempotent() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();

        let inputs = json!({
            "review_id": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
            "alert_justification": "DIA parity gap",
            "implicated_files": ["docs/features/foo/spec.md", "README.md"],
            "persist_ref": "docs/features/foo",
            "pr_branch": "feat/foo",
            "impacts_doc": false
        });

        let out = run(repo, &inputs).expect("first run");
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert_eq!(out.get("message"), Some(&json!("TODO Kaizen materializado")));
        let hash8 = out.get("hash8").and_then(|v| v.as_str()).expect("hash8");
        assert_eq!(hash8.len(), 8);

        let path = out
            .get("target_path")
            .and_then(|v| v.as_str())
            .expect("target_path");
        let full = repo.join(path);
        assert!(full.is_file());
        let content = fs::read_to_string(&full).unwrap();
        assert!(content.contains("PENDING_AUDIT_DOC_"));
        assert!(content.contains("DIA parity gap"));
        assert!(content.contains("docs/features/foo/spec.md"));
        assert!(content.contains("Checklist DIA"));

        let out2 = run(repo, &inputs).expect("second run");
        assert_eq!(
            out2.get("message"),
            Some(&json!("TODO Kaizen ya existente (idempotente)"))
        );
        assert_eq!(out2.get("hash8"), Some(&json!(hash8)));
    }

    #[test]
    fn materialize_kaizen_alert_doc_rejects_empty_files() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = run(
            tmp.path(),
            &json!({
                "review_id": "r",
                "alert_justification": "j",
                "implicated_files": ["", "  "]
            }),
        )
        .unwrap_err();
        assert!(err.contains("implicated_files no contiene rutas válidas"));
    }
}
