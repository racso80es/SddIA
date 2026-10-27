use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;

pub fn process_renamed(
    _repo: &Path,
    inputs: &Value,
    legacy_process: &str,
    ref_field: &str,
) -> OrchestratorEnvelope {
    let replacement = inputs
        .get(ref_field)
        .or_else(|| inputs.get("item_ref"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    OrchestratorEnvelope {
        success: false,
        status_code: 1,
        data: Some(json!({
            "replacement_process": "refine-item",
            "legacy_process": legacy_process,
            ref_field: replacement,
        })),
        error: Some(format!(
            "PROCESS_RENAMED: {legacy_process} retirado; usar refine-item con item_ref"
        )),
        execution_report: None,
        exit_code: 1,
    }
}
