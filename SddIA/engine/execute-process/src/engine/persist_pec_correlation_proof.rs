//! Handler nativo `persist-pec-correlation-proof` — testigo durable PEC (S2).

use super::workspace::load_paths_config;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub const PROOF_KIND: &str = "pec-correlation-proof";
pub const PROOF_NAMESPACE: &str = "pec-correlation";

#[derive(Debug, PartialEq, Eq)]
pub enum PersistOutcome {
    Wrote(String),
    SkippedNoCid,
}

pub fn resolve_eda_proofs_dir(repo: &Path) -> PathBuf {
    if let Ok(cfg) = load_paths_config(repo) {
        if let Some(p) = cfg
            .get("eda_instance")
            .and_then(|e| e.get("proofs"))
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let pb = PathBuf::from(p);
            return if pb.is_absolute() {
                pb
            } else {
                repo.join(pb)
            };
        }
    }
    repo.join(".SddIA").join("proofs")
}

pub fn proof_path(repo: &Path, correlation_id: &str) -> PathBuf {
    resolve_eda_proofs_dir(repo)
        .join(PROOF_NAMESPACE)
        .join(format!("{correlation_id}.json"))
}

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Ejecuta desde inputs de acción (payload PEC o envelope enriquecido).
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    match run_inner(repo, inputs)? {
        PersistOutcome::Wrote(rel) => Ok(json!({
            "success": true,
            "skipped": false,
            "proof_path": rel,
        })),
        PersistOutcome::SkippedNoCid => Ok(json!({
            "success": true,
            "skipped": true,
            "proof_path": Value::Null,
        })),
    }
}

/// Despacho desde el bus: usa el evento PEC completo.
pub fn run_from_event(repo: &Path, event: &Value) -> Result<PersistOutcome, String> {
    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    let mut inputs = payload;
    if let Some(obj) = inputs.as_object_mut() {
        if let Some(eid) = event.get("event_id") {
            obj.entry("event_id".to_string()).or_insert(eid.clone());
        }
        if let Some(ts) = event.get("timestamp") {
            obj.entry("timestamp".to_string()).or_insert(ts.clone());
        }
    }
    run_inner(repo, &inputs)
}

fn run_inner(repo: &Path, inputs: &Value) -> Result<PersistOutcome, String> {
    let Some(cid) = str_field(inputs, "correlation_id") else {
        return Ok(PersistOutcome::SkippedNoCid);
    };
    let process_name = str_field(inputs, "process_name").unwrap_or_else(|| "?".into());
    let status = str_field(inputs, "status").unwrap_or_else(|| "success".into());
    let cycle_phase = str_field(inputs, "cycle_phase").unwrap_or_else(|| "completed".into());
    let pec_event_id = str_field(inputs, "event_id");
    let timestamp = str_field(inputs, "timestamp")
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());

    let proof = json!({
        "kind": PROOF_KIND,
        "correlation_id": cid,
        "pec_event_id": pec_event_id,
        "timestamp": timestamp,
        "payload": {
            "process_name": process_name,
            "status": status,
            "cycle_phase": cycle_phase,
        }
    });

    let path = proof_path(repo, &cid);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&proof).map_err(|e| e.to_string())?;
    fs::write(&path, format!("{text}\n")).map_err(|e| e.to_string())?;

    let rel = path
        .strip_prefix(repo)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"));
    Ok(PersistOutcome::Wrote(rel))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn persist_writes_namespaced_proof() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        let cid = "e273713c-dd91-487b-8716-1bdc8c5da741";
        let event = json!({
            "event_id": "9ff24776-26c7-4596-8b08-7b6fc4531641",
            "timestamp": "2026-08-15T12:00:00Z",
            "payload": {
                "correlation_id": cid,
                "process_name": "feature",
                "status": "success",
                "cycle_phase": "completed"
            }
        });
        let out = run_from_event(repo, &event).unwrap();
        match out {
            PersistOutcome::Wrote(rel) => {
                assert!(rel.contains("pec-correlation"));
                assert!(rel.contains(cid));
            }
            PersistOutcome::SkippedNoCid => panic!("expected write"),
        }
        let body: Value = serde_json::from_str(
            &fs::read_to_string(proof_path(repo, cid)).unwrap(),
        )
        .unwrap();
        assert_eq!(body["kind"], PROOF_KIND);
        assert_eq!(body["payload"]["process_name"], "feature");
        assert_eq!(body["payload"]["cycle_phase"], "completed");
    }

    #[test]
    fn persist_skips_without_correlation_id() {
        let tmp = tempfile::tempdir().unwrap();
        let event = json!({
            "event_id": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
            "payload": {"process_name": "orphan", "status": "success"}
        });
        let out = run_from_event(tmp.path(), &event).unwrap();
        assert_eq!(out, PersistOutcome::SkippedNoCid);
    }

    #[test]
    fn route_persists_proof_then_purges_parent() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
              "eda_fractal": {
                "orchestration": "./.events/orchestration",
                "orchestration_subscriptions": "SddIA/core/event-orchestration-subscriptions.json"
              },
              "eda_instance": {"proofs": ".SddIA/proofs"}
            }"#,
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/core/event-orchestration-subscriptions.json"),
            r#"{
              "Process_Execution_Completed": [
                {"agent":"cumulo","action":"persist-pec-correlation-proof"}
              ]
            }"#,
        )
        .unwrap();
        let orch = repo.join(".events/orchestration");
        fs::create_dir_all(&orch).unwrap();
        let eid = "9ff24776-26c7-4596-8b08-7b6fc4531641";
        let cid = "e273713c-dd91-487b-8716-1bdc8c5da741";
        let ev = json!({
            "event_id": eid,
            "event_type": "Process_Execution_Completed",
            "timestamp": "2026-08-15T12:00:00Z",
            "emitter_agent": "execute-process",
            "payload": {
                "correlation_id": cid,
                "process_name": "feature",
                "status": "success",
                "cycle_phase": "completed"
            }
        });
        let ev_path = orch.join(format!("{eid}.json"));
        fs::write(&ev_path, serde_json::to_string_pretty(&ev).unwrap()).unwrap();
        let result = super::super::route_fractal_core::route_orchestration_event(
            repo,
            ev_path.to_str().unwrap(),
        );
        assert_eq!(result["success"], true, "{result}");
        assert_eq!(result["data"]["purged"], true, "{result}");
        assert!(!ev_path.is_file(), "parent must be purged");
        assert!(proof_path(repo, cid).is_file(), "proof must survive purge");
    }
}
