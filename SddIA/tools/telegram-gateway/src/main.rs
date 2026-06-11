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

fn main() {
    let req = read_stdin_json();

    let text = match req.get("text").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            let out = json!({
                "success": false,
                "exitCode": 1,
                "error": "text obligatorio",
                "emitted": false
            });
            emit_success(Some(out));
            return;
        }
    };

    let event = transmute_text(text);
    if let Some(ev) = event {
        let event_type = ev.get("event_type").cloned();
        let out = json!({
            "success": true,
            "emitted": true,
            "event": ev,
            "event_type": event_type
        });
        emit_success(Some(out));
    } else {
        let out = json!({
            "success": true,
            "emitted": false,
            "event": serde_json::Value::Null
        });
        emit_success(Some(out));
    }
}
