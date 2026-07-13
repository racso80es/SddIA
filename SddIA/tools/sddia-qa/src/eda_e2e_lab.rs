use execute_process::core::env::load_hierarchical_env;
use execute_process::engine::eda_bus_topology::{ensure_event_bus_topology, list_witnesses};
use execute_process::engine::invoke_orchestrator::resolve_orchestrator_bin;
use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

use crate::lab_teardown::{cleanup_lab_entity_forge, cleanup_orphan_core_eda_e2e_tools, keep_tmp};
use crate::resolve::{flag_value, has_flag, print_json_report, resolve_event_watcher, run_json_cmd};

fn seed_key(entity_class: &str) -> &'static str {
    match entity_class {
        "tool" => "tool_name",
        "action" => "action_name",
        "process" => "process_name",
        "agent" => "agent_name",
        "norm" => "tactical_norm_name",
        "codex" => "domain_codex_slug",
        "skill" => "skill_name",
        "event" => "event_name",
        _ => "entity_name",
    }
}

fn create_entity(repo: &Path, entity_class: &str, entity_name: &str) -> Result<Value, String> {
    let key = seed_key(entity_class);
    let mut semantic_seed = json!({
        key: entity_name,
        "scope": "local",
        "execution_logic": format!("E2E lab {entity_class}"),
        "orchestration_logic": format!("E2E lab {entity_class}"),
        "process_description": format!("E2E {entity_name}"),
        "agent_purpose": format!("E2E {entity_name}"),
        "tactical_norm_friction": format!("E2E {entity_name}"),
        "domain_codex_name": entity_name,
        "event_type": "E2E_Smoke_Event",
        "event_description": "Smoke E2E",
        "payload_required": ["entity_uuid"],
        "skill_inputs_schema": [],
        "skill_outputs_schema": [],
    });
    if entity_class == "event" {
        semantic_seed["event_type"] =
            json!(format!("E2E_{}", entity_name.replace('-', "_")));
        semantic_seed["event_family"] = json!("domain");
    }
    let payload = json!({
        "entity_class": entity_class,
        "entity_name": entity_name,
        "lifecycle_operation": "create",
        "semantic_seed": semantic_seed,
    });
    let bin = resolve_orchestrator_bin(repo)?;
    let inputs = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let mut cmd = Command::new(&bin);
    cmd.args(["--process", "entity-manager", "--inputs", &inputs])
        .current_dir(repo);
    let body = run_json_cmd(&mut cmd)?;
    Ok(body.get("data").cloned().unwrap_or(json!({})))
}

fn route_event(repo: &Path, rel_path: &str) -> Result<Value, String> {
    std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "1");
    std::env::set_var("SDDIA_LAB_SIMULATE_SYNC_INDEX", "1");
    std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
    let watcher = resolve_event_watcher(repo)?;
    let mut cmd = Command::new(&watcher);
    cmd.args(["--event-file-path", rel_path]).current_dir(repo);
    run_json_cmd(&mut cmd)
}

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let entity_class = flag_value(args, "--entity-class").unwrap_or("tool");
    let entity_name_arg = flag_value(args, "--entity-name");
    let event_file_path = flag_value(args, "--event-file-path");
    let json_out = has_flag(args, "--json");

    let mut report = json!({"steps": []});
    let mut exit_code = 1;
    let mut created_entity = false;
    let mut entity_name: Option<String> = entity_name_arg.map(str::to_string);
    let mut event_id: Option<String> = None;
    let mut rel: Option<String> = None;

    let run_result = (|| -> Result<(), String> {
        load_hierarchical_env(repo)?;
        std::env::remove_var("EVENT_BUS_PATH");
        let bus = ensure_event_bus_topology(repo)?;

        if let Some(path) = event_file_path {
            rel = Some(path.replace('\\', "/"));
            event_id = Some(Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string());
        } else if has_flag(args, "--skip-create") {
            return Err("Indique --event-file-path o omita --skip-create".into());
        } else {
            let name = entity_name
                .clone()
                .unwrap_or_else(|| format!("eda-e2e-{entity_class}-{}", &Uuid::new_v4().simple().to_string()[..8]));
            entity_name = Some(name.clone());
            let create_data = create_entity(repo, entity_class, &name)?;
            let handoff = create_data.get("handoff").cloned().unwrap_or(json!({}));
            event_id = handoff
                .get("event_id")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            rel = handoff
                .get("target_path")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            if rel.as_deref().unwrap_or("").is_empty() || event_id.is_none() {
                report["success"] = false.into();
                report["error"] = "entity-manager sin event_id".into();
                report["data"] = create_data.into();
                return Ok(());
            }
            created_entity = true;
            report["steps"]
                .as_array_mut()
                .unwrap()
                .push(json!({"create": name, "event_id": event_id}));
            report["entity_class"] = entity_class.into();
            report["entity_name"] = name.into();
        }

        if let Some(ref rel_path) = rel {
            if report.get("error").is_some() {
                return Ok(());
            }
            let pending = repo.join(rel_path);
            if !pending.is_file() {
                report["success"] = serde_json::Value::Bool(false);
                report["error"] = format!("pending no encontrado: {rel_path}").into();
                return Ok(());
            }
            let route_result = route_event(repo, rel_path)?;
            report["steps"]
                .as_array_mut()
                .unwrap()
                .push(json!({"route": route_result}));

            let eid = event_id.clone().unwrap_or_else(|| {
                Path::new(rel_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string()
            });
            let witnesses = list_witnesses(repo, &bus, "processed_subscribers", &eid);
            let processing_header = repo.join(&bus.processing).join(format!("{eid}.json"));
            report["witnesses_processed"] = json!(witnesses.iter().map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string()).collect::<Vec<_>>());
            report["processing_header_created"] = processing_header.is_file().into();
            let sweep = route_result
                .get("data")
                .and_then(|d| d.get("sweep"))
                .cloned()
                .unwrap_or(json!({}));
            report["parent_still_pending"] = pending.is_file().into();
            report["sweep"] = sweep.clone();
            report["parent_purged"] = (!pending.is_file()).into();
            report["dispatch_mode"] = route_result
                .get("data")
                .and_then(|d| d.get("dispatch_mode"))
                .cloned()
                .unwrap_or(Value::Null);
            let success = route_result.get("success") == Some(&json!(true))
                && !pending.is_file()
                && sweep.get("status") == Some(&json!("purged"));
            report["success"] = success.into();
            exit_code = if success { 0 } else { 1 };
        }
        Ok(())
    })();

    if let Err(e) = run_result {
        report["error"] = e.into();
        report["success"] = false.into();
    }

    let bus = ensure_event_bus_topology(repo).unwrap_or_else(|_| {
        execute_process::engine::eda_bus_topology::EventBusTopology {
            pending: ".events/pending".into(),
            processing: ".events/processing".into(),
            processing_subscribers: ".events/processing/subscribers".into(),
            processed: ".events/processed".into(),
            processed_subscribers: ".events/processed/subscribers".into(),
            dead_letter: ".events/dead-letter".into(),
            dead_letter_subscribers: ".events/dead-letter/subscribers".into(),
            subscriptions: "SddIA/core/event-domain-subscriptions.json".into(),
        }
    });
    let orphan_removed = cleanup_orphan_core_eda_e2e_tools(repo);
    if !orphan_removed.is_empty() {
        report["orphan_core_removed"] = json!(orphan_removed);
    }
    if created_entity {
        if let (Some(name), Some(eid)) = (entity_name.as_deref(), event_id.as_deref()) {
            report["cleanup"] = cleanup_lab_entity_forge(repo, &bus, entity_class, name, Some(eid));
        }
    }
    report["cleaned"] = (!keep_tmp()).into();

    print_json_report(&report, json_out);
    exit_code
}
