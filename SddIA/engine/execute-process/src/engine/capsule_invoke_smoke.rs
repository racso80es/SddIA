//! Proceso `capsule-invoke-smoke` — golden D-P5.2 (fase tool: ejecutada).

use super::capsules::invoke_tool;
use super::workspace::bootstrap_workspace;
use crate::core::resolver::{validate_process_inputs, ProcessDef};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;

fn workspace_template(process_def: &ProcessDef) -> Result<String, String> {
    process_def
        .get("workspace_template")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "workspace_template ausente".into())
}

pub fn run(
    repo: &Path,
    process_name: &str,
    process_def: &ProcessDef,
    phases: &[Value],
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    validate_process_inputs(process_def, process_inputs, process_name)?;
    let template = workspace_template(process_def)?;
    let mut state = json!({"handoff": {}, "inputs": process_inputs});
    let mut inputs_mut = process_inputs.clone();
    bootstrap_workspace(repo, process_name, &template, &mut inputs_mut, &mut state)?;

    let workspace_path = state
        .get("workspace_path")
        .and_then(|v| v.as_str())
        .ok_or("workspace_path ausente tras bootstrap")?;

    let tool_payload = json!({
        "workspace_path": workspace_path,
        "target_file": ".capsule-smoke-target",
    });

    let mut phase_reports: Vec<Value> = Vec::new();
    for phase in phases {
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
        if phase_name == "Invocación io-choke" {
            match invoke_tool(repo, "io-choke", &tool_payload) {
                Ok(body) => {
                    entry["status"] = json!("executed");
                    entry["handler"] = json!("capsule-tool-io-choke");
                    entry["capsule_result"] = body;
                }
                Err(e) => {
                    entry["status"] = json!("failed");
                    entry["handler"] = json!("capsule-tool-io-choke");
                    entry["error"] = json!(e);
                }
            }
        } else {
            entry["status"] = json!("simulated");
        }
        phase_reports.push(entry);
    }

    let success = phase_reports.iter().all(|p| p.get("status") == Some(&json!("executed")));
    let status_code = if success { 0 } else { 1 };
    Ok(OrchestratorEnvelope {
        success,
        status_code,
        data: Some(json!({
            "process_name": process_name,
            "workspace_path": workspace_path,
            "capsule_invoked": success,
        })),
        error: if success {
            None
        } else {
            Some("capsule-invoke-smoke falló".into())
        },
        execution_report: Some(json!({
            "process_name": process_name,
            "phases": phase_reports,
        })),
        exit_code: status_code,
    })
}
