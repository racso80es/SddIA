pub mod delegate_python;
pub mod handlers;

use crate::core::resolver::load_process_def;
use crate::envelope::OrchestratorEnvelope;
use serde_json::Value;
use std::path::Path;

/// Punto de entrada del motor — handlers nativos Rust o delegación Python (Fase C incremental).
pub fn run_process(
    repo: &Path,
    process_name: &str,
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let (canonical, _process_def, _phases) = load_process_def(repo, process_name)?;
    match canonical.as_str() {
        "kalma2-interact" => handlers::kalma2::run(process_inputs),
        _ => delegate_python::run_process(repo, process_name, process_inputs),
    }
}
