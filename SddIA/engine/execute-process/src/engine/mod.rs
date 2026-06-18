pub mod actions;
pub mod capsule_invoke_smoke;
pub mod capsule_paths;
pub mod capsules;
pub mod daemons;
pub mod delegate_python;
pub mod delivery_close;
pub mod crypto_broker;
pub mod domain_mutation;
pub mod eda_bus;
pub mod eda_coverage;
pub mod ecst_validation;
pub mod executor;
pub mod fractal;
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

use crate::core::resolver::load_process_def;
use crate::envelope::OrchestratorEnvelope;
use serde_json::Value;
use std::path::Path;

/// Punto de entrada del motor.
pub fn run_process(
    repo: &Path,
    process_name: &str,
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let (canonical, process_def, phases) = load_process_def(repo, process_name)?;

    if canonical == "kalma2-interact" {
        return handlers::kalma2::run(process_inputs);
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

    if canonical == "route-domain-event" {
        return handlers::route_domain::run(repo, process_inputs);
    }

    if canonical == "delivery-close-cycle" {
        return delivery_close::run(repo, &canonical, &process_def, &phases, process_inputs);
    }

    if canonical == "capsule-invoke-smoke" {
        return capsule_invoke_smoke::run(repo, &canonical, &process_def, &phases, process_inputs);
    }

    if executor::handles(&canonical) {
        return executor::run_generic(repo, &canonical, &process_def, &phases, process_inputs);
    }

    delegate_python::run_process(repo, process_name, process_inputs)
}
