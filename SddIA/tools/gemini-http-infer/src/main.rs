use sddia_io::outbound_lab::{lab_mock_gemini_url, lab_mock_outbound_enabled};
use sddia_io::read_stdin_json;
use serde_json::{json, Value};
use std::env;
use std::process;
use std::time::{Duration, Instant};

const ENTITY_ID: &str = "gemini-http-infer";
const DEFAULT_BASE: &str = "https://generativelanguage.googleapis.com";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const MAX_TIMEOUT_SECS: u64 = 300;

fn emit_v2(success: bool, exit_code: i32, message: &str, result: Option<Value>, feedback: Option<&str>) -> ! {
    let mut body = json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": ENTITY_ID,
        },
        "success": success,
        "exitCode": exit_code,
        "message": message,
    });
    if let Some(r) = result {
        body["result"] = r;
    }
    if let Some(fb) = feedback {
        body["feedback"] = json!(fb);
        body["error"] = json!(fb);
    }
    println!("{body}");
    process::exit(exit_code);
}

fn request_inner(doc: &Value) -> &Value {
    doc.get("request").unwrap_or(doc)
}

fn required_str(req: &Value, key: &str) -> Result<String, String> {
    req.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("request.{key} obligatorio"))
}

fn timeout_secs() -> u64 {
    env::var("SDDIA_GEMINI_HTTP_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_TIMEOUT_SECS)
        .min(MAX_TIMEOUT_SECS)
}

fn api_base() -> String {
    env::var("SDDIA_GEMINI_API_BASE_URL")
        .ok()
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_BASE.to_string())
}

fn generate_url(base: &str, model: &str) -> String {
    format!("{}/v1beta/models/{model}:generateContent", base.trim_end_matches('/'))
}

fn extract_text(body: &Value) -> Option<String> {
    body.pointer("/candidates/0/content/parts/0/text")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn mock_result(prompt: &str, model: &str) -> Value {
    json!({
        "text": format!("lab-mock:{model}:{}", prompt.chars().take(80).collect::<String>()),
        "raw_response": {
            "mode": "lab-mock-outbound",
            "model": model
        }
    })
}

fn post_generate(url: &str, api_key: Option<&str>, prompt: &str, model: &str, temperature: Option<f64>) -> Result<Value, String> {
    let mut payload = json!({
        "contents": [{"parts": [{"text": prompt}]}]
    });
    if let Some(t) = temperature {
        payload["generationConfig"] = json!({ "temperature": t });
    }
    let mut req = ureq::post(url)
        .set("Content-Type", "application/json")
        .timeout(Duration::from_secs(timeout_secs()));
    if let Some(key) = api_key {
        req = req.set("x-goog-api-key", key);
    }
    let resp = req
        .send_string(&payload.to_string())
        .map_err(|e| format!("http-post-failed: {e}"))?;
    let status = resp.status();
    let body: Value = resp
        .into_json()
        .map_err(|e| format!("http-body-invalid-json: {e}"))?;
    if status >= 400 {
        return Err(format!("http-status-{status}: {body}"));
    }
    let text = extract_text(&body).unwrap_or_default();
    Ok(json!({
        "text": text,
        "raw_response": body,
        "model": model
    }))
}

fn run(doc: &Value) -> Result<Value, String> {
    let req = request_inner(doc);
    let prompt = required_str(req, "prompt")?;
    let model = required_str(req, "model")?;
    let temperature = req.get("temperature").and_then(|v| v.as_f64());

    if lab_mock_outbound_enabled() && lab_mock_gemini_url().is_none() {
        return Ok(mock_result(&prompt, &model));
    }

    if let Some(mock_url) = lab_mock_gemini_url() {
        return post_generate(&mock_url, None, &prompt, &model, temperature);
    }

    let api_key = env::var("GEMINI_API_KEY")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "GEMINI_API_KEY ausente".to_string())?;

    let url = generate_url(&api_base(), &model);
    post_generate(&url, Some(&api_key), &prompt, &model, temperature)
}

fn main() {
    let started = Instant::now();
    let doc = read_stdin_json();
    match run(&doc) {
        Ok(mut result) => {
            result["durationMs"] = json!(started.elapsed().as_millis() as u64);
            emit_v2(true, 0, "ok", Some(result), None);
        }
        Err(msg) => emit_v2(false, 1, "infer-failed", None, Some(&msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_url_joins_model() {
        assert_eq!(
            generate_url("https://generativelanguage.googleapis.com", "gemini-flash"),
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash:generateContent"
        );
    }

    #[test]
    fn extract_text_from_candidate() {
        let body = json!({
            "candidates": [{"content": {"parts": [{"text": "  hola  "}]}}]
        });
        assert_eq!(extract_text(&body).as_deref(), Some("hola"));
    }

    #[test]
    fn required_fields_reject_empty_model() {
        let req = json!({"prompt": "x", "model": "  "});
        assert!(required_str(&req, "model").is_err());
    }

    #[test]
    fn mock_result_prefixes_lab() {
        let v = mock_result("abc", "m1");
        assert!(v["text"].as_str().unwrap().starts_with("lab-mock:m1:"));
    }
}
