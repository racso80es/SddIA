//! Motor genérico feature / bug-fix / refactorization (P1–P3).

use super::execution_workspace_report;
use super::thermodynamic;
use super::workspace::bootstrap_workspace;
use super::execution_registry::{
    active_execution_entry, design_artifacts_ready, load_phase_reports, phase_id_of,
    persist_phase_reports_atomic, resume_requested, stop_after_from_inputs,
    upsert_active_execution,
};
use super::workspace_init::{
    is_workspace_init_phase, maybe_emit_validation_started_after_tekton_agent,
    maybe_emit_work_initiated_on_tekton_entry, merge_work_initiated_workspace_success,
    run as run_workspace_init,
};
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

fn verification_allowed_despite_barrier(
    phase_name: &str,
    process_name: &str,
    repo: &Path,
    inputs: &Value,
) -> bool {
    if phase_name != "Verificación" || !process_has_doc_cascade_delivery(process_name) {
        return false;
    }
    super::lifecycle_phase_action::persist_forge_artifacts_ready(repo, inputs)
        || persist_validacion_exists(repo, inputs)
}

fn is_forge_agent_phase(phase_name: &str) -> bool {
    matches!(
        phase_name,
        "Estabilización de Requisitos"
            | "Estabilización de alcance"
            | "Diseño de Blueprint"
            | "Diseño del fix"
            | "Diseño de refactor"
            | "Ejecución"
    )
}

fn try_skip_forge_agent_phase(
    repo: &Path,
    process_name: &str,
    phase_name: &str,
    phase: &Value,
    inputs: &Value,
) -> Option<Value> {
    if !process_has_doc_cascade_delivery(process_name) || !is_forge_agent_phase(phase_name) {
        return None;
    }
    if !delegates_are_only_agents(&phase_delegates(phase)) {
        return None;
    }
    if !super::lifecycle_phase_action::persist_forge_artifacts_ready(repo, inputs) {
        return None;
    }
    Some(json!({
        "phase_name": phase_name,
        "delegates_to": phase_delegates(phase),
        "status": "skipped",
        "skipped": true,
        "handler": "forge-reentry-skip",
        "reason": "forge_already_present",
    }))
}

fn attach_lifecycle_json(entry: &mut Value, key: &str, value: Value) {
    entry[key] = value;
}

fn agent_phase_blocks_downstream(status: &str) -> bool {
    matches!(
        status,
        "failed" | "blocked" | "awaiting_agents" | "awaiting"
    )
}

fn process_has_doc_cascade_delivery(process_name: &str) -> bool {
    GENERIC_PROCESSES.contains(&process_name)
}

fn persist_validacion_exists(repo: &Path, inputs: &Value) -> bool {
    inputs
        .get("persist_ref")
        .and_then(|v| v.as_str())
        .map(|p| repo.join(p).join("validacion.md").is_file())
        .unwrap_or(false)
}

/// Relevo IDE (`simulated`) sin cascada documental completa: no encadenar DCC.
fn simulated_relay_blocks_close(
    process_name: &str,
    status: &str,
    repo: &Path,
    inputs: &Value,
) -> bool {
    status == "simulated"
        && process_has_doc_cascade_delivery(process_name)
        && !persist_validacion_exists(repo, inputs)
}

#[cfg(test)]
fn skipped_barrier_entry(phase: &Value, prior_status: &str) -> Value {
    skipped_barrier_entry_reason(phase, prior_status, "prior_agent_phase_not_executed")
}

fn skipped_barrier_entry_reason(phase: &Value, prior_status: &str, reason: &str) -> Value {
    let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
    json!({
        "phase_name": phase_name,
        "delegates_to": phase.get("delegates_to").cloned().unwrap_or(json!([])),
        "status": "skipped",
        "skipped": true,
        "handler": "phase-barrier",
        "reason": reason,
        "prior_status": prior_status,
    })
}

fn barrier_from_phase_entry(
    entry: &Value,
    agent_only: bool,
    phase: &Value,
    _phase_name: &str,
    process_name: &str,
    repo: &Path,
    inputs: &Value,
    stop_after: &[String],
) -> Option<(String, &'static str)> {
    let status = entry.get("status").and_then(|v| v.as_str()).unwrap_or("");
    if entry.get("handler").and_then(|v| v.as_str()) == Some("workspace-init") && status == "failed"
    {
        return Some(("failed".to_string(), "workspace_init_failed"));
    }
    if !agent_only {
        return None;
    }
    if status == "skipped"
        && entry.get("reason").and_then(|v| v.as_str()) == Some("forge_already_present")
    {
        return None;
    }
    if agent_phase_blocks_downstream(status) {
        return Some((status.to_string(), "prior_agent_phase_not_executed"));
    }
    if status == "executed" {
        if let Some(pid) = phase_id_of(phase) {
            if stop_after.iter().any(|s| s == pid) {
                return Some(("executed".to_string(), "stop_after"));
            }
        }
    }
    if simulated_relay_blocks_close(process_name, status, repo, inputs) {
        return Some((
            "awaiting_agents".to_string(),
            "prior_agent_phase_not_executed",
        ));
    }
    None
}

fn flush_reports(workspace_path: Option<&str>, reports: &[Value]) {
    if let Some(ws) = workspace_path {
        if let Err(e) = persist_phase_reports_atomic(ws, reports) {
            eprintln!("phase_reports.json: {e}");
        }
    }
}

fn flush_active(
    repo: &Path,
    process_name: &str,
    execution_id: Option<&str>,
    workspace_path: Option<&str>,
    inputs: &Value,
    status: &str,
    stop_after: &[String],
) {
    let Some(eid) = execution_id.map(str::trim).filter(|s| !s.is_empty()) else {
        return;
    };
    let Some(ws) = workspace_path.map(str::trim).filter(|s| !s.is_empty()) else {
        return;
    };
    let pref = inputs.get("persist_ref").and_then(|v| v.as_str());
    let entry = active_execution_entry(process_name, ws, pref, status, stop_after);
    if let Err(e) = upsert_active_execution(repo, eid, entry) {
        eprintln!("active-executions.json: {e}");
    }
}

fn prior_report_for_phase<'a>(prior: &'a [Value], phase: &Value) -> Option<&'a Value> {
    let pid = phase_id_of(phase);
    let pname = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
    prior.iter().find(|r| {
        if let Some(id) = pid {
            if r.get("phase_id").and_then(|v| v.as_str()) == Some(id) {
                return true;
            }
        }
        r.get("phase_name").and_then(|v| v.as_str()) == Some(pname)
    })
}

fn stop_after_frontier_consumed(prior: &[Value], stop_after: &[String]) -> bool {
    if stop_after.is_empty() {
        return false;
    }
    prior.iter().any(|r| {
        let status = r.get("status").and_then(|v| v.as_str()).unwrap_or("");
        let reason = r.get("reason").and_then(|v| v.as_str()).unwrap_or("");
        if status == "skipped" && reason == "stop_after" {
            return true;
        }
        if status == "executed" {
            if let Some(pid) = r.get("phase_id").and_then(|v| v.as_str()) {
                if stop_after.iter().any(|s| s == pid) {
                    return true;
                }
            }
        }
        false
    })
}

fn attach_phase_id(entry: &mut Value, phase: &Value) {
    if let Some(id) = phase_id_of(phase) {
        entry["phase_id"] = json!(id);
    }
}

fn skeleton_report(phase: &Value) -> Value {
    let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let mut entry = json!({
        "phase_name": phase_name,
        "delegates_to": phase.get("delegates_to").cloned().unwrap_or(json!([])),
        "status": "pending",
    });
    attach_phase_id(&mut entry, phase);
    entry
}

fn require_plan_for_process(process_name: &str) -> bool {
    matches!(process_name, "feature" | "refactorization")
}

fn resume_skip_or_block(
    repo: &Path,
    process_name: &str,
    phase: &Value,
    prior: &Value,
    inputs: &Value,
) -> Result<Option<Value>, Value> {
    let status = prior.get("status").and_then(|v| v.as_str()).unwrap_or("");
    if status == "executed" {
        let pid = phase_id_of(phase).unwrap_or("");
        if pid == "design"
            || prior
                .get("phase_id")
                .and_then(|v| v.as_str())
                == Some("design")
        {
            // artifact gate applies when later resuming into execution; checked at execution entry
        }
        let mut skipped = prior.clone();
        skipped["status"] = json!("executed");
        skipped["skipped"] = json!(true);
        skipped["reason"] = json!("resume_already_executed");
        skipped["handler"] = json!("execution-resume");
        attach_phase_id(&mut skipped, phase);
        return Ok(Some(skipped));
    }
    if matches!(status, "skipped") {
        let reason = prior.get("reason").and_then(|v| v.as_str()).unwrap_or("");
        if reason == "stop_after" {
            // Barrera consumida: relanzar fase pendiente en resume.
            return Ok(None);
        }
        let mut kept = prior.clone();
        attach_phase_id(&mut kept, phase);
        return Ok(Some(kept));
    }
    if matches!(status, "in_progress" | "failed" | "blocked") {
        let pid = phase_id_of(phase).unwrap_or("");
        if pid == "execution" || phase.get("name").and_then(|v| v.as_str()) == Some("Ejecución") {
            if !design_artifacts_ready(repo, inputs, require_plan_for_process(process_name)) {
                return Err(json!({
                    "status": "blocked",
                    "friction_id": "resume_design_artifacts_missing",
                    "message": "resume tras design executed sin spec.md (y plan.md si aplica)",
                    "operator_hint": "Materializar artefactos de diseño bajo persist_ref o no usar resume",
                }));
            }
        }
        return Ok(None); // relaunch
    }
    Ok(None)
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

    if is_workspace_init_phase(phase, inputs, process_name) {
        let mut entry = json!({
            "phase_name": phase_name,
            "delegates_to": phase.get("delegates_to").cloned().unwrap_or(json!([])),
        });
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
                merge_work_initiated_workspace_success(&result, state, &mut entry);
                return entry;
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["handler"] = json!("workspace-init");
                entry["error"] = json!(e);
                return entry;
            }
        }
    }

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
                merge_work_initiated_workspace_success(&result, state, &mut entry);
                return entry;
            }
            Err(e) => {
                entry["status"] = json!("failed");
                entry["handler"] = json!("workspace-init");
                entry["error"] = json!(e);
                return entry;
            }
        }
    }

    if (process_name == "feature" || process_name == "bug-fix" || process_name == "refactorization")
        && matches!(
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

    if delegates_are_only_agents(&delegates) {
        if let Some(skipped) =
            try_skip_forge_agent_phase(repo, process_name, phase_name, phase, inputs)
        {
            return skipped;
        }
        if let Some(skipped) =
            super::qa_profile::skip_agent_phase_entry(process_name, phase_name, inputs)
        {
            return skipped;
        }
        if phase_name == "Verificación" && process_has_doc_cascade_delivery(process_name) {
            if let Some(lc) = super::lifecycle_phase_action::maybe_apply_forge_closed(
                repo,
                inputs,
                "argos-preflight",
            ) {
                attach_lifecycle_json(&mut entry, "lifecycle_forge_closed", lc);
            }
        }
        if phase_name == "Cierre documental en rama" && process_has_doc_cascade_delivery(process_name)
        {
            if let Some(lc) =
                super::lifecycle_phase_action::maybe_apply_validation_from_validacion(repo, inputs, "argos")
            {
                attach_lifecycle_json(&mut entry, "lifecycle_validation", lc);
            }
        }
        maybe_emit_work_initiated_on_tekton_entry(
            repo,
            process_name,
            &delegates,
            state,
            &mut entry,
        );
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
                    .map(super::capability_di_resolver::di_binding_object),
            );
            if process_name == "pull-request-review"
                && agent_entry.get("status").and_then(|v| v.as_str()) == Some("blocked")
            {
                if let Some(obj) = state.as_object_mut() {
                    obj.insert("argos_verdict".into(), json!("block"));
                }
            }
            if let Some(b) = resolved_bindings.first() {
                agent_entry["di_binding"] = super::capability_di_resolver::di_binding_object(b);
                agent_entry["resolved_provider"] = json!(b.provider);
            }
            let agent_status = agent_entry
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            maybe_emit_validation_started_after_tekton_agent(
                repo,
                process_name,
                &delegates,
                &agent_status,
                inputs,
                state,
                &mut agent_entry,
            );
            if phase_name == "Verificación" && process_has_doc_cascade_delivery(process_name) {
                if let Some(lc) =
                    super::lifecycle_phase_action::maybe_apply_validation_from_validacion(
                        repo,
                        inputs,
                        "argos",
                    )
                {
                    agent_entry["lifecycle_validation"] = lc;
                }
            }
            return agent_entry;
        }
        entry["status"] = json!("simulated");
        entry["note"] = json!("agentes IDE; sin SDDIA_AGENT_RUNTIME_COMMAND");
        if phase_name == "Verificación" && process_has_doc_cascade_delivery(process_name) {
            if let Some(lc) =
                super::lifecycle_phase_action::maybe_apply_validation_from_validacion(repo, inputs, "argos")
            {
                entry["lifecycle_validation"] = lc;
            }
        }
        return entry;
    }

    if let Some(capsule_entry) = super::phase_capsules::try_invoke_delegates(
        repo,
        &delegates,
        inputs,
        resolved_bindings,
        process_name,
        phase_name,
        state,
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

    let stop_after = stop_after_from_inputs(&inputs_mut)?;
    let do_resume = resume_requested(&inputs_mut);
    let workspace_path_str = state
        .get("workspace_path")
        .or_else(|| state.get("workspace").and_then(|w| w.get("workspace_path")))
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let execution_id_str = state
        .get("execution_id")
        .or_else(|| inputs_mut.get("execution_id"))
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let prior_reports: Vec<Value> = if do_resume {
        match workspace_path_str.as_deref() {
            Some(ws) => load_phase_reports(ws)?.unwrap_or_default(),
            None => Vec::new(),
        }
    } else {
        Vec::new()
    };
    let frontier_consumed = do_resume && stop_after_frontier_consumed(&prior_reports, &stop_after);

    let mut phase_reports: Vec<Value> = phases.iter().map(skeleton_report).collect();
    flush_reports(workspace_path_str.as_deref(), &phase_reports);
    flush_active(
        repo,
        process_name,
        execution_id_str.as_deref(),
        workspace_path_str.as_deref(),
        &inputs_mut,
        "running",
        &stop_after,
    );

    let mut barrier_prior: Option<String> = None;
    let mut barrier_reason: &'static str = "prior_agent_phase_not_executed";
    let correlation_id = inputs_mut
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    for (idx, phase) in phases.iter().enumerate() {
        let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let agent_only = delegates_are_only_agents(&phase_delegates(phase));
        if let Some(prior) = barrier_prior.as_deref() {
            let skip = if agent_only {
                true
            } else if verification_allowed_despite_barrier(phase_name, process_name, repo, &inputs_mut)
            {
                false
            } else {
                is_verify_or_close_phase(phase_name)
            };
            if skip {
                let mut skipped = skipped_barrier_entry_reason(phase, prior, barrier_reason);
                attach_phase_id(&mut skipped, phase);
                phase_reports[idx] = skipped;
                flush_reports(workspace_path_str.as_deref(), &phase_reports);
                continue;
            }
        }

        if do_resume {
            if let Some(prev) = prior_report_for_phase(&prior_reports, phase) {
                match resume_skip_or_block(repo, process_name, phase, prev, &inputs_mut) {
                    Ok(Some(skipped)) => {
                        phase_reports[idx] = skipped;
                        flush_reports(workspace_path_str.as_deref(), &phase_reports);
                        continue;
                    }
                    Ok(None) => {}
                    Err(block) => {
                        phase_reports[idx] = block.clone();
                        flush_reports(workspace_path_str.as_deref(), &phase_reports);
                        flush_active(
                            repo,
                            process_name,
                            execution_id_str.as_deref(),
                            workspace_path_str.as_deref(),
                            &inputs_mut,
                            "failed",
                            &stop_after,
                        );
                        state["phase_reports"] = json!(phase_reports);
                        return Ok(OrchestratorEnvelope {
                            success: false,
                            status_code: 1,
                            data: Some(json!({
                                "process_name": process_name,
                                "friction_id": "resume_design_artifacts_missing",
                                "blocked": block,
                            })),
                            error: Some("resume_design_artifacts_missing".into()),
                            execution_report: Some(json!({
                                "process_name": process_name,
                                "phases": phase_reports,
                            })),
                            exit_code: 1,
                        });
                    }
                }
            }
        }

        // pre-fase write-ahead
        {
            let mut pending = phase_reports[idx].clone();
            pending["status"] = json!("in_progress");
            attach_phase_id(&mut pending, phase);
            phase_reports[idx] = pending;
            flush_reports(workspace_path_str.as_deref(), &phase_reports);
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
                None,
            );
        }
        let phase_t0 = Instant::now();
        let mut entry = execution_workspace_report::inject_elapsed_ms(
            execute_phase(
                repo,
                phase,
                process_name,
                process_def,
                &inputs_mut,
                &mut state,
            ),
            phase_t0.elapsed().as_millis() as i64,
        );
        attach_phase_id(&mut entry, phase);
        if let Some(cid) = correlation_id {
            let status = entry.get("status").and_then(|v| v.as_str());
            let verdict =
                super::progress_trace::short_verdict(&entry, state.get("handoff"));
            super::progress_trace::emit_progress_trace(
                repo,
                cid,
                phase_name,
                &delegates,
                "end",
                status,
                Some(process_name),
                verdict.as_deref(),
            );
        }
        let apply_stop = !(frontier_consumed
            && phase_id_of(phase).map(|id| stop_after.iter().any(|s| s == id)).unwrap_or(false));
        if apply_stop {
            if let Some((prior, reason)) = barrier_from_phase_entry(
                &entry,
                agent_only,
                phase,
                phase_name,
                process_name,
                repo,
                &inputs_mut,
                &stop_after,
            ) {
                barrier_prior = Some(prior);
                barrier_reason = reason;
            }
        }
        phase_reports[idx] = entry;
        flush_reports(workspace_path_str.as_deref(), &phase_reports);
        let idx_status = if barrier_reason == "stop_after" && barrier_prior.is_some() {
            "awaiting_agents"
        } else {
            "running"
        };
        flush_active(
            repo,
            process_name,
            execution_id_str.as_deref(),
            workspace_path_str.as_deref(),
            &inputs_mut,
            idx_status,
            &stop_after,
        );
    }
    state["phase_reports"] = json!(phase_reports);

    let final_status = if phase_reports.iter().any(|r| {
        r.get("reason").and_then(|v| v.as_str()) == Some("stop_after")
    }) {
        "awaiting_agents"
    } else {
        "completed"
    };
    flush_reports(workspace_path_str.as_deref(), &phase_reports);
    flush_active(
        repo,
        process_name,
        execution_id_str.as_deref(),
        workspace_path_str.as_deref(),
        &inputs_mut,
        final_status,
        &stop_after,
    );

    let mut data = json!({
        "process_name": process_name,
        "handoff": state.get("handoff").cloned().unwrap_or(json!({})),
    });
    for key in [
        "workspace_path",
        "execution_id",
        "pr_url",
        "event_id",
        "event_uri",
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

    let workspace_path = state
        .get("workspace_path")
        .or_else(|| state.get("workspace").and_then(|w| w.get("workspace_path")))
        .and_then(|v| v.as_str());
    let execution_id = state.get("execution_id").and_then(|v| v.as_str());
    execution_workspace_report::try_persist_execution_report(
        process_name,
        thermodynamic::is_exempt(process_name),
        workspace_path,
        execution_id,
        correlation_id,
        status_code,
        duration_ms,
        &phase_reports,
    );

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
    use std::fs;

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
    fn work_initiated_hooks_tekton_delegate_only() {
        use crate::engine::workspace_init::phase_delegates_to_tekton;
        assert!(phase_delegates_to_tekton(&[json!("agent:tekton")]));
        assert!(!phase_delegates_to_tekton(&[json!("agent:dedalo")]));
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

    #[test]
    fn simulated_relay_blocks_close_without_validacion() {
        assert!(simulated_relay_blocks_close(
            "bug-fix",
            "simulated",
            Path::new("/tmp"),
            &json!({"persist_ref": "docs/fixes/missing"})
        ));
        assert!(!simulated_relay_blocks_close(
            "evolution-audit",
            "simulated",
            Path::new("/tmp"),
            &json!({"persist_ref": "docs/fixes/missing"})
        ));
    }

    #[test]
    fn barrier_sequence_skips_close_after_simulated_relay_without_validacion() {
        struct Fake {
            name: &'static str,
            agent: bool,
            close: bool,
            status: &'static str,
        }
        let phases = [
            Fake {
                name: "Diseño",
                agent: true,
                close: false,
                status: "simulated",
            },
            Fake {
                name: "Ejecución",
                agent: true,
                close: false,
                status: "simulated",
            },
            Fake {
                name: "Verificación",
                agent: true,
                close: true,
                status: "simulated",
            },
            Fake {
                name: "Cierre documental en rama",
                agent: false,
                close: true,
                status: "would-archive",
            },
            Fake {
                name: "Cierre de entrega",
                agent: false,
                close: true,
                status: "would-close",
            },
        ];
        let process_name = "bug-fix";
        let inputs = json!({"persist_ref": "docs/fixes/missing-validacion"});
        let repo = Path::new("/tmp");
        let mut barrier: Option<String> = None;
        let mut out: Vec<(&str, &str)> = Vec::new();
        for p in &phases {
            if barrier.is_some() && (p.agent || p.close) {
                out.push((p.name, "skipped"));
                continue;
            }
            out.push((p.name, p.status));
            if p.agent {
                if agent_phase_blocks_downstream(p.status) {
                    barrier = Some(p.status.to_string());
                } else if simulated_relay_blocks_close(process_name, p.status, repo, &inputs) {
                    barrier = Some("awaiting_agents".to_string());
                }
            }
        }
        assert_eq!(
            out,
            vec![
                ("Diseño", "simulated"),
                ("Ejecución", "skipped"),
                ("Verificación", "skipped"),
                ("Cierre documental en rama", "skipped"),
                ("Cierre de entrega", "skipped"),
            ]
        );
        assert_eq!(barrier.as_deref(), Some("awaiting_agents"));
    }

    #[test]
    fn workspace_init_failed_skips_design_execution_and_verification() {
        struct Phase {
            name: &'static str,
            agent: bool,
            entry: Value,
        }
        let phases = [
            Phase {
                name: "Inicialización de Espacio de Trabajo",
                agent: false,
                entry: json!({"handler": "workspace-init", "status": "failed", "error": "dirty-worktree"}),
            },
            Phase {
                name: "Diseño del fix",
                agent: true,
                entry: json!({"status": "would-run"}),
            },
            Phase {
                name: "Ejecución",
                agent: true,
                entry: json!({"status": "would-run"}),
            },
            Phase {
                name: "Verificación",
                agent: true,
                entry: json!({"status": "would-run"}),
            },
        ];
        let mut barrier: Option<String> = None;
        let mut out: Vec<(&str, &str)> = Vec::new();
        for phase in &phases {
            if barrier.is_some() && (phase.agent || is_verify_or_close_phase(phase.name)) {
                out.push((phase.name, "skipped"));
                continue;
            }
            let status = phase
                .entry
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            out.push((phase.name, status));
            let phase_val = json!({"name": phase.name, "id": "x"});
            if let Some((prior, reason)) = barrier_from_phase_entry(
                &phase.entry,
                phase.agent,
                &phase_val,
                phase.name,
                "bug-fix",
                Path::new("/tmp"),
                &json!({}),
                &[],
            ) {
                assert_eq!(reason, "workspace_init_failed");
                barrier = Some(prior);
            }
        }
        assert_eq!(
            out,
            vec![
                ("Inicialización de Espacio de Trabajo", "failed"),
                ("Diseño del fix", "skipped"),
                ("Ejecución", "skipped"),
                ("Verificación", "skipped"),
            ]
        );
        assert_eq!(barrier.as_deref(), Some("failed"));
    }

    #[test]
    fn forge_already_present_skips_barrier() {
        let entry = json!({
            "status": "skipped",
            "reason": "forge_already_present",
            "handler": "forge-reentry-skip",
        });
        assert!(barrier_from_phase_entry(
            &entry,
            true,
            &json!({"name": "Ejecución", "id": "execution"}),
            "Ejecución",
            "feature",
            Path::new("/tmp"),
            &json!({}),
            &[],
        )
        .is_none());
    }

    #[test]
    fn stop_after_by_phase_id_triggers_barrier() {
        let phase = json!({"name": "Diseño de Blueprint", "id": "design", "delegates_to": ["agent:dedalo"]});
        let entry = json!({"status": "executed"});
        let stop = vec!["design".to_string()];
        let hit = barrier_from_phase_entry(
            &entry,
            true,
            &phase,
            "Diseño de Blueprint",
            "feature",
            Path::new("/tmp"),
            &json!({}),
            &stop,
        );
        assert_eq!(hit.unwrap().1, "stop_after");
    }

    #[test]
    fn barrier_sequence_stop_after_phase_id_skips_tekton() {
        struct Fake {
            name: &'static str,
            id: &'static str,
            agent: bool,
            voc: bool,
            status: &'static str,
        }
        let phases = [
            Fake { name: "Inicialización", id: "workspace_init", agent: false, voc: false, status: "executed" },
            Fake { name: "Diseño del fix", id: "design", agent: true, voc: false, status: "executed" },
            Fake { name: "Ejecución", id: "execution", agent: true, voc: false, status: "would-tekton" },
            Fake { name: "Verificación", id: "verification", agent: true, voc: true, status: "would-argos" },
        ];
        let stop_after = ["design"];
        let mut barrier: Option<&str> = None;
        let mut reason = "prior_agent_phase_not_executed";
        let mut out: Vec<(&str, &str, &str)> = Vec::new();
        for p in &phases {
            if barrier.is_some() && (p.agent || p.voc) {
                out.push((p.name, "skipped", reason));
                continue;
            }
            out.push((p.name, p.status, ""));
            if p.agent {
                if agent_phase_blocks_downstream(p.status) {
                    barrier = Some(p.status);
                    reason = "prior_agent_phase_not_executed";
                } else if stop_after.contains(&p.id) && p.status == "executed" {
                    barrier = Some("executed");
                    reason = "stop_after";
                }
            }
        }
        assert_eq!(
            out,
            vec![
                ("Inicialización", "executed", ""),
                ("Diseño del fix", "executed", ""),
                ("Ejecución", "skipped", "stop_after"),
                ("Verificación", "skipped", "stop_after"),
            ]
        );
    }

    #[test]
    fn persist_phase_reports_atomic_writes_file() {
        let dir = std::env::temp_dir().join(format!("sddia-pr-{}", Uuid::new_v4()));
        let reports = vec![json!({"phase_name": "Diseño del fix", "phase_id": "design", "status": "executed"})];
        persist_phase_reports_atomic(dir.to_str().unwrap(), &reports).expect("write");
        let raw = fs::read_to_string(dir.join("phase_reports.json")).expect("read");
        assert!(raw.contains("design"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn normalize_stop_after_legacy_string_via_registry() {
        use super::super::execution_registry::normalize_stop_after;
        assert_eq!(normalize_stop_after(&json!("design")).unwrap(), vec!["design".to_string()]);
        assert_eq!(
            normalize_stop_after(&json!(["design", "execution"])).unwrap(),
            vec!["design".to_string(), "execution".to_string()]
        );
    }
}
