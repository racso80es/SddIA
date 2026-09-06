//! Handler nativo `notify-humanized-pr-merged` — estático ola 1 + síntesis Gemini fail-soft.

use super::capsules::{invoke_capsule_json, invoke_git_manager, invoke_tool_capsule_json};
use serde_json::{json, Value};
use std::env;
use std::path::Path;

pub const ACTION_NAME: &str = "notify-humanized-pr-merged";
const SYNTHESIS_MAX_CHARS: usize = 400;
const PROMPT_KERNEL: &str =
    "[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. MAX 2 LINES]";

#[derive(Debug, PartialEq, Eq)]
pub enum NotifyOutcome {
    Sent { synthesized: bool },
    SkippedEmpty,
}

fn payload_str(payload: Option<&serde_json::Map<String, Value>>, key: &str) -> String {
    payload
        .and_then(|p| p.get(key))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("")
        .to_string()
}

/// Bloque estático `PullRequest_Merged` (misma semántica que el compositor ola 1).
pub fn pr_merged_static_message(event: &Value) -> Option<String> {
    if event.get("event_type").and_then(|v| v.as_str()) != Some("PullRequest_Merged") {
        return None;
    }
    let payload = event.get("payload").and_then(|v| v.as_object());
    let branch = payload
        .and_then(|p| p.get("source_branch"))
        .and_then(|v| v.as_str())
        .unwrap_or("?");
    let hash = payload
        .and_then(|p| p.get("merge_commit_hash"))
        .and_then(|v| v.as_str())
        .map(|h| &h[..7.min(h.len())])
        .unwrap_or("?");
    let author = payload
        .and_then(|p| p.get("author"))
        .and_then(|v| v.as_str())
        .unwrap_or("?");
    let target = payload
        .and_then(|p| p.get("target_branch"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("main");
    let mut lines = vec![
        format!("✅ PR Fusionado — {branch}"),
        "━━━━━━━━━━━━━━━━━━━━━━━━".into(),
        format!("📦 Commit: {hash} ({target})"),
        format!("👤 Integrador: {author}"),
    ];
    if let Some(sc) = payload.and_then(|p| p.get("security_clearance")) {
        let auditor = sc.get("auditor").and_then(|v| v.as_str()).unwrap_or("?");
        let policy = sc
            .get("policy_applied")
            .and_then(|v| v.as_str())
            .unwrap_or("?");
        lines.push(format!("🔐 Auditor: {auditor} · {policy}"));
    }
    if let Some(url) = payload
        .and_then(|p| p.get("pr_url"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        lines.push(url.to_string());
    }
    if let Some(cid) = event
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .filter(|s| s.len() >= 8)
    {
        lines.push(format!("🔗 Correlación: {}…", &cid[..8]));
    }
    Some(lines.join("\n"))
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommitSummary {
    pub subject: String,
    pub files: Vec<String>,
    pub total_files_changed: usize,
    pub truncated: bool,
}

pub fn commit_summary_from_data(data: &Value) -> Option<CommitSummary> {
    let files = data
        .get("files")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let subject = data
        .get("subject")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("")
        .to_string();
    let total = data
        .get("totalFilesChanged")
        .and_then(|v| v.as_u64())
        .unwrap_or(files.len() as u64) as usize;
    let truncated = data
        .get("truncated")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if subject.is_empty() && files.is_empty() && !truncated {
        return None;
    }
    Some(CommitSummary {
        subject,
        files,
        total_files_changed: total,
        truncated,
    })
}

pub fn try_commit_summary(repo: &Path, merge_hash: &str) -> Option<CommitSummary> {
    let hash = merge_hash.trim();
    if hash.is_empty() {
        return None;
    }
    match invoke_git_manager(
        repo,
        "commit_summary",
        &json!({
            "ref": hash,
            "max_files": 30,
            "max_subject_chars": 200
        }),
    ) {
        Ok(data) => commit_summary_from_data(&data),
        Err(_) => None,
    }
}

pub fn build_synthesis_prompt(event: &Value) -> String {
    build_synthesis_prompt_with_git(event, None)
}

pub fn build_synthesis_prompt_with_git(event: &Value, git: Option<&CommitSummary>) -> String {
    let payload = event.get("payload").and_then(|v| v.as_object());
    let sc = payload.and_then(|p| p.get("security_clearance"));
    let auditor = sc
        .and_then(|v| v.get("auditor"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let policy = sc
        .and_then(|v| v.get("policy_applied"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mut ctx = vec![
        format!(
            "source_branch={}",
            payload_str(payload, "source_branch")
        ),
        format!(
            "target_branch={}",
            payload_str(payload, "target_branch")
        ),
        format!(
            "merge_commit_hash={}",
            payload_str(payload, "merge_commit_hash")
        ),
        format!("author={}", payload_str(payload, "author")),
        format!("auditor={auditor}"),
        format!("policy={policy}"),
    ];
    let pr_url = payload_str(payload, "pr_url");
    if !pr_url.is_empty() {
        ctx.push(format!("pr_url={pr_url}"));
    }
    let repo = payload_str(payload, "repository_name");
    if !repo.is_empty() {
        ctx.push(format!("repository_name={repo}"));
    }
    if let Some(g) = git {
        if !g.subject.is_empty() {
            ctx.push(format!("SUBJECT: {}", g.subject));
        }
        if !g.files.is_empty() {
            ctx.push(format!("FILES: {}", g.files.join(", ")));
        }
        if g.truncated {
            ctx.push(format!(
                "truncated=true totalFilesChanged={}",
                g.total_files_changed
            ));
        }
    }
    format!(
        "{PROMPT_KERNEL}\nReturn only business value of this merge. Do not restate hash, auditor, branch, or correlation.\nDo not invent files, commits, or intent absent from CONTEXT.\nCONTEXT:\n{}",
        ctx.join("\n")
    )
}

pub fn truncate_synthesis(text: &str) -> String {
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .take(2)
        .collect();
    let joined = lines.join("\n");
    if joined.chars().count() <= SYNTHESIS_MAX_CHARS {
        joined
    } else {
        joined.chars().take(SYNTHESIS_MAX_CHARS).collect()
    }
}

pub fn assemble_humanized_message(static_msg: &str, synthesis: Option<&str>) -> String {
    match synthesis.map(str::trim).filter(|s| !s.is_empty()) {
        Some(s) => format!("{static_msg}\n\n🧠 Síntesis de Valor: {s}"),
        None => static_msg.to_string(),
    }
}

fn synthesis_from_capsule(body: &Value) -> Option<String> {
    let text = body
        .pointer("/result/text")
        .or_else(|| body.get("text"))
        .or_else(|| body.pointer("/result/result/text"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())?;
    let cut = truncate_synthesis(text);
    if cut.is_empty() {
        None
    } else {
        Some(cut)
    }
}

pub fn try_infer_synthesis(repo: &Path, event: &Value) -> Option<String> {
    let merge_hash = payload_str(
        event.get("payload").and_then(|v| v.as_object()),
        "merge_commit_hash",
    );
    let git = try_commit_summary(repo, &merge_hash);
    let mut req = json!({
        "prompt": build_synthesis_prompt_with_git(event, git.as_ref()),
        "temperature": 0.2
    });
    if let Ok(model) = env::var("SDDIA_GEMINI_MODEL") {
        let m = model.trim();
        if !m.is_empty() {
            req["model"] = json!(m);
        }
    }
    let wrapped = json!({ "request": req });
    match invoke_tool_capsule_json(repo, "gemini-http-infer", &wrapped, false) {
        Ok(r)
            if r.exit_code == 0 && r.body.get("success") == Some(&json!(true)) =>
        {
            synthesis_from_capsule(&r.body)
        }
        _ => None,
    }
}

fn invoke_send_telegram(repo: &Path, message: &str) -> Result<(), String> {
    let req = json!({ "message": message });
    match invoke_capsule_json(repo, "send-telegram-notification", &req, false) {
        Ok(result)
            if result.exit_code == 0 && result.body.get("success") == Some(&json!(true)) =>
        {
            Ok(())
        }
        Ok(result) => Err(result
            .body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("send-telegram-notification failed")
            .to_string()),
        Err(e) => Err(e),
    }
}

pub fn run_from_event(repo: &Path, event: &Value) -> Result<NotifyOutcome, String> {
    let Some(static_msg) = pr_merged_static_message(event) else {
        return Ok(NotifyOutcome::SkippedEmpty);
    };
    let synthesis = try_infer_synthesis(repo, event);
    let msg = assemble_humanized_message(&static_msg, synthesis.as_deref());
    invoke_send_telegram(repo, &msg)?;
    Ok(NotifyOutcome::Sent {
        synthesized: synthesis.is_some(),
    })
}

/// Camino genérico `try_run_native` (solo payload): envelope ausente → sin correlación.
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let event = if inputs.get("event_type").and_then(|v| v.as_str()) == Some("PullRequest_Merged")
    {
        inputs.clone()
    } else {
        json!({
            "event_type": "PullRequest_Merged",
            "payload": inputs,
        })
    };
    match run_from_event(repo, &event)? {
        NotifyOutcome::Sent { synthesized } => Ok(json!({
            "success": true,
            "synthesized": synthesized,
        })),
        NotifyOutcome::SkippedEmpty => Ok(json!({
            "success": true,
            "skipped": true,
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_event() -> Value {
        json!({
            "event_type": "PullRequest_Merged",
            "correlation_id": "7f3a9c2e-1111-4222-8333-444444444444",
            "payload": {
                "source_branch": "feat/accept-pr-telegram-notify",
                "target_branch": "main",
                "merge_commit_hash": "a1b2c3d4e5f6789012345678901234567890abcd",
                "author": "integration-operator",
                "security_clearance": {
                    "auditor": "Argos",
                    "policy_applied": "pr-acceptance-protocol"
                },
                "traceability_anomaly": "merge_huérfano"
            }
        })
    }

    #[test]
    fn notify_humanized_prompt_includes_kernel_and_ecst_not_anomaly() {
        let p = build_synthesis_prompt(&fixture_event());
        assert!(p.contains("MAX 2 LINES"));
        assert!(p.contains("PENALIZE CONJECTURE"));
        assert!(p.contains("source_branch=feat/accept-pr-telegram-notify"));
        assert!(p.contains("merge_commit_hash=a1b2c3d4e5f6789012345678901234567890abcd"));
        assert!(p.contains("author=integration-operator"));
        assert!(!p.contains("traceability"));
        assert!(!p.contains("merge_huérfano"));
        assert!(!p.contains("commits/diffs"));
        assert!(!p.contains("SUBJECT:"));
        assert!(!p.contains("FILES:"));
    }

    #[test]
    fn notify_humanized_prompt_injects_git_facts_only_from_summary() {
        let git = CommitSummary {
            subject: "add capsule commit_summary".into(),
            files: vec![
                "SddIA/skills/git-manager/src/main.rs".into(),
                "SddIA/norms/skill-io-git-manager-frozen.md".into(),
            ],
            total_files_changed: 2,
            truncated: false,
        };
        let p = build_synthesis_prompt_with_git(&fixture_event(), Some(&git));
        assert!(p.contains("PENALIZE CONJECTURE"));
        assert!(p.contains("Do not invent files"));
        assert!(p.contains("SUBJECT: add capsule commit_summary"));
        assert!(p.contains("FILES: SddIA/skills/git-manager/src/main.rs, SddIA/norms/skill-io-git-manager-frozen.md"));
        assert!(!p.contains("invented.rs"));
        assert!(!p.contains("truncated=true"));
    }

    #[test]
    fn notify_humanized_prompt_git_failsoft_omits_subject_files() {
        let p = build_synthesis_prompt_with_git(&fixture_event(), None);
        assert!(!p.contains("SUBJECT:"));
        assert!(!p.contains("FILES:"));
        assert!(p.contains("source_branch=feat/accept-pr-telegram-notify"));
    }

    #[test]
    fn notify_humanized_prompt_truncated_note() {
        let git = CommitSummary {
            subject: "Merge branch 'feat/x'".into(),
            files: vec!["a.rs".into(), "b.rs".into()],
            total_files_changed: 40,
            truncated: true,
        };
        let p = build_synthesis_prompt_with_git(&fixture_event(), Some(&git));
        assert!(p.contains("SUBJECT: Merge branch 'feat/x'"));
        assert!(p.contains("truncated=true totalFilesChanged=40"));
        assert!(p.contains("FILES: a.rs, b.rs"));
    }

    #[test]
    fn notify_humanized_commit_summary_from_data_parses() {
        let data = json!({
            "commitHash": "a1b2c3d4e5f6789012345678901234567890abcd",
            "subject": "hello",
            "files": ["a.rs", "b.rs"],
            "totalFilesChanged": 2,
            "truncated": false
        });
        let s = commit_summary_from_data(&data).unwrap();
        assert_eq!(s.subject, "hello");
        assert_eq!(s.files, vec!["a.rs", "b.rs"]);
        assert!(!s.truncated);
        assert!(commit_summary_from_data(&json!({})).is_none());
    }

    #[test]
    fn notify_humanized_truncate_max_two_lines() {
        let t = truncate_synthesis("uno\n\ndos\ntres\n");
        assert_eq!(t, "uno\ndos");
    }

    #[test]
    fn notify_humanized_assemble_failsoft_omits_synthesis() {
        let st = pr_merged_static_message(&fixture_event()).unwrap();
        let only = assemble_humanized_message(&st, None);
        assert!(!only.contains("Síntesis de Valor"));
        assert!(only.contains("7f3a9c2e…"));
        assert!(!only.contains("merge_huérfano"));
        let with = assemble_humanized_message(&st, Some("Valor de negocio."));
        assert!(with.contains("🧠 Síntesis de Valor: Valor de negocio."));
        assert!(with.contains("✅ PR Fusionado"));
    }

    #[test]
    fn notify_humanized_synthesis_from_capsule_reads_result_text() {
        let body = json!({"success": true, "result": {"text": "  linea A\nlinea B\nlinea C  "}});
        assert_eq!(
            synthesis_from_capsule(&body).as_deref(),
            Some("linea A\nlinea B")
        );
        let empty = json!({"success": true, "result": {"text": "   "}});
        assert!(synthesis_from_capsule(&empty).is_none());
    }

    #[test]
    fn notify_humanized_skipped_when_not_merged() {
        let ev = json!({"event_type": "PullRequest_Presented", "payload": {"branch": "x"}});
        assert!(pr_merged_static_message(&ev).is_none());
    }
}
