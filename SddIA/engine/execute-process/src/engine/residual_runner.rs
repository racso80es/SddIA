//! Motor residual nativo — reemplaza `delegate_python` / capsules bridge.

use super::accept_pr::execute_accept_pr_phase;
use super::invoke_orchestrator::invoke_process_full;
use super::phase_capsules::{
    capsule_eda_genomic_audit_gate, execute_delivery_close_phase, execute_feature_phase,
    try_invoke_delegates,
};
use super::route_fractal_core::{
    invoke_radamanto_batch, invoke_route_fractal, invoke_telemetry_compliance,
};
use super::telemetry_batch_stub::run_telemetry_batch_stub;
use super::thermodynamic;
use super::workspace::bootstrap_workspace;
use crate::core::resolver::{validate_process_inputs, ProcessDef};
use crate::envelope::OrchestratorEnvelope;
use crate::forges::materialize_by_inputs;
use serde_json::{json, Value};
use std::path::Path;
use std::time::Instant;
use uuid::Uuid;

const CHAOS_AUDIT: &[&str] = &[
    "audit-thermodynamic-toll-failsoft",
    "audit-telemetry-compliance-breach",
    "audit-sandbox-isolation-rbac",
];

fn route_fractal_fn(process_name: &str) -> Option<&'static str> {
    match process_name {
        "route-telemetry" => Some("route_telemetry_event"),
        "route-orchestration" => Some("route_orchestration_event"),
        "route-domain" => Some("route_domain_fractal_event"),
        _ => None,
    }
}

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

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

fn is_creator(process_name: &str) -> bool {
    process_name.ends_with("-creator")
}

fn route_handler_result(canonical: &str, out: &Value, handler: &str) -> OrchestratorEnvelope {
    let ok = out.get("success").and_then(|v| v.as_bool()).unwrap_or(false)
        && out.get("exitCode").and_then(|v| v.as_i64()).unwrap_or(1) == 0;
    let status_code = out
        .get("exitCode")
        .and_then(|v| v.as_i64())
        .unwrap_or(if ok { 0 } else { 1 }) as i32;
    OrchestratorEnvelope {
        success: ok,
        status_code,
        data: out.get("data").cloned(),
        error: out
            .get("error")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        execution_report: Some(json!({
            "process_name": canonical,
            "phases": [{
                "phase_name": format!("Orquestación {canonical}"),
                "status": if ok { "executed" } else { "failed" },
                "handler": handler,
            }],
        })),
        exit_code: status_code,
    }
}

fn run_route_fractal_process(
    repo: &Path,
    canonical: &str,
    inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let rel = str_field(inputs, "event_file_path").ok_or("event_file_path requerido")?;
    let fn_name = route_fractal_fn(canonical).ok_or("route fractal desconocido")?;
    let out = invoke_route_fractal(repo, fn_name, &rel)?;
    Ok(route_handler_result(canonical, &out, &format!("{canonical}-core")))
}

fn run_radamanto_batch(
    repo: &Path,
    canonical: &str,
    phases: &[Value],
    inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let rel = str_field(inputs, "event_file_path").ok_or("event_file_path requerido")?;
    let mut phase_reports = Vec::new();
    let mut ok = true;
    for phase in phases {
        let name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
        if name == "Consumo batch Radamanto" {
            match invoke_radamanto_batch(repo, &rel) {
                Ok(result) if result.get("ok").and_then(|v| v.as_bool()) == Some(true) => {
                    phase_reports.push(json!({
                        "phase_name": name,
                        "status": "executed",
                        "handler": "radamanto-batch",
                        "entity_id": result.get("entity_id"),
                        "actions": result.get("actions"),
                        "purged": result.get("purged"),
                    }));
                }
                Ok(result) => {
                    ok = false;
                    phase_reports.push(json!({
                        "phase_name": name,
                        "status": "failed",
                        "handler": "radamanto-batch",
                        "error": result.get("error"),
                    }));
                }
                Err(e) => {
                    ok = false;
                    phase_reports.push(json!({
                        "phase_name": name,
                        "status": "failed",
                        "handler": "radamanto-batch",
                        "error": e,
                    }));
                }
            }
        } else {
            phase_reports.push(json!({
                "phase_name": name,
                "status": "simulated",
                "delegates_to": phase.get("delegates_to"),
            }));
        }
    }
    let status_code = if ok { 0 } else { 1 };
    Ok(OrchestratorEnvelope {
        success: ok,
        status_code,
        data: Some(json!({"process_name": canonical})),
        error: if ok { None } else { Some("radamanto-batch falló".into()) },
        execution_report: Some(json!({
            "process_name": canonical,
            "phases": phase_reports,
        })),
        exit_code: status_code,
    })
}

fn run_telemetry_compliance_audit(
    repo: &Path,
    canonical: &str,
    phases: &[Value],
    inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let rel = str_field(inputs, "event_file_path").ok_or("event_file_path requerido")?;
    let mut phase_reports = Vec::new();
    let mut ok = true;
    for phase in phases {
        let name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
        if name == "Auditoría cumplimiento termodinámico" {
            match invoke_telemetry_compliance(repo, &rel) {
                Ok(result) if result.get("ok").and_then(|v| v.as_bool()) == Some(true) => {
                    phase_reports.push(json!({
                        "phase_name": name,
                        "status": "executed",
                        "handler": "telemetry-compliance-audit",
                        "audit_status": result.get("status"),
                        "breach": result.get("breach"),
                    }));
                }
                Ok(result) => {
                    ok = false;
                    phase_reports.push(json!({
                        "phase_name": name,
                        "status": "failed",
                        "handler": "telemetry-compliance-audit",
                        "error": result.get("error"),
                    }));
                }
                Err(e) => {
                    ok = false;
                    phase_reports.push(json!({
                        "phase_name": name,
                        "status": "failed",
                        "error": e,
                    }));
                }
            }
        } else {
            phase_reports.push(json!({
                "phase_name": name,
                "status": "simulated",
                "delegates_to": phase.get("delegates_to"),
            }));
        }
    }
    let status_code = if ok { 0 } else { 1 };
    Ok(OrchestratorEnvelope {
        success: ok,
        status_code,
        data: Some(json!({"process_name": canonical})),
        error: if ok {
            None
        } else {
            Some("telemetry-compliance-audit falló".into())
        },
        execution_report: Some(json!({
            "process_name": canonical,
            "phases": phase_reports,
        })),
        exit_code: status_code,
    })
}

fn run_telemetry_batch_stub_process(
    repo: &Path,
    canonical: &str,
    phases: &[Value],
    inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let rel = str_field(inputs, "event_file_path").ok_or("event_file_path requerido")?;
    let mut phase_reports = Vec::new();
    let mut ok = true;
    for phase in phases {
        let name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
        if name == "Consumo batch stub" {
            match run_telemetry_batch_stub(repo, &rel) {
                Ok(result) => {
                    phase_reports.push(json!({
                        "phase_name": name,
                        "status": "executed",
                        "handler": "telemetry-batch-stub",
                        "event_id": result.get("event_id"),
                        "event_type": result.get("event_type"),
                        "purged": result.get("purged"),
                    }));
                }
                Err(e) => {
                    ok = false;
                    phase_reports.push(json!({
                        "phase_name": name,
                        "status": "failed",
                        "error": e,
                    }));
                }
            }
        } else {
            phase_reports.push(json!({
                "phase_name": name,
                "status": "simulated",
                "delegates_to": phase.get("delegates_to"),
            }));
        }
    }
    let status_code = if ok { 0 } else { 1 };
    Ok(OrchestratorEnvelope {
        success: ok,
        status_code,
        data: Some(json!({"process_name": canonical, "telemetry_consumed": ok})),
        error: None,
        execution_report: Some(json!({
            "process_name": canonical,
            "phases": phase_reports,
        })),
        exit_code: status_code,
    })
}

fn run_execute_suite(
    repo: &Path,
    canonical: &str,
    process_def: &ProcessDef,
    inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let suite_id = str_field(inputs, "suite_id").ok_or("suite_id requerido")?;
    let mut state = json!({"handoff": {}, "inputs": inputs});
    let template = workspace_template(process_def)?;
    let mut inputs_mut = inputs.clone();
    bootstrap_workspace(repo, canonical, &template, &mut inputs_mut, &mut state)?;

    let suite_path = repo.join("SddIA/suites").join(format!("{suite_id}.md"));
    if !suite_path.is_file() {
        return Err(format!("Suite no encontrada: {suite_id}"));
    }
    let fm = crate::core::parser::parse_frontmatter(&suite_path)?;
    let atomic_nodes = fm
        .get("atomic_nodes")
        .and_then(|v| serde_json::to_value(v).ok())
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default();
    let strategy = str_field(inputs, "execution_strategy")
        .or_else(|| fm.get("execution_strategy").and_then(|v| v.as_str()).map(str::to_string))
        .unwrap_or_else(|| "run_all".into());

    let mut phase_reports = vec![json!({
        "phase_name": "Resolución Suite",
        "status": "executed",
        "handler": "load-suite-spec",
        "suite_id": suite_id,
        "node_count": atomic_nodes.len(),
    })];

    let orchestrator_ws = state
        .get("workspace_path")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let mut node_reports = Vec::new();
    let mut abort = false;
    for (index, node) in atomic_nodes.iter().enumerate() {
        if abort {
            break;
        }
        let process_name = node
            .get("process_name")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty());
        let Some(process_name) = process_name else {
            continue;
        };
        let expected_exit = node
            .get("expected_exit_code")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let child_execution_id = Uuid::new_v4().to_string();
        let child_ws = Path::new(&orchestrator_ws).join(format!("node-{index}-{process_name}"));
        std::fs::create_dir_all(&child_ws).ok();
        let child_inputs = json!({
            "workspace_path": child_ws.to_string_lossy(),
            "execution_id": child_execution_id,
            "parent_execution_id": state.get("execution_id"),
            "parent_suite_id": suite_id,
        });
        let started = Instant::now();
        let body = invoke_process_full(repo, process_name, &child_inputs);
        let (actual_exit, child_error) = match body {
            Ok(b) => {
                let code = b.get("status_code").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                let err = if b.get("success").and_then(|v| v.as_bool()) != Some(true) {
                    b.get("error")
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                } else {
                    None
                };
                (code, err)
            }
            Err(e) => (1, Some(e)),
        };
        let duration_ms = started.elapsed().as_millis() as i64;
        let verdict = if actual_exit == expected_exit { "pass" } else { "fail" };
        let mut report = json!({
            "index": index,
            "process_name": process_name,
            "execution_id": child_execution_id,
            "workspace_path": child_ws.to_string_lossy(),
            "expected_exit_code": expected_exit,
            "actual_exit_code": actual_exit,
            "duration_ms": duration_ms,
            "verdict": verdict,
        });
        if let Some(e) = child_error {
            report["error"] = json!(e);
        }
        node_reports.push(report);
        if strategy == "fail_fast" && verdict != "pass" {
            abort = true;
        }
    }
    phase_reports.push(json!({
        "phase_name": "Ejecución nodos",
        "status": "executed",
        "handler": "execute-suite-nodes",
        "nodes": node_reports,
    }));
    let all_pass = node_reports.iter().all(|n| n.get("verdict") == Some(&json!("pass")));
    let status_code = if all_pass { 0 } else { 1 };
    Ok(OrchestratorEnvelope {
        success: all_pass,
        status_code,
        data: Some(json!({
            "process_name": canonical,
            "suite_id": suite_id,
            "node_reports": node_reports,
        })),
        error: if all_pass {
            None
        } else {
            Some("execute-suite: nodo falló".into())
        },
        execution_report: Some(json!({
            "process_name": canonical,
            "phases": phase_reports,
            "nodes": node_reports,
        })),
        exit_code: status_code,
    })
}

fn execute_phase(
    repo: &Path,
    phase: &Value,
    process_name: &str,
    _process_def: &ProcessDef,
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
                return entry;
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["error"] = json!(e);
                return entry;
            }
        }
    }

    if process_name == "delivery-close-cycle" {
        if let Some(result) = execute_delivery_close_phase(repo, phase_name, inputs, state) {
            return match result {
                Ok(phase_entry) => {
                    if let Some(obj) = phase_entry.as_object() {
                        for (k, v) in obj {
                            entry[k.clone()] = v.clone();
                        }
                    }
                    entry
                }
                Err(e) => {
                    entry["status"] = json!("failed");
                    entry["error"] = json!(e);
                    entry
                }
            };
        }
    }

    if process_name == "accept-pr" {
        if let Some(result) = execute_accept_pr_phase(repo, phase_name, inputs, state) {
            return match result {
                Ok(phase_entry) => {
                    if let Some(obj) = phase_entry.as_object() {
                        for (k, v) in obj {
                            entry[k.clone()] = v.clone();
                        }
                    }
                    entry
                }
                Err(e) => {
                    entry["status"] = json!("failed");
                    entry["error"] = json!(e);
                    entry
                }
            };
        }
    }

    if process_name == "feature" || process_name == "bug-fix" {
        if let Some(result) = execute_feature_phase(repo, phase_name, inputs, state) {
            return match result {
                Ok(phase_entry) => {
                    if let Some(obj) = phase_entry.as_object() {
                        for (k, v) in obj {
                            entry[k.clone()] = v.clone();
                        }
                    }
                    entry
                }
                Err(e) => {
                    entry["status"] = json!("failed");
                    entry["error"] = json!(e);
                    entry
                }
            };
        }
    }

    if is_creator(process_name)
        && (phase_name.contains("Forja") || phase_name.contains("forja") || delegates.iter().any(|d| {
            d.as_str()
                .map(|s| s.contains("crypto-broker") || s.contains("filesystem-manager"))
                .unwrap_or(false)
        }))
    {
        match materialize_by_inputs(repo, inputs) {
            Ok(forge) => {
                if let Some(obj) = state.as_object_mut() {
                    let handoff = obj.entry("handoff".to_string()).or_insert(json!({}));
                    if let Some(h) = handoff.as_object_mut() {
                        if let Some(f) = forge.as_object() {
                            for (k, v) in f {
                                h.insert(k.clone(), v.clone());
                            }
                        }
                    }
                }
                entry["status"] = json!("executed");
                entry["handler"] = json!("native-forge");
                entry["forge"] = json!(true);
                return entry;
            }
            Err(e) if process_name == "daemon-creator" => {
                entry["status"] = json!("simulated");
                entry["note"] = json!(format!("daemon-creator forja pendiente porte: {e}"));
                return entry;
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["error"] = json!(e);
                return entry;
            }
        }
    }

    if delegates_are_only_agents(&delegates) {
        if super::agent_runtime::is_configured() {
            return super::agent_runtime::invoke_agent_phase(
                repo,
                process_name,
                phase_name,
                &delegates,
                inputs,
                state,
            );
        }
        entry["status"] = json!("simulated");
        entry["note"] = json!("agentes IDE; sin SDDIA_AGENT_RUNTIME_COMMAND");
        return entry;
    }

    if let Some(capsule_entry) = try_invoke_delegates(repo, &delegates, inputs) {
        if let Some(obj) = capsule_entry.as_object() {
            for (k, v) in obj {
                entry[k.clone()] = v.clone();
            }
        }
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
    if route_fractal_fn(process_name).is_some() {
        return run_route_fractal_process(repo, process_name, process_inputs);
    }
    if process_name == "radamanto-batch" {
        return run_radamanto_batch(repo, process_name, phases, process_inputs);
    }
    if process_name == "telemetry-compliance-audit" {
        return run_telemetry_compliance_audit(repo, process_name, phases, process_inputs);
    }
    if process_name == "telemetry-batch-stub" {
        return run_telemetry_batch_stub_process(repo, process_name, phases, process_inputs);
    }
    if process_name == "execute-suite" {
        return run_execute_suite(repo, process_name, process_def, process_inputs);
    }
    if CHAOS_AUDIT.contains(&process_name) {
        return run_generic(repo, process_name, process_def, phases, process_inputs);
    }

    run_generic(repo, process_name, process_def, phases, process_inputs)
}

fn run_generic(
    repo: &Path,
    process_name: &str,
    process_def: &ProcessDef,
    phases: &[Value],
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    validate_process_inputs(process_def, process_inputs, process_name)?;

    let toll_start = if thermodynamic::is_exempt(process_name) {
        None
    } else {
        Some(Instant::now())
    };

    let mut state = json!({
        "handoff": {},
        "inputs": process_inputs,
    });
    if !thermodynamic::is_exempt(process_name) {
        state["asset_id"] = json!(Uuid::new_v4().to_string());
    }

    let template = workspace_template(process_def)?;
    let mut inputs_mut = process_inputs.clone();
    bootstrap_workspace(repo, process_name, &template, &mut inputs_mut, &mut state)?;

    let mut phase_reports = Vec::new();
    for phase in phases {
        phase_reports.push(execute_phase(
            repo,
            phase,
            process_name,
            process_def,
            &inputs_mut,
            &mut state,
        ));
    }

    let blocked = state.get("argos_verdict") == Some(&json!("block"));
    let status_code = if blocked { 1 } else { 0 };
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
        "merge_commit_hash",
        "closed_branch",
        "pbi_archived_path",
    ] {
        if let Some(v) = state.get(key) {
            data[key] = v.clone();
        }
    }

    if toll_start.is_some() {
        data["thermodynamic_toll"] = thermodynamic::run(
            repo,
            process_name,
            &state,
            &inputs_mut,
            status_code,
            duration_ms,
            !blocked,
        );
    }

    Ok(OrchestratorEnvelope {
        success: !blocked,
        status_code,
        data: Some(data),
        error: if blocked {
            Some("Argos: Ruido de Sistema".into())
        } else {
            None
        },
        execution_report: Some(json!({
            "process_name": process_name,
            "phases": phase_reports,
        })),
        exit_code: status_code,
    })
}
