//! Proceso `tracker-backlog-query` — listado Linear vía `linear-tracker-adapter` (síncrono).

use super::super::capsules::invoke_tool_for_process;
use super::super::project_binding::{self, TrackerConfig};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_kind(raw: Option<&str>) -> Result<&'static str, String> {
    match raw.unwrap_or("all") {
        "hu" => Ok("hu"),
        "pbi" => Ok("pbi"),
        "all" => Ok("all"),
        other => Err(format!("kind no admitido: {other}")),
    }
}

fn canonical_state(
    linear_state: &str,
    state_map: &std::collections::BTreeMap<String, String>,
) -> String {
    for (canonical, name) in state_map {
        if name.eq_ignore_ascii_case(linear_state) {
            return canonical.clone();
        }
    }
    linear_state.to_string()
}

fn issue_kind(labels: &[Value], tc: &TrackerConfig) -> Option<&'static str> {
    let names: Vec<&str> = labels
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    let is_hu = names.iter().any(|n| *n == tc.label_hu.as_str());
    let is_pbi = names.iter().any(|n| *n == tc.label_pbi.as_str());
    if is_hu && !is_pbi {
        return Some("hu");
    }
    if is_pbi && !is_hu {
        return Some("pbi");
    }
    if is_hu && is_pbi {
        return Some("hu");
    }
    None
}

fn map_items(raw: &Value, tc: &TrackerConfig, kind_filter: &str, state_filter: Option<&str>) -> Vec<Value> {
    let Some(items) = raw.get("items").and_then(|v| v.as_array()) else {
        return vec![];
    };
    let mut out = Vec::new();
    for item in items {
        let labels = item
            .get("labels")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let Some(kind) = issue_kind(&labels, tc) else {
            continue;
        };
        if kind_filter != "all" && kind != kind_filter {
            continue;
        }
        let state_raw = item.get("state").and_then(|v| v.as_str()).unwrap_or("");
        let state = canonical_state(state_raw, &tc.state_map);
        if let Some(want) = state_filter {
            if state != want {
                continue;
            }
        }
        out.push(json!({
            "kind": kind,
            "id": item.get("identifier").or_else(|| item.get("id")),
            "title": item.get("title"),
            "state": state,
            "priority": item.get("priority"),
            "parent_id": item.get("parent"),
            "url": item.get("url"),
        }));
    }
    out
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let kind = parse_kind(process_inputs.get("kind").and_then(|v| v.as_str()))?;
    let state_filter = str_field(process_inputs, "state");

    let tracker = project_binding::tracker_config_for_inputs(repo, process_inputs)?;
    if tracker.is_none() {
        return Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({
                "tracker_configured": false,
                "items": [],
            })),
            error: None,
            execution_report: Some(json!({
                "phases": [{
                    "phase_name": "Consulta backlog",
                    "status": "skipped",
                    "handler": "tracker-backlog-query",
                    "reason": "sin tracker en manifiesto",
                }]
            })),
            exit_code: 0,
        });
    }
    let tc = tracker.unwrap();

    let mut request = json!({
        "operation": "list_issues",
        "team_key": tc.team_key,
    });
    if let Some(pid) = &tc.project_id {
        request["project_id"] = json!(pid);
    }
    if kind == "hu" {
        request["labels"] = json!([tc.label_hu.clone()]);
    } else if kind == "pbi" {
        request["labels"] = json!([tc.label_pbi.clone()]);
    }

    let tool_body = invoke_tool_for_process(
        repo,
        "linear-tracker-adapter",
        &json!({ "request": request }),
        Some("tracker-backlog-query"),
    )?;

    let items = map_items(&tool_body, &tc, kind, state_filter.as_deref());

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "tracker_configured": true,
            "items": items,
        })),
        error: None,
        execution_report: Some(json!({
            "phases": [{
                "phase_name": "Consulta backlog",
                "status": "executed",
                "handler": "tracker-backlog-query",
                "tool": "linear-tracker-adapter",
            }]
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_kind_respects_labels() {
        let tc = TrackerConfig {
            team_key: "BX".into(),
            project_id: None,
            state_map: Default::default(),
            label_hu: "hu".into(),
            label_pbi: "pbi".into(),
        };
        assert_eq!(
            issue_kind(&[json!("hu")], &tc),
            Some("hu")
        );
        assert!(issue_kind(&[json!("other")], &tc).is_none());
    }
}
