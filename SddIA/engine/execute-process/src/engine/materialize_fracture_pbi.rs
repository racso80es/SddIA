//! Handler nativo `materialize-fracture-pbi` — materializa PBI Cúmulo ante fractura (D-P6T.1).

use crate::core::fracture_pbi::{
    display_filename_fix, display_filename_regression,
    resolve_materialize, resolve_todos_pending_rel, scan_fracture_ledger,
    MaterializeReason,
};
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub use crate::core::fracture_pbi::{fracture_trace_hash, slugify_process_name};

/// Solo para presentación al escribir; el motor no deduplica por nombre.
pub fn fracture_pbi_filename(process_name: &str, error_trace: &str) -> String {
    display_filename_fix(process_name, &fracture_trace_hash(error_trace))
}

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

fn emit_resolver_telemetry(repo: &Path, scan: &crate::core::fracture_pbi::FractureLedgerScan) {
    use super::fractal::{load_fractal_dirs, write_fractal_event};
    let (tele_dir, _, _, _) = load_fractal_dirs(repo);
    let event = json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "Fracture_Pbi_Resolver_Scan",
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": "materialize-fracture-pbi",
        "payload": {
            "docs_scanned": scan.docs_scanned,
            "bytes_read": scan.bytes_read,
            "duration_ms": scan.duration_ms,
        },
    });
    let _ = write_fractal_event(repo, &event, &tele_dir);
}

fn build_pbi_body(
    process_name: &str,
    error_trace: &str,
    agent_emitter: &str,
    attempted_action: &str,
    trace_hash: &str,
    fracture_process: &str,
    document_id: &str,
    title_prefix: &str,
    persist_ref: Option<&str>,
    branch_name: Option<&str>,
    regression_of: Option<&str>,
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
    let regression_line = regression_of
        .map(|r| format!("regression_of: {r}\n"))
        .unwrap_or_default();

    format!(
        r#"---
document_id: {document_id}
title: "{title_prefix} {process_name} — fractura sistémica"
format: markdown
version: "1.0.0"
created: "{today}"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: {trace_hash}
fracture_process: {fracture_process}
incident_ref: "System_Fracture_Detected — {trace_hash}"
{regression_line}related:
{related}
---

# {title_prefix} {process_name} — fractura sistémica

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
    let fracture_process = slugify_process_name(&process_name);

    let pending_rel = resolve_todos_pending_rel(repo)?;
    let pending_dir = repo.join(&pending_rel);
    fs::create_dir_all(&pending_dir).map_err(|e| e.to_string())?;

    let scan = scan_fracture_ledger(repo)?;
    emit_resolver_telemetry(repo, &scan);
    let resolution = resolve_materialize(&scan, &trace_hash, &fracture_process);

    let persist_ref = optional_str(inputs, "persist_ref");
    let branch_name = optional_str(inputs, "branch_name");

    match resolution.reason {
        MaterializeReason::AlreadyOpen | MaterializeReason::DedupedByProcess => {
            return Ok(json!({
                "success": true,
                "target_path": resolution.target_path,
                "reason": resolution.reason.as_str(),
                "message": resolution.reason.as_str(),
            }));
        }
        MaterializeReason::RegressionOpened => {
            let n = resolution.regression_n.unwrap_or(1);
            let document_id = format!("PBI-FIX-FRACTURE-{trace_hash}-R{n}");
            let predecessor = resolution
                .predecessor_document_id
                .as_deref()
                .unwrap_or("");
            let filename = display_filename_regression(&process_name, &trace_hash, n);
            let target = pending_dir.join(&filename);
            let body = build_pbi_body(
                &process_name,
                &error_trace,
                &agent_emitter,
                &attempted_action,
                &trace_hash,
                &fracture_process,
                &document_id,
                "[REGRESIÓN]",
                persist_ref.as_deref(),
                branch_name.as_deref(),
                if predecessor.is_empty() {
                    None
                } else {
                    Some(predecessor)
                },
            );
            fs::write(&target, body).map_err(|e| e.to_string())?;
            let rel_path = target
                .strip_prefix(repo)
                .unwrap_or(&target)
                .to_string_lossy()
                .replace('\\', "/");
            return Ok(json!({
                "success": true,
                "target_path": rel_path,
                "reason": "regression_opened",
                "message": "regression_opened",
                "canonical_ref": resolution.canonical_ref,
                "trace_hash": trace_hash,
            }));
        }
        MaterializeReason::Materialized => {}
    }

    let filename = display_filename_fix(&process_name, &trace_hash);
    let target = pending_dir.join(&filename);
    let document_id = format!("PBI-FIX-FRACTURE-{trace_hash}");
    let body = build_pbi_body(
        &process_name,
        &error_trace,
        &agent_emitter,
        &attempted_action,
        &trace_hash,
        &fracture_process,
        &document_id,
        "[FIX]",
        persist_ref.as_deref(),
        branch_name.as_deref(),
        None,
    );
    fs::write(&target, body).map_err(|e| e.to_string())?;

    let rel_path = target
        .strip_prefix(repo)
        .unwrap_or(&target)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(json!({
        "success": true,
        "target_path": rel_path,
        "reason": "materialized",
        "message": "materialized",
        "trace_hash": trace_hash,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_repo(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"todos":{"pending":"docs/todos/pending","done":"docs/todos/done"}}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();
        fs::create_dir_all(repo.join("docs/todos/done")).unwrap();
        fs::create_dir_all(repo.join(".events/telemetry")).unwrap();
    }

    fn write_closed_done(repo: &Path, hash: &str, process: &str) {
        let body = format!(
            r#"---
document_id: PBI-FIX-FRACTURE-{hash}
fracture_hash: {hash}
fracture_process: {process}
status: cerrado
process: bug-fix
---

# closed
"#
        );
        fs::write(
            repo.join(format!("docs/todos/done/canonical-{hash}.md")),
            body,
        )
        .unwrap();
    }

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
        setup_repo(repo);

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
        assert_eq!(out.get("reason"), Some(&json!("materialized")));
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
        assert!(content.contains("fracture_hash:"));
        assert!(content.contains("fracture_process: test-daemon"));
        assert!(content.contains("docs/features/foo"));
        assert!(content.contains("branch: feat/test"));
        assert!(content.contains("Pendiente de síntesis Mayeuta"));

        let out2 = run(repo, &inputs).expect("second run");
        assert_eq!(out2.get("reason"), Some(&json!("already_open")));
    }

    #[test]
    fn materialize_fracture_pbi_missing_field() {
        let tmp = tempfile::tempdir().expect("tempdir");
        setup_repo(tmp.path());
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
        setup_repo(repo);

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
        assert_eq!(out1.get("reason"), Some(&json!("materialized")));
        let path1 = out1.get("target_path").and_then(|v| v.as_str()).unwrap().to_string();

        let out2 = run(repo, &second).expect("second");
        assert_eq!(out2.get("reason"), Some(&json!("deduped_by_process")));
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

    #[test]
    fn materialize_opens_regression_when_closed_in_done() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        setup_repo(repo);
        let hash = fracture_trace_hash("same trace");
        write_closed_done(repo, &hash, "route-domain-event");

        let inputs = json!({
            "process_name": "route-domain-event",
            "error_trace": "same trace",
            "agent_emitter": "execute-process",
            "attempted_action": "merkle-batch-preseal",
        });
        let out = run(repo, &inputs).expect("regression");
        assert_eq!(out.get("reason"), Some(&json!("regression_opened")));
        assert!(out.get("canonical_ref").is_some());
        let path = out.get("target_path").and_then(|v| v.as_str()).unwrap();
        let content = fs::read_to_string(repo.join(path)).unwrap();
        assert!(content.contains("regression_of: PBI-FIX-FRACTURE-"));
        assert!(content.contains("[REGRESIÓN]"));
        let done = fs::read_to_string(repo.join(format!("docs/todos/done/canonical-{hash}.md"))).unwrap();
        assert!(!done.contains("regression_of"));
    }

    #[test]
    fn materialize_burst_after_done_opens_single_regression() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        setup_repo(repo);
        let hash = fracture_trace_hash("burst trace");
        write_closed_done(repo, &hash, "route-domain-event");
        let inputs = json!({
            "process_name": "route-domain-event",
            "error_trace": "burst trace",
            "agent_emitter": "execute-process",
            "attempted_action": "merkle-batch-preseal",
        });
        let mut regression_path = String::new();
        for i in 0..7 {
            let out = run(repo, &inputs).expect("burst");
            if i == 0 {
                assert_eq!(out.get("reason"), Some(&json!("regression_opened")));
                regression_path = out
                    .get("target_path")
                    .and_then(|v| v.as_str())
                    .unwrap()
                    .to_string();
            } else {
                assert_eq!(out.get("reason"), Some(&json!("already_open")));
                assert_eq!(
                    out.get("target_path").and_then(|v| v.as_str()),
                    Some(regression_path.as_str())
                );
            }
        }
        let count = fs::read_dir(repo.join("docs/todos/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("md"))
            .count();
        assert_eq!(count, 1);
    }
}
