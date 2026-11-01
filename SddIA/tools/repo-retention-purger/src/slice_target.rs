//! Slice A — generaciones Cargo en `SddIA/target`.

use crate::plan::PlanCandidate;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const NATIVE_BINS: &str = "SddIA/target/native-debug-bins";

fn rel_from_repo(repo: &Path, abs: &Path) -> Option<String> {
    abs.strip_prefix(repo)
        .ok()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
}

fn mtime(path: &Path) -> SystemTime {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

pub fn dep_logical_name(file_name: &str) -> String {
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);
    if let Some(idx) = stem.rfind('-') {
        let suffix = &stem[idx + 1..];
        if suffix.len() >= 8 && suffix.chars().all(|c| c.is_ascii_hexdigit()) {
            return stem[..idx].to_string();
        }
    }
    stem.to_string()
}

fn scan_deps_dir(repo: &Path, deps: &Path, profile: &str) -> Result<Vec<PlanCandidate>, String> {
    if !deps.is_dir() {
        return Ok(Vec::new());
    }
    let mut groups: HashMap<String, Vec<(PathBuf, SystemTime, u64)>> = HashMap::new();
    for entry in fs::read_dir(deps).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let key = dep_logical_name(&name);
        let bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let t = mtime(&path);
        groups.entry(key).or_default().push((path, t, bytes));
    }
    let mut out = Vec::new();
    for (_key, mut files) in groups {
        if files.len() <= 1 {
            continue;
        }
        files.sort_by_key(|(_, t, _)| *t);
        let keep = files.last().expect("non-empty");
        for (path, _, bytes) in files.iter().take(files.len() - 1) {
            let Some(rel) = rel_from_repo(repo, path) else {
                continue;
            };
            out.push(PlanCandidate {
                path: rel,
                bytes: *bytes,
                policy: format!("keep-latest-dep-{profile}"),
                kept: false,
            });
        }
        if let Some(rel) = rel_from_repo(repo, &keep.0) {
            out.push(PlanCandidate {
                path: rel,
                bytes: keep.2,
                policy: format!("keep-latest-dep-{profile}"),
                kept: true,
            });
        }
    }
    Ok(out)
}

fn dir_size(path: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_file() {
                total += fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
            } else if p.is_dir() {
                total += dir_size(&p);
            }
        }
    }
    total
}

fn scan_incremental(repo: &Path, inc_root: &Path) -> Result<Vec<PlanCandidate>, String> {
    if !inc_root.is_dir() {
        return Ok(Vec::new());
    }
    let mut sessions: Vec<(PathBuf, SystemTime)> = Vec::new();
    for entry in fs::read_dir(inc_root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.path().is_dir() {
            continue;
        }
        sessions.push((entry.path(), mtime(&entry.path())));
    }
    sessions.sort_by_key(|(_, t)| *t);
    if sessions.len() <= 2 {
        return Ok(Vec::new());
    }
    let keep_n = 2usize;
    let drop_count = sessions.len().saturating_sub(keep_n);
    let mut out = Vec::new();
    for (path, _) in sessions.iter().take(drop_count) {
        let bytes = dir_size(path);
        let Some(rel) = rel_from_repo(repo, path) else {
            continue;
        };
        out.push(PlanCandidate {
            path: rel,
            bytes,
            policy: "keep-two-incremental-sessions".into(),
            kept: false,
        });
    }
    for (path, _) in sessions.iter().skip(drop_count) {
        let Some(rel) = rel_from_repo(repo, path) else {
            continue;
        };
        out.push(PlanCandidate {
            path: rel,
            bytes: dir_size(path),
            policy: "keep-two-incremental-sessions".into(),
            kept: true,
        });
    }
    Ok(out)
}

pub fn enumerate(repo: &Path) -> Result<Vec<PlanCandidate>, String> {
    let target = repo.join("SddIA/target");
    let mut all = Vec::new();
    all.extend(scan_deps_dir(repo, &target.join("debug/deps"), "debug")?);
    all.extend(scan_deps_dir(repo, &target.join("release/deps"), "release")?);
    all.extend(scan_incremental(repo, &target.join("debug/incremental"))?);
    Ok(all
        .into_iter()
        .filter(|c| !c.path.contains(NATIVE_BINS))
        .collect())
}

pub fn native_debug_bin_exists(repo: &Path) -> bool {
    repo.join(NATIVE_BINS)
        .join("execute-process")
        .is_file()
        || repo
            .join(NATIVE_BINS)
            .read_dir()
            .map(|mut d| d.next().is_some())
            .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn dep_groups_keep_newest() {
        let repo = std::env::temp_dir().join(format!("ret-target-{}", std::process::id()));
        let _ = fs::remove_dir_all(&repo);
        fs::create_dir_all(repo.join("SddIA")).unwrap();
        fs::write(repo.join("SddIA/Cargo.toml"), "[workspace]\n").unwrap();
        let deps = repo.join("SddIA/target/debug/deps");
        fs::create_dir_all(&deps).unwrap();
        fs::write(deps.join("libfoo-aaaaaaaaaaaaaaaa.rlib"), b"old").unwrap();
        thread::sleep(Duration::from_millis(20));
        fs::write(deps.join("libfoo-bbbbbbbbbbbbbbbb.rlib"), b"new").unwrap();
        let c = enumerate(&repo).expect("enum");
        let purge: Vec<_> = c.iter().filter(|x| !x.kept).collect();
        assert_eq!(purge.len(), 1);
        assert!(purge[0].path.contains("aaaaaaaa"));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn dep_logical_name_strips_hash() {
        assert_eq!(
            dep_logical_name("execute_process-deadbeefdeadbeef"),
            "execute_process"
        );
    }
}
