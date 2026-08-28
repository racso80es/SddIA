//! Resolución SSOT de artefactos compilados (`compiled_capsules` en cumulo.paths.json).

use super::capsule_digest::{compute_crate_source_digest, sha256_file_hex};
use super::workspace::load_paths_config;
use crate::core::parser::parse_frontmatter;
use std::fs;
use std::path::{Path, PathBuf};

pub struct CompiledCapsuleRoots {
    pub native_root: PathBuf,
    pub wasm_root: PathBuf,
    pub profiles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapsuleResolveError {
    NotFound,
    StaleHash { message: String },
}

pub struct CapsuleWitness {
    pub source_sha256: String,
    pub elf_sha256: String,
}

fn default_roots(repo: &Path) -> CompiledCapsuleRoots {
    CompiledCapsuleRoots {
        native_root: repo.join("SddIA/target"),
        wasm_root: repo.join("SddIA/target/wasm32-wasip1"),
        profiles: vec!["release".into(), "debug".into()],
    }
}

pub fn anchor_enabled() -> bool {
    std::env::var("SDDIA_CAPSULE_ANCHOR")
        .map(|v| matches!(v.trim().to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

pub fn load_compiled_capsule_roots(repo: &Path) -> Result<CompiledCapsuleRoots, String> {
    let cfg = load_paths_config(repo)?;
    let Some(cc) = cfg.get("compiled_capsules") else {
        return Ok(default_roots(repo));
    };
    let mut roots = default_roots(repo);
    if let Some(native) = cc.get("native_root").and_then(|v| v.as_str()) {
        roots.native_root = repo.join(native.trim().trim_start_matches("./"));
    }
    if let Some(wasm) = cc.get("wasm_root").and_then(|v| v.as_str()) {
        roots.wasm_root = repo.join(wasm.trim().trim_start_matches("./"));
    }
    if let Some(profiles) = cc.get("profiles").and_then(|v| v.as_array()) {
        let parsed: Vec<String> = profiles
            .iter()
            .filter_map(|p| {
                p.as_str()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
            })
            .collect();
        if !parsed.is_empty() {
            roots.profiles = parsed;
        }
    }
    Ok(roots)
}

fn resolve_capsule_native_legacy(repo: &Path, name: &str) -> Option<PathBuf> {
    let roots = load_compiled_capsule_roots(repo).ok()?;
    for profile in &roots.profiles {
        let p = roots.native_root.join(profile).join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn resolve_capsule_crate_dir(repo: &Path, name: &str) -> Option<PathBuf> {
    for class in ["tools", "skills", "daemons"] {
        let p = repo.join("SddIA").join(class).join(name);
        if p.join("Cargo.toml").is_file() {
            return Some(p);
        }
    }
    if name == "execute-process" || name == "kalma2-bridge" {
        let p = repo.join("SddIA/engine").join(name);
        if p.join("Cargo.toml").is_file() {
            return Some(p);
        }
    }
    None
}

fn resolve_capsule_genome(repo: &Path, name: &str) -> Option<PathBuf> {
    for class in ["tools", "skills", "daemons"] {
        let p = repo.join("SddIA").join(class).join(format!("{name}.md"));
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn fm_string(fm: &std::collections::HashMap<String, serde_yaml::Value>, key: &str) -> Option<String> {
    fm.get(key).and_then(|v| match v {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        serde_yaml::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    })
}

pub fn read_capsule_witness(elf_path: &Path) -> Result<CapsuleWitness, String> {
    let wit_path = PathBuf::from(format!("{}.sha256", elf_path.display()));
    let text = fs::read_to_string(&wit_path).map_err(|e| e.to_string())?;
    let mut source = None;
    let mut elf = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("source_sha256:") {
            source = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("elf_sha256:") {
            elf = Some(rest.trim().to_string());
        }
    }
    let source_sha256 = source.ok_or_else(|| "testigo sin source_sha256".to_string())?;
    let elf_sha256 = elf.ok_or_else(|| "testigo sin elf_sha256".to_string())?;
    Ok(CapsuleWitness {
        source_sha256,
        elf_sha256,
    })
}

pub fn write_capsule_witness(elf_path: &Path, source_sha256: &str) -> Result<(), String> {
    let elf_sha256 = sha256_file_hex(elf_path)?;
    let wit_path = PathBuf::from(format!("{}.sha256", elf_path.display()));
    let body = format!("source_sha256: {source_sha256}\nelf_sha256: {elf_sha256}\n");
    fs::write(&wit_path, body).map_err(|e| e.to_string())
}

fn genome_source_sha256(repo: &Path, name: &str) -> Result<String, String> {
    let genome = resolve_capsule_genome(repo, name)
        .ok_or_else(|| format!("genome ausente para cápsula '{name}'"))?;
    let fm = parse_frontmatter(&genome)?;
    fm_string(&fm, "source_sha256")
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| format!("genome-missing-source-sha256: {name}"))
}

fn artifact_satisfies_anchor(repo: &Path, name: &str, elf: &Path) -> Result<(), String> {
    let expected_source = genome_source_sha256(repo, name)?;
    let witness = read_capsule_witness(elf)?;
    let actual_elf = sha256_file_hex(elf)?;
    if witness.elf_sha256 != actual_elf {
        return Err(format!(
            "capsule-stale-hash: {name} — testigo elf_sha256 no coincide con ELF en disco"
        ));
    }
    if witness.source_sha256 != expected_source {
        return Err(format!(
            "capsule-stale-hash: {name} — genoma {expected_source} / testigo {}",
            witness.source_sha256
        ));
    }
    Ok(())
}

/// Resolución con aduana de anclaje (genoma + testigo + ELF).
pub fn resolve_capsule_native_anchored(repo: &Path, name: &str) -> Result<PathBuf, String> {
    let roots = load_compiled_capsule_roots(repo)?;
    for profile in &roots.profiles {
        let elf = roots.native_root.join(profile).join(name);
        if !elf.is_file() {
            continue;
        }
        match artifact_satisfies_anchor(repo, name, &elf) {
            Ok(()) => return Ok(elf),
            Err(e) if e.starts_with("capsule-stale-hash:") => return Err(e),
            Err(e) => {
                return Err(format!("capsule-stale-hash: {name} — {e}"));
            }
        }
    }
    Err(format!("capsule-stale-hash: {name} — artefacto ausente o sin testigo conforme"))
}

pub fn resolve_capsule_native(repo: &Path, name: &str) -> Option<PathBuf> {
    if anchor_enabled() {
        resolve_capsule_native_anchored(repo, name).ok()
    } else {
        resolve_capsule_native_legacy(repo, name)
    }
}

pub fn resolve_capsule_native_checked(repo: &Path, name: &str) -> Result<PathBuf, CapsuleResolveError> {
    if anchor_enabled() {
        resolve_capsule_native_anchored(repo, name).map_err(|msg| CapsuleResolveError::StaleHash {
            message: msg,
        })
    } else if let Some(p) = resolve_capsule_native_legacy(repo, name) {
        Ok(p)
    } else {
        Err(CapsuleResolveError::NotFound)
    }
}

pub fn resolve_capsule_wasm(repo: &Path, name: &str) -> Option<PathBuf> {
    let roots = load_compiled_capsule_roots(repo).ok()?;
    for profile in &roots.profiles {
        let p = roots.wasm_root.join(profile).join(format!("{name}.wasm"));
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

/// Localiza el directorio crate de una cápsula indexada.
pub fn locate_capsule_crate_dir(repo: &Path, name: &str) -> Option<PathBuf> {
    resolve_capsule_crate_dir(repo, name)
}

/// Calcula digest de fuente para sellar genoma/testigo.
pub fn compute_capsule_source_digest(repo: &Path, name: &str) -> Result<String, String> {
    let crate_dir = resolve_capsule_crate_dir(repo, name)
        .ok_or_else(|| format!("crate no localizable: {name}"))?;
    compute_crate_source_digest(&crate_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_min_crate(root: &Path, class: &str, name: &str) {
        let crate_dir = root.join("SddIA").join(class).join(name);
        fs::create_dir_all(crate_dir.join("src")).unwrap();
        fs::write(
            crate_dir.join("Cargo.toml"),
            format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
        )
        .unwrap();
        fs::write(crate_dir.join("src/main.rs"), "fn main() {}\n").unwrap();
        let genome = root.join("SddIA").join(class).join(format!("{name}.md"));
        fs::create_dir_all(genome.parent().unwrap()).unwrap();
        fs::write(
            &genome,
            format!(
                "---\nname: {name}\nsource_sha256: sha256:placeholder\n---\n\n# {name}\n"
            ),
        )
        .unwrap();
    }

    fn write_cumulo(root: &Path) {
        fs::create_dir_all(root.join("SddIA/core")).unwrap();
        fs::write(
            root.join("SddIA/core/cumulo.paths.json"),
            r#"{"compiled_capsules":{"native_root":"SddIA/target","wasm_root":"SddIA/target/wasm32-wasip1","profiles":["release","debug"]}}"#,
        )
        .unwrap();
    }

    #[test]
    fn legacy_returns_first_profile_file() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        fs::create_dir_all(root.join("SddIA/target/release")).unwrap();
        fs::create_dir_all(root.join("SddIA/target/debug")).unwrap();
        fs::write(root.join("SddIA/target/release/demo-tool"), b"elf-release").unwrap();
        fs::write(root.join("SddIA/target/debug/demo-tool"), b"elf-debug").unwrap();
        std::env::remove_var("SDDIA_CAPSULE_ANCHOR");
        let p = resolve_capsule_native_legacy(root, "demo-tool").unwrap();
        assert!(p.ends_with("release/demo-tool"));
    }

    #[test]
    fn anchored_rejects_without_witness() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        write_min_crate(root, "tools", "demo-tool");
        fs::create_dir_all(root.join("SddIA/target/release")).unwrap();
        fs::write(root.join("SddIA/target/release/demo-tool"), b"elf").unwrap();
        std::env::set_var("SDDIA_CAPSULE_ANCHOR", "1");
        let err = resolve_capsule_native_anchored(root, "demo-tool").unwrap_err();
        assert!(err.contains("capsule-stale-hash"));
        std::env::remove_var("SDDIA_CAPSULE_ANCHOR");
    }

    #[test]
    fn anchored_accepts_conformant_artifact() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_cumulo(root);
        write_min_crate(root, "tools", "demo-tool");
        let digest = compute_capsule_source_digest(root, "demo-tool").unwrap();
        let genome = root.join("SddIA/tools/demo-tool.md");
        fs::write(
            &genome,
            format!("---\nname: demo-tool\nsource_sha256: {digest}\n---\n\n# demo-tool\n"),
        )
        .unwrap();
        fs::create_dir_all(root.join("SddIA/target/release")).unwrap();
        let elf = root.join("SddIA/target/release/demo-tool");
        fs::write(&elf, b"elf-v1").unwrap();
        write_capsule_witness(&elf, &digest).unwrap();
        std::env::set_var("SDDIA_CAPSULE_ANCHOR", "1");
        let p = resolve_capsule_native_anchored(root, "demo-tool").unwrap();
        assert_eq!(p, elf);
        std::env::remove_var("SDDIA_CAPSULE_ANCHOR");
    }
}
