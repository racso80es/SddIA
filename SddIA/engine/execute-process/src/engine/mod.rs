pub mod capsules;
pub mod delegate_handler;
pub mod delegate_python;
pub mod executor;
pub mod handlers;
pub mod thermodynamic;
pub mod workspace;
pub mod workspace_init;

use crate::core::resolver::load_process_def;
use crate::envelope::OrchestratorEnvelope;
use serde_json::Value;
use std::path::Path;

const HANDLER_BRIDGE: &[&str] = &[
    "route-domain-event",
    "telegram-fallback-responder",
    "telegram-gateway",
    "daemon-kill-switch",
    "governance-daemon-manager",
    "daemon-heartbeat-audit",
];

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

    if HANDLER_BRIDGE.contains(&canonical.as_str()) {
        return delegate_handler::run_handler(repo, &canonical, process_inputs);
    }

    if executor::handles(&canonical) {
        return executor::run_generic(repo, &canonical, &process_def, &phases, process_inputs);
    }

    delegate_python::run_process(repo, process_name, process_inputs)
}
