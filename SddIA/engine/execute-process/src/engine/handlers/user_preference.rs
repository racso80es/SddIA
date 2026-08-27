//! Memoria soberana de preferencias — ingest/fractal en execute-process; store en `user-preference-core`.

use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub use user_preference_core::*;

pub fn build_pref_context_hint(repo: &Path) -> String {
    let pref_ctx = query_context_block_with_capsule_fallback(
        repo,
        &QuerySpec {
            max_results: Some(8),
            ..Default::default()
        },
    );
    if pref_ctx
        .get("preferences")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        == Some(true)
    {
        format!(
            "\n[Contexto usuario — preferencias activas: {}]",
            pref_ctx["preferences"]
        )
    } else {
        String::new()
    }
}

pub fn query_context_block_with_capsule_fallback(repo: &Path, spec: &QuerySpec) -> Value {
    let payload = json!({"op": "QUERY_CONTEXT", "spec": spec});
    if let Ok(inv) = crate::engine::capsules::invoke_capsule_json(
        repo,
        "user-preference-store",
        &payload,
        true,
    ) {
        if inv.exit_code == 0 && inv.body.get("success") == Some(&json!(true)) {
            if let Some(block) = inv
                .body
                .get("result")
                .and_then(|r| r.get("schema_version").map(|_| r.clone()))
                .or_else(|| {
                    inv.body
                        .pointer("/result/result")
                        .filter(|v| v.get("schema_version").is_some())
                        .cloned()
                })
            {
                return block;
            }
        }
    }
    user_preference_core::query_context_block(repo, spec)
}

pub fn run_capsule(repo: &Path, request: &Value) -> Value {
    user_preference_core::run_capsule(repo, request)
}

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn emit_changed_event(
    repo: &Path,
    pref: &UserPreference,
    operation: &str,
) -> Result<Value, String> {
    use super::super::fractal::{load_fractal_dirs, write_fractal_event};

    let event_id = uuid::Uuid::new_v4().to_string();
    let event = json!({
        "event_id": event_id,
        "event_type": "User_Preference_Changed",
        "timestamp": iso_now(),
        "emitter_agent": "user-preference-ingest",
        "payload": {
            "preference_id": pref.preference_id,
            "revision_id": pref.revision_id,
            "operation": operation,
            "scope_type": format!("{:?}", pref.scope_type).to_lowercase(),
            "status": format!("{:?}", pref.status).to_lowercase(),
            "predicate": pref.predicate,
            "sensitivity": pref.sensitivity,
        }
    });
    let (_, _, domain_dir, _) = load_fractal_dirs(repo);
    write_fractal_event(repo, &event, &domain_dir)
}

pub fn run_ingest(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let rel = process_inputs
        .get("event_file_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_file_path requerido")?;
    let path = if Path::new(rel).is_absolute() {
        PathBuf::from(rel)
    } else {
        repo.join(rel)
    };
    let raw = fs::read_to_string(&path).map_err(|e| format!("leer evento: {e}"))?;
    let event: Value = serde_json::from_str(&raw).map_err(|e| format!("JSON evento: {e}"))?;
    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    let operation = payload
        .get("operation")
        .and_then(|v| v.as_str())
        .unwrap_or("activate")
        .to_ascii_lowercase();

    let mut phases = Vec::new();
    if operation == "ignore" {
        phases.push(json!({"phase_name": "Gate", "status": "skipped", "reason": "ignore"}));
        return Ok(ok_ingest_envelope(false, phases, None));
    }
    if operation == "purge" {
        let pid = payload
            .get("preference_id")
            .and_then(|v| v.as_str())
            .ok_or("preference_id requerido para purge")?;
        purge_preference(repo, pid)?;
        phases.push(json!({"phase_name": "Persistir", "status": "executed", "op": "purge"}));
        return Ok(ok_ingest_envelope(true, phases, None));
    }

    phases.push(json!({"phase_name": "Gate", "status": "executed", "operation": operation}));

    let Some(mut pref) = preference_from_event_payload(&payload, &operation)? else {
        phases.push(json!({"phase_name": "Destilar", "status": "skipped"}));
        return Ok(ok_ingest_envelope(false, phases, None));
    };
    phases.push(json!({"phase_name": "Destilar", "status": "executed"}));

    pref = put_revision(repo, pref)?;
    phases.push(json!({
        "phase_name": "Persistir",
        "status": "executed",
        "preference_id": pref.preference_id,
        "revision_id": pref.revision_id,
    }));

    let seal = emit_changed_event(repo, &pref, &operation).ok();
    phases.push(json!({
        "phase_name": "Sellar",
        "status": if seal.is_some() { "executed" } else { "failed" },
    }));

    Ok(ok_ingest_envelope(true, phases, Some(pref)))
}

fn ok_ingest_envelope(
    recorded: bool,
    phases: Vec<Value>,
    pref: Option<UserPreference>,
) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "recorded": recorded,
            "preference_id": pref.as_ref().map(|p| p.preference_id.clone()),
            "revision_id": pref.as_ref().map(|p| p.revision_id.clone()),
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": "user-preference-ingest",
            "phases": phases,
        })),
        exit_code: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_repo(tmp: &Path) {
        fs::create_dir_all(tmp.join("SddIA/core")).unwrap();
        fs::create_dir_all(tmp.join(".events/domain")).unwrap();
        fs::write(
            tmp.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"userPreferencesStore":".SddIA/vector_store/user_preferences"},"eda_fractal":{"domain":".events/domain"}}"#,
        )
        .unwrap();
    }

    fn write_domain_event(tmp: &Path, event_id: &str, payload: Value) -> PathBuf {
        let path = tmp.join(".events/domain").join(format!("{event_id}.json"));
        let event = json!({
            "event_id": event_id,
            "event_type": "User_Preference_Change_Requested",
            "timestamp": "2026-08-27T12:00:00Z",
            "payload": payload,
        });
        fs::write(&path, serde_json::to_string(&event).unwrap()).unwrap();
        path
    }

    #[test]
    fn cross_channel_activate_query_revoke_via_ingest() {
        let tmp = tempfile::tempdir().unwrap();
        setup_repo(tmp.path());
        let subject = "hash-juan-smoke";

        write_domain_event(
            tmp.path(),
            "evt-activate-1",
            json!({
                "operation": "activate",
                "channel": "kalma2",
                "subject_key": subject,
                "subject_kind": "person",
                "predicate": "priority",
                "priority_level": "max",
                "scope_type": "channel",
                "scope_id": "email",
            }),
        );
        let activate = run_ingest(
            tmp.path(),
            &json!({"event_file_path": ".events/domain/evt-activate-1.json"}),
        )
        .unwrap();
        assert!(activate.success);
        assert_eq!(activate.data.as_ref().unwrap()["recorded"], true);

        let ctx = query_context_block(
            tmp.path(),
            &QuerySpec {
                subject_key: Some(subject.into()),
                ..Default::default()
            },
        );
        assert_eq!(ctx["preferences"].as_array().unwrap().len(), 1);
        assert_eq!(ctx["preferences"][0]["value"]["level"], "max");

        write_domain_event(
            tmp.path(),
            "evt-revoke-1",
            json!({
                "operation": "revoke",
                "channel": "kalma2",
                "subject_key": subject,
                "subject_kind": "person",
                "predicate": "priority",
                "scope_type": "channel",
                "scope_id": "email",
            }),
        );
        let revoke = run_ingest(
            tmp.path(),
            &json!({"event_file_path": ".events/domain/evt-revoke-1.json"}),
        )
        .unwrap();
        assert!(revoke.success);

        let ctx_after = query_context_block(
            tmp.path(),
            &QuerySpec {
                subject_key: Some(subject.into()),
                ..Default::default()
            },
        );
        assert!(ctx_after["preferences"].as_array().unwrap().is_empty());
    }
}
