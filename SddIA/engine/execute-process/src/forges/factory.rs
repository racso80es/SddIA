//! Forjas por `entity_class` (paridad `execute_process_forges.py` + skill/event en capsules).

use super::common::{
    append_row, capability_name, handoff_create, idempotent_forge_handoff, parse_frontmatter,
    refresh_process_hash, repo_tool_base, required_str, sha256_canon,
    str_field, generate_uuid,
};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const TRINITY_FAMILIES: &[&str] = &["telemetry", "orchestration", "domain"];

fn resolve_effective_event_family(inputs: &Value) -> Result<String, String> {
    let raw = inputs
        .get("event_family")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "event_family requerido (telemetry | orchestration | domain)".to_string())?;
    let effective = raw.to_lowercase();
    if TRINITY_FAMILIES.contains(&effective.as_str()) {
        Ok(effective)
    } else {
        Err(format!(
            "event_family inválido: {raw:?}; debe ser telemetry, orchestration o domain"
        ))
    }
}

fn event_family_subscriptions_note(family: &str, event_type: &str) -> String {
    let rel = match family {
        "domain" => "event-domain-subscriptions.json",
        "telemetry" => "event-telemetry-subscriptions.json",
        "orchestration" => "event-orchestration-subscriptions.json",
        _ => "event-subscriptions.json",
    };
    format!("Ver `SddIA/core/{rel}` → clave `{event_type}`.")
}

pub fn run_tool_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "tool_name")?;
    let scope = str_field(inputs, "scope", "core");
    let base = repo_tool_base(repo, &scope);
    let tool_path = base.join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if let Some(skip) = idempotent_forge_handoff(&tool_path, &lifecycle)? {
        return Ok(skip);
    }

    let context = str_field(inputs, "tool_context", "ecosystem-evolution");
    let version = str_field(inputs, "tool_version", "1.0.0");
    let contract_ver = str_field(inputs, "tools_contract_version", "1.2.0");
    let domain_origin = str_field(inputs, "domain_origin", "SddIA");
    let desc = inputs
        .get("execution_logic")
        .and_then(|v| v.as_str())
        .unwrap_or(&format!("Tool {name}"))
        .to_string();
    let tool_uuid = generate_uuid(repo)?;
    let cap = capability_name(&name);
    let hash_sig = sha256_canon(
        repo,
        &json!({"tool_name": name, "tool_context": context, "scope": scope}),
    )?;

    let body = format!(
        r#"---
uuid: "{tool_uuid}"
name: "{name}"
version: "{version}"
contract: "tools-contract v{contract_ver}"
domain_origin: "{domain_origin}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
implementation_path_ref: "SddIA/tools/{name}"
---

# {name}

{desc}
"#
    );
    fs::create_dir_all(&base).map_err(|e| e.to_string())?;
    fs::write(&tool_path, body).map_err(|e| e.to_string())?;
    let row = format!(
        "| `{name}.md` | `{tool_uuid}` | {name} | {version} | tools-contract v{contract_ver} | {context} | `{cap}` |"
    );
    append_row(&base.join("index.md"), &row, &name)?;
    Ok(handoff_create(
        &tool_uuid,
        &hash_sig,
        &version,
        json!({
            "origin_topology": if scope == "local" { "local" } else { "core" },
        }),
    ))
}

pub fn run_action_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "action_name")?;
    let action_path = repo.join("SddIA/actions").join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if let Some(skip) = idempotent_forge_handoff(&action_path, &lifecycle)? {
        return Ok(skip);
    }

    let context = str_field(inputs, "action_context", "ecosystem-evolution");
    let version = str_field(inputs, "action_version", "1.0.0");
    let contract_ver = str_field(inputs, "actions_contract_version", "1.2.0");
    let desc = inputs
        .get("orchestration_logic")
        .and_then(|v| v.as_str())
        .unwrap_or(&format!("Acción {name}"))
        .chars()
        .take(80)
        .collect::<String>();
    let action_uuid = generate_uuid(repo)?;
    let cap = capability_name(&name);
    let hash_sig = sha256_canon(repo, &json!({"action_name": name, "action_context": context}))?;

    let body = format!(
        r#"---
uuid: "{action_uuid}"
name: "{name}"
version: "{version}"
contract: "actions-contract v{contract_ver}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
---

# Acción: {name}

{desc}
"#
    );
    fs::create_dir_all(action_path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&action_path, body).map_err(|e| e.to_string())?;
    let row = format!(
        "| {name} | `{action_uuid}` | {version} | {context} | {desc} | `{cap}` |"
    );
    append_row(&repo.join("SddIA/actions/index.md"), &row, &name)?;
    Ok(handoff_create(&action_uuid, &hash_sig, &version, json!({})))
}

pub fn run_process_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "process_name")?;
    let process_path = repo.join("SddIA/process").join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");

    if lifecycle == "update" && process_path.is_file() {
        let fm = parse_frontmatter(&process_path)?;
        let (old_hash, new_hash) = refresh_process_hash(&process_path)?;
        let version = fm
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0.0")
            .to_string();
        return Ok(json!({
            "handoff_entity_uuid": fm.get("uuid"),
            "handoff_hash_signature_new": new_hash,
            "handoff_hash_signature_old": old_hash,
            "handoff_version": version,
        }));
    }

    if let Some(skip) = idempotent_forge_handoff(&process_path, &lifecycle)? {
        return Ok(skip);
    }

    let context = str_field(inputs, "process_context", "ecosystem-evolution");
    let version = str_field(inputs, "process_version", "1.0.0");
    let contract_ver = str_field(inputs, "process_contract_version", "1.3.0");
    let desc = str_field(inputs, "process_description", &format!("Proceso {name}"));
    let phases = inputs
        .get("process_phases")
        .cloned()
        .unwrap_or_else(|| json!([{"name": "Fase inicial", "intent": desc}]));
    let process_uuid = generate_uuid(repo)?;
    let hash_sig = sha256_canon(repo, &json!({"process_phases": phases}))?;

    let body = format!(
        r#"---
uuid: "{process_uuid}"
name: "{name}"
version: "{version}"
contract: "process-contract v{contract_ver}"
context: "{context}"
hash_signature: "{hash_sig}"
phases:
  - name: "Fase inicial"
    intent: "{desc}"
---

# {name}

{desc}
"#
    );
    fs::create_dir_all(process_path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&process_path, body).map_err(|e| e.to_string())?;
    let summary: String = desc.chars().take(60).collect();
    let row = format!("| {name} | {process_uuid} | {version} | {context} | — | {summary} |");
    append_row(&repo.join("SddIA/process/index.md"), &row, &name)?;
    Ok(handoff_create(&process_uuid, &hash_sig, &version, json!({})))
}

pub fn run_agent_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "agent_name")?;
    let agent_path = repo.join("SddIA/agents").join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if let Some(skip) = idempotent_forge_handoff(&agent_path, &lifecycle)? {
        return Ok(skip);
    }

    let policies = inputs
        .get("allowed_policies")
        .cloned()
        .unwrap_or_else(|| json!(["ecosystem-evolution"]));
    let version = str_field(inputs, "agent_version", "1.0.0");
    let contract_ver = str_field(inputs, "agents_contract_version", "1.0.0");
    let purpose = str_field(inputs, "agent_purpose", &format!("Agente {name}"));
    let agent_uuid = generate_uuid(repo)?;
    let hash_sig = sha256_canon(
        repo,
        &json!({"agent_name": name, "allowed_policies": policies}),
    )?;
    let pol_list: Vec<String> = policies
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| format!("`{s}`")))
                .collect()
        })
        .unwrap_or_default();
    let pol_str = pol_list.join(", ");
    let pol_yaml: String = policies
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| format!("  - \"{s}\"")))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| "  - \"ecosystem-evolution\"".into());

    let body = format!(
        r#"---
uuid: "{agent_uuid}"
name: "{name}"
version: "{version}"
contract: "agents-contract v{contract_ver}"
allowed_policies:
{pol_yaml}
hash_signature: "{hash_sig}"
---

# Agente: {name}

{purpose}
"#
    );
    fs::create_dir_all(agent_path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&agent_path, body).map_err(|e| e.to_string())?;
    let row = format!(
        "| `{name}.md` | `{agent_uuid}` | {name} | {version} | agents-contract v{contract_ver} | {pol_str} |"
    );
    append_row(&repo.join("SddIA/agents/index.md"), &row, &name)?;
    Ok(handoff_create(&agent_uuid, &hash_sig, &version, json!({})))
}

pub fn run_norm_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "tactical_norm_name")?;
    let norm_path = repo.join("SddIA/library/norms").join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if let Some(skip) = idempotent_forge_handoff(&norm_path, &lifecycle)? {
        return Ok(skip);
    }

    let version = str_field(inputs, "tactical_norm_version", "1.0.0");
    let friction = str_field(inputs, "tactical_norm_friction", &format!("Norma {name}"));
    let author = str_field(inputs, "tactical_norm_author", "laboratorio");
    let scope = str_field(inputs, "norm_scope", "agnostic");
    let category = str_field(inputs, "norm_category", "workflow");
    let norm_uuid = generate_uuid(repo)?;
    let hash_sig = sha256_canon(
        repo,
        &json!({"tactical_norm_name": name, "friction": friction, "scope": scope}),
    )?;

    let body = format!(
        r#"---
uuid: "{norm_uuid}"
name: "{name}"
version: "{version}"
nature: "tactical-norm"
author: "{author}"
scope: "{scope}"
category: "{category}"
hash_signature: "{hash_sig}"
---

## Directriz Core

{friction}
"#
    );
    fs::create_dir_all(norm_path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&norm_path, body).map_err(|e| e.to_string())?;
    let row = format!("| `{name}.md` | `{norm_uuid}` | {name} | {version} | {scope} | {category} |");
    append_row(&repo.join("SddIA/library/norms/index.md"), &row, &name)?;
    Ok(handoff_create(&norm_uuid, &hash_sig, &version, json!({})))
}

pub fn run_codex_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let slug = optional_name(inputs, "domain_codex_slug")?;
    let codex_path = repo.join("SddIA/library/codexes").join(format!("{slug}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if let Some(skip) = idempotent_forge_handoff(&codex_path, &lifecycle)? {
        return Ok(skip);
    }

    let cname = str_field(inputs, "domain_codex_name", &slug);
    let version = str_field(inputs, "domain_codex_version", "1.0.0");
    let author = str_field(inputs, "domain_codex_author", "laboratorio");
    let envs = inputs
        .get("target_environment")
        .cloned()
        .unwrap_or_else(|| json!(["dev"]));
    let grade = str_field(inputs, "domain_codex_certification_grade", "Pendiente");
    let codex_uuid = generate_uuid(repo)?;
    let env_str = envs
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "dev".into());
    let env_yaml: String = envs
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| format!("  - \"{s}\"")))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| "  - \"dev\"".into());
    let hash_sig = sha256_canon(
        repo,
        &json!({"domain_codex_slug": slug, "target_environment": envs}),
    )?;

    let body = format!(
        r#"---
uuid: "{codex_uuid}"
name: "{cname}"
version: "{version}"
author: "{author}"
certification_grade: "{grade}"
target_environment:
{env_yaml}
hash_signature: "{hash_sig}"
---

# Códice: {cname}

Estrategia de dominio para {env_str}.
"#
    );
    fs::create_dir_all(codex_path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&codex_path, body).map_err(|e| e.to_string())?;
    let row = format!(
        "| `{slug}.md` | `{codex_uuid}` | {cname} | {version} | {env_str} | {grade} |"
    );
    append_row(&repo.join("SddIA/library/codexes/index.md"), &row, &slug)?;
    Ok(handoff_create(&codex_uuid, &hash_sig, &version, json!({})))
}

pub fn run_suite_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "suite_name")?;
    let suite_path = repo.join("SddIA/suites").join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if let Some(skip) = idempotent_forge_handoff(&suite_path, &lifecycle)? {
        return Ok(skip);
    }

    let strategy = str_field(inputs, "execution_strategy", "run_all");
    if strategy != "fail_fast" && strategy != "run_all" {
        return Err("execution_strategy debe ser fail_fast o run_all".into());
    }
    let atomic_nodes = inputs
        .get("atomic_nodes")
        .and_then(|v| v.as_array())
        .filter(|a| !a.is_empty())
        .ok_or_else(|| "atomic_nodes no vacío requerido".to_string())?;

    let context = str_field(inputs, "suite_context", "chaos-engineering");
    let version = str_field(inputs, "suite_version", "1.0.0");
    let contract_ver = str_field(inputs, "suites_contract_version", "1.0.0");
    let suite_uuid = generate_uuid(repo)?;
    let hash_sig = sha256_canon(
        repo,
        &json!({
            "atomic_nodes": atomic_nodes,
            "execution_strategy": strategy,
            "version": version,
        }),
    )?;

    let nodes_yaml: String = atomic_nodes
        .iter()
        .filter_map(|n| {
            let obj = n.as_object()?;
            let pn = obj.get("process_name")?.as_str()?;
            let exit = obj
                .get("expected_exit_code")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let timeout = obj.get("timeout_ms").and_then(|v| v.as_i64()).unwrap_or(120000);
            Some(format!(
                "- process_name: {pn}\n  expected_exit_code: {exit}\n  timeout_ms: {timeout}"
            ))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let ctx_yaml = format!("- {context}");

    let body = format!(
        r#"---
uuid: "{suite_uuid}"
name: {name}
version: "{version}"
contract: suites-contract v{contract_ver}
context:
{ctx_yaml}
hash_signature: {hash_sig}
execution_strategy: {strategy}
atomic_nodes:
{nodes_yaml}
---

# {name}

Suite forjada por suite-creator (laboratorio SddIA).
"#
    );
    fs::create_dir_all(suite_path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&suite_path, body).map_err(|e| e.to_string())?;
    let node_count = atomic_nodes
        .iter()
        .filter(|n| n.get("process_name").and_then(|v| v.as_str()).is_some())
        .count();
    let row = format!(
        "| `{name}.md` | `{suite_uuid}` | {name} | {version} | {strategy} | {node_count} | Suite forjada ({name}). |"
    );
    append_row(&repo.join("SddIA/suites/index.md"), &row, &name)?;
    Ok(handoff_create(&suite_uuid, &hash_sig, &version, json!({})))
}

pub fn run_skill_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "skill_name")?;
    let skill_path = repo.join("SddIA/skills").join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if lifecycle == "create" && skill_path.is_file() {
        return Err(format!("Ya existe {}", skill_path.display()));
    }

    let context = str_field(inputs, "skill_context", "ecosystem-evolution");
    let version = str_field(inputs, "skill_version", "1.0.0");
    let contract_ver = str_field(inputs, "skills_contract_version", "1.1.0");
    let desc = str_field(inputs, "skill_description", &format!("Skill {name}"));
    let in_schema = inputs
        .get("skill_inputs_schema")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let out_schema = inputs
        .get("skill_outputs_schema")
        .cloned()
        .unwrap_or_else(|| json!([]));

    let skill_uuid = generate_uuid(repo)?;
    let hash_sig = sha256_canon(
        repo,
        &json!({
            "skill_context": context,
            "skill_inputs_schema": in_schema,
            "skill_name": name,
            "skill_outputs_schema": out_schema,
            "skill_version": version,
        }),
    )?;
    let cap = capability_name(&name);

    let body = format!(
        r#"---
uuid: "{skill_uuid}"
name: "{name}"
version: "{version}"
contract: "skills-contract v{contract_ver}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
inputs:
  - "inputs_placeholder": "definir segun skill_inputs_schema en forja completa"
outputs:
  - "success": "boolean"
---

# Skill: {name}

{desc}
"#
    );
    fs::create_dir_all(skill_path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&skill_path, body).map_err(|e| e.to_string())?;

    let index_path = repo.join("SddIA/skills/index.md");
    let row = format!(
        "| `{name}.md` | `{skill_uuid}` | {name} | {version} | skills-contract v{contract_ver} | {context} | `{cap}` |"
    );
    if index_path.is_file() {
        let mut idx = fs::read_to_string(&index_path).map_err(|e| e.to_string())?;
        if !idx.contains(&name) {
            let marker = "| `shell-executor.md` |";
            if idx.contains(marker) {
                idx = idx.replacen(marker, &(row.clone() + "\n" + marker), 1);
            } else if !idx.contains(&name) {
                idx = format!("{}\n{row}\n", idx.trim_end());
            }
            fs::write(&index_path, idx).map_err(|e| e.to_string())?;
        }
    }

    Ok(json!({
        "artifact_skill_md": skill_path.strip_prefix(repo).unwrap_or(&skill_path).to_string_lossy().replace('\\', "/"),
        "artifact_skills_index": "SddIA/skills/index.md",
        "handoff_entity_uuid": skill_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": null,
        "handoff_version": version,
    }))
}

pub fn run_event_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "event_name")?;
    let effective_family = resolve_effective_event_family(inputs)?;
    let events_root = repo.join("SddIA/events").join(&effective_family);
    let event_path = events_root.join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if lifecycle == "create" && event_path.is_file() {
        return Err(format!("Ya existe {}", event_path.display()));
    }

    let event_type = required_str(inputs, "event_type")?;
    let context = str_field(inputs, "event_context", "ecosystem-evolution");
    let version = str_field(inputs, "event_version", "1.0.0");
    let contract_ver = str_field(inputs, "events_contract_version", "1.1.0");
    let desc = str_field(inputs, "event_description", &format!("Clase de Evento {event_type}"));
    let payload_required = inputs.get("payload_required").cloned().unwrap_or_else(|| json!([]));
    let payload_optional = inputs.get("payload_optional").cloned().unwrap_or_else(|| json!([]));
    let payload_forbidden = inputs.get("payload_forbidden").cloned().unwrap_or_else(|| json!([]));
    let emitters = inputs.get("emitter_agents").cloned().unwrap_or_else(|| json!([]));

    let event_uuid = generate_uuid(repo)?;
    let hash_sig = sha256_canon(
        repo,
        &json!({
            "event_name": name,
            "event_type": event_type,
            "event_family": effective_family,
            "event_version": version,
            "event_context": context,
            "payload_required": payload_required,
            "payload_optional": payload_optional,
            "payload_forbidden": payload_forbidden,
        }),
    )?;
    let cap = capability_name(&name);

    let req_lines = list_lines(&payload_required, "*(ninguno)*");
    let opt_lines = list_lines(&payload_optional, "*(ninguno)*");
    let forb_lines = list_lines(&payload_forbidden, "*(ninguno)*");
    let emitter_lines = emitter_lines(&emitters);
    let subs_note = event_family_subscriptions_note(&effective_family, &event_type);

    let body = format!(
        r#"---
uuid: "{event_uuid}"
name: "{name}"
version: "{version}"
contract: "events-contract v{contract_ver}"
event_family: "{effective_family}"
event_type: "{event_type}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
---

# Event: {event_type}

{desc}

## Payload ECST

### REQUIRED
{req_lines}

### OPTIONAL
{opt_lines}

### FORBIDDEN
{forb_lines}

## Emisores autorizados

{emitter_lines}

## Suscripciones

{subs_note}
"#
    );
    fs::create_dir_all(&events_root).map_err(|e| e.to_string())?;
    fs::write(&event_path, body).map_err(|e| e.to_string())?;

    let index_path = events_root.join("index.md");
    let row = format!(
        "| `{name}.md` | `{event_uuid}` | {name} | {event_type} | {version} | events-contract v{contract_ver} | {context} | `{cap}` |"
    );
    if index_path.is_file() {
        let mut idx = fs::read_to_string(&index_path).map_err(|e| e.to_string())?;
        if !idx.contains(&name) {
            let header = "| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |\n|----------------|------|------|------------|---------|----------|---------|--------------|\n";
            if idx.contains("| Archivo fuente | uuid | name | event_type |") {
                idx = idx.replacen(header, &(header.to_string() + &row + "\n"), 1);
            } else {
                idx = format!("{}\n{row}\n", idx.trim_end());
            }
            fs::write(&index_path, idx).map_err(|e| e.to_string())?;
        }
    }

    Ok(json!({
        "artifact_event_md": event_path.strip_prefix(repo).unwrap_or(&event_path).to_string_lossy().replace('\\', "/"),
        "artifact_events_index": index_path.strip_prefix(repo).unwrap_or(&index_path).to_string_lossy().replace('\\', "/"),
        "handoff_entity_uuid": event_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": null,
        "handoff_version": version,
    }))
}

fn optional_name(inputs: &Value, key: &str) -> Result<String, String> {
    if let Some(s) = inputs.get(key).and_then(|v| v.as_str()) {
        if !s.trim().is_empty() {
            return Ok(s.trim().to_string());
        }
    }
    if let Some(s) = inputs.get("entity_name").and_then(|v| v.as_str()) {
        if !s.trim().is_empty() {
            return Ok(s.trim().to_string());
        }
    }
    Err(format!("{key} requerido"))
}

fn list_lines(arr: &Value, empty: &str) -> String {
    arr.as_array()
        .map(|items| {
            if items.is_empty() {
                format!("- {empty}")
            } else {
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| format!("- `{s}`")))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        })
        .unwrap_or_else(|| format!("- {empty}"))
}

fn emitter_lines(emitters: &Value) -> String {
    emitters
        .as_array()
        .map(|items| {
            if items.is_empty() {
                "- *(definir en forja completa)*".into()
            } else {
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| format!("- `{s}`")))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        })
        .unwrap_or_else(|| "- *(definir en forja completa)*".into())
}

pub fn materialize_by_inputs(repo: &Path, inputs: &Value) -> Result<Value, String> {
    if let Some(class) = inputs.get("entity_class").and_then(|v| v.as_str()) {
        return match class {
            "tool" => run_tool_forge(repo, inputs),
            "action" => run_action_forge(repo, inputs),
            "process" => run_process_forge(repo, inputs),
            "agent" => run_agent_forge(repo, inputs),
            "norm" => run_norm_forge(repo, inputs),
            "codex" => run_codex_forge(repo, inputs),
            "suite" => run_suite_forge(repo, inputs),
            "skill" => run_skill_forge(repo, inputs),
            "event" => run_event_forge(repo, inputs),
            other => Err(format!("entity_class no soportada en forja nativa: {other}")),
        };
    }
    if inputs.get("skill_name").is_some()
        || (inputs.get("skill_inputs_schema").is_some() && inputs.get("skill_context").is_some())
    {
        return run_skill_forge(repo, inputs);
    }
    if inputs.get("event_type").is_some() || inputs.get("event_name").is_some() {
        return run_event_forge(repo, inputs);
    }
    if inputs.get("tool_name").is_some() {
        return run_tool_forge(repo, inputs);
    }
    if inputs.get("action_name").is_some() {
        return run_action_forge(repo, inputs);
    }
    if inputs.get("process_name").is_some() {
        return run_process_forge(repo, inputs);
    }
    if inputs.get("agent_name").is_some() {
        return run_agent_forge(repo, inputs);
    }
    if inputs.get("tactical_norm_name").is_some() {
        return run_norm_forge(repo, inputs);
    }
    if inputs.get("domain_codex_slug").is_some() {
        return run_codex_forge(repo, inputs);
    }
    if inputs.get("suite_name").is_some() || inputs.get("atomic_nodes").is_some() {
        return run_suite_forge(repo, inputs);
    }
    Err("Forja física no disponible para esta forma de inputs".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forges::common::canon_json_sorted;
    use std::fs;

    #[test]
    fn tool_forge_hash_signature_deterministic_with_lab_sha256() {
        std::env::set_var("SDDIA_FORGE_LAB_SHA256", "abc123deadbeef");
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "11111111-1111-4111-8111-111111111111");
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join(".SddIA/tools")).unwrap();
        fs::write(
            repo.join(".SddIA/tools/index.md"),
            "| Archivo fuente | uuid | name | version | contract | context | Capabilities |\n|----------------|------|------|---------|------------|---------|--------------|\n",
        )
        .unwrap();

        let inputs = json!({
            "entity_class": "tool",
            "tool_name": "forge-parity-lab",
            "scope": "local",
            "execution_logic": "lab",
            "lifecycle_operation": "create",
        });
        let out = run_tool_forge(repo, &inputs).expect("forge");
        assert_eq!(
            out.get("handoff_hash_signature_new"),
            Some(&json!("sha256:abc123deadbeef"))
        );
        let body = fs::read_to_string(repo.join(".SddIA/tools/forge-parity-lab.md")).unwrap();
        assert!(body.contains("sha256:abc123deadbeef"));
        std::env::remove_var("SDDIA_FORGE_LAB_SHA256");
        std::env::remove_var("SDDIA_FORGE_LAB_UUID");
    }

    #[test]
    fn tool_forge_canon_matches_python_shape() {
        let canon = json!({"tool_name": "x", "tool_context": "ecosystem-evolution", "scope": "local"});
        assert_eq!(
            canon_json_sorted(&canon),
            r#"{"scope":"local","tool_context":"ecosystem-evolution","tool_name":"x"}"#
        );
    }
}
