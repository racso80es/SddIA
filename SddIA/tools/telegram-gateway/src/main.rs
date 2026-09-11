use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::json;
use chrono::Utc;
use uuid::Uuid;
use regex::Regex;

fn iso_now() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn transmute_text(text: &str) -> Option<serde_json::Value> {
    let stripped = text.trim();
    if stripped.is_empty() {
        return None;
    }

    let todo_re = Regex::new(r"(?i)^\s*TODO:\s*(.+)$").unwrap();
    let idea_re = Regex::new(r"(?i)^\s*IDEA:\s*(.+)$").unwrap();

    let emitter = "telegram-gateway";
    let now = iso_now();
    let event_id = Uuid::new_v4().to_string();

    if let Some(caps) = todo_re.captures(stripped).or_else(|| idea_re.captures(stripped)) {
        let idea_text = caps.get(1).unwrap().as_str().trim();
        return Some(json!({
            "event_id": event_id,
            "event_type": "Kaizen_Idea_Captured",
            "event_family": "domain",
            "timestamp": now,
            "emitter_agent": emitter,
            "payload": {
                "idea_text": idea_text,
                "source": "telegram",
                "raw_text": text
            },
            "delivery_state": {}
        }));
    }

    Some(json!({
        "event_id": event_id,
        "event_type": "Manual_Task_Requested",
        "event_family": "domain",
        "timestamp": now,
        "emitter_agent": emitter,
        "payload": {
            "task_text": stripped,
            "source": "telegram",
            "raw_text": text
        },
        "delivery_state": {}
    }))
}

fn transmute_callback(req: &serde_json::Value) -> Result<serde_json::Value, String> {
    let data = req
        .get("callback_data")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "callback_data obligatorio".to_string())?;
    if data.len() > 64 {
        return Err("callback_data excede 64 bytes".into());
    }
    let chat_id = req
        .get("chat_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    let mut payload = json!({
        "callback_data": data,
        "chat_id": chat_id,
        "source": "telegram",
    });
    if let Some(mid) = req.get("message_id").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        payload["message_id"] = json!(mid);
    }
    Ok(json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "TelegramCallback_Received",
        "event_family": "domain",
        "timestamp": iso_now(),
        "emitter_agent": "telegram-gateway",
        "payload": payload,
        "delivery_state": {}
    }))
}

fn main() {
    let req = read_stdin_json();

    let text = req.get("text").and_then(|v| v.as_str());
    let callback = req.get("callback_data").and_then(|v| v.as_str());
    let has_text = text.map(|s| !s.trim().is_empty()).unwrap_or(false);
    let has_cb = callback.map(|s| !s.trim().is_empty()).unwrap_or(false);

    match (has_text, has_cb) {
        (true, true) | (false, false) => {
            emit_error("text y callback_data son XOR", 1);
            return;
        }
        (false, true) => match transmute_callback(&req) {
            Ok(ev) => {
                let event_type = ev.get("event_type").cloned();
                emit_success(Some(json!({
                    "success": true,
                    "emitted": true,
                    "event": ev,
                    "event_type": event_type
                })));
            }
            Err(e) => emit_error(&e, 1),
        },
        (true, false) => {
            let text = text.unwrap();
            let event = transmute_text(text);
            if let Some(ev) = event {
                let event_type = ev.get("event_type").cloned();
                emit_success(Some(json!({
                    "success": true,
                    "emitted": true,
                    "event": ev,
                    "event_type": event_type
                })));
            } else {
                emit_success(Some(json!({
                    "success": true,
                    "emitted": false,
                    "event": serde_json::Value::Null
                })));
            }
        }
    }
}
