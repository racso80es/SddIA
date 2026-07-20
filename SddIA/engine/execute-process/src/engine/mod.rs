pub mod actions;
pub mod capsule_invoke_smoke;
pub mod capsule_paths;
pub mod capsules;
pub mod daemons;
pub mod accept_pr;
pub mod entity_manager;
pub mod delivery_close;
pub mod cerbero_governance_react_core;
pub mod fix_tool_process_core;
pub mod fractal_bus;
pub mod residual_runner;
pub mod route_domain_core;
pub mod route_fractal_core;
pub mod radamanto_batch_core;
pub mod memory_evolution_ingest_core;
pub mod telemetry_batch_stub;
pub mod telemetry_compliance_core;
pub mod invoke_orchestrator;
pub mod crypto_broker;
pub mod domain_mutation;
pub mod eda_bus;
pub mod eda_bus_topology;
pub mod eda_coverage;
pub mod ecst_validation;
pub mod executor;
pub mod fractal;
pub mod enrich_fracture_pbi_kaizen;
pub mod materialize_fracture_pbi;
pub mod materialize_kaizen_alert_doc;
pub mod handlers;
pub mod phase_capsules;
pub mod policy_validator;
pub mod sync_entity_index;
pub mod suite_execution_requested;
pub mod thermodynamic;
pub mod workspace;
pub mod workspace_init;
pub mod verify_process_integrity;

use crate::core::resolver::load_process_def;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;

fn run_memory_evolution_ingest(
    repo: &Path,
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let rel = process_inputs
        .get("event_file_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_file_path requerido")?;
    let result = memory_evolution_ingest_core::ingest_domain_event_file(repo, rel);
    let ok = result.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
    let status_code = if ok { 0 } else { 1 };
    Ok(OrchestratorEnvelope {
        success: ok,
        status_code,
        data: Some(result.clone()),
        error: if ok {
            None
        } else {
            result
                .get("error")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        },
        execution_report: Some(json!({
            "process_name": "memory-evolution-ingest",
            "phases": [{
                "phase_name": "Ingesta evolution",
                "status": if ok { "executed" } else { "failed" },
                "handler": "memory-evolution-ingest-core",
                "result": result,
            }],
        })),
        exit_code: status_code,
    })
}

/// Punto de entrada del motor.
pub fn run_process(
    repo: &Path,
    process_name: &str,
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let (canonical, process_def, phases) = load_process_def(repo, process_name)?;

    if canonical == "kalma2-interact" {
        return handlers::kalma2::run(repo, process_inputs);
    }

    if canonical == "task-queue-manager" {
        return handlers::task_queue_manager::run(repo, process_inputs);
    }

    if canonical == "telegram-fallback-responder" {
        return handlers::telegram_fallback::run(repo, process_inputs);
    }

    if canonical == "telegram-gateway" {
        return handlers::telegram_gateway::run(repo, process_inputs);
    }

    if canonical == "governance-daemon-manager" {
        return handlers::governance_daemon::run(repo, process_inputs);
    }

    if canonical == "daemon-kill-switch" {
        return handlers::daemon_kill_switch::run(repo, process_inputs);
    }

    if canonical == "daemon-heartbeat-audit" {
        return handlers::daemon_heartbeat::run(repo, process_inputs);
    }

    if canonical == "memory-evolution-ingest" {
        return run_memory_evolution_ingest(repo, process_inputs);
    }

    if canonical == "route-domain-event" {
        return handlers::route_domain::run(repo, process_inputs);
    }

    if canonical == "delivery-close-cycle" {
        return delivery_close::run(repo, &canonical, &process_def, &phases, process_inputs);
    }

    if canonical == "capsule-invoke-smoke" {
        return capsule_invoke_smoke::run(repo, &canonical, &process_def, &phases, process_inputs);
    }

    if canonical == "entity-manager" {
        return entity_manager::run(repo, &process_def, &phases, process_inputs);
    }

    if executor::handles(&canonical) {
        return executor::run_generic(repo, &canonical, &process_def, &phases, process_inputs);
    }

    residual_runner::run(repo, &canonical, &process_def, &phases, process_inputs)
}
