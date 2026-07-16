//! Handler nativo `materialize-fracture-pbi` — materializa PBI Cúmulo ante fractura (D-P6T.1).

use chrono::Utc;
use regex::Regex;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

fn required_str(inputs: &Value, key: &str) -> Result<String, String> {
    match inputs.get(key).and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => Ok(s.trim().to_string()),
        _ => Err(format!("{key} es obligatorio (string)")),
    }
}

fn optional_str(inputs: &Value, key: &str) -> Option<String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Paridad `execute-action.py::_slugify_process_name`.
pub fn slugify_process_name(name: &str) -> String {
    static RE_NON_WORD: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static RE_DASHES: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re_non = RE_NON_WORD.get_or_init(|| Regex::new(r"[^\w\-]+").expect("regex"));
    let re_dash = RE_DASHES.get_or_init(|| Regex::new(r"-+").expect("regex"));
    let lower = name.trim().to_lowercase();
    let slug = re_non.replace_all(&lower, "-");
    let slug = re_dash.replace_all(&slug, "-");
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "fracture".to_string()
    } else if slug.len() > 48 {
        slug[..48].to_string()
    } else {
        slug.to_string()
    }
}

/// Paridad `execute-action.py::_fracture_trace_hash`.
pub fn fracture_trace_hash(error_trace: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(error_trace.trim().as_bytes());
    format!("{:x}", hasher.finalize())[..12].to_string()
}

/// Paridad `execute-action.py::_fracture_pbi_filename`.
pub fn fracture_pbi_filename(process_name: &str, error_trace: &str) -> String {
    let slug = slugify_process_name(process_name);
    format!(
        "[FIX] {slug} — fractura sistémica ({}).md",
        fracture_trace_hash(error_trace)
    )
}

fn fracture_pbi_path(repo: &Path, process_name: &str, error_trace: &str) -> PathBuf {
    repo.join("docs/todos/pending").join(fracture_pbi_filename(
        process_name,
        error_trace,
    ))
}

/// PBI pending abierto del mismo `process_name` (anti-spam ola heartbeat).
fn find_open_fracture_pbi(repo: &Path, process_name: &str) -> Option<PathBuf> {
    let pending = repo.join("docs/todos/pending");
    if !pending.is_dir() {
        return None;
    }
    let slug = slugify_process_name(process_name);
    let prefix = format!("[FIX] {slug} — fractura sistémica (");
    let suffix = ").md";
    let mut matches: Vec<PathBuf> = fs::read_dir(&pending)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension().and_then(|x| x.to_str()) == Some("md")
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with(&prefix) && n.ends_with(suffix))
                    .unwrap_or(false)
        })
        .collect();
    matches.sort();
    for path in matches {
        let Ok(raw) = fs::read_to_string(&path) else {
            continue;
        };
        // Frontmatter status abierto (default de materialize).
        let status_open = raw.lines().take(40).any(|line| {
            let t = line.trim();
            t == "status: abierto"
                || t == "status: \"abierto\""
                || t == "status: 'abierto'"
        });
        if status_open {
            return Some(path);
        }
    }
    None
}

fn build_pbi_body(
    process_name: &str,
    error_trace: &str,
    agent_emitter: &str,
    attempted_action: &str,
    trace_hash: &str,
    persist_ref: Option<&str>,
    branch_name: Option<&str>,
) -> String {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let mut related_lines = vec![
        "  - SddIA/norms/obediencia-procesos.md".to_string(),
        "  - SddIA/events/domain/system-fracture-detected.md".to_string(),
    ];
    if let Some(p) = persist_ref {
        related_lines.push(format!("  - {p}"));
    }
    if let Some(b) = branch_name {
        related_lines.push(format!("  - branch: {b}"));
    }
    let related = related_lines.join("\n");

    format!(
        r#"---
document_id: PBI-FIX-FRACTURE-{trace_hash}
title: "[FIX] {process_name} — fractura sistémica"
format: markdown
version: "1.0.0"
created: "{today}"
status: "abierto"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — {trace_hash}"
related:
{related}
---

# [FIX] {process_name} — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `{process_name}` |
| Emisor | `{agent_emitter}` |
| Acción intentada | `{attempted_action}` |

## Traza de error

```
{error_trace}
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

_Pendiente de síntesis Mayeuta (Kintsugi async)._

## Criterio de cierre

- [ ] Causa raíz resuelta
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/`
"#
    )
}

/// Ejecuta `materialize-fracture-pbi` (paridad `execute-action.py::_run_materialize_fracture_pbi`).
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let process_name = required_str(inputs, "process_name")?;
    let error_trace = required_str(inputs, "error_trace")?;
    let agent_emitter = required_str(inputs, "agent_emitter")?;
    let attempted_action = required_str(inputs, "attempted_action")?;

    let trace_hash = fracture_trace_hash(&error_trace);
    let pending_dir = repo.join("docs/todos/pending");
    fs::create_dir_all(&pending_dir).map_err(|e| e.to_string())?;
    let target = fracture_pbi_path(repo, &process_name, &error_trace);
    let rel_path = target
        .strip_prefix(repo)
        .unwrap_or(&target)
        .to_string_lossy()
        .replace('\\', "/");

    if target.is_file() {
        return Ok(json!({
            "success": true,
            "target_path": rel_path,
            "message": "PBI ya existente (idempotente)",
        }));
    }

    // Misma clase de fractura, traza distinta (p.ej. missed_cycles/timestamp): no spamear.
    if let Some(existing) = find_open_fracture_pbi(repo, &process_name) {
        let existing_rel = existing
            .strip_prefix(repo)
            .unwrap_or(&existing)
            .to_string_lossy()
            .replace('\\', "/");
        return Ok(json!({
            "success": true,
            "target_path": existing_rel,
            "message": "PBI abierto del mismo proceso (idempotente por process_name)",
            "deduped_trace_hash": trace_hash,
        }));
    }

    let persist_ref = optional_str(inputs, "persist_ref");
    let branch_name = optional_str(inputs, "branch_name");
    let body = build_pbi_body(
        &process_name,
        &error_trace,
        &agent_emitter,
        &attempted_action,
        &trace_hash,
        persist_ref.as_deref(),
        branch_name.as_deref(),
    );
    fs::write(&target, body).map_err(|e| e.to_string())?;

    Ok(json!({
        "success": true,
        "target_path": rel_path,
        "message": "PBI materializado",
        "trace_hash": trace_hash,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn slugify_truncates_and_normalizes() {
        assert_eq!(slugify_process_name("Event Watcher!!!"), "event-watcher");
        assert_eq!(slugify_process_name("   "), "fracture");
        let long = "a".repeat(60);
        assert_eq!(slugify_process_name(&long).len(), 48);
    }

    #[test]
    fn fracture_trace_hash_deterministic() {
        let h1 = fracture_trace_hash("same error");
        let h2 = fracture_trace_hash("same error");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 12);
        assert_ne!(fracture_trace_hash("other"), h1);
    }

    #[test]
    fn materialize_fracture_pbi_creates_and_idempotent() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();

        let inputs = json!({
            "process_name": "test-daemon",
            "error_trace": "colapsó el watcher",
            "agent_emitter": "event-watcher",
            "attempted_action": "delivery-close-cycle push",
            "persist_ref": "docs/features/foo",
            "branch_name": "feat/test"
        });

        let out = run(repo, &inputs).expect("first run");
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert_eq!(out.get("message"), Some(&json!("PBI materializado")));
        assert!(out.get("trace_hash").and_then(|v| v.as_str()).is_some());

        let path = out
            .get("target_path")
            .and_then(|v| v.as_str())
            .expect("target_path");
        let full = repo.join(path);
        assert!(full.is_file());
        let content = fs::read_to_string(&full).unwrap();
        assert!(content.contains("test-daemon"));
        assert!(content.contains("colapsó el watcher"));
        assert!(content.contains("docs/features/foo"));
        assert!(content.contains("branch: feat/test"));
        assert!(content.contains("Pendiente de síntesis Mayeuta"));

        let out2 = run(repo, &inputs).expect("second run");
        assert_eq!(out2.get("message"), Some(&json!("PBI ya existente (idempotente)")));
        assert!(out2.get("trace_hash").is_none());
    }

    #[test]
    fn materialize_fracture_pbi_missing_field() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = run(
            tmp.path(),
            &json!({
                "process_name": "x",
                "error_trace": "e",
                "agent_emitter": "a"
            }),
        )
        .unwrap_err();
        assert!(err.contains("attempted_action"));
    }

    #[test]
    fn materialize_dedupes_open_pbi_same_process_different_trace() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();

        let first = json!({
            "process_name": "event-watcher",
            "error_trace": "Centinela event-watcher omitió 13 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-16T15:55:03Z",
            "agent_emitter": "argos",
            "attempted_action": "daemon-heartbeat-audit",
        });
        let second = json!({
            "process_name": "event-watcher",
            "error_trace": "Centinela event-watcher omitió 37 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-16T16:08:11Z",
            "agent_emitter": "argos",
            "attempted_action": "daemon-heartbeat-audit",
        });

        let out1 = run(repo, &first).expect("first");
        assert_eq!(out1.get("message"), Some(&json!("PBI materializado")));
        let path1 = out1.get("target_path").and_then(|v| v.as_str()).unwrap().to_string();

        let out2 = run(repo, &second).expect("second");
        assert_eq!(
            out2.get("message"),
            Some(&json!("PBI abierto del mismo proceso (idempotente por process_name)"))
        );
        assert_eq!(
            out2.get("target_path").and_then(|v| v.as_str()),
            Some(path1.as_str())
        );

        let pending = repo.join("docs/todos/pending");
        let count = fs::read_dir(&pending)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("md"))
            .count();
        assert_eq!(count, 1);
    }
}
