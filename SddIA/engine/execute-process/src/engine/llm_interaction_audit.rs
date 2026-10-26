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

pub const FM_LLMA_AUDIT: &str = "llm_audit";
const ACTIVE: &str = "active";
const INACTIVE: &str = "inactive";

pub struct InteractionRow {
    pub timestamp: String,
    pub rol: String,
    pub tipo: String,
    pub resumen: String,
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

fn row_key(timestamp: &str, resumen: &str) -> String {
    format!("{}|{}", timestamp.trim(), resumen.trim())
}

fn parse_existing_rows(body: &str) -> std::collections::HashSet<String> {
    let mut keys = std::collections::HashSet::new();
    for line in body.lines() {
        let line = line.trim();
        if !line.starts_with('|') || line.contains("---") || line.contains("Timestamp") {
            continue;
        }
        let cols: Vec<&str> = line.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
        if cols.len() >= 4 {
            let (ts, resumen) = if cols.len() >= 5 {
                (cols[1], cols[4])
            } else {
                (cols[0], cols[3])
            };
            if resumen != "Resumen" && !ts.contains("---") {
                keys.insert(row_key(ts, resumen));
            }
        }
    }
    keys
}

fn ensure_audit_shell(repo: &Path, path: &Path, tracker_ref: &str) -> Result<(), String> {
    if path.is_file() {
        return Ok(());
    }
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let created = Utc::now().format("%Y-%m-%d").to_string();
    let content = format!(
        "---\nid: llm-interaction-audit-{tracker_ref}\n\
type: audit\nversion: \"1.0.0\"\ntracker_ref: {tracker_ref}\n\
process: llm-interaction-audit\ncreated: \"{created}\"\n---\n\n\
# Auditoría de interacciones LLM — {tracker_ref}\n\n\
| Timestamp (UTC) | Rol | Tipo | Resumen |\n\
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
    let keys = parse_existing_rows(&text);
    let key = row_key(&row.timestamp, &row.resumen);
    if keys.contains(&key) {
        return Ok(json!({
            "skipped": true,
            "reason": "duplicate_row",
            "path": path.to_string_lossy()
        }));
    }
    let line = format!(
        "| {} | {} | {} | {} |\n",
        row.timestamp, row.rol, row.tipo, row.resumen
    );
    if let Some(idx) = text.find("| --- | --- | --- | --- |") {
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
            timestamp: "2026-10-26T12:00:00Z".into(),
            rol: "Tekton".into(),
            tipo: "tool_call".into(),
            resumen: "test".into(),
        };
        let r1 = append_interaction(repo, "LAB-1", &row, false).unwrap();
        assert_eq!(r1.get("written"), Some(&json!(true)));
        let r2 = append_interaction(repo, "LAB-1", &row, false).unwrap();
        assert_eq!(r2.get("reason"), Some(&json!("duplicate_row")));
    }
}
