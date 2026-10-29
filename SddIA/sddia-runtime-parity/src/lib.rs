//! Paridad binario/fuente para `execute-process` y `kalma2-bridge` (MAN-25 / KZ-CA2–CA3).

use regex::Regex;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const LIVE_ORGANS: &[&str] = &["execute-process", "kalma2-bridge"];

pub fn allow_stale_runtime() -> bool {
    std::env::var("SDDIA_ALLOW_STALE_RUNTIME")
        .ok()
        .map(|s| {
            matches!(
                s.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
pub struct CapsuleWitness {
    pub source_sha256: String,
    pub elf_sha256: String,
}

pub fn read_capsule_witness(elf_path: &Path) -> Result<CapsuleWitness, String> {
    let wit_path = PathBuf::from(format!("{}.sha256", elf_path.display()));
    let text = fs::read_to_string(&wit_path).map_err(|e| format!("testigo ausente: {e}"))?;
    let mut source = None;
    let mut elf = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("source_sha256:") {
            source = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("elf_sha256:") {
            elf = Some(rest.trim().to_string());
        }
    }
    Ok(CapsuleWitness {
        source_sha256: source.ok_or("testigo sin source_sha256")?,
        elf_sha256: elf.ok_or("testigo sin elf_sha256")?,
    })
}

pub fn sha256_file_hex(path: &Path) -> Result<String, String> {
    Ok(format!("sha256:{}", file_sha256_hex(path)?))
}

fn file_sha256_hex(path: &Path) -> Result<String, String> {
    let mut f = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = f.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub fn resolve_organ_crate_dir(repo: &Path, name: &str) -> Option<PathBuf> {
    if name == "execute-process" {
        let p = repo.join("SddIA/engine/execute-process");
        return p.join("Cargo.toml").is_file().then_some(p);
    }
    if name == "kalma2-bridge" {
        for rel in ["SddIA/interfaces/kalma2-bridge", "SddIA/engine/kalma2-bridge"] {
            let p = repo.join(rel);
            if p.join("Cargo.toml").is_file() {
                return Some(p);
            }
        }
    }
    None
}

pub fn resolve_release_elf(repo: &Path, name: &str) -> Option<PathBuf> {
    let p = repo.join("SddIA/target/release").join(name);
    p.is_file().then_some(p)
}

fn collect_crate_source_files(crate_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let manifest = crate_dir.join("Cargo.toml");
    if manifest.is_file() {
        files.push(manifest);
    }
    let build_rs = crate_dir.join("build.rs");
    if build_rs.is_file() {
        files.push(build_rs);
    }
    let src = crate_dir.join("src");
    if src.is_dir() {
        collect_files_recursive(&src, &mut files)?;
    }
    files.sort();
    Ok(files)
}

fn collect_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    for ent in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let ent = ent.map_err(|e| e.to_string())?;
        let p = ent.path();
        if p.is_dir() {
            if p.file_name().and_then(|n| n.to_str()) == Some("target") {
                continue;
            }
            collect_files_recursive(&p, out)?;
        } else if p.is_file() {
            out.push(p);
        }
    }
    Ok(())
}

fn collect_path_dep_dirs(crate_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let path_re = Regex::new(r#"path\s*=\s*"([^"]+)""#).map_err(|e| e.to_string())?;
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut queue = vec![crate_dir.to_path_buf()];
    let mut ordered = Vec::new();
    while let Some(dir) = queue.pop() {
        let canon = fs::canonicalize(&dir).unwrap_or(dir);
        if !seen.insert(canon.clone()) {
            continue;
        }
        ordered.push(canon.clone());
        let toml = canon.join("Cargo.toml");
        if !toml.is_file() {
            continue;
        }
        let text = fs::read_to_string(&toml).map_err(|e| e.to_string())?;
        for cap in path_re.captures_iter(&text) {
            let rel = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            if rel.is_empty() {
                continue;
            }
            let dep = if Path::new(rel).is_absolute() {
                PathBuf::from(rel)
            } else {
                canon.join(rel)
            };
            if dep.join("Cargo.toml").is_file() {
                queue.push(dep);
            }
        }
    }
    Ok(ordered)
}

fn collect_bundle_source_files(repo: &Path, crate_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for dir in collect_path_dep_dirs(crate_dir)? {
        files.extend(collect_crate_source_files(&dir)?);
    }
    let ws_toml = repo.join("SddIA/Cargo.toml");
    let ws_lock = repo.join("SddIA/Cargo.lock");
    if ws_toml.is_file() {
        files.push(ws_toml);
    }
    if ws_lock.is_file() {
        files.push(ws_lock);
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn digest_from_file_list(repo: &Path, files: &mut [PathBuf]) -> Result<String, String> {
    files.sort();
    let mut lines = Vec::with_capacity(files.len());
    for f in files {
        let rel = f
            .strip_prefix(repo)
            .unwrap_or(f)
            .to_string_lossy()
            .replace('\\', "/");
        let hash = file_sha256_hex(f)?;
        lines.push(format!("{rel}\t{hash}"));
    }
    lines.sort();
    let blob = lines.join("\n");
    let mut hasher = Sha256::new();
    hasher.update(blob.as_bytes());
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

pub fn compute_organ_source_digest(repo: &Path, name: &str) -> Result<String, String> {
    let crate_dir = resolve_organ_crate_dir(repo, name)
        .ok_or_else(|| format!("crate no localizable: {name}"))?;
    let mut files = collect_bundle_source_files(repo, &crate_dir)?;
    digest_from_file_list(repo, &mut files)
}

fn check_organ(
    repo: &Path,
    name: &str,
    live_elf: Option<&Path>,
) -> Result<Vec<String>, String> {
    let mut reasons = Vec::new();
    let Some(elf) = resolve_release_elf(repo, name) else {
        reasons.push(format!("{name}:elf_release_ausente"));
        return Ok(reasons);
    };
    let witness = match read_capsule_witness(&elf) {
        Ok(w) => w,
        Err(e) => {
            reasons.push(format!("{name}:witness:{e}"));
            return Ok(reasons);
        }
    };
    let current_source = compute_organ_source_digest(repo, name)?;
    if witness.source_sha256 != current_source {
        reasons.push(format!("{name}:source_drift"));
    }
    let disk_elf = sha256_file_hex(&elf)?;
    if witness.elf_sha256 != disk_elf {
        reasons.push(format!("{name}:elf_drift"));
    }
    if let Some(live) = live_elf {
        if live.is_file() {
            let live_hash = sha256_file_hex(live)?;
            if witness.elf_sha256 != live_hash {
                reasons.push(format!("{name}:live_drift"));
            }
        }
    }
    Ok(reasons)
}

/// Informe JSON para system-health y gate de despacho.
pub fn assess_runtime_binary_parity(
    repo: &Path,
    live_bridge: Option<&Path>,
    live_execute: Option<&Path>,
) -> Result<Value, String> {
    let mut stale: Vec<String> = Vec::new();
    for (name, live) in [
        ("execute-process", live_execute),
        ("kalma2-bridge", live_bridge),
    ] {
        stale.extend(check_organ(repo, name, live)?);
    }
    let ok = stale.is_empty();
    Ok(json!({
        "ok": ok,
        "stale": stale,
        "organs": LIVE_ORGANS,
        "profile": "release",
        "allow_stale_env": "SDDIA_ALLOW_STALE_RUNTIME",
    }))
}

pub fn merge_touches_live_organs(changed_paths: &[String]) -> bool {
    changed_paths.iter().any(|p| {
        let n = p.replace('\\', "/");
        n.starts_with("SddIA/engine/execute-process/")
            || n.starts_with("SddIA/interfaces/kalma2-bridge/")
            || n.starts_with("SddIA/engine/kalma2-bridge/")
            || n.starts_with("SddIA/sddia-")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_touch_detects_organ_paths() {
        assert!(merge_touches_live_organs(&[
            "SddIA/engine/execute-process/src/main.rs".into()
        ]));
        assert!(!merge_touches_live_organs(&["README.md".into()]));
    }
}
