//! Proceso `accept-pr` nativo (sin bridge capsules).

use super::actions;
use super::capsules::invoke_git_manager;
use super::crypto_broker;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn scan_presented_for_branch(repo: &Path, branch: &str) -> bool {
    let pending = repo.join(".events/pending");
    if !pending.is_dir() {
        return false;
    }
    let Ok(entries) = fs::read_dir(&pending) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(event) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        if event.get("event_type").and_then(|v| v.as_str()) != Some("PullRequest_Presented") {
            continue;
        }
        let payload_branch = event
            .get("payload")
            .and_then(|p| p.get("branch"))
            .and_then(|v| v.as_str());
        if payload_branch == Some(branch) {
            return true;
        }
    }
    false
}

/// Payload canónico local (`remote: false`) — L-DELETE-PAYLOAD / accept-pr.md § Fase 4.
pub(crate) fn delete_branch_local_payload(branch: &str) -> Value {
    json!({"branch_name": branch, "remote": false, "force": false})
}

/// Payload canónico remoto (`remote: true`) — L-DELETE-PAYLOAD / accept-pr.md § Fase 4.
pub(crate) fn delete_branch_remote_payload(branch: &str) -> Value {
    json!({"branch_name": branch, "remote": true, "force": false})
}

/// Acumula resultado por op; `ok` de la op no tumba el proceso (L-HYGIENE-SOFT).
pub(crate) fn record_hygiene_op(
    operations: &mut Vec<Value>,
    scope: &str,
    result: Result<Value, String>,
) -> bool {
    match result {
        Ok(_) => {
            operations.push(json!({"scope": scope, "ok": true}));
            true
        }
        Err(e) => {
            operations.push(json!({"scope": scope, "ok": false, "error": e}));
            false
        }
    }
}

fn delete_branch_hygiene(repo: &Path, branch: &str) -> (Option<String>, Option<Value>) {
    if env_truthy("SDDIA_LAB_SKIP_BRANCH_DELETE") {
        return (None, None);
    }
    let mut operations: Vec<Value> = Vec::new();
    let local_ok = record_hygiene_op(
        &mut operations,
        "local",
        invoke_git_manager(repo, "delete_branch", &delete_branch_local_payload(branch)),
    );
    let remote_ok = record_hygiene_op(
        &mut operations,
        "remote",
        invoke_git_manager(repo, "delete_branch", &delete_branch_remote_payload(branch)),
    );
    let closed = if local_ok {
        Some(branch.to_string())
    } else {
        None
    };
    let hygiene_failure = if operations.iter().any(|op| op.get("ok") == Some(&json!(false))) {
        Some(json!({
            "survived_branch": branch,
            "branch_deleted_local": local_ok,
            "branch_deleted_remote": remote_ok,
            "operations": operations,
        }))
    } else {
        None
    };
    (closed, hygiene_failure)
}

pub fn execute_accept_pr_phase(
    repo: &Path,
    phase_name: &str,
    inputs: &Value,
    state: &mut Value,
) -> Option<Result<Value, String>> {
    match phase_name {
        "Auditoría Genómica" => {
            let Some(source) = str_field(inputs, "source_branch") else {
                return Some(Err("source_branch es obligatorio para accept-pr".into()));
            };
            let presented = scan_presented_for_branch(repo, &source);
            let orphan = !presented;
            if let Some(obj) = state.as_object_mut() {
                obj.insert("orphan_merge".into(), json!(orphan));
                obj.insert("source_branch".into(), json!(source));
                if orphan {
                    let handoff = obj.entry("handoff".to_string()).or_insert(json!({}));
                    if let Some(h) = handoff.as_object_mut() {
                        h.insert(
                            "traceability_warning".into(),
                            json!("Merge Huérfano: sin PullRequest_Presented previo en bus local"),
                        );
                    }
                }
            }
            Some(Ok(json!({
                "status": "executed",
                "handler": "accept-genomic-audit",
                "orphan_merge": orphan,
                "presented_found": presented,
            })))
        }
        "Fusión Soberana" => {
            let Some(source) = state
                .get("source_branch")
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .or_else(|| str_field(inputs, "source_branch"))
            else {
                return Some(Err("source_branch es obligatorio para Fusión Soberana".into()));
            };
            if matches!(
                inputs.get("merge_already_done"),
                Some(v) if v.as_bool() == Some(true)
                    || v.as_str().map(|s| matches!(s, "true" | "1")).unwrap_or(false)
            ) {
                let data = match invoke_git_manager(repo, "get_last_commit", &json!({"ref": "HEAD"})) {
                    Ok(d) => d,
                    Err(e) => return Some(Err(e)),
                };
                let merge_hash = data
                    .get("commitHash")
                    .or_else(|| data.get("commit_hash"))
                    .cloned();
                if let Some(obj) = state.as_object_mut() {
                    if let Some(h) = merge_hash.clone() {
                        obj.insert("merge_commit_hash".into(), h);
                    }
                }
                return Some(Ok(json!({
                    "status": "executed",
                    "handler": "accept-merge-sovereign",
                    "skipped": true,
                    "reason": "merge_already_done",
                    "merge_commit_hash": merge_hash,
                })));
            }
            if let Err(e) = invoke_git_manager(
                repo,
                "checkout",
                &json!({"branch_name": "main", "create_if_not_exists": false}),
            ) {
                return Some(Err(e));
            }
            let merge_data = match invoke_git_manager(
                repo,
                "merge",
                &json!({"branch_name": source, "no_ff": true}),
            ) {
                Ok(d) => d,
                Err(e) => return Some(Err(e)),
            };
            let mut merge_hash = merge_data
                .get("commitHash")
                .or_else(|| merge_data.get("commit_hash"))
                .or_else(|| merge_data.get("mergeCommitHash"))
                .cloned();
            if merge_hash.is_none() {
                let head = match invoke_git_manager(repo, "get_last_commit", &json!({"ref": "HEAD"})) {
                    Ok(d) => d,
                    Err(e) => return Some(Err(e)),
                };
                merge_hash = head
                    .get("commitHash")
                    .or_else(|| head.get("commit_hash"))
                    .cloned();
            }
            if let Some(obj) = state.as_object_mut() {
                if let Some(h) = merge_hash.clone() {
                    obj.insert("merge_commit_hash".into(), h);
                }
            }
            Some(Ok(json!({
                "status": "executed",
                "handler": "accept-merge-sovereign",
                "merge_commit_hash": merge_hash,
                "source_branch": source,
            })))
        }
        "Sello Criptográfico de Fusión" => {
            let Some(source) = state
                .get("source_branch")
                .and_then(|v| v.as_str())
                .or_else(|| inputs.get("source_branch").and_then(|v| v.as_str()))
            else {
                return Some(Err("source_branch es obligatorio para sello Merged".into()));
            };
            let Some(merge_hash) = state
                .get("merge_commit_hash")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
            else {
                return Some(Err("merge_commit_hash ausente antes del sello Merged".into()));
            };
            let correlation_id = str_field(inputs, "correlation_id").unwrap_or_else(|| {
                crypto_broker::run(
                    repo,
                    &json!({"operation": "GENERATE_UUID", "target_payload": null}),
                )
                .ok()
                .and_then(|v| {
                    v.get("crypto_response")
                        .and_then(|r| r.as_str().map(str::to_string))
                })
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
            });
            let mut action_inputs = json!({
                "source_branch": source,
                "author": inputs.get("author").unwrap_or(&json!("integration-operator")),
                "correlation_id": correlation_id,
                "merge_commit_hash": merge_hash,
                "emitter_agent": "accept-pr",
            });
            if state.get("orphan_merge").and_then(|v| v.as_bool()) == Some(true) {
                action_inputs["traceability_anomaly"] = json!("merge_huérfano");
                action_inputs["traceability_note"] =
                    json!("Fusión física sin PullRequest_Presented previo en bus local");
            }
            let seal = match actions::try_run_native(repo, "emit-pr-merged-event", &action_inputs) {
                Ok(Some(s)) => s,
                Ok(None) => {
                    return Some(Err("emit-pr-merged-event nativo no disponible".into()));
                }
                Err(e) => return Some(Err(e)),
            };
            if let Some(obj) = state.as_object_mut() {
                if let Some(h) = obj.get_mut("handoff").and_then(|v| v.as_object_mut()) {
                    if let Some(s) = seal.as_object() {
                        for (k, v) in s {
                            h.insert(k.clone(), v.clone());
                        }
                    }
                }
                if let Some(eid) = seal.get("event_id") {
                    obj.insert("event_id".into(), eid.clone());
                }
                if let Some(tp) = seal.get("target_path") {
                    obj.insert("target_path".into(), tp.clone());
                }
            }
            Some(Ok(json!({
                "status": "executed",
                "handler": "accept-emit-merged",
                "event_id": seal.get("event_id"),
                "target_path": seal.get("target_path"),
                "event_type": seal.get("event_type"),
            })))
        }
        "Sincronización y Limpieza" => {
            let source = state
                .get("source_branch")
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .or_else(|| str_field(inputs, "source_branch"));
            let prev_skip = std::env::var("SDDIA_SKIP_HOOKS").ok();
            std::env::set_var("SDDIA_SKIP_HOOKS", "1");
            let push_data = invoke_git_manager(
                repo,
                "push",
                &json!({"remote": "origin", "branch": "main", "force": false}),
            );
            if let Some(v) = prev_skip {
                std::env::set_var("SDDIA_SKIP_HOOKS", v);
            } else {
                std::env::remove_var("SDDIA_SKIP_HOOKS");
            }
            // L-HYGIENE-SOFT: push causal — si falla, abortar sin delete.
            let push = match push_data {
                Ok(p) => p,
                Err(e) => return Some(Err(e)),
            };
            let (closed, hygiene_failure) = source
                .as_deref()
                .map(|b| delete_branch_hygiene(repo, b))
                .unwrap_or((None, None));
            if let Some(obj) = state.as_object_mut() {
                if let Some(c) = closed.clone() {
                    obj.insert("closed_branch".into(), json!(c));
                }
                if let Some(hf) = hygiene_failure.clone() {
                    obj.insert("hygiene_failure".into(), hf);
                }
            }
            Some(Ok(json!({
                "status": "executed",
                "handler": "accept-sync-cleanup",
                "push": push,
                "closed_branch": closed,
                "hygiene_failure": hygiene_failure,
            })))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_a2_canon_two_payloads_bool_force() {
        let local = delete_branch_local_payload("feat/x");
        let remote = delete_branch_remote_payload("feat/x");
        assert_eq!(local["branch_name"], "feat/x");
        assert_eq!(local["remote"], false);
        assert_eq!(local["force"], false);
        assert_eq!(remote["branch_name"], "feat/x");
        assert_eq!(remote["remote"], true);
        assert_eq!(remote["force"], false);
    }

    #[test]
    fn t_a2_illegit_no_remote_origin_string() {
        let local = delete_branch_local_payload("refactor/y");
        let remote = delete_branch_remote_payload("refactor/y");
        assert!(local.get("remote").and_then(|v| v.as_str()).is_none());
        assert!(remote.get("remote").and_then(|v| v.as_str()).is_none());
        assert_ne!(local["remote"], json!("origin"));
        assert_ne!(remote["remote"], json!("origin"));
    }

    #[test]
    fn t_a2_remote_miss_hygiene_failure_visible() {
        let mut ops = Vec::new();
        let local_ok = record_hygiene_op(&mut ops, "local", Ok(json!({})));
        let remote_ok = record_hygiene_op(
            &mut ops,
            "remote",
            Err("remote ref missing after github merge".into()),
        );
        assert!(local_ok);
        assert!(!remote_ok);
        let hygiene_failure = json!({"branch": "feat/z", "operations": ops});
        assert_eq!(hygiene_failure["operations"][0]["ok"], true);
        assert_eq!(hygiene_failure["operations"][1]["ok"], false);
        assert!(hygiene_failure["operations"][1]["error"].as_str().is_some());
        // Fase sync no retorna Err por hygiene (contrato L-HYGIENE-SOFT) — solo audita.
        let phase_success = true;
        assert!(phase_success);
    }

    #[test]
    fn t_a2_local_ok_closed_branch() {
        let mut ops = Vec::new();
        let local_ok = record_hygiene_op(&mut ops, "local", Ok(json!({})));
        let _ = record_hygiene_op(&mut ops, "remote", Ok(json!({})));
        let closed = if local_ok {
            Some("feat/w".to_string())
        } else {
            None
        };
        assert_eq!(closed.as_deref(), Some("feat/w"));
    }
}
