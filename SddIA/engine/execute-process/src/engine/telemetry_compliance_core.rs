//! Auditoría cumplimiento termodinámico (paridad `telemetry_compliance_audit_core.py`).

use super::eda_bus::write_fractal_event;
use super::fractal_bus::{
    build_telemetry_compliance_breached_event, lab_route_sync,
    load_telemetry_compliance_config, receipt_satisfies_schema, resolve_ed_telemetry_contract,
    stamp_fractal_delivery_state, COMPLIANCE_SUBSCRIBER_KEY,
};
use super::invoke_orchestrator::invoke_process_full;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn write_json_atomic(path: &Path, data: &Value) -> Result<(), String> {
    super::eda_bus_topology::write_json_atomic(path, data)
}

fn emitted_path(repo: &Path) -> Result<std::path::PathBuf, String> {
    let cfg = load_telemetry_compliance_config(repo)?;
    Ok(repo.join(
        cfg.get("emitted_registry")
            .map(String::as_str)
            .unwrap_or(".SddIA/telemetry-compliance/emitted.json"),
    ))
}

fn load_emitted_breaches(repo: &Path) -> HashSet<String> {
    let path = emitted_path(repo).ok();
    let Some(path) = path.filter(|p| p.is_file()) else {
        return HashSet::new();
    };
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|data| {
            data.get("breach_asset_ids")
                .and_then(|v| v.as_array())
                .map(|ids| ids.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        })
        .unwrap_or_default()
}

fn mark_breach_emitted(repo: &Path, asset_id: &str) -> Result<(), String> {
    let mut emitted = load_emitted_breaches(repo);
    emitted.insert(asset_id.to_string());
    let mut ids: Vec<_> = emitted.into_iter().collect();
    ids.sort();
    let path = emitted_path(repo)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_json_atomic(&path, &json!({"breach_asset_ids": ids}))
}

fn emit_breach_if_needed(
    repo: &Path,
    asset_id: &str,
    capsule_id: &str,
    process_name: &str,
    breach_reason: &str,
    expected_schema: Option<Vec<String>>,
) -> Result<Option<Value>, String> {
    if load_emitted_breaches(repo).contains(asset_id) {
        return Ok(None);
    }
    let event = build_telemetry_compliance_breached_event(
        asset_id,
        capsule_id,
        process_name,
        breach_reason,
        expected_schema.as_deref(),
    );
    let seal = write_fractal_event(repo, &event, "domain")?;
    mark_breach_emitted(repo, asset_id)?;
    let route_out = if lab_route_sync() {
        if let Some(target) = seal.get("target_path").and_then(|v| v.as_str()) {
            invoke_process_full(
                repo,
                "route-domain",
                &json!({"event_file_path": target}),
            )
            .ok()
        } else {
            None
        }
    } else {
        None
    };
    Ok(Some(json!({"seal": seal, "route": route_out})))
}

pub fn audit_telemetry_compliance(repo: &Path, rel_path: &str) -> Value {
    match audit_telemetry_compliance_inner(repo, rel_path) {
        Ok(v) => v,
        Err(e) => json!({"ok": false, "error": e}),
    }
}

fn audit_telemetry_compliance_inner(repo: &Path, rel_path: &str) -> Result<Value, String> {
    let event_path = repo.join(rel_path.trim());
    if !event_path.is_file() {
        return Err(format!("no existe: {rel_path}"));
    }
    let body: Value =
        serde_json::from_str(&fs::read_to_string(&event_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

    if body.get("event_type").and_then(|v| v.as_str()) != Some("Raw_Execution_Finished") {
        stamp_fractal_delivery_state(&event_path, COMPLIANCE_SUBSCRIBER_KEY, "skipped");
        return Ok(json!({"ok": true, "status": "skipped", "reason": "wrong_event_type"}));
    }

    let payload = body.get("payload").cloned().unwrap_or(json!({}));
    if !payload.is_object() {
        return Err("payload invalido".into());
    }
    let asset_id = payload
        .get("asset_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("asset_id requerido")?;

    let capsule_str = payload
        .get("capsule_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("");
    let process_name = payload
        .get("process_name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let contract = resolve_ed_telemetry_contract(
        repo,
        if capsule_str.is_empty() {
            None
        } else {
            Some(capsule_str)
        },
    );

    if !contract
        .get("telemetry_provided")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        stamp_fractal_delivery_state(&event_path, COMPLIANCE_SUBSCRIBER_KEY, "success");
        return Ok(json!({"ok": true, "status": "skipped", "reason": "not_required"}));
    }

    let schema_list: Option<Vec<String>> = contract
        .get("telemetry_schema")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        });

    let receipt = payload.get("telemetry_receipt");
    let breach = if receipt.is_none()
        || !receipt.map(|r| r.is_object()).unwrap_or(false)
        || receipt == Some(&json!({}))
    {
        emit_breach_if_needed(
            repo,
            asset_id,
            if capsule_str.is_empty() {
                process_name
            } else {
                capsule_str
            },
            process_name,
            "missing_receipt",
            schema_list.clone(),
        )?
    } else if let Some(ref schema) = schema_list {
        if !receipt_satisfies_schema(receipt.unwrap(), schema) {
            emit_breach_if_needed(
                repo,
                asset_id,
                if capsule_str.is_empty() {
                    process_name
                } else {
                    capsule_str
                },
                process_name,
                "schema_mismatch",
                schema_list.clone(),
            )?
        } else {
            None
        }
    } else {
        None
    };

    stamp_fractal_delivery_state(&event_path, COMPLIANCE_SUBSCRIBER_KEY, "success");
    Ok(json!({
        "ok": true,
        "status": if breach.is_some() { "breach" } else { "pass" },
        "breach": breach,
        "asset_id": asset_id,
    }))
}
