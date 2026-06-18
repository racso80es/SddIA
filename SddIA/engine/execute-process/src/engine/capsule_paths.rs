//! Resolución SSOT de artefactos compilados (`compiled_capsules` en cumulo.paths.json).

use super::workspace::load_paths_config;
use std::path::{Path, PathBuf};

pub struct CompiledCapsuleRoots {
    pub native_root: PathBuf,
    pub wasm_root: PathBuf,
    pub profiles: Vec<String>,
}

fn default_roots(repo: &Path) -> CompiledCapsuleRoots {
    CompiledCapsuleRoots {
        native_root: repo.join("SddIA/target"),
        wasm_root: repo.join("SddIA/target/wasm32-wasip1"),
        profiles: vec!["release".into(), "debug".into()],
    }
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
            .filter_map(|p| p.as_str().map(str::trim).filter(|s| !s.is_empty()).map(str::to_string))
            .collect();
        if !parsed.is_empty() {
            roots.profiles = parsed;
        }
    }
    Ok(roots)
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

pub fn resolve_capsule_native(repo: &Path, name: &str) -> Option<PathBuf> {
    let roots = load_compiled_capsule_roots(repo).ok()?;
    for profile in &roots.profiles {
        let p = roots.native_root.join(profile).join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;

    #[test]
    fn loads_compiled_capsule_roots_from_cumulo() {
        let repo = find_repo_root().unwrap();
        let roots = load_compiled_capsule_roots(&repo).unwrap();
        assert!(roots.native_root.ends_with("SddIA/target"));
        assert!(roots.wasm_root.ends_with("wasm32-wasip1"));
        assert!(roots.profiles.contains(&"release".to_string()));
    }
}
