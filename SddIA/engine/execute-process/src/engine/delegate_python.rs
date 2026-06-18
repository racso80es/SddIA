//! Delegación al bridge Python residual (`execute_process_capsules` vía subprocess).
//! Procesos no portados a nativo (creators complejos, radamanto, telemetry, accept-pr, …).

use super::invoke_orchestrator::invoke_capsules_bridge;
use crate::envelope::OrchestratorEnvelope;

use serde_json::Value;
use std::path::Path;

pub fn run_process(
    repo: &Path,
    process_name: &str,
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let body = invoke_capsules_bridge(repo, process_name, process_inputs)?;
    Ok(OrchestratorEnvelope::from_value(body))
}
