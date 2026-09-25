use sddia_io::read_stdin_json;
use serde_json::{json, Map, Value};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

const ENTITY_ID: &str = "llm-router";
const DEFAULT_MAX_HOPS: usize = 3;
const RETRYABLE: &[&str] = &["rate_limited", "timeout", "upstream_unavailable", "network"];

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
    std::process::exit(exit_code);
}

fn request_inner(doc: &Value) -> &Value {
    doc.get("request").unwrap_or(doc)
}

fn optional_str(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn adapter_bin_name(adapter_ref: &str) -> Result<String, String> {
    adapter_ref
        .strip_prefix("tool:")
        .or_else(|| adapter_ref.strip_prefix("skill:"))
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("adapter_ref inválido: {adapter_ref}"))
}

fn registry_path(doc: &Value) -> Result<PathBuf, String> {
    if let Ok(p) = env::var("SDDIA_LLM_REGISTRY_PATH") {
        let t = p.trim();
        if !t.is_empty() {
            return Ok(PathBuf::from(t));
        }
    }
    let repo = optional_str(doc, "repo_root")
        .or_else(|| env::var("SDDIA_REPO_ROOT").ok())
        .unwrap_or_else(|| ".".into());
    Ok(Path::new(&repo).join(".SddIA/llm-registry.json"))
}

fn load_registry(path: &Path) -> Result<Value, String> {
    if !path.is_file() {
        return Err("llm-registry-missing".into());
    }
    let raw = fs::read_to_string(path).map_err(|e| format!("llm-registry-invalid: {e}"))?;
    serde_json::from_str(&raw).map_err(|e| format!("llm-registry-invalid: {e}"))
}

fn oracles_map(reg: &Value) -> Result<&Map<String, Value>, String> {
    reg.get("oracles")
        .and_then(|v| v.as_object())
        .filter(|m| !m.is_empty())
        .ok_or_else(|| "llm-registry-invalid: oracles vacío".to_string())
}

fn oracle_entry<'a>(oracles: &'a Map<String, Value>, id: &str) -> Result<&'a Value, String> {
    oracles
        .get(id)
        .ok_or_else(|| format!("llm-registry-invalid: oracle desconocido {id}"))
}

fn is_active(entry: &Value) -> bool {
    entry.get("status").and_then(|v| v.as_str()).unwrap_or("active") == "active"
}

fn has_affinity(entry: &Value, affinity: &str) -> bool {
    entry
        .get("affinity")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().any(|x| x.as_str() == Some(affinity)))
        .unwrap_or(false)
}

fn detect_cycle(oracles: &Map<String, Value>) -> Result<(), String> {
    for start in oracles.keys() {
        let mut seen = HashSet::new();
        let mut cur = Some(start.as_str());
        while let Some(id) = cur {
            if !seen.insert(id.to_string()) {
                return Err(format!("llm-registry-invalid: ciclo en {id}"));
            }
            let next = oracles
                .get(id)
                .and_then(|e| e.get("fallback"))
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty());
            if let Some(n) = next {
                if !oracles.contains_key(n) {
                    return Err(format!("llm-registry-invalid: fallback inexistente {n}"));
                }
            }
            cur = next;
        }
    }
    Ok(())
}

fn select_oracle(oracles: &Map<String, Value>, oracle_id: Option<&str>, affinity: Option<&str>) -> Result<String, String> {
    if let Some(id) = oracle_id {
        let e = oracle_entry(oracles, id)?;
        if !is_active(e) {
            return Err(format!("llm-registry-no-active-oracle: {id} disabled"));
        }
        return Ok(id.to_string());
    }
    if let Some(aff) = affinity {
        for (id, e) in oracles {
            if is_active(e) && has_affinity(e, aff) {
                return Ok(id.clone());
            }
        }
    }
    for (id, e) in oracles {
        if is_active(e) {
            return Ok(id.clone());
        }
    }
    Err("llm-registry-no-active-oracle".into())
}

fn adapter_bin_dir(repo_root: &Path) -> PathBuf {
    if let Ok(p) = env::var("SDDIA_LLM_ROUTER_ADAPTER_BIN_DIR") {
        let t = p.trim();
        if !t.is_empty() {
            return PathBuf::from(t);
        }
    }
    let release = repo_root.join("SddIA/target/release");
    let debug = repo_root.join("SddIA/target/debug");
    if release.is_dir() {
        return release;
    }
    debug
}

fn invoke_adapter(bin_dir: &Path, adapter_ref: &str, payload: &Value) -> Result<Value, String> {
    let name = adapter_bin_name(adapter_ref)?;
    let path = bin_dir.join(&name);
    if !path.is_file() {
        return Err(format!("adapter-bin-missing: {name}"));
    }
    let mut child = Command::new(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn {name}: {e}"))?;
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(payload.to_string().as_bytes());
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("wait {name}: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if stdout.is_empty() {
        return Err(format!(
            "adapter-empty-stdout: {name}: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    serde_json::from_str(&stdout).map_err(|e| format!("adapter-malformed: {name}: {e}"))
}

fn adapter_error_code(body: &Value) -> String {
    body.pointer("/result/error_code")
        .or_else(|| body.pointer("/data/error_code"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| "unknown".into())
}

fn adapter_success(body: &Value) -> bool {
    body.get("success") == Some(&json!(true)) && body.get("exitCode") != Some(&json!(1))
}

fn adapter_text_result(body: &Value) -> Value {
    body.get("result")
        .cloned()
        .or_else(|| body.get("data").cloned())
        .unwrap_or_else(|| body.clone())
}

fn retryable(code: &str) -> bool {
    RETRYABLE.contains(&code)
}

fn build_adapter_request(infer: &Value, entry: &Value) -> Value {
    let mut req = infer.clone();
    if optional_str(&req, "model").is_none() {
        if let Some(m) = optional_str(entry, "model") {
            req["model"] = json!(m);
        }
    }
    if req.get("timeout_ms").is_none() {
        if let Some(ms) = entry.get("timeout_ms") {
            req["timeout_ms"] = ms.clone();
        }
    }
    json!({ "request": req })
}

fn route(doc: &Value) -> Result<Value, (String, Value)> {
    let infer = request_inner(doc).clone();
    if optional_str(&infer, "prompt").is_none() {
        return Err((
            "request.prompt obligatorio".into(),
            json!({
                "error_code": "unknown",
                "routing": { "attempts": [], "hops": 0 }
            }),
        ));
    }
    let path = registry_path(doc).map_err(|e| {
        (
            e.clone(),
            json!({"error_code": "unknown", "routing": {"attempts": [], "hops": 0}}),
        )
    })?;
    let reg = load_registry(&path).map_err(|e| {
        let code = if e == "llm-registry-missing" {
            "unknown"
        } else {
            "unknown"
        };
        (
            e,
            json!({"error_code": code, "routing": {"attempts": [], "hops": 0}}),
        )
    })?;
    let oracles = oracles_map(&reg).map_err(|e| {
        (
            e,
            json!({"error_code": "unknown", "routing": {"attempts": [], "hops": 0}}),
        )
    })?;
    detect_cycle(oracles).map_err(|e| {
        (
            e,
            json!({"error_code": "unknown", "routing": {"attempts": [], "hops": 0}}),
        )
    })?;

    let oracle_id = optional_str(doc, "oracle_id");
    let affinity = optional_str(doc, "affinity");
    let mut current = select_oracle(oracles, oracle_id.as_deref(), affinity.as_deref()).map_err(|e| {
        (
            e,
            json!({"error_code": "unknown", "routing": {"attempts": [], "hops": 0}}),
        )
    })?;

    let max_hops = doc
        .get("max_hops")
        .and_then(|v| v.as_u64())
        .unwrap_or(DEFAULT_MAX_HOPS as u64) as usize;
    let repo = optional_str(doc, "repo_root")
        .or_else(|| env::var("SDDIA_REPO_ROOT").ok())
        .unwrap_or_else(|| ".".into());
    let bin_dir = adapter_bin_dir(Path::new(&repo));

    let mut attempts = Vec::new();
    let mut hops = 0usize;
    let mut last_err = "llm-registry-no-active-oracle".to_string();
    let mut last_code = "unknown".to_string();
    let mut last_result = json!({});

    while hops < max_hops {
        hops += 1;
        let entry = oracle_entry(oracles, &current).map_err(|e| {
            (
                e,
                json!({"error_code": "unknown", "routing": {"attempts": attempts.clone(), "hops": hops}}),
            )
        })?;
        let adapter_ref = optional_str(entry, "adapter_ref").ok_or_else(|| {
            (
                format!("llm-registry-invalid: adapter_ref ausente en {current}"),
                json!({"error_code": "unknown", "routing": {"attempts": attempts.clone(), "hops": hops}}),
            )
        })?;
        let payload = build_adapter_request(&infer, entry);
        let started = Instant::now();
        match invoke_adapter(&bin_dir, &adapter_ref, &payload) {
            Ok(body) if adapter_success(&body) => {
                let mut result = adapter_text_result(&body);
                result["routing"] = json!({
                    "oracle_id": current,
                    "hops": hops,
                    "attempts": attempts,
                });
                return Ok(result);
            }
            Ok(body) => {
                let code = adapter_error_code(&body);
                let ms = started.elapsed().as_millis() as u64;
                attempts.push(json!({
                    "oracle_id": current,
                    "adapter_ref": adapter_ref,
                    "error_code": code,
                    "provider_latency_ms": ms,
                }));
                last_code = code.clone();
                last_err = body
                    .get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&code)
                    .to_string();
                last_result = adapter_text_result(&body);
                let fallback = entry.get("fallback").and_then(|v| v.as_str()).filter(|s| !s.is_empty());
                if retryable(&code) {
                    if let Some(next) = fallback {
                        current = next.to_string();
                        continue;
                    }
                }
                last_result["routing"] = json!({
                    "oracle_id": current,
                    "hops": hops,
                    "attempts": attempts,
                });
                last_result["error_code"] = json!(code);
                return Err((last_err, last_result));
            }
            Err(e) => {
                let ms = started.elapsed().as_millis() as u64;
                let code = if e.contains("adapter-bin-missing") {
                    "unknown"
                } else if e.contains("adapter-malformed") {
                    "malformed_response"
                } else {
                    "unknown"
                };
                attempts.push(json!({
                    "oracle_id": current,
                    "adapter_ref": adapter_ref,
                    "error_code": code,
                    "provider_latency_ms": ms,
                }));
                last_code = code.to_string();
                last_err = e;
                let fallback = entry.get("fallback").and_then(|v| v.as_str()).filter(|s| !s.is_empty());
                if retryable(code) {
                    if let Some(next) = fallback {
                        current = next.to_string();
                        continue;
                    }
                }
                break;
            }
        }
    }
    Err((
        last_err,
        json!({
            "error_code": last_code,
            "routing": { "attempts": attempts, "hops": hops }
        }),
    ))
}

fn main() {
    let doc = read_stdin_json();
    match route(&doc) {
        Ok(result) => emit_v2(true, 0, "ok", Some(result), None),
        Err((msg, result)) => emit_v2(false, 1, "route-failed", Some(result), Some(&msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn stub_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("llm-router-stubs-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        dir
    }

    fn write_stub(dir: &Path, name: &str, body: &str) {
        let p = dir.join(name);
        fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
        let mut perms = fs::metadata(&p).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&p, perms).unwrap();
    }

    fn sample_reg(primary_fail: bool) -> Value {
        let primary_status = if primary_fail { "active" } else { "active" };
        json!({
            "registry_version": "1.0.0",
            "oracles": {
                "oracle-agy": {
                    "adapter_ref": "skill:antigravity-cli-executor",
                    "model": "",
                    "affinity": ["aiua"],
                    "timeout_ms": 5000,
                    "fallback": "oracle-gemini",
                    "status": primary_status
                },
                "oracle-gemini": {
                    "adapter_ref": "tool:gemini-http-infer",
                    "model": "",
                    "affinity": [],
                    "timeout_ms": 5000,
                    "fallback": null,
                    "status": "active"
                }
            }
        })
    }

    #[test]
    fn adapter_bin_name_strips_prefix() {
        assert_eq!(adapter_bin_name("skill:antigravity-cli-executor").unwrap(), "antigravity-cli-executor");
        assert_eq!(adapter_bin_name("tool:gemini-http-infer").unwrap(), "gemini-http-infer");
        assert!(adapter_bin_name("gemini-http-infer").is_err());
    }

    #[test]
    fn select_by_affinity_then_id() {
        let reg = sample_reg(false);
        let o = oracles_map(&reg).unwrap();
        assert_eq!(select_oracle(o, None, Some("aiua")).unwrap(), "oracle-agy");
        assert_eq!(select_oracle(o, Some("oracle-gemini"), None).unwrap(), "oracle-gemini");
    }

    #[test]
    fn detect_cycle_rejects_loop() {
        let reg = json!({
            "oracles": {
                "a": {"adapter_ref": "tool:x", "fallback": "b", "status": "active"},
                "b": {"adapter_ref": "tool:y", "fallback": "a", "status": "active"}
            }
        });
        assert!(detect_cycle(oracles_map(&reg).unwrap()).is_err());
    }

    #[test]
    fn retryable_set() {
        assert!(retryable("rate_limited"));
        assert!(!retryable("auth"));
        assert!(!retryable("malformed_response"));
    }

    #[test]
    fn route_fallback_on_rate_limited() {
        let dir = stub_dir();
        write_stub(
            &dir,
            "antigravity-cli-executor",
            r#"printf '%s\n' '{"success":false,"exitCode":1,"error":"429","result":{"error_code":"rate_limited"}}'"#,
        );
        write_stub(
            &dir,
            "gemini-http-infer",
            r#"printf '%s\n' '{"success":true,"exitCode":0,"result":{"text":"ok-gemini","telemetry_receipt":{"provider":"gemini-http-infer"}}}'"#,
        );
        let reg_path = dir.join("reg.json");
        fs::write(&reg_path, sample_reg(false).to_string()).unwrap();
        env::set_var("SDDIA_LLM_REGISTRY_PATH", &reg_path);
        env::set_var("SDDIA_LLM_ROUTER_ADAPTER_BIN_DIR", &dir);
        let doc = json!({"request": {"prompt": "hola"}, "affinity": "aiua"});
        let out = route(&doc).expect("fallback");
        assert_eq!(out["text"], "ok-gemini");
        assert_eq!(out["routing"]["attempts"].as_array().unwrap().len(), 1);
        assert_eq!(out["routing"]["hops"], 2);
        env::remove_var("SDDIA_LLM_REGISTRY_PATH");
        env::remove_var("SDDIA_LLM_ROUTER_ADAPTER_BIN_DIR");
    }

    #[test]
    fn route_no_fallback_on_auth() {
        let dir = stub_dir();
        write_stub(
            &dir,
            "antigravity-cli-executor",
            r#"printf '%s\n' '{"success":false,"exitCode":1,"error":"auth","result":{"error_code":"auth"}}'"#,
        );
        write_stub(
            &dir,
            "gemini-http-infer",
            r#"printf '%s\n' '{"success":true,"exitCode":0,"result":{"text":"should-not"}}'"#,
        );
        let reg_path = dir.join("reg-auth.json");
        fs::write(&reg_path, sample_reg(false).to_string()).unwrap();
        env::set_var("SDDIA_LLM_REGISTRY_PATH", &reg_path);
        env::set_var("SDDIA_LLM_ROUTER_ADAPTER_BIN_DIR", &dir);
        let doc = json!({"request": {"prompt": "hola"}, "affinity": "aiua"});
        let err = route(&doc).unwrap_err();
        assert_eq!(err.1["error_code"], "auth");
        assert_eq!(err.1["routing"]["attempts"].as_array().unwrap().len(), 1);
        env::remove_var("SDDIA_LLM_REGISTRY_PATH");
        env::remove_var("SDDIA_LLM_ROUTER_ADAPTER_BIN_DIR");
    }

    #[test]
    fn route_missing_registry() {
        env::set_var("SDDIA_LLM_REGISTRY_PATH", "/tmp/llm-registry-does-not-exist-xyz.json");
        let doc = json!({"request": {"prompt": "hola"}});
        let err = route(&doc).unwrap_err();
        assert_eq!(err.0, "llm-registry-missing");
        env::remove_var("SDDIA_LLM_REGISTRY_PATH");
    }
}
