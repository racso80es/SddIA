//! Verifica coherencia SSOT: crates con `src/main.rs` ↔ binarios en `compiled_capsules` y frescura (H-NEW-07 / MAN-52).

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
            out.insert(
                name,
                crate_dir.strip_prefix(repo).unwrap_or(&crate_dir).to_path_buf(),
            );
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

pub fn check_binary_freshness(repo: &Path, name: &str, bin: &Path) -> Option<String> {
    if !sddia_runtime_parity::LIVE_ORGANS.contains(&name) {
        return None;
    }
    let wit_path = PathBuf::from(format!("{}.sha256", bin.display()));

    if wit_path.is_file() {
        let witness = match sddia_runtime_parity::read_capsule_witness(bin) {
            Ok(w) => w,
            Err(e) => return Some(format!("{name}:witness_invalid:{e}")),
        };
        match sddia_runtime_parity::sha256_file_hex(bin) {
            Ok(elf_hex) if elf_hex != witness.elf_sha256 => {
                return Some(format!("{name}:elf_drift"));
            }
            Err(e) => return Some(format!("{name}:elf_read_error:{e}")),
            _ => {}
        }
        match sddia_runtime_parity::compute_organ_source_digest(repo, name) {
            Ok(src_hex) if src_hex != witness.source_sha256 => {
                return Some(format!("{name}:source_drift"));
            }
            Err(e) => return Some(format!("{name}:source_digest_error:{e}")),
            _ => {}
        }
        return None;
    }

    let bin_mtime = fs::metadata(bin).ok()?.modified().ok()?;
    if let Ok(src_files) = sddia_runtime_parity::collect_organ_source_files(repo, name) {
        for f in src_files {
            if let Ok(meta) = fs::metadata(&f) {
                if let Ok(src_mtime) = meta.modified() {
                    if src_mtime > bin_mtime {
                        return Some(format!("{name}:source_newer_than_binary"));
                    }
                }
            }
        }
    }

    None
}

pub fn run(repo: &Path, json_out: bool) -> i32 {
    let profiles = load_compiled_capsule_roots(repo)
        .map(|r| r.profiles)
        .unwrap_or_else(|_| vec!["release".into(), "debug".into()]);

    let crates = discover_crates(repo);
    let mandatory = mandatory_bins();
    let mut missing: Vec<Value> = Vec::new();
    let mut stale: Vec<Value> = Vec::new();
    let mut found: Vec<Value> = Vec::new();
    let mut orphan_profiles: Vec<String> = Vec::new();

    for (name, crate_rel) in &crates {
        match resolve_native_bin(repo, name, &profiles) {
            Some(bin) => {
                let bin_rel = bin.strip_prefix(repo).unwrap_or(&bin).to_string_lossy().to_string();
                if let Some(reason) = check_binary_freshness(repo, name, &bin) {
                    stale.push(json!({
                        "name": name,
                        "crate_path": crate_rel.to_string_lossy(),
                        "binary": bin_rel,
                        "reason": reason,
                    }));
                } else {
                    found.push(json!({
                        "name": name,
                        "crate_path": crate_rel.to_string_lossy(),
                        "binary": bin_rel,
                    }));
                }
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

    let ok = missing.is_empty() && stale.is_empty();
    let report = json!({
        "success": ok,
        "crate_count": crates.len(),
        "binary_count": found.len(),
        "missing_count": missing.len(),
        "stale_count": stale.len(),
        "profiles_checked": profiles,
        "found": found,
        "missing": missing,
        "stale": stale,
        "orphan_profile_hints": orphan_profiles,
        "build_hint": "cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build --workspace",
    });

    if json_out {
        println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    } else if ok {
        println!(
            "verify-compiled-capsules: OK — {}/{} binarios nativos presentes y frescos",
            found.len(),
            crates.len()
        );
    } else {
        eprintln!(
            "verify-compiled-capsules: FAIL — {} ausente(s), {} stale",
            missing.len(),
            stale.len()
        );
        for item in &missing {
            eprintln!(
                "  - [missing] {} ({})",
                item.get("name").and_then(|v| v.as_str()).unwrap_or("?"),
                item.get("expected_under")
                    .or_else(|| item.get("error"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("?")
            );
        }
        for item in &stale {
            eprintln!(
                "  - [stale] {} ({})",
                item.get("name").and_then(|v| v.as_str()).unwrap_or("?"),
                item.get("reason").and_then(|v| v.as_str()).unwrap_or("?")
            );
        }
        eprintln!("hint: ./SddIA/scripts/sync-live-organs.sh");
    }

    if ok { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use execute_process::core::repo::find_repo_root;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn discovers_execute_process_and_sddia_qa() {
        let repo = find_repo_root().unwrap();
        let crates = discover_crates(&repo);
        assert!(crates.contains_key("execute-process"));
        assert!(crates.contains_key("sddia-qa"));
        assert!(crates.len() >= 20);
    }

    #[test]
    fn check_binary_freshness_detects_stale_witness_and_newer_source() {
        let td = tempfile::tempdir().unwrap();
        let r = td.path();
        fs::create_dir_all(r.join("SddIA/tools/sddia-qa/src")).unwrap();
        fs::write(
            r.join("SddIA/tools/sddia-qa/Cargo.toml"),
            "[package]\nname = \"sddia-qa\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::write(r.join("SddIA/tools/sddia-qa/src/main.rs"), "fn main() {}\n").unwrap();

        let bin_dir = r.join("SddIA/target/release");
        fs::create_dir_all(&bin_dir).unwrap();
        let bin = bin_dir.join("sddia-qa");
        fs::write(&bin, b"fake-elf").unwrap();

        let elf_hex = sddia_runtime_parity::sha256_file_hex(&bin).unwrap();
        let src_hex = sddia_runtime_parity::compute_organ_source_digest(r, "sddia-qa").unwrap();
        let wit = bin_dir.join("sddia-qa.sha256");

        // Testigo válido → None
        fs::write(&wit, format!("source_sha256: {src_hex}\nelf_sha256: {elf_hex}\n")).unwrap();
        assert_eq!(check_binary_freshness(r, "sddia-qa", &bin), None);

        // Testigo con source_sha256 obsoleto → source_drift
        fs::write(
            &wit,
            format!("source_sha256: sha256:0000\nelf_sha256: {elf_hex}\n"),
        )
        .unwrap();
        assert_eq!(
            check_binary_freshness(r, "sddia-qa", &bin).as_deref(),
            Some("sddia-qa:source_drift")
        );

        // Sin testigo, fuente modificada después del binario → source_newer_than_binary
        fs::remove_file(&wit).unwrap();
        thread::sleep(Duration::from_millis(20));
        fs::write(
            r.join("SddIA/tools/sddia-qa/src/main.rs"),
            "fn main() { println!(\"new\"); }\n",
        )
        .unwrap();
        assert_eq!(
            check_binary_freshness(r, "sddia-qa", &bin).as_deref(),
            Some("sddia-qa:source_newer_than_binary")
        );
    }
}
