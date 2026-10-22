//! Registro saliente anti-eco (HU-B 01): `.SddIA/state/tracker-outbound/{issue_ref}.jsonl`.

use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const CAPABILITY_ID: &str = "tracker:lifecycle";
const RECORDED_OPS: &[&str] = &[
    "create_issue",
    "update_issue_state",
    "create_comment",
    "update_issue_description",
];

/// Nombre de artefacto de la fila `tracker:lifecycle` (`tool:nombre` → `nombre`).
fn resolved_tracker_tool_name(repo: &Path) -> Option<String> {
    let cfg = crate::core::paths::load_paths_config(repo).ok()?;
    let rel = cfg
        .pointer("/capability_di/bindings")
        .and_then(|v| v.as_str())?;
    let path = repo.join(rel.trim().trim_start_matches("./"));
    let fm = crate::core::parser::parse_frontmatter(&path).ok()?;
    let seq = fm.get("bindings")?.as_sequence()?;
    for item in seq {
        let id = item.get("capability_id").and_then(|v| v.as_str()).unwrap_or("");
        if id != CAPABILITY_ID {
            continue;
        }
        let provider = item.get("provider").and_then(|v| v.as_str()).unwrap_or("");
        let name = provider.split_once(':').map(|(_, n)| n).unwrap_or(provider);
        let name = name.trim();
        if name.is_empty() {
            return None;
        }
        return Some(name.to_string());
    }
    None
}

fn request_inner(doc: &Value) -> Option<&Value> {
    doc.get("request").or(Some(doc))
}

fn outbound_dir(repo: &Path) -> PathBuf {
    repo.join(".SddIA/state/tracker-outbound")
}

fn outbound_file(repo: &Path, issue_ref: &str) -> PathBuf {
    let safe = issue_ref
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
        .collect::<String>();
    outbound_dir(repo).join(format!("{safe}.jsonl"))
}

fn issue_ref_for_record(request: &Value, tool_result: &Value) -> Option<String> {
    let inner = request_inner(request)?;
    let op = inner.get("operation").and_then(|v| v.as_str())?;
    if op == "create_issue" {
        return tool_result
            .get("issue_ref")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
    }
    inner
        .get("issue_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Append fail-soft tras operación Linear aceptada (invocado desde `invoke_tool_for_process`).
pub fn record_linear_outbound_after_success(
    repo: &Path,
    process_name: Option<&str>,
    request_payload: &Value,
    tool_result: &Value,
) {
    if let Err(e) = try_record(repo, process_name, request_payload, tool_result) {
        eprintln!("warn: tracker-outbound — {e}");
    }
}

pub fn try_record(
    repo: &Path,
    process_name: Option<&str>,
    request_payload: &Value,
    tool_result: &Value,
) -> Result<(), String> {
    let inner = request_inner(request_payload).ok_or("sin request")?;
    let operation = inner
        .get("operation")
        .and_then(|v| v.as_str())
        .ok_or("sin operation")?;
    if !RECORDED_OPS.contains(&operation) {
        return Ok(());
    }
    let issue_ref = issue_ref_for_record(request_payload, tool_result)
        .ok_or("sin issue_ref para registro")?;
    let event_id = uuid::Uuid::new_v4().to_string();
    let occurred_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let source_process = process_name
        .filter(|s| !s.is_empty())
        .unwrap_or("unknown");
    let line = json!({
        "event_id": event_id,
        "operation": operation,
        "issue_ref": issue_ref,
        "occurred_at": occurred_at,
        "source_process": source_process,
    });
    let serialized = serde_json::to_string(&line).map_err(|e| e.to_string())?;
    if serialized.contains("LINEAR_API_TOKEN") {
        return Err("línea contiene secreto prohibido".into());
    }
    let dir = outbound_dir(repo);
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir outbound: {e}"))?;
    let path = outbound_file(repo, &issue_ref);
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("open outbound: {e}"))?;
    f.write_all(serialized.as_bytes())
        .and_then(|_| f.write_all(b"\n"))
        .map_err(|e| format!("write outbound: {e}"))?;
    Ok(())
}

pub fn should_record_linear_tool(repo: &Path, tool_name: &str, request_payload: &Value) -> bool {
    let Some(expected) = resolved_tracker_tool_name(repo) else {
        return false;
    };
    if tool_name != expected {
        return false;
    }
    let inner = match request_inner(request_payload) {
        Some(v) => v,
        None => return false,
    };
    match inner.get("operation").and_then(|v| v.as_str()) {
        Some(op) => RECORDED_OPS.contains(&op),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_repo() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "sddia-outbound-{}",
            uuid::Uuid::new_v4()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn ac_out_1_accepted_writes_five_fields() {
        let repo = tmp_repo();
        let req = json!({
            "request": {
                "operation": "update_issue_state",
                "issue_ref": "OSC-9",
                "state_name": "Done",
            }
        });
        try_record(&repo, Some("tracker-stamp"), &req, &json!({})).unwrap();
        let path = outbound_file(&repo, "OSC-9");
        let text = fs::read_to_string(&path).unwrap();
        let line = text.lines().next().unwrap();
        let v: Value = serde_json::from_str(line).unwrap();
        assert!(v.get("event_id").and_then(|x| x.as_str()).is_some());
        assert_eq!(v.get("operation"), Some(&json!("update_issue_state")));
        assert_eq!(v.get("issue_ref"), Some(&json!("OSC-9")));
        assert!(v.get("occurred_at").and_then(|x| x.as_str()).is_some());
        assert_eq!(v.get("source_process"), Some(&json!("tracker-stamp")));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn ac_out_1_create_issue_uses_result_ref() {
        let repo = tmp_repo();
        let req = json!({
            "request": {
                "operation": "create_issue",
                "team_key": "OSC",
                "title": "t",
            }
        });
        try_record(
            &repo,
            Some("forge-pbi"),
            &req,
            &json!({ "issue_ref": "OSC-42" }),
        )
        .unwrap();
        let text = fs::read_to_string(outbound_file(&repo, "OSC-42")).unwrap();
        assert!(text.contains("\"operation\":\"create_issue\""));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn ac_out_1_fetch_does_not_write() {
        let repo = tmp_repo();
        let req = json!({
            "request": { "operation": "fetch_issue", "issue_ref": "OSC-1" }
        });
        try_record(&repo, Some("tracker-stamp"), &req, &json!({})).unwrap();
        assert!(!outbound_dir(&repo).exists() || fs::read_dir(outbound_dir(&repo)).unwrap().count() == 0);
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn ac_out_2_line_has_no_token_or_graphql_body() {
        let repo = tmp_repo();
        let req = json!({
            "request": {
                "operation": "create_comment",
                "issue_ref": "OSC-7",
                "body": "note",
            }
        });
        try_record(&repo, Some("tracker-sync-replay"), &req, &json!({})).unwrap();
        let raw = fs::read_to_string(outbound_file(&repo, "OSC-7")).unwrap();
        assert!(!raw.contains("LINEAR_API_TOKEN"));
        assert!(!raw.contains("\"data\":{\"issue\""));
        let _ = fs::remove_dir_all(&repo);
    }

    fn write_file(dir: &Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(p, body).unwrap();
    }

    fn fixture_binding(dir: &Path, provider: &str) {
        write_file(
            dir,
            "SddIA/core/cumulo.paths.json",
            r#"{ "capability_di": { "bindings": "SddIA/core/capability-bindings.md" } }"#,
        );
        write_file(
            dir,
            "SddIA/core/capability-bindings.md",
            &format!(
                "---\nbindings:\n  - capability_id: \"tracker:lifecycle\"\n    contract: \"tracker.lifecycle\"\n    provider: \"{provider}\"\n---\n"
            ),
        );
    }

    #[test]
    fn ac2_update_issue_description_writes_jsonl() {
        let repo = tmp_repo();
        let req = json!({
            "request": {
                "operation": "update_issue_description",
                "issue_ref": "OSC-15",
                "description": "persist_ref docs/features/x"
            }
        });
        try_record(&repo, Some("tracker-stamp"), &req, &json!({})).unwrap();
        let text = fs::read_to_string(outbound_file(&repo, "OSC-15")).unwrap();
        assert!(text.contains("\"operation\":\"update_issue_description\""));
        assert!(text.contains("\"issue_ref\":\"OSC-15\""));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn records_only_the_resolved_provider() {
        let repo = tmp_repo();
        fixture_binding(&repo, "tool:linear-tracker-adapter");
        let req = json!({
            "request": { "operation": "update_issue_description", "issue_ref": "OSC-1" }
        });
        assert!(should_record_linear_tool(&repo, "linear-tracker-adapter", &req));
        assert!(!should_record_linear_tool(&repo, "other-adapter", &req));
        let fetch = json!({
            "request": { "operation": "fetch_issue", "issue_ref": "OSC-1" }
        });
        assert!(!should_record_linear_tool(&repo, "linear-tracker-adapter", &fetch));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn provides_matches_binding_row() {
        std::env::remove_var("SDDIA_LAB_SKIP_CAPABILITY_DI");
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .expect("repo");
        let name = resolved_tracker_tool_name(&repo).expect("fila");
        assert_eq!(name, "linear-tracker-adapter");
        let phase = json!({
            "requires_capability": [{
                "id": "tracker:lifecycle",
                "contract": "tracker.lifecycle",
                "version": ">=1.0.0"
            }],
            "delegates_to": ["tool:linear-tracker-adapter"]
        });
        super::super::capability_di_gate::validate_phase_capability_di(
            &repo,
            &phase,
            "tracker-lifecycle-provider",
        )
        .expect("provides alineado al binding y outputs cubren success/exitCode");
    }
}
