//! Aduana Temprana — validación DI por capacidades pre-ignición (PBI-042 MVP).

use crate::core::parser::parse_frontmatter;
use super::eda_bus_topology::load_event_bus_topology;
use super::workspace::load_paths_config;
use chrono::Utc;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const SKIP_ENV: &str = "SDDIA_LAB_SKIP_CAPABILITY_DI";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiGateCode {
    CapabilityNotIndexed,
    CapabilityProviderMismatch,
    ContractSchemaMismatch,
}

impl DiGateCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CapabilityNotIndexed => "CAPABILITY_NOT_INDEXED",
            Self::CapabilityProviderMismatch => "CAPABILITY_PROVIDER_MISMATCH",
            Self::ContractSchemaMismatch => "CONTRACT_SCHEMA_MISMATCH",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiRequirement {
    pub id: String,
    pub contract: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct DiGateError {
    pub code: DiGateCode,
    pub message: String,
}

fn env_skip() -> bool {
    std::env::var(SKIP_ENV)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn yaml_str_map(v: &YamlValue, key: &str) -> Option<String> {
    match v {
        YamlValue::Mapping(m) => m
            .get(YamlValue::String(key.into()))
            .and_then(|x| x.as_str())
            .map(str::to_string),
        _ => None,
    }
}

fn parse_requirements(phase: &Value) -> Vec<DiRequirement> {
    let Some(arr) = phase.get("requires_capability").and_then(|v| v.as_array()) else {
        return vec![];
    };
    let mut out = Vec::new();
    for item in arr {
        let id = item
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        if id.is_empty() {
            continue;
        }
        let contract = item
            .get("contract")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let version = item
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0.0")
            .trim()
            .to_string();
        out.push(DiRequirement {
            id,
            contract,
            version,
        });
    }
    out
}

fn taxonomy_path(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_paths_config(repo)?;
    if let Some(rel) = cfg
        .get("normative_documents")
        .and_then(|n| n.get("capability_taxonomy"))
        .and_then(|v| v.as_str())
    {
        return Ok(repo.join(rel.trim().trim_start_matches("./")));
    }
    let norms = cfg
        .get("directories")
        .and_then(|d| d.get("library_norms"))
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/library/norms/");
    Ok(repo.join(norms.trim().trim_end_matches('/')).join("capability-taxonomy.md"))
}

fn contracts_dir(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_paths_config(repo)?;
    if let Some(rel) = cfg
        .get("directories")
        .and_then(|d| d.get("capability_contracts"))
        .and_then(|v| v.as_str())
    {
        return Ok(repo.join(rel.trim().trim_end_matches('/')));
    }
    Ok(repo.join("SddIA/library/norms/capability-contracts"))
}

fn load_catalog_ids(repo: &Path) -> Result<HashSet<String>, String> {
    let path = taxonomy_path(repo)?;
    if !path.is_file() {
        return Err(format!("capability-taxonomy inaccesible: {}", path.display()));
    }
    let fm = parse_frontmatter(&path)?;
    let mut ids = HashSet::new();
    if let Some(YamlValue::Sequence(seq)) = fm.get("catalog") {
        for item in seq {
            if let Some(id) = yaml_str_map(item, "id") {
                ids.insert(id);
            }
        }
    }
    Ok(ids)
}

fn resolve_capsule_md(repo: &Path, delegate: &str) -> Option<PathBuf> {
    let (kind, name) = delegate.split_once(':')?;
    let rel = match kind {
        "skill" => format!("SddIA/skills/{name}.md"),
        "action" => format!("SddIA/actions/{name}.md"),
        "tool" => format!("SddIA/tools/{name}.md"),
        "agent" => format!("SddIA/agents/{name}.md"),
        _ => return None,
    };
    let p = repo.join(rel);
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}

fn provider_provides(path: &Path) -> Result<Vec<DiRequirement>, String> {
    let fm = parse_frontmatter(path)?;
    let mut out = Vec::new();
    let Some(YamlValue::Sequence(seq)) = fm.get("provides") else {
        return Ok(out);
    };
    for item in seq {
        let id = yaml_str_map(item, "id").unwrap_or_default();
        if id.is_empty() {
            continue;
        }
        out.push(DiRequirement {
            id,
            contract: yaml_str_map(item, "contract").unwrap_or_default(),
            version: yaml_str_map(item, "version").unwrap_or_else(|| "1.0.0".into()),
        });
    }
    Ok(out)
}

fn output_keys(path: &Path) -> Result<HashSet<String>, String> {
    let fm = parse_frontmatter(path)?;
    let mut keys = HashSet::new();
    let Some(YamlValue::Sequence(seq)) = fm.get("outputs") else {
        return Ok(keys);
    };
    for item in seq {
        match item {
            YamlValue::Mapping(m) => {
                for k in m.keys() {
                    if let Some(s) = k.as_str() {
                        keys.insert(s.to_string());
                    }
                }
            }
            YamlValue::String(s) => {
                // Formato legado: "exitCode": "desc" como string completo
                if let Some((k, _)) = s.split_once(':') {
                    let k = k.trim().trim_matches('"').trim();
                    if !k.is_empty() {
                        keys.insert(k.to_string());
                    }
                }
            }
            _ => {}
        }
    }
    Ok(keys)
}

fn version_compatible(provided: &str, required: &str) -> bool {
    let req = required.trim();
    let prov = provided.trim();
    if let Some(min) = req.strip_prefix(">=") {
        return version_gte(prov, min.trim());
    }
    prov == req || prov.starts_with(req.trim_start_matches('='))
}

fn parse_semver_triple(s: &str) -> Option<(u64, u64, u64)> {
    let mut parts = s.trim().split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().unwrap_or(0);
    let patch = parts
        .next()
        .unwrap_or("0")
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);
    Some((major, minor, patch))
}

fn version_gte(a: &str, b: &str) -> bool {
    match (parse_semver_triple(a), parse_semver_triple(b)) {
        (Some(x), Some(y)) => x >= y,
        _ => a >= b,
    }
}

fn schema_required_keys(schema_path: &Path) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(schema_path).map_err(|e| e.to_string())?;
    let v: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let mut keys = Vec::new();
    if let Some(arr) = v.get("required").and_then(|x| x.as_array()) {
        for item in arr {
            if let Some(s) = item.as_str() {
                keys.push(s.to_string());
            }
        }
    }
    Ok(keys)
}

pub fn write_di_dead_letter(repo: &Path, err: &DiGateError, phase_name: &str, process_name: &str) {
    let Ok(topo) = load_event_bus_topology(repo) else {
        return;
    };
    let dir = repo.join(&topo.dead_letter);
    let _ = fs::create_dir_all(&dir);
    let id = Uuid::new_v4();
    let path = dir.join(format!("{id}.capability-di-gate.json"));
    let payload = json!({
        "event_id": id.to_string(),
        "event_type": "Capability_Di_Gate_Failed",
        "state": "dead-letter",
        "code": err.code.as_str(),
        "message": err.message,
        "process_name": process_name,
        "phase_name": phase_name,
        "emitted_at": Utc::now().to_rfc3339(),
        "emitter": "capability_di_gate",
    });
    let _ = fs::write(path, serde_json::to_string_pretty(&payload).unwrap_or_default());
}

/// Valida `requires_capability` de la fase antes de ignición.
/// `Ok(())` si no hay requisitos, skip lab, o validación APTO.
pub fn validate_phase_capability_di(
    repo: &Path,
    phase: &Value,
    process_name: &str,
) -> Result<(), DiGateError> {
    if env_skip() {
        return Ok(());
    }
    let reqs = parse_requirements(phase);
    if reqs.is_empty() {
        return Ok(());
    }

    let catalog = load_catalog_ids(repo).map_err(|e| DiGateError {
        code: DiGateCode::CapabilityNotIndexed,
        message: e,
    })?;

    let delegates = phase
        .get("delegates_to")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let contracts = contracts_dir(repo).map_err(|e| DiGateError {
        code: DiGateCode::ContractSchemaMismatch,
        message: e,
    })?;

    for req in &reqs {
        if !catalog.contains(&req.id) {
            return Err(DiGateError {
                code: DiGateCode::CapabilityNotIndexed,
                message: format!(
                    "capacidad '{}' no pertenece a capability-taxonomy (process={process_name})",
                    req.id
                ),
            });
        }

        let mut matched_provider: Option<PathBuf> = None;
        for d in &delegates {
            let Some(del) = d.as_str() else { continue };
            // Solo action|skill en MVP
            if !(del.starts_with("skill:") || del.starts_with("action:")) {
                continue;
            }
            let Some(path) = resolve_capsule_md(repo, del) else {
                continue;
            };
            let provides = provider_provides(&path).unwrap_or_default();
            if let Some(prov) = provides.iter().find(|p| p.id == req.id) {
                if !req.contract.is_empty() && prov.contract != req.contract {
                    return Err(DiGateError {
                        code: DiGateCode::CapabilityProviderMismatch,
                        message: format!(
                            "contrato proveedor '{}' != requerido '{}' (capsule={del})",
                            prov.contract, req.contract
                        ),
                    });
                }
                if !version_compatible(&prov.version, &req.version) {
                    return Err(DiGateError {
                        code: DiGateCode::CapabilityProviderMismatch,
                        message: format!(
                            "versión proveedor '{}' no satisface '{}' (capsule={del})",
                            prov.version, req.version
                        ),
                    });
                }
                matched_provider = Some(path);
                break;
            }
        }

        let Some(provider_path) = matched_provider else {
            return Err(DiGateError {
                code: DiGateCode::CapabilityProviderMismatch,
                message: format!(
                    "ningún delegates_to declara provides id='{}' (process={process_name})",
                    req.id
                ),
            });
        };

        let contract_name = if req.contract.is_empty() {
            req.id.replace(':', ".")
        } else {
            req.contract.clone()
        };
        let schema_path = contracts.join(format!("{contract_name}.schema.json"));
        if !schema_path.is_file() {
            return Err(DiGateError {
                code: DiGateCode::ContractSchemaMismatch,
                message: format!("schema ausente: {}", schema_path.display()),
            });
        }
        let required = schema_required_keys(&schema_path).map_err(|e| DiGateError {
            code: DiGateCode::ContractSchemaMismatch,
            message: e,
        })?;
        let outs = output_keys(&provider_path).map_err(|e| DiGateError {
            code: DiGateCode::ContractSchemaMismatch,
            message: e,
        })?;
        for key in &required {
            if !outs.contains(key) {
                return Err(DiGateError {
                    code: DiGateCode::ContractSchemaMismatch,
                    message: format!(
                        "proveedor {} omite output obligatorio '{}' del contrato {}",
                        provider_path.display(),
                        key,
                        contract_name
                    ),
                });
            }
        }
    }

    let _ = process_name;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;

    fn write_md(dir: &Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut f = fs::File::create(p).unwrap();
        f.write_all(body.as_bytes()).unwrap();
    }

    fn fixture_repo() -> tempfile::TempDir {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_md(
            root,
            "SddIA/core/cumulo.paths.json",
            r#"{
  "version": "1.5.3",
  "directories": {
    "library_norms": "SddIA/library/norms/",
    "capability_contracts": "SddIA/library/norms/capability-contracts/"
  },
  "capability_di": {
    "bindings": "SddIA/core/capability-bindings.md"
  },
  "normative_documents": {
    "capability_taxonomy": "SddIA/library/norms/capability-taxonomy.md"
  },
  "event_bus": "./.events",
  "eda_bus": {
    "dead_letter": "./.events/dead-letter"
  }
}"#,
        );
        write_md(
            root,
            "SddIA/core/capability-bindings.md",
            r#"---
name: capability-bindings
bindings:
  - capability_id: "doc:closure"
    contract: "doc.closure"
    provider: "skill:filesystem-manager"
---
"#,
        );
        write_md(
            root,
            "SddIA/library/norms/capability-taxonomy.md",
            r#"---
name: capability-taxonomy
catalog:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
---
"#,
        );
        write_md(
            root,
            "SddIA/library/norms/capability-contracts/doc.closure.schema.json",
            r#"{
  "type": "object",
  "required": ["exitCode", "data"],
  "properties": {
    "exitCode": {"type": "integer"},
    "data": {"type": "object"}
  }
}"#,
        );
        write_md(
            root,
            "SddIA/skills/filesystem-manager.md",
            r#"---
name: filesystem-manager
provides:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
outputs:
  - exitCode: "0|1"
  - data: "payload"
---
"#,
        );
        td
    }

    #[test]
    fn ac_p1_ok() {
        let td = fixture_repo();
        let phase = json!({
            "name": "Cierre documental en rama",
            "requires_capability": [{
                "id": "doc:closure",
                "contract": "doc.closure",
                "version": ">=1.0.0"
            }],
            "delegates_to": ["skill:filesystem-manager"]
        });
        assert!(validate_phase_capability_di(td.path(), &phase, "feature").is_ok());
    }

    #[test]
    fn ac_p3_not_indexed() {
        let td = fixture_repo();
        let phase = json!({
            "name": "X",
            "requires_capability": [{
                "id": "documentos:cerrar_archivo",
                "contract": "doc.closure",
                "version": "1.0.0"
            }],
            "delegates_to": ["skill:filesystem-manager"]
        });
        let err = validate_phase_capability_di(td.path(), &phase, "feature").unwrap_err();
        assert_eq!(err.code, DiGateCode::CapabilityNotIndexed);
    }

    #[test]
    fn ac_p2_schema_mismatch() {
        let td = fixture_repo();
        write_md(
            td.path(),
            "SddIA/skills/filesystem-manager.md",
            r#"---
name: filesystem-manager
provides:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
outputs:
  - exitCode: "0|1"
---
"#,
        );
        let phase = json!({
            "name": "Cierre documental en rama",
            "requires_capability": [{
                "id": "doc:closure",
                "contract": "doc.closure",
                "version": "1.0.0"
            }],
            "delegates_to": ["skill:filesystem-manager"]
        });
        let err = validate_phase_capability_di(td.path(), &phase, "feature").unwrap_err();
        assert_eq!(err.code, DiGateCode::ContractSchemaMismatch);
    }

    #[test]
    fn provider_mismatch_without_provides() {
        let td = fixture_repo();
        write_md(
            td.path(),
            "SddIA/skills/filesystem-manager.md",
            r#"---
name: filesystem-manager
outputs:
  - exitCode: "0"
  - data: "x"
---
"#,
        );
        let phase = json!({
            "name": "Cierre documental en rama",
            "requires_capability": [{
                "id": "doc:closure",
                "contract": "doc.closure",
                "version": "1.0.0"
            }],
            "delegates_to": ["skill:filesystem-manager"]
        });
        let err = validate_phase_capability_di(td.path(), &phase, "feature").unwrap_err();
        assert_eq!(err.code, DiGateCode::CapabilityProviderMismatch);
    }

    #[test]
    fn ac_p1_real_repo_feature_phase_blind_via_resolver() {
        let root = crate::core::repo::find_repo_root().expect("repo root");
        let phase = json!({
            "name": "Cierre documental en rama",
            "requires_capability": [{
                "id": "doc:closure",
                "contract": "doc.closure",
                "version": ">=1.0.0"
            }]
        });
        let bindings = crate::engine::capability_di_resolver::resolve_phase_bindings(&root, &phase)
            .expect("resolve");
        let effective =
            crate::engine::capability_di_resolver::phase_with_effective_delegates(&phase, &bindings);
        validate_phase_capability_di(&root, &effective, "feature").expect("gate after resolve");
    }

    #[test]
    fn ac_p1_real_repo_feature_phase() {
        let root = crate::core::repo::find_repo_root().expect("repo root");
        let phase = json!({
            "name": "Cierre documental en rama",
            "requires_capability": [{
                "id": "doc:closure",
                "contract": "doc.closure",
                "version": ">=1.0.0"
            }],
            "delegates_to": ["skill:filesystem-manager"]
        });
        validate_phase_capability_di(&root, &phase, "feature").expect("real repo DI gate");
    }
}
