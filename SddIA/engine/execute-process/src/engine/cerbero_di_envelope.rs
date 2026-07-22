//! Revalidación Cerbero del envelope `di_binding` empaquetado (PBI-042 Hito 4 — R9).

use super::capability_di_resolver::{self, ResolvedBinding};
use super::eda_bus_topology::{load_event_bus_topology, write_json_atomic};
use crate::core::parser::parse_frontmatter;
use chrono::Utc;
use jsonschema::Validator;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const SKIP_ENV: &str = "SDDIA_LAB_SKIP_CAPABILITY_DI";
const BINDING_SSOT_KEY: &str = "capability_di.bindings";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CerberoEnvelopeCode {
    SchemaMismatch,
    BindingIncoherent,
    ConfigError,
}

impl CerberoEnvelopeCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaMismatch => "CERBERO_ENVELOPE_SCHEMA_MISMATCH",
            Self::BindingIncoherent => "CERBERO_DI_BINDING_INCOHERENT",
            Self::ConfigError => "CERBERO_CONFIG_ERROR",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CerberoEnvelopeError {
    pub code: CerberoEnvelopeCode,
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

fn envelope_schema_path(repo: &Path) -> Result<PathBuf, CerberoEnvelopeError> {
    let cfg = super::workspace::load_paths_config(repo).map_err(|e| CerberoEnvelopeError {
        code: CerberoEnvelopeCode::ConfigError,
        message: e,
    })?;
    let rel = cfg
        .get("directories")
        .and_then(|d| d.get("capability_contracts"))
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/library/norms/capability-contracts/");
    Ok(repo.join(rel.trim().trim_end_matches('/')).join("di.binding.schema.json"))
}

fn compile_envelope_schema(repo: &Path) -> Result<Validator, CerberoEnvelopeError> {
    let path = envelope_schema_path(repo)?;
    if !path.is_file() {
        return Err(CerberoEnvelopeError {
            code: CerberoEnvelopeCode::ConfigError,
            message: format!("schema envelope ausente: {}", path.display()),
        });
    }
    let text = fs::read_to_string(&path).map_err(|e| CerberoEnvelopeError {
        code: CerberoEnvelopeCode::ConfigError,
        message: e.to_string(),
    })?;
    let schema_val: Value = serde_json::from_str(&text).map_err(|e| CerberoEnvelopeError {
        code: CerberoEnvelopeCode::ConfigError,
        message: e.to_string(),
    })?;
    Validator::new(&schema_val).map_err(|e| CerberoEnvelopeError {
        code: CerberoEnvelopeCode::ConfigError,
        message: format!("schema envelope inválido {}: {e}", path.display()),
    })
}

fn load_binding_table(
    repo: &Path,
) -> Result<HashMap<String, (String, String)>, CerberoEnvelopeError> {
    let cfg = super::workspace::load_paths_config(repo).map_err(|e| CerberoEnvelopeError {
        code: CerberoEnvelopeCode::ConfigError,
        message: e,
    })?;
    let rel = cfg
        .get("capability_di")
        .and_then(|c| c.get("bindings"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| CerberoEnvelopeError {
            code: CerberoEnvelopeCode::ConfigError,
            message: "capability_di.bindings ausente en Cúmulo".into(),
        })?;
    let path = repo.join(rel.trim().trim_start_matches("./"));
    if !path.is_file() {
        return Err(CerberoEnvelopeError {
            code: CerberoEnvelopeCode::ConfigError,
            message: format!("capability-bindings inaccesible: {}", path.display()),
        });
    }
    let fm = parse_frontmatter(&path).map_err(|e| CerberoEnvelopeError {
        code: CerberoEnvelopeCode::ConfigError,
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
        if cap.is_empty() || provider.is_empty() {
            continue;
        }
        map.insert(cap, (contract, provider));
    }
    Ok(map)
}

fn str_field(obj: &Value, key: &str) -> Option<String> {
    obj.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn cross_semantic(
    expected: &Value,
    packaged: &Value,
    binding_table: &HashMap<String, (String, String)>,
) -> Result<(), CerberoEnvelopeError> {
    let cap_id = str_field(packaged, "capability_id").ok_or_else(|| CerberoEnvelopeError {
        code: CerberoEnvelopeCode::BindingIncoherent,
        message: "capability_id ausente en packaged".into(),
    })?;

    for key in [
        "capability_id",
        "contract",
        "contract_schema_ref",
        "provider",
        "provider_ref",
        "resolved_version",
        "binding_ssot",
    ] {
        let exp = expected.get(key);
        let got = packaged.get(key);
        if exp != got {
            return Err(CerberoEnvelopeError {
                code: CerberoEnvelopeCode::BindingIncoherent,
                message: format!(
                    "campo '{key}' incoherente: expected={exp:?} packaged={got:?}"
                ),
            });
        }
    }

    let contract = str_field(packaged, "contract").unwrap_or_default();
    let provider = str_field(packaged, "provider").unwrap_or_default();
    let ssot = str_field(packaged, "binding_ssot").unwrap_or_default();
    if ssot != BINDING_SSOT_KEY {
        return Err(CerberoEnvelopeError {
            code: CerberoEnvelopeCode::BindingIncoherent,
            message: format!("binding_ssot inválido: {ssot}"),
        });
    }

    let (table_contract, table_provider) =
        binding_table
            .get(&cap_id)
            .ok_or_else(|| CerberoEnvelopeError {
                code: CerberoEnvelopeCode::BindingIncoherent,
                message: format!("capability_id '{cap_id}' sin fila en binding table"),
            })?;
    if &contract != table_contract || &provider != table_provider {
        return Err(CerberoEnvelopeError {
            code: CerberoEnvelopeCode::BindingIncoherent,
            message: format!(
                "cruce binding table fallido para '{cap_id}': packaged contract={contract} provider={provider} vs table contract={table_contract} provider={table_provider}"
            ),
        });
    }
    Ok(())
}

pub fn write_cerbero_envelope_dead_letter(
    repo: &Path,
    err: &CerberoEnvelopeError,
    phase_name: &str,
    process_name: &str,
) {
    let Ok(topo) = load_event_bus_topology(repo) else {
        return;
    };
    let dir = repo.join(&topo.dead_letter);
    let _ = fs::create_dir_all(&dir);
    let id = Uuid::new_v4();
    let path = dir.join(format!("{id}.cerbero-di-envelope.json"));
    let payload = json!({
        "event_id": id.to_string(),
        "event_type": "Cerbero_Di_Envelope_Failed",
        "state": "dead-letter",
        "code": err.code.as_str(),
        "message": err.message,
        "process_name": process_name,
        "phase_name": phase_name,
        "emitted_at": Utc::now().to_rfc3339(),
        "emitter": "cerbero_di_envelope",
    });
    let _ = write_json_atomic(&path, &payload);
}

/// Valida objetos `di_binding` empaquetados post-gate y post-RBAC.
pub fn validate_packaged_bindings(
    repo: &Path,
    resolved: &[ResolvedBinding],
    packaged: &[Value],
) -> Result<(), CerberoEnvelopeError> {
    if env_skip() || resolved.is_empty() {
        return Ok(());
    }
    if resolved.len() != packaged.len() {
        return Err(CerberoEnvelopeError {
            code: CerberoEnvelopeCode::BindingIncoherent,
            message: format!(
                "conteo mismatch: resolved={} packaged={}",
                resolved.len(),
                packaged.len()
            ),
        });
    }

    let validator = compile_envelope_schema(repo)?;
    let binding_table = load_binding_table(repo)?;

    for (binding, pkg) in resolved.iter().zip(packaged.iter()) {
        if let Err(e) = validator.validate(pkg) {
            return Err(CerberoEnvelopeError {
                code: CerberoEnvelopeCode::SchemaMismatch,
                message: format!("schema envelope: {e}"),
            });
        }
        let expected = capability_di_resolver::di_binding_object(binding);
        cross_semantic(&expected, pkg, &binding_table)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_file(dir: &Path, rel: &str, body: &str) {
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
        write_file(
            root,
            "SddIA/core/cumulo.paths.json",
            r#"{
  "directories": {
    "capability_contracts": "SddIA/library/norms/capability-contracts/"
  },
  "capability_di": {
    "bindings": "SddIA/core/capability-bindings.md"
  },
  "event_bus": "./.events",
  "eda_bus": { "dead_letter": "./.events/dead-letter" }
}"#,
        );
        write_file(
            root,
            "SddIA/core/capability-bindings.md",
            r#"---
bindings:
  - capability_id: "doc:closure"
    contract: "doc.closure"
    provider: "skill:filesystem-manager"
---
"#,
        );
        write_file(
            root,
            "SddIA/library/norms/capability-contracts/di.binding.schema.json",
            include_str!("../../../../library/norms/capability-contracts/di.binding.schema.json"),
        );
        // Proveedor real en fixture: sin él validate_di_rbac → CERBERO_CONFIG_ERROR (no RBAC).
        write_file(
            root,
            "SddIA/skills/filesystem-manager.md",
            r#"---
name: filesystem-manager
context: filesystem-ops
provides:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
---
"#,
        );
        td
    }

    fn sample_binding() -> ResolvedBinding {
        ResolvedBinding {
            capability_id: "doc:closure".into(),
            contract: "doc.closure".into(),
            provider: "skill:filesystem-manager".into(),
            provider_md_rel: "directories.skills/filesystem-manager.md".into(),
            contract_schema_rel: "SddIA/library/norms/capability-contracts/doc.closure.schema.json"
                .into(),
            version: "1.0.0".into(),
        }
    }

    #[test]
    fn ac_r9_valid_envelope_ok() {
        let td = fixture_repo();
        let binding = sample_binding();
        let packaged = vec![capability_di_resolver::di_binding_object(&binding)];
        validate_packaged_bindings(td.path(), &[binding], &packaged).expect("ok");
    }

    #[test]
    fn ac_r9_tampered_contract_incoherent() {
        let td = fixture_repo();
        let binding = sample_binding();
        let mut packaged = capability_di_resolver::di_binding_object(&binding);
        packaged["contract"] = json!("doc.tampered");
        let err =
            validate_packaged_bindings(td.path(), &[binding], &[packaged]).unwrap_err();
        assert_eq!(err.code, CerberoEnvelopeCode::BindingIncoherent);
        assert_eq!(err.code.as_str(), "CERBERO_DI_BINDING_INCOHERENT");
    }

    #[test]
    fn ac_r9_missing_required_schema_mismatch() {
        let td = fixture_repo();
        let binding = sample_binding();
        let mut packaged = capability_di_resolver::di_binding_object(&binding);
        if let Some(obj) = packaged.as_object_mut() {
            obj.remove("provider_ref");
        }
        let err =
            validate_packaged_bindings(td.path(), &[binding], &[packaged]).unwrap_err();
        assert_eq!(err.code, CerberoEnvelopeCode::SchemaMismatch);
        assert_eq!(err.code.as_str(), "CERBERO_ENVELOPE_SCHEMA_MISMATCH");
    }

    #[test]
    fn ac_r5_rbac_deny_never_reaches_envelope_regression() {
        let td = fixture_repo();
        let policies = vec!["knowledge-management".into()];
        let bindings = vec![sample_binding()];
        let rbac_err = super::super::cerbero_di_rbac::validate_di_rbac(
            td.path(),
            "feature",
            "lab",
            &policies,
            &bindings,
        )
        .unwrap_err();
        assert_eq!(rbac_err.code.as_str(), "CERBERO_RBAC_DENIED");
    }
}
