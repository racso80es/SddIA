//! Handler nativo `materialize-fracture-pbi` — materializa PBI Cúmulo ante fractura (D-P6T.1).

use crate::core::fracture_pbi::{
    display_filename_fix, display_filename_regression,
    resolve_materialize, scan_fracture_ledger,
    MaterializeReason,
};
use crate::core::fracture_signatures::load_fracture_catalog;
use chrono::Utc;
use serde_json::{json, Value};
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

fn issue_ref_of(body: &Value) -> Result<String, String> {
    body.get("issue_ref")
        .or_else(|| body.get("identifier"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| "LINEAR_CREATE_ISSUE_EMPTY: sin issue_ref".into())
}

fn description_has_hash(item: &Value, hash: &str) -> bool {
    let marker = format!("fracture_hash: {hash}");
    item.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .contains(&marker)
}

fn state_of(item: &Value) -> String {
    item.get("state")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

/// `Open` si hay una issue no cerrada con el hash. `ClosedOnly` si todas las coincidencias están en Done o Cancelled.
enum RemoteHashHit {
    Open(String),
    ClosedOnly,
    None,
}

fn classify_remote_hash(
    items: &[Value],
    hash: &str,
    done_state: &str,
    cancelled_state: Option<&str>,
) -> RemoteHashHit {
    let mut saw_closed = false;
    for item in items {
        if !description_has_hash(item, hash) {
            continue;
        }
        let state = state_of(item);
        let closed = state == done_state || cancelled_state.is_some_and(|c| state == c);
        if !closed {
            if let Ok(id) = issue_ref_of(item) {
                return RemoteHashHit::Open(id);
            }
        } else {
            saw_closed = true;
        }
    }
    if saw_closed {
        RemoteHashHit::ClosedOnly
    } else {
        RemoteHashHit::None
    }
}

fn list_remote_issues(
    repo: &Path,
    inputs: &Value,
    tc: &super::project_binding::TrackerConfig,
) -> Result<Vec<Value>, String> {
    let mut items = Vec::new();
    let mut after: Option<String> = None;
    for _ in 0..10 {
        let mut request = json!({
            "operation": "list_issues",
            "team_key": tc.team_key,
            "project_id": tc.project_id,
        });
        if let Some(cursor) = &after {
            request["after"] = json!(cursor);
        }
        let listed = super::tracker_lifecycle_port::invoke(
            repo,
            inputs,
            request,
            "materialize-fracture-pbi",
        )?;
        if let Some(arr) = listed.get("items").and_then(|v| v.as_array()) {
            items.extend(arr.iter().cloned());
        }
        let more = listed
            .pointer("/page_info/has_next_page")
            .and_then(|v| v.as_bool())
            == Some(true);
        let cursor = listed
            .pointer("/page_info/end_cursor")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        if !more {
            break;
        }
        let Some(cursor) = cursor else {
            break;
        };
        if after.as_deref() == Some(cursor.as_str()) {
            break;
        }
        after = Some(cursor);
    }
    Ok(items)
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
    friction_id: Option<&str>,
    tracker_column: Option<&str>,
    refinement: Option<&str>,
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
    let friction_line = friction_id
        .map(|f| format!("friction_id: {f}\n"))
        .unwrap_or_default();
    let tracker_line = match (tracker_column, refinement) {
        (Some(column), Some(refine)) => {
            format!("tracker_column: {column}\nrefinement: {refine}\n")
        }
        _ => String::new(),
    };

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
{friction_line}{regression_line}{tracker_line}related:
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
- [ ] Caso añadido a `fracture-corpus` (firma esperada o `unclassified` justificado)
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este ítem cerrado en el sumidero del proveedor
"#
    )
}

fn emit_materialized_fact(
    repo: &Path,
    inputs: &Value,
    target: &str,
    reason: &str,
    trace_hash: &str,
    remote: bool,
) -> bool {
    let mut payload = json!({
        "process_name": inputs.get("process_name"),
        "error_trace": inputs.get("error_trace"),
        "agent_emitter": inputs.get("agent_emitter"),
        "attempted_action": inputs.get("attempted_action"),
        "target_path": target,
        "trace_hash": trace_hash,
        "reason": reason,
    });
    if let Some(obj) = payload.as_object_mut() {
        for key in ["project_slug", "persist_ref", "branch_name", "friction_id"] {
            if let Some(v) = inputs.get(key).filter(|v| !v.is_null()) {
                obj.insert(key.into(), v.clone());
            }
        }
        if remote {
            obj.insert("tracker_ref".into(), json!(target));
        }
    }
    super::route_domain_core::materialize_pending_domain_event(
        repo,
        "Fracture_Pbi_Materialized",
        "materialize-fracture-pbi",
        payload,
    )
    .is_ok()
}

fn finish(
    repo: &Path,
    inputs: &Value,
    target: String,
    reason: &str,
    trace_hash: &str,
    remote: bool,
    extra: Value,
) -> Value {
    let fact_emitted = emit_materialized_fact(repo, inputs, &target, reason, trace_hash, remote);
    let mut out = json!({
        "success": true,
        "target_path": target,
        "reason": reason,
        "message": reason,
        "trace_hash": trace_hash,
        "fact_emitted": fact_emitted,
    });
    if remote {
        out["issue_ref"] = json!(out["target_path"].as_str().unwrap_or(""));
        out["tracker_ref"] = out["target_path"].clone();
    }
    if let (Some(obj), Some(extra_obj)) = (out.as_object_mut(), extra.as_object()) {
        for (k, v) in extra_obj {
            obj.insert(k.clone(), v.clone());
        }
    }
    out
}

/// Ejecuta `materialize-fracture-pbi`.
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let bound = super::project_binding::inputs_with_self_hosted_slug(repo, inputs)?;
    let process_name = required_str(&bound, "process_name")?;
    let error_trace = required_str(&bound, "error_trace")?;
    let agent_emitter = required_str(&bound, "agent_emitter")?;
    let attempted_action = required_str(&bound, "attempted_action")?;

    let trace_hash = fracture_trace_hash(&error_trace);
    let fracture_process = slugify_process_name(&process_name);
    let persist_ref = optional_str(&bound, "persist_ref");
    let branch_name = optional_str(&bound, "branch_name");
    let friction_id = optional_str(&bound, "friction_id").filter(|id| {
        load_fracture_catalog(repo)
            .map(|c| c.catalog_contains_id(id))
            .unwrap_or(false)
    });

    match super::tracker_lifecycle_port::resolve_target(repo, &bound)? {
        super::tracker_lifecycle_port::LifecycleTarget::Remote { .. } => run_remote(
            repo,
            &bound,
            &process_name,
            &error_trace,
            &agent_emitter,
            &attempted_action,
            &trace_hash,
            &fracture_process,
            persist_ref.as_deref(),
            branch_name.as_deref(),
            friction_id.as_deref(),
        ),
        super::tracker_lifecycle_port::LifecycleTarget::File => run_file(
            repo,
            &bound,
            &process_name,
            &error_trace,
            &agent_emitter,
            &attempted_action,
            &trace_hash,
            &fracture_process,
            persist_ref.as_deref(),
            branch_name.as_deref(),
            friction_id.as_deref(),
        ),
        super::tracker_lifecycle_port::LifecycleTarget::Unconfigured => Err(
            "TRACKER_UNCONFIGURED: sin tracker.provider resuelto; no se escribe backlog local"
                .into(),
        ),
    }
}

fn run_remote(
    repo: &Path,
    inputs: &Value,
    process_name: &str,
    error_trace: &str,
    agent_emitter: &str,
    attempted_action: &str,
    trace_hash: &str,
    fracture_process: &str,
    persist_ref: Option<&str>,
    branch_name: Option<&str>,
    friction_id: Option<&str>,
) -> Result<Value, String> {
    let tc = super::project_binding::tracker_config_for_inputs(repo, inputs)?
        .ok_or_else(|| "TRACKER_UNCONFIGURED: provider linear sin TrackerConfig".to_string())?;
    let done_state = tc
        .state_map
        .get("done")
        .cloned()
        .unwrap_or_else(|| "Done".into());
    let cancelled = tc.state_map.get("cancelled").cloned();
    let backlog = tc
        .state_map
        .get("backlog")
        .cloned()
        .unwrap_or_else(|| "Backlog".into());
    let items = list_remote_issues(repo, inputs, &tc)?;
    let regression = match classify_remote_hash(
        &items,
        trace_hash,
        &done_state,
        cancelled.as_deref(),
    ) {
        RemoteHashHit::Open(id) => {
            return Ok(json!({
                "success": true,
                "target_path": id,
                "issue_ref": id,
                "tracker_ref": id,
                "reason": "already_open",
                "message": "already_open",
                "trace_hash": trace_hash,
            }));
        }
        RemoteHashHit::ClosedOnly => true,
        RemoteHashHit::None => false,
    };
    let title_prefix = if regression { "[REGRESIÓN]" } else { "[FIX]" };
    let title = format!("{title_prefix} {process_name} — fractura sistémica");
    let n = 1u32;
    let document_id = if regression {
        format!("PBI-FIX-FRACTURE-{trace_hash}-R{n}")
    } else {
        format!("PBI-FIX-FRACTURE-{trace_hash}")
    };
    let body = build_pbi_body(
        process_name,
        error_trace,
        agent_emitter,
        attempted_action,
        trace_hash,
        fracture_process,
        &document_id,
        title_prefix,
        persist_ref,
        branch_name,
        None,
        friction_id,
        Some("backlog"),
        Some("pending"),
    );
    let label_pbi = super::project_binding::tracker_label_for_inputs(repo, inputs, "pbi")?
        .unwrap_or_else(|| "pbi".into());
    let label_fix = super::project_binding::tracker_label_for_inputs(repo, inputs, "fix")?
        .unwrap_or_else(|| "fix".into());
    let mut request = json!({
        "operation": "create_issue",
        "team_key": tc.team_key,
        "title": title,
        "description": body,
        "labels": [label_pbi, label_fix],
        "state_name": backlog,
    });
    if let Some(pid) = &tc.project_id {
        request["project_id"] = json!(pid);
    }
    let created = super::tracker_lifecycle_port::invoke(
        repo,
        inputs,
        request,
        "materialize-fracture-pbi",
    )?;
    let issue = issue_ref_of(&created)?;
    let reason = if regression {
        "regression_opened"
    } else {
        "materialized"
    };
    Ok(finish(
        repo,
        inputs,
        issue,
        reason,
        trace_hash,
        true,
        json!({}),
    ))
}

fn run_file(
    repo: &Path,
    inputs: &Value,
    process_name: &str,
    error_trace: &str,
    agent_emitter: &str,
    attempted_action: &str,
    trace_hash: &str,
    fracture_process: &str,
    persist_ref: Option<&str>,
    branch_name: Option<&str>,
    friction_id: Option<&str>,
) -> Result<Value, String> {
    let scan = scan_fracture_ledger(repo)?;
    emit_resolver_telemetry(repo, &scan);
    let resolution = resolve_materialize(&scan, trace_hash, fracture_process);
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
            let predecessor = resolution.predecessor_document_id.as_deref().unwrap_or("");
            let filename = display_filename_regression(process_name, trace_hash, n);
            let body = build_pbi_body(
                process_name,
                error_trace,
                agent_emitter,
                attempted_action,
                trace_hash,
                fracture_process,
                &document_id,
                "[REGRESIÓN]",
                persist_ref,
                branch_name,
                if predecessor.is_empty() {
                    None
                } else {
                    Some(predecessor)
                },
                friction_id,
                None,
                None,
            );
            let created = super::tracker_lifecycle_port::invoke(
                repo,
                inputs,
                json!({
                    "operation": "create_issue",
                    "title": filename,
                    "description": body,
                }),
                "materialize-fracture-pbi",
            )?;
            let rel_path = created
                .get("artifact_path")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            return Ok(finish(
                repo,
                inputs,
                rel_path,
                "regression_opened",
                trace_hash,
                false,
                json!({
                    "canonical_ref": resolution.canonical_ref,
                    "regression_n": n,
                }),
            ));
        }
        MaterializeReason::Materialized => {}
    }

    let filename = display_filename_fix(process_name, trace_hash);
    let document_id = format!("PBI-FIX-FRACTURE-{trace_hash}");
    let body = build_pbi_body(
        process_name,
        error_trace,
        agent_emitter,
        attempted_action,
        trace_hash,
        fracture_process,
        &document_id,
        "[FIX]",
        persist_ref,
        branch_name,
        None,
        friction_id,
        None,
        None,
    );
    let created = super::tracker_lifecycle_port::invoke(
        repo,
        inputs,
        json!({
            "operation": "create_issue",
            "title": filename,
            "description": body,
        }),
        "materialize-fracture-pbi",
    )?;
    let rel_path = created
        .get("artifact_path")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    Ok(finish(
        repo,
        inputs,
        rel_path,
        "materialized",
        trace_hash,
        false,
        json!({}),
    ))
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
        fs::create_dir_all(repo.join(".SddIA/projects")).unwrap();
        fs::write(
            repo.join(".SddIA/projects/lab.md"),
            "---\nid: lab\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\nproject_root: .\nmanifest_ref: .SddIA/project.md\nstatus: active\n---\n",
        )
        .unwrap();
        fs::write(
            repo.join(".SddIA/project.md"),
            "---\nid: lab\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\ngit_remote: https://example.invalid/lab.git\ndefault_branch: main\ndelivery_mode: branch_pr\ncontract_version: \"1.3.0\"\ncodex_slug: codex-software-engineering\ntracker:\n  provider: file\n---\n",
        )
        .unwrap();
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

    #[test]
    fn unconfigured_does_not_write_markdown() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"todos":{"pending":"docs/todos/pending","done":"docs/todos/done"}}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();
        let err = run(
            repo,
            &json!({
                "process_name": "event-watcher",
                "error_trace": "sin proveedor",
                "agent_emitter": "domain-authority",
                "attempted_action": "forge-pbi",
            }),
        )
        .unwrap_err();
        assert!(err.contains("TRACKER_UNCONFIGURED"));
        let count = fs::read_dir(repo.join("docs/todos/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .count();
        assert_eq!(count, 0);
    }

    #[test]
    fn remote_hash_open_blocks_and_done_only_is_regression() {
        let hash = "abc123def456";
        let open = json!({"identifier": "AUT-1", "description": format!("fracture_hash: {hash}\n"), "state": "Backlog"});
        let done = json!({"identifier": "AUT-2", "description": format!("fracture_hash: {hash}\n"), "state": "Done"});
        let other = json!({"identifier": "AUT-3", "description": "fracture_hash: fff\n", "state": "Backlog"});
        assert!(matches!(
            classify_remote_hash(&[open, other.clone()], hash, "Done", Some("Cancelled")),
            RemoteHashHit::Open(_)
        ));
        assert!(matches!(
            classify_remote_hash(&[done, other.clone()], hash, "Done", Some("Cancelled")),
            RemoteHashHit::ClosedOnly
        ));
        assert!(matches!(
            classify_remote_hash(&[other], hash, "Done", Some("Cancelled")),
            RemoteHashHit::None
        ));
    }

    #[test]
    fn self_hosted_linear_ignores_orphan_markdown() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let core = crate::core::repo::find_repo_root().expect("repo");
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        let native = core.join("SddIA/target");
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            format!(
                r#"{{
  "paths": {{"todos": {{"pending": "docs/todos/pending", "done": "docs/todos/done"}}}},
  "directories": {{"norms": "SddIA/norms"}},
  "capability_di": {{"bindings": "SddIA/core/capability-bindings.md"}},
  "compiled_capsules": {{
    "native_root": "{}",
    "wasm_root": "{}/wasm32-wasip1",
    "profiles": ["debug", "release"]
  }}
}}"#,
                native.display(),
                native.display()
            ),
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/capability-bindings.md"),
            "---\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\nname: capability-bindings\nversion: \"1.0.0\"\nbindings:\n  - capability_id: \"tracker:lifecycle\"\n    contract: \"tracker.lifecycle\"\n    provider: \"tool:linear-tracker-adapter\"\n    provider_version: \">=1.0.0\"\n---\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/norms")).unwrap();
        fs::write(
            repo.join("SddIA/norms/execution-contexts.md"),
            "### 2.10. Tracker\n**Alcance:** crear issues.\n**Fuera de alcance:** borrar issues.\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/actions")).unwrap();
        fs::write(
            repo.join("SddIA/actions/materialize-fracture-pbi.md"),
            "---\nname: materialize-fracture-pbi\n---\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::copy(
            core.join("SddIA/tools/linear-tracker-adapter.md"),
            repo.join("SddIA/tools/linear-tracker-adapter.md"),
        )
        .unwrap();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();
        fs::create_dir_all(repo.join("docs/todos/done")).unwrap();
        fs::create_dir_all(repo.join(".events/telemetry")).unwrap();
        fs::write(
            repo.join("docs/todos/pending/huerfano.md"),
            "---\nfracture_hash: pendiente\nfracture_process: event-watcher\nstatus: abierto\n---\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join(".SddIA/projects")).unwrap();
        fs::write(
            repo.join(".SddIA/projects/lab.md"),
            "---\nid: lab\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\nproject_root: .\nmanifest_ref: .SddIA/project.md\nstatus: active\n---\n",
        )
        .unwrap();
        fs::write(
            repo.join(".SddIA/project.md"),
            "---\nid: lab\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\ngit_remote: https://example.invalid/lab.git\ndefault_branch: main\ndelivery_mode: branch_pr\ncontract_version: \"1.3.0\"\ncodex_slug: codex-software-engineering\ntracker:\n  provider: linear\n  team_key: LAB\n  project_id: proj-lab\n  state_map:\n    backlog: Backlog\n    done: Done\n    cancelled: Cancelled\n  labels:\n    pbi: pbi\n    fix: fix\n---\n",
        )
        .unwrap();
        let prev = std::env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        std::env::set_var("SDDIA_LAB_RESET_STORE", "1");
        let inputs = json!({
            "process_name": "event-watcher",
            "error_trace": "fractura sin slug AUT-30",
            "agent_emitter": "domain-authority",
            "attempted_action": "forge-pbi",
        });
        let out = run(repo, &inputs).expect("linear sin slug");
        std::env::remove_var("SDDIA_LAB_RESET_STORE");
        assert_eq!(out["reason"], "materialized");
        let issue = out["target_path"].as_str().unwrap();
        assert!(!issue.contains('/'), "{issue}");
        assert_eq!(out["fact_emitted"], true);
        let again = run(repo, &inputs).expect("dedupe");
        assert_eq!(again["reason"], "already_open");
        assert_eq!(again["target_path"], out["target_path"]);
        let md = fs::read_dir(repo.join("docs/todos/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("md"))
            .count();
        assert_eq!(md, 1, "el huerfano no veta y no nace otro markdown");
        match prev {
            Some(v) => std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", v),
            None => std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND"),
        }
    }
}
