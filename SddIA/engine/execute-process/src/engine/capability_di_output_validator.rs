//! Validación JSON Schema del payload real post-cápsula (PBI-042 Hito 3 — R8).

use super::capability_di_resolver::ResolvedBinding;
use super::eda_bus_topology::{load_event_bus_topology, write_json_atomic};
use super::workspace::load_paths_config;
use chrono::Utc;
use jsonschema::Validator;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const SKIP_ENV: &str = "SDDIA_LAB_SKIP_CAPABILITY_DI";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputValidatorCode {
    SchemaMismatch,
    ConfigError,
}

impl OutputValidatorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaMismatch => "CONTRACT_OUTPUT_SCHEMA_MISMATCH",
            Self::ConfigError => "CONTRACT_OUTPUT_CONFIG_ERROR",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputValidatorError {
    pub code: OutputValidatorCode,
    pub message: String,
}

fn env_skip() -> bool {
    std::env::var(SKIP_ENV)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
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

fn schema_path(repo: &Path, contract: &str) -> Result<PathBuf, OutputValidatorError> {
    let dir = contracts_dir(repo).map_err(|e| OutputValidatorError {
        code: OutputValidatorCode::ConfigError,
        message: e,
    })?;
    let path = dir.join(format!("{contract}.schema.json"));
    if !path.is_file() {
        return Err(OutputValidatorError {
            code: OutputValidatorCode::ConfigError,
            message: format!("schema ausente: {}", path.display()),
        });
    }
    Ok(path)
}

fn compile_schema(path: &Path) -> Result<Validator, OutputValidatorError> {
    let text = fs::read_to_string(path).map_err(|e| OutputValidatorError {
        code: OutputValidatorCode::ConfigError,
        message: e.to_string(),
    })?;
    let schema_val: Value = serde_json::from_str(&text).map_err(|e| OutputValidatorError {
        code: OutputValidatorCode::ConfigError,
        message: e.to_string(),
    })?;
    Validator::new(&schema_val).map_err(|e| OutputValidatorError {
        code: OutputValidatorCode::ConfigError,
        message: format!("schema inválido {}: {e}", path.display()),
    })
}

pub fn write_output_dead_letter(
    repo: &Path,
    err: &OutputValidatorError,
    phase_name: &str,
    process_name: &str,
    contract: &str,
) {
    let Ok(topo) = load_event_bus_topology(repo) else {
        return;
    };
    let dir = repo.join(&topo.dead_letter);
    let _ = fs::create_dir_all(&dir);
    let id = Uuid::new_v4();
    let path = dir.join(format!("{id}.capability-di-output.json"));
    let payload = json!({
        "event_id": id.to_string(),
        "event_type": "Capability_Di_Output_Schema_Failed",
        "state": "dead-letter",
        "code": err.code.as_str(),
        "message": err.message,
        "contract": contract,
        "process_name": process_name,
        "phase_name": phase_name,
        "emitted_at": Utc::now().to_rfc3339(),
        "emitter": "capability_di_output_validator",
    });
    let _ = write_json_atomic(&path, &payload);
}

/// Valida `payload` contra el schema del contrato del binding.
pub fn validate_output_payload(
    repo: &Path,
    binding: &ResolvedBinding,
    payload: &Value,
) -> Result<(), OutputValidatorError> {
    if env_skip() {
        return Ok(());
    }
    let path = schema_path(repo, &binding.contract)?;
    let compiled = compile_schema(&path)?;
    if let Err(e) = compiled.validate(payload) {
        return Err(OutputValidatorError {
            code: OutputValidatorCode::SchemaMismatch,
            message: format!(
                "payload no cumple contrato '{}': {e}",
                binding.contract
            ),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
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
  "event_bus": "./.events",
  "eda_bus": { "dead_letter": "./.events/dead-letter" }
}"#,
        );
        write_file(
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
        td
    }

    fn binding() -> ResolvedBinding {
        ResolvedBinding {
            capability_id: "doc:closure".into(),
            contract: "doc.closure".into(),
            provider: "skill:filesystem-manager".into(),
            provider_md_rel: "x".into(),
            contract_schema_rel: "x".into(),
            version: "1.0.0".into(),
        }
    }

    #[test]
    fn ac_r8_valid_payload() {
        let td = fixture_repo();
        let payload = json!({"exitCode": 0, "data": {}});
        validate_output_payload(td.path(), &binding(), &payload).expect("ok");
    }

    #[test]
    fn ac_r8_schema_mismatch() {
        let td = fixture_repo();
        let payload = json!({"exitCode": 0});
        let err = validate_output_payload(td.path(), &binding(), &payload).unwrap_err();
        assert_eq!(err.code, OutputValidatorCode::SchemaMismatch);
        assert_eq!(err.code.as_str(), "CONTRACT_OUTPUT_SCHEMA_MISMATCH");
    }

    #[test]
    fn ac_r8_dlq_written() {
        let td = fixture_repo();
        let err = OutputValidatorError {
            code: OutputValidatorCode::SchemaMismatch,
            message: "test".into(),
        };
        write_output_dead_letter(td.path(), &err, "phase", "feature", "doc.closure");
        let dlq = td.path().join(".events/dead-letter");
        let count = fs::read_dir(&dlq).unwrap().count();
        assert!(count >= 1);
    }
}
