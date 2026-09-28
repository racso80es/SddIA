use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone)]
pub enum SandboxError {
    Escape(String),
    Io(String),
}

impl SandboxError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Escape(_) => "PROJECT_SCOPE_ESCAPE",
            Self::Io(_) => "SANDBOX_IO",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Escape(m) | Self::Io(m) => m.clone(),
        }
    }
}

pub fn resolve_under_root(root: &Path, rel: &str) -> Result<PathBuf, SandboxError> {
    let rel = rel.trim().trim_start_matches("./");
    if rel.is_empty() {
        return Err(SandboxError::Escape("empty path".into()));
    }
    if Path::new(rel).is_absolute() {
        return Err(SandboxError::Escape(format!("absolute '{rel}'")));
    }
    let mut joined = root.to_path_buf();
    for comp in Path::new(rel).components() {
        match comp {
            Component::ParentDir => return Err(SandboxError::Escape(format!("'{rel}'"))),
            Component::RootDir | Component::Prefix(_) => {
                return Err(SandboxError::Escape(format!("'{rel}'")));
            }
            Component::CurDir => {}
            Component::Normal(p) => joined.push(p),
        }
    }
    let root_canon = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    if joined.exists() {
        let canon = joined
            .canonicalize()
            .map_err(|e| SandboxError::Escape(e.to_string()))?;
        if !canon.starts_with(&root_canon) {
            return Err(SandboxError::Escape(format!("'{rel}'")));
        }
        return Ok(canon);
    }
    if !joined.starts_with(root) {
        return Err(SandboxError::Escape(format!("'{rel}'")));
    }
    Ok(joined)
}

pub fn parse_project_uri(uri: &str) -> Option<(String, String)> {
    let rest = uri.strip_prefix("project://")?;
    if let Some(path) = rest.strip_prefix("file/") {
        return Some(("file".into(), path.to_string()));
    }
    if rest == "tree" {
        return Some(("tree".into(), String::new()));
    }
    if rest == "git-status" {
        return Some(("git-status".into(), String::new()));
    }
    if let Some(key) = rest.strip_prefix("docs/") {
        return Some(("docs".into(), key.to_string()));
    }
    None
}
