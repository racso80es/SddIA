//! Cerbero RBAC post-gate pre-inject (PBI-042 Hito 3 — R5).

use super::capability_di_resolver::ResolvedBinding;
use super::eda_bus_topology::{load_event_bus_topology, write_json_atomic};
use super::fractal_bus::load_radamanto_config;
use crate::core::parser::parse_frontmatter;
use crate::core::resolver::ProcessDef;
use chrono::Utc;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const SKIP_ENV: &str = "SDDIA_LAB_SKIP_CAPABILITY_DI";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CerberoDiCode {
    RbacDenied,
    EntityRevoked,
    ConfigError,
}

impl CerberoDiCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RbacDenied => "CERBERO_RBAC_DENIED",
            Self::EntityRevoked => "CERBERO_ENTITY_REVOKED",
            Self::ConfigError => "CERBERO_CONFIG_ERROR",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CerberoDiError {
    pub code: CerberoDiCode,
    pub message: String,
}

fn env_skip() -> bool {
    std::env::var(SKIP_ENV)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn provider_fs_path(repo: &Path, provider: &str) -> Option<PathBuf> {
    let (kind, name) = provider.split_once(':')?;
    let rel = match kind {
        "skill" => format!("SddIA/skills/{name}.md"),
        "action" => format!("SddIA/actions/{name}.md"),
        "tool" => format!("SddIA/tools/{name}.md"),
        _ => return None,
    };
    let p = repo.join(rel);
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}

fn yaml_contexts(fm: &HashMap<String, YamlValue>) -> Vec<String> {
    let Some(ctx) = fm.get("context") else {
        return vec![];
    };
    match ctx {
        YamlValue::String(s) => {
            let t = s.trim();
            if t.is_empty() {
                vec![]
            } else {
                vec![t.to_string()]
            }
        }
        YamlValue::Sequence(seq) => seq
            .iter()
            .filter_map(|v| {
                v.as_str()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
            })
            .collect(),
        _ => vec![],
    }
}

fn revoked_path(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_radamanto_config(repo)?;
    let rel = cfg
        .get("revoked_entities")
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/cerbero/revoked_entities.json");
    Ok(repo.join(rel.trim_start_matches("./")))
}

fn is_entity_revoked(repo: &Path, entity_id: &str) -> Result<bool, String> {
    let path = revoked_path(repo)?;
    if !path.is_file() {
        return Ok(false);
    }
    let data: Value =
        serde_json::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    for key in ["revoked", "permanent"] {
        if data
            .get(key)
            .and_then(|v| v.as_object())
            .map(|m| m.contains_key(entity_id))
            .unwrap_or(false)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Políticas del solicitante: `target_executor_rbac.allowed_policies` o `context[]` del proceso.
pub fn resolve_requester_policies(process_def: &ProcessDef, process_inputs: &Value) -> Vec<String> {
    if let Some(arr) = process_inputs
        .get("target_executor_rbac")
        .and_then(|v| v.get("allowed_policies"))
        .and_then(|v| v.as_array())
    {
        let policies: Vec<String> = arr
            .iter()
            .filter_map(|v| {
                v.as_str()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
            })
            .collect();
        if !policies.is_empty() {
            return policies;
        }
    }
    if let Some(YamlValue::Sequence(seq)) = process_def.get("context") {
        return seq
            .iter()
            .filter_map(|v| {
                v.as_str()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
            })
            .collect();
    }
    vec![]
}

fn provider_allowed_for_requester(
    provider: &str,
    provider_contexts: &[String],
    requester_policies: &[String],
) -> bool {
    if provider == "action:crypto-broker" {
        return requester_policies.iter().any(|p| p == "cryptography-broker");
    }
    if provider_contexts.is_empty() {
        return false;
    }
    let allowed: HashSet<_> = requester_policies.iter().cloned().collect();
    provider_contexts.iter().any(|c| allowed.contains(c))
}

pub fn write_cerbero_di_dead_letter(
    repo: &Path,
    err: &CerberoDiError,
    phase_name: &str,
    process_name: &str,
) {
    let Ok(topo) = load_event_bus_topology(repo) else {
        return;
    };
    let dir = repo.join(&topo.dead_letter);
    let _ = fs::create_dir_all(&dir);
    let id = Uuid::new_v4();
    let path = dir.join(format!("{id}.cerbero-di-rbac.json"));
    let payload = json!({
        "event_id": id.to_string(),
        "event_type": "Cerbero_Di_Rbac_Failed",
        "state": "dead-letter",
        "code": err.code.as_str(),
        "message": err.message,
        "process_name": process_name,
        "phase_name": phase_name,
        "emitted_at": Utc::now().to_rfc3339(),
        "emitter": "cerbero_di_rbac",
    });
    let _ = write_json_atomic(&path, &payload);
}

/// Valida RBAC Cerbero sobre proveedores resueltos. `Ok(())` si skip, sin bindings, o allow.
pub fn validate_di_rbac(
    repo: &Path,
    process_name: &str,
    phase_name: &str,
    requester_policies: &[String],
    bindings: &[ResolvedBinding],
) -> Result<(), CerberoDiError> {
    if env_skip() || bindings.is_empty() {
        return Ok(());
    }
    if requester_policies.is_empty() {
        return Err(CerberoDiError {
            code: CerberoDiCode::ConfigError,
            message: format!(
                "políticas solicitante vacías (process={process_name}, phase={phase_name})"
            ),
        });
    }

    for binding in bindings {
        let provider = &binding.provider;
        if is_entity_revoked(repo, provider).map_err(|e| CerberoDiError {
            code: CerberoDiCode::ConfigError,
            message: e,
        })? {
            return Err(CerberoDiError {
                code: CerberoDiCode::EntityRevoked,
                message: format!("proveedor '{provider}' revocado en revoked_entities"),
            });
        }

        let path = provider_fs_path(repo, provider).ok_or_else(|| CerberoDiError {
            code: CerberoDiCode::ConfigError,
            message: format!("proveedor no resoluble: {provider}"),
        })?;
        let fm = parse_frontmatter(&path).map_err(|e| CerberoDiError {
            code: CerberoDiCode::ConfigError,
            message: e,
        })?;
        let contexts = yaml_contexts(&fm);
        if !provider_allowed_for_requester(provider, &contexts, requester_policies) {
            return Err(CerberoDiError {
                code: CerberoDiCode::RbacDenied,
                message: format!(
                    "RBAC deny: provider '{provider}' context={contexts:?} ∉ requester={requester_policies:?}"
                ),
            });
        }
    }
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
  "event_bus": "./.events",
  "eda_bus": { "dead_letter": "./.events/dead-letter" }
}"#,
        );
        write_md(
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

    #[test]
    fn ac_r5_gate_pass_cerbero_deny() {
        let td = fixture_repo();
        let bindings = vec![super::super::capability_di_resolver::ResolvedBinding {
            capability_id: "doc:closure".into(),
            contract: "doc.closure".into(),
            provider: "skill:filesystem-manager".into(),
            provider_md_rel: "directories.skills/filesystem-manager.md".into(),
            contract_schema_rel: "SddIA/library/norms/capability-contracts/doc.closure.schema.json"
                .into(),
            version: "1.0.0".into(),
        }];
        let policies = vec!["knowledge-management".into()];
        let err = validate_di_rbac(td.path(), "feature", "lab", &policies, &bindings).unwrap_err();
        assert_eq!(err.code, CerberoDiCode::RbacDenied);
        assert_eq!(err.code.as_str(), "CERBERO_RBAC_DENIED");
    }

    #[test]
    fn ac_r5_allow_when_context_matches() {
        let td = fixture_repo();
        let bindings = vec![super::super::capability_di_resolver::ResolvedBinding {
            capability_id: "doc:closure".into(),
            contract: "doc.closure".into(),
            provider: "skill:filesystem-manager".into(),
            provider_md_rel: "directories.skills/filesystem-manager.md".into(),
            contract_schema_rel: "x".into(),
            version: "1.0.0".into(),
        }];
        validate_di_rbac(
            td.path(),
            "feature",
            "lab",
            &["filesystem-ops".into()],
            &bindings,
        )
        .expect("allow");
    }

    #[test]
    fn resolve_policies_from_inputs() {
        let mut def = ProcessDef::new();
        def.insert(
            "context".into(),
            YamlValue::Sequence(vec![YamlValue::String("ecosystem-evolution".into())]),
        );
        let inputs = json!({
            "target_executor_rbac": {
                "allowed_policies": ["knowledge-management"]
            }
        });
        let p = resolve_requester_policies(&def, &inputs);
        assert_eq!(p, vec!["knowledge-management"]);
    }
}
