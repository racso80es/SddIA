//! Handler nativo `email-digest-preference-reply`.
//! TelegramCallback_Received `dpref:*` → User_Preference_Change_Requested.

use super::super::daemons::state_dir;
use super::super::user_preference_change_requested;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

const STATE_FILE: &str = "email-noise-digest.json";
const NS: &str = "dpref:";

fn ok_data(data: Value, phases: Vec<Value>) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(data),
        error: None,
        execution_report: Some(json!({
            "process_name": "email-digest-preference-reply",
            "phases": phases,
        })),
        exit_code: 0,
    }
}

fn fail_envelope(error: impl Into<String>, phases: Vec<Value>) -> OrchestratorEnvelope {
    let error = error.into();
    OrchestratorEnvelope {
        success: false,
        status_code: 1,
        data: Some(json!({
            "success": false,
            "preference_emitted": false,
        })),
        error: Some(error.clone()),
        execution_report: Some(json!({
            "process_name": "email-digest-preference-reply",
            "phases": phases,
        })),
        exit_code: 1,
    }
}

fn skipped(reason: &str, phases: Vec<Value>) -> OrchestratorEnvelope {
    ok_data(
        json!({
            "success": true,
            "skipped": true,
            "reason": reason,
            "preference_emitted": false,
        }),
        phases,
    )
}

fn load_event(repo: &Path, process_inputs: &Value) -> Result<Value, String> {
    if let Some(rel) = process_inputs
        .get("event_file_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let path = if Path::new(rel).is_absolute() {
            PathBuf::from(rel)
        } else {
            repo.join(rel)
        };
        let raw = fs::read_to_string(&path).map_err(|e| format!("leer evento: {e}"))?;
        return serde_json::from_str(&raw).map_err(|e| format!("JSON evento: {e}"));
    }
    let callback_data = process_inputs
        .get("callback_data")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_file_path o callback_data requerido")?;
    Ok(json!({
        "event_id": process_inputs.get("event_id").cloned().unwrap_or(json!("")),
        "event_type": "TelegramCallback_Received",
        "payload": {
            "callback_data": callback_data,
            "chat_id": process_inputs.get("chat_id").cloned().unwrap_or(json!("")),
            "message_id": process_inputs.get("message_id").cloned().unwrap_or(Value::Null),
            "source": "telegram",
        }
    }))
}

fn parse_dpref(raw: &str) -> Option<(&str, &str)> {
    let trimmed = raw.trim();
    let rest = trimmed.strip_prefix(NS)?;
    let (action, token) = rest.split_once(':')?;
    if action.is_empty() || token.is_empty() {
        return None;
    }
    if !token.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some((action, token))
}

fn load_tokens(repo: &Path) -> Value {
    let Ok(path) = state_dir(repo).map(|d| d.join(STATE_FILE)) else {
        return json!({});
    };
    if !path.is_file() {
        return json!({});
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.get("tokens").cloned())
        .unwrap_or_else(|| json!({}))
}

fn emit_pref(
    repo: &Path,
    action: &str,
    subject_key: &str,
    source_event_id: &str,
) -> Result<Value, String> {
    if subject_key.contains('@') {
        return Err("pii-forbidden".into());
    }
    let mut payload = json!({
        "subject_kind": "person",
        "subject_key": subject_key,
        "scope_type": "channel",
        "scope_id": "email",
        "source_event_id": source_event_id,
    });
    match action {
        "max" => {
            payload["predicate"] = json!("priority");
            payload["priority_level"] = json!("max");
        }
        "mute" => {
            payload["predicate"] = json!("mute");
            payload["value"] = json!({"muted": true});
        }
        _ => return Err(format!("acción no emisible: {action}")),
    }
    user_preference_change_requested::run(
        repo,
        &json!({
            "operation": "activate",
            "channel": "telegram",
            "payload": payload,
        }),
    )
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let event = match load_event(repo, process_inputs) {
        Ok(v) => v,
        Err(e) => return Ok(fail_envelope(e, vec![])),
    };
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    let callback_data = payload
        .get("callback_data")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    let mut phases = Vec::new();
    if !callback_data.starts_with(NS) {
        phases.push(json!({
            "phase_name": "Gate-Callback",
            "status": "skipped",
            "reason": "foreign-namespace",
        }));
        return Ok(skipped("foreign-namespace", phases));
    }
    let Some((action, token)) = parse_dpref(&callback_data) else {
        phases.push(json!({
            "phase_name": "Gate-Callback",
            "status": "skipped",
            "reason": "invalid-callback",
        }));
        return Ok(skipped("invalid-callback", phases));
    };
    phases.push(json!({
        "phase_name": "Gate-Callback",
        "status": "executed",
        "action": action,
    }));

    if action != "max" && action != "mute" && action != "ign" {
        phases.push(json!({
            "phase_name": "Correlacion-Token",
            "status": "skipped",
            "reason": "unknown-action",
        }));
        return Ok(skipped("unknown-action", phases));
    }

    let tokens = load_tokens(repo);
    let Some(entry) = tokens.get(token) else {
        phases.push(json!({
            "phase_name": "Correlacion-Token",
            "status": "skipped",
            "reason": "skipped-expired",
            "token": token,
        }));
        return Ok(skipped("skipped-expired", phases));
    };
    let Some(subject_key) = entry
        .get("subject_key")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()))
        .map(str::to_string)
    else {
        phases.push(json!({
            "phase_name": "Correlacion-Token",
            "status": "skipped",
            "reason": "skipped-expired",
        }));
        return Ok(skipped("skipped-expired", phases));
    };
    phases.push(json!({
        "phase_name": "Correlacion-Token",
        "status": "executed",
    }));

    if action == "ign" {
        phases.push(json!({
            "phase_name": "Emision-Preferencia",
            "status": "skipped",
            "reason": "ignore",
        }));
        return Ok(skipped("ignore", phases));
    }

    match emit_pref(repo, action, &subject_key, &event_id) {
        Ok(seal) => {
            if seal.get("success") != Some(&json!(true)) {
                phases.push(json!({
                    "phase_name": "Emision-Preferencia",
                    "status": "failed",
                    "error": seal.get("error").cloned().unwrap_or(json!("emit failed")),
                }));
                return Ok(fail_envelope("emit-user-preference-change-requested failed", phases));
            }
            let target_event_id = seal.get("event_id").cloned();
            phases.push(json!({
                "phase_name": "Emision-Preferencia",
                "status": "executed",
                "operation": "activate",
                "target_event_id": target_event_id,
            }));
            Ok(ok_data(
                json!({
                    "success": true,
                    "skipped": false,
                    "preference_emitted": true,
                    "operation": "activate",
                    "target_event_id": target_event_id,
                }),
                phases,
            ))
        }
        Err(e) => {
            phases.push(json!({
                "phase_name": "Emision-Preferencia",
                "status": "failed",
                "error": e,
            }));
            Ok(fail_envelope("emit-user-preference-change-requested failed", phases))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::email_triage;
    use super::super::user_preference;
    use user_preference_core::{canonical_subject_key_from_addr, query, QuerySpec};

    fn repo_with_cumulo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join(".events/domain")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
  "eda_instance": {"proofs": ".SddIA/proofs"},
  "daemons_instance": {"state": ".SddIA/daemons/state"},
  "eda_fractal": {"domain": ".events/domain", "orchestration": ".events/orchestration", "telemetry": ".events/telemetry"},
  "paths": {"userPreferencesStore": ".SddIA/vector_store/user_preferences"}
}"#,
        )
        .unwrap();
        tmp
    }

    fn write_callback(repo: &Path, name: &str, callback_data: &str) -> String {
        let path = repo.join(".events/domain").join(format!("{name}.json"));
        let event = json!({
            "event_id": name,
            "event_type": "TelegramCallback_Received",
            "payload": {
                "callback_data": callback_data,
                "chat_id": "1",
                "source": "telegram"
            }
        });
        fs::write(&path, serde_json::to_string(&event).unwrap()).unwrap();
        format!(".events/domain/{name}.json")
    }

    fn seed_token(repo: &Path, token: &str, from: &str) {
        let dir = repo.join(".SddIA/daemons/state");
        fs::create_dir_all(&dir).unwrap();
        let sk = canonical_subject_key_from_addr(from);
        fs::write(
            dir.join(STATE_FILE),
            serde_json::to_string(&json!({
                "tokens": {
                    token: {
                        "subject_key": sk,
                        "rule": "C-NOREPLY",
                        "created_at": "2026-09-12T00:00:00Z"
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn foreign_namespace_skips() {
        let tmp = repo_with_cumulo();
        let rel = write_callback(tmp.path(), "cb1", "other:foo");
        let env = run(tmp.path(), &json!({"event_file_path": rel})).unwrap();
        assert!(env.success);
        let data = env.data.unwrap();
        assert_eq!(data["skipped"], json!(true));
        assert_eq!(data["reason"], json!("foreign-namespace"));
        assert_eq!(data["preference_emitted"], json!(false));
    }

    #[test]
    fn unknown_token_skipped_expired() {
        let tmp = repo_with_cumulo();
        seed_token(tmp.path(), "abcd1234", "noreply@shop.tld");
        let rel = write_callback(tmp.path(), "cb2", "dpref:max:ffffffff");
        let env = run(tmp.path(), &json!({"event_file_path": rel})).unwrap();
        assert!(env.success);
        assert_eq!(env.data.unwrap()["reason"], json!("skipped-expired"));
    }

    #[test]
    fn ign_does_not_emit() {
        let tmp = repo_with_cumulo();
        seed_token(tmp.path(), "abcd1234", "noreply@shop.tld");
        let rel = write_callback(tmp.path(), "cb3", "dpref:ign:abcd1234");
        let env = run(tmp.path(), &json!({"event_file_path": rel})).unwrap();
        assert!(env.success);
        let data = env.data.unwrap();
        assert_eq!(data["reason"], json!("ignore"));
        assert_eq!(data["preference_emitted"], json!(false));
        let domain = tmp.path().join(".events/domain");
        let extra: Vec<_> = fs::read_dir(&domain)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() != "cb3.json")
            .collect();
        assert!(extra.is_empty());
    }

    #[test]
    fn max_emits_priority_without_addr() {
        let tmp = repo_with_cumulo();
        seed_token(tmp.path(), "abcd1234", "noreply@shop.tld");
        let rel = write_callback(tmp.path(), "cb4", "dpref:max:abcd1234");
        let env = run(tmp.path(), &json!({"event_file_path": rel})).unwrap();
        assert!(env.success);
        let data = env.data.unwrap();
        assert_eq!(data["preference_emitted"], json!(true));
        let eid = data["target_event_id"].as_str().unwrap();
        let emitted = fs::read_to_string(tmp.path().join(".events/domain").join(format!("{eid}.json")))
            .unwrap();
        assert!(!emitted.contains("noreply@"));
        assert!(!emitted.contains("authority"));
        let ev: Value = serde_json::from_str(&emitted).unwrap();
        assert_eq!(ev["event_type"], json!("User_Preference_Change_Requested"));
        assert_eq!(ev["payload"]["operation"], json!("activate"));
        assert_eq!(ev["payload"]["predicate"], json!("priority"));
        assert_eq!(ev["payload"]["priority_level"], json!("max"));
        assert_eq!(
            ev["payload"]["subject_key"],
            json!(canonical_subject_key_from_addr("noreply@shop.tld"))
        );
    }

    #[test]
    fn mute_emits_muted_true() {
        let tmp = repo_with_cumulo();
        seed_token(tmp.path(), "abcd1234", "noreply@shop.tld");
        let rel = write_callback(tmp.path(), "cb5", "dpref:mute:abcd1234");
        let env = run(tmp.path(), &json!({"event_file_path": rel})).unwrap();
        assert!(env.success);
        let eid = env.data.unwrap()["target_event_id"].as_str().unwrap().to_string();
        let ev: Value = serde_json::from_str(
            &fs::read_to_string(tmp.path().join(".events/domain").join(format!("{eid}.json")))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(ev["payload"]["predicate"], json!("mute"));
        assert_eq!(ev["payload"]["value"]["muted"], json!(true));
        assert!(ev["payload"].get("priority_level").is_none());
    }

    #[test]
    fn max_loop_triggers_p_exempt_c() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        seed_token(repo, "abcd1234", "noreply@example.com");
        let rel = write_callback(repo, "cb6", "dpref:max:abcd1234");
        let env = run(repo, &json!({"event_file_path": rel})).unwrap();
        assert_eq!(env.data.as_ref().unwrap()["preference_emitted"], json!(true));
        let eid = env.data.as_ref().unwrap()["target_event_id"]
            .as_str()
            .unwrap()
            .to_string();
        let ingest = user_preference::run_ingest(
            repo,
            &json!({"event_file_path": format!(".events/domain/{eid}.json")}),
        )
        .unwrap();
        assert!(ingest.success);
        let hits = query(
            repo,
            &QuerySpec {
                subject_key: Some(canonical_subject_key_from_addr("noreply@example.com")),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(hits[0].authority, user_preference_core::PreferenceAuthority::ExplicitUser);
        assert_eq!(hits[0].status, user_preference_core::PreferenceStatus::Active);
        assert_eq!(hits[0].value["level"], json!("max"));

        let recv = repo.join(".events/domain/mail1.json");
        fs::write(
            &recv,
            serde_json::to_string(&json!({
                "event_id": "mail1",
                "event_type": "Email_Received",
                "payload": {
                    "message_uid": "9",
                    "from": "noreply@example.com",
                    "subject": "Receipt"
                }
            }))
            .unwrap(),
        )
        .unwrap();
        let triage = email_triage::run(repo, &json!({"event_file_path": ".events/domain/mail1.json"}))
            .unwrap();
        let phases = triage.execution_report.as_ref().unwrap()["phases"]
            .as_array()
            .unwrap();
        let c = phases
            .iter()
            .find(|p| p["phase_name"] == "Triaje-C")
            .unwrap();
        assert_eq!(c["status"], json!("skipped"));
        assert_eq!(c["reason"], json!("P-EXEMPT-C"));
    }
}
