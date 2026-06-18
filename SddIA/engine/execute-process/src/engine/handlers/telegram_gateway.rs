//! Handler nativo `telegram-gateway` (P4).

use super::super::capsules::invoke_tool;
use super::super::fractal::{load_fractal_dirs, write_fractal_event};
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use regex::Regex;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::LazyLock;
use uuid::Uuid;

static TODO_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^\s*TODO:\s*(.+)$").unwrap());
static IDEA_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^\s*IDEA:\s*(.+)$").unwrap());

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn transmute_text(text: &str) -> Option<Value> {
    let stripped = text.trim();
    if stripped.is_empty() {
        return None;
    }
    let event_id = Uuid::new_v4().to_string();
    let now = iso_now();
    if let Some(caps) = TODO_RE
        .captures(stripped)
        .or_else(|| IDEA_RE.captures(stripped))
    {
        let idea = caps.get(1)?.as_str().trim();
        return Some(json!({
            "event_id": event_id,
            "event_type": "Kaizen_Idea_Captured",
            "event_family": "domain",
            "timestamp": now,
            "emitter_agent": "telegram-gateway",
            "payload": {
                "idea_text": idea,
                "source": "telegram",
                "raw_text": text,
            },
            "delivery_state": {},
        }));
    }
    Some(json!({
        "event_id": event_id,
        "event_type": "Manual_Task_Requested",
        "event_family": "domain",
        "timestamp": now,
        "emitter_agent": "telegram-gateway",
        "payload": {
            "task_text": stripped,
            "source": "telegram",
            "raw_text": text,
        },
        "delivery_state": {},
    }))
}

fn build_telegram_message_received(text: &str, chat_id: &str) -> Value {
    json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "TelegramMessage_Received",
        "event_family": "domain",
        "timestamp": iso_now(),
        "emitter_agent": "telegram-gateway",
        "payload": {
            "text": text,
            "chat_id": chat_id,
            "source": "telegram",
            "raw_text": text,
        },
        "delivery_state": {},
    })
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let text = process_inputs
        .get("text")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "text requerido".to_string())?;

    let capsule = invoke_tool(repo, "telegram-gateway", &json!({"text": text}))?;
    if capsule.get("success").and_then(|v| v.as_bool()) != Some(true) {
        return Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            data: Some(json!({
                "ok": false,
                "emitted": false,
                "error": capsule.get("error").cloned().unwrap_or(json!("tool failed")),
            })),
            error: capsule
                .get("error")
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .or_else(|| Some("tool failed".into())),
            execution_report: Some(json!({
                "process_name": "telegram-gateway",
                "phases": [{
                    "phase_name": "Transmutación e inyección",
                    "status": "failed",
                    "handler": "telegram-gateway-core",
                    "emitted": false,
                }],
            })),
            exit_code: 1,
        });
    }

    if capsule.get("emitted").and_then(|v| v.as_bool()) != Some(true) {
        return Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({
                "ok": true,
                "emitted": false,
                "reason": capsule.get("reason").cloned().unwrap_or(json!("empty_text")),
            })),
            error: None,
            execution_report: Some(json!({
                "process_name": "telegram-gateway",
                "phases": [{
                    "phase_name": "Transmutación e inyección",
                    "status": "executed",
                    "handler": "telegram-gateway-core",
                    "emitted": false,
                }],
            })),
            exit_code: 0,
        });
    }

    let event = capsule
        .get("event")
        .cloned()
        .or_else(|| transmute_text(text));
    let Some(event) = event else {
        return Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({"ok": true, "emitted": false, "reason": "empty_text"})),
            error: None,
            execution_report: Some(json!({
                "process_name": "telegram-gateway",
                "phases": [{
                    "phase_name": "Transmutación e inyección",
                    "status": "executed",
                    "handler": "telegram-gateway-core",
                    "emitted": false,
                }],
            })),
            exit_code: 0,
        });
    };

    let (_, _, domain_dir) = load_fractal_dirs(repo);
    let chat_id = std::env::var("TELEGRAM_ALLOWED_CHAT_ID")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let sensorial_seal = if let Some(ref cid) = chat_id {
        let sensorial = build_telegram_message_received(text, cid);
        write_fractal_event(repo, &sensorial, &domain_dir).ok()
    } else {
        None
    };

    let seal = write_fractal_event(repo, &event, &domain_dir)?;
    let event_type = event.get("event_type").cloned();
    let event_id = event.get("event_id").and_then(|v| v.as_str());

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "ok": true,
            "emitted": true,
            "event_type": event_type,
            "event_id": event_id,
            "telegram_message_received_id": sensorial_seal.as_ref().and_then(|s| s.get("event_id").cloned()),
            "seal": seal,
            "sensorial_seal": sensorial_seal,
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": "telegram-gateway",
            "phases": [{
                "phase_name": "Transmutación e inyección",
                "status": "executed",
                "handler": "telegram-gateway-core",
                "emitted": true,
                "event_type": event_type,
            }],
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmute_todo_to_kaizen() {
        let ev = transmute_text("TODO: golden lab").unwrap();
        assert_eq!(
            ev.get("event_type").and_then(|v| v.as_str()),
            Some("Kaizen_Idea_Captured")
        );
    }
}
