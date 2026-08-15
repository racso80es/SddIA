//! Motor genérico feature / bug-fix / refactorization (P1–P3).

use super::thermodynamic;
use super::workspace::bootstrap_workspace;
use super::workspace_init::{is_workspace_init_phase, run as run_workspace_init};
use crate::core::resolver::{validate_process_inputs, ProcessDef};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;
use std::time::Instant;
use uuid::Uuid;

const GENERIC_PROCESSES: &[&str] = &["feature", "bug-fix", "refactorization"];

pub fn handles(process_name: &str) -> bool {
    GENERIC_PROCESSES.contains(&process_name)
}

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn delegates_are_only_agents(delegates: &[Value]) -> bool {
    !delegates.is_empty()
        && delegates
            .iter()
            .all(|d| d.as_str().map(|s| s.starts_with("agent:")).unwrap_or(false))
}

fn phase_delegates(phase: &Value) -> Vec<Value> {
    phase
        .get("delegates_to")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

fn is_verify_or_close_phase(phase_name: &str) -> bool {
    matches!(
        phase_name,
        "Verificación" | "Cierre documental en rama" | "Cierre de entrega"
    )
}

fn agent_phase_blocks_downstream(status: &str) -> bool {
    matches!(
        status,
        "failed" | "blocked" | "awaiting_agents" | "awaiting"
    )
}

fn skipped_barrier_entry(phase: &Value, prior_status: &str) -> Value {
    let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
    json!({
        "phase_name": phase_name,
        "delegates_to": phase.get("delegates_to").cloned().unwrap_or(json!([])),
        "status": "skipped",
        "skipped": true,
        "handler": "phase-barrier",
        "reason": "prior_agent_phase_not_executed",
        "prior_status": prior_status,
    })
}

fn workspace_template(process_def: &ProcessDef) -> Result<String, String> {
    process_def
        .get("workspace_template")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "workspace_template ausente en definición del proceso".into())
}

fn execute_phase(
    repo: &Path,
    phase: &Value,
    process_name: &str,
    process_def: &ProcessDef,
    inputs: &Value,
    state: &mut Value,
) -> Value {
    let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");

    // R6: piloto EDA — emitir evento y omitir cadena sync DI (sin await del reactor).
    if super::capability_di_reactor::is_eda_pilot_phase(phase) {
        let mut entry = json!({
            "phase_name": phase_name,
            "delegates_to": phase.get("delegates_to").cloned().unwrap_or(json!([])),
            "di_composition": "eda_pilot",
        });
        match super::capability_di_reactor::emit_di_requested(repo, phase, process_name, inputs) {
            Ok(event_id) => {
                entry["capability_di_event_id"] = json!(event_id);
                entry["handler"] = json!("capability-di-reactor-emit");
                super::capability_di_reactor::spawn_reactor_background(
                    repo.to_path_buf(),
                    process_def.clone(),
                );
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["handler"] = json!("capability-di-reactor-emit");
                entry["error"] = json!(e);
                return entry;
            }
        }
        // Continuar ejecución de fase sin di_binding (path piloto).
        return execute_phase_body(
            repo,
            phase,
            process_name,
            process_def,
            inputs,
            state,
            entry,
            &[],
        );
    }

    // PBI-042 Hito 2/3: resolve ciego → fase efectiva → Aduana Temprana → Cerbero RBAC.
    let resolved_bindings = match super::capability_di_resolver::resolve_phase_bindings(repo, phase)
    {
        Ok(b) => b,
        Err(di_err) => {
            super::capability_di_resolver::write_resolve_dead_letter(
                repo,
                &di_err,
                phase_name,
                process_name,
            );
            return json!({
                "phase_name": phase_name,
                "delegates_to": phase.get("delegates_to").cloned().unwrap_or(json!([])),
                "status": "failed",
                "handler": "capability-di-resolver",
                "error": di_err.message,
                "di_resolve_code": di_err.code.as_str(),
            });
        }
    };
    let effective_phase =
        super::capability_di_resolver::phase_with_effective_delegates(phase, &resolved_bindings);
    let delegates = effective_phase
        .get("delegates_to")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut entry = json!({
        "phase_name": phase_name,
        "delegates_to": delegates.clone(),
    });
    if let Some(b) = resolved_bindings.first() {
        entry["di_binding"] = super::capability_di_resolver::di_binding_object(b);
        entry["resolved_provider"] = json!(b.provider);
    }

    if let Err(di_err) = super::capability_di_gate::validate_phase_capability_di(
        repo,
        &effective_phase,
        process_name,
    ) {
        super::capability_di_gate::write_di_dead_letter(repo, &di_err, phase_name, process_name);
        entry["status"] = json!("failed");
        entry["handler"] = json!("capability-di-gate");
        entry["error"] = json!(di_err.message);
        entry["di_gate_code"] = json!(di_err.code.as_str());
        return entry;
    }

    let requester_policies =
        super::cerbero_di_rbac::resolve_requester_policies(process_def, inputs);
    if let Err(cerbero_err) = super::cerbero_di_rbac::validate_di_rbac(
        repo,
        process_name,
        phase_name,
        &requester_policies,
        &resolved_bindings,
    ) {
        super::cerbero_di_rbac::write_cerbero_di_dead_letter(
            repo,
            &cerbero_err,
            phase_name,
            process_name,
        );
        entry["status"] = json!("failed");
        entry["handler"] = json!("cerbero-di-rbac");
        entry["error"] = json!(cerbero_err.message);
        entry["cerbero_di_code"] = json!(cerbero_err.code.as_str());
        return entry;
    }

    let packaged_for_check: Vec<Value> = if let Some(di) = entry.get("di_binding") {
        if di.is_array() {
            di.as_array().cloned().unwrap_or_default()
        } else {
            vec![di.clone()]
        }
    } else {
        resolved_bindings
            .iter()
            .map(super::capability_di_resolver::di_binding_object)
            .collect()
    };
    if let Err(envelope_err) = super::cerbero_di_envelope::validate_packaged_bindings(
        repo,
        &resolved_bindings,
        &packaged_for_check,
    ) {
        super::cerbero_di_envelope::write_cerbero_envelope_dead_letter(
            repo,
            &envelope_err,
            phase_name,
            process_name,
        );
        entry["status"] = json!("failed");
        entry["handler"] = json!("cerbero-di-envelope");
        entry["error"] = json!(envelope_err.message);
        entry["cerbero_envelope_di_code"] = json!(envelope_err.code.as_str());
        return entry;
    }

    execute_phase_body(
        repo,
        &effective_phase,
        process_name,
        process_def,
        inputs,
        state,
        entry,
        &resolved_bindings,
    )
}

fn execute_phase_body(
    repo: &Path,
    phase: &Value,
    process_name: &str,
    _process_def: &ProcessDef,
    inputs: &Value,
    state: &mut Value,
    mut entry: Value,
    resolved_bindings: &[super::capability_di_resolver::ResolvedBinding],
) -> Value {
    let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let delegates = phase
        .get("delegates_to")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    if is_workspace_init_phase(phase, inputs, process_name) {
        match run_workspace_init(repo, inputs, process_name) {
            Ok(result) => {
                entry["status"] = json!("executed");
                entry["handler"] = json!("workspace-init");
                if let Some(steps) = result.get("git_steps") {
                    entry["git_steps"] = steps.clone();
                }
                if let Some(op) = result.get("objectives_path") {
                    entry["objectives_path"] = op.clone();
                }
                if let Some(bn) = result.get("branch_name") {
                    entry["branch_name"] = bn.clone();
                }
                if let Some(obj) = state.as_object_mut() {
                    obj.insert("workspace".into(), result.clone());
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

    if process_name == "feature" || process_name == "bug-fix" || process_name == "refactorization" {
        if matches!(
            phase_name,
            "Cierre documental en rama" | "Cierre de entrega"
        ) {
            if phase_name == "Cierre documental en rama" && env_truthy("SDDIA_LAB_SKIP_PBI_ARCHIVE")
            {
                entry["status"] = json!("skipped");
                entry["handler"] = json!("feature-pbi-archive");
                entry["skipped"] = json!(true);
                entry["reason"] = json!("SDDIA_LAB_SKIP_PBI_ARCHIVE");
                return entry;
            }
            if phase_name == "Cierre de entrega" && env_truthy("SDDIA_LAB_SKIP_DELIVERY_CLOSE") {
                entry["status"] = json!("skipped");
                entry["handler"] = json!("feature-delivery-close");
                entry["skipped"] = json!(true);
                entry["reason"] = json!("SDDIA_LAB_SKIP_DELIVERY_CLOSE");
                return entry;
            }
            if let Some(phase_result) =
                super::phase_capsules::execute_feature_phase(repo, phase_name, inputs, state)
            {
                match phase_result {
                    Ok(phase_entry) => {
                        if let Some(obj) = phase_entry.as_object() {
                            for (k, v) in obj {
                                entry[k.clone()] = v.clone();
                            }
                        }
                        if entry.get("status").is_none() {
                            entry["status"] = json!("executed");
                        }
                    }
                    Err(e) => {
                        entry["status"] = json!("failed");
                        entry["error"] = json!(e);
                    }
                }
                return entry;
            }
        }
    }

    if delegates_are_only_agents(&delegates) {
        if super::agent_runtime::is_configured() {
            let mut agent_entry = super::agent_runtime::invoke_agent_phase(
                repo,
                process_name,
                phase_name,
                &delegates,
                inputs,
                state,
                resolved_bindings
                    .first()
                    .map(|b| super::capability_di_resolver::di_binding_object(b)),
            );
            if let Some(b) = resolved_bindings.first() {
                agent_entry["di_binding"] = super::capability_di_resolver::di_binding_object(b);
                agent_entry["resolved_provider"] = json!(b.provider);
            }
            return agent_entry;
        }
        entry["status"] = json!("simulated");
        entry["note"] = json!("agentes IDE; sin SDDIA_AGENT_RUNTIME_COMMAND");
        return entry;
    }

    if let Some(capsule_entry) = super::phase_capsules::try_invoke_delegates(
        repo,
        &delegates,
        inputs,
        resolved_bindings,
        process_name,
        phase_name,
    ) {
        if let Some(obj) = capsule_entry.as_object() {
            for (k, v) in obj {
                entry[k.clone()] = v.clone();
            }
        }
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

    if delegates.iter().any(|d| {
        d.as_str()
            .map(|s| s.starts_with("action:"))
            .unwrap_or(false)
    }) {
        entry["status"] = json!("simulated");
        entry["note"] = json!("acción sin handler nativo ni bridge resuelto");
        return entry;
    }

    entry["status"] = json!("simulated");
    entry
}

pub fn run_generic(
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

    let mut phase_reports: Vec<Value> = Vec::new();
    let mut barrier_prior: Option<String> = None;
    let correlation_id = inputs_mut
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    for phase in phases {
        let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let agent_only = delegates_are_only_agents(&phase_delegates(phase));
        if let Some(prior) = barrier_prior.as_deref() {
            if agent_only || is_verify_or_close_phase(phase_name) {
                phase_reports.push(skipped_barrier_entry(phase, prior));
                continue;
            }
        }
        let delegates = phase_delegates(phase);
        if let Some(cid) = correlation_id {
            super::progress_trace::emit_progress_trace(
                repo,
                cid,
                phase_name,
                &delegates,
                "start",
                None,
                Some(process_name),
            );
        }
        let entry = execute_phase(
            repo,
            phase,
            process_name,
            process_def,
            &inputs_mut,
            &mut state,
        );
        if let Some(cid) = correlation_id {
            let status = entry.get("status").and_then(|v| v.as_str());
            super::progress_trace::emit_progress_trace(
                repo,
                cid,
                phase_name,
                &delegates,
                "end",
                status,
                Some(process_name),
            );
        }
        if agent_only {
            let status = entry.get("status").and_then(|v| v.as_str()).unwrap_or("");
            if agent_phase_blocks_downstream(status) {
                barrier_prior = Some(status.to_string());
            }
        }
        phase_reports.push(entry);
    }
    state["phase_reports"] = json!(phase_reports);

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
        "pbi_archived_path",
        "delivery_close",
    ] {
        if let Some(v) = state.get(key) {
            data[key] = v.clone();
        }
    }
    if let Some(ws) = state.get("workspace") {
        if let Some(obj) = ws.as_object() {
            for (k, v) in obj {
                if data.get(k).is_none() {
                    data[k] = v.clone();
                }
            }
        }
    }

    let verdict = super::phase_terminal::aggregate_execution_terminal(&phase_reports, &state);
    let status_code = verdict.status_code;
    let duration_ms = toll_start
        .map(|t| t.elapsed().as_millis() as i64)
        .unwrap_or(0);

    super::phase_terminal::apply_failed_phase_fields(&mut data, &verdict);

    if toll_start.is_some() {
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
    }

    Ok(OrchestratorEnvelope {
        success: verdict.success,
        status_code,
        data: Some(data),
        error: verdict.error.clone(),
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

    fn agent_phase(name: &str, agent: &str) -> Value {
        json!({
            "name": name,
            "delegates_to": [format!("agent:{agent}")]
        })
    }

    #[test]
    fn barrier_helpers_classify_statuses() {
        assert!(agent_phase_blocks_downstream("failed"));
        assert!(agent_phase_blocks_downstream("awaiting_agents"));
        assert!(agent_phase_blocks_downstream("blocked"));
        assert!(!agent_phase_blocks_downstream("executed"));
        assert!(!agent_phase_blocks_downstream("simulated"));
        assert!(!agent_phase_blocks_downstream("skipped"));
        assert!(is_verify_or_close_phase("Verificación"));
        assert!(is_verify_or_close_phase("Cierre de entrega"));
        assert!(!is_verify_or_close_phase("Ejecución"));
    }

    #[test]
    fn skipped_barrier_entry_marks_verification() {
        let phase = agent_phase("Verificación", "argos");
        let entry = skipped_barrier_entry(&phase, "failed");
        assert_eq!(entry["status"], "skipped");
        assert_eq!(entry["handler"], "phase-barrier");
        assert_eq!(entry["reason"], "prior_agent_phase_not_executed");
        assert_eq!(entry["prior_status"], "failed");
        assert_eq!(entry["phase_name"], "Verificación");
    }

    #[test]
    fn empty_delegates_are_not_agent_only() {
        assert!(!delegates_are_only_agents(&[]));
        assert!(delegates_are_only_agents(&[json!("agent:tekton")]));
        assert!(!delegates_are_only_agents(&[
            json!("agent:tekton"),
            json!("skill:git-manager")
        ]));
    }

    #[test]
    fn barrier_sequence_skips_verification_after_failed_execution() {
        struct Fake {
            name: &'static str,
            agent: bool,
            voc: bool,
            status: &'static str,
        }
        let phases = [
            Fake {
                name: "Inicialización",
                agent: false,
                voc: false,
                status: "executed",
            },
            Fake {
                name: "Ejecución",
                agent: true,
                voc: false,
                status: "failed",
            },
            Fake {
                name: "Verificación",
                agent: true,
                voc: true,
                status: "would-invoke-argos",
            },
            Fake {
                name: "Cierre de entrega",
                agent: false,
                voc: true,
                status: "would-close",
            },
        ];
        let mut barrier: Option<&str> = None;
        let mut out: Vec<(&str, &str)> = Vec::new();
        for p in &phases {
            if barrier.is_some() && (p.agent || p.voc) {
                out.push((p.name, "skipped"));
                continue;
            }
            out.push((p.name, p.status));
            if p.agent && agent_phase_blocks_downstream(p.status) {
                barrier = Some(p.status);
            }
        }
        assert_eq!(
            out,
            vec![
                ("Inicialización", "executed"),
                ("Ejecución", "failed"),
                ("Verificación", "skipped"),
                ("Cierre de entrega", "skipped"),
            ]
        );
        assert!(barrier.is_some());
    }
}
