//! Skill `mayeuta-llm` — transductor CLI local (C1/C3).

use serde_json::{json, Value};
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

const OP_SYNTHESIZE: &str = "SYNTHESIZE";
const OP_CLASSIFY: &str = "CLASSIFY_INTENT";
/// STREAM: orquesta subproceso inyectado y reenvía stdout línea a línea (sin envelope JSON).
const OP_STREAM: &str = "STREAM";
const CONFIDENCE_MIN: f64 = 0.0;
const CONFIDENCE_MAX: f64 = 1.0;

const ALLOWED_PROCESSES: &[&str] = &[
    "bug-fix",
    "feature",
    "refactorization",
    "task-queue-manager",
];

fn read_stdin() -> Value {
    let mut buf = String::new();
    if io::stdin().read_to_string(&mut buf).is_err() {
        emit(false, None, "failed to read stdin");
    }
    let buf = buf.trim();
    if buf.is_empty() {
        emit(false, None, "empty stdin");
    }
    match serde_json::from_str(buf) {
        Ok(v) => v,
        Err(e) => emit(false, None, &format!("invalid JSON stdin: {e}")),
    }
}

fn emit(success: bool, data: Option<Value>, error: &str) -> ! {
    let out = json!({
        "success": success,
        "data": data,
        "error": if error.is_empty() { Value::Null } else { json!(error) },
    });
    println!("{}", serde_json::to_string(&out).unwrap_or_else(|_| "{}".into()));
    std::process::exit(if success { 0 } else { 1 });
}

fn operation(doc: &Value) -> &str {
    doc.get("operation")
        .and_then(|v| v.as_str())
        .unwrap_or("")
}

fn prompt(doc: &Value) -> &str {
    doc.get("prompt")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("")
}

fn split_command(raw: &str) -> Result<Vec<String>, String> {
    let mut parts: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quote: Option<char> = None;
    for c in raw.trim().chars() {
        match (c, in_quote) {
            ('"', None) => in_quote = Some('"'),
            ('\'', None) => in_quote = Some('\''),
            ('"', Some('"')) => in_quote = None,
            ('\'', Some('\'')) => in_quote = None,
            (' ', None) if cur.is_empty() => {}
            (' ', None) => {
                parts.push(cur.clone());
                cur.clear();
            }
            (_, _) => cur.push(c),
        }
    }
    if !cur.is_empty() {
        parts.push(cur);
    }
    if parts.is_empty() {
        return Err("comando vacío".into());
    }
    Ok(parts)
}

fn resolve_cli_raw() -> Result<String, String> {
    for key in ["SDDIA_LLM_CHAT_COMMAND", "SDDIA_LLM_CLI_COMMAND"] {
        if let Ok(raw) = std::env::var(key) {
            let trimmed = raw.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.to_string());
            }
        }
    }
    Err("SDDIA_LLM_CLI_COMMAND / SDDIA_LLM_CHAT_COMMAND ausente".into())
}

fn run_cli(prompt_assembled: &str) -> Result<String, String> {
    let raw = resolve_cli_raw()?;
    let parts = split_command(&raw)?;
    let (bin, args) = parts
        .split_first()
        .ok_or_else(|| "comando vacío".to_string())?;

    let mut child = Command::new(bin)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn CLI: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(prompt_assembled.as_bytes())
            .map_err(|e| format!("stdin CLI: {e}"))?;
    }

    let out = child
        .wait_with_output()
        .map_err(|e| format!("output CLI: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(if err.is_empty() {
            format!("CLI exit {}", out.status.code().unwrap_or(1))
        } else {
            err
        });
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Orquesta subproceso y vuelca stdout en tiempo real (Ceguera Espacial: no interpreta destino).
fn handle_stream(doc: &Value) -> ! {
    let p = prompt(doc);
    if p.is_empty() {
        // STREAM no usa envelope JSON en éxito; en fallo escribe una línea a stderr y exit 1.
        eprintln!("prompt required");
        std::process::exit(1);
    }
    let raw = match resolve_cli_raw() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let parts = match split_command(&raw) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let (bin, args) = match parts.split_first() {
        Some(x) => x,
        None => {
            eprintln!("comando vacío");
            std::process::exit(1);
        }
    };

    let payload = json!({
        "operation": "CHAT_STREAM",
        "prompt": p,
    })
    .to_string();

    let mut child = match Command::new(bin)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("spawn CLI: {e}");
            std::process::exit(1);
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(payload.as_bytes()) {
            eprintln!("stdin CLI: {e}");
            let _ = child.kill();
            std::process::exit(1);
        }
    }

    let Some(stdout) = child.stdout.take() else {
        eprintln!("sin stdout del subproceso");
        let _ = child.kill();
        std::process::exit(1);
    };

    let mut reader = io::BufReader::new(stdout);
    let mut line = String::new();
    let mut out = io::stdout().lock();
    loop {
        line.clear();
        match io::BufRead::read_line(&mut reader, &mut line) {
            Ok(0) => break,
            Ok(_) => {
                if out.write_all(line.as_bytes()).is_err() {
                    let _ = child.kill();
                    std::process::exit(1);
                }
                let _ = out.flush();
            }
            Err(e) => {
                eprintln!("read stdout: {e}");
                let _ = child.kill();
                std::process::exit(1);
            }
        }
    }

    match child.wait() {
        Ok(status) if status.success() => std::process::exit(0),
        Ok(status) => {
            eprintln!("CLI exit {}", status.code().unwrap_or(1));
            std::process::exit(status.code().unwrap_or(1));
        }
        Err(e) => {
            eprintln!("wait CLI: {e}");
            std::process::exit(1);
        }
    }
}

fn build_classify_prompt(user_prompt: &str) -> String {
    format!(
        "Clasifica la intención del operador para SddIA.\n\
         Responde ÚNICAMENTE con un objeto JSON en la última línea (sin markdown):\n\
         {{\"intent\":\"chat\"|\"execute\",\"process_name\":null|\"bug-fix\"|\"feature\"|\"refactorization\"|\"task-queue-manager\",\
         \"process_inputs\":{{}},\"confidence\":0.0-1.0}}\n\
         Reglas:\n\
         - intent=execute solo si pide iniciar un proceso de ciclo de vida (fix, feature, refactor)\n\
         - process_name debe estar en la allowlist o ser null\n\
         - process_inputs.pbi_ref si aparece ruta docs/todos/pending/*.md\n\
         - confidence refleja certeza\n\n\
         Prompt del operador:\n{user_prompt}"
    )
}

fn extract_json_line(text: &str) -> Option<Value> {
    for line in text.lines().rev() {
        let t = line.trim();
        if t.starts_with('{') {
            if let Ok(v) = serde_json::from_str(t) {
                return Some(v);
            }
        }
    }
    None
}

fn normalize_process_name(raw: Option<&str>) -> Option<String> {
    let name = raw?.trim();
    if name.is_empty() {
        return None;
    }
    if ALLOWED_PROCESSES.contains(&name) {
        Some(name.to_string())
    } else {
        None
    }
}

fn heuristic_classify(prompt: &str) -> Value {
    let lower = prompt.to_lowercase();
    let pbi_ref = prompt
        .split_whitespace()
        .find(|t| t.contains("docs/todos/pending/") && t.ends_with(".md"))
        .map(str::trim)
        .map(str::to_string);

    let mut process_name: Option<String> = None;
    let mut confidence = 0.55;

    if prompt.contains("[FIX]") || lower.contains("bug-fix") || lower.contains("inicia fix") {
        process_name = Some("bug-fix".into());
        confidence = 0.85;
    } else if prompt.contains("[FEATURE]") || lower.contains("inicia feature") {
        process_name = Some("feature".into());
        confidence = 0.85;
    } else if lower.contains("refactor") {
        process_name = Some("refactorization".into());
        confidence = 0.8;
    }

    let intent = if process_name.is_some() {
        "execute"
    } else {
        "chat"
    };

    let mut process_inputs = json!({});
    if let Some(ref p) = pbi_ref {
        process_inputs["pbi_ref"] = json!(p);
    }

    json!({
        "intent": intent,
        "process_name": process_name,
        "process_inputs": process_inputs,
        "confidence": if intent == "chat" { 0.0 } else { confidence },
        "raw_prompt": prompt,
        "source": "heuristic",
    })
}

fn parse_classify_response(text: &str, fallback_prompt: &str) -> Value {
    let Some(raw) = extract_json_line(text) else {
        return heuristic_classify(fallback_prompt);
    };

    let intent = raw
        .get("intent")
        .and_then(|v| v.as_str())
        .unwrap_or("chat");
    let process_name = normalize_process_name(raw.get("process_name").and_then(|v| v.as_str()));
    let mut confidence = raw
        .get("confidence")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    confidence = confidence.clamp(CONFIDENCE_MIN, CONFIDENCE_MAX);

    let process_inputs = raw
        .get("process_inputs")
        .cloned()
        .unwrap_or_else(|| json!({}));

    let intent = if intent == "execute" && process_name.is_some() {
        "execute"
    } else {
        "chat"
    };

    json!({
        "intent": intent,
        "process_name": if intent == "execute" { process_name } else { None::<String> },
        "process_inputs": process_inputs,
        "confidence": if intent == "execute" { confidence } else { 0.0 },
        "raw_prompt": fallback_prompt,
        "source": "cli",
    })
}

fn handle_classify(doc: &Value) {
    let p = prompt(doc);
    if p.is_empty() {
        emit(false, None, "prompt required");
    }
    let assembled = build_classify_prompt(p);
    let data = match run_cli(&assembled) {
        Ok(out) if !out.is_empty() => parse_classify_response(&out, p),
        _ => heuristic_classify(p),
    };
    emit(true, Some(data), "");
}

fn handle_synthesize(doc: &Value) {
    let p = prompt(doc);
    if p.is_empty() {
        emit(false, None, "prompt required");
    }
    match run_cli(p) {
        Ok(text) if !text.is_empty() => emit(true, Some(json!({ "text": text })), ""),
        Ok(_) => emit(false, None, "CLI stdout vacío"),
        Err(e) => emit(false, None, &e),
    }
}

fn main() {
    let doc = read_stdin();
    match operation(&doc) {
        OP_SYNTHESIZE => handle_synthesize(&doc),
        OP_CLASSIFY => handle_classify(&doc),
        OP_STREAM => handle_stream(&doc),
        other => emit(false, None, &format!("unknown operation: {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_command_handles_quotes() {
        let p = split_command("echo hello").unwrap();
        assert_eq!(p, vec!["echo", "hello"]);
        let q = split_command("echo \"hello world\"").unwrap();
        assert_eq!(q, vec!["echo", "hello world"]);
    }

    #[test]
    fn synthesize_fails_without_cli_env() {
        std::env::remove_var("SDDIA_LLM_CLI_COMMAND");
        assert!(run_cli("test").is_err());
    }

    #[test]
    fn synthesize_with_echo_mock() {
        std::env::set_var("SDDIA_LLM_CLI_COMMAND", "echo mock-response");
        let out = run_cli("ignored").unwrap();
        assert!(out.contains("mock-response"));
        std::env::remove_var("SDDIA_LLM_CLI_COMMAND");
    }

    #[test]
    fn heuristic_detects_bug_fix() {
        let d = heuristic_classify("inicia fix docs/todos/pending/[FIX] x.md");
        assert_eq!(d.get("intent").and_then(|v| v.as_str()), Some("execute"));
        assert_eq!(
            d.get("process_name").and_then(|v| v.as_str()),
            Some("bug-fix")
        );
    }

    #[test]
    fn parse_classify_json_line() {
        let raw = "thinking...\n{\"intent\":\"execute\",\"process_name\":\"bug-fix\",\"process_inputs\":{},\"confidence\":0.9}";
        let d = parse_classify_response(raw, "fix");
        assert_eq!(d.get("intent").and_then(|v| v.as_str()), Some("execute"));
        assert!(d.get("confidence").and_then(|v| v.as_f64()).unwrap() >= 0.9);
    }
}
