//! Proceso `entity-manager` nativo (P17 — sin engine bridge Python).

use super::actions;
use super::invoke_orchestrator::invoke_process;
use super::thermodynamic;
use super::workspace::bootstrap_workspace;
use crate::core::parser::parse_frontmatter;
use crate::core::resolver::{validate_process_inputs, ProcessDef};
use crate::envelope::OrchestratorEnvelope;
use crate::forges::materialize_by_inputs;
use serde_json::{json, Map, Value};
use std::path::Path;
use std::time::Instant;
use uuid::Uuid;

const PILOT_CLASSES: &[&str] = &[
    "skill", "event", "process", "agent", "tool", "action", "norm", "codex", "suite", "daemon",
];

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn merge_maps(base: &Value, extra: Map<String, Value>) -> Value {
    let mut m = base.as_object().cloned().unwrap_or_default();
    for (k, v) in extra {
        m.insert(k, v);
    }
    Value::Object(m)
}

fn creator_name(class: &str) -> Option<&'static str> {
    match class {
        "skill" => Some("skill-creator"),
        "process" => Some("process-creator"),
        "agent" => Some("agent-creator"),
        "tool" => Some("tool-creator"),
        "action" => Some("action-creator"),
        "norm" => Some("norm-creator"),
        "codex" => Some("codex-creator"),
        "event" => Some("event-creator"),
        "suite" => Some("suite-creator"),
        "daemon" => Some("daemon-creator"),
        _ => None,
    }
}

fn dir_by_class(class: &str) -> Option<&'static str> {
    match class {
        "skill" => Some("SddIA/skills"),
        "process" => Some("SddIA/process"),
        "agent" => Some("SddIA/agents"),
        "tool" => Some("SddIA/tools"),
        "action" => Some("SddIA/actions"),
        "norm" => Some("SddIA/library/norms"),
        "codex" => Some("SddIA/library/codexes"),
        "event" => Some("SddIA/events"),
        "suite" => Some("SddIA/suites"),
        "daemon" => Some("SddIA/daemons"),
        _ => None,
    }
}

fn base_creator_inputs(class: &str, lifecycle: &str, seed: &Value) -> Value {
    let scope = seed.get("scope").and_then(|v| v.as_str()).unwrap_or("core");
    let origin = if scope == "local" { "local" } else { "core" };
    json!({
        "entity_class": class,
        "lifecycle_operation": lifecycle,
        "origin_topology": seed.get("origin_topology").unwrap_or(&json!(origin)),
    })
}

fn seed_field(seed: &Value, key: &str, default: Value) -> Value {
    seed.get(key).cloned().unwrap_or(default)
}

fn with_forge_seed_optionals(mut out: Value, seed: &Value) -> Value {
    if let Value::Object(map) = &mut out {
        for key in ["hash_refresh_only", "markdown_body_replacements"] {
            if let Some(v) = seed.get(key) {
                map.insert(key.into(), v.clone());
            }
        }
    }
    out
}

fn creator_inputs_from_entity(
    class: &str,
    entity_name: &str,
    lifecycle: &str,
    seed: &Value,
) -> Result<Value, String> {
    let base = base_creator_inputs(class, lifecycle, seed);
    let out = match class {
        "tool" => {
            let tname = seed.get("tool_name").and_then(|v| v.as_str()).unwrap_or(entity_name);
            merge_maps(
                &base,
                Map::from_iter([
                    ("tool_name".into(), json!(tname)),
                    ("tool_id".into(), seed_field(seed, "tool_id", json!(tname))),
                    ("scope".into(), seed_field(seed, "scope", json!("core"))),
                    ("domain_origin".into(), seed_field(seed, "domain_origin", json!("SddIA"))),
                    ("tool_context".into(), seed_field(seed, "tool_context", json!("ecosystem-evolution"))),
                    ("required_secrets".into(), seed_field(seed, "required_secrets", json!([]))),
                    ("dependencies".into(), seed_field(seed, "dependencies", json!([]))),
                    ("tool_outputs".into(), seed_field(seed, "tool_outputs", json!([]))),
                    (
                        "execution_logic".into(),
                        seed_field(seed, "execution_logic", json!(format!("Tool {entity_name}"))),
                    ),
                    ("tools_contract_version".into(), seed_field(seed, "tools_contract_version", json!("1.2.0"))),
                ]),
            )
        }
        "skill" => merge_maps(
            &base,
            Map::from_iter([
                ("skill_name".into(), seed_field(seed, "skill_name", json!(entity_name))),
                ("skill_context".into(), seed_field(seed, "skill_context", json!("ecosystem-evolution"))),
                ("skill_description".into(), seed_field(seed, "skill_description", json!(""))),
                ("skill_inputs_schema".into(), seed_field(seed, "skill_inputs_schema", json!([]))),
                ("skill_outputs_schema".into(), seed_field(seed, "skill_outputs_schema", json!([]))),
                ("skill_version".into(), seed_field(seed, "skill_version", json!("1.0.0"))),
                ("skills_contract_version".into(), seed_field(seed, "skills_contract_version", json!("1.1.0"))),
            ]),
        ),
        "event" => {
            let mut fields: Map<String, Value> = Map::from_iter([
                ("event_name".into(), seed_field(seed, "event_name", json!(entity_name))),
                ("event_type".into(), seed_field(seed, "event_type", json!(""))),
                ("event_context".into(), seed_field(seed, "event_context", json!("ecosystem-evolution"))),
                ("event_description".into(), seed_field(seed, "event_description", json!(""))),
                ("payload_required".into(), seed_field(seed, "payload_required", json!([]))),
                ("payload_optional".into(), seed_field(seed, "payload_optional", json!([]))),
                ("payload_forbidden".into(), seed_field(seed, "payload_forbidden", json!([]))),
                ("emitter_agents".into(), seed_field(seed, "emitter_agents", json!([]))),
                ("event_version".into(), seed_field(seed, "event_version", json!("1.0.0"))),
                ("events_contract_version".into(), seed_field(seed, "events_contract_version", json!("1.1.0"))),
            ]);
            if let Some(family) = seed.get("event_family") {
                fields.insert("event_family".into(), family.clone());
            }
            merge_maps(&base, fields)
        }
        "action" => {
            let mut fields: Map<String, Value> = Map::from_iter([
                ("action_name".into(), seed_field(seed, "action_name", json!(entity_name))),
                ("action_context".into(), seed_field(seed, "action_context", json!("ecosystem-evolution"))),
                ("action_inputs".into(), seed_field(seed, "action_inputs", json!([]))),
                ("action_outputs".into(), seed_field(seed, "action_outputs", json!([]))),
                (
                    "orchestration_logic".into(),
                    seed_field(seed, "orchestration_logic", json!(format!("Acción {entity_name}"))),
                ),
                ("actions_contract_version".into(), seed_field(seed, "actions_contract_version", json!("1.2.0"))),
            ]);
            for optional_key in [
                "action_version",
                "action_body",
                "action_capabilities",
            ] {
                if let Some(value) = seed.get(optional_key) {
                    fields.insert(optional_key.into(), value.clone());
                }
            }
            merge_maps(&base, fields)
        }
        "process" => {
            // process_phases / process_version: solo si vienen en seed.
            // Default stub en update destruiría genoma rico vía patch_process_phases_update.
            let mut fields: Map<String, Value> = Map::from_iter([
                ("process_name".into(), seed_field(seed, "process_name", json!(entity_name))),
                (
                    "process_description".into(),
                    seed_field(seed, "process_description", json!(format!("Proceso {entity_name}"))),
                ),
                ("process_context".into(), seed_field(seed, "process_context", json!("ecosystem-evolution"))),
                ("process_contract_version".into(), seed_field(seed, "process_contract_version", json!("1.3.0"))),
                ("process_aliases".into(), seed_field(seed, "process_aliases", json!([]))),
            ]);
            if let Some(phases) = seed.get("process_phases") {
                fields.insert("process_phases".into(), phases.clone());
            }
            if let Some(ver) = seed
                .get("process_version")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                fields.insert("process_version".into(), json!(ver));
            }
            for optional_key in [
                "process_jurisdiction",
                "process_domain_root",
                "workspace_template",
                "process_inputs",
                "inputs",
                "process_outputs",
                "outputs",
                "process_phase_invocations",
                "phase_invocations",
            ] {
                if let Some(value) = seed.get(optional_key) {
                    fields.insert(optional_key.into(), value.clone());
                }
            }
            merge_maps(&base, fields)
        }
        "agent" => merge_maps(
            &base,
            Map::from_iter([
                ("agent_name".into(), seed_field(seed, "agent_name", json!(entity_name))),
                ("allowed_policies".into(), seed_field(seed, "allowed_policies", json!(["ecosystem-evolution"]))),
                ("agent_inputs".into(), seed_field(seed, "agent_inputs", json!([]))),
                ("agent_outputs".into(), seed_field(seed, "agent_outputs", json!([]))),
                (
                    "agent_purpose".into(),
                    seed_field(seed, "agent_purpose", json!(format!("Agente {entity_name}"))),
                ),
                ("agents_contract_version".into(), seed_field(seed, "agents_contract_version", json!("1.0.0"))),
            ]),
        ),
        "norm" => merge_maps(
            &base,
            Map::from_iter([
                ("tactical_norm_name".into(), seed_field(seed, "tactical_norm_name", json!(entity_name))),
                ("tactical_norm_version".into(), seed_field(seed, "tactical_norm_version", json!("1.0.0"))),
                (
                    "tactical_norm_friction".into(),
                    seed_field(seed, "tactical_norm_friction", json!(format!("Norma {entity_name}"))),
                ),
                (
                    "tactical_norm_hard_constraints".into(),
                    seed_field(seed, "tactical_norm_hard_constraints", json!("Ninguna.")),
                ),
                ("tactical_norm_author".into(), seed_field(seed, "tactical_norm_author", json!("laboratorio"))),
                ("tactical_norm_dependencies".into(), seed_field(seed, "tactical_norm_dependencies", json!([]))),
                ("norms_contract_version".into(), seed_field(seed, "norms_contract_version", json!("1.1.0"))),
                ("norm_scope".into(), seed_field(seed, "norm_scope", json!("agnostic"))),
                ("norm_category".into(), seed_field(seed, "norm_category", json!("workflow"))),
            ]),
        ),
        "codex" => merge_maps(
            &base,
            Map::from_iter([
                ("domain_codex_slug".into(), seed_field(seed, "domain_codex_slug", json!(entity_name))),
                ("domain_codex_name".into(), seed_field(seed, "domain_codex_name", json!(entity_name))),
                ("domain_codex_version".into(), seed_field(seed, "domain_codex_version", json!("1.0.0"))),
                ("domain_codex_author".into(), seed_field(seed, "domain_codex_author", json!("laboratorio"))),
                ("target_environment".into(), seed_field(seed, "target_environment", json!(["dev"]))),
                ("tactical_norm_inventory".into(), seed_field(seed, "tactical_norm_inventory", json!([]))),
                ("codex_contract_version".into(), seed_field(seed, "codex_contract_version", json!("1.0.0"))),
                (
                    "domain_codex_certification_grade".into(),
                    seed_field(seed, "domain_codex_certification_grade", json!("Pendiente")),
                ),
            ]),
        ),
        "suite" => merge_maps(
            &base,
            Map::from_iter([
                ("suite_name".into(), seed_field(seed, "suite_name", json!(entity_name))),
                ("suite_context".into(), seed_field(seed, "suite_context", json!("chaos-engineering"))),
                ("execution_strategy".into(), seed_field(seed, "execution_strategy", json!("run_all"))),
                ("atomic_nodes".into(), seed_field(seed, "atomic_nodes", json!([]))),
                ("suite_version".into(), seed_field(seed, "suite_version", json!("1.0.0"))),
                ("suites_contract_version".into(), seed_field(seed, "suites_contract_version", json!("1.0.0"))),
            ]),
        ),
        other => return Err(format!("entity_class no soportada en forja rápida: {other}")),
    };
    Ok(with_forge_seed_optionals(out, seed))
}

fn merge_handoff(state: &mut Value, forge: &Value) {
    if let Some(obj) = state.as_object_mut() {
        let handoff = obj.entry("handoff".to_string()).or_insert(json!({}));
        if let Some(h) = handoff.as_object_mut() {
            if let Some(f) = forge.as_object() {
                for (k, v) in f {
                    h.insert(k.clone(), v.clone());
                }
            }
        }
    }
}

fn delegate_creator(repo: &Path, inputs: &Value, state: &mut Value) -> Result<Value, String> {
    let lifecycle = str_field(inputs, "lifecycle_operation").unwrap_or_default();
    if lifecycle == "delete" {
        return Ok(json!({"skipped": true, "reason": "delete omite delegación al creator"}));
    }
    let class = str_field(inputs, "entity_class").ok_or("entity_class requerido")?;
    if !PILOT_CLASSES.contains(&class.as_str()) {
        return Err(format!("entity_class fuera del piloto: {class}"));
    }
    let entity_name = str_field(inputs, "entity_name").ok_or("entity_name requerido")?;
    let creator = creator_name(&class).ok_or_else(|| format!("sin creator para {class}"))?;
    let seed = inputs.get("semantic_seed").cloned().unwrap_or(json!({}));
    let child_inputs = creator_inputs_from_entity(&class, &entity_name, &lifecycle, &seed)?;

    let forge_error = match materialize_by_inputs(repo, &child_inputs) {
        Ok(forge) => {
            if forge.get("handoff_entity_uuid").is_some() {
                merge_handoff(state, &forge);
                return Ok(json!({
                    "child_process": creator,
                    "handoff": state.get("handoff"),
                    "forge_only": true,
                }));
            }
            None
        }
        Err(error) => Some(error),
    };

    let data = invoke_process(repo, creator, &child_inputs).map_err(|invoke_error| {
        format!(
            "forja nativa fallida: {}; fallback {creator} fallido: {invoke_error}",
            forge_error.as_deref().unwrap_or("handoff ausente")
        )
    })?;
    let fallback_has_handoff = data.get("handoff_entity_uuid").is_some()
        || data
            .get("handoff")
            .and_then(|handoff| handoff.get("handoff_entity_uuid"))
            .is_some();
    if !fallback_has_handoff {
        return Err(format!(
            "forja nativa fallida: {}; fallback {creator} terminó sin handoff_entity_uuid",
            forge_error.as_deref().unwrap_or("handoff ausente")
        ));
    }
    merge_handoff(state, &data);
    if let Some(h) = data.get("handoff") {
        merge_handoff(state, h);
    }
    for key in [
        "handoff_entity_uuid",
        "handoff_hash_signature_new",
        "handoff_hash_signature_old",
        "handoff_version",
    ] {
        if let Some(v) = data.get(key) {
            merge_handoff(state, &json!({key: v}));
        }
    }
    Ok(json!({
        "child_process": creator,
        "handoff": state.get("handoff"),
    }))
}

fn filesystem_delete(repo: &Path, inputs: &Value, state: &mut Value) -> Result<Value, String> {
    if inputs.get("lifecycle_operation").and_then(|v| v.as_str()) != Some("delete") {
        return Ok(json!({"skipped": true}));
    }
    let class = str_field(inputs, "entity_class").ok_or("entity_class requerido")?;
    let name = str_field(inputs, "entity_name").ok_or("entity_name requerido")?;
    let rel_dir = dir_by_class(&class).ok_or_else(|| format!("entity_class desconocida: {class}"))?;
    let artifact = repo.join(rel_dir).join(format!("{name}.md"));
    if !artifact.is_file() {
        return Err(format!("artefacto no encontrado: {}", artifact.display()));
    }
    let fm = parse_frontmatter(&artifact)?;
    let uuid = fm.get("uuid").and_then(|v| v.as_str());
    let hash_old = fm.get("hash_signature").and_then(|v| v.as_str());
    let version = fm.get("version").and_then(|v| v.as_str());
    let handoff = json!({
        "handoff_entity_uuid": uuid,
        "handoff_hash_signature_new": null,
        "handoff_hash_signature_old": hash_old,
        "handoff_version": version,
    });
    std::fs::remove_file(&artifact).map_err(|e| e.to_string())?;
    merge_handoff(state, &handoff);
    Ok(json!({
        "deleted": artifact.strip_prefix(repo).unwrap_or(&artifact).to_string_lossy().replace('\\', "/"),
    }))
}

fn emit_domain_seal(repo: &Path, inputs: &Value, state: &Value) -> Result<Value, String> {
    let handoff = state.get("handoff").cloned().unwrap_or(json!({}));
    let seed = inputs.get("semantic_seed").cloned().unwrap_or(json!({}));
    let scope = seed.get("scope").and_then(|v| v.as_str()).unwrap_or("core");
    let origin_topology = handoff
        .get("origin_topology")
        .or_else(|| seed.get("origin_topology"))
        .cloned()
        .unwrap_or_else(|| json!(if scope == "local" { "local" } else { "core" }));
    let action_inputs = json!({
        "entity_class": inputs.get("entity_class"),
        "entity_name": inputs.get("entity_name"),
        "lifecycle_operation": inputs.get("lifecycle_operation"),
        "entity_uuid": handoff.get("handoff_entity_uuid"),
        "version": handoff.get("handoff_version"),
        "hash_signature_new": handoff.get("handoff_hash_signature_new"),
        "hash_signature_old": handoff.get("handoff_hash_signature_old"),
        "origin_topology": origin_topology,
        "emitter_agent": inputs.get("emitter_agent").unwrap_or(&json!("entity-manager")),
        "changes_summary": format!(
            "{} {} {}",
            inputs.get("lifecycle_operation").and_then(|v| v.as_str()).unwrap_or(""),
            inputs.get("entity_class").and_then(|v| v.as_str()).unwrap_or(""),
            inputs.get("entity_name").and_then(|v| v.as_str()).unwrap_or(""),
        ),
    });
    actions::try_run_native(repo, "emit-domain-mutation", &action_inputs)?
        .ok_or_else(|| "emit-domain-mutation nativo no disponible".into())
}

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
    process_def: &ProcessDef,
    phases: &[Value],
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    if process_inputs
        .get("lifecycle_operation")
        .and_then(|v| v.as_str())
        == Some("seal-anchor")
    {
        return super::capsule_seal::run_entity_anchor(repo, process_inputs);
    }

    validate_process_inputs(process_def, process_inputs, "entity-manager")?;

    let toll_start = Instant::now();
    let mut state = json!({
        "handoff": {},
        "inputs": process_inputs,
        "asset_id": Uuid::new_v4().to_string(),
    });
    let template = workspace_template(process_def)?;
    let mut inputs_mut = process_inputs.clone();
    bootstrap_workspace(repo, "entity-manager", &template, &mut inputs_mut, &mut state)?;

    let mut phase_reports: Vec<Value> = Vec::new();
    let mut failed = false;

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

        if phase_name == "Delegación al creator" {
            match delegate_creator(repo, &inputs_mut, &mut state) {
                Ok(r) => {
                    entry["status"] = json!("executed");
                    entry["child"] = r.get("child_process").cloned().unwrap_or(Value::Null);
                }
                Err(e) => {
                    entry["status"] = json!("failed");
                    entry["error"] = json!(e);
                    failed = true;
                }
            }
        } else if phase_name == "Delete físico" {
            if inputs_mut.get("lifecycle_operation").and_then(|v| v.as_str()) == Some("delete") {
                match filesystem_delete(repo, &inputs_mut, &mut state) {
                    Ok(_) => {
                        entry["status"] = json!("executed");
                        entry["handler"] = json!("filesystem-delete");
                    }
                    Err(e) => {
                        entry["status"] = json!("failed");
                        entry["error"] = json!(e);
                        failed = true;
                    }
                }
            } else {
                entry["status"] = json!("skipped");
                entry["note"] = json!("fase omitida fuera de delete");
            }
        } else if phase_name == "Sello universal" {
            match emit_domain_seal(repo, &inputs_mut, &state) {
                Ok(seal) => {
                    entry["status"] = json!("executed");
                    if let Some(obj) = seal.as_object() {
                        for (k, v) in obj {
                            entry[k.clone()] = v.clone();
                        }
                    }
                    merge_handoff(&mut state, &seal);
                }
                Err(e) => {
                    entry["status"] = json!("failed");
                    entry["error"] = json!(e);
                    failed = true;
                }
            }
        } else {
            entry["status"] = json!("simulated");
        }

        phase_reports.push(entry);
        if failed {
            break;
        }
    }

    let handoff = state.get("handoff").cloned().unwrap_or(json!({}));
    let mut data = json!({
        "process_name": "entity-manager",
        "handoff": handoff,
    });
    for key in ["event_id", "target_path", "workspace_path", "execution_id"] {
        if let Some(v) = handoff.get(key).or_else(|| state.get(key)) {
            data[key] = v.clone();
        }
    }

    let status_code = if failed { 1 } else { 0 };
    let duration_ms = toll_start.elapsed().as_millis() as i64;
    data["thermodynamic_toll"] = thermodynamic::run(
        repo,
        "entity-manager",
        &state,
        &inputs_mut,
        status_code,
        duration_ms,
        !failed,
    );

    Ok(OrchestratorEnvelope {
        success: !failed,
        status_code,
        data: Some(data),
        error: if failed {
            Some("entity-manager fase fallida".into())
        } else {
            None
        },
        execution_report: Some(json!({
            "process_name": "entity-manager",
            "phases": phase_reports,
        })),
        exit_code: status_code,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_creator_inputs_include_declared_jurisdiction_fields() {
        let inputs = creator_inputs_from_entity(
            "process",
            "evolution-audit",
            "create",
            &json!({
                "process_jurisdiction": "domain",
                "process_domain_root": "domain/process"
            }),
        )
        .expect("process inputs");

        assert_eq!(inputs["process_jurisdiction"], json!("domain"));
        assert_eq!(inputs["process_domain_root"], json!("domain/process"));
    }

    #[test]
    fn process_creator_inputs_propagate_contract_payload() {
        let inputs = creator_inputs_from_entity(
            "process",
            "evolution-audit",
            "create",
            &json!({
                "process_phases": [{"name": "Inventario", "intent": "listar"}],
                "workspace_template": ".SddIA/workspaces/{process_name}/{execution_id}/",
                "process_inputs": [{"audit_date": "ISO"}],
                "process_outputs": [{"summary": "conteos"}],
                "phase_invocations": []
            }),
        )
        .expect("process inputs");

        assert_eq!(
            inputs["process_phases"][0]["name"],
            json!("Inventario")
        );
        assert!(inputs["workspace_template"].as_str().unwrap().contains("workspaces"));
        assert_eq!(inputs["process_inputs"][0]["audit_date"], json!("ISO"));
        assert_eq!(inputs["process_outputs"][0]["summary"], json!("conteos"));
        assert_eq!(inputs["phase_invocations"], json!([]));
    }
}
