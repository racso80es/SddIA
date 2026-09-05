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

/// Huella estable sin `review_id` — anti-spam ola event-bus-audit.
pub fn kaizen_alert_content_fingerprint(alert_kind: &str, implicated_files: &[String]) -> String {
    let mut sorted = implicated_files.to_vec();
    sorted.sort();
    let key = format!("{}|{}", alert_kind.trim(), sorted.join("\n"));
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())[..12].to_string()
}

fn kaizen_todo_path(repo: &Path, hash8: &str) -> PathBuf {
    repo.join("docs/todos/pending")
        .join(format!("PENDING_AUDIT_DOC_{hash8}.md"))
}

fn parse_table_cell(raw: &str, field: &str) -> Option<String> {
    let needle = format!("| `{field}` |");
    for line in raw.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix(&needle) {
            let cell = rest.trim().trim_end_matches('|').trim();
            return Some(cell.to_string());
        }
    }
    None
}

fn parse_implicated_files_from_body(raw: &str) -> Vec<String> {
    let Some(cell) = parse_table_cell(raw, "implicated_files") else {
        return Vec::new();
    };
    cell.split(',')
        .map(|s| {
            s.trim()
                .trim_matches('`')
                .trim()
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect()
}

/// PENDING_AUDIT_DOC abierto: casilla DIA o de bus sin marcar.
fn is_open_kaizen_checklist(raw: &str) -> bool {
    raw.contains("- [ ] Revisar `spec.md`")
        || raw.contains("- [ ] Contrastar métricas de la alerta con el censo del bus")
}

/// PENDING_AUDIT_DOC abierto con misma huella (alert_kind + files).
fn find_open_kaizen_audit_doc(
    repo: &Path,
    alert_kind: &str,
    implicated_files: &[String],
) -> Option<PathBuf> {
    let pending = repo.join("docs/todos/pending");
    if !pending.is_dir() {
        return None;
    }
    let want = kaizen_alert_content_fingerprint(alert_kind, implicated_files);
    let mut matches: Vec<PathBuf> = fs::read_dir(&pending)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("PENDING_AUDIT_DOC_") && n.ends_with(".md"))
                .unwrap_or(false)
        })
        .collect();
    matches.sort();
    for path in matches {
        let Ok(raw) = fs::read_to_string(&path) else {
            continue;
        };
        if !is_open_kaizen_checklist(&raw) {
            continue;
        }
        let kind = parse_table_cell(&raw, "alert_kind")
            .unwrap_or_default()
            .trim_matches('`')
            .to_string();
        let files = parse_implicated_files_from_body(&raw);
        if kind == alert_kind
            && kaizen_alert_content_fingerprint(&kind, &files) == want
        {
            return Some(path);
        }
    }
    None
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
    let (origen, alerta, checklist) = if alert_kind == "doc_parity" {
        (
            "> Origen: `Kaizen_Alert_Required` / sensor DIA / evento EDA v2",
            "**Alerta:** posible fuga de conocimiento documental.",
            "## Checklist DIA\n\n- [ ] Revisar `spec.md` § Impacto en Documentación\n- [ ] Actualizar README/manuales afectados o corregir `impacts_doc`\n",
        )
    } else {
        (
            "> Origen: `Kaizen_Alert_Required` / event-bus-audit / evento EDA v2",
            "**Alerta:** auditoría de infraestructura de bus EDA (no DIA).",
            "## Checklist bus\n\n- [ ] Contrastar métricas de la alerta con el censo del bus (no purgar dead-letter)\n- [ ] Confirmar que `needs_kaizen` no es acumulación histórica\n",
        )
    };
    format!(
        r#"# {todo_name}

{origen}

{alerta}

| Campo | Valor |
|-------|-------|
| `review_id` | `{review_id}` |
| `alert_justification` | `{alert_justification}` |
| `alert_kind` | `{alert_kind}` |
| `persist_ref` | {persist_cell} |
| `pr_branch` | {branch_cell} |
| `impacts_doc` | {impacts_cell} |
| `implicated_files` | {hits_md} |

{checklist}"#
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

    if let Some(existing) = find_open_kaizen_audit_doc(repo, alert_kind, &files) {
        let existing_rel = existing
            .strip_prefix(repo)
            .unwrap_or(&existing)
            .to_string_lossy()
            .replace('\\', "/");
        return Ok(json!({
            "success": true,
            "target_path": existing_rel,
            "message": "TODO Kaizen abierto misma huella (idempotente por alert_kind+files)",
            "hash8": hash8,
            "content_fingerprint": kaizen_alert_content_fingerprint(alert_kind, &files),
        }));
    }

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

    #[test]
    fn materialize_dedupes_open_doc_same_files_different_review() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();

        let files = json!([
            ".events/pending/aaa.json",
            ".events/pending/bbb.json"
        ]);
        let first = json!({
            "review_id": "11111111-1111-1111-1111-111111111111",
            "alert_justification": "Auditoría event-bus-audit: 86 dead-letter",
            "alert_kind": "event-bus-audit",
            "implicated_files": files,
        });
        let second = json!({
            "review_id": "22222222-2222-2222-2222-222222222222",
            "alert_justification": "Auditoría event-bus-audit: 86 dead-letter (otra corrida)",
            "alert_kind": "event-bus-audit",
            "implicated_files": [
                ".events/pending/bbb.json",
                ".events/pending/aaa.json"
            ],
        });

        let out1 = run(repo, &first).expect("first");
        assert_eq!(out1.get("message"), Some(&json!("TODO Kaizen materializado")));
        let path1 = out1.get("target_path").and_then(|v| v.as_str()).unwrap().to_string();
        let content = fs::read_to_string(repo.join(&path1)).unwrap();
        assert!(!content.contains("fuga de conocimiento documental"), "{content}");
        assert!(!content.contains("sensor DIA"), "{content}");
        assert!(content.contains("Checklist bus"), "{content}");
        assert!(
            content.contains("- [ ] Contrastar métricas de la alerta con el censo del bus"),
            "{content}"
        );

        let out2 = run(repo, &second).expect("second");
        assert_eq!(
            out2.get("message"),
            Some(&json!("TODO Kaizen abierto misma huella (idempotente por alert_kind+files)"))
        );
        assert_eq!(
            out2.get("target_path").and_then(|v| v.as_str()),
            Some(path1.as_str())
        );

        let count = fs::read_dir(repo.join("docs/todos/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("md"))
            .count();
        assert_eq!(count, 1);
    }
}
