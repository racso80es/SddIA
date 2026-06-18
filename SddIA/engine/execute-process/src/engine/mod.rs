pub mod capsules;
pub mod daemons;
pub mod delegate_python;
pub mod executor;
pub mod fractal;
pub mod handlers;
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

    if executor::handles(&canonical) {
        return executor::run_generic(repo, &canonical, &process_def, &phases, process_inputs);
    }

    delegate_python::run_process(repo, process_name, process_inputs)
}
