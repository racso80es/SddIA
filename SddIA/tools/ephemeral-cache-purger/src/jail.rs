//! Jail determinista y enumeración de candidatos de purga.

use regex::Regex;
use serde::Serialize;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;
use std::time::SystemTime;

pub const ALLOW_RE: &str = r"^/tmp/cursor-sandbox-cache(/[a-f0-9]{16,64}(/.*)?)?$";
pub const HASH_RE: &str = r"^[a-f0-9]{16,64}$";
pub const DEFAULT_TARGET: &str = "/tmp/cursor-sandbox-cache";

const VETO_PREFIXES: &[&str] = &[
    "/bin", "/usr", "/etc", "/var", "/lib", "/lib64", "/boot", "/dev", "/proc", "/sys", "/home",
    "/root", "/opt", "/sbin",
];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct NodeError {
    pub path: String,
    pub code: String,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct PurgePlan {
    pub target_dir: PathBuf,
    pub simulated: bool,
    pub candidate_targets: Vec<PathBuf>,
    pub bytes_scanned: u64,
    pub purged_directories_count: u64,
    pub purged_files_count: u64,
    pub errors: Vec<NodeError>,
}

#[derive(Debug)]
pub enum JailError {
    Security(String),
    Io(String),
}

impl JailError {
    pub fn code_and_msg(&self) -> (&'static str, String) {
        match self {
            JailError::Security(m) => ("SECURITY_VIOLATION_PATH_OUT_OF_BOUNDS", m.clone()),
            JailError::Io(m) => ("IO_ERROR", m.clone()),
        }
    }
}

fn allow_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(ALLOW_RE).expect("ALLOW_RE"))
}

fn hash_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(HASH_RE).expect("HASH_RE"))
}

pub fn normalize_abs(raw: &str) -> Result<PathBuf, JailError> {
    let t = raw.trim();
    if t.is_empty() {
        return Err(JailError::Security("ruta vacía".into()));
    }
    let p = PathBuf::from(t);
    if !p.is_absolute() {
        return Err(JailError::Security(format!("ruta no absoluta: {t}")));
    }
    for c in p.components() {
        match c {
            Component::ParentDir | Component::CurDir => {
                return Err(JailError::Security(format!("componente léxico ilegal: {t}")));
            }
            _ => {}
        }
    }
    let s = p.to_string_lossy();
    let trimmed = s.trim_end_matches('/');
    let canon_s = if trimmed.is_empty() { "/" } else { trimmed };
    Ok(PathBuf::from(canon_s))
}

fn is_veto_prefix(path: &Path) -> bool {
    let s = path.to_string_lossy();
    if s == "/" {
        return true;
    }
    for prefix in VETO_PREFIXES {
        if s == *prefix || s.starts_with(&format!("{prefix}/")) {
            return true;
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        let home = home.trim_end_matches('/');
        if !home.is_empty() && (s == home || s.starts_with(&format!("{home}/"))) {
            return true;
        }
    }
    false
}

/// Walk de componentes con `symlink_metadata`. No sigue enlaces.
pub fn assert_jail(path: &Path) -> Result<(), JailError> {
    if is_veto_prefix(path) {
        return Err(JailError::Security(format!(
            "SECURITY_VIOLATION_PATH_OUT_OF_BOUNDS: {}",
            path.display()
        )));
    }
    let s = path.to_string_lossy();
    if !allow_re().is_match(&s) {
        return Err(JailError::Security(format!(
            "SECURITY_VIOLATION_PATH_OUT_OF_BOUNDS: {}",
            path.display()
        )));
    }
    let mut acc = PathBuf::new();
    for c in path.components() {
        acc.push(c);
        if acc == Path::new("/") {
            continue;
        }
        match fs::symlink_metadata(&acc) {
            Ok(meta) if meta.file_type().is_symlink() => {
                return Err(JailError::Security(format!(
                    "SECURITY_VIOLATION_SYMLINK: {}",
                    acc.display()
                )));
            }
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => break,
            Err(e) => {
                return Err(JailError::Io(format!("{}: {e}", acc.display())));
            }
        }
    }
    Ok(())
}

pub fn human_size(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    let n = bytes as f64;
    if n >= GIB {
        format!("{:.1} GB", n / GIB)
    } else if n >= MIB {
        format!("{:.1} MB", n / MIB)
    } else if n >= KIB {
        format!("{:.1} KB", n / KIB)
    } else {
        format!("{bytes} B")
    }
}

fn mtime_hours_ago(path: &Path) -> Option<f64> {
    let meta = fs::metadata(path).ok()?;
    let modified = meta.modified().ok()?;
    let now = SystemTime::now();
    let dur = now.duration_since(modified).ok()?;
    Some(dur.as_secs_f64() / 3600.0)
}

fn older_enough(path: &Path, older_than_hours: f64) -> bool {
    if older_than_hours <= 0.0 {
        return true;
    }
    mtime_hours_ago(path).map(|h| h >= older_than_hours).unwrap_or(false)
}

fn push_io(errors: &mut Vec<NodeError>, path: &Path, code: &str, detail: String) {
    errors.push(NodeError {
        path: path.to_string_lossy().into_owned(),
        code: code.into(),
        detail,
    });
}

/// Recorre sin seguir symlinks. Symlink interno → error, no se cuenta.
pub fn scan_tree(path: &Path, errors: &mut Vec<NodeError>) -> (u64, u64, u64) {
    let mut bytes = 0u64;
    let mut dirs = 0u64;
    let mut files = 0u64;
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) => {
            let code = if e.kind() == std::io::ErrorKind::PermissionDenied {
                "IO_PERMISSION_DENIED"
            } else {
                "IO_ERROR"
            };
            push_io(errors, path, code, e.to_string());
            return (0, 0, 0);
        }
    };
    if meta.file_type().is_symlink() {
        push_io(
            errors,
            path,
            "SECURITY_VIOLATION_SYMLINK",
            "symlink en árbol candidato".into(),
        );
        return (0, 0, 0);
    }
    if meta.is_dir() {
        dirs += 1;
        let rd = match fs::read_dir(path) {
            Ok(r) => r,
            Err(e) => {
                let code = if e.kind() == std::io::ErrorKind::PermissionDenied {
                    "IO_PERMISSION_DENIED"
                } else {
                    "IO_ERROR"
                };
                push_io(errors, path, code, e.to_string());
                return (bytes, dirs, files);
            }
        };
        for ent in rd.flatten() {
            let (b, d, f) = scan_tree(&ent.path(), errors);
            bytes += b;
            dirs += d;
            files += f;
        }
    } else {
        files += 1;
        bytes += meta.len();
    }
    (bytes, dirs, files)
}

fn collect_from_root(
    root: &Path,
    purge_sandbox_root: bool,
    older_than_hours: f64,
    errors: &mut Vec<NodeError>,
) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let rd = match fs::read_dir(root) {
        Ok(r) => r,
        Err(e) => {
            let code = if e.kind() == std::io::ErrorKind::PermissionDenied {
                "IO_PERMISSION_DENIED"
            } else {
                "IO_ERROR"
            };
            push_io(errors, root, code, e.to_string());
            return out;
        }
    };
    for ent in rd.flatten() {
        let name = ent.file_name();
        let Some(name_s) = name.to_str() else {
            continue;
        };
        if !hash_re().is_match(name_s) {
            continue;
        }
        let hash_dir = ent.path();
        if let Ok(meta) = fs::symlink_metadata(&hash_dir) {
            if meta.file_type().is_symlink() {
                push_io(
                    errors,
                    &hash_dir,
                    "SECURITY_VIOLATION_SYMLINK",
                    "sandbox hash es symlink".into(),
                );
                continue;
            }
        }
        let candidate = if purge_sandbox_root {
            hash_dir
        } else {
            hash_dir.join("cargo-target")
        };
        if !candidate.exists() {
            continue;
        }
        if let Err(e) = assert_jail(&candidate) {
            let (code, msg) = e.code_and_msg();
            push_io(errors, &candidate, code, msg);
            continue;
        }
        if !older_enough(&candidate, older_than_hours) {
            continue;
        }
        out.push(candidate);
    }
    out
}

pub fn plan_purge(
    target_raw: &str,
    simulate: bool,
    older_than_hours: f64,
    purge_sandbox_root: bool,
) -> Result<PurgePlan, JailError> {
    let target_dir = normalize_abs(target_raw)?;
    assert_jail(&target_dir)?;
    let mut errors = Vec::new();
    let mut candidates = Vec::new();

    let s = target_dir.to_string_lossy();
    if s == DEFAULT_TARGET {
        candidates = collect_from_root(&target_dir, purge_sandbox_root, older_than_hours, &mut errors);
    } else if hash_re().is_match(
        target_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(""),
    ) && target_dir.parent().map(|p| p.to_string_lossy() == DEFAULT_TARGET).unwrap_or(false)
    {
        let candidate = if purge_sandbox_root {
            target_dir.clone()
        } else {
            target_dir.join("cargo-target")
        };
        if candidate.exists() {
            assert_jail(&candidate)?;
            if older_enough(&candidate, older_than_hours) {
                candidates.push(candidate);
            }
        }
    } else {
        if target_dir.exists() && older_enough(&target_dir, older_than_hours) {
            candidates.push(target_dir.clone());
        }
    }

    let mut bytes_scanned = 0u64;
    let mut dirs_total = 0u64;
    let mut files_total = 0u64;
    let mut admitted = Vec::new();
    for c in candidates {
        let err_before = errors.len();
        let (b, d, f) = scan_tree(&c, &mut errors);
        let new_sec = errors[err_before..]
            .iter()
            .any(|e| e.code.starts_with("SECURITY_"));
        if new_sec {
            continue;
        }
        bytes_scanned += b;
        dirs_total += d;
        files_total += f;
        admitted.push(c);
    }

    let mut purged_directories_count = 0u64;
    let mut purged_files_count = 0u64;
    if !simulate {
        for c in &admitted {
            let (b, d, f) = scan_tree(c, &mut Vec::new());
            let _ = b;
            match fs::remove_dir_all(c) {
                Ok(()) => {
                    purged_directories_count += d;
                    purged_files_count += f;
                }
                Err(e) => {
                    let code = if e.kind() == std::io::ErrorKind::PermissionDenied {
                        "IO_PERMISSION_DENIED"
                    } else {
                        "IO_ERROR"
                    };
                    push_io(&mut errors, c, code, e.to_string());
                }
            }
        }
        if !admitted.is_empty()
            && purged_directories_count == 0
            && purged_files_count == 0
            && errors.iter().any(|e| e.code == "IO_PERMISSION_DENIED")
        {
            return Ok(PurgePlan {
                target_dir,
                simulated: false,
                candidate_targets: admitted,
                bytes_scanned,
                purged_directories_count: 0,
                purged_files_count: 0,
                errors,
            });
        }
    } else {
        let _ = (dirs_total, files_total);
    }

    Ok(PurgePlan {
        target_dir,
        simulated: simulate,
        candidate_targets: admitted,
        bytes_scanned,
        purged_directories_count,
        purged_files_count,
        errors,
    })
}

pub fn path_in_allowlist(s: &str) -> bool {
    allow_re().is_match(s.trim_end_matches('/')) || allow_re().is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_HASH: &str = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";

    fn fixture_root() -> PathBuf {
        PathBuf::from(DEFAULT_TARGET).join(TEST_HASH)
    }

    fn reset_fixture() -> PathBuf {
        let root = fixture_root();
        let cargo = root.join("cargo-target");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&cargo).expect("mkdir fixture");
        fs::write(cargo.join("artifact.bin"), vec![0u8; 4096]).expect("write");
        root
    }

    #[test]
    fn accept_default_prefix() {
        assert!(path_in_allowlist(DEFAULT_TARGET));
        assert!(path_in_allowlist(&format!(
            "{DEFAULT_TARGET}/{TEST_HASH}/cargo-target"
        )));
        let p = normalize_abs(DEFAULT_TARGET).unwrap();
        assert!(assert_jail(&p).is_ok());
    }

    #[test]
    fn reject_root_home_relative() {
        assert!(normalize_abs("/").and_then(|p| assert_jail(&p)).is_err());
        assert!(normalize_abs("/home").and_then(|p| assert_jail(&p)).is_err());
        assert!(normalize_abs("../tmp").is_err());
        assert!(normalize_abs("/tmp/cursor-sandbox-cache/../etc").is_err());
        assert!(normalize_abs("/etc/passwd").and_then(|p| assert_jail(&p)).is_err());
        assert!(!path_in_allowlist("/tmp/other"));
    }

    #[test]
    fn simulate_does_not_delete() {
        let root = reset_fixture();
        let cargo = root.join("cargo-target");
        let plan = plan_purge(cargo.to_str().unwrap(), true, 0.0, false).expect("plan");
        assert!(plan.simulated);
        assert!(plan.candidate_targets.iter().any(|p| p == &cargo));
        assert!(plan.bytes_scanned >= 4096);
        assert!(cargo.join("artifact.bin").is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn purge_deletes_fixture() {
        let root = reset_fixture();
        let cargo = root.join("cargo-target");
        let plan = plan_purge(cargo.to_str().unwrap(), false, 0.0, false).expect("purge");
        assert!(!plan.simulated);
        assert!(!cargo.exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn reject_symlink_component() {
        let root = reset_fixture();
        let link = root.join("cargo-target-link");
        let _ = fs::remove_file(&link);
        std::os::unix::fs::symlink("/etc", &link).expect("symlink");
        let err = assert_jail(&link).expect_err("symlink");
        match err {
            JailError::Security(m) => assert!(m.contains("SYMLINK")),
            JailError::Io(_) => panic!("expected security"),
        }
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn human_size_binary_gb() {
        assert_eq!(human_size(8 * 1024 * 1024 * 1024), "8.0 GB");
    }
}
