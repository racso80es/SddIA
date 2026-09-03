use sddia_io::outbound_lab::{lab_mock_outbound_enabled, truthy_env};
use sddia_io::read_stdin_json;
use serde_json::{json, Value};
use std::env;
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const ENTITY_ID: &str = "antigravity-cli-executor";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(300);

fn emit_v2(success: bool, exit_code: i32, message: &str, result: Option<Value>, feedback: Option<&str>) -> ! {
    let mut body = json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "skill",
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

fn required_prompt(req: &Value) -> Result<String, String> {
    req.get("prompt")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| "request.prompt obligatorio".to_string())
}

fn resolve_bin() -> Result<String, String> {
    if let Ok(raw) = env::var("SDDIA_AGY_PATH") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
    Ok("agy".to_string())
}

fn parse_print_timeout(raw: &str) -> Result<Duration, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(DEFAULT_TIMEOUT);
    }
    let (num, unit) = raw.split_at(raw.len().saturating_sub(1));
    let n: u64 = num
        .parse()
        .map_err(|_| format!("print_timeout inválido: {raw}"))?;
    match unit {
        "s" => Ok(Duration::from_secs(n.max(1))),
        "m" => Ok(Duration::from_secs(n.max(1) * 60)),
        "h" => Ok(Duration::from_secs(n.max(1) * 3600)),
        _ => Err(format!("print_timeout unidad inválida: {raw}")),
    }
}

pub(crate) fn build_argv(
    prompt: &str,
    params: &Value,
    allow_skip_permissions: bool,
) -> Result<(Vec<String>, Duration), String> {
    let mut argv = vec![
        "--print".to_string(),
        "--output-format".to_string(),
        "json".to_string(),
    ];

    let skip = params
        .get("skip_permissions")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
        && allow_skip_permissions;
    if skip {
        argv.push("--dangerously-skip-permissions".to_string());
    } else {
        argv.push("--sandbox".to_string());
    }

    if let Some(model) = params
        .get("model")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        argv.push("--model".to_string());
        argv.push(model.to_string());
    }

    if let Some(effort) = params
        .get("effort")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if !matches!(effort, "low" | "medium" | "high") {
            return Err(format!("effort inválido: {effort}"));
        }
        argv.push("--effort".to_string());
        argv.push(effort.to_string());
    }

    let mut timeout = DEFAULT_TIMEOUT;
    if let Some(raw) = params
        .get("print_timeout")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        timeout = parse_print_timeout(raw)?;
        argv.push("--print-timeout".to_string());
        argv.push(raw.to_string());
    }

    match params.get("add_dir") {
        Some(Value::String(p)) => {
            let p = p.trim();
            if !p.is_empty() {
                if p.contains("..") {
                    return Err("add_dir path traversal rechazado".into());
                }
                argv.push("--add-dir".to_string());
                argv.push(p.to_string());
            }
        }
        Some(Value::Array(items)) => {
            for item in items {
                let p = item.as_str().map(str::trim).filter(|s| !s.is_empty());
                let Some(p) = p else { continue };
                if p.contains("..") {
                    return Err("add_dir path traversal rechazado".into());
                }
                argv.push("--add-dir".to_string());
                argv.push(p.to_string());
            }
        }
        Some(_) => return Err("parameters.add_dir debe ser string o array".into()),
        None => {}
    }

    argv.push("-p".to_string());
    argv.push(prompt.to_string());
    Ok((argv, timeout))
}

fn map_agy_result(stdout: &str, stderr: &str) -> Result<Value, String> {
    let trimmed = stdout.trim();
    if trimmed.to_lowercase().contains("authentication required")
        || stderr.to_lowercase().contains("authentication required")
    {
        return Err("agy authentication required".into());
    }
    if trimmed.is_empty() {
        return Err(format!("agy stdout vacío; stderr={}", stderr.trim()));
    }
    let parsed: Value = serde_json::from_str(trimmed)
        .map_err(|e| format!("agy stdout no es JSON: {e}"))?;
    let status = parsed
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let text = parsed
        .get("response")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if status != "SUCCESS" {
        let err = parsed
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or(status);
        return Err(format!("agy status={status}: {err}"));
    }
    Ok(json!({
        "text": text,
        "raw_response": parsed,
        "usage": parsed.get("usage").cloned().unwrap_or(json!({})),
        "conversation_id": parsed.get("conversation_id").cloned()
    }))
}

fn spawn_agy(bin: &str, argv: &[String], timeout: Duration) -> Result<(String, String, i32), String> {
    let mut child = Command::new(bin)
        .args(argv)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn agy: {e}"))?;
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout = String::new();
                let mut stderr = String::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_string(&mut stdout);
                }
                if let Some(mut err) = child.stderr.take() {
                    let _ = err.read_to_string(&mut stderr);
                }
                return Ok((stdout, stderr, status.code().unwrap_or(1)));
            }
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("agy-timeout".into());
            }
            Ok(None) => thread::sleep(Duration::from_millis(50)),
            Err(e) => return Err(format!("wait agy: {e}")),
        }
    }
}

fn mock_result(prompt: &str) -> Value {
    json!({
        "text": format!("lab-mock-agy:{}", prompt.chars().take(80).collect::<String>()),
        "raw_response": {
            "status": "SUCCESS",
            "response": "lab-mock",
            "mode": "lab-mock-outbound"
        },
        "usage": {}
    })
}

fn run(doc: &Value) -> Result<Value, String> {
    let req = request_inner(doc);
    let prompt = required_prompt(req)?;
    let params = req.get("parameters").cloned().unwrap_or_else(|| json!({}));
    let allow_skip = truthy_env("SDDIA_AGY_ALLOW_SKIP_PERMISSIONS");
    let (argv, timeout) = build_argv(&prompt, &params, allow_skip)?;

    if lab_mock_outbound_enabled() {
        return Ok(mock_result(&prompt));
    }

    let bin = resolve_bin()?;
    let (stdout, stderr, code) = spawn_agy(&bin, &argv, timeout)?;
    if code != 0 {
        if let Ok(mapped) = map_agy_result(&stdout, &stderr) {
            return Ok(mapped);
        }
        return Err(format!(
            "agy exit={code}; stderr={}; stdout={}",
            stderr.trim(),
            stdout.trim()
        ));
    }
    map_agy_result(&stdout, &stderr)
}

fn main() {
    let started = Instant::now();
    let doc = read_stdin_json();
    match run(&doc) {
        Ok(mut result) => {
            result["durationMs"] = json!(started.elapsed().as_millis() as u64);
            emit_v2(true, 0, "ok", Some(result), None);
        }
        Err(msg) => emit_v2(false, 1, "agy-failed", None, Some(&msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argv_default_uses_sandbox_not_skip() {
        let (argv, _) = build_argv("hola", &json!({}), false).unwrap();
        assert!(argv.windows(2).any(|w| w == ["--output-format", "json"]));
        assert!(argv.contains(&"--sandbox".to_string()));
        assert!(!argv.iter().any(|a| a == "--dangerously-skip-permissions"));
        assert_eq!(argv[argv.len() - 2], "-p");
        assert_eq!(argv[argv.len() - 1], "hola");
    }

    #[test]
    fn argv_skip_requires_dual_opt_in() {
        let params = json!({"skip_permissions": true});
        let (denied, _) = build_argv("x", &params, false).unwrap();
        assert!(denied.contains(&"--sandbox".to_string()));
        let (allowed, _) = build_argv("x", &params, true).unwrap();
        assert!(allowed.contains(&"--dangerously-skip-permissions".to_string()));
        assert!(!allowed.contains(&"--sandbox".to_string()));
    }

    #[test]
    fn argv_effort_and_model_whitelisted() {
        let params = json!({"model": "gemini-x", "effort": "low", "print_timeout": "30s"});
        let (argv, timeout) = build_argv("p", &params, false).unwrap();
        assert!(argv.windows(2).any(|w| w == ["--model", "gemini-x"]));
        assert!(argv.windows(2).any(|w| w == ["--effort", "low"]));
        assert_eq!(timeout, Duration::from_secs(30));
    }

    #[test]
    fn effort_rejects_unknown() {
        let err = build_argv("p", &json!({"effort": "ultra"}), false).unwrap_err();
        assert!(err.contains("effort inválido"));
    }

    #[test]
    fn map_success_envelope() {
        let raw = r#"{"status":"SUCCESS","response":"ok","conversation_id":"c1","usage":{"input_tokens":1}}"#;
        let mapped = map_agy_result(raw, "").unwrap();
        assert_eq!(mapped["text"], "ok");
    }

    #[test]
    fn map_auth_required() {
        let err = map_agy_result("", "authentication required").unwrap_err();
        assert!(err.contains("authentication required"));
    }

    #[test]
    fn spawn_stub_returns_json() {
        use std::os::unix::fs::PermissionsExt;
        let stub = std::env::temp_dir().join(format!("agy-stub-{}.sh", std::process::id()));
        std::fs::write(
            &stub,
            "#!/bin/sh\nprintf '%s\\n' '{\"status\":\"SUCCESS\",\"response\":\"stub\"}'\n",
        )
        .unwrap();
        let mut perms = std::fs::metadata(&stub).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&stub, perms).unwrap();
        let (out, err, code) =
            spawn_agy(stub.to_str().unwrap(), &[], Duration::from_secs(2)).unwrap();
        let _ = std::fs::remove_file(&stub);
        assert_eq!(code, 0);
        assert_eq!(map_agy_result(&out, &err).unwrap()["text"], "stub");
    }
}
