//! Resolución soberana de backend Event Manager (MAN-36).
//! Precedencia: SO > bóveda instancia (`.SddIA/.dev/.env`) > bóveda global (`.dev/.env`) > default.

use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const VAR_PROVIDER: &str = "SDDIA_EVENT_MANAGER_PROVIDER";
pub const VAR_REDB_PATH: &str = "SDDIA_EVENT_MANAGER_REDB_PATH";
pub const VAR_FS_ROOT: &str = "SDDIA_EVENT_MANAGER_FS_ROOT";

pub const DEFAULT_PROVIDER: &str = "redb";
pub const DEFAULT_REDB_REL: &str = ".SddIA/runtime/events.redb";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventManagerProvider {
    Redb,
    Fs,
}

impl EventManagerProvider {
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "" | "redb" => Ok(Self::Redb),
            "fs" | "filesystem" => Ok(Self::Fs),
            other => Err(format!(
                "{VAR_PROVIDER}: valor inválido '{other}' (esperado redb|fs)"
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Redb => "redb",
            Self::Fs => "fs",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventManagerBackendConfig {
    pub provider: EventManagerProvider,
    pub redb_path: PathBuf,
    pub fs_root: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventManagerConfigError {
    Empty { variable: String },
    Traversal { variable: String, raw: String },
    EscapeRepo { variable: String, raw: String },
    FsNotAllowedInProd,
    FsRootRequired,
    InvalidProvider(String),
    Other(String),
}

impl std::fmt::Display for EventManagerConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty { variable } => write!(f, "{variable}: empty"),
            Self::Traversal { variable, raw } => {
                write!(f, "{variable}: traversal ({raw})")
            }
            Self::EscapeRepo { variable, raw } => {
                write!(f, "{variable}: escapes repo_root ({raw})")
            }
            Self::FsNotAllowedInProd => write!(f, "fs_not_allowed_in_prod"),
            Self::FsRootRequired => {
                write!(f, "{VAR_FS_ROOT}: required when provider=fs")
            }
            Self::InvalidProvider(msg) => write!(f, "{msg}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<EventManagerConfigError> for String {
    fn from(e: EventManagerConfigError) -> Self {
        e.to_string()
    }
}

/// Valida ruta según L-PATH-POLICY (relativa acotada a `repo_root`; absoluta sin `..`).
pub fn validate_configured_path(
    repo_root: &Path,
    raw: &str,
    variable: &str,
) -> Result<PathBuf, EventManagerConfigError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(EventManagerConfigError::Empty {
            variable: variable.into(),
        });
    }
    let path = PathBuf::from(trimmed);
    if path.is_absolute() {
        if path_has_parent_segment(&path) {
            return Err(EventManagerConfigError::Traversal {
                variable: variable.into(),
                raw: trimmed.into(),
            });
        }
        return Ok(path);
    }
    if path_has_parent_segment(&path) {
        return Err(EventManagerConfigError::Traversal {
            variable: variable.into(),
            raw: trimmed.into(),
        });
    }
    let joined = repo_root.join(&path);
    let normalized = normalize_lexically(&joined);
    let repo_norm = normalize_lexically(repo_root);
    if !normalized.starts_with(&repo_norm) {
        return Err(EventManagerConfigError::EscapeRepo {
            variable: variable.into(),
            raw: trimmed.into(),
        });
    }
    Ok(normalized)
}

fn path_has_parent_segment(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

fn normalize_lexically(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn parse_dotenv_file(path: &Path) -> HashMap<String, String> {
    let Ok(raw) = fs::read_to_string(path) else {
        return HashMap::new();
    };
    let mut out = HashMap::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let key = k.trim();
        if key.is_empty() {
            continue;
        }
        let mut val = v.trim().to_string();
        if (val.starts_with('"') && val.ends_with('"'))
            || (val.starts_with('\'') && val.ends_with('\''))
        {
            val = val[1..val.len() - 1].to_string();
        }
        out.insert(key.to_string(), val);
    }
    out
}

fn load_vault_layers(repo_root: &Path) -> (HashMap<String, String>, HashMap<String, String>) {
    let global = parse_dotenv_file(&repo_root.join(".dev/.env"));
    let instance = parse_dotenv_file(&repo_root.join(".SddIA/.dev/.env"));
    (global, instance)
}

/// Resuelve una variable: SO > instancia > global > `default`.
pub fn resolve_var(
    repo_root: &Path,
    key: &str,
    default: Option<&str>,
) -> Option<String> {
    if let Ok(v) = std::env::var(key) {
        let t = v.trim();
        if !t.is_empty() || std::env::var_os(key).is_some() {
            // Presente en SO (incluso vacío) gana — vacío se trata en validate.
            return Some(v);
        }
    }
    let (global, instance) = load_vault_layers(repo_root);
    if let Some(v) = instance.get(key) {
        return Some(v.clone());
    }
    if let Some(v) = global.get(key) {
        return Some(v.clone());
    }
    default.map(str::to_string)
}

/// Resuelve proveedor + rutas validadas (no abre el backend).
pub fn resolve_backend_config(
    repo_root: &Path,
) -> Result<EventManagerBackendConfig, EventManagerConfigError> {
    let provider_raw = resolve_var(repo_root, VAR_PROVIDER, Some(DEFAULT_PROVIDER))
        .unwrap_or_else(|| DEFAULT_PROVIDER.to_string());
    let provider = EventManagerProvider::parse(&provider_raw)
        .map_err(EventManagerConfigError::InvalidProvider)?;

    let redb_raw = resolve_var(repo_root, VAR_REDB_PATH, Some(DEFAULT_REDB_REL))
        .unwrap_or_else(|| DEFAULT_REDB_REL.to_string());
    let redb_path = validate_configured_path(repo_root, &redb_raw, VAR_REDB_PATH)?;

    let fs_root = match resolve_var(repo_root, VAR_FS_ROOT, None) {
        Some(raw) => Some(validate_configured_path(repo_root, &raw, VAR_FS_ROOT)?),
        None => None,
    };

    Ok(EventManagerBackendConfig {
        provider,
        redb_path,
        fs_root,
    })
}

/// Config productiva: solo Redb; FS rechazado.
pub fn resolve_productive_backend_config(
    repo_root: &Path,
) -> Result<EventManagerBackendConfig, EventManagerConfigError> {
    let cfg = resolve_backend_config(repo_root)?;
    if cfg.provider != EventManagerProvider::Redb {
        return Err(EventManagerConfigError::FsNotAllowedInProd);
    }
    Ok(cfg)
}

/// Config FS solo para test/migración (exige FS_ROOT).
pub fn resolve_fs_backend_config(
    repo_root: &Path,
) -> Result<PathBuf, EventManagerConfigError> {
    let cfg = resolve_backend_config(repo_root)?;
    let root = cfg
        .fs_root
        .ok_or(EventManagerConfigError::FsRootRequired)?;
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tempfile::tempdir;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn clear_em_env() {
        for k in [VAR_PROVIDER, VAR_REDB_PATH, VAR_FS_ROOT] {
            std::env::remove_var(k);
        }
    }

    #[test]
    fn relative_path_joins_repo_root() {
        let dir = tempdir().unwrap();
        let p = validate_configured_path(dir.path(), "data/store.redb", VAR_REDB_PATH).unwrap();
        assert_eq!(p, dir.path().join("data/store.redb"));
    }

    #[test]
    fn relative_traversal_rejected() {
        let dir = tempdir().unwrap();
        let err = validate_configured_path(dir.path(), "../outside.redb", VAR_REDB_PATH).unwrap_err();
        assert!(matches!(err, EventManagerConfigError::Traversal { .. }));
    }

    #[test]
    fn empty_rejected() {
        let dir = tempdir().unwrap();
        let err = validate_configured_path(dir.path(), "  ", VAR_REDB_PATH).unwrap_err();
        assert!(matches!(err, EventManagerConfigError::Empty { .. }));
    }

    #[test]
    fn absolute_valid_accepted() {
        let dir = tempdir().unwrap();
        let abs = dir.path().join("abs.redb");
        let p = validate_configured_path(dir.path(), abs.to_str().unwrap(), VAR_REDB_PATH).unwrap();
        assert_eq!(p, abs);
    }

    #[test]
    fn absolute_with_parent_rejected() {
        let dir = tempdir().unwrap();
        let raw = format!("{}/../escape.redb", dir.path().display());
        let err = validate_configured_path(dir.path(), &raw, VAR_REDB_PATH).unwrap_err();
        assert!(matches!(err, EventManagerConfigError::Traversal { .. }));
    }

    #[test]
    fn precedence_so_over_instance_over_global() {
        let _g = ENV_LOCK.lock().unwrap();
        clear_em_env();
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join(".dev")).unwrap();
        fs::create_dir_all(root.join(".SddIA/.dev")).unwrap();
        fs::write(
            root.join(".dev/.env"),
            format!("{VAR_REDB_PATH}=global/events.redb\n"),
        )
        .unwrap();
        fs::write(
            root.join(".SddIA/.dev/.env"),
            format!("{VAR_REDB_PATH}=instance/events.redb\n"),
        )
        .unwrap();

        let cfg = resolve_backend_config(root).unwrap();
        assert_eq!(cfg.redb_path, root.join("instance/events.redb"));

        std::env::set_var(VAR_REDB_PATH, "so/events.redb");
        let cfg_so = resolve_backend_config(root).unwrap();
        assert_eq!(cfg_so.redb_path, root.join("so/events.redb"));
        clear_em_env();
    }

    #[test]
    fn global_used_when_instance_absent() {
        let _g = ENV_LOCK.lock().unwrap();
        clear_em_env();
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join(".dev")).unwrap();
        fs::write(
            root.join(".dev/.env"),
            format!("{VAR_REDB_PATH}=only-global/events.redb\n"),
        )
        .unwrap();
        let cfg = resolve_backend_config(root).unwrap();
        assert_eq!(cfg.redb_path, root.join("only-global/events.redb"));
    }

    #[test]
    fn default_redb_when_no_vault() {
        let _g = ENV_LOCK.lock().unwrap();
        clear_em_env();
        let dir = tempdir().unwrap();
        let cfg = resolve_backend_config(dir.path()).unwrap();
        assert_eq!(cfg.provider, EventManagerProvider::Redb);
        assert_eq!(cfg.redb_path, dir.path().join(DEFAULT_REDB_REL));
    }

    #[test]
    fn productive_rejects_fs_provider() {
        let _g = ENV_LOCK.lock().unwrap();
        clear_em_env();
        let dir = tempdir().unwrap();
        std::env::set_var(VAR_PROVIDER, "fs");
        std::env::set_var(VAR_FS_ROOT, "fs-root");
        let err = resolve_productive_backend_config(dir.path()).unwrap_err();
        assert_eq!(err, EventManagerConfigError::FsNotAllowedInProd);
        clear_em_env();
    }
}
