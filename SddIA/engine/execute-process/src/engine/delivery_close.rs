//! Proceso `delivery-close-cycle` nativo (P5 — fases skill/tool/action).

use super::phase_capsules::{capsule_eda_genomic_audit_gate, execute_delivery_close_phase};
use super::thermodynamic;
use super::workspace::bootstrap_workspace;
use crate::core::resolver::{validate_process_inputs, ProcessDef};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;
use std::time::Instant;
use uuid::Uuid;

fn workspace_template(process_def: &ProcessDef) -> Result<String, String> {
    process_def
        .get("workspace_template")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "workspace_template ausente en definición del proceso".into())
}

fn delegates_are_only_agents(delegates: &[Value]) -> bool {
    delegates.iter().all(|d| {
        d.as_str()
            .map(|s| s.starts_with("agent:"))
            .unwrap_or(false)
    })
}

/// Fases secundarias post umbral físico (push + pr_url): no bloquean sello Presented.
fn is_dcc_secondary_phase(phase_name: &str) -> bool {
    matches!(
        phase_name,
        "Impacto SddIA condicional" | "Higiene local"
    )
}

pub(crate) fn mark_fail_soft_if_secondary(entry: &mut Value, phase_name: &str, state: &Value) {
    let has_pr = state
        .get("pr_url")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_some();
    let pushed = state.get("delivery_push").is_some();
    if (has_pr || pushed) && is_dcc_secondary_phase(phase_name) {
        if entry.get("status").and_then(|v| v.as_str()) == Some("failed")
            || entry.get("status").and_then(|v| v.as_str()) == Some("blocked")
        {
            entry["fail_soft"] = json!(true);
        }
    }
}

fn execute_phase(
    repo: &Path,
    phase: &Value,
    inputs: &Value,
    state: &mut Value,
) -> Value {
    let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let delegates = phase
        .get("delegates_to")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut entry = json!({
        "phase_name": phase_name,
        "delegates_to": delegates,
    });

    if phase_name == "Aduana EDA genómica" {
        match capsule_eda_genomic_audit_gate(repo, inputs, state) {
            Ok(gate) => {
                if let Some(obj) = gate.as_object() {
                    for (k, v) in obj {
                        entry[k.clone()] = v.clone();
                    }
                }
                mark_fail_soft_if_secondary(&mut entry, phase_name, state);
                return entry;
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["error"] = json!(e);
                mark_fail_soft_if_secondary(&mut entry, phase_name, state);
                return entry;
            }
        }
    }

    if let Some(result) = execute_delivery_close_phase(repo, phase_name, inputs, state) {
        return match result {
            Ok(phase_entry) => {
                if let Some(obj) = phase_entry.as_object() {
                    for (k, v) in obj {
                        entry[k.clone()] = v.clone();
                    }
                }
                mark_fail_soft_if_secondary(&mut entry, phase_name, state);
                entry
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["error"] = json!(e);
                // L-FAILSOFT-OLA2: telemetría/validación secundaria post pr_url.
                let soft_err = {
                    let el = e.to_lowercase();
                    el.contains("timeout")
                        || el.contains("telemetry")
                        || el.contains("receipt")
                        || el.contains("validaci")
                };
                let has_pr = state
                    .get("pr_url")
                    .and_then(|v| v.as_str())
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .is_some();
                if has_pr && (is_dcc_secondary_phase(phase_name) || soft_err) {
                    entry["fail_soft"] = json!(true);
                }
                mark_fail_soft_if_secondary(&mut entry, phase_name, state);
                entry
            }
        };
    }

    if delegates_are_only_agents(&delegates) {
        entry["status"] = json!("simulated");
        entry["note"] = json!("agentes IDE; sin handler físico en laboratorio");
        return entry;
    }

    if let Some(capsule_entry) =
        super::phase_capsules::try_invoke_delegates(repo, &delegates, inputs, &[], "delivery-close-cycle", "")
    {
        if let Some(obj) = capsule_entry.as_object() {
            for (k, v) in obj {
                entry[k.clone()] = v.clone();
            }
        }
        mark_fail_soft_if_secondary(&mut entry, phase_name, state);
        return entry;
    }

    if delegates.iter().any(|d| {
        d.as_str()
            .map(|s| s.starts_with("skill:") || s.starts_with("tool:"))
            .unwrap_or(false)
    }) {
        entry["status"] = json!("simulated");
        entry["note"] = json!("cápsula ausente en compiled_capsules (SSOT SddIA/target)");
        return entry;
    }

    entry["status"] = json!("simulated");
    entry
}

pub fn run(
    repo: &Path,
    process_name: &str,
    process_def: &ProcessDef,
    phases: &[Value],
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    validate_process_inputs(process_def, process_inputs, process_name)?;

    let toll_start = Some(Instant::now());
    let mut state = json!({
        "handoff": {},
        "inputs": process_inputs,
        "asset_id": Uuid::new_v4().to_string(),
    });

    let template = workspace_template(process_def)?;
    let mut inputs_mut = process_inputs.clone();
    bootstrap_workspace(repo, process_name, &template, &mut inputs_mut, &mut state)?;

    let mut phase_reports: Vec<Value> = Vec::new();
    for phase in phases {
        phase_reports.push(execute_phase(repo, phase, &inputs_mut, &mut state));
    }
    state["phase_reports"] = json!(phase_reports);

    let verdict =
        super::phase_terminal::aggregate_execution_terminal(&phase_reports, &state);
    let status_code = verdict.status_code;
    let duration_ms = toll_start
        .map(|t| t.elapsed().as_millis() as i64)
        .unwrap_or(0);

    let mut data = json!({
        "process_name": process_name,
        "handoff": state.get("handoff").cloned().unwrap_or(json!({})),
    });
    for key in [
        "workspace_path",
        "execution_id",
        "pr_url",
        "event_id",
        "target_path",
        "closed_branch",
        "snapshot_commit_hash",
        "delivery_close",
        "delivery_push",
    ] {
        if let Some(v) = state.get(key) {
            data[key] = v.clone();
        }
    }
    super::phase_terminal::apply_failed_phase_fields(&mut data, &verdict);

    let toll = thermodynamic::run(
        repo,
        process_name,
        &state,
        &inputs_mut,
        status_code,
        duration_ms,
        verdict.success,
    );
    data["thermodynamic_toll"] = toll;

    Ok(OrchestratorEnvelope {
        success: verdict.success,
        status_code,
        data: Some(data),
        error: verdict.error.clone().or_else(|| {
            if verdict.success {
                None
            } else {
                Some("delivery-close-cycle con fases blocked/failed".into())
            }
        }),
        execution_report: Some(json!({
            "process_name": process_name,
            "phases": phase_reports,
        })),
        exit_code: status_code,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dcc_hygiene_failed_is_fail_soft_when_pr_url_present() {
        let mut entry = json!({
            "phase_name": "Higiene local",
            "status": "failed",
            "error": "timeout telemetry_receipt",
        });
        mark_fail_soft_if_secondary(
            &mut entry,
            "Higiene local",
            &json!({"pr_url": "https://github.com/x/y/pull/1"}),
        );
        assert_eq!(entry["fail_soft"], true);
        let v = super::super::phase_terminal::aggregate_execution_terminal(
            &[entry],
            &json!({}),
        );
        assert!(v.success);
    }

    #[test]
    fn dcc_hygiene_failed_without_pr_stays_causal() {
        let mut entry = json!({
            "phase_name": "Higiene local",
            "status": "failed",
        });
        mark_fail_soft_if_secondary(&mut entry, "Higiene local", &json!({}));
        assert!(entry.get("fail_soft").is_none());
    }

    #[test]
    fn dcc_snapshot_failed_never_fail_soft() {
        let mut entry = json!({
            "phase_name": "Snapshot final",
            "status": "failed",
        });
        mark_fail_soft_if_secondary(
            &mut entry,
            "Snapshot final",
            &json!({"pr_url": "https://github.com/x/y/pull/1"}),
        );
        assert!(entry.get("fail_soft").is_none());
    }
}

