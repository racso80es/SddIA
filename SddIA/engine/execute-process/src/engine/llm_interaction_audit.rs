//! Auditoría de interacciones LLM por `tracker_ref` (AUT-7).

use super::done_gate::fetch_issue;
use super::item_lifecycle_frontmatter::normalize_item_markdown;
use super::project_binding;
use super::workspace::load_paths_config;
use chrono::Utc;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub const FM_LLMA_AUDIT: &str = "llm_audit";
const ACTIVE: &str = "active";
const INACTIVE: &str = "inactive";

const KARMA2_ENTITY_TYPES: &[&str] = &["skill", "tool", "action", "process"];

pub struct InteractionRow {
    pub entity_type: String,
    pub entity_id: String,
    pub invoked_by: String,
    pub timestamp: String,
}

pub fn normalize_entity_type(raw: &str) -> Result<String, String> {
    let t = raw.trim().to_lowercase();
    if KARMA2_ENTITY_TYPES.contains(&t.as_str()) {
        Ok(t)
    } else {
        Err(format!(
            "entity_type inválido: '{raw}' (esperado skill|tool|action|process)"
        ))
    }
}

pub struct FrictionRow {
    pub comando: String,
    pub success: bool,
    pub error_code: String,
    pub handler: String,
    pub mensaje: String,
}

fn audits_dir(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_paths_config(repo)?;
    let rel = cfg
        .get("paths")
        .and_then(|p| p.get("auditsPath"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_end_matches('/').replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "paths.auditsPath no resuelto".to_string())?;
    Ok(repo.join(rel))
}

fn audit_path(repo: &Path, tracker_ref: &str) -> Result<PathBuf, String> {
    let safe = tracker_ref
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>();
    if safe.is_empty() {
        return Err("tracker_ref inválido".into());
    }
    Ok(audits_dir(repo)?.join(format!("{safe}.md")))
}

fn fm_from_description(description: &str) -> Result<YamlValue, String> {
    let normalized = normalize_item_markdown(description)?;
    let trimmed = normalized.trim_start();
    if !trimmed.starts_with("---") {
        return Err("sin frontmatter".into());
    }
    let rest = trimmed.strip_prefix("---").unwrap_or(trimmed);
    let end = rest
        .find("\n---")
        .ok_or_else(|| "frontmatter sin cierre".to_string())?;
    let fm_text = rest[..end].trim();
    serde_yaml::from_str(fm_text).map_err(|e| e.to_string())
}

fn llm_audit_from_fm(fm: &YamlValue) -> &'static str {
    match fm.get(FM_LLMA_AUDIT).and_then(|v| v.as_str()).map(str::trim) {
        Some(ACTIVE) => ACTIVE,
        Some(INACTIVE) => INACTIVE,
        _ => INACTIVE,
    }
}

pub fn llm_audit_mode_from_description(description: &str) -> Result<&'static str, String> {
    let fm = fm_from_description(description)?;
    Ok(llm_audit_from_fm(&fm))
}

pub fn resolve_tracker_ref(inputs: &Value) -> Option<String> {
    for key in ["tracker_ref", "pbi_ref", "issue_ref"] {
        if let Some(s) = inputs.get(key).and_then(|v| v.as_str()) {
            let t = s.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    None
}

pub fn fetch_llm_audit_active(repo: &Path, tracker_ref: &str) -> Result<bool, String> {
    let project_id = project_binding::tracker_config_for_inputs(repo, &json!({}))?
        .and_then(|tc| tc.project_id);
    let issue = fetch_issue(repo, tracker_ref, "llm-interaction-audit", project_id.as_deref())?;
    let desc = issue
        .get("description")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "fetch_issue sin description".to_string())?;
    Ok(llm_audit_mode_from_description(desc)? == ACTIVE)
}

fn row_key(row: &InteractionRow) -> String {
    format!(
        "{}|{}|{}|{}",
        row.entity_type.trim(),
        row.entity_id.trim(),
        row.invoked_by.trim(),
        row.timestamp.trim()
    )
}

fn parse_existing_rows(body: &str) -> std::collections::HashSet<String> {
    let mut keys = std::collections::HashSet::new();
    let karma2 = body.contains("entity_type") && body.contains("invoked_by");
    for line in body.lines() {
        let line = line.trim();
        if !line.starts_with('|') || line.contains("---") {
            continue;
        }
        if line.contains("Timestamp") || line.contains("entity_type") {
            continue;
        }
        let cols: Vec<&str> = line.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
        if karma2 && cols.len() >= 4 {
            keys.insert(format!(
                "{}|{}|{}|{}",
                cols[0].to_lowercase(),
                cols[1],
                cols[2],
                cols[3]
            ));
        } else if !karma2 && cols.len() >= 4 {
            let (ts, resumen) = if cols.len() >= 5 {
                (cols[1], cols[4])
            } else {
                (cols[0], cols[3])
            };
            if resumen != "Resumen" {
                keys.insert(format!("legacy|{ts}|{resumen}"));
            }
        }
    }
    keys
}

fn ensure_audit_shell(_repo: &Path, path: &Path, tracker_ref: &str) -> Result<(), String> {
    if path.is_file() {
        return Ok(());
    }
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let created = Utc::now().format("%Y-%m-%d").to_string();
    let doc_uuid = Uuid::new_v4();
    let content = format!(
        "---\nuuid: \"{doc_uuid}\"\nid: llm-interaction-audit-{tracker_ref}\n\
type: audit\nversion: \"1.0.0\"\ntracker_ref: {tracker_ref}\n\
process: llm-interaction-audit\ncreated: \"{created}\"\n---\n\n\
# Auditoría de interacciones LLM — {tracker_ref}\n\n\
| entity_type | entity_id | invoked_by | timestamp |\n\
| --- | --- | --- | --- |\n"
    );
    fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn append_interaction(
    repo: &Path,
    tracker_ref: &str,
    row: &InteractionRow,
    require_active: bool,
) -> Result<Value, String> {
    if require_active && !fetch_llm_audit_active(repo, tracker_ref)? {
        return Ok(json!({"skipped": true, "reason": "llm_audit_inactive"}));
    }
    let path = audit_path(repo, tracker_ref)?;
    ensure_audit_shell(repo, &path, tracker_ref)?;
    let mut text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    normalize_entity_type(&row.entity_type)?;
    let keys = parse_existing_rows(&text);
    let key = row_key(row);
    if keys.contains(&key) {
        return Ok(json!({
            "skipped": true,
            "reason": "duplicate_row",
            "path": path.to_string_lossy()
        }));
    }
    let line = format!(
        "| {} | {} | {} | {} |\n",
        row.entity_type, row.entity_id, row.invoked_by, row.timestamp
    );
    let sep = "| --- | --- | --- | --- |";
    if let Some(idx) = text.find(sep) {
        let insert_at = text[idx..]
            .find('\n')
            .map(|o| idx + o + 1)
            .unwrap_or(text.len());
        text.insert_str(insert_at, &line);
    } else {
        text.push('\n');
        text.push_str(&line);
    }
    fs::write(&path, text).map_err(|e| e.to_string())?;
    Ok(json!({"written": true, "path": path.to_string_lossy()}))
}

pub fn append_friction(
    repo: &Path,
    tracker_ref: &str,
    friction: &FrictionRow,
    require_active: bool,
) -> Result<Value, String> {
    if require_active && !fetch_llm_audit_active(repo, tracker_ref)? {
        return Ok(json!({"skipped": true, "reason": "llm_audit_inactive"}));
    }
    let path = audit_path(repo, tracker_ref)?;
    ensure_audit_shell(repo, &path, tracker_ref)?;
    let mut text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    if !text.contains("## Fricción") {
        text.push_str("\n## Fricción — `./sddia-run.sh`\n\n");
        text.push_str("| Campo | Valor |\n| --- | --- |\n");
    }
    let block = format!(
        "\n| Comando | `{}` |\n| `success` | `{}` |\n| `error_code` | `{}` |\n| handler | `{}` |\n| mensaje | {} |\n",
        friction.comando.replace('`', "'"),
        friction.success,
        friction.error_code,
        friction.handler,
        friction.mensaje.replace('|', "/")
    );
    text.push_str(&block);
    fs::write(&path, text).map_err(|e| e.to_string())?;
    Ok(json!({"written": true, "path": path.to_string_lossy()}))
}

pub fn maybe_record_friction_on_failure(
    repo: &Path,
    process_name: &str,
    process_inputs: &Value,
    state: &Value,
    success: bool,
    exit_code: i32,
) {
    if success || exit_code == 0 {
        return;
    }
    if !matches!(process_name, "feature" | "bug-fix" | "refactorization") {
        return;
    }
    let tracker_ref = resolve_tracker_ref(process_inputs).or_else(|| {
        process_inputs
            .get("persist_ref")
            .and_then(|v| v.as_str())
            .and_then(|persist| {
                super::tracker_pbi_meta::resolve_pbi_path_from_persist(repo, persist)
            })
            .and_then(|pbi| super::tracker_pbi_meta::tracker_ref_from_pbi(repo, &pbi))
    });
    let tracker_ref = match tracker_ref {
        Some(t) => t,
        None => return,
    };
    let failed = state.get("phase_reports").and_then(|v| v.as_array()).map(|arr| {
        super::phase_terminal::aggregate_execution_terminal(arr, state)
    });
    let (handler, error_code, mensaje) = if let Some(v) = failed {
        if let Some(fp) = v.failed_phase.as_ref() {
            (
                fp.handler.clone().unwrap_or_else(|| "execute-process".into()),
                fp.code.clone().unwrap_or_else(|| "PROCESS_FAILED".into()),
                fp.error.clone().unwrap_or_default(),
            )
        } else {
            ("execute-process".into(), "PROCESS_FAILED".into(), String::new())
        }
    } else {
        ("execute-process".into(), "PROCESS_FAILED".into(), String::new())
    };
    let friction = FrictionRow {
        comando: format!("./sddia-run.sh --process {process_name}"),
        success: false,
        error_code,
        handler,
        mensaje,
    };
    let _ = append_friction(repo, &tracker_ref, &friction, true);
}

/// Registra el peaje de un ciclo lifecycle cuando `llm_audit: active`.
pub fn maybe_record_process_peaje(
    repo: &Path,
    process_name: &str,
    process_inputs: &Value,
    _success: bool,
    _exit_code: i32,
) {
    if !matches!(process_name, "feature" | "bug-fix" | "refactorization") {
        return;
    }
    let tracker_ref = match resolve_tracker_ref(process_inputs).or_else(|| {
        process_inputs
            .get("persist_ref")
            .and_then(|v| v.as_str())
            .and_then(|persist| {
                super::tracker_pbi_meta::resolve_pbi_path_from_persist(repo, persist)
            })
            .and_then(|pbi| super::tracker_pbi_meta::tracker_ref_from_pbi(repo, &pbi))
    }) {
        Some(t) => t,
        None => return,
    };
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let row = InteractionRow {
        entity_type: "process".into(),
        entity_id: process_name.to_string(),
        invoked_by: "execute-process".into(),
        timestamp,
    };
    let _ = append_interaction(repo, &tracker_ref, &row, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inactive_without_field() {
        let raw = "---\ntracker_ref: X\n---\n\n# b\n";
        assert_eq!(llm_audit_mode_from_description(raw).unwrap(), INACTIVE);
    }

    #[test]
    fn active_when_set() {
        let raw = "---\nllm_audit: active\ntracker_ref: X\n---\n\n# b\n";
        assert_eq!(llm_audit_mode_from_description(raw).unwrap(), ACTIVE);
    }

    #[test]
    fn dedup_row() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"auditsPath":"docs/audits"}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join("docs/audits")).unwrap();
        let row = InteractionRow {
            entity_type: "process".into(),
            entity_id: "feature".into(),
            invoked_by: "execute-process".into(),
            timestamp: "2026-10-26T12:00:00Z".into(),
        };
        let r1 = append_interaction(repo, "LAB-1", &row, false).unwrap();
        assert_eq!(r1.get("written"), Some(&json!(true)));
        let r2 = append_interaction(repo, "LAB-1", &row, false).unwrap();
        assert_eq!(r2.get("reason"), Some(&json!("duplicate_row")));
    }

    #[test]
    fn shell_has_uuid_v4_and_karma2_header() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"auditsPath":"docs/audits"}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join("docs/audits")).unwrap();
        let row = InteractionRow {
            entity_type: "tool".into(),
            entity_id: "git-manager".into(),
            invoked_by: "tekton".into(),
            timestamp: "2026-10-28T18:00:01Z".into(),
        };
        append_interaction(repo, "AUT-10", &row, false).unwrap();
        let text = fs::read_to_string(repo.join("docs/audits/AUT-10.md")).unwrap();
        assert!(text.contains("uuid:"));
        assert!(text.contains("entity_type"));
        assert!(text.contains("| tool | git-manager | tekton | 2026-10-28T18:00:01Z |"));
    }

    #[test]
    fn rejects_invalid_entity_type() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"auditsPath":"docs/audits"}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join("docs/audits")).unwrap();
        let row = InteractionRow {
            entity_type: "agent".into(),
            entity_id: "x".into(),
            invoked_by: "y".into(),
            timestamp: "2026-10-28T18:00:00Z".into(),
        };
        assert!(normalize_entity_type("process").is_ok());
        assert!(append_interaction(repo, "LAB-1", &row, false).is_err());
    }
}
