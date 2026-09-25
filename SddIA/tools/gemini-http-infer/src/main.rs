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

fn resolve_model(req: &Value) -> Result<String, String> {
    if let Ok(m) = required_str(req, "model") {
        return Ok(m);
    }
    env::var("SDDIA_GEMINI_MODEL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "request.model o SDDIA_GEMINI_MODEL obligatorio".to_string())
}

fn normalize_thinking_level(raw: &str) -> Result<String, String> {
    match raw.trim().to_ascii_uppercase().as_str() {
        "HIGH" | "MEDIUM" | "LOW" => Ok(raw.trim().to_ascii_uppercase()),
        _ => Err(format!("thinking-level-invalid: {raw}")),
    }
}

fn resolve_thinking_level(req: &Value) -> Result<Option<String>, String> {
    if let Ok(raw) = required_str(req, "thinking_level") {
        return normalize_thinking_level(&raw).map(Some);
    }
    if let Ok(raw) = required_str(req, "thinkingLevel") {
        return normalize_thinking_level(&raw).map(Some);
    }
    if let Some(effort) = optional_str(req, "effort") {
        return normalize_thinking_level(&effort).map(Some);
    }
    match env::var("SDDIA_GEMINI_THINKING_LEVEL") {
        Ok(v) => {
            let t = v.trim();
            if t.is_empty() {
                Ok(None)
            } else {
                normalize_thinking_level(t).map(Some)
            }
        }
        Err(_) => Ok(None),
    }
}

fn generation_config(temperature: Option<f64>, thinking_level: Option<&str>) -> Option<Value> {
    if temperature.is_none() && thinking_level.is_none() {
        return None;
    }
    let mut cfg = serde_json::Map::new();
    if let Some(t) = temperature {
        cfg.insert("temperature".into(), json!(t));
    }
    if let Some(level) = thinking_level {
        cfg.insert(
            "thinkingConfig".into(),
            json!({ "thinkingLevel": level }),
        );
    }
    Some(Value::Object(cfg))
}

fn generate_content_payload(
    prompt: &str,
    system_prompt: Option<&str>,
    temperature: Option<f64>,
    thinking_level: Option<&str>,
) -> Value {
    let mut payload = json!({
        "contents": [{"parts": [{"text": prompt}]}]
    });
    if let Some(sys) = system_prompt.filter(|s| !s.is_empty()) {
        payload["systemInstruction"] = json!({"parts": [{"text": sys}]});
    }
    if let Some(cfg) = generation_config(temperature, thinking_level) {
        payload["generationConfig"] = cfg;
    }
    payload
}

fn timeout_secs_from(req: &Value) -> u64 {
    if let Some(ms) = req.get("timeout_ms").and_then(|v| v.as_u64()) {
        return (ms / 1000).max(1).min(MAX_TIMEOUT_SECS);
    }
    timeout_secs()
}

fn optional_str(req: &Value, key: &str) -> Option<String> {
    req.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn classify_error_code(msg: &str) -> &'static str {
    let l = msg.to_lowercase();
    if l.contains("http-status-429") || l.contains("resource_exhausted") || l.contains("quota") {
        return "rate_limited";
    }
    if l.contains("http-status-401") || l.contains("http-status-403") || l.contains("gemini_api_key ausente") {
        return "auth";
    }
    if l.contains("http-status-503")
        || l.contains("http-status-502")
        || l.contains("http-status-500")
        || l.contains("http-status-504")
        || l.contains("gemini-model-unavailable")
    {
        return "upstream_unavailable";
    }
    if l.contains("timed out") || l.contains("timeout") {
        return "timeout";
    }
    if l.contains("http-post-failed")
        || l.contains("connection")
        || l.contains("transport")
        || l.contains("network")
    {
        return "network";
    }
    if l.contains("empty-candidate") || l.contains("invalid-json") || l.contains("body-not-json") {
        return "malformed_response";
    }
    "unknown"
}

fn usage_from_body(body: &Value) -> (Option<i64>, Option<i64>) {
    let prompt = body
        .pointer("/usageMetadata/promptTokenCount")
        .and_then(|v| v.as_i64());
    let completion = body
        .pointer("/usageMetadata/candidatesTokenCount")
        .and_then(|v| v.as_i64());
    (prompt, completion)
}

fn telemetry_receipt(
    model: &str,
    latency_ms: u64,
    prompt_tokens: Option<i64>,
    completion_tokens: Option<i64>,
) -> Value {
    let mut rec = json!({
        "provider": ENTITY_ID,
        "llm_model": model,
        "provider_latency_ms": latency_ms,
    });
    if let Some(p) = prompt_tokens {
        rec["prompt_tokens"] = json!(p);
    }
    if let Some(c) = completion_tokens {
        rec["completion_tokens"] = json!(c);
    }
    rec
}

fn extract_text(body: &Value) -> Option<String> {
    body.pointer("/candidates/0/content/parts/0/text")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn finish_reason(body: &Value) -> &str {
    body.pointer("/candidates/0/finishReason")
        .and_then(|v| v.as_str())
        .unwrap_or("")
}

fn google_error_message(body: &Value) -> String {
    body.pointer("/error/message")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("")
        .to_string()
}

fn map_http_error_body(code: u16, body: &Value) -> String {
    let msg = google_error_message(body);
    let blob = body.to_string();
    let catalog = msg.to_lowercase().contains("no longer available")
        || (code == 404
            && (blob.contains("NOT_FOUND") || msg.to_lowercase().contains("not found")));
    if catalog && !msg.is_empty() {
        format!("gemini-model-unavailable: {msg}")
    } else if catalog {
        format!("gemini-model-unavailable: http-status-{code}: {body}")
    } else {
        format!("http-status-{code}: {body}")
    }
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

fn post_generate(
    url: &str,
    api_key: Option<&str>,
    prompt: &str,
    model: &str,
    temperature: Option<f64>,
    thinking_level: Option<&str>,
    timeout: u64,
    system_prompt: Option<&str>,
) -> Result<Value, String> {
    let payload = generate_content_payload(prompt, system_prompt, temperature, thinking_level);
    let mut req = ureq::post(url)
        .set("Content-Type", "application/json")
        .timeout(Duration::from_secs(timeout));
    if let Some(key) = api_key {
        req = req.set("x-goog-api-key", key);
    }
    let resp = match req.send_string(&payload.to_string()) {
        Ok(r) => r,
        Err(ureq::Error::Status(code, resp)) => {
            let body: Value = resp
                .into_json()
                .unwrap_or_else(|_| json!({"error": {"message": "body-not-json"}}));
            return Err(map_http_error_body(code, &body));
        }
        Err(e) => return Err(format!("http-post-failed: {e}")),
    };
    let body: Value = resp
        .into_json()
        .map_err(|e| format!("http-body-invalid-json: {e}"))?;
    let text = extract_text(&body).unwrap_or_default();
    if text.is_empty() {
        return Err(format!(
            "gemini-empty-candidate: finishReason={}",
            finish_reason(&body)
        ));
    }
    let (pt, ct) = usage_from_body(&body);
    Ok(json!({
        "text": text,
        "raw_response": body,
        "model": model,
        "telemetry_receipt": telemetry_receipt(model, 0, pt, ct)
    }))
}

fn run(doc: &Value) -> Result<Value, String> {
    let req = request_inner(doc);
    let prompt = required_str(req, "prompt")?;
    let model = resolve_model(req)?;
    let temperature = req.get("temperature").and_then(|v| v.as_f64());
    let thinking_level = resolve_thinking_level(req)?;
    let timeout = timeout_secs_from(req);
    let system_prompt = optional_str(req, "system_prompt");

    if lab_mock_outbound_enabled() && lab_mock_gemini_url().is_none() {
        let mut mock = mock_result(&prompt, &model);
        mock["telemetry_receipt"] = telemetry_receipt(&model, 0, None, None);
        return Ok(mock);
    }

    if let Some(mock_url) = lab_mock_gemini_url() {
        return post_generate(
            &mock_url,
            None,
            &prompt,
            &model,
            temperature,
            thinking_level.as_deref(),
            timeout,
            system_prompt.as_deref(),
        );
    }

    let api_key = env::var("GEMINI_API_KEY")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "GEMINI_API_KEY ausente".to_string())?;

    let url = generate_url(&api_base(), &model);
    post_generate(
        &url,
        Some(&api_key),
        &prompt,
        &model,
        temperature,
        thinking_level.as_deref(),
        timeout,
        system_prompt.as_deref(),
    )
}

fn main() {
    let started = Instant::now();
    let doc = read_stdin_json();
    let model = request_inner(&doc)
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    match run(&doc) {
        Ok(mut result) => {
            let ms = started.elapsed().as_millis() as u64;
            result["durationMs"] = json!(ms);
            if result.get("telemetry_receipt").is_none() {
                result["telemetry_receipt"] = telemetry_receipt(&model, ms, None, None);
            } else {
                result["telemetry_receipt"]["provider_latency_ms"] = json!(ms);
            }
            emit_v2(true, 0, "ok", Some(result), None);
        }
        Err(msg) => {
            let ms = started.elapsed().as_millis() as u64;
            let fail = json!({
                "error_code": classify_error_code(&msg),
                "telemetry_receipt": telemetry_receipt(&model, ms, None, None),
            });
            emit_v2(false, 1, "infer-failed", Some(fail), Some(&msg));
        }
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
    fn map_404_catalog_prefixes_unavailable() {
        let body = json!({
            "error": {
                "code": 404,
                "message": "This model models/gemini-2.5-flash is no longer available to new users.",
                "status": "NOT_FOUND"
            }
        });
        let mapped = map_http_error_body(404, &body);
        assert!(mapped.starts_with("gemini-model-unavailable:"));
        assert!(mapped.contains("no longer available"));
    }

    #[test]
    fn empty_candidate_uses_finish_reason() {
        let body = json!({"candidates": [{"finishReason": "MAX_TOKENS", "content": {}}]});
        assert_eq!(finish_reason(&body), "MAX_TOKENS");
        assert!(extract_text(&body).is_none());
    }

    #[test]
    fn mock_result_prefixes_lab() {
        let v = mock_result("abc", "m1");
        assert!(v["text"].as_str().unwrap().starts_with("lab-mock:m1:"));
    }

    fn with_cleared_thinking_env<T>(f: impl FnOnce() -> T) -> T {
        let prev = std::env::var("SDDIA_GEMINI_THINKING_LEVEL").ok();
        std::env::remove_var("SDDIA_GEMINI_THINKING_LEVEL");
        let out = f();
        match prev {
            Some(v) => std::env::set_var("SDDIA_GEMINI_THINKING_LEVEL", v),
            None => std::env::remove_var("SDDIA_GEMINI_THINKING_LEVEL"),
        }
        out
    }

    #[test]
    fn thinking_level_empty_is_none() {
        with_cleared_thinking_env(|| {
            let req = json!({"prompt": "x", "model": "m"});
            assert_eq!(resolve_thinking_level(&req).unwrap(), None);
        });
    }

    #[test]
    fn thinking_level_from_env() {
        with_cleared_thinking_env(|| {
            std::env::set_var("SDDIA_GEMINI_THINKING_LEVEL", "high");
            let req = json!({"prompt": "x", "model": "m"});
            assert_eq!(resolve_thinking_level(&req).unwrap().as_deref(), Some("HIGH"));
            std::env::remove_var("SDDIA_GEMINI_THINKING_LEVEL");
        });
    }

    #[test]
    fn thinking_level_request_beats_env() {
        with_cleared_thinking_env(|| {
            std::env::set_var("SDDIA_GEMINI_THINKING_LEVEL", "low");
            let req = json!({"prompt": "x", "model": "m", "thinking_level": "high"});
            assert_eq!(resolve_thinking_level(&req).unwrap().as_deref(), Some("HIGH"));
            std::env::remove_var("SDDIA_GEMINI_THINKING_LEVEL");
        });
    }

    #[test]
    fn thinking_level_request_normalizes_high() {
        let req = json!({"prompt": "x", "model": "m", "thinking_level": "high"});
        assert_eq!(resolve_thinking_level(&req).unwrap().as_deref(), Some("HIGH"));
    }

    #[test]
    fn thinking_level_alias_camel_case() {
        let req = json!({"prompt": "x", "model": "m", "thinkingLevel": "medium"});
        assert_eq!(
            resolve_thinking_level(&req).unwrap().as_deref(),
            Some("MEDIUM")
        );
    }

    #[test]
    fn thinking_level_invalid_errors() {
        let req = json!({"prompt": "x", "model": "m", "thinking_level": "ultra"});
        let err = resolve_thinking_level(&req).unwrap_err();
        assert!(err.starts_with("thinking-level-invalid:"));
    }

    #[test]
    fn generation_config_omitted_when_empty() {
        assert!(generation_config(None, None).is_none());
        let payload = generate_content_payload("hola", None, None, None);
        assert!(payload.get("generationConfig").is_none());
        assert!(payload.get("systemInstruction").is_none());
    }

    #[test]
    fn classify_error_code_table() {
        assert_eq!(classify_error_code("http-status-429: x"), "rate_limited");
        assert_eq!(classify_error_code("http-status-503: x"), "upstream_unavailable");
        assert_eq!(classify_error_code("http-status-401: x"), "auth");
        assert_eq!(classify_error_code("http-post-failed: timed out"), "timeout");
        assert_eq!(classify_error_code("gemini-empty-candidate: finishReason=X"), "malformed_response");
        assert_eq!(classify_error_code("GEMINI_API_KEY ausente"), "auth");
    }

    #[test]
    fn payload_includes_system_instruction() {
        let p = generate_content_payload("u", Some("sys"), None, None);
        assert_eq!(p["systemInstruction"]["parts"][0]["text"], "sys");
    }

    #[test]
    fn timeout_ms_overrides_env() {
        let req = json!({"timeout_ms": 5000});
        assert_eq!(timeout_secs_from(&req), 5);
    }

    #[test]
    fn effort_maps_to_thinking_level() {
        let req = json!({"prompt": "x", "model": "m", "effort": "low"});
        assert_eq!(resolve_thinking_level(&req).unwrap().as_deref(), Some("LOW"));
    }

    #[test]
    fn generation_config_merges_temperature_and_thinking() {
        let cfg = generation_config(Some(0.2), Some("HIGH")).unwrap();
        assert_eq!(cfg["temperature"], json!(0.2));
        assert_eq!(cfg["thinkingConfig"]["thinkingLevel"], json!("HIGH"));
    }

    #[test]
    fn payload_has_no_model_slug_literal() {
        let src = include_str!("main.rs");
        let needle = format!("gemini-{}-flash", "3.8");
        assert!(!src.contains(&needle), "slug de catálogo eterno en crate");
    }
}
