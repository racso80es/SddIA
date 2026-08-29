//! Forjas por `entity_class` (paridad `execute_process_forges.py` + skill/event en capsules).

use super::common::{
    append_row, capability_name, dependencies_from_inputs, format_dependencies_yaml, handoff_create,
    idempotent_forge_handoff, optional_str, parse_frontmatter, patch_action_content_update,
    patch_artifact_body_replacements, patch_hash_signature_refresh, patch_norm_content_update,
    patch_process_phases_update, refresh_process_hash, repo_tool_base, required_str, sha256_canon,
    str_field,     sync_action_index_row, sync_daemons_index_census, update_library_norm_index_version,
    update_process_index_version, norm_integrity_hash, generate_uuid,
};
use crate::core::paths::load_paths_config;
use crate::core::resolver::{process_search_roots, resolve_process_path};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

/// Fallback alineado a `domain_authority::SOFTWARE_PROCESS_MEMBERSHIP` (evita ciclo forges↔engine).
const SOFTWARE_PROCESS_MEMBERSHIP_FALLBACK: &[&str] = &[
    "feature",
    "bug-fix",
    "refactorization",
    "pull-request-review",
    "accept-pr",
    "delivery-close-cycle",
];

fn process_in_software_membership(repo: &Path, name: &str) -> bool {
    let path = repo.join("SddIA/library/codexes/codex-software-engineering.md");
    if path.is_file() {
        if let Ok(raw) = fs::read_to_string(&path) {
            if let Some((fm, _)) = split_md_frontmatter(&raw) {
                if let Ok(v) = serde_yaml::from_str::<Value>(&fm) {
                    if let Some(arr) = v.get("process_membership").and_then(|x| x.as_array()) {
                        return arr.iter().any(|x| x.as_str() == Some(name));
                    }
                }
            }
        }
    }
    SOFTWARE_PROCESS_MEMBERSHIP_FALLBACK.contains(&name)
}

fn split_md_frontmatter(raw: &str) -> Option<(String, String)> {
    let mut lines = raw.lines();
    if lines.next()? != "---" {
        return None;
    }
    let mut fm = String::new();
    for line in lines.by_ref() {
        if line == "---" {
            let body: String = lines.collect::<Vec<_>>().join("\n");
            return Some((fm, body));
        }
        fm.push_str(line);
        fm.push('\n');
    }
    None
}

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

    if lifecycle == "update" && tool_path.is_file() {
        if inputs
            .get("hash_refresh_only")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let patch = patch_hash_signature_refresh(&tool_path)?;
            return Ok(json!({
                "handoff_entity_uuid": patch.entity_uuid,
                "handoff_hash_signature_new": patch.new_hash,
                "handoff_hash_signature_old": patch.old_hash,
                "handoff_version": patch.version,
            }));
        }
    }

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

    if lifecycle == "update" && action_path.is_file() {
        if inputs
            .get("hash_refresh_only")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let patch = patch_hash_signature_refresh(&action_path)?;
            return Ok(json!({
                "handoff_entity_uuid": patch.entity_uuid,
                "handoff_hash_signature_new": patch.new_hash,
                "handoff_hash_signature_old": patch.old_hash,
                "handoff_version": patch.version,
            }));
        }
        let patch = patch_action_content_update(&action_path, inputs)?;
        let context = str_field(inputs, "action_context", "ecosystem-evolution");
        let description = inputs
            .get("orchestration_logic")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let capabilities = inputs
            .get("action_capabilities")
            .cloned()
            .unwrap_or_else(|| json!([]));
        sync_action_index_row(
            &repo.join("SddIA/actions/index.md"),
            &name,
            &patch.entity_uuid,
            &patch.new_version,
            &context,
            &description,
            &capabilities,
        )?;
        return Ok(json!({
            "handoff_entity_uuid": patch.entity_uuid,
            "handoff_hash_signature_new": patch.new_hash,
            "handoff_hash_signature_old": patch.old_hash,
            "handoff_version": patch.new_version,
        }));
    }

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

/// Destino de escritura process (L-JURIS-MEMBERSHIP-PLUS-FLAG + process_domain_roots).
struct ProcessWriteDest {
    root_abs: PathBuf,
    root_rel: String,
    jurisdiction: String,
}

fn relpath_under_repo(repo: &Path, abs: &Path) -> String {
    abs.strip_prefix(repo)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| abs.display().to_string().replace('\\', "/"))
}

fn cfg_process_core_rel(cfg: &Value) -> String {
    cfg.get("directories")
        .and_then(|d| d.get("process"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_end_matches('/').replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "SddIA/process".to_string())
}

fn cfg_process_domain_rels(cfg: &Value) -> Vec<String> {
    cfg.get("directories")
        .and_then(|d| d.get("process_domain_roots"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.trim().trim_end_matches('/').replace('\\', "/"))
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn classify_process_jurisdiction(repo: &Path, inputs: &Value, name: &str) -> Result<String, String> {
    match inputs
        .get("process_jurisdiction")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some("domain") => Ok("domain".into()),
        Some("core") => Ok("core".into()),
        Some(other) => Err(format!(
            "process_jurisdiction inválido: {other:?}; debe ser domain | core"
        )),
        None => {
            if process_in_software_membership(repo, name) {
                Ok("domain".into())
            } else {
                Ok("core".into())
            }
        }
    }
}

fn resolve_process_write_dest(
    repo: &Path,
    inputs: &Value,
    name: &str,
) -> Result<ProcessWriteDest, String> {
    let cfg = load_paths_config(repo).unwrap_or_else(|_| {
        json!({
            "directories": {
                "process": "SddIA/process",
                "process_domain_roots": []
            }
        })
    });
    let core_rel = cfg_process_core_rel(&cfg);
    let domain_rels = cfg_process_domain_rels(&cfg);
    let jurisdiction = classify_process_jurisdiction(repo, inputs, name)?;

    if jurisdiction == "core" {
        return Ok(ProcessWriteDest {
            root_abs: repo.join(&core_rel),
            root_rel: core_rel,
            jurisdiction,
        });
    }

    if domain_rels.is_empty() {
        return Err(
            "process_jurisdiction=domain pero directories.process_domain_roots vacío (Cúmulo/overlay)"
                .into(),
        );
    }

    let chosen = if let Some(explicit) = inputs
        .get("process_domain_root")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let explicit = explicit.trim_end_matches('/').replace('\\', "/");
        if !domain_rels.iter().any(|r| r == &explicit) {
            return Err(format!(
                "process_domain_root {explicit:?} no ∈ directories.process_domain_roots fusionado"
            ));
        }
        explicit
    } else {
        domain_rels[0].clone()
    };

    Ok(ProcessWriteDest {
        root_abs: repo.join(&chosen),
        root_rel: chosen,
        jurisdiction,
    })
}

/// L-UNIQ-MULTI: colisión name/aliases en unión Core ∪ process_domain_roots.
fn find_process_identity_collision(
    repo: &Path,
    name: &str,
    extra_aliases: &[String],
) -> Result<Option<PathBuf>, String> {
    let roots = match process_search_roots(repo) {
        Ok(r) => r,
        Err(_) => vec![repo.join("SddIA/process")],
    };
    let mut needles: Vec<&str> = vec![name];
    for a in extra_aliases {
        needles.push(a.as_str());
    }

    for root in roots {
        if !root.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&root).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if stem == "index" || stem == "process-contract" {
                continue;
            }
            if needles.iter().any(|n| *n == stem) {
                return Ok(Some(path));
            }
            let fm = parse_frontmatter(&path)?;
            if let Some(n) = fm.get("name").and_then(|v| v.as_str()) {
                if needles.iter().any(|x| *x == n) {
                    return Ok(Some(path));
                }
            }
            if let Some(aliases) = fm.get("aliases").and_then(|v| v.as_array()) {
                for a in aliases {
                    if let Some(as_) = a.as_str() {
                        if needles.iter().any(|x| *x == as_) {
                            return Ok(Some(path));
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

fn process_aliases_from_inputs(inputs: &Value) -> Vec<String> {
    inputs
        .get("process_aliases")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(|s| s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn locate_existing_process_path(repo: &Path, name: &str) -> Option<PathBuf> {
    resolve_process_path(repo, name).ok().or_else(|| {
        let legacy = repo.join("SddIA/process").join(format!("{name}.md"));
        legacy.is_file().then_some(legacy)
    })
}

fn forge_outputs_extra(dest: &ProcessWriteDest, name: &str) -> Value {
    json!({
        "resolved_process_root": dest.root_rel,
        "process_jurisdiction_applied": dest.jurisdiction,
        "artifact_process_md": format!("{}/{name}.md", dest.root_rel),
        "artifact_process_index": format!("{}/index.md", dest.root_rel),
    })
}

pub fn run_process_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "process_name")?;
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");

    if lifecycle == "update" {
        if let Some(process_path) = locate_existing_process_path(repo, &name) {
            let dest_root = process_path
                .parent()
                .ok_or_else(|| "process path sin parent".to_string())?
                .to_path_buf();
            let root_rel = relpath_under_repo(repo, &dest_root);
            let core_rel = load_paths_config(repo)
                .map(|c| cfg_process_core_rel(&c))
                .unwrap_or_else(|_| "SddIA/process".into());
            let jurisdiction = if root_rel == core_rel {
                "core"
            } else {
                "domain"
            };
            let dest = ProcessWriteDest {
                root_abs: dest_root.clone(),
                root_rel: root_rel.clone(),
                jurisdiction: jurisdiction.into(),
            };
            if let Some(replacements) = inputs.get("markdown_body_replacements") {
                let (entity_uuid, body_old_hash, _, version) =
                    patch_artifact_body_replacements(&process_path, replacements)?;
                let (phase_old_hash, new_hash) = refresh_process_hash(&process_path)?;
                let old_hash = phase_old_hash.unwrap_or(body_old_hash);
                let mut out = json!({
                    "handoff_entity_uuid": entity_uuid,
                    "handoff_hash_signature_new": new_hash,
                    "handoff_hash_signature_old": old_hash,
                    "handoff_version": version,
                });
                if let (Value::Object(base), Value::Object(ext)) =
                    (&mut out, forge_outputs_extra(&dest, &name))
                {
                    for (k, v) in ext {
                        base.insert(k, v);
                    }
                }
                return Ok(out);
            }
            let phases_opt = inputs.get("process_phases").filter(|p| {
                p.as_array().map(|a| !a.is_empty()).unwrap_or(false)
            });
            if let Some(phases) = phases_opt {
                let explicit_ver = optional_str(inputs, "process_version");
                let inputs_patch = inputs
                    .get("process_inputs")
                    .or_else(|| inputs.get("inputs"));
                let patch = patch_process_phases_update(
                    &process_path,
                    phases,
                    explicit_ver.as_deref(),
                    inputs_patch,
                )?;
                if patch.old_version != patch.new_version {
                    update_process_index_version(
                        &dest_root.join("index.md"),
                        &name,
                        &patch.new_version,
                    )?;
                }
                let mut out = json!({
                    "handoff_entity_uuid": patch.entity_uuid,
                    "handoff_hash_signature_new": patch.new_hash,
                    "handoff_hash_signature_old": patch.old_hash,
                    "handoff_version": patch.new_version,
                });
                if let (Value::Object(base), Value::Object(ext)) =
                    (&mut out, forge_outputs_extra(&dest, &name))
                {
                    for (k, v) in ext {
                        base.insert(k, v);
                    }
                }
                return Ok(out);
            }

            let fm = parse_frontmatter(&process_path)?;
            let (old_hash, new_hash) = refresh_process_hash(&process_path)?;
            let version = fm
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("1.0.0")
                .to_string();
            let mut out = json!({
                "handoff_entity_uuid": fm.get("uuid"),
                "handoff_hash_signature_new": new_hash,
                "handoff_hash_signature_old": old_hash,
                "handoff_version": version,
            });
            if let (Value::Object(base), Value::Object(ext)) =
                (&mut out, forge_outputs_extra(&dest, &name))
            {
                for (k, v) in ext {
                    base.insert(k, v);
                }
            }
            return Ok(out);
        }
    }

    let dest = resolve_process_write_dest(repo, inputs, &name)?;
    let process_path = dest.root_abs.join(format!("{name}.md"));
    let aliases = process_aliases_from_inputs(inputs);

    if lifecycle == "create" {
        if let Some(hit) = find_process_identity_collision(repo, &name, &aliases)? {
            if hit != process_path {
                return Err(format!(
                    "L-UNIQ-MULTI: identidad {name:?} colisiona con {}",
                    relpath_under_repo(repo, &hit)
                ));
            }
        }
    }

    if let Some(skip) = idempotent_forge_handoff(&process_path, &lifecycle)? {
        let mut out = skip;
        if let (Value::Object(base), Value::Object(ext)) =
            (&mut out, forge_outputs_extra(&dest, &name))
        {
            for (k, v) in ext {
                base.insert(k, v);
            }
        }
        return Ok(out);
    }

    let context_val = match inputs.get("process_context") {
        Some(v @ Value::Array(a)) if !a.is_empty() => v.clone(),
        Some(Value::String(s)) if !s.trim().is_empty() => json!(s.trim()),
        _ => json!("ecosystem-evolution"),
    };
    let context_cell = match &context_val {
        Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        Value::String(s) => s.clone(),
        _ => "ecosystem-evolution".into(),
    };
    let version = str_field(inputs, "process_version", "1.0.0");
    let contract_ver = str_field(inputs, "process_contract_version", "1.3.0");
    let desc = str_field(inputs, "process_description", &format!("Proceso {name}"));
    let phases = inputs
        .get("process_phases")
        .cloned()
        .filter(|p| p.as_array().map(|a| !a.is_empty()).unwrap_or(false))
        .unwrap_or_else(|| json!([{"name": "Fase inicial", "intent": desc}]));
    let process_uuid = generate_uuid(repo)?;
    let workspace_template = str_field(
        inputs,
        "workspace_template",
        ".SddIA/workspaces/{process_name}/{execution_id}/",
    );

    let mut fm = serde_json::Map::new();
    fm.insert("uuid".into(), json!(process_uuid));
    fm.insert("name".into(), json!(name));
    fm.insert("version".into(), json!(version));
    fm.insert(
        "contract".into(),
        json!(format!("process-contract v{contract_ver}")),
    );
    fm.insert("workspace_template".into(), json!(workspace_template));
    fm.insert("context".into(), context_val);
    fm.insert("hash_signature".into(), json!("sha256:pending"));
    if !aliases.is_empty() {
        fm.insert("aliases".into(), json!(aliases));
    }
    if let Some(ins) = inputs
        .get("process_inputs")
        .or_else(|| inputs.get("inputs"))
        .cloned()
        .filter(|v| !v.is_null())
    {
        fm.insert("inputs".into(), ins);
    }
    if let Some(outs) = inputs
        .get("process_outputs")
        .or_else(|| inputs.get("outputs"))
        .cloned()
        .filter(|v| !v.is_null())
    {
        fm.insert("outputs".into(), outs);
    }
    fm.insert("phases".into(), phases.clone());
    if let Some(inv) = inputs
        .get("process_phase_invocations")
        .or_else(|| inputs.get("phase_invocations"))
        .cloned()
        .filter(|v| !v.is_null())
    {
        fm.insert("phase_invocations".into(), inv);
    }

    let yaml_out = serde_yaml::to_string(&Value::Object(fm)).map_err(|e| e.to_string())?;
    let yaml_out = yaml_out
        .strip_prefix("---\n")
        .unwrap_or(&yaml_out)
        .trim_end()
        .to_string();
    let body = format!("---\n{yaml_out}\n---\n\n# {name}\n\n{desc}\n");
    fs::create_dir_all(&dest.root_abs).map_err(|e| e.to_string())?;
    fs::write(&process_path, &body).map_err(|e| e.to_string())?;
    let written = parse_frontmatter(&process_path)?;
    let written_phases = written.get("phases").cloned().unwrap_or(Value::Null);
    if written_phases != phases {
        let _ = fs::remove_file(&process_path);
        return Err(
            "materialización incompleta: phases escritas ≠ process_phases (EV-AUD-003)".into(),
        );
    }
    let (_hash_old, hash_sig) = match refresh_process_hash(&process_path) {
        Ok(h) => h,
        Err(e) => {
            let _ = fs::remove_file(&process_path);
            return Err(e);
        }
    };
    let summary: String = desc.chars().take(60).collect();
    let aliases_cell = if aliases.is_empty() {
        "—".to_string()
    } else {
        aliases.join(", ")
    };
    let row = format!(
        "| {name} | {process_uuid} | {version} | {context_cell} | {aliases_cell} | {summary} |"
    );
    if let Err(e) = append_row(&dest.root_abs.join("index.md"), &row, &name) {
        let _ = fs::remove_file(&process_path);
        return Err(e);
    }
    Ok(handoff_create(
        &process_uuid,
        &hash_sig,
        &version,
        forge_outputs_extra(&dest, &name),
    ))
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

    if lifecycle == "update" && norm_path.is_file() {
        let patch = patch_norm_content_update(&norm_path, inputs)?;
        if patch.old_version != patch.new_version {
            update_library_norm_index_version(
                &repo.join("SddIA/library/norms/index.md"),
                &name,
                &patch.new_version,
            )?;
        }
        return Ok(json!({
            "handoff_entity_uuid": patch.entity_uuid,
            "handoff_hash_signature_new": patch.new_hash,
            "handoff_hash_signature_old": patch.old_hash,
            "handoff_version": patch.new_version,
        }));
    }

    if let Some(skip) = idempotent_forge_handoff(&norm_path, &lifecycle)? {
        return Ok(skip);
    }

    let version = str_field(inputs, "tactical_norm_version", "1.0.0");
    let friction = str_field(inputs, "tactical_norm_friction", &format!("Norma {name}"));
    let hard_constraints =
        str_field(inputs, "tactical_norm_hard_constraints", "Ninguna.");
    let author = str_field(inputs, "tactical_norm_author", "laboratorio");
    let scope = str_field(inputs, "norm_scope", "agnostic");
    let category = str_field(inputs, "norm_category", "workflow");
    let deps = dependencies_from_inputs(inputs);
    let dependencies_yaml = format_dependencies_yaml(&deps);
    let norm_uuid = generate_uuid(repo)?;
    let hash_sig = norm_integrity_hash(&name, &friction, &hard_constraints, &scope, &deps);

    let body = format!(
        r#"---
uuid: "{norm_uuid}"
name: "{name}"
version: "{version}"
nature: "tactical-norm"
author: "{author}"
scope: "{scope}"
category: "{category}"
{dependencies_yaml}hash_signature: "{hash_sig}"
---

## Directriz Core

{friction}

## Restricciones Duras (Aduana de Fricción)

{hard_constraints}
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

/// Forja nativa `daemon-creator` (L-FORGE): `{name}.md` + fila en `daemons/index.md`.
pub fn run_daemon_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = optional_name(inputs, "daemon_name")?;
    let kebab = regex::Regex::new(r"^[a-z][a-z0-9]*(-[a-z0-9]+)*$").map_err(|e| e.to_string())?;
    if !kebab.is_match(&name) {
        return Err(format!("daemon_name no kebab-case: {name}"));
    }

    let daemons_root = repo.join("SddIA/daemons");
    let daemon_path = daemons_root.join(format!("{name}.md"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");
    if let Some(skip) = idempotent_forge_handoff(&daemon_path, &lifecycle)? {
        return Ok(skip);
    }
    if lifecycle == "create" && daemon_path.is_file() {
        return Err(format!("Ya existe {}", daemon_path.display()));
    }

    let context = str_field(inputs, "daemon_context", "ecosystem-evolution");
    let version = str_field(inputs, "daemon_version", "1.0.0");
    let contract_ver = str_field(inputs, "daemons_contract_version", "1.0.0");
    let desc = str_field(
        inputs,
        "daemon_description",
        &format!("Centinela {name}"),
    );
    let jurisdiction = str_field(
        inputs,
        "daemon_jurisdiction",
        "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus",
    );
    let caps = inputs
        .get("daemon_capabilities")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let exec = inputs
        .get("daemon_execution")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let entrypoint = exec
        .get("entrypoint")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("SddIA/daemons/{name}.sh")
        .replace("{name}", &name);
    let runtime = exec
        .get("runtime")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("native-rust");
    let heartbeat = exec
        .get("heartbeat_interval_seconds")
        .and_then(|v| v.as_u64())
        .or_else(|| {
            exec.get("heartbeat_interval_seconds")
                .and_then(|v| v.as_i64())
                .map(|n| n as u64)
        })
        .unwrap_or(30);
    if heartbeat < 5 {
        return Err(format!(
            "heartbeat_interval_seconds debe ser ≥ 5 (recibido {heartbeat})"
        ));
    }

    let daemon_uuid = generate_uuid(repo)?;
    let hash_sig = sha256_canon(
        repo,
        &json!({
            "daemon_name": name,
            "daemon_version": version,
            "daemon_context": context,
            "daemon_capabilities": caps,
            "daemon_execution": {
                "entrypoint": entrypoint,
                "runtime": runtime,
                "heartbeat_interval_seconds": heartbeat,
            },
        }),
    )?;

    let cap_yaml = caps
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| format!("  - \"{s}\""))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "  - \"daemon-heartbeat\"".into());

    let caps_index = caps
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| format!("`{s}`"))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "`daemon-heartbeat`".into());

    let body = format!(
        r#"---
uuid: "{daemon_uuid}"
name: "{name}"
version: "{version}"
contract: "daemons-contract v{contract_ver}"
context: "{context}"
hash_signature: "{hash_sig}"
capabilities:
{cap_yaml}
execution:
  entrypoint: "{entrypoint}"
  runtime: "{runtime}"
  heartbeat_interval_seconds: {heartbeat}
jurisdiction: "{jurisdiction}"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# {name}

{desc}

Forja: `daemon-creator` (porte nativo `run_daemon_forge`). UUID vía `action:crypto-broker`.
"#
    );
    fs::create_dir_all(&daemons_root).map_err(|e| e.to_string())?;
    fs::write(&daemon_path, body).map_err(|e| e.to_string())?;

    let index_path = daemons_root.join("index.md");
    let row = format!(
        "| `{name}.md` | `{daemon_uuid}` | {name} | {version} | daemons-contract v{contract_ver} | {context} | {caps_index} | {heartbeat} |"
    );
    append_row(&index_path, &row, &name)?;
    sync_daemons_index_census(&index_path)?;

    Ok(json!({
        "artifact_daemon_md": daemon_path.strip_prefix(repo).unwrap_or(&daemon_path).to_string_lossy().replace('\\', "/"),
        "artifact_daemons_index": index_path.strip_prefix(repo).unwrap_or(&index_path).to_string_lossy().replace('\\', "/"),
        "handoff_entity_uuid": daemon_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": null,
        "handoff_version": version,
    }))
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
            "daemon" => run_daemon_forge(repo, inputs),
            other => Err(format!("entity_class no soportada en forja nativa: {other}")),
        };
    }
    if inputs.get("daemon_name").is_some() {
        return run_daemon_forge(repo, inputs);
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

    fn fixture_cumulo(repo: &Path, domain_roots: &[&str]) {
        let core = repo.join("SddIA/core");
        fs::create_dir_all(&core).unwrap();
        let roots_json: Value = domain_roots.iter().map(|s| json!(s)).collect();
        let cfg = json!({
            "version": "1.6.0",
            "directories": {
                "process": "SddIA/process",
                "process_domain_roots": roots_json
            }
        });
        fs::write(
            core.join("cumulo.paths.json"),
            serde_json::to_string_pretty(&cfg).unwrap(),
        )
        .unwrap();
    }

    fn write_index(path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(
            path,
            "| Name | UUID | Versión | Context | Aliases | Descripción |\n|------|------|---------|---------|---------|-------------|\n",
        )
        .unwrap();
    }

    fn write_minimal_process(dir: &Path, name: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join(format!("{name}.md")),
            format!(
                "---\nname: \"{name}\"\nuuid: \"00000000-0000-4000-8000-000000000099\"\nversion: \"1.0.0\"\nphases: []\n---\n# {name}\n"
            ),
        )
        .unwrap();
    }

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

    #[test]
    fn process_forge_update_with_phases_adds_requires_capability_preserves_inputs() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fixture_cumulo(repo, &[]);
        let process_dir = repo.join("SddIA/process");
        fs::create_dir_all(&process_dir).unwrap();
        fs::write(
            repo.join("SddIA/process/index.md"),
            "| Name | UUID | Versión | Context | Aliases | Descripción |\n|------|------|---------|---------|---------|-------------|\n| di-patch-lab | aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee | 1.0.0 | ecosystem-evolution | — | lab |\n",
        )
        .unwrap();
        let md = process_dir.join("di-patch-lab.md");
        fs::write(
            &md,
            r#"---
uuid: "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee"
name: "di-patch-lab"
version: "1.0.0"
contract: "process-contract v1.4.0"
context: "ecosystem-evolution"
hash_signature: sha256:deadbeef
inputs:
  - "foo": "bar"
outputs:
  - "baz": "qux"
phases:
  - name: "Forja"
    intent: "escribir"
    delegates_to:
      - "skill:filesystem-manager"
  - name: "Indexación"
    intent: "índice"
    delegates_to:
      - "skill:filesystem-manager"
      - "agent:cumulo"
phase_invocations:
  - phase_name: "Forja"
    invocations: []
---

# di-patch-lab

cuerpo preservado
"#,
        )
        .unwrap();

        let phases = json!([
            {
                "name": "Forja",
                "intent": "escribir",
                "requires_capability": [{
                    "id": "fs:persist",
                    "contract": "fs.persist",
                    "version": ">=1.0.0"
                }],
                "delegates_to": ["action:crypto-broker"]
            },
            {
                "name": "Indexación",
                "intent": "índice",
                "requires_capability": [{
                    "id": "fs:persist",
                    "contract": "fs.persist",
                    "version": ">=1.0.0"
                }],
                "delegates_to": ["agent:cumulo"]
            }
        ]);
        let inputs = json!({
            "process_name": "di-patch-lab",
            "lifecycle_operation": "update",
            "process_phases": phases,
            "process_version": "1.0.1",
        });
        let out = run_process_forge(repo, &inputs).expect("update forge");
        assert_eq!(out["handoff_version"], json!("1.0.1"));
        assert_eq!(
            out["handoff_entity_uuid"],
            json!("aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee")
        );
        assert!(out["handoff_hash_signature_new"]
            .as_str()
            .unwrap_or("")
            .starts_with("sha256:"));
        assert_eq!(out["resolved_process_root"], json!("SddIA/process"));
        assert_eq!(out["process_jurisdiction_applied"], json!("core"));

        let body = fs::read_to_string(&md).unwrap();
        assert!(body.contains("requires_capability"));
        assert!(body.contains("fs:persist"));
        assert!(body.contains("cuerpo preservado"));
        assert!(body.contains("phase_invocations"));
        assert!(body.contains("inputs:"));
        assert!(body.contains("outputs:"));
        assert!(body.contains("foo"));
        assert!(!body.contains("skill:filesystem-manager"));
        assert!(body.contains("1.0.1"));

        let idx = fs::read_to_string(repo.join("SddIA/process/index.md")).unwrap();
        assert!(idx.contains("| di-patch-lab |") && idx.contains("1.0.1"));
    }

    #[test]
    fn process_forge_update_without_phases_remains_hash_only() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fixture_cumulo(repo, &[]);
        let process_dir = repo.join("SddIA/process");
        fs::create_dir_all(&process_dir).unwrap();
        let md = process_dir.join("hash-only-lab.md");
        fs::write(
            &md,
            r#"---
uuid: "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb"
name: "hash-only-lab"
version: "2.0.0"
contract: "process-contract v1.4.0"
context: "ecosystem-evolution"
hash_signature: sha256:1111111111111111111111111111111111111111111111111111111111111111
phases:
  - name: "A"
    intent: "x"
    delegates_to:
      - "agent:cumulo"
---

# hash-only-lab
"#,
        )
        .unwrap();
        let before = fs::read_to_string(&md).unwrap();
        let out = run_process_forge(
            repo,
            &json!({
                "process_name": "hash-only-lab",
                "lifecycle_operation": "update",
            }),
        )
        .expect("hash-only");
        assert_eq!(out["handoff_version"], json!("2.0.0"));
        let after = fs::read_to_string(&md).unwrap();
        assert!(after.contains("name: \"hash-only-lab\"") || after.contains("name: hash-only-lab"));
        assert!(after.contains("intent: \"x\"") || after.contains("intent: x"));
        // version unchanged
        assert!(after.contains("2.0.0"));
        assert_ne!(before, after); // hash_signature refreshed
    }

    #[test]
    fn ac_juris_domain_flag_writes_domain_root() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let domain = "SddIA/library/codexes/codex-software-engineering/process";
        fixture_cumulo(repo, &[domain]);
        write_index(&repo.join("SddIA/process/index.md"));
        write_index(&repo.join(domain).join("index.md"));
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "cccccccc-cccc-4ccc-8ccc-cccccccccccc");
        std::env::set_var("SDDIA_FORGE_LAB_SHA256", "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd");

        let out = run_process_forge(
            repo,
            &json!({
                "process_name": "lab-domain-juris",
                "process_jurisdiction": "domain",
                "process_description": "alta domain",
                "lifecycle_operation": "create",
            }),
        )
        .expect("domain forge");

        assert_eq!(out["process_jurisdiction_applied"], json!("domain"));
        assert_eq!(out["resolved_process_root"], json!(domain));
        assert!(repo.join(domain).join("lab-domain-juris.md").is_file());
        assert!(!repo.join("SddIA/process/lab-domain-juris.md").exists());
        let core_idx = fs::read_to_string(repo.join("SddIA/process/index.md")).unwrap();
        assert!(!core_idx.contains("lab-domain-juris"));
        let dom_idx = fs::read_to_string(repo.join(domain).join("index.md")).unwrap();
        assert!(dom_idx.contains("lab-domain-juris"));
        // AC-RESOLVE-COMPAT: post-alta domain
        let resolved = resolve_process_path(repo, "lab-domain-juris").unwrap();
        assert!(resolved.ends_with("lab-domain-juris.md"));
        assert!(resolved.to_string_lossy().contains("codex-software-engineering"));

        std::env::remove_var("SDDIA_FORGE_LAB_UUID");
        std::env::remove_var("SDDIA_FORGE_LAB_SHA256");
    }

    #[test]
    fn ac_juris_default_non_membership_writes_core() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let domain = "SddIA/library/codexes/codex-software-engineering/process";
        fixture_cumulo(repo, &[domain]);
        write_index(&repo.join("SddIA/process/index.md"));
        fs::create_dir_all(repo.join(domain)).unwrap();
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "dddddddd-dddd-4ddd-8ddd-dddddddddddd");
        std::env::set_var("SDDIA_FORGE_LAB_SHA256", "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");

        let out = run_process_forge(
            repo,
            &json!({
                "process_name": "lab-core-default",
                "process_description": "alta core",
                "lifecycle_operation": "create",
            }),
        )
        .expect("core forge");

        assert_eq!(out["process_jurisdiction_applied"], json!("core"));
        assert_eq!(out["resolved_process_root"], json!("SddIA/process"));
        assert!(repo.join("SddIA/process/lab-core-default.md").is_file());
        assert!(!repo.join(domain).join("lab-core-default.md").exists());

        std::env::remove_var("SDDIA_FORGE_LAB_UUID");
        std::env::remove_var("SDDIA_FORGE_LAB_SHA256");
    }

    #[test]
    fn ac_uniq_packing_name_blocks_core_create() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let domain = "SddIA/library/codexes/codex-software-engineering/process";
        fixture_cumulo(repo, &[domain]);
        write_minimal_process(&repo.join(domain), "feature");
        write_index(&repo.join("SddIA/process/index.md"));

        let err = run_process_forge(
            repo,
            &json!({
                "process_name": "feature",
                "process_jurisdiction": "core",
                "lifecycle_operation": "create",
            }),
        )
        .expect_err("must abort L-UNIQ-MULTI");
        assert!(err.contains("L-UNIQ-MULTI"), "got {err}");
        assert!(!repo.join("SddIA/process/feature.md").exists());
    }

    #[test]
    fn ev_aud_003_create_persists_requested_phases_not_stub() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fixture_cumulo(repo, &[]);
        write_index(&repo.join("SddIA/process/index.md"));
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa");
        std::env::remove_var("SDDIA_FORGE_LAB_SHA256");

        let phases = json!([
            {"name": "Validación de inputs", "intent": "comprobar contrato"},
            {"name": "Forja del archivo", "intent": "persistir fases"}
        ]);
        let out = run_process_forge(
            repo,
            &json!({
                "process_name": "lab-full-contract",
                "process_description": "EV-AUD-003",
                "lifecycle_operation": "create",
                "process_phases": phases,
                "workspace_template": ".SddIA/workspaces/{process_name}/{execution_id}/",
                "process_inputs": [{"process_name": "kebab"}],
                "process_outputs": [{"artifact_process_md": "path"}],
            }),
        )
        .expect("create full contract");

        let md = repo.join("SddIA/process/lab-full-contract.md");
        let body = fs::read_to_string(&md).unwrap();
        assert!(body.contains("Validación de inputs"), "got:\n{body}");
        assert!(body.contains("Forja del archivo"), "got:\n{body}");
        assert!(!body.contains("Fase inicial"), "stub leaked:\n{body}");
        assert!(body.contains("workspace_template:"));
        assert!(body.contains("inputs:"));
        assert!(body.contains("outputs:"));
        assert!(out["handoff_hash_signature_new"]
            .as_str()
            .unwrap_or("")
            .starts_with("sha256:"));
        assert_ne!(
            out["handoff_hash_signature_new"].as_str().unwrap_or(""),
            "sha256:pending"
        );
        crate::engine::verify_process_integrity::verify(repo)
            .unwrap_or_else(|e| panic!("integrity: {e:?}"));
        let idx = fs::read_to_string(repo.join("SddIA/process/index.md")).unwrap();
        assert!(idx.contains("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa"));
        assert!(idx.contains("lab-full-contract"));
        assert!(idx.contains("EV-AUD-003"));

        let again = run_process_forge(
            repo,
            &json!({
                "process_name": "lab-full-contract",
                "lifecycle_operation": "create",
                "process_phases": phases,
            }),
        )
        .expect("idempotent create");
        assert_eq!(again.get("idempotent"), Some(&json!(true)));

        std::env::remove_var("SDDIA_FORGE_LAB_UUID");
    }

    #[test]
    fn ev_aud_003_evolution_audit_fixture_recreates_without_stub() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fixture_cumulo(repo, &[]);
        write_index(&repo.join("SddIA/process/index.md"));
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "8f4b09da-e277-4fc2-9890-8a363fa8a96f");
        std::env::remove_var("SDDIA_FORGE_LAB_SHA256");

        let phases = json!([
            {
                "name": "Inventario normalizado",
                "intent": "Resolver evolution_root_ref vía Cúmulo",
                "delegates_to": ["agent:cumulo", "agent:argos"]
            },
            {
                "name": "Clasificación de relevancia",
                "intent": "Asignar R1-R5",
                "delegates_to": ["agent:argos"]
            },
            {
                "name": "Validación material",
                "intent": "Contrastar evidencia vigente",
                "delegates_to": ["agent:argos"]
            },
            {
                "name": "Persistencia oficial",
                "intent": "Persistir informe",
                "requires_capability": [{
                    "id": "fs:persist",
                    "contract": "fs.persist",
                    "version": ">=1.0.0"
                }]
            },
            {
                "name": "Protocolo de Acero",
                "intent": "Revisar contradicciones",
                "delegates_to": ["agent:argos"]
            }
        ]);
        let out = run_process_forge(
            repo,
            &json!({
                "process_name": "evolution-audit",
                "process_description": "Auditoría periódica del registro evolution",
                "process_context": ["quality-assurance", "filesystem-ops"],
                "process_version": "1.0.0",
                "process_contract_version": "1.4.0",
                "lifecycle_operation": "create",
                "workspace_template": ".SddIA/workspaces/{process_name}/{execution_id}/",
                "process_inputs": [
                    {"audit_date": "Fecha ISO de corte de la auditoría"},
                    {"mode": "Enum estricto: full | since_last"}
                ],
                "process_outputs": [
                    {"audit_report_path": "Informe oficial versionado"}
                ],
                "process_phases": phases,
            }),
        )
        .expect("evolution-audit fixture");

        let body = fs::read_to_string(repo.join("SddIA/process/evolution-audit.md")).unwrap();
        assert!(body.contains("Inventario normalizado"));
        assert!(body.contains("Protocolo de Acero"));
        assert!(body.contains("fs:persist"));
        assert!(body.contains("quality-assurance"));
        assert!(!body.contains("Fase inicial"));
        crate::engine::verify_process_integrity::verify(repo)
            .unwrap_or_else(|e| panic!("evolution-audit integrity: {e:?}"));
        let idx = fs::read_to_string(repo.join("SddIA/process/index.md")).unwrap();
        assert!(idx.contains("8f4b09da-e277-4fc2-9890-8a363fa8a96f"));
        assert!(idx.contains("quality-assurance, filesystem-ops"));
        assert_eq!(
            out["handoff_entity_uuid"],
            json!("8f4b09da-e277-4fc2-9890-8a363fa8a96f")
        );

        std::env::remove_var("SDDIA_FORGE_LAB_UUID");
    }

    #[test]
    fn ev_aud_003_create_fail_closed_deletes_artifact_if_index_write_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fixture_cumulo(repo, &[]);
        write_index(&repo.join("SddIA/process/index.md"));
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb");
        std::env::remove_var("SDDIA_FORGE_LAB_SHA256");

        let idx = repo.join("SddIA/process/index.md");
        let mut perms = fs::metadata(&idx).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&idx, perms).unwrap();

        let err = run_process_forge(
            repo,
            &json!({
                "process_name": "lab-fail-closed",
                "process_description": "EV-AUD-003 fail-closed",
                "lifecycle_operation": "create",
                "process_phases": [{"name": "Solo", "intent": "borrar si índice falla"}],
            }),
        )
        .expect_err("index write must fail");
        assert!(!err.is_empty(), "got {err}");
        assert!(
            !repo.join("SddIA/process/lab-fail-closed.md").exists(),
            "artefacto huérfano"
        );

        let mut restore = fs::metadata(&idx).unwrap().permissions();
        restore.set_readonly(false);
        let _ = fs::set_permissions(&idx, restore);
        std::env::remove_var("SDDIA_FORGE_LAB_UUID");
    }

    #[test]
    fn ac_uniq_alias_cross_root_aborts() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let domain = "SddIA/library/codexes/codex-software-engineering/process";
        fixture_cumulo(repo, &[domain]);
        fs::create_dir_all(repo.join(domain)).unwrap();
        fs::write(
            repo.join(domain).join("packed-alias.md"),
            "---\nname: \"packed-alias\"\naliases:\n  - \"ghost-alias\"\nuuid: \"00000000-0000-4000-8000-000000000077\"\nversion: \"1.0.0\"\nphases: []\n---\n# packed-alias\n",
        )
        .unwrap();
        write_index(&repo.join("SddIA/process/index.md"));

        let err = run_process_forge(
            repo,
            &json!({
                "process_name": "ghost-alias",
                "lifecycle_operation": "create",
            }),
        )
        .expect_err("alias cross-root");
        assert!(err.contains("L-UNIQ-MULTI"), "got {err}");
    }

    #[test]
    fn ac_smoke_domain_no_core_executable() {
        // AC-SMOKE: alta domain no deja artefacto bajo SddIA/process/
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let domain = "SddIA/library/codexes/codex-software-engineering/process";
        fixture_cumulo(repo, &[domain]);
        write_index(&repo.join("SddIA/process/index.md"));
        write_index(&repo.join(domain).join("index.md"));
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee");
        std::env::set_var("SDDIA_FORGE_LAB_SHA256", "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

        run_process_forge(
            repo,
            &json!({
                "process_name": "smoke-software-lab",
                "process_jurisdiction": "domain",
                "lifecycle_operation": "create",
            }),
        )
        .expect("smoke");

        assert!(!repo.join("SddIA/process/smoke-software-lab.md").exists());
        assert!(repo.join(domain).join("smoke-software-lab.md").is_file());
        let core_idx = fs::read_to_string(repo.join("SddIA/process/index.md")).unwrap();
        assert!(!core_idx.contains("smoke-software-lab"));

        std::env::remove_var("SDDIA_FORGE_LAB_UUID");
        std::env::remove_var("SDDIA_FORGE_LAB_SHA256");
    }

    #[test]
    fn run_norm_forge_emits_dependencies_and_hard_constraints() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        fixture_cumulo(repo, &[]);
        let norms_dir = repo.join("SddIA/library/norms");
        fs::create_dir_all(&norms_dir).unwrap();
        fs::write(
            norms_dir.join("index.md"),
            "| Archivo fuente | uuid | name | version | scope | category |\n|----------------|------|------|---------|-------|----------|\n",
        )
        .unwrap();
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "f0b8ce4a-2f79-4516-bee0-acfe0d25bd58");
        std::env::set_var(
            "SDDIA_FORGE_LAB_SHA256",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        );

        run_norm_forge(
            repo,
            &json!({
                "tactical_norm_name": "lab-norm",
                "tactical_norm_version": "1.0.0",
                "tactical_norm_friction": "Directriz de prueba.",
                "tactical_norm_hard_constraints": "- Prohibido X.",
                "tactical_norm_dependencies": ["4c448c82-de41-460f-b24f-82a84fa5ed69"],
                "lifecycle_operation": "create",
            }),
        )
        .expect("norm forge");

        let body = fs::read_to_string(norms_dir.join("lab-norm.md")).unwrap();
        assert!(body.contains("dependencies:"));
        assert!(body.contains("4c448c82-de41-460f-b24f-82a84fa5ed69"));
        assert!(body.contains("## Directriz Core"));
        assert!(body.contains("## Restricciones Duras (Aduana de Fricción)"));
        assert!(body.contains("- Prohibido X."));

        std::env::remove_var("SDDIA_FORGE_LAB_UUID");
        std::env::remove_var("SDDIA_FORGE_LAB_SHA256");
    }
}
