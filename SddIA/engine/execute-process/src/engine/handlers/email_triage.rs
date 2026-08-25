//! Handler nativo `email-triage-gateway` (PBI-KALMA2-MVP-01A). Peaje G5: Triaje-C early-exit.

use super::super::capsules::invoke_capsule_json;
use super::super::fractal::{load_fractal_dirs, write_fractal_event};
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TriageC {
    concluded: bool,
    verdict: Option<&'static str>,
    matched_rule: Option<&'static str>,
}

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn zeros_cost() -> Value {
    json!({ "tokens_in": 0, "tokens_out": 0, "duration_ms": 0 })
}

fn from_addr(payload: &Value) -> String {
    decode_rfc2047(
        payload
            .get("from")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
    )
    .to_ascii_lowercase()
}

fn subject_of(payload: &Value) -> String {
    decode_rfc2047(
        payload
            .get("subject")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
    )
    .to_ascii_lowercase()
}

fn decode_q_bytes(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'_' => {
                out.push(b' ');
                i += 1;
            }
            b'=' if i + 2 < b.len() => {
                match u8::from_str_radix(std::str::from_utf8(&b[i + 1..i + 3]).unwrap_or(""), 16) {
                    Ok(v) => {
                        out.push(v);
                        i += 3;
                    }
                    Err(_) => {
                        out.push(b[i]);
                        i += 1;
                    }
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}

fn decode_base64_bytes(s: &str) -> Vec<u8> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' | b'-' => Some(62),
            b'/' | b'_' => Some(63),
            _ => None,
        }
    }
    let filtered: Vec<u8> = s.bytes().filter(|c| *c != b'=' && !c.is_ascii_whitespace()).collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < filtered.len() {
        let a = val(filtered[i]).unwrap_or(0) as u32;
        let b = filtered.get(i + 1).copied().and_then(val).unwrap_or(0) as u32;
        let c = filtered.get(i + 2).copied().and_then(val).unwrap_or(0) as u32;
        let d = filtered.get(i + 3).copied().and_then(val).unwrap_or(0) as u32;
        let n = (a << 18) | (b << 12) | (c << 6) | d;
        out.push((n >> 16) as u8);
        if i + 2 < filtered.len() {
            out.push(((n >> 8) & 0xff) as u8);
        }
        if i + 3 < filtered.len() {
            out.push((n & 0xff) as u8);
        }
        i += 4;
    }
    out
}

fn charset_bytes(charset: &str, bytes: &[u8]) -> String {
    match charset.trim().to_ascii_lowercase().as_str() {
        "utf-8" | "utf8" | "us-ascii" => String::from_utf8_lossy(bytes).into_owned(),
        _ => bytes.iter().map(|&b| char::from(b)).collect(),
    }
}

fn take_encoded_word(s: &str) -> Option<(usize, String)> {
    if !s.starts_with("=?") {
        return None;
    }
    let rest = &s[2..];
    let c_end = rest.find('?')?;
    let charset = &rest[..c_end];
    let rest = &rest[c_end + 1..];
    let e_end = rest.find('?')?;
    let enc = rest[..e_end].to_ascii_uppercase();
    let rest = &rest[e_end + 1..];
    let b_end = rest.find("?=")?;
    let body = &rest[..b_end];
    let total = 2 + c_end + 1 + e_end + 1 + b_end + 2;
    let raw = match enc.as_str() {
        "Q" => decode_q_bytes(body),
        "B" => decode_base64_bytes(body),
        _ => return None,
    };
    Some((total, charset_bytes(charset, &raw)))
}

fn decode_rfc2047(input: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < input.len() {
        if input[i..].starts_with("=?") {
            if let Some((consumed, decoded)) = take_encoded_word(&input[i..]) {
                out.push_str(&decoded);
                i += consumed;
                let rest = &input[i..];
                let ws = rest
                    .chars()
                    .take_while(|c| matches!(*c, ' ' | '\t' | '\r' | '\n'))
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
                if rest.get(ws..).is_some_and(|r| r.starts_with("=?")) {
                    i += ws;
                }
                continue;
            }
        }
        let ch = input[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn llm_output_blob(body: &Value) -> String {
    body.get("data")
        .and_then(|d| d.get("text"))
        .and_then(|v| v.as_str())
        .or_else(|| {
            body.get("data")
                .and_then(|d| d.get("result"))
                .and_then(|v| v.as_str())
        })
        .or_else(|| body.get("result").and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string()
}

fn parse_triage_llm_blob(text: &str) -> Value {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return json!({});
    }
    if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
        return v;
    }
    for line in trimmed.lines().rev() {
        let t = line.trim().trim_start_matches("```json").trim_start_matches("```").trim();
        if t.starts_with('{') {
            if let Ok(v) = serde_json::from_str::<Value>(t) {
                return v;
            }
        }
    }
    if let (Some(s), Some(e)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if e > s {
            if let Ok(v) = serde_json::from_str::<Value>(&trimmed[s..=e]) {
                return v;
            }
        }
    }
    json!({})
}

fn extract_actionable_from_subject(subject: &str) -> Option<(String, String)> {
    let decoded = decode_rfc2047(subject);
    let lower = decoded.to_lowercase();
    let acto = ["reunión", "reunion", "meeting", "cita", "llamada"]
        .iter()
        .any(|k| lower.contains(k));
    if !acto {
        return None;
    }
    let re = regex::Regex::new(
        r"(?i)(\d{1,2}/\d{1,2}/\d{4})(?:\s*(?:a\s+las\s+)?(\d{1,2}:\d{2}))?",
    )
    .ok()?;
    let cap = re.captures(&decoded)?;
    let date = cap.get(1)?.as_str();
    let datetime = match cap.get(2).map(|m| m.as_str()) {
        Some(t) => format!("{date} {t}"),
        None => date.to_string(),
    };
    let title = decoded.trim().to_string();
    if title.is_empty() {
        return None;
    }
    Some((title, datetime))
}

fn list_headers_text(payload: &Value) -> String {
    match payload.get("list_headers") {
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join("\n")
            .to_ascii_lowercase(),
        Some(Value::String(s)) => s.to_ascii_lowercase(),
        _ => String::new(),
    }
}

fn triaje_c(payload: &Value) -> TriageC {
    let lists = list_headers_text(payload);
    if lists.contains("list-id")
        || lists.contains("list-unsubscribe")
        || lists.contains("precedence: bulk")
        || lists.contains("precedence: list")
        || (lists.contains("auto-submitted") && !lists.contains("auto-submitted: no"))
    {
        return TriageC {
            concluded: true,
            verdict: Some("noise"),
            matched_rule: Some("C-LIST"),
        };
    }
    let from = from_addr(payload);
    if from.contains("no-reply@")
        || from.contains("noreply@")
        || from.contains("mailer-daemon@")
        || from.contains("notifications@")
    {
        return TriageC {
            concluded: true,
            verdict: Some("noise"),
            matched_rule: Some("C-NOREPLY"),
        };
    }
    let subject = subject_of(payload);
    for pat in ["unsubscribe", "viagra", "newsletter", "view in browser"] {
        if subject.contains(pat) {
            return TriageC {
                concluded: true,
                verdict: Some("noise"),
                matched_rule: Some("C-SUBJECT-NOISE"),
            };
        }
    }
    TriageC {
        concluded: false,
        verdict: None,
        matched_rule: None,
    }
}

fn commercial_verbosity_trap(payload: &Value) -> bool {
    let snippet = payload
        .get("snippet")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let subject = subject_of(payload);
    let blob = format!("{subject} {snippet}");
    let long = snippet.chars().count() > 400;
    let urgent = blob.contains("urgent")
        || blob.contains("act now")
        || blob.contains("!!!")
        || blob.contains("limited time");
    let commercial = blob.contains("buy now")
        || blob.contains("discount")
        || blob.contains("oferta")
        || blob.contains("unsubscribe");
    long && (urgent || commercial)
}

fn persist_agenda(repo: &Path, payload: &Value, title: &str, datetime: &str) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let dir = repo.join(".SddIA").join("agenda");
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir agenda: {e}"))?;
    let entry = json!({
        "agenda_entry_id": id,
        "title": title,
        "datetime": datetime,
        "source_ref": payload.get("body_ref").cloned().unwrap_or(json!(null)),
        "message_uid": payload.get("message_uid"),
        "created_at": iso_now(),
    });
    fs::write(dir.join(format!("{id}.json")), format!("{entry}\n"))
        .map_err(|e| format!("write agenda: {e}"))?;
    Ok(id)
}

fn llm_require_infer() -> bool {
    std::env::var("SDDIA_LLM_REQUIRE_INFER")
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

/// L-INFER: peaje 0 sin elevación estructural → `classification-degraded`.
fn mark_classification_degraded(
    extras: &mut Value,
    require_infer: bool,
    tokens_in: u64,
    tokens_out: u64,
    elevated: bool,
) {
    if require_infer && tokens_in + tokens_out == 0 && !elevated {
        extras["classification-degraded"] = json!(true);
    }
}

/// Post-LLM: extracción estructural completa eleva a actionable (L-GUARD).
fn maybe_elevate_from_subject(
    verdict: &str,
    title: Option<String>,
    datetime: Option<String>,
    subject_plain: &str,
) -> (String, Option<String>, Option<String>, bool) {
    let already_complete = verdict == "actionable" && datetime.is_some();
    if already_complete {
        return (verdict.to_string(), title, datetime, false);
    }
    if let Some((t, dt)) = extract_actionable_from_subject(subject_plain) {
        let title = if title.as_ref().map(|s| !s.is_empty()).unwrap_or(false) {
            title
        } else {
            Some(t)
        };
        return ("actionable".into(), title, Some(dt), true);
    }
    let mut verdict = verdict.to_string();
    if verdict.is_empty() {
        verdict = "passive".into();
    }
    if verdict == "actionable" && datetime.is_none() {
        verdict = "passive".into();
    }
    (verdict, title, datetime, false)
}

fn classify_llm(repo: &Path, payload: &Value) -> Result<(String, Option<String>, Option<String>, Value, Value), String> {
    let started = std::time::Instant::now();
    let from_plain = decode_rfc2047(payload.get("from").and_then(|v| v.as_str()).unwrap_or(""));
    let subject_plain = decode_rfc2047(payload.get("subject").and_then(|v| v.as_str()).unwrap_or(""));
    let snippet = payload.get("snippet").and_then(|v| v.as_str()).unwrap_or("");
    let prompt = format!(
        "Clasifica este correo como noise, passive o actionable. JSON estricto {{\"verdict\":\"...\",\"title\":null,\"datetime\":null}}. Reunión o cita con fecha extraíble en el asunto es candidato actionable (datetime obligatorio). No uses verbosidad ni urgencia comercial para elevar a actionable. from={} subject={} snippet={}",
        from_plain, subject_plain, snippet,
    );
    let body = match invoke_capsule_json(
        repo,
        "mayeuta-llm",
        &json!({"operation": "SYNTHESIZE", "prompt": prompt}),
        false,
    ) {
        Ok(r) => r.body,
        Err(_) => {
            let (verdict, title, datetime, elevated) =
                maybe_elevate_from_subject("", None, None, &subject_plain);
            let mut extras = json!({});
            if elevated {
                extras["subject_elevation"] = json!(true);
            }
            mark_classification_degraded(&mut extras, llm_require_infer(), 0, 0, elevated);
            let cost = json!({
                "tokens_in": 0,
                "tokens_out": 0,
                "duration_ms": started.elapsed().as_millis() as u64,
            });
            return Ok((verdict, title, datetime, cost, extras));
        }
    };
    let text = llm_output_blob(&body);
    let parsed = parse_triage_llm_blob(&text);
    let mut verdict = parsed
        .get("verdict")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if !matches!(verdict.as_str(), "noise" | "passive" | "actionable") {
        verdict = String::new();
    }
    if commercial_verbosity_trap(payload) && verdict == "actionable" {
        verdict = "passive".into();
    }
    let title = parsed
        .get("title")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let datetime = parsed
        .get("datetime")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let (verdict, title, datetime, extracted) =
        maybe_elevate_from_subject(&verdict, title, datetime, &subject_plain);
    let mut extras = json!({});
    if extracted {
        extras["subject_elevation"] = json!(true);
    }
    let tokens_in = body.get("tokens_in").and_then(|v| v.as_u64()).unwrap_or(0);
    let tokens_out = body.get("tokens_out").and_then(|v| v.as_u64()).unwrap_or(0);
    mark_classification_degraded(
        &mut extras,
        llm_require_infer(),
        tokens_in,
        tokens_out,
        extracted,
    );
    let cost = json!({
        "tokens_in": tokens_in,
        "tokens_out": tokens_out,
        "duration_ms": started.elapsed().as_millis() as u64,
    });
    Ok((verdict, title, datetime, cost, extras))
}

fn emit_triaged(
    repo: &Path,
    src: &Value,
    verdict: &str,
    decision_path: &str,
    matched_rule: Option<&str>,
    cost: &Value,
    agenda_entry_id: Option<&str>,
    extras: Option<&Value>,
) -> Result<Value, String> {
    let message_uid = src
        .get("message_uid")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let (_, _, domain_dir, _) = load_fractal_dirs(repo);
    let mut payload = json!({
        "message_uid": message_uid,
        "verdict": verdict,
        "decision_path": decision_path,
        "thermodynamic_cost": cost,
    });
    if let Some(rule) = matched_rule {
        payload["matched_rule"] = json!(rule);
    }
    if let Some(aid) = agenda_entry_id {
        payload["agenda_entry_id"] = json!(aid);
    }
    if let Some(ex) = extras.and_then(|v| v.as_object()) {
        for (k, v) in ex {
            payload[k.clone()] = v.clone();
        }
    }
    if let Some(from) = src
        .get("from")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        payload["from"] = json!(decode_rfc2047(from));
    }
    if let Some(subject) = src
        .get("subject")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        payload["subject"] = json!(decode_rfc2047(subject));
    }
    let event = json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "Email_Triaged",
        "event_family": "domain",
        "timestamp": iso_now(),
        "emitter_agent": "email-triage-gateway",
        "payload": payload,
    });
    let seal = write_fractal_event(repo, &event, &domain_dir)?;
    persist_email_triaged_proof(repo, &event)?;
    Ok(seal)
}

fn proofs_root(repo: &Path) -> PathBuf {
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let default = repo.join(".SddIA").join("proofs");
    let Ok(text) = fs::read_to_string(&cfg_path) else {
        return default;
    };
    let Ok(cfg) = serde_json::from_str::<Value>(&text) else {
        return default;
    };
    cfg.get("eda_instance")
        .and_then(|e| e.get("proofs"))
        .and_then(|v| v.as_str())
        .map(|s| {
            let rel = s.trim().trim_start_matches("./");
            repo.join(rel)
        })
        .unwrap_or(default)
}

fn persist_email_triaged_proof(repo: &Path, event: &Value) -> Result<(), String> {
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .ok_or("event_id required for proof")?;
    let dir = proofs_root(repo).join("email-triaged");
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir email-triaged proof: {e}"))?;
    let body = json!({
        "kind": "email-triaged-proof",
        "event_id": event_id,
        "event_type": "Email_Triaged",
        "timestamp": event.get("timestamp"),
        "payload": event.get("payload"),
    });
    fs::write(dir.join(format!("{event_id}.json")), format!("{body}\n"))
        .map_err(|e| format!("write email-triaged proof: {e}"))?;
    Ok(())
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let rel = process_inputs
        .get("event_file_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_file_path requerido")?;
    let path: PathBuf = {
        let p = Path::new(rel);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            repo.join(rel)
        }
    };
    let raw = fs::read_to_string(&path).map_err(|e| format!("leer evento: {e}"))?;
    let event: Value = serde_json::from_str(&raw).map_err(|e| format!("JSON evento: {e}"))?;
    let payload = event.get("payload").cloned().unwrap_or(json!({}));

    let mut phases = Vec::new();
    let c = triaje_c(&payload);
    let mut classification_ran = false;

    let (verdict, decision_path, matched_rule, cost, agenda_id, extras) = if c.concluded {
        phases.push(json!({
            "phase_name": "Triaje-C",
            "status": "executed",
            "matched_rule": c.matched_rule,
            "verdict": c.verdict,
        }));
        phases.push(json!({
            "phase_name": "Clasificacion",
            "status": "skipped",
            "reason": "triaje-c-concluded",
        }));
        phases.push(json!({
            "phase_name": "Asiento-Agenda",
            "status": "skipped",
            "reason": "not-actionable",
        }));
        (
            c.verdict.unwrap_or("noise").to_string(),
            "deterministic".to_string(),
            c.matched_rule,
            zeros_cost(),
            None,
            json!({}),
        )
    } else {
        phases.push(json!({
            "phase_name": "Triaje-C",
            "status": "executed",
            "concluded": false,
        }));
        classification_ran = true;
        let classified = classify_llm(repo, &payload);
        match classified {
            Ok((verdict, title, datetime, cost, extras)) => {
                phases.push(json!({
                    "phase_name": "Clasificacion",
                    "status": "executed",
                    "verdict": verdict,
                }));
                let mut agenda_id = None;
                if verdict == "actionable" {
                    if let (Some(t), Some(dt)) = (title.as_deref(), datetime.as_deref()) {
                        match persist_agenda(repo, &payload, t, dt) {
                            Ok(id) => {
                                agenda_id = Some(id);
                                phases.push(json!({
                                    "phase_name": "Asiento-Agenda",
                                    "status": "executed",
                                    "agenda_entry_id": agenda_id,
                                }));
                            }
                            Err(e) => {
                                phases.push(json!({
                                    "phase_name": "Asiento-Agenda",
                                    "status": "failed",
                                    "error": e,
                                }));
                            }
                        }
                    } else {
                        phases.push(json!({
                            "phase_name": "Asiento-Agenda",
                            "status": "skipped",
                            "reason": "extraction-incomplete",
                        }));
                    }
                } else {
                    phases.push(json!({
                        "phase_name": "Asiento-Agenda",
                        "status": "skipped",
                        "reason": "not-actionable",
                    }));
                }
                (verdict, "llm".to_string(), None, cost, agenda_id, extras)
            }
            Err(e) => {
                phases.push(json!({
                    "phase_name": "Clasificacion",
                    "status": "failed",
                    "error": e,
                }));
                phases.push(json!({
                    "phase_name": "Asiento-Agenda",
                    "status": "skipped",
                    "reason": "classification-failed",
                }));
                (
                    "passive".to_string(),
                    "llm".to_string(),
                    None,
                    zeros_cost(),
                    None,
                    json!({}),
                )
            }
        }
    };

    let seal = emit_triaged(
        repo,
        &payload,
        &verdict,
        &decision_path,
        matched_rule,
        &cost,
        agenda_id.as_deref(),
        Some(&extras),
    )?;
    phases.push(json!({
        "phase_name": "Emision",
        "status": "executed",
        "event_type": "Email_Triaged",
    }));

    let data = json!({
        "verdict": verdict,
        "decision_path": decision_path,
        "emitted": true,
        "classification_ran": classification_ran,
        "seal": seal,
    });
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(data),
        error: None,
        execution_report: Some(json!({
            "process_name": "email-triage-gateway",
            "phases": phases,
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_headers_are_deterministic_noise() {
        let payload = json!({
            "from": "list@example.com",
            "subject": "Weekly",
            "list_headers": ["List-Id: <news.example.com>"]
        });
        let c = triaje_c(&payload);
        assert!(c.concluded);
        assert_eq!(c.verdict, Some("noise"));
        assert_eq!(c.matched_rule, Some("C-LIST"));
    }

    #[test]
    fn noreply_is_noise() {
        let payload = json!({"from": "noreply@shop.tld", "subject": "Receipt"});
        let c = triaje_c(&payload);
        assert_eq!(c.matched_rule, Some("C-NOREPLY"));
    }

    #[test]
    fn verbosity_does_not_conclude_triaje_c() {
        let payload = json!({
            "from": "human@example.com",
            "subject": "URGENT ACT NOW !!!",
            "snippet": "buy now ".repeat(80),
        });
        let c = triaje_c(&payload);
        assert!(!c.concluded);
        assert!(commercial_verbosity_trap(&payload));
    }

    #[test]
    fn emit_triaged_copies_from_subject_not_snippet() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_fractal":{"domain":".events/domain","orchestration":".events/orchestration","telemetry":".events/telemetry","dead_letter":".events/dead-letter"},"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        let src = json!({
            "message_uid": "7",
            "from": "a@b",
            "subject": "Acto",
            "snippet": "secreto"
        });
        let seal = emit_triaged(repo, &src, "actionable", "deterministic", None, &zeros_cost(), None, None)
            .expect("emit");
        let event_id = seal.get("event_id").and_then(|v| v.as_str()).unwrap();
        let proof: Value = serde_json::from_str(
            &fs::read_to_string(repo.join(".SddIA/proofs/email-triaged").join(format!("{event_id}.json")))
                .unwrap(),
        )
        .unwrap();
        let p = &proof["payload"];
        assert_eq!(p["from"], json!("a@b"));
        assert_eq!(p["subject"], json!("Acto"));
        assert!(p.get("snippet").is_none());
    }

    #[test]
    fn rfc2047_q_decodes_spanish_meeting_subject() {
        let raw = "=?UTF-8?Q?Reuni=C3=B3n_con_Racso_el_21=2F08=2F2026_a_las_10=3A00?=";
        assert_eq!(
            decode_rfc2047(raw),
            "Reunión con Racso el 21/08/2026 a las 10:00"
        );
    }

    #[test]
    fn parse_triage_blob_prefers_json_in_prose() {
        let blob = "ok\n{\"verdict\":\"actionable\",\"title\":\"Reunión\",\"datetime\":\"21/08/2026 10:00\"}\n";
        let v = parse_triage_llm_blob(blob);
        assert_eq!(v["verdict"], json!("actionable"));
        assert_eq!(v["datetime"], json!("21/08/2026 10:00"));
    }

    #[test]
    fn extract_actionable_from_encoded_meeting_subject() {
        let raw = "=?UTF-8?Q?Reuni=C3=B3n_con_Racso_el_21=2F08=2F2026_a_las_10=3A00?=";
        let (title, dt) = extract_actionable_from_subject(raw).expect("extract");
        assert!(title.contains("Reunión"));
        assert!(dt.contains("21/08/2026"));
        assert!(dt.contains("10:00"));
    }

    #[test]
    fn infer_degraded_when_require_zero_tokens_subject_without_date() {
        let (v, _, dt, elev) =
            maybe_elevate_from_subject("passive", None, None, "Hola equipo, ¿cómo va?");
        assert_eq!(v, "passive");
        assert!(dt.is_none());
        assert!(!elev);
        let mut extras = json!({});
        mark_classification_degraded(&mut extras, true, 0, 0, elev);
        assert_eq!(extras["classification-degraded"], json!(true));
    }

    #[test]
    fn infer_not_degraded_when_subject_elevates() {
        let (v, _, _, elev) = maybe_elevate_from_subject(
            "passive",
            None,
            None,
            "Reunión con Racso el 25/08/2026 a las 10:00",
        );
        assert_eq!(v, "actionable");
        assert!(elev);
        let mut extras = json!({});
        mark_classification_degraded(&mut extras, true, 0, 0, elev);
        assert!(extras.get("classification-degraded").is_none());
    }

    #[test]
    fn infer_not_degraded_when_tokens_nonzero() {
        let mut extras = json!({});
        mark_classification_degraded(&mut extras, true, 12, 4, false);
        assert!(extras.get("classification-degraded").is_none());
    }

    #[test]
    fn llm_passive_meeting_subject_elevates_to_actionable() {
        let (v, title, dt, elev) = maybe_elevate_from_subject(
            "passive",
            None,
            None,
            "Reunión con Racso el 25/08/2026 a las 10:00",
        );
        assert_eq!(v, "actionable");
        assert!(elev);
        assert!(title.unwrap().contains("Reunión"));
        assert!(dt.unwrap().contains("25/08/2026"));
    }

    #[test]
    fn llm_output_blob_reads_data_text() {
        let body = json!({"success": true, "data": {"text": "{\"verdict\":\"passive\"}"}});
        assert!(llm_output_blob(&body).contains("passive"));
        let parsed = parse_triage_llm_blob(&llm_output_blob(&body));
        assert_eq!(parsed["verdict"], json!("passive"));
    }
}
