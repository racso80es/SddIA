//! Handler nativo `email-quick-action-ingest`. Persiste intención; no IMAP STORE.

use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

const ACTIONS: &[&str] = &["archive", "draft", "delegate"];

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn proofs_root(repo: &Path) -> PathBuf {
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let default = repo.join(".SddIA").join("proofs");
    let Ok(text) = fs::read_to_string(&cfg_path) else {
        return default;
    };
    let Ok(cfg) = serde_json::from_str::<Value>(&text) else {
        return default;
    };
    cfg.get("eda_instance")
        .and_then(|e| e.get("proofs"))
        .and_then(|v| v.as_str())
        .map(|s| {
            let rel = s.trim().trim_start_matches("./");
            repo.join(rel)
        })
        .unwrap_or(default)
}

fn persist_proof(repo: &Path, event: &Value) -> Result<PathBuf, String> {
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .ok_or("event_id requerido")?;
    let dir = proofs_root(repo).join("email-quick-action");
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir email-quick-action proof: {e}"))?;
    let path = dir.join(format!("{event_id}.json"));
    let body = json!({
        "kind": "email-quick-action-proof",
        "event_id": event_id,
        "event_type": "Email_Quick_Action_Requested",
        "timestamp": event.get("timestamp").cloned().unwrap_or_else(|| json!(iso_now())),
        "payload": event.get("payload"),
        "recorded_at": iso_now(),
    });
    fs::write(&path, format!("{body}\n")).map_err(|e| format!("write proof: {e}"))?;
    Ok(path)
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let rel = process_inputs
        .get("event_file_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_file_path requerido")?;
    let path: PathBuf = {
        let p = Path::new(rel);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            repo.join(rel)
        }
    };
    let raw = fs::read_to_string(&path).map_err(|e| format!("leer evento: {e}"))?;
    let event: Value = serde_json::from_str(&raw).map_err(|e| format!("JSON evento: {e}"))?;
    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    let uid = payload
        .get("message_uid")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let action = payload
        .get("action")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_ascii_lowercase);

    let mut phases = Vec::new();
    let Some(uid) = uid else {
        phases.push(json!({
            "phase_name": "Gate",
            "status": "skipped",
            "reason": "missing-message_uid",
        }));
        return Ok(ok_envelope(false, phases));
    };
    let Some(action) = action else {
        phases.push(json!({
            "phase_name": "Gate",
            "status": "skipped",
            "reason": "missing-action",
        }));
        return Ok(ok_envelope(false, phases));
    };
    if !ACTIONS.contains(&action.as_str()) {
        phases.push(json!({
            "phase_name": "Gate",
            "status": "skipped",
            "reason": "invalid-action",
            "action": action,
        }));
        return Ok(ok_envelope(false, phases));
    }
    phases.push(json!({
        "phase_name": "Gate",
        "status": "executed",
        "message_uid": uid,
        "action": action,
    }));

    match persist_proof(repo, &event) {
        Ok(proof) => {
            phases.push(json!({
                "phase_name": "Persistencia",
                "status": "executed",
                "proof": proof.to_string_lossy(),
            }));
            Ok(ok_envelope(true, phases))
        }
        Err(e) => {
            phases.push(json!({
                "phase_name": "Persistencia",
                "status": "failed",
                "error": e,
            }));
            Ok(OrchestratorEnvelope {
                success: false,
                status_code: 1,
                data: Some(json!({"recorded": false})),
                error: Some("persist proof failed".into()),
                execution_report: Some(json!({
                    "process_name": "email-quick-action-ingest",
                    "phases": phases,
                })),
                exit_code: 1,
            })
        }
    }
}

fn ok_envelope(recorded: bool, phases: Vec<Value>) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({"recorded": recorded})),
        error: None,
        execution_report: Some(json!({
            "process_name": "email-quick-action-ingest",
            "phases": phases,
        })),
        exit_code: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_skips_invalid_action() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let ev_path = repo.join("ev.json");
        fs::write(
            &ev_path,
            r#"{"event_id":"aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee","event_type":"Email_Quick_Action_Requested","payload":{"message_uid":"1","action":"explode"}}"#,
        )
        .unwrap();
        let env = run(repo, &json!({"event_file_path": ev_path.to_string_lossy()})).unwrap();
        assert!(env.success);
        assert_eq!(env.data.unwrap()["recorded"], json!(false));
    }

    #[test]
    fn persist_archive_writes_proof() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        let ev_path = repo.join("ev.json");
        fs::write(
            &ev_path,
            r#"{"event_id":"11111111-1111-1111-1111-111111111111","event_type":"Email_Quick_Action_Requested","timestamp":"2026-08-19T00:00:00Z","payload":{"message_uid":"42","action":"archive","channel":"kalma2"}}"#,
        )
        .unwrap();
        let env = run(repo, &json!({"event_file_path": ev_path.to_string_lossy()})).unwrap();
        assert!(env.success);
        assert_eq!(env.data.unwrap()["recorded"], json!(true));
        assert!(repo
            .join(".SddIA/proofs/email-quick-action/11111111-1111-1111-1111-111111111111.json")
            .is_file());
    }
}
