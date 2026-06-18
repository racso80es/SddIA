use crate::core::parser::load_frontmatter_yaml;
use regex::Regex;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

static INPUT_KEY_RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

fn input_key_re() -> &'static Regex {
    INPUT_KEY_RE.get_or_init(|| Regex::new(r#""([A-Za-z_][A-Za-z0-9_]*)"\s*:"#).unwrap())
}

const RUNTIME_INJECTED: &[&str] = &[
    "cumulo_topology",
    "active_norm_pack",
    "active_norms",
    "target_executor_rbac",
];

const DEFAULTABLE: &[&str] = &[
    "persist_ref",
    "base_branch",
    "branch_name",
    "pbi_ref",
    "refined_requirements",
    "description",
    "pr_title",
    "pr_body",
    "target_branch",
];

pub type ProcessDef = HashMap<String, YamlValue>;

pub fn resolve_process_path(repo: &Path, process_name: &str) -> Result<PathBuf, String> {
    let process_dir = repo.join("SddIA/process");
    let direct = process_dir.join(format!("{process_name}.md"));
    if direct.is_file() {
        return Ok(direct);
    }
    for entry in std::fs::read_dir(&process_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if stem == "index" || stem == "process-contract" {
            continue;
        }
        let fm = load_frontmatter_yaml(&path)?;
        if yaml_str(&fm, "name").as_deref() == Some(process_name) {
            return Ok(path);
        }
        if let Some(YamlValue::Sequence(aliases)) = fm.get("aliases") {
            for a in aliases {
                if a.as_str() == Some(process_name) {
                    return Ok(path);
                }
            }
        }
    }
    Err(format!("Proceso no encontrado: {process_name}"))
}

pub fn load_process_def(
    repo: &Path,
    process_name: &str,
) -> Result<(String, ProcessDef, Vec<Value>), String> {
    let path = resolve_process_path(repo, process_name)?;
    let fm = load_frontmatter_yaml(&path)?;
    if fm.is_empty() {
        return Err(format!("Frontmatter inválido en {}", path.display()));
    }
    let canonical = yaml_str(&fm, "name")
        .unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().into_owned());
    let phases: Vec<Value> = match fm.get("phases") {
        Some(YamlValue::Sequence(seq)) => seq
            .iter()
            .filter_map(|v| serde_json::to_value(v).ok())
            .collect(),
        _ => vec![],
    };
    Ok((canonical, fm, phases))
}

fn yaml_str(fm: &ProcessDef, key: &str) -> Option<String> {
    fm.get(key).and_then(|v| v.as_str()).map(str::to_string)
}

pub fn extract_input_keys(process_def: &ProcessDef) -> Vec<String> {
    let raw = process_def
        .get("inputs_schema")
        .or_else(|| process_def.get("inputs"));
    let mut keys = Vec::new();
    match raw {
        Some(YamlValue::Mapping(m)) => {
            for k in m.keys() {
                if let Some(s) = k.as_str() {
                    keys.push(s.to_string());
                }
            }
        }
        Some(YamlValue::Sequence(seq)) => {
            for item in seq {
                match item {
                    YamlValue::Mapping(m) => {
                        for k in m.keys() {
                            if let Some(s) = k.as_str() {
                                keys.push(s.to_string());
                            }
                        }
                    }
                    YamlValue::String(s) => {
                        if let Some(cap) = input_key_re().captures(s) {
                            keys.push(cap[1].to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    keys
}

pub fn validate_process_inputs(
    process_def: &ProcessDef,
    process_inputs: &Value,
    canonical: &str,
) -> Result<(), String> {
    let declared = extract_input_keys(process_def);
    if declared.is_empty() {
        return Ok(());
    }
    let injected: HashSet<&str> = RUNTIME_INJECTED.iter().copied().collect();
    let defaultable: HashSet<&str> = DEFAULTABLE.iter().copied().collect();
    let required: Vec<&String> = declared
        .iter()
        .filter(|k| !injected.contains(k.as_str()) && !defaultable.contains(k.as_str()))
        .collect();
    let obj = process_inputs.as_object().ok_or_else(|| {
        format!("process_inputs debe ser objeto JSON para '{canonical}'")
    })?;
    let mut missing: Vec<String> = required
        .iter()
        .filter(|k| {
            obj.get(k.as_str())
                .map(|v| v.is_null() || v.as_str() == Some(""))
                .unwrap_or(true)
        })
        .map(|s| (*s).clone())
        .collect();
    if missing.contains(&"refined_requirements".to_string())
        && obj
            .get("description")
            .and_then(|v| v.as_str())
            .is_some_and(|s| !s.is_empty())
    {
        missing.retain(|k| k != "refined_requirements");
    }
    if !missing.is_empty() {
        return Err(
            serde_json::to_string(&json!({
                "code": "INPUT_VALIDATION",
                "message": format!("Faltan variables obligatorias para proceso '{canonical}'"),
                "missing": missing,
                "declared_inputs": declared,
            }))
            .unwrap_or_else(|_| "INPUT_VALIDATION".into()),
        );
    }
    Ok(())
}

pub fn normalize_request(raw: &Value) -> Result<(String, Value), String> {
    let _ = raw;
    Err(
        "Entrada inválida o formato legacy no soportado: use estrictamente \
         --process <nombre> --inputs '<json>'"
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_kalma2_by_filename() {
        let repo = crate::core::repo::find_repo_root().unwrap();
        let path = resolve_process_path(&repo, "kalma2-interact").unwrap();
        assert!(path.ends_with("kalma2-interact.md"));
    }

    #[test]
    fn load_kalma2_def() {
        let repo = crate::core::repo::find_repo_root().unwrap();
        let (name, _, phases) = load_process_def(&repo, "kalma2-interact").unwrap();
        assert_eq!(name, "kalma2-interact");
        assert!(!phases.is_empty());
    }
}
