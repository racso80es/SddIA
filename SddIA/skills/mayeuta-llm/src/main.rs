//! Skill `mayeuta-llm` — transductor CLI local (C1/C3).

use serde_json::{json, Value};
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

const OP_SYNTHESIZE: &str = "SYNTHESIZE";
const OP_CLASSIFY: &str = "CLASSIFY_INTENT";

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

fn run_cli(prompt_assembled: &str) -> Result<String, String> {
    let raw = std::env::var("SDDIA_LLM_CLI_COMMAND")
        .map_err(|_| "SDDIA_LLM_CLI_COMMAND ausente".to_string())?;
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

fn stub_classify(prompt: &str) -> Value {
    json!({
        "intent": "chat",
        "process_name": null,
        "process_inputs": {},
        "confidence": 0.0,
        "raw_prompt": prompt,
    })
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

fn handle_classify(doc: &Value) {
    let p = prompt(doc);
    if p.is_empty() {
        emit(false, None, "prompt required");
    }
    emit(true, Some(stub_classify(p)), "");
}

fn main() {
    let doc = read_stdin();
    match operation(&doc) {
        OP_SYNTHESIZE => handle_synthesize(&doc),
        OP_CLASSIFY => handle_classify(&doc),
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
}
