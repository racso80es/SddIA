//! Verifica coherencia SSOT: crates con `src/main.rs` ↔ binarios en `compiled_capsules`.

use execute_process::engine::capsule_paths::load_compiled_capsule_roots;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SCAN_ROOTS: &[&str] = &[
    "SddIA/engine",
    "SddIA/skills",
    "SddIA/tools",
    "SddIA/daemons",
    "SddIA/interfaces",
];

fn read_package_name(cargo_toml: &Path) -> Option<String> {
    let text = fs::read_to_string(cargo_toml).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            if line != "[package]" {
                break;
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("name = ") {
            let raw = rest.trim().trim_matches('"');
            if !raw.is_empty() {
                return Some(raw.to_string());
            }
        }
    }
    None
}

fn discover_crates(repo: &Path) -> BTreeMap<String, PathBuf> {
    let mut out = BTreeMap::new();
    for root_rel in SCAN_ROOTS {
        let root = repo.join(root_rel);
        let Ok(rd) = fs::read_dir(&root) else {
            continue;
        };
        for entry in rd.flatten() {
            let crate_dir = entry.path();
            if !crate_dir.is_dir() {
                continue;
            }
            let main_rs = crate_dir.join("src/main.rs");
            if !main_rs.is_file() {
                continue;
            }
            let cargo = crate_dir.join("Cargo.toml");
            if !cargo.is_file() {
                continue;
            }
            let Some(name) = read_package_name(&cargo) else {
                continue;
            };
            out.insert(name, crate_dir.strip_prefix(repo).unwrap_or(&crate_dir).to_path_buf());
        }
    }
    out
}

fn resolve_native_bin(repo: &Path, name: &str, profiles: &[String]) -> Option<PathBuf> {
    let roots = load_compiled_capsule_roots(repo).ok()?;
    for profile in profiles {
        let candidate = roots.native_root.join(profile).join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn mandatory_bins() -> BTreeSet<&'static str> {
    BTreeSet::from(["execute-process", "sddia-qa"])
}

pub fn run(repo: &Path, json_out: bool) -> i32 {
    let profiles = load_compiled_capsule_roots(repo)
        .map(|r| r.profiles)
        .unwrap_or_else(|_| vec!["release".into(), "debug".into()]);

    let crates = discover_crates(repo);
    let mandatory = mandatory_bins();
    let mut missing: Vec<Value> = Vec::new();
    let mut found: Vec<Value> = Vec::new();
    let mut orphan_profiles: Vec<String> = Vec::new();

    for (name, crate_rel) in &crates {
        match resolve_native_bin(repo, name, &profiles) {
            Some(bin) => {
                found.push(json!({
                    "name": name,
                    "crate_path": crate_rel.to_string_lossy(),
                    "binary": bin.strip_prefix(repo).unwrap_or(&bin).to_string_lossy(),
                }));
            }
            None => {
                missing.push(json!({
                    "name": name,
                    "crate_path": crate_rel.to_string_lossy(),
                    "expected_under": format!(
                        "SddIA/target/{{{}}}/{}",
                        profiles.join("|"),
                        name
                    ),
                }));
            }
        }
    }

    for req in &mandatory {
        if !crates.contains_key(*req) {
            missing.push(json!({
                "name": req,
                "error": "crate con main.rs no descubierto en workspace",
            }));
        }
    }

    // Perfiles declarados en cumulo sin ningún binario presente (hint de build incompleto).
    let roots = load_compiled_capsule_roots(repo).ok();
    if let Some(roots) = roots {
        for profile in &profiles {
            let dir = roots.native_root.join(profile);
            if !dir.is_dir() {
                orphan_profiles.push(format!("{profile} (directorio ausente)"));
            }
        }
    }

    let ok = missing.is_empty();
    let report = json!({
        "success": ok,
        "crate_count": crates.len(),
        "binary_count": found.len(),
        "missing_count": missing.len(),
        "profiles_checked": profiles,
        "found": found,
        "missing": missing,
        "orphan_profile_hints": orphan_profiles,
        "build_hint": "cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build --workspace",
    });

    if json_out {
        println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    } else if ok {
        println!(
            "verify-compiled-capsules: OK — {}/{} binarios nativos presentes",
            found.len(),
            crates.len()
        );
    } else {
        eprintln!(
            "verify-compiled-capsules: FAIL — {} binario(s) ausente(s)",
            missing.len()
        );
        for item in &missing {
            eprintln!(
                "  - {} ({})",
                item.get("name").and_then(|v| v.as_str()).unwrap_or("?"),
                item.get("expected_under")
                    .or_else(|| item.get("error"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("?")
            );
        }
        eprintln!("hint: cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build --workspace");
    }

    if ok { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use execute_process::core::repo::find_repo_root;

    #[test]
    fn discovers_execute_process_and_sddia_qa() {
        let repo = find_repo_root().unwrap();
        let crates = discover_crates(&repo);
        assert!(crates.contains_key("execute-process"));
        assert!(crates.contains_key("sddia-qa"));
        assert!(crates.len() >= 20);
    }
}
