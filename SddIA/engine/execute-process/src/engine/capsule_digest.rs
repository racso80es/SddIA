//! Digest determinista de fuente de cápsulas (paridad con L-BUNDLE-STALE v2).

use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

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

/// Digest SHA-256 del crate (`Cargo.toml`, `build.rs`, `src/**`) — política mínima (sin lockfile).
pub fn compute_crate_source_digest(crate_dir: &Path) -> Result<String, String> {
    let files = collect_crate_source_files(crate_dir)?;
    if files.is_empty() {
        return Err("crate sin fuentes localizables".into());
    }
    let mut lines = Vec::with_capacity(files.len());
    for f in files {
        let rel = f
            .strip_prefix(crate_dir)
            .unwrap_or(&f)
            .to_string_lossy()
            .replace('\\', "/");
        let hash = file_sha256_hex(&f)?;
        lines.push(format!("{rel}\t{hash}"));
    }
    lines.sort();
    let blob = lines.join("\n");
    let mut hasher = Sha256::new();
    hasher.update(blob.as_bytes());
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

/// Digest con workspace manifests (política B / fallback bundle).
pub fn compute_crate_source_digest_with_workspace(
    repo: &Path,
    crate_dir: &Path,
) -> Result<String, String> {
    let mut files = collect_crate_source_files(crate_dir)?;
    let ws_toml = repo.join("SddIA/Cargo.toml");
    let ws_lock = repo.join("SddIA/Cargo.lock");
    if ws_toml.is_file() {
        files.push(ws_toml);
    }
    if ws_lock.is_file() {
        files.push(ws_lock);
    }
    files.sort();
    let mut lines = Vec::with_capacity(files.len());
    for f in &files {
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

pub fn sha256_file_hex(path: &Path) -> Result<String, String> {
    Ok(format!("sha256:{}", file_sha256_hex(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn digest_stable_for_same_crate() {
        let td = tempfile::tempdir().unwrap();
        let crate_dir = td.path().join("demo-capsule");
        fs::create_dir_all(crate_dir.join("src")).unwrap();
        fs::write(
            crate_dir.join("Cargo.toml"),
            "[package]\nname = \"demo-capsule\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(crate_dir.join("src/main.rs"), "fn main() {}\n").unwrap();
        let d1 = compute_crate_source_digest(&crate_dir).unwrap();
        let d2 = compute_crate_source_digest(&crate_dir).unwrap();
        assert_eq!(d1, d2);
        assert!(d1.starts_with("sha256:"));
    }

    #[test]
    fn digest_changes_when_src_changes() {
        let td = tempfile::tempdir().unwrap();
        let crate_dir = td.path().join("demo-capsule");
        fs::create_dir_all(crate_dir.join("src")).unwrap();
        fs::write(crate_dir.join("Cargo.toml"), "[package]\nname = \"x\"\nversion=\"0.1.0\"\nedition=\"2021\"\n").unwrap();
        fs::write(crate_dir.join("src/main.rs"), "fn main() {}\n").unwrap();
        let d1 = compute_crate_source_digest(&crate_dir).unwrap();
        fs::write(crate_dir.join("src/main.rs"), "fn main() { println!(\"y\"); }\n").unwrap();
        let d2 = compute_crate_source_digest(&crate_dir).unwrap();
        assert_ne!(d1, d2);
    }
}
