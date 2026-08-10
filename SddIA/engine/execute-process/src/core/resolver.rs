use crate::core::parser::load_frontmatter_yaml;
use crate::core::paths::load_paths_config;
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
    // pull-request-review v1.1 / genoma: opcional hasta que delivery-close aporte URL
    "pr_url",
    "document_context",
];

pub type ProcessDef = HashMap<String, YamlValue>;

/// Raíces de resolución process: domain roots (orden Cúmulo) + Core `directories.process`.
pub fn process_search_roots(repo: &Path) -> Result<Vec<PathBuf>, String> {
    let cfg = load_paths_config(repo)?;
    let dirs = cfg.get("directories");
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Some(arr) = dirs
        .and_then(|d| d.get("process_domain_roots"))
        .and_then(|v| v.as_array())
    {
        for item in arr {
            if let Some(rel) = item.as_str() {
                let rel = rel.trim().trim_end_matches('/').replace('\\', "/");
                if !rel.is_empty() {
                    roots.push(repo.join(rel));
                }
            }
        }
    }
    let core_rel = dirs
        .and_then(|d| d.get("process"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_end_matches('/').replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "SddIA/process".to_string());
    roots.push(repo.join(core_rel));
    Ok(roots)
}

fn resolve_in_root(root: &Path, process_name: &str) -> Result<Option<PathBuf>, String> {
    if !root.is_dir() {
        return Ok(None);
    }
    let direct = root.join(format!("{process_name}.md"));
    if direct.is_file() {
        return Ok(Some(direct));
    }

    let mut by_name: Option<PathBuf> = None;
    let mut by_alias: Option<PathBuf> = None;

    for entry in std::fs::read_dir(root).map_err(|e| e.to_string())? {
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
            by_name = Some(path);
            break;
        }
        if by_alias.is_none() {
            if let Some(YamlValue::Sequence(aliases)) = fm.get("aliases") {
                for a in aliases {
                    if a.as_str() == Some(process_name) {
                        by_alias = Some(path.clone());
                        break;
                    }
                }
            }
        }
    }

    Ok(by_name.or(by_alias))
}

pub fn resolve_process_path(repo: &Path, process_name: &str) -> Result<PathBuf, String> {
    let roots = process_search_roots(repo)?;
    for root in roots {
        if let Some(path) = resolve_in_root(&root, process_name)? {
            return Ok(path);
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
    use std::fs;

    fn write_minimal_process(dir: &Path, name: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join(format!("{name}.md")),
            format!(
                "---\nname: \"{name}\"\nuuid: \"00000000-0000-4000-8000-000000000001\"\nversion: \"1.0.0\"\nphases: []\n---\n# {name}\n"
            ),
        )
        .unwrap();
    }

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

    #[test]
    fn resolves_kalma2_by_filename() {
        let repo = crate::core::repo::find_repo_root().unwrap();
        let path = resolve_process_path(&repo, "kalma2-interact").unwrap();
        assert!(path.ends_with("kalma2-interact.md"));
        assert!(
            path.to_string_lossy().contains("SddIA/process"),
            "Core process must resolve under directories.process"
        );
    }

    #[test]
    fn load_kalma2_def() {
        let repo = crate::core::repo::find_repo_root().unwrap();
        let (name, _, phases) = load_process_def(&repo, "kalma2-interact").unwrap();
        assert_eq!(name, "kalma2-interact");
        assert!(!phases.is_empty());
    }

    #[test]
    fn ac_resolve_domain_root_only() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        let domain = "SddIA/library/codexes/codex-software-engineering/process";
        fixture_cumulo(repo, &[domain]);
        write_minimal_process(&repo.join(domain), "feature");
        // Core process dir ausente de feature — solo domain
        fs::create_dir_all(repo.join("SddIA/process")).unwrap();

        let path = resolve_process_path(repo, "feature").unwrap();
        assert!(
            path.ends_with("codex-software-engineering/process/feature.md"),
            "got {}",
            path.display()
        );
    }

    #[test]
    fn ac_resolve_core_when_absent_from_domain() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        let domain = "SddIA/library/codexes/codex-software-engineering/process";
        fixture_cumulo(repo, &[domain]);
        fs::create_dir_all(repo.join(domain)).unwrap();
        write_minimal_process(&repo.join("SddIA/process"), "kalma2-interact");

        let path = resolve_process_path(repo, "kalma2-interact").unwrap();
        assert!(path.ends_with("SddIA/process/kalma2-interact.md"));
    }

    #[test]
    fn ac_resolve_missing_errors() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        fixture_cumulo(
            repo,
            &["SddIA/library/codexes/codex-software-engineering/process"],
        );
        fs::create_dir_all(repo.join("SddIA/process")).unwrap();
        let err = resolve_process_path(repo, "no-such-process").unwrap_err();
        assert!(err.contains("Proceso no encontrado"));
    }

    #[test]
    fn ac_resolve_domain_precedes_core() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        let domain = "SddIA/library/codexes/codex-software-engineering/process";
        fixture_cumulo(repo, &[domain]);
        write_minimal_process(&repo.join(domain), "feature");
        write_minimal_process(&repo.join("SddIA/process"), "feature");

        let path = resolve_process_path(repo, "feature").unwrap();
        assert!(
            path.to_string_lossy()
                .contains("codex-software-engineering/process"),
            "domain-first; got {}",
            path.display()
        );
    }

    #[test]
    fn ac_resolve_no_single_core_hardcode() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        // Solo domain root; sin SddIA/process en absoluto
        let domain = "pack/domain-process";
        let core = repo.join("SddIA/core");
        fs::create_dir_all(&core).unwrap();
        fs::write(
            core.join("cumulo.paths.json"),
            serde_json::to_string_pretty(&json!({
                "directories": {
                    "process": "SddIA/process",
                    "process_domain_roots": [domain]
                }
            }))
            .unwrap(),
        )
        .unwrap();
        write_minimal_process(&repo.join(domain), "refactorization");

        let path = resolve_process_path(repo, "refactorization").unwrap();
        assert!(path.ends_with("pack/domain-process/refactorization.md"));
    }
}
