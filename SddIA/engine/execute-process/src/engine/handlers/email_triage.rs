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
    payload
        .get("from")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn subject_of(payload: &Value) -> String {
    payload
        .get("subject")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_ascii_lowercase()
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

fn classify_llm(repo: &Path, payload: &Value) -> Result<(String, Option<String>, Option<String>, Value), String> {
    let started = std::time::Instant::now();
    let prompt = format!(
        "Clasifica este correo como noise, passive o actionable. JSON estricto {{\"verdict\":\"...\",\"title\":null,\"datetime\":null}}. No uses verbosidad ni urgencia comercial para elevar a actionable. from={} subject={} snippet={}",
        payload.get("from").and_then(|v| v.as_str()).unwrap_or(""),
        payload.get("subject").and_then(|v| v.as_str()).unwrap_or(""),
        payload.get("snippet").and_then(|v| v.as_str()).unwrap_or(""),
    );
    let body = invoke_capsule_json(
        repo,
        "mayeuta-llm",
        &json!({"operation": "SYNTHESIZE", "prompt": prompt}),
        false,
    )?
    .body;
    let text = body
        .get("data")
        .and_then(|d| d.get("result"))
        .and_then(|v| v.as_str())
        .or_else(|| body.get("result").and_then(|v| v.as_str()))
        .unwrap_or("");
    let parsed: Value = serde_json::from_str(text.trim()).unwrap_or(json!({}));
    let mut verdict = parsed
        .get("verdict")
        .and_then(|v| v.as_str())
        .unwrap_or("passive")
        .to_ascii_lowercase();
    if !matches!(verdict.as_str(), "noise" | "passive" | "actionable") {
        verdict = "passive".into();
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
    if verdict == "actionable" && datetime.is_none() {
        verdict = "passive".into();
    }
    let cost = json!({
        "tokens_in": body.get("tokens_in").and_then(|v| v.as_u64()).unwrap_or(0),
        "tokens_out": body.get("tokens_out").and_then(|v| v.as_u64()).unwrap_or(0),
        "duration_ms": started.elapsed().as_millis() as u64,
    });
    Ok((verdict, title, datetime, cost))
}

fn emit_triaged(
    repo: &Path,
    src: &Value,
    verdict: &str,
    decision_path: &str,
    matched_rule: Option<&str>,
    cost: &Value,
    agenda_entry_id: Option<&str>,
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
    if let Some(from) = src
        .get("from")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        payload["from"] = json!(from);
    }
    if let Some(subject) = src
        .get("subject")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        payload["subject"] = json!(subject);
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

    let (verdict, decision_path, matched_rule, cost, agenda_id) = if c.concluded {
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
            Ok((verdict, title, datetime, cost)) => {
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
                (verdict, "llm".to_string(), None, cost, agenda_id)
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
        let seal = emit_triaged(repo, &src, "actionable", "deterministic", None, &zeros_cost(), None)
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
}
