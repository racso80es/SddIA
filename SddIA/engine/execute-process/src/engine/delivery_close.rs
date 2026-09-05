//! Proceso `delivery-close-cycle` nativo (P5 — fases skill/tool/action).

use super::phase_capsules::{
    capsule_eda_genomic_audit_gate, capsule_evolution_audit_gate,
    capsule_index_integrity_audit_gate, execute_delivery_close_phase,
};
use super::route_domain_core::materialize_pending_domain_event;
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

fn dcc_physical_threshold_crossed(state: &Value) -> bool {
    let has_pr = state
        .get("pr_url")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_some();
    let pushed = state.get("delivery_push").is_some();
    has_pr || pushed
}

/// L-FAILSOFT-RETRO (PPR #187): adjudicación retroactiva de `fail_soft` sobre
/// `"Aduana EDA genómica"` cuando el umbral físico ya cruzó (huérfanos preexistentes).
/// Idempotente. No debilita la señal Argos (`argos_verdict: block` se conserva).
pub(crate) fn adjudicate_eda_fail_soft_post_physical(phase_reports: &mut [Value], state: &Value) {
    if !dcc_physical_threshold_crossed(state) {
        return;
    }
    for report in phase_reports.iter_mut() {
        let phase_name = report
            .get("phase_name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if phase_name != "Aduana EDA genómica" {
            continue;
        }
        let status = report.get("status").and_then(|v| v.as_str()).unwrap_or("");
        if status != "blocked" && status != "failed" {
            continue;
        }
        let orphan_count = report
            .get("orphan_count")
            .and_then(|v| v.as_i64().or_else(|| v.as_u64().map(|u| u as i64)))
            .unwrap_or(0);
        if orphan_count <= 0 {
            continue;
        }
        if report.get("argos_verdict").and_then(|v| v.as_str()) != Some("block") {
            continue;
        }
        report["fail_soft"] = json!(true);
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

    if phase_name == "Aduana evolution" {
        match capsule_evolution_audit_gate(repo, inputs, state) {
            Ok(gate) => {
                if let Some(obj) = gate.as_object() {
                    for (k, v) in obj {
                        entry[k.clone()] = v.clone();
                    }
                }
                return entry;
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["error"] = json!(e);
                return entry;
            }
        }
    }

    if phase_name == "Aduana integridad índices" {
        match capsule_index_integrity_audit_gate(repo, inputs, state) {
            Ok(gate) => {
                if let Some(obj) = gate.as_object() {
                    for (k, v) in obj {
                        entry[k.clone()] = v.clone();
                    }
                }
                return entry;
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["error"] = json!(e);
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
                entry["error"] = json!(e);
                if stamp_dcc_network_block(&mut entry, phase_name, &e) {
                    return entry;
                }
                if stamp_dcc_hook_evol_block(&mut entry, phase_name, &e) {
                    return entry;
                }
                if stamp_dcc_workflow_scope_block(&mut entry, phase_name, &e) {
                    return entry;
                }
                entry["status"] = json!("failed");
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
        super::phase_capsules::try_invoke_delegates(repo, &delegates, inputs, &[], "delivery-close-cycle", "", state)
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

fn dcc_friction_id(phase_name: &str, report: &Value) -> String {
    if let Some(id) = report.get("friction_id").and_then(|v| v.as_str()) {
        return id.to_string();
    }
    match phase_name {
        "Aduana EDA genómica" => "F-DCC-EDA-GENOMIC-BLOCK".into(),
        "Aduana evolution" => "F-DCC-EVOLUTION-GATE".into(),
        "Aduana integridad índices" => "F-DCC-INDEX-INTEGRITY".into(),
        _ => format!(
            "F-DCC-{}",
            phase_name
                .to_uppercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
                .collect::<String>()
        ),
    }
}

fn dcc_gate_block_suppresses_fracture(phase_name: &str, status: &str) -> bool {
    status == "blocked"
        && matches!(
            phase_name,
            "Aduana evolution" | "Aduana EDA genómica"
        )
}

fn dcc_transient_network_trace(trace: &str) -> bool {
    let t = trace.to_lowercase();
    t.contains("could not resolve host")
        || t.contains("temporary failure in name resolution")
        || t.contains("name or service not known")
        || t.contains("network is unreachable")
        || t.contains("connection timed out")
}

fn dcc_net_block_suppresses_fracture(phase_name: &str, status: &str, error_trace: &str) -> bool {
    matches!(status, "failed" | "blocked")
        && matches!(phase_name, "Publicación remota" | "Apertura en forja")
        && dcc_transient_network_trace(error_trace)
}

fn dcc_hook_evol_gate_trace(trace: &str) -> bool {
    trace
        .to_lowercase()
        .contains("evolution gate (--range --if-touched) failed")
}

fn dcc_hook_evol_block_suppresses_fracture(
    phase_name: &str,
    status: &str,
    error_trace: &str,
) -> bool {
    matches!(status, "failed" | "blocked")
        && phase_name == "Publicación remota"
        && dcc_hook_evol_gate_trace(error_trace)
}

fn dcc_workflow_scope_trace(trace: &str) -> bool {
    let t = trace.to_lowercase();
    t.contains("without") && t.contains("workflow") && t.contains("scope")
}

fn dcc_workflow_scope_block_suppresses_fracture(
    phase_name: &str,
    status: &str,
    error_trace: &str,
) -> bool {
    matches!(status, "failed" | "blocked")
        && phase_name == "Publicación remota"
        && dcc_workflow_scope_trace(error_trace)
}

fn dcc_title_metachar_trace(trace: &str) -> bool {
    trace.contains("PR_TITLE_METACHAR") || trace.contains("F-DCC-PR-TITLE-METACHAR")
}

fn dcc_title_metachar_block_suppresses_fracture(
    phase_name: &str,
    status: &str,
    error_trace: &str,
    report: &Value,
) -> bool {
    matches!(status, "failed" | "blocked")
        && phase_name == "Apertura en forja"
        && (dcc_title_metachar_trace(error_trace)
            || report.get("friction_id").and_then(|v| v.as_str())
                == Some("F-DCC-PR-TITLE-METACHAR")
            || report.get("error_code").and_then(|v| v.as_str()) == Some("PR_TITLE_METACHAR"))
}

/// Receta de compile (ELF/cápsula ausente) ≠ colapso ontológico. No `fail_soft`.
fn dcc_lab_binary_missing_trace(trace: &str) -> bool {
    let t = trace.to_lowercase();
    if t.contains("sddia-qa no encontrado") {
        return true;
    }
    if t.contains("shell-executor wasm fallback marker") {
        return true;
    }
    t.contains("cápsula skill") && t.contains("no encontrada bajo sddia/target")
}

fn dcc_lab_binary_missing_suppresses_fracture(
    _phase_name: &str,
    status: &str,
    error_trace: &str,
) -> bool {
    matches!(status, "failed" | "blocked") && dcc_lab_binary_missing_trace(error_trace)
}

fn dcc_post_push_phase(phase_name: &str) -> bool {
    matches!(
        phase_name,
        "Apertura en forja" | "Sello Presentación ECST" | "Higiene local"
    )
}

fn dcc_push_terminal_halt(status: &str) -> bool {
    matches!(status, "failed" | "blocked")
}

fn current_branch_from_list(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        let t = line.trim();
        let Some(rest) = t.strip_prefix("* ") else {
            continue;
        };
        let name = rest.split_whitespace().next().unwrap_or("");
        if name.is_empty() || name.starts_with('(') {
            return None;
        }
        return Some(name.to_string());
    }
    None
}

fn resolve_symbolic_head_branch(repo: &Path, inputs: &mut Value) -> Result<(), String> {
    let raw = inputs
        .get("branch_name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("");
    if !raw.eq_ignore_ascii_case("head") {
        return Ok(());
    }
    let data = super::capsules::invoke_git_manager(repo, "branch_list", &json!({}))?;
    let stdout = data
        .get("gitStdout")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let Some(name) = current_branch_from_list(stdout) else {
        return Err(
            "branch_name=HEAD simbólico y no hay rama actual (detached); abortar DCC".into(),
        );
    };
    if let Some(obj) = inputs.as_object_mut() {
        obj.insert("branch_name".into(), json!(name));
    }
    Ok(())
}

fn dcc_report_error_trace(report: &Value) -> String {
    report
        .get("error")
        .and_then(|v| v.as_str())
        .or_else(|| report.get("message").and_then(|v| v.as_str()))
        .or_else(|| report.get("status").and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string()
}

/// F4c: DNS/red transitoria en push/forja → `blocked` accionable, no Kintsugi.
fn stamp_dcc_network_block(entry: &mut Value, phase_name: &str, error: &str) -> bool {
    if !dcc_net_block_suppresses_fracture(phase_name, "failed", error) {
        return false;
    }
    entry["status"] = json!("blocked");
    entry["friction_id"] = json!("F-DCC-DNS-UNRESOLVED");
    true
}

/// F-DCC-HOOK-EVOL-OVERESCALATION: pre-push evolution gate ≠ colapso Kintsugi.
fn stamp_dcc_hook_evol_block(entry: &mut Value, phase_name: &str, error: &str) -> bool {
    if !dcc_hook_evol_block_suppresses_fracture(phase_name, "failed", error) {
        return false;
    }
    entry["status"] = json!("blocked");
    entry["friction_id"] = json!("F-DCC-HOOK-EVOL-OVERESCALATION");
    true
}

/// F-DCC-WORKFLOW-SCOPE: PAT git sin scope workflow ≠ recursión hook.
fn stamp_dcc_workflow_scope_block(entry: &mut Value, phase_name: &str, error: &str) -> bool {
    if !dcc_workflow_scope_block_suppresses_fracture(phase_name, "failed", error) {
        return false;
    }
    entry["status"] = json!("blocked");
    entry["friction_id"] = json!("F-DCC-WORKFLOW-SCOPE");
    entry["operator_hint"] = json!(
        "Unificar credential helper git→gh (`gh auth setup-git`). `gh auth refresh -s workflow` solo si git ya delega en gh."
    );
    true
}

pub(crate) fn emit_dcc_phase_fractures(repo: &Path, phase_reports: &[Value]) {
    for report in phase_reports {
        if report.get("fail_soft").and_then(|v| v.as_bool()) == Some(true) {
            continue;
        }
        let status = report.get("status").and_then(|v| v.as_str()).unwrap_or("");
        if status != "blocked" && status != "failed" {
            continue;
        }
        let phase_name = report
            .get("phase_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        if dcc_gate_block_suppresses_fracture(phase_name, status) {
            continue;
        }
        let error_trace = dcc_report_error_trace(report);
        if dcc_net_block_suppresses_fracture(phase_name, status, &error_trace) {
            continue;
        }
        if dcc_hook_evol_block_suppresses_fracture(phase_name, status, &error_trace) {
            continue;
        }
        if dcc_workflow_scope_block_suppresses_fracture(phase_name, status, &error_trace) {
            continue;
        }
        if dcc_title_metachar_block_suppresses_fracture(phase_name, status, &error_trace, report) {
            continue;
        }
        if dcc_lab_binary_missing_suppresses_fracture(phase_name, status, &error_trace) {
            continue;
        }
        let friction_id = dcc_friction_id(phase_name, report);
        let payload = json!({
            "process_name": "delivery-close-cycle",
            "error_trace": error_trace,
            "agent_emitter": "execute-process",
            "attempted_action": phase_name,
            "friction_id": friction_id,
        });
        let _ = materialize_pending_domain_event(
            repo,
            "System_Fracture_Detected",
            "execute-process",
            payload,
        );
    }
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
    resolve_symbolic_head_branch(repo, &mut inputs_mut)?;

    let mut phase_reports: Vec<Value> = Vec::new();
    let mut halt_after_push = false;
    for phase in phases {
        let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
        if halt_after_push && dcc_post_push_phase(phase_name) {
            phase_reports.push(json!({
                "phase_name": phase_name,
                "status": "skipped",
                "skipped": true,
                "reason": "prior_push_not_ok",
            }));
            continue;
        }
        let entry = execute_phase(repo, phase, &inputs_mut, &mut state);
        if phase_name == "Publicación remota" {
            let st = entry.get("status").and_then(|v| v.as_str()).unwrap_or("");
            if dcc_push_terminal_halt(st) {
                halt_after_push = true;
            }
        }
        phase_reports.push(entry);
    }
    emit_dcc_phase_fractures(repo, &phase_reports);
    adjudicate_eda_fail_soft_post_physical(&mut phase_reports, &state);
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
    use std::fs;

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

    #[test]
    fn dcc_fracture_suppressed_on_evolution_gate_block() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let reports = vec![json!({
            "phase_name": "Aduana evolution",
            "status": "blocked",
            "message": "EVOL_MATERIAL_UNREGISTERED",
        })];
        emit_dcc_phase_fractures(repo, &reports);
        let pending: Vec<_> = fs::read_dir(repo.join(".events/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(pending.is_empty());
    }

    #[test]
    fn dcc_fracture_emits_on_failed_forge_phase() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let reports = vec![json!({
            "phase_name": "Apertura en forja",
            "status": "failed",
            "error": "no se pudo resolver pr_url desde gh",
        })];
        emit_dcc_phase_fractures(repo, &reports);
        let pending: Vec<_> = fs::read_dir(repo.join(".events/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(!pending.is_empty());
    }

    #[test]
    fn dcc_transient_network_trace_positives_and_pr_url_negative() {
        assert!(dcc_transient_network_trace(
            "fatal: Could not resolve host: github.com"
        ));
        assert!(dcc_transient_network_trace(
            "Temporary failure in name resolution"
        ));
        assert!(dcc_transient_network_trace("Name or service not known"));
        assert!(dcc_transient_network_trace("Network is unreachable"));
        assert!(dcc_transient_network_trace("Connection timed out"));
        assert!(!dcc_transient_network_trace(
            "no se pudo resolver pr_url desde gh"
        ));
    }

    #[test]
    fn dcc_fracture_suppressed_on_remote_push_dns() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let reports = vec![json!({
            "phase_name": "Publicación remota",
            "status": "failed",
            "error": "fatal: no es posible acceder a 'https://github.com/racso80es/SddIA.git/': Could not resolve host: github.com",
        })];
        emit_dcc_phase_fractures(repo, &reports);
        let pending: Vec<_> = fs::read_dir(repo.join(".events/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(pending.is_empty());
    }

    #[test]
    fn dcc_fracture_suppressed_on_forge_dns() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let reports = vec![json!({
            "phase_name": "Apertura en forja",
            "status": "failed",
            "error": "Could not resolve host: github.com",
        })];
        emit_dcc_phase_fractures(repo, &reports);
        let pending: Vec<_> = fs::read_dir(repo.join(".events/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(pending.is_empty());
    }

    #[test]
    fn stamp_dcc_network_block_sets_friction_and_aggregator_fails() {
        let mut entry = json!({
            "phase_name": "Publicación remota",
            "error": "Could not resolve host: github.com",
        });
        assert!(stamp_dcc_network_block(
            &mut entry,
            "Publicación remota",
            "Could not resolve host: github.com",
        ));
        assert_eq!(entry["status"], "blocked");
        assert_eq!(entry["friction_id"], "F-DCC-DNS-UNRESOLVED");
        assert!(entry.get("fail_soft").is_none());
        let v = super::super::phase_terminal::aggregate_execution_terminal(
            &[entry],
            &json!({}),
        );
        assert!(!v.success);
        assert_eq!(v.status_code, 1);
    }

    #[test]
    fn dcc_hook_evol_gate_trace_matches_canonical() {
        assert!(dcc_hook_evol_gate_trace(
            "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed"
        ));
        assert!(!dcc_hook_evol_gate_trace(
            "SddIA pre-push: BLOCKED — delivery-close-cycle failed for feat/x"
        ));
        assert!(!dcc_hook_evol_gate_trace(
            "Could not resolve host: github.com"
        ));
    }

    #[test]
    fn dcc_fracture_suppressed_on_remote_push_hook_evol_gate() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let reports = vec![json!({
            "phase_name": "Publicación remota",
            "status": "failed",
            "error": "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed\nerror: falló el empuje de algunas referencias a 'https://github.com/racso80es/SddIA.git'",
        })];
        emit_dcc_phase_fractures(repo, &reports);
        let pending: Vec<_> = fs::read_dir(repo.join(".events/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(pending.is_empty());
    }

    #[test]
    fn stamp_dcc_hook_evol_block_sets_friction() {
        let mut entry = json!({
            "phase_name": "Publicación remota",
            "error": "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed",
        });
        assert!(stamp_dcc_hook_evol_block(
            &mut entry,
            "Publicación remota",
            "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed",
        ));
        assert_eq!(entry["status"], "blocked");
        assert_eq!(entry["friction_id"], "F-DCC-HOOK-EVOL-OVERESCALATION");
        assert!(entry.get("fail_soft").is_none());
        assert!(!stamp_dcc_hook_evol_block(
            &mut entry,
            "Apertura en forja",
            "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed",
        ));
    }

    #[test]
    fn stamp_dcc_workflow_scope_block_sets_friction() {
        let err = "refusing to allow a Personal Access Token to create or update workflow `.github/workflows/sddia-index-qa.yml` without `workflow` scope";
        let mut entry = json!({
            "phase_name": "Publicación remota",
            "error": err,
        });
        assert!(stamp_dcc_workflow_scope_block(
            &mut entry,
            "Publicación remota",
            err,
        ));
        assert_eq!(entry["status"], "blocked");
        assert_eq!(entry["friction_id"], "F-DCC-WORKFLOW-SCOPE");
        assert!(entry["operator_hint"].as_str().unwrap().contains("setup-git"));
        assert!(entry.get("fail_soft").is_none());
        let v = super::super::phase_terminal::aggregate_execution_terminal(
            &[entry.clone()],
            &json!({}),
        );
        assert!(!v.success);
        assert_eq!(v.status_code, 1);
    }

    #[test]
    fn dcc_fracture_suppressed_on_workflow_scope() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let reports = vec![json!({
            "phase_name": "Publicación remota",
            "status": "blocked",
            "friction_id": "F-DCC-WORKFLOW-SCOPE",
            "error": "without `workflow` scope",
        })];
        emit_dcc_phase_fractures(repo, &reports);
        let pending: Vec<_> = fs::read_dir(repo.join(".events/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(pending.is_empty());
    }

    #[test]
    fn dcc_lab_binary_missing_trace_positives_and_negatives() {
        assert!(dcc_lab_binary_missing_trace(
            "sddia-qa no encontrado (compilar: cd SddIA && cargo build -p sddia-qa)"
        ));
        assert!(dcc_lab_binary_missing_trace(
            "SddIA pre-commit: sddia-qa no encontrado (compilar: cd SddIA && cargo build -p sddia-qa)"
        ));
        assert!(dcc_lab_binary_missing_trace(
            "cápsula skill 'git-manager' no encontrada bajo SddIA/target"
        ));
        assert!(dcc_lab_binary_missing_trace(
            "cápsula skill 'shell-executor' no encontrada bajo SddIA/target"
        ));
        assert!(dcc_lab_binary_missing_trace(
            "shell-executor wasm fallback marker"
        ));
        assert!(!dcc_lab_binary_missing_trace(
            "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed"
        ));
        assert!(!dcc_lab_binary_missing_trace(
            "proveedor 'skill:git-manager' revocado en revoked_entities"
        ));
    }

    fn pending_fracture_count(repo: &std::path::Path) -> usize {
        fs::read_dir(repo.join(".events/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .count()
    }

    #[test]
    fn dcc_fracture_suppressed_on_sddia_qa_missing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let err = "sddia-qa no encontrado (compilar: cd SddIA && cargo build -p sddia-qa)";
        let reports = vec![
            json!({
                "phase_name": "Aduana evolution",
                "status": "failed",
                "error": err,
            }),
            json!({
                "phase_name": "Aduana integridad índices",
                "status": "failed",
                "error": err,
            }),
        ];
        emit_dcc_phase_fractures(repo, &reports);
        assert_eq!(pending_fracture_count(repo), 0);
    }

    #[test]
    fn dcc_fracture_suppressed_on_shell_executor_wasm_fallback_marker() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        emit_dcc_phase_fractures(
            repo,
            &[json!({
                "phase_name": "Apertura en forja",
                "status": "failed",
                "error": "shell-executor wasm fallback marker",
            })],
        );
        assert_eq!(pending_fracture_count(repo), 0);
    }

    #[test]
    fn dcc_fracture_suppressed_on_git_manager_capsule_missing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let err = "cápsula skill 'git-manager' no encontrada bajo SddIA/target";
        let reports = vec![
            json!({
                "phase_name": "Snapshot final",
                "status": "failed",
                "error": err,
            }),
            json!({
                "phase_name": "Publicación remota",
                "status": "failed",
                "error": err,
            }),
        ];
        emit_dcc_phase_fractures(repo, &reports);
        assert_eq!(pending_fracture_count(repo), 0);
    }

    #[test]
    fn dcc_fracture_still_emits_on_rbac_revocation_and_evol_gate_failed() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        emit_dcc_phase_fractures(
            repo,
            &[json!({
                "phase_name": "Snapshot final",
                "status": "failed",
                "error": "proveedor 'skill:git-manager' revocado en revoked_entities",
            })],
        );
        assert!(pending_fracture_count(repo) >= 1);
        let tmp2 = tempfile::tempdir().expect("tempdir");
        let repo2 = tmp2.path();
        fs::create_dir_all(repo2.join(".events/pending")).unwrap();
        emit_dcc_phase_fractures(
            repo2,
            &[json!({
                "phase_name": "Aduana evolution",
                "status": "failed",
                "error": "evolution gate (--range --if-touched) failed",
            })],
        );
        assert!(pending_fracture_count(repo2) >= 1);
    }

    #[test]
    fn dcc_fracture_suppressed_on_forge_title_metachar() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let reports = vec![json!({
            "phase_name": "Apertura en forja",
            "status": "blocked",
            "error_code": "PR_TITLE_METACHAR",
            "friction_id": "F-DCC-PR-TITLE-METACHAR",
            "error": "[PR_TITLE_METACHAR] arguments[3] contains forbidden shell metacharacters",
        })];
        emit_dcc_phase_fractures(repo, &reports);
        let pending: Vec<_> = fs::read_dir(repo.join(".events/pending"))
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(pending.is_empty());
    }

    #[test]
    fn dcc_halt_skips_post_push_phases() {
        assert!(dcc_post_push_phase("Apertura en forja"));
        assert!(dcc_post_push_phase("Sello Presentación ECST"));
        assert!(dcc_post_push_phase("Higiene local"));
        assert!(!dcc_post_push_phase("Publicación remota"));
        assert!(dcc_push_terminal_halt("failed"));
        assert!(dcc_push_terminal_halt("blocked"));
        assert!(!dcc_push_terminal_halt("executed"));
    }

    #[test]
    fn current_branch_from_list_reads_star() {
        let stdout = "  main abc\n* feat/eda-telegram-notify-pr-merged def msg\n";
        assert_eq!(
            current_branch_from_list(stdout).as_deref(),
            Some("feat/eda-telegram-notify-pr-merged")
        );
        assert!(current_branch_from_list("* (HEAD detached at abc)\n").is_none());
    }

    fn eda_blocked_orphans_report() -> Value {
        json!({
            "phase_name": "Aduana EDA genómica",
            "status": "blocked",
            "argos_verdict": "block",
            "orphan_count": 2,
            "handler": "eda-genomic-audit",
        })
    }

    #[test]
    fn eda_blocked_with_pr_url_gets_fail_soft_and_aggregator_success() {
        let mut reports = vec![eda_blocked_orphans_report()];
        let state = json!({"pr_url": "https://github.com/x/y/pull/187"});
        adjudicate_eda_fail_soft_post_physical(&mut reports, &state);
        assert_eq!(reports[0]["fail_soft"], true);
        assert_eq!(reports[0]["argos_verdict"], "block");
        assert_eq!(reports[0]["status"], "blocked");
        let v = super::super::phase_terminal::aggregate_execution_terminal(&reports, &state);
        assert!(v.success);
        assert_eq!(v.status_code, 0);
    }

    #[test]
    fn eda_blocked_without_physical_stays_causal() {
        let mut reports = vec![eda_blocked_orphans_report()];
        let state = json!({});
        adjudicate_eda_fail_soft_post_physical(&mut reports, &state);
        assert!(reports[0].get("fail_soft").is_none());
        let v = super::super::phase_terminal::aggregate_execution_terminal(&reports, &state);
        assert!(!v.success);
        assert_eq!(v.status_code, 1);
    }

    #[test]
    fn eda_blocked_with_delivery_push_only_gets_fail_soft() {
        let mut reports = vec![eda_blocked_orphans_report()];
        let state = json!({"delivery_push": {"ok": true}});
        adjudicate_eda_fail_soft_post_physical(&mut reports, &state);
        assert_eq!(reports[0]["fail_soft"], true);
        let v = super::super::phase_terminal::aggregate_execution_terminal(&reports, &state);
        assert!(v.success);
        assert_eq!(v.status_code, 0);
    }

    #[test]
    fn adjudicate_eda_fail_soft_is_idempotent() {
        let mut reports = vec![eda_blocked_orphans_report()];
        let state = json!({"pr_url": "https://github.com/x/y/pull/1"});
        adjudicate_eda_fail_soft_post_physical(&mut reports, &state);
        adjudicate_eda_fail_soft_post_physical(&mut reports, &state);
        assert_eq!(reports[0]["fail_soft"], true);
    }

    #[test]
    fn evolution_phase_blocks_unregistered_material_ca12() {
        use std::fs;
        use std::path::PathBuf;
        use std::process::Command;
        use uuid::Uuid;

        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.pop();
        root.pop();
        root.pop();
        let qa = root.join("SddIA/target/debug/sddia-qa");
        if !qa.is_file() {
            panic!("compilar sddia-qa: cd SddIA && cargo build -p sddia-qa");
        }
        let wt = root.join("target/ca12-worktrees").join(Uuid::new_v4().to_string());
        fs::create_dir_all(wt.parent().expect("parent")).expect("mkdir");
        assert!(
            Command::new("git")
                .args(["worktree", "add", "-q", wt.to_str().expect("wt"), "HEAD"])
                .current_dir(&root)
                .status()
                .expect("worktree")
                .success()
        );
        let qa_dst = wt.join("SddIA/target");
        if qa_dst.exists() {
            let _ = fs::remove_dir_all(&qa_dst);
        }
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(root.join("SddIA/target"), &qa_dst).expect("symlink target");
        }
        let probe = wt.join("SddIA/tools/_ca12_smoke_probe.txt");
        fs::create_dir_all(probe.parent().expect("parent")).expect("mkdir");
        fs::write(&probe, "probe\n").expect("write");
        assert!(
            Command::new("git")
                .args(["add", "SddIA/tools/_ca12_smoke_probe.txt"])
                .current_dir(&wt)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            Command::new("git")
                .args([
                    "-c",
                    "user.email=ca12@test",
                    "-c",
                    "user.name=ca12",
                    "commit",
                    "-m",
                    "test: ca12 dcc",
                    "--no-verify",
                ])
                .current_dir(&wt)
                .status()
                .unwrap()
                .success()
        );
        let phase = json!({"name": "Aduana evolution", "delegates_to": []});
        let mut state = json!({});
        let entry = execute_phase(&wt, &phase, &json!({}), &mut state);
        let _ = Command::new("git")
            .args(["worktree", "remove", "-f"])
            .arg(&wt)
            .current_dir(&root)
            .status();
        assert_eq!(
            entry.get("status").and_then(|v| v.as_str()),
            Some("blocked"),
            "entry={entry}"
        );
        let codes = entry
            .get("reason_codes")
            .and_then(|v| v.as_array())
            .expect("reason_codes");
        assert!(
            codes
                .iter()
                .any(|c| c.as_str() == Some("EVOL_MATERIAL_UNREGISTERED"))
        );
    }
}

