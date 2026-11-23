//! Escalado reactivo e idempotente DLQ → `Kaizen_Alert_Required` (AUT-117).

use crate::engine::eda_port;
use chrono::Utc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const ALERTS_STATE_REL: &str = ".SddIA/state/dlq-kaizen-alerts.json";

/// Calcula firma determinista `sha256(error_fingerprint | subscriber_id | event_type)`.
pub fn dlq_signature_hash(error_fingerprint: &str, subscriber_id: &str, event_type: &str) -> String {
    let canonical = format!(
        "{}|{}|{}",
        error_fingerprint.trim(),
        subscriber_id.trim(),
        event_type.trim()
    );
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub trait DlqAlertSink {
    fn already_alerted(&self, repo: &Path, signature: &str) -> bool;
    fn mark_alerted(&self, repo: &Path, signature: &str, meta: &Value) -> Result<(), String>;
    fn emit_kaizen_alert(&self, repo: &Path, event: &Value) -> Result<(), String>;
}

pub struct ProdDlqAlertSink;

fn alerts_state_path(repo: &Path) -> PathBuf {
    repo.join(ALERTS_STATE_REL)
}

impl DlqAlertSink for ProdDlqAlertSink {
    fn already_alerted(&self, repo: &Path, signature: &str) -> bool {
        let p = alerts_state_path(repo);
        let Ok(raw) = fs::read_to_string(&p) else {
            return false;
        };
        let Ok(val) = serde_json::from_str::<Value>(&raw) else {
            return false;
        };
        val.get("signatures")
            .and_then(|v| v.as_object())
            .is_some_and(|m| m.contains_key(signature))
    }

    fn mark_alerted(&self, repo: &Path, signature: &str, meta: &Value) -> Result<(), String> {
        let p = alerts_state_path(repo);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut signatures: BTreeMap<String, Value> = fs::read_to_string(&p)
            .ok()
            .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
            .and_then(|v| v.get("signatures").and_then(|s| s.as_object()).cloned())
            .map(|m| m.into_iter().collect())
            .unwrap_or_default();
        signatures.insert(signature.to_string(), meta.clone());
        let body = json!({
            "schema": "dlq-kaizen-alerts/1.0",
            "signatures": signatures,
        });
        fs::write(&p, serde_json::to_string_pretty(&body).unwrap_or_default())
            .map_err(|e| e.to_string())
    }

    fn emit_kaizen_alert(&self, repo: &Path, event: &Value) -> Result<(), String> {
        let mut stamped = event.clone();
        sddia_eda_catalog::stamp_published_event(repo, &mut stamped)?;
        eda_port::publish_ecst(repo, "domain", &stamped).map(|_| ())
    }
}

/// Emite `Kaizen_Alert_Required` exactamente una vez por firma `(error, subscriber_id, event_type)`.
pub fn notify_dlq_failure_with_sink(
    repo: &Path,
    event_uuid: &str,
    event_type: &str,
    subscriber_id: &str,
    error_trace: &str,
    sink: &dyn DlqAlertSink,
) -> Result<bool, String> {
    if event_type.trim() == "Kaizen_Alert_Required"
        || subscriber_id.contains("materialize-kaizen-alert")
    {
        return Ok(false);
    }
    let sig = dlq_signature_hash(error_trace, subscriber_id, event_type);
    if sink.already_alerted(repo, &sig) {
        return Ok(false);
    }
    let review_id = format!("dlq-{}", &sig[..16.min(sig.len())]);
    let kaizen_ev = json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "Kaizen_Alert_Required",
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": "route-domain-event",
        "correlation_id": event_uuid,
        "payload": {
            "review_id": review_id,
            "alert_kind": "dlq-subscriber-failure",
            "alert_justification": format!(
                "Fallo determinista / transición a DLQ en suscriptor '{subscriber_id}' para '{event_type}': {error_trace}"
            ),
            "implicated_files": ["SddIA/core/event-domain-subscriptions.json"],
            "dlq_signature": sig,
            "subscriber_id": subscriber_id,
            "source_event_type": event_type,
            "source_event_id": event_uuid,
        },
        "delivery_state": {}
    });
    sink.emit_kaizen_alert(repo, &kaizen_ev)?;
    sink.mark_alerted(
        repo,
        &sig,
        &json!({
            "subscriber_id": subscriber_id,
            "event_type": event_type,
            "error_trace": error_trace.chars().take(240).collect::<String>(),
            "first_event_id": event_uuid,
            "alerted_at": Utc::now().to_rfc3339(),
        }),
    )?;
    Ok(true)
}

pub fn notify_dlq_failure_fail_soft(
    repo: &Path,
    event_uuid: &str,
    event_type: &str,
    subscriber_id: &str,
    error_trace: &str,
) {
    if let Err(e) = notify_dlq_failure_with_sink(
        repo,
        event_uuid,
        event_type,
        subscriber_id,
        error_trace,
        &ProdDlqAlertSink,
    ) {
        eprintln!(
            "[dlq-kaizen-alert] error emitiendo Kaizen_Alert_Required para {subscriber_id}/{event_type}: {e}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Mutex;

    struct InMemoryDlqSink {
        seen: Mutex<HashSet<String>>,
        emitted: Mutex<Vec<Value>>,
    }

    impl InMemoryDlqSink {
        fn new() -> Self {
            Self {
                seen: Mutex::new(HashSet::new()),
                emitted: Mutex::new(Vec::new()),
            }
        }
    }

    impl DlqAlertSink for InMemoryDlqSink {
        fn already_alerted(&self, _repo: &Path, signature: &str) -> bool {
            self.seen.lock().unwrap().contains(signature)
        }
        fn mark_alerted(&self, _repo: &Path, signature: &str, _meta: &Value) -> Result<(), String> {
            self.seen.lock().unwrap().insert(signature.to_string());
            Ok(())
        }
        fn emit_kaizen_alert(&self, _repo: &Path, event: &Value) -> Result<(), String> {
            self.emitted.lock().unwrap().push(event.clone());
            Ok(())
        }
    }

    #[test]
    fn aut117_dlq_failure_emits_kaizen_alert_idempotently() {
        let tmp = tempfile::tempdir().unwrap();
        let sink = InMemoryDlqSink::new();
        let err = "capsule missing";
        assert_eq!(
            notify_dlq_failure_with_sink(
                tmp.path(),
                "11111111-1111-4111-8111-111111111111",
                "PullRequest_Merged",
                "argos.notify-humanized-pr-merged",
                err,
                &sink
            )
            .unwrap(),
            true
        );
        assert_eq!(
            notify_dlq_failure_with_sink(
                tmp.path(),
                "22222222-2222-4222-8222-222222222222",
                "PullRequest_Merged",
                "argos.notify-humanized-pr-merged",
                err,
                &sink
            )
            .unwrap(),
            false
        );
        assert_eq!(sink.emitted.lock().unwrap().len(), 1);
    }

    #[test]
    fn aut117_distinct_dlq_signatures_emit_independent_alerts() {
        let tmp = tempfile::tempdir().unwrap();
        let sink = InMemoryDlqSink::new();
        assert!(notify_dlq_failure_with_sink(
            tmp.path(),
            "11111111-1111-4111-8111-111111111111",
            "PullRequest_Merged",
            "argos.notify-humanized-pr-merged",
            "err A",
            &sink
        )
        .unwrap());
        assert!(notify_dlq_failure_with_sink(
            tmp.path(),
            "22222222-2222-4222-8222-222222222222",
            "PullRequest_Presented",
            "argos.send-telegram-notification",
            "err B",
            &sink
        )
        .unwrap());
        assert_eq!(sink.emitted.lock().unwrap().len(), 2);
    }

    #[test]
    fn aut117_kaizen_alert_failure_does_not_recurse() {
        let tmp = tempfile::tempdir().unwrap();
        let sink = InMemoryDlqSink::new();
        assert_eq!(
            notify_dlq_failure_with_sink(
                tmp.path(),
                "33333333-3333-4333-8333-333333333333",
                "Kaizen_Alert_Required",
                "cumulo.materialize-kaizen-alert-doc",
                "boom",
                &sink
            )
            .unwrap(),
            false
        );
        assert_eq!(sink.emitted.lock().unwrap().len(), 0);
    }
}
