//! Handler nativo `email-noise-digest` (PBI-EMAIL-NOISE-HEURISTIC-DIGEST).
//! Batch determinista de ruido Triaje-C. Cero LLM. Cero IMAP.

use super::super::capsules::invoke_capsule_json;
use super::super::daemons::{iso_now, state_dir, write_json_atomic};
use crate::envelope::OrchestratorEnvelope;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use user_preference_core::{canonical_subject_key_from_addr, normalize_email_addr};

const C_RULES: &[&str] = &["C-LIST", "C-NOREPLY", "C-SUBJECT-NOISE"];
const STATE_FILE: &str = "email-noise-digest.json";
const MSG_CAP: usize = 4000;
const SUBJECT_MAX: usize = 80;
const MAX_BUTTON_ROWS: usize = 50;
const FOOTER: &str = "⭐ N = priorizar (max) · 🔇 N = silenciar";

#[derive(Clone)]
struct Agg {
    count: u64,
    rule_freq: BTreeMap<String, u64>,
    latest_ts: DateTime<Utc>,
    latest_event_id: String,
    latest_subject: String,
}

struct SenderRow {
    key: String,
    count: u64,
    rule: String,
    subject: String,
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

fn parse_rfc3339(raw: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(raw.trim())
        .map(|d| d.with_timezone(&Utc))
        .map_err(|e| format!("timestamp RFC3339 inválido: {e}"))
}

fn required_rfc3339(inputs: &Value, key: &str) -> Result<DateTime<Utc>, String> {
    let s = inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("{key} requerido"))?;
    parse_rfc3339(s)
}

fn state_path(repo: &Path) -> Result<PathBuf, String> {
    Ok(state_dir(repo)?.join(STATE_FILE))
}

fn load_state(repo: &Path) -> Value {
    let Ok(path) = state_path(repo) else {
        return json!({});
    };
    if !path.is_file() {
        return json!({});
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_else(|| json!({}))
}

fn save_state(
    repo: &Path,
    until: &DateTime<Utc>,
    events_scanned: u64,
    senders: usize,
    notified: bool,
    tokens: Option<Value>,
) -> Result<(), String> {
    let path = state_path(repo)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir state: {e}"))?;
    }
    let tokens_val = match tokens {
        Some(t) => t,
        None => load_state(repo)
            .get("tokens")
            .cloned()
            .unwrap_or_else(|| json!({})),
    };
    let body = json!({
        "last_until": until.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "last_run": iso_now(),
        "last_events_scanned": events_scanned,
        "last_senders_count": senders,
        "last_notified": notified,
        "tokens": tokens_val,
    });
    write_json_atomic(&path, &body)
}

fn skip_envelope() -> OrchestratorEnvelope {
    ok_data(
        json!({
            "success": true,
            "skipped": true,
            "reason": "already-processed",
            "events_scanned": 0,
            "senders": 0,
            "notified": false,
        }),
        vec![json!({
            "phase_name": "Agregacion-Cuarentena",
            "status": "skipped",
            "reason": "already-processed",
        })],
    )
}

fn fail_envelope(error: impl Into<String>, phases: Vec<Value>) -> OrchestratorEnvelope {
    let error = error.into();
    OrchestratorEnvelope {
        success: false,
        status_code: 1,
        data: Some(json!({"success": false, "notified": false})),
        error: Some(error.clone()),
        execution_report: Some(json!({
            "process_name": "email-noise-digest",
            "phases": phases,
        })),
        exit_code: 1,
    }
}

fn ok_data(data: Value, phases: Vec<Value>) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(data),
        error: None,
        execution_report: Some(json!({
            "process_name": "email-noise-digest",
            "phases": phases,
        })),
        exit_code: 0,
    }
}

fn proof_matches(
    proof: &Value,
    effective_since: DateTime<Utc>,
    until: DateTime<Utc>,
) -> Option<(DateTime<Utc>, String)> {
    if proof.get("kind").and_then(|v| v.as_str()) != Some("email-triaged-proof") {
        return None;
    }
    if proof.get("event_type").and_then(|v| v.as_str()) != Some("Email_Triaged") {
        return None;
    }
    let ts_raw = proof.get("timestamp").and_then(|v| v.as_str())?;
    let ts = parse_rfc3339(ts_raw).ok()?;
    if ts < effective_since || ts >= until {
        return None;
    }
    let payload = proof.get("payload")?;
    if payload.get("verdict").and_then(|v| v.as_str()) != Some("noise") {
        return None;
    }
    if payload.get("decision_path").and_then(|v| v.as_str()) != Some("deterministic") {
        return None;
    }
    let rule = payload.get("matched_rule").and_then(|v| v.as_str())?;
    if !C_RULES.contains(&rule) {
        return None;
    }
    let event_id = proof
        .get("event_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    Some((ts, event_id))
}

fn sender_key(payload: &Value) -> String {
    let from = payload
        .get("from")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let n = normalize_email_addr(from);
    if n.is_empty() {
        "_unknown".into()
    } else {
        n
    }
}

fn subject_of(payload: &Value) -> String {
    payload
        .get("subject")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("(sin asunto)")
        .to_string()
}

fn moda_rule(freq: &BTreeMap<String, u64>) -> String {
    let mut best: Option<(&str, u64)> = None;
    for (rule, n) in freq {
        match best {
            None => best = Some((rule, *n)),
            Some((_, bn)) if *n > bn => best = Some((rule, *n)),
            Some((br, bn)) if *n == bn && rule.as_str() < br => best = Some((rule, *n)),
            _ => {}
        }
    }
    best.map(|(r, _)| r.to_string())
        .unwrap_or_else(|| "C-LIST".into())
}

fn collect_rows(repo: &Path, effective_since: DateTime<Utc>, until: DateTime<Utc>) -> Vec<SenderRow> {
    let dir = proofs_root(repo).join("email-triaged");
    let mut aggs: BTreeMap<String, Agg> = BTreeMap::new();
    let Ok(rd) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    for ent in rd.flatten() {
        let path = ent.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(raw) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(proof) = serde_json::from_str::<Value>(&raw) else {
            continue;
        };
        let Some((ts, event_id)) = proof_matches(&proof, effective_since, until) else {
            continue;
        };
        let payload = proof.get("payload").cloned().unwrap_or(json!({}));
        let key = sender_key(&payload);
        let rule = payload
            .get("matched_rule")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let subject = subject_of(&payload);
        let entry = aggs.entry(key).or_insert_with(|| Agg {
            count: 0,
            rule_freq: BTreeMap::new(),
            latest_ts: ts,
            latest_event_id: event_id.clone(),
            latest_subject: subject.clone(),
        });
        entry.count += 1;
        *entry.rule_freq.entry(rule).or_insert(0) += 1;
        if ts > entry.latest_ts || (ts == entry.latest_ts && event_id < entry.latest_event_id) {
            entry.latest_ts = ts;
            entry.latest_event_id = event_id;
            entry.latest_subject = subject;
        }
    }
    let mut rows: Vec<SenderRow> = aggs
        .into_iter()
        .map(|(key, agg)| SenderRow {
            rule: moda_rule(&agg.rule_freq),
            subject: agg.latest_subject,
            count: agg.count,
            key,
        })
        .collect();
    rows.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.key.cmp(&b.key)));
    rows
}

fn truncate_subject(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= SUBJECT_MAX {
        return s.to_string();
    }
    format!("{}…", chars.into_iter().take(SUBJECT_MAX).collect::<String>())
}

fn sender_line(idx: usize, row: &SenderRow) -> String {
    format!(
        "{}. {} ({}, {}) {}",
        idx,
        row.key,
        row.count,
        row.rule,
        truncate_subject(&row.subject)
    )
}

fn format_digest_message(
    since_date: &str,
    events_scanned: u64,
    rows: &[SenderRow],
) -> (String, usize) {
    let header = format!(
        "Ruido Triaje-C {since_date}\neventos={events_scanned} remitentes={}",
        rows.len()
    );
    let unique = rows.len();
    if unique == 0 {
        return (format!("{header}\n{FOOTER}"), 0);
    }
    let cap_k = unique.min(MAX_BUTTON_ROWS);
    let lines: Vec<String> = rows
        .iter()
        .take(cap_k)
        .enumerate()
        .map(|(i, row)| sender_line(i + 1, row))
        .collect();
    let all = if cap_k < unique {
        format!(
            "{header}\n{}\n+ {} remitentes omitidos\n{FOOTER}",
            lines.join("\n"),
            unique - cap_k
        )
    } else {
        format!("{header}\n{}\n{FOOTER}", lines.join("\n"))
    };
    if all.chars().count() <= MSG_CAP {
        return (all, cap_k);
    }
    for k in (0..cap_k).rev() {
        let omitted = unique - k;
        let body = if k == 0 {
            String::new()
        } else {
            format!("\n{}", lines[..k].join("\n"))
        };
        let omission = format!("\n+ {omitted} remitentes omitidos");
        let candidate = format!("{header}{body}{omission}\n{FOOTER}");
        if candidate.chars().count() <= MSG_CAP {
            return (candidate, k);
        }
    }
    (
        format!("{header}\n+ {unique} remitentes omitidos\n{FOOTER}"),
        0,
    )
}

fn assign_tokens(rows: &[SenderRow]) -> Vec<(String, String)> {
    let mut used = HashSet::new();
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let subject_key = canonical_subject_key_from_addr(&row.key);
        let t8: String = subject_key.chars().take(8).collect();
        let token = if used.contains(&t8) {
            subject_key.chars().take(12).collect::<String>()
        } else {
            t8
        };
        if token.len() < 8 || used.contains(&token) {
            out.push((String::new(), subject_key));
            continue;
        }
        used.insert(token.clone());
        out.push((token, subject_key));
    }
    out
}

fn tokens_map(listed: &[SenderRow], assigned: &[(String, String)]) -> Value {
    let mut map = serde_json::Map::new();
    let created = iso_now();
    for (row, (token, subject_key)) in listed.iter().zip(assigned.iter()) {
        if token.is_empty() {
            continue;
        }
        map.insert(
            token.clone(),
            json!({
                "subject_key": subject_key,
                "rule": row.rule,
                "created_at": created,
            }),
        );
    }
    Value::Object(map)
}

fn build_reply_markup(assigned: &[(String, String)]) -> Option<Value> {
    let mut keyboard = Vec::new();
    for (i, (token, _)) in assigned.iter().enumerate() {
        if token.is_empty() {
            continue;
        }
        let n = i + 1;
        let max_cb = format!("dpref:max:{token}");
        let mute_cb = format!("dpref:mute:{token}");
        if max_cb.len() > 64 || mute_cb.len() > 64 {
            continue;
        }
        keyboard.push(json!([
            {"text": format!("⭐ {n}"), "callback_data": max_cb},
            {"text": format!("🔇 {n}"), "callback_data": mute_cb}
        ]));
    }
    if keyboard.is_empty() {
        None
    } else {
        Some(json!({"inline_keyboard": keyboard}))
    }
}

fn default_notify(
    repo: &Path,
    message: &str,
    reply_markup: Option<&Value>,
) -> Result<Value, String> {
    let mut payload = json!({
        "message": message,
        "parse_mode": Value::Null,
    });
    if let Some(rm) = reply_markup {
        payload["reply_markup"] = rm.clone();
    }
    let inv = invoke_capsule_json(repo, "send-telegram-notification", &payload, false)?;
    if inv.body.get("success") == Some(&json!(true)) {
        return Ok(inv.body);
    }
    let err = inv
        .body
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("send-telegram-notification failed");
    Err(err.to_string())
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    run_with_notify(repo, process_inputs, default_notify)
}

fn run_with_notify<F>(
    repo: &Path,
    process_inputs: &Value,
    mut notify: F,
) -> Result<OrchestratorEnvelope, String>
where
    F: FnMut(&Path, &str, Option<&Value>) -> Result<Value, String>,
{
    let since = match required_rfc3339(process_inputs, "since") {
        Ok(v) => v,
        Err(e) => return Ok(fail_envelope(e, vec![])),
    };
    let until = match required_rfc3339(process_inputs, "until") {
        Ok(v) => v,
        Err(e) => return Ok(fail_envelope(e, vec![])),
    };
    if since >= until {
        return Ok(fail_envelope("since debe ser < until", vec![]));
    }

    let state = load_state(repo);
    let last_until = state
        .get("last_until")
        .and_then(|v| v.as_str())
        .and_then(|s| parse_rfc3339(s).ok());
    if let Some(lu) = last_until {
        if until <= lu {
            return Ok(skip_envelope());
        }
    }
    let effective_since = match last_until {
        Some(lu) if since < lu && lu < until => lu,
        _ => since,
    };

    let rows = collect_rows(repo, effective_since, until);
    let events_scanned: u64 = rows.iter().map(|r| r.count).sum();
    let senders = rows.len();
    let since_date = since.format("%Y-%m-%d").to_string();

    let mut phases = vec![json!({
        "phase_name": "Agregacion-Cuarentena",
        "status": "executed",
        "events_scanned": events_scanned,
        "senders": senders,
        "effective_since": effective_since.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    })];

    if events_scanned == 0 {
        if let Err(e) = save_state(repo, &until, 0, 0, false, None) {
            return Ok(fail_envelope(e, phases));
        }
        phases.push(json!({
            "phase_name": "Notificacion-Digest",
            "status": "skipped",
            "reason": "empty-window",
        }));
        return Ok(ok_data(
            json!({
                "success": true,
                "skipped": false,
                "events_scanned": 0,
                "senders": 0,
                "notified": false,
            }),
            phases,
        ));
    }

    let (message, listed_k) = format_digest_message(&since_date, events_scanned, &rows);
    let listed = &rows[..listed_k];
    let assigned = assign_tokens(listed);
    let token_map = tokens_map(listed, &assigned);
    let markup = build_reply_markup(&assigned);
    match notify(repo, &message, markup.as_ref()) {
        Ok(_) => {
            if let Err(e) = save_state(repo, &until, events_scanned, senders, true, Some(token_map)) {
                return Ok(fail_envelope(e, phases));
            }
            phases.push(json!({
                "phase_name": "Notificacion-Digest",
                "status": "executed",
                "parse_mode": Value::Null,
                "chars": message.chars().count(),
                "buttons": listed_k,
            }));
            Ok(ok_data(
                json!({
                    "success": true,
                    "skipped": false,
                    "events_scanned": events_scanned,
                    "senders": senders,
                    "notified": true,
                }),
                phases,
            ))
        }
        Err(e) => {
            phases.push(json!({
                "phase_name": "Notificacion-Digest",
                "status": "failed",
                "error": e,
            }));
            Ok(fail_envelope("send-telegram-notification failed", phases))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_with_cumulo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
  "eda_instance": {"proofs": ".SddIA/proofs"},
  "daemons_instance": {"state": ".SddIA/daemons/state"}
}"#,
        )
        .unwrap();
        tmp
    }

    fn write_proof(
        repo: &Path,
        event_id: &str,
        ts: &str,
        verdict: &str,
        path: &str,
        rule: &str,
        from: &str,
        subject: &str,
    ) {
        let dir = repo.join(".SddIA/proofs/email-triaged");
        fs::create_dir_all(&dir).unwrap();
        let body = json!({
            "kind": "email-triaged-proof",
            "event_id": event_id,
            "event_type": "Email_Triaged",
            "timestamp": ts,
            "payload": {
                "verdict": verdict,
                "decision_path": path,
                "matched_rule": rule,
                "from": from,
                "subject": subject,
            }
        });
        fs::write(dir.join(format!("{event_id}.json")), format!("{body}\n")).unwrap();
    }

    fn ok_notify(_repo: &Path, _msg: &str, _markup: Option<&Value>) -> Result<Value, String> {
        Ok(json!({"success": true}))
    }

    fn fail_notify(_repo: &Path, _msg: &str, _markup: Option<&Value>) -> Result<Value, String> {
        Err("boom".into())
    }

    #[test]
    fn email_noise_digest_filters_and_normalizes() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        write_proof(
            repo,
            "a1",
            "2026-09-06T10:00:00Z",
            "noise",
            "deterministic",
            "C-NOREPLY",
            "Shop <noreply@shop.tld>",
            "Hello",
        );
        write_proof(
            repo,
            "a2",
            "2026-09-06T11:00:00Z",
            "noise",
            "deterministic",
            "C-NOREPLY",
            "noreply@shop.tld",
            "Later",
        );
        write_proof(
            repo,
            "b1",
            "2026-09-06T10:30:00Z",
            "noise",
            "preference",
            "P-MUTE-SENDER",
            "mute@x.tld",
            "Muted",
        );
        write_proof(
            repo,
            "c1",
            "2026-09-06T10:40:00Z",
            "actionable",
            "llm",
            "C-NOREPLY",
            "real@x.tld",
            "Meet",
        );
        write_proof(
            repo,
            "d1",
            "2026-09-05T10:00:00Z",
            "noise",
            "deterministic",
            "C-LIST",
            "old@x.tld",
            "Old",
        );
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-07T00:00:00Z"
            }),
            ok_notify,
        )
        .unwrap();
        assert!(env.success);
        let data = env.data.unwrap();
        assert_eq!(data["events_scanned"], json!(2));
        assert_eq!(data["senders"], json!(1));
        assert_eq!(data["notified"], json!(true));
    }

    #[test]
    fn email_noise_digest_tie_break_alpha_and_rule() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        write_proof(
            repo,
            "z1",
            "2026-09-06T10:00:00Z",
            "noise",
            "deterministic",
            "C-SUBJECT-NOISE",
            "zeta@x.tld",
            "Z",
        );
        write_proof(
            repo,
            "a1",
            "2026-09-06T10:00:00Z",
            "noise",
            "deterministic",
            "C-LIST",
            "alpha@x.tld",
            "A",
        );
        write_proof(
            repo,
            "a2",
            "2026-09-06T11:00:00Z",
            "noise",
            "deterministic",
            "C-NOREPLY",
            "alpha@x.tld",
            "A2",
        );
        let captured = std::sync::Mutex::new(String::new());
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-07T00:00:00Z"
            }),
            |_, msg, _| {
                *captured.lock().unwrap() = msg.to_string();
                Ok(json!({"success": true}))
            },
        )
        .unwrap();
        assert!(env.success);
        let msg = captured.lock().unwrap().clone();
        let pos_a = msg.find("1. alpha@x.tld").unwrap();
        let pos_z = msg.find("2. zeta@x.tld").unwrap();
        assert!(pos_a < pos_z);
        assert!(msg.contains("1. alpha@x.tld (2, C-LIST) A2") || msg.contains("C-LIST"));
        assert!(msg.contains("A2"));
    }

    #[test]
    fn email_noise_digest_empty_updates_cursor_no_notify() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        let mut notified = false;
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-07T00:00:00Z"
            }),
            |_, _, _| {
                notified = true;
                Ok(json!({"success": true}))
            },
        )
        .unwrap();
        assert!(env.success);
        assert_eq!(env.data.as_ref().unwrap()["notified"], json!(false));
        assert_eq!(env.data.as_ref().unwrap()["events_scanned"], json!(0));
        assert!(!notified);
        let st = load_state(repo);
        assert_eq!(st["last_until"], json!("2026-09-07T00:00:00Z"));
    }

    #[test]
    fn email_noise_digest_skip_and_clamp() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        save_state(
            repo,
            &parse_rfc3339("2026-09-07T00:00:00Z").unwrap(),
            1,
            1,
            true,
            Some(json!({"deadbeef": {"subject_key": "aa", "rule": "C-LIST", "created_at": "2026-09-07T00:00:00Z"}})),
        )
        .unwrap();
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-07T00:00:00Z"
            }),
            ok_notify,
        )
        .unwrap();
        assert_eq!(env.data.unwrap()["skipped"], json!(true));
        assert_eq!(load_state(repo)["tokens"]["deadbeef"]["rule"], json!("C-LIST"));

        write_proof(
            repo,
            "n1",
            "2026-09-06T10:00:00Z",
            "noise",
            "deterministic",
            "C-LIST",
            "old@x.tld",
            "Old",
        );
        write_proof(
            repo,
            "n2",
            "2026-09-07T10:00:00Z",
            "noise",
            "deterministic",
            "C-LIST",
            "new@x.tld",
            "New",
        );
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-08T00:00:00Z"
            }),
            ok_notify,
        )
        .unwrap();
        let data = env.data.unwrap();
        assert_eq!(data["events_scanned"], json!(1));
        assert_eq!(data["senders"], json!(1));
    }

    #[test]
    fn email_noise_digest_notify_failure_keeps_cursor() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        write_proof(
            repo,
            "n1",
            "2026-09-06T10:00:00Z",
            "noise",
            "deterministic",
            "C-LIST",
            "a@x.tld",
            "S",
        );
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-07T00:00:00Z"
            }),
            fail_notify,
        )
        .unwrap();
        assert!(!env.success);
        assert!(!state_path(repo).unwrap().is_file());
    }

    #[test]
    fn email_noise_digest_truncates_under_cap() {
        let rows: Vec<SenderRow> = (0..80)
            .map(|i| SenderRow {
                key: format!("sender{i:02}@example.tld"),
                count: 100 - i,
                rule: "C-NOREPLY".into(),
                subject: "x".repeat(90),
            })
            .collect();
        let (msg, k) = format_digest_message("2026-09-06", 400, &rows);
        assert!(msg.chars().count() <= MSG_CAP);
        assert!(msg.contains("remitentes omitidos"));
        assert!(msg.contains(FOOTER));
        assert!(!msg.contains("parse_mode"));
        assert!(k <= MAX_BUTTON_ROWS);
        assert!(k < rows.len());
    }

    #[test]
    fn email_noise_digest_unknown_from() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        let dir = repo.join(".SddIA/proofs/email-triaged");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("u1.json"),
            r#"{"kind":"email-triaged-proof","event_id":"u1","event_type":"Email_Triaged","timestamp":"2026-09-06T10:00:00Z","payload":{"verdict":"noise","decision_path":"deterministic","matched_rule":"C-LIST"}}"#,
        )
        .unwrap();
        let captured = std::sync::Mutex::new(String::new());
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-07T00:00:00Z"
            }),
            |_, msg, _| {
                *captured.lock().unwrap() = msg.to_string();
                Ok(json!({"success": true}))
            },
        )
        .unwrap();
        assert!(env.success);
        assert!(captured.lock().unwrap().contains("_unknown"));
    }

    #[test]
    fn email_noise_digest_markup_tokens_no_pii() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        write_proof(
            repo,
            "n1",
            "2026-09-06T10:00:00Z",
            "noise",
            "deterministic",
            "C-NOREPLY",
            "noreply@shop.tld",
            "Hi",
        );
        let captured = std::sync::Mutex::new(None);
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-07T00:00:00Z"
            }),
            |_, _, markup| {
                *captured.lock().unwrap() = markup.cloned();
                Ok(json!({"success": true}))
            },
        )
        .unwrap();
        assert!(env.success);
        let markup = captured.lock().unwrap().clone().expect("markup");
        let rows = markup["inline_keyboard"].as_array().unwrap();
        assert_eq!(rows.len(), 1);
        let max_cb = rows[0][0]["callback_data"].as_str().unwrap();
        let mute_cb = rows[0][1]["callback_data"].as_str().unwrap();
        assert!(max_cb.starts_with("dpref:max:"));
        assert!(mute_cb.starts_with("dpref:mute:"));
        assert!(max_cb.len() <= 64);
        assert!(mute_cb.len() <= 64);
        assert!(!max_cb.contains('@'));
        assert!(!max_cb.contains("noreply"));
        let st = load_state(repo);
        let tokens = st["tokens"].as_object().unwrap();
        assert_eq!(tokens.len(), 1);
        let entry = tokens.values().next().unwrap();
        let sk = entry["subject_key"].as_str().unwrap();
        assert_eq!(sk.len(), 64);
        assert!(!serde_json::to_string(&st).unwrap().contains("noreply@"));
    }

    #[test]
    fn email_noise_digest_empty_preserves_tokens() {
        let tmp = repo_with_cumulo();
        let repo = tmp.path();
        save_state(
            repo,
            &parse_rfc3339("2026-09-05T00:00:00Z").unwrap(),
            1,
            1,
            true,
            Some(json!({"abcdef01": {"subject_key": "aa", "rule": "C-LIST", "created_at": "2026-09-05T00:00:00Z"}})),
        )
        .unwrap();
        let env = run_with_notify(
            repo,
            &json!({
                "since": "2026-09-06T00:00:00Z",
                "until": "2026-09-07T00:00:00Z"
            }),
            ok_notify,
        )
        .unwrap();
        assert_eq!(env.data.unwrap()["notified"], json!(false));
        assert_eq!(load_state(repo)["tokens"]["abcdef01"]["rule"], json!("C-LIST"));
    }
}
