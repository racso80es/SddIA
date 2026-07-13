use sddia_io::outbound_lab::{lab_mock_outbound_enabled, lab_mock_telegram_url, truthy_env};
use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::env;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
fn main() {
    let _req = read_stdin_json();
    emit_error(
        "WASI environment cannot establish outbound HTTP connections directly without host capabilities.",
        1,
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let req = read_stdin_json();
    match run(&req) {
        Ok(result) => emit_success(Some(result)),
        Err(msg) => emit_error(&msg, 1),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run(req: &Value) -> Result<Value, String> {
    let message = req
        .get("message")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "message obligatorio".to_string())?;

    let parse_mode = req
        .get("parse_mode")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.eq_ignore_ascii_case("null"));

    if lab_mock_outbound_enabled() {
        return Ok(mock_success(message, parse_mode, "lab-mock-outbound"));
    }

    if let Some(mock_url) = lab_mock_telegram_url() {
        return post_mock(&mock_url, message, parse_mode);
    }

    let token = env::var("TELEGRAM_BOT_TOKEN")
        .unwrap_or_default()
        .trim()
        .to_string();
    let chat_id = env::var("TELEGRAM_ALLOWED_CHAT_ID")
        .unwrap_or_default()
        .trim()
        .to_string();

    if token.is_empty() || chat_id.is_empty() {
        if truthy_env("SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM") {
            return Ok(mock_success(message, parse_mode, "skipped-lab-no-credentials"));
        }
        return Err("config-missing: TELEGRAM_BOT_TOKEN or TELEGRAM_ALLOWED_CHAT_ID".into());
    }

    if token.eq_ignore_ascii_case("lab-mock") {
        return Ok(mock_success(message, parse_mode, "lab-mock-token"));
    }

    send_telegram(&token, &chat_id, message, parse_mode)
}

#[cfg(not(target_arch = "wasm32"))]
fn mock_success(message: &str, parse_mode: Option<&str>, mode: &str) -> Value {
    json!({
        "message_id": format!("mock-{}", &message.len()),
        "attempt": 1,
        "degraded_plain_fallback": false,
        "parse_mode_requested": parse_mode,
        "mode": mode,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn post_mock(mock_url: &str, message: &str, parse_mode: Option<&str>) -> Result<Value, String> {
    let agent = ureq::agent();
    let payload = json!({
        "message": message,
        "parse_mode": parse_mode,
    });
    let resp = agent
        .post(mock_url)
        .set("Content-Type", "application/json")
        .timeout(Duration::from_secs(10))
        .send_string(&payload.to_string())
        .map_err(|e| format!("mock-telegram-unreachable: {e}"))?;
    let body: Value = resp
        .into_json()
        .map_err(|e| format!("mock-telegram-invalid-json: {e}"))?;
    if body.get("success").and_then(|v| v.as_bool()) != Some(true) {
        let err = body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("mock-telegram-failed");
        return Err(err.to_string());
    }
    Ok(body.get("result").cloned().unwrap_or_else(|| {
        json!({
            "message_id": "mock-remote",
            "attempt": 1,
            "degraded_plain_fallback": false,
            "parse_mode_requested": parse_mode,
        })
    }))
}

#[cfg(not(target_arch = "wasm32"))]
fn send_telegram(
    token: &str,
    chat_id: &str,
    message: &str,
    parse_mode: Option<&str>,
) -> Result<Value, String> {
    let url = format!("https://api.telegram.org/bot{token}/sendMessage");
    let agent = ureq::agent();
    let (message_id, attempt, degraded) =
        try_send(&agent, &url, chat_id, message, parse_mode).or_else(|first_err| {
            if parse_mode.is_some() && first_err.contains("400") {
                let (id, _, _) = try_send(&agent, &url, chat_id, message, None)?;
                return Ok((id, 2, true));
            }
            Err(first_err)
        })?;
    Ok(json!({
        "message_id": message_id,
        "attempt": attempt,
        "degraded_plain_fallback": degraded,
        "parse_mode_requested": parse_mode,
    }))
}

#[cfg(not(target_arch = "wasm32"))]
fn try_send(
    agent: &ureq::Agent,
    url: &str,
    chat_id: &str,
    message: &str,
    parse_mode: Option<&str>,
) -> Result<(String, u32, bool), String> {
    let mut form: Vec<(&str, &str)> = vec![("chat_id", chat_id), ("text", message)];
    if let Some(pm) = parse_mode {
        form.push(("parse_mode", pm));
    }
    let resp = agent
        .post(url)
        .timeout(Duration::from_secs(20))
        .send_form(&form)
        .map_err(|e| format!("telegram-http-failed: {e}"))?;
    let status = resp.status();
    let body: Value = resp
        .into_json()
        .map_err(|e| format!("telegram-invalid-json: {e}"))?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        let desc = body
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("telegram-api-error");
        return Err(format!("telegram-api-rejected ({status}): {desc}"));
    }
    let message_id = body
        .get("result")
        .and_then(|r| r.get("message_id"))
        .map(|v| v.to_string())
        .unwrap_or_else(|| "unknown".into());
    Ok((message_id, 1, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_message() {
        let err = run(&json!({})).unwrap_err();
        assert!(err.contains("message obligatorio"));
    }

    #[test]
    fn lab_mock_outbound_returns_success_shape() {
        env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        let out = run(&json!({"message": "hola"})).expect("mock success");
        assert!(out.get("message_id").is_some());
        env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
    }
}
