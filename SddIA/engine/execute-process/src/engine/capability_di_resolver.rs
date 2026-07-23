//! Resolución ciega capability → proveedor (PBI-042 Hito 2).
//!
//! Orden canónico: resolve → fase efectiva → capability_di_gate → inject.

use crate::core::parser::parse_frontmatter;
use super::eda_bus_topology::load_event_bus_topology;
use super::workspace::load_paths_config;
use chrono::Utc;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const SKIP_ENV: &str = "SDDIA_LAB_SKIP_CAPABILITY_DI";
const BINDING_SSOT_KEY: &str = "capability_di.bindings";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiResolveCode {
    BindingMissing,
    ProviderAmbiguous,
    ProviderMismatch,
    ConfigError,
}

impl DiResolveCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BindingMissing => "CAPABILITY_BINDING_MISSING",
            Self::ProviderAmbiguous => "CAPABILITY_PROVIDER_AMBIGUOUS",
            Self::ProviderMismatch => "CAPABILITY_PROVIDER_MISMATCH",
            Self::ConfigError => "CAPABILITY_DI_CONFIG_ERROR",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiResolveError {
    pub code: DiResolveCode,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedBinding {
    pub capability_id: String,
    pub contract: String,
    pub provider: String,
    pub provider_md_rel: String,
    pub contract_schema_rel: String,
    pub version: String,
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

fn bindings_path(repo: &Path) -> Result<PathBuf, DiResolveError> {
    let cfg = load_paths_config(repo).map_err(|e| DiResolveError {
        code: DiResolveCode::ConfigError,
        message: e,
    })?;
    let rel = cfg
        .get("capability_di")
        .and_then(|c| c.get("bindings"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| DiResolveError {
            code: DiResolveCode::ConfigError,
            message: "capability_di.bindings ausente en Cúmulo".into(),
        })?;
    Ok(repo.join(rel.trim().trim_start_matches("./")))
}

fn contracts_dir_rel(repo: &Path) -> Result<String, DiResolveError> {
    let cfg = load_paths_config(repo).map_err(|e| DiResolveError {
        code: DiResolveCode::ConfigError,
        message: e,
    })?;
    Ok(cfg
        .get("directories")
        .and_then(|d| d.get("capability_contracts"))
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/library/norms/capability-contracts/")
        .trim()
        .trim_end_matches('/')
        .to_string())
}

fn is_capsule_provider(delegate: &str) -> bool {
    delegate.starts_with("skill:")
        || delegate.starts_with("action:")
        || delegate.starts_with("tool:")
}

fn provider_md_rel(provider: &str) -> Option<String> {
    let (kind, name) = provider.split_once(':')?;
    match kind {
        "skill" => Some(format!("directories.skills/{name}.md")),
        "action" => Some(format!("directories.actions/{name}.md")),
        "tool" => Some(format!("directories.tools/{name}.md")),
        _ => None,
    }
}

fn provider_fs_rel(provider: &str) -> Option<String> {
    let (kind, name) = provider.split_once(':')?;
    match kind {
        "skill" => Some(format!("SddIA/skills/{name}.md")),
        "action" => Some(format!("SddIA/actions/{name}.md")),
        "tool" => Some(format!("SddIA/tools/{name}.md")),
        _ => None,
    }
}

fn load_binding_rows(repo: &Path) -> Result<HashMap<String, (String, String, Option<String>)>, DiResolveError> {
    let path = bindings_path(repo)?;
    if !path.is_file() {
        return Err(DiResolveError {
            code: DiResolveCode::ConfigError,
            message: format!("capability-bindings inaccesible: {}", path.display()),
        });
    }
    let fm = parse_frontmatter(&path).map_err(|e| DiResolveError {
        code: DiResolveCode::ConfigError,
        message: e,
    })?;
    let mut map = HashMap::new();
    let Some(YamlValue::Sequence(seq)) = fm.get("bindings") else {
        return Ok(map);
    };
    for item in seq {
        let cap = yaml_str_map(item, "capability_id").unwrap_or_default();
        let contract = yaml_str_map(item, "contract").unwrap_or_default();
        let provider = yaml_str_map(item, "provider").unwrap_or_default();
        let provider_version = yaml_str_map(item, "provider_version");
        if cap.is_empty() || provider.is_empty() {
            continue;
        }
        if map.contains_key(&cap) {
            return Err(DiResolveError {
                code: DiResolveCode::ProviderAmbiguous,
                message: format!("fila duplicada en binding table para capability_id='{cap}'"),
            });
        }
        map.insert(cap, (contract, provider, provider_version));
    }
    Ok(map)
}

fn parse_phase_requirements(phase: &Value) -> Vec<(String, String, String)> {
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
        out.push((id, contract, version));
    }
    out
}

fn provider_provides_id(path: &Path, capability_id: &str) -> Option<(String, String)> {
    let fm = parse_frontmatter(path).ok()?;
    let YamlValue::Sequence(seq) = fm.get("provides")? else {
        return None;
    };
    for item in seq {
        let id = yaml_str_map(item, "id").unwrap_or_default();
        if id == capability_id {
            return Some((
                yaml_str_map(item, "contract").unwrap_or_default(),
                yaml_str_map(item, "version").unwrap_or_else(|| "1.0.0".into()),
            ));
        }
    }
    None
}

fn scan_catalog_providers(repo: &Path, capability_id: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (kind, dir) in [
        ("skill", "SddIA/skills"),
        ("action", "SddIA/actions"),
        ("tool", "SddIA/tools"),
    ] {
        let root = repo.join(dir);
        let Ok(entries) = fs::read_dir(&root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if stem.ends_with("-contract") || stem == "index" {
                continue;
            }
            if provider_provides_id(&path, capability_id).is_some() {
                found.push(format!("{kind}:{stem}"));
            }
        }
    }
    found.sort();
    found.dedup();
    found
}

pub fn write_resolve_dead_letter(
    repo: &Path,
    err: &DiResolveError,
    phase_name: &str,
    process_name: &str,
) {
    let Ok(topo) = load_event_bus_topology(repo) else {
        return;
    };
    let dir = repo.join(&topo.dead_letter);
    let _ = fs::create_dir_all(&dir);
    let id = Uuid::new_v4();
    let path = dir.join(format!("{id}.capability-di-resolve.json"));
    let payload = json!({
        "event_id": id.to_string(),
        "event_type": "Capability_Di_Resolve_Failed",
        "state": "dead-letter",
        "code": err.code.as_str(),
        "message": err.message,
        "process_name": process_name,
        "phase_name": phase_name,
        "emitted_at": Utc::now().to_rfc3339(),
        "emitter": "capability_di_resolver",
    });
    let _ = fs::write(path, serde_json::to_string_pretty(&payload).unwrap_or_default());
}

/// Resuelve bindings de la fase. `Ok(vec![])` si no hay requires o skip lab.
pub fn resolve_phase_bindings(
    repo: &Path,
    phase: &Value,
) -> Result<Vec<ResolvedBinding>, DiResolveError> {
    if env_skip() {
        return Ok(vec![]);
    }
    let reqs = parse_phase_requirements(phase);
    if reqs.is_empty() {
        return Ok(vec![]);
    }

    let rows = load_binding_rows(repo)?;
    let contracts_rel = contracts_dir_rel(repo)?;
    let delegates: Vec<String> = phase
        .get("delegates_to")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    let mut out = Vec::new();
    for (cap_id, req_contract, _req_version) in reqs {
        let Some((row_contract, provider, _pv)) = rows.get(&cap_id) else {
            return Err(DiResolveError {
                code: DiResolveCode::BindingMissing,
                message: format!("sin fila en binding table para capability_id='{cap_id}'"),
            });
        };

        let catalog = scan_catalog_providers(repo, &cap_id);
        // Q2: fila canónica es SSOT; ambigüedad solo si el catálogo diverge de la fila.
        if catalog.len() > 1 && !catalog.contains(provider) {
            return Err(DiResolveError {
                code: DiResolveCode::ProviderAmbiguous,
                message: format!(
                    "proveedores provides('{cap_id}')={catalog:?} divergen de fila canónica '{provider}'"
                ),
            });
        }

        // H9: preferir skill|action|tool en delegates_to que ya declare provides(cap);
        // si no hay, usar fila binding (path ciego → inyección posterior).
        let mut resolved_provider = provider.clone();
        for d in &delegates {
            if !is_capsule_provider(d) {
                continue;
            }
            let Some(fs) = provider_fs_rel(d) else {
                continue;
            };
            let path = repo.join(&fs);
            if provider_provides_id(&path, &cap_id).is_some() {
                resolved_provider = d.clone();
                break;
            }
        }

        let fs_rel = provider_fs_rel(&resolved_provider).ok_or_else(|| DiResolveError {
            code: DiResolveCode::ConfigError,
            message: format!("provider no resoluble: {resolved_provider}"),
        })?;
        let provider_path = repo.join(&fs_rel);
        let (prov_contract, prov_version) = provider_provides_id(&provider_path, &cap_id).ok_or_else(
            || DiResolveError {
                code: DiResolveCode::ProviderMismatch,
                message: format!("provider '{resolved_provider}' no declara provides id='{cap_id}'"),
            },
        )?;

        let contract = if !req_contract.is_empty() {
            req_contract.clone()
        } else if !row_contract.is_empty() {
            row_contract.clone()
        } else if !prov_contract.is_empty() {
            prov_contract
        } else {
            cap_id.replace(':', ".")
        };

        let logical_ref =
            provider_md_rel(&resolved_provider).unwrap_or_else(|| fs_rel.clone());
        let schema_rel = format!("{contracts_rel}/{contract}.schema.json");

        out.push(ResolvedBinding {
            capability_id: cap_id,
            contract,
            provider: resolved_provider,
            provider_md_rel: logical_ref,
            contract_schema_rel: schema_rel,
            version: prov_version,
        });
    }
    Ok(out)
}

/// Sintetiza fase de trabajo: si path ciego, inyecta `delegates_to` efectivo = providers resueltos.
pub fn phase_with_effective_delegates(phase: &Value, bindings: &[ResolvedBinding]) -> Value {
    let mut effective = phase.clone();
    if bindings.is_empty() {
        return effective;
    }
    let existing = phase
        .get("delegates_to")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    // H9: skill|action|tool ya anclado ⇒ no inyectar; solo agentes ⇒ path ciego + inject.
    let has_capsule = existing.iter().any(|d| {
        d.as_str()
            .map(is_capsule_provider)
            .unwrap_or(false)
    });
    if has_capsule {
        return effective;
    }
    let mut dels = existing;
    for b in bindings {
        let p = json!(b.provider);
        if !dels.contains(&p) {
            dels.push(p);
        }
    }
    if let Some(obj) = effective.as_object_mut() {
        obj.insert("delegates_to".into(), Value::Array(dels));
        obj.insert(
            "resolved_provider".into(),
            json!(bindings.iter().map(|b| &b.provider).collect::<Vec<_>>()),
        );
    }
    effective
}

/// Objeto `di_binding` para envelope capsule-json-io v2 (R2).
pub fn di_binding_object(binding: &ResolvedBinding) -> Value {
    json!({
        "capability_id": binding.capability_id,
        "contract": binding.contract,
        "contract_schema_ref": format!("capability_contracts/{}", binding.contract),
        "provider": binding.provider,
        "provider_ref": binding.provider_md_rel,
        "resolved_version": binding.version,
        "binding_ssot": BINDING_SSOT_KEY,
    })
}

/// Merge `di_binding` en payload de invocación (hermano de `request` si existe envelope).
pub fn merge_di_binding_into_payload(payload: &Value, binding: &ResolvedBinding) -> Value {
    let di = di_binding_object(binding);
    let mut out = payload.clone();
    if let Some(obj) = out.as_object_mut() {
        obj.insert("di_binding".into(), di);
        return out;
    }
    json!({
        "di_binding": di,
        "request": payload,
    })
}

/// Si hay varios bindings, usa el primero (piloto single-capability por invocación).
pub fn merge_first_di_binding(payload: &Value, bindings: &[ResolvedBinding]) -> Value {
    match bindings.first() {
        Some(b) => merge_di_binding_into_payload(payload, b),
        None => payload.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
  }
}"#,
        );
        write_md(
            root,
            "SddIA/core/capability-bindings.md",
            r#"---
uuid: "c4a8f2e1-7b3d-4e9a-a1c6-5d8f0b2e4a71"
name: capability-bindings
version: "1.0.0"
bindings:
  - capability_id: "doc:closure"
    contract: "doc.closure"
    provider: "skill:filesystem-manager"
    provider_version: ">=1.0.0"
---
"#,
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
    fn resolve_blind_ok() {
        let td = fixture_repo();
        let phase = json!({
            "name": "Cierre documental en rama",
            "requires_capability": [{
                "id": "doc:closure",
                "contract": "doc.closure",
                "version": ">=1.0.0"
            }]
        });
        let bindings = resolve_phase_bindings(td.path(), &phase).expect("resolve");
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].provider, "skill:filesystem-manager");
        let effective = phase_with_effective_delegates(&phase, &bindings);
        assert_eq!(
            effective["delegates_to"],
            json!(["skill:filesystem-manager"])
        );
    }

    #[test]
    fn resolve_missing_binding() {
        let td = fixture_repo();
        write_md(
            td.path(),
            "SddIA/core/capability-bindings.md",
            r#"---
name: capability-bindings
bindings: []
---
"#,
        );
        let phase = json!({
            "requires_capability": [{"id": "doc:closure", "contract": "doc.closure"}]
        });
        let err = resolve_phase_bindings(td.path(), &phase).unwrap_err();
        assert_eq!(err.code, DiResolveCode::BindingMissing);
    }

    #[test]
    fn resolve_prefers_delegate_provider_over_binding_row() {
        // H9: si delegates_to declara provides(cap), prevalece sobre fila canónica.
        let td = fixture_repo();
        write_md(
            td.path(),
            "SddIA/skills/other-skill.md",
            r#"---
name: other-skill
provides:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
---
"#,
        );
        let phase = json!({
            "requires_capability": [{"id": "doc:closure", "contract": "doc.closure"}],
            "delegates_to": ["skill:other-skill"]
        });
        let bindings = resolve_phase_bindings(td.path(), &phase).expect("resolve");
        assert_eq!(bindings[0].provider, "skill:other-skill");
    }

    #[test]
    fn di_binding_shape() {
        let b = ResolvedBinding {
            capability_id: "doc:closure".into(),
            contract: "doc.closure".into(),
            provider: "skill:filesystem-manager".into(),
            provider_md_rel: "directories.skills/filesystem-manager.md".into(),
            contract_schema_rel: "SddIA/library/norms/capability-contracts/doc.closure.schema.json"
                .into(),
            version: "1.0.0".into(),
        };
        let payload = json!({
            "meta": {"schemaVersion": "2.0", "entityKind": "skill", "entityId": "filesystem-manager"},
            "request": {"operation": "MOVE"}
        });
        let merged = merge_di_binding_into_payload(&payload, &b);
        assert_eq!(merged["di_binding"]["capability_id"], "doc:closure");
        assert_eq!(merged["di_binding"]["provider"], "skill:filesystem-manager");
        assert_eq!(merged["di_binding"]["binding_ssot"], BINDING_SSOT_KEY);
        assert!(merged.get("request").is_some());
    }

    #[test]
    fn resolve_ambiguous_when_row_not_in_catalog() {
        let td = fixture_repo();
        write_md(
            td.path(),
            "SddIA/core/capability-bindings.md",
            r#"---
name: capability-bindings
bindings:
  - capability_id: "doc:closure"
    contract: "doc.closure"
    provider: "skill:ghost-provider"
---
"#,
        );
        write_md(
            td.path(),
            "SddIA/skills/alt-provider.md",
            r#"---
name: alt-provider
provides:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
---
"#,
        );
        let phase = json!({
            "requires_capability": [{"id": "doc:closure", "contract": "doc.closure"}]
        });
        let err = resolve_phase_bindings(td.path(), &phase).unwrap_err();
        assert!(
            matches!(
                err.code,
                DiResolveCode::ProviderAmbiguous | DiResolveCode::ProviderMismatch
            ),
            "got {:?}",
            err.code
        );
    }

    #[test]
    fn resolve_real_repo_feature_blind() {
        let root = crate::core::repo::find_repo_root().expect("repo root");
        let phase = json!({
            "name": "Cierre documental en rama",
            "requires_capability": [{
                "id": "doc:closure",
                "contract": "doc.closure",
                "version": ">=1.0.0"
            }]
        });
        let bindings = resolve_phase_bindings(&root, &phase).expect("real resolve");
        assert_eq!(bindings[0].provider, "skill:filesystem-manager");
    }
}
