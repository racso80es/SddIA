//! Skill `mayeuta-llm` — transductor CLI local (C1/C3).
//! Contrato stdin/stdout: ver docs/features/kalma2-mayeuta-llm-router/spec.md

use serde_json::{json, Value};
use std::io::{self, Read};

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
    let mut out = json!({
        "success": success,
        "data": data,
        "error": if error.is_empty() { Value::Null } else { json!(error) },
    });
    if success {
        out.as_object_mut().unwrap().remove("error");
        out["error"] = Value::Null;
    }
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

fn stub_synthesize(prompt: &str) -> Value {
    let snippet: String = prompt.chars().take(80).collect();
    json!({
        "text": format!("[mayeuta-llm/stub] Recibo: «{snippet}»")
    })
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
    emit(true, Some(stub_synthesize(p)), "");
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
    fn stub_synthesize_has_text() {
        let d = stub_synthesize("hola");
        assert!(d.get("text").and_then(|v| v.as_str()).unwrap().contains("hola"));
    }

    #[test]
    fn stub_classify_is_chat() {
        let d = stub_classify("fix algo");
        assert_eq!(d.get("intent").and_then(|v| v.as_str()), Some("chat"));
    }
}
