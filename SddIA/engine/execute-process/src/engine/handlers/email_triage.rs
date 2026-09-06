//! Handler nativo `email-triage-gateway` (PBI-KALMA2-MVP-01A / PBI-EMAIL-TRIAGE-HEURISTIC).
//! Peaje G5: Triaje-C o mute P cierran sin Clasificacion.

use super::super::capsules::invoke_capsule_json;
use super::super::fractal::{load_fractal_dirs, write_fractal_event};
use super::user_preference::query_context_block_with_capsule_fallback;
use crate::envelope::OrchestratorEnvelope;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use user_preference_core::{
    canonical_subject_key_from_addr, query, PreferenceAuthority, PreferenceStatus, QuerySpec,
    UserPreference,
};
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

fn p_exempt_c(prefs: &[UserPreference]) -> bool {
    prefs.iter().any(|p| {
        p.status == PreferenceStatus::Active
            && p.authority == PreferenceAuthority::ExplicitUser
            && p.predicate == "priority"
            && matches!(
                p.value.get("level").and_then(|v| v.as_str()),
                Some("max") | Some("high")
            )
    })
}

fn mute_until_active(until: Option<&str>) -> bool {
    let Some(u) = until.map(str::trim).filter(|s| !s.is_empty()) else {
        return true;
    };
    DateTime::parse_from_rfc3339(u)
        .map(|dt| dt.with_timezone(&Utc) > Utc::now())
        .unwrap_or(true)
}

fn p_mute_sender(prefs: &[UserPreference]) -> bool {
    prefs.iter().any(|p| {
        p.status == PreferenceStatus::Active
            && p.predicate == "mute"
            && p.value.get("muted").and_then(|v| v.as_bool()) == Some(true)
            && mute_until_active(p.value.get("until").and_then(|v| v.as_str()))
    })
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

fn classification_prompt(payload: &Value, pref_ctx: &Value) -> String {
    let from_plain = decode_rfc2047(payload.get("from").and_then(|v| v.as_str()).unwrap_or(""));
    let subject_plain = decode_rfc2047(payload.get("subject").and_then(|v| v.as_str()).unwrap_or(""));
    let snippet = payload.get("snippet").and_then(|v| v.as_str()).unwrap_or("");
    let mut prompt = format!(
        "Clasifica este correo como noise, passive o actionable. JSON estricto {{\"verdict\":\"...\",\"title\":null,\"datetime\":null}}. Reunión o cita con fecha extraíble en el asunto es candidato actionable (datetime obligatorio). No uses verbosidad ni urgencia comercial para elevar a actionable. from={} subject={} snippet={}",
        from_plain, subject_plain, snippet,
    );
    if pref_ctx
        .get("preferences")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        == Some(true)
    {
        prompt.push_str(&format!(" user_preference_context={pref_ctx}"));
    }
    prompt
}

fn classify_llm(
    repo: &Path,
    payload: &Value,
    pref_ctx: &Value,
) -> Result<(String, Option<String>, Option<String>, Value, Value), String> {
    let started = std::time::Instant::now();
    let subject_plain = decode_rfc2047(payload.get("subject").and_then(|v| v.as_str()).unwrap_or(""));
    let prompt = classification_prompt(payload, pref_ctx);
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
    let from_decoded = decode_rfc2047(payload.get("from").and_then(|v| v.as_str()).unwrap_or(""));
    let spec = QuerySpec {
        subject_key: Some(canonical_subject_key_from_addr(&from_decoded)),
        include_proposed: Some(false),
        max_results: Some(8),
        ..Default::default()
    };
    let pref_ctx = query_context_block_with_capsule_fallback(repo, &spec);
    let prefs = query(repo, &spec).unwrap_or_default();
    phases.push(json!({
        "phase_name": "Triaje-P",
        "status": "executed",
        "preference_hits": prefs.len(),
    }));

    let exempt = p_exempt_c(&prefs);
    let c = if exempt {
        phases.push(json!({
            "phase_name": "Triaje-C",
            "status": "skipped",
            "reason": "P-EXEMPT-C",
        }));
        TriageC {
            concluded: false,
            verdict: None,
            matched_rule: None,
        }
    } else {
        let c = triaje_c(&payload);
        if c.concluded {
            phases.push(json!({
                "phase_name": "Triaje-C",
                "status": "executed",
                "matched_rule": c.matched_rule,
                "verdict": c.verdict,
            }));
        } else {
            phases.push(json!({
                "phase_name": "Triaje-C",
                "status": "executed",
                "concluded": false,
            }));
        }
        c
    };

    let mut classification_ran = false;

    let (verdict, decision_path, matched_rule, cost, agenda_id, extras) = if c.concluded {
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
    } else if p_mute_sender(&prefs) {
        phases.push(json!({
            "phase_name": "Clasificacion",
            "status": "skipped",
            "reason": "p-mute-sender",
        }));
        phases.push(json!({
            "phase_name": "Asiento-Agenda",
            "status": "skipped",
            "reason": "not-actionable",
        }));
        (
            "noise".to_string(),
            "preference".to_string(),
            Some("P-MUTE-SENDER"),
            zeros_cost(),
            None,
            json!({}),
        )
    } else {
        classification_ran = true;
        let classified = classify_llm(repo, &payload, &pref_ctx);
        match classified {
            Ok((verdict, title, datetime, cost, mut extras)) => {
                phases.push(json!({
                    "phase_name": "Clasificacion",
                    "status": "executed",
                    "verdict": verdict,
                }));
                if exempt {
                    extras["exempt_rule"] = json!("P-EXEMPT-C");
                }
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

    #[test]
    fn email_triage_does_not_invoke_iota_publisher() {
        let src = include_str!("email_triage.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap_or(src);
        assert!(!prod.contains("iota-immutable-publisher"));
        assert!(!prod.contains("invoke_iota_publisher"));
        assert!(!prod.contains("publish_immutable_data"));
        assert!(!prod.to_ascii_lowercase().contains("expunge"));
        assert!(!prod.contains("UID STORE"));
    }

    fn setup_triage_repo(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join(".events/domain")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"userPreferencesStore":".SddIA/vector_store/user_preferences"},"eda_fractal":{"domain":".events/domain","orchestration":".events/orchestration","telemetry":".events/telemetry","dead_letter":".events/dead-letter"},"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
    }

    fn write_received(repo: &Path, name: &str, payload: Value) -> PathBuf {
        let path = repo.join(".events/domain").join(format!("{name}.json"));
        let event = json!({
            "event_id": name,
            "event_type": "Email_Received",
            "event_family": "domain",
            "payload": payload,
        });
        fs::write(&path, serde_json::to_string(&event).unwrap()).unwrap();
        path
    }

    fn seed_pref(
        repo: &Path,
        from: &str,
        predicate: &str,
        value: Value,
        authority: PreferenceAuthority,
        status: PreferenceStatus,
    ) {
        let pref = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "person".into(),
            subject_key: canonical_subject_key_from_addr(from),
            predicate: predicate.into(),
            value,
            scope_type: user_preference_core::ScopeType::Channel,
            scope_id: Some("email".into()),
            status,
            authority,
            sensitivity: "personal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({"channel": "email"}),
            recorded_at: String::new(),
        };
        user_preference_core::put_revision(repo, pref).unwrap();
    }

    fn run_received(repo: &Path, name: &str, payload: Value) -> OrchestratorEnvelope {
        let path = write_received(repo, name, payload);
        let rel = path.strip_prefix(repo).unwrap().to_string_lossy().replace('\\', "/");
        run(repo, &json!({"event_file_path": rel})).expect("run")
    }

    #[test]
    fn cold_start_noreply_is_c_noreply_without_llm() {
        let tmp = tempfile::tempdir().unwrap();
        setup_triage_repo(tmp.path());
        let env = run_received(
            tmp.path(),
            "e1",
            json!({"message_uid": "1", "from": "noreply@shop.tld", "subject": "Receipt"}),
        );
        let data = env.data.as_ref().unwrap();
        assert_eq!(data["verdict"], "noise");
        assert_eq!(data["decision_path"], "deterministic");
        assert_eq!(data["classification_ran"], false);
        let phases = env.execution_report.as_ref().unwrap()["phases"].as_array().unwrap();
        assert_eq!(phases[0]["phase_name"], "Triaje-P");
        assert_eq!(phases[1]["status"], "executed");
        assert_eq!(phases[1]["matched_rule"], "C-NOREPLY");
        assert_eq!(phases[2]["status"], "skipped");
        assert_eq!(phases[2]["reason"], "triaje-c-concluded");
    }

    #[test]
    fn mute_active_closes_preference_without_llm() {
        let tmp = tempfile::tempdir().unwrap();
        setup_triage_repo(tmp.path());
        seed_pref(
            tmp.path(),
            "human@example.com",
            "mute",
            json!({"muted": true, "until": null}),
            PreferenceAuthority::ExplicitUser,
            PreferenceStatus::Active,
        );
        let env = run_received(
            tmp.path(),
            "e2",
            json!({"message_uid": "2", "from": "Human <human@example.com>", "subject": "Hola"}),
        );
        let data = env.data.as_ref().unwrap();
        assert_eq!(data["verdict"], "noise");
        assert_eq!(data["decision_path"], "preference");
        assert_eq!(data["classification_ran"], false);
        let proof_dir = tmp.path().join(".SddIA/proofs/email-triaged");
        let proof = fs::read_dir(&proof_dir).unwrap().next().unwrap().unwrap();
        let body: Value = serde_json::from_str(&fs::read_to_string(proof.path()).unwrap()).unwrap();
        assert_eq!(body["payload"]["matched_rule"], "P-MUTE-SENDER");
        assert!(body["payload"].get("snippet").is_none());
    }

    #[test]
    fn list_headers_beat_mute() {
        let tmp = tempfile::tempdir().unwrap();
        setup_triage_repo(tmp.path());
        seed_pref(
            tmp.path(),
            "list@example.com",
            "mute",
            json!({"muted": true}),
            PreferenceAuthority::ExplicitUser,
            PreferenceStatus::Active,
        );
        let env = run_received(
            tmp.path(),
            "e3",
            json!({
                "message_uid": "3",
                "from": "list@example.com",
                "subject": "Weekly",
                "list_headers": ["List-Id: <news.example.com>"]
            }),
        );
        let data = env.data.as_ref().unwrap();
        assert_eq!(data["verdict"], "noise");
        assert_eq!(data["decision_path"], "deterministic");
        assert_eq!(data["classification_ran"], false);
    }

    #[test]
    fn explicit_priority_max_exempts_noreply_wall() {
        let tmp = tempfile::tempdir().unwrap();
        setup_triage_repo(tmp.path());
        seed_pref(
            tmp.path(),
            "noreply@shop.tld",
            "priority",
            json!({"level": "max"}),
            PreferenceAuthority::ExplicitUser,
            PreferenceStatus::Active,
        );
        let env = run_received(
            tmp.path(),
            "e4",
            json!({"message_uid": "4", "from": "noreply@shop.tld", "subject": "Receipt"}),
        );
        let data = env.data.as_ref().unwrap();
        assert_eq!(data["classification_ran"], true);
        assert_ne!(data["decision_path"], "deterministic");
        let phases = env.execution_report.as_ref().unwrap()["phases"].as_array().unwrap();
        assert_eq!(phases[1]["reason"], "P-EXEMPT-C");
        assert_eq!(phases[1]["status"], "skipped");
        assert_ne!(data["verdict"], json!(""));
        let v = data["verdict"].as_str().unwrap();
        assert!(matches!(v, "noise" | "passive" | "actionable"));
    }

    #[test]
    fn inferred_priority_max_does_not_exempt() {
        let tmp = tempfile::tempdir().unwrap();
        setup_triage_repo(tmp.path());
        seed_pref(
            tmp.path(),
            "noreply@shop.tld",
            "priority",
            json!({"level": "max"}),
            PreferenceAuthority::Inferred,
            PreferenceStatus::Active,
        );
        let env = run_received(
            tmp.path(),
            "e5",
            json!({"message_uid": "5", "from": "noreply@shop.tld", "subject": "Receipt"}),
        );
        assert_eq!(env.data.as_ref().unwrap()["decision_path"], "deterministic");
        assert_eq!(env.data.as_ref().unwrap()["classification_ran"], false);
    }

    #[test]
    fn proposed_priority_max_does_not_exempt() {
        let tmp = tempfile::tempdir().unwrap();
        setup_triage_repo(tmp.path());
        seed_pref(
            tmp.path(),
            "noreply@shop.tld",
            "priority",
            json!({"level": "max"}),
            PreferenceAuthority::ExplicitUser,
            PreferenceStatus::Proposed,
        );
        let env = run_received(
            tmp.path(),
            "e6",
            json!({"message_uid": "6", "from": "noreply@shop.tld", "subject": "Receipt"}),
        );
        assert_eq!(env.data.as_ref().unwrap()["classification_ran"], false);
        assert_eq!(env.data.as_ref().unwrap()["decision_path"], "deterministic");
    }

    #[test]
    fn query_subject_key_is_never_plaintext_addr() {
        let from = "Alice <alice@example.com>";
        let key = canonical_subject_key_from_addr(from);
        assert!(!key.contains('@'));
        assert_ne!(key, "alice@example.com");
        let spec = QuerySpec {
            subject_key: Some(key.clone()),
            ..Default::default()
        };
        assert_eq!(spec.subject_key.as_deref(), Some(key.as_str()));
    }

    #[test]
    fn conjugacion_prompt_omitted_when_empty_present_when_partial() {
        let payload = json!({"from": "a@b", "subject": "Hola", "snippet": "x"});
        let empty = json!({"schema_version": "1.0.0", "preferences": []});
        let p0 = classification_prompt(&payload, &empty);
        assert!(!p0.contains("user_preference_context"));
        let partial = json!({
            "schema_version": "1.0.0",
            "preferences": [{"predicate": "attention_window", "value": {"dow": [1]}}]
        });
        let p1 = classification_prompt(&payload, &partial);
        assert!(p1.contains("user_preference_context"));
        assert!(p1.contains("schema_version"));
        assert!(!p1.contains("body"));
        let baseline = classification_prompt(&payload, &json!({}));
        assert_eq!(p0, baseline);
    }

    #[test]
    fn p_exempt_requires_explicit_active_high() {
        let mut pref = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "person".into(),
            subject_key: "k".into(),
            predicate: "priority".into(),
            value: json!({"level": "normal"}),
            scope_type: user_preference_core::ScopeType::Channel,
            scope_id: Some("email".into()),
            status: PreferenceStatus::Active,
            authority: PreferenceAuthority::ExplicitUser,
            sensitivity: "personal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({}),
            recorded_at: String::new(),
        };
        assert!(!p_exempt_c(&[pref.clone()]));
        pref.value = json!({"level": "high"});
        assert!(p_exempt_c(&[pref]));
    }
}
