//! Verificación de integridad de procesos (paridad `verify-process-integrity.py`).

use crate::core::parser::load_frontmatter_yaml;
use crate::forges::common::sha256_phases_integrity;
use serde_json::Value;
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::Path;

const SKIP_NAMES: &[&str] = &["process-contract", "index"];

fn phases_as_json(fm: &std::collections::HashMap<String, YamlValue>) -> Result<Value, String> {
    match fm.get("phases") {
        Some(YamlValue::Sequence(seq)) => {
            let phases: Vec<Value> = seq
                .iter()
                .map(serde_json::to_value)
                .collect::<Result<_, _>>()
                .map_err(|e| e.to_string())?;
            Ok(Value::Array(phases))
        }
        None => Err("missing phases array".into()),
        _ => Err("missing phases array".into()),
    }
}

pub fn verify(repo: &Path) -> Result<(), Vec<String>> {
    let process_dir = repo.join("SddIA/process");
    let mut errors = Vec::new();
    if !process_dir.is_dir() {
        errors.push(format!("Missing {}", process_dir.display()));
        return Err(errors);
    }

    let mut entries: Vec<_> = fs::read_dir(&process_dir)
        .map_err(|e| vec![format!("read process dir: {e}")])?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .collect();
    entries.sort();

    for md in entries {
        let stem = md.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if SKIP_NAMES.contains(&stem) {
            continue;
        }
        let fm = match load_frontmatter_yaml(&md) {
            Ok(fm) if !fm.is_empty() => fm,
            Ok(_) | Err(_) => {
                errors.push(format!("{}: frontmatter error: no frontmatter", md.file_name().unwrap_or_default().to_string_lossy()));
                continue;
            }
        };

        let phases = match phases_as_json(&fm) {
            Ok(p) => p,
            Err(e) => {
                errors.push(format!("{}: {e}", md.file_name().unwrap_or_default().to_string_lossy()));
                continue;
            }
        };

        if let Some(arr) = phases.as_array() {
            for (i, ph) in arr.iter().enumerate() {
                let delegates = ph
                    .get("delegates_to")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|x| x.as_str())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                if delegates.iter().any(|d| *d == "skill:cryptography-manager") {
                    errors.push(format!(
                        "{}: phase {i} declares skill:cryptography-manager; use action:crypto-broker per process-contract v1.2.0+",
                        md.file_name().unwrap_or_default().to_string_lossy()
                    ));
                }
            }
        }

        let hs = fm
            .get("hash_signature")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if hs.starts_with("sha256:") {
            let expected = hs.split(':').nth(1).unwrap_or("");
            let computed_full = sha256_phases_integrity(&phases);
            let computed = computed_full
                .strip_prefix("sha256:")
                .unwrap_or(&computed_full);
            if computed != expected {
                errors.push(format!(
                    "{}: hash_signature mismatch (file {}… vs computed {}…)",
                    md.file_name().unwrap_or_default().to_string_lossy(),
                    &expected[..expected.len().min(16)],
                    &computed[..computed.len().min(16)]
                ));
            }
        }

        if let (Some(YamlValue::Sequence(blocks)), Some(phases_arr)) =
            (fm.get("phase_invocations"), phases.as_array())
        {
            for ph in phases_arr {
                let Some(obj) = ph.as_object() else { continue };
                let delegates: Vec<&str> = obj
                    .get("delegates_to")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|x| x.as_str()).collect())
                    .unwrap_or_default();
                if !delegates.iter().any(|d| *d == "action:crypto-broker") {
                    continue;
                }
                let pname = obj.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let has_block = blocks.iter().any(|b| {
                    b.get("phase_name")
                        .and_then(|v| v.as_str())
                        == Some(pname)
                });
                if !has_block {
                    errors.push(format!(
                        "{}: phase {pname:?} delegates to crypto-broker but has no phase_invocations block",
                        md.file_name().unwrap_or_default().to_string_lossy()
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn run_cli(repo: &Path) -> i32 {
    match verify(repo) {
        Ok(()) => {
            println!("verify-process-integrity: OK");
            0
        }
        Err(errors) => {
            eprintln!("verify-process-integrity: FAILED");
            for e in errors {
                eprintln!("{e}");
            }
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn verify_skips_contract_and_index() {
        let tmp = TempDir::new().expect("tempdir");
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/process")).unwrap();
        fs::write(
            repo.join("SddIA/process/process-contract.md"),
            "---\nid: process-contract\nphases: []\n---\n",
        )
        .unwrap();
        assert!(verify(repo).is_ok());
    }
}
