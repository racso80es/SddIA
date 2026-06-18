use std::path::{Path, PathBuf};

/// Localiza la raíz del workspace (presencia de `SddIA/core/cumulo.paths.json`).
pub fn find_repo_root() -> Result<PathBuf, String> {
    let mut dir = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        if dir.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    Err("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)".into())
}

pub fn repo_root_from(start: &Path) -> Result<PathBuf, String> {
    let mut dir = start.to_path_buf();
    loop {
        if dir.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    Err("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_repo_from_cwd() {
        let root = find_repo_root().expect("repo root");
        assert!(root.join("SddIA/Cargo.toml").is_file());
    }
}
