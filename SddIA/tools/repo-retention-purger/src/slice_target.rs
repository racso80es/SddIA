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
    let (logical, _) = dep_logical_and_hash(file_name);
    logical
}

pub fn dep_logical_and_hash(file_name: &str) -> (String, String) {
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);
    if let Some(idx) = stem.rfind('-') {
        let suffix = &stem[idx + 1..];
        if suffix.len() >= 8 && suffix.chars().all(|c| c.is_ascii_hexdigit()) {
            return (stem[..idx].to_string(), suffix.to_ascii_lowercase());
        }
    }
    (stem.to_string(), String::new())
}

fn hash_has_artifact(paths: &[PathBuf]) -> bool {
    paths.iter().any(|p| {
        matches!(
            p.extension().and_then(|e| e.to_str()),
            Some("rlib") | Some("so") | Some("a") | Some("dylib")
        )
    })
}

fn newest_hash_by_mtime(hashes: &HashMap<String, Vec<(PathBuf, SystemTime, u64)>>) -> String {
    let mut best_hash = String::new();
    let mut best_mtime = SystemTime::UNIX_EPOCH;
    for (h, files) in hashes {
        if h.is_empty() {
            continue;
        }
        let max_t = files
            .iter()
            .map(|(_, t, _)| *t)
            .max()
            .unwrap_or(SystemTime::UNIX_EPOCH);
        if max_t > best_mtime {
            best_mtime = max_t;
            best_hash.clone_from(h);
        }
    }
    best_hash
}

fn scan_deps_dir(repo: &Path, deps: &Path, profile: &str) -> Result<Vec<PlanCandidate>, String> {
    if !deps.is_dir() {
        return Ok(Vec::new());
    }
    let mut by_logical: HashMap<String, HashMap<String, Vec<(PathBuf, SystemTime, u64)>>> =
        HashMap::new();
    for entry in fs::read_dir(deps).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let (logical, hash) = dep_logical_and_hash(&name);
        let bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let t = mtime(&path);
        by_logical
            .entry(logical)
            .or_default()
            .entry(hash)
            .or_default()
            .push((path, t, bytes));
    }

    let mut out = Vec::new();
    let policy_keep = format!("keep-latest-dep-{profile}");
    for (_logical, hashes) in by_logical {
        if hashes.len() <= 1 {
            continue;
        }
        let keep_hash = newest_hash_by_mtime(&hashes);
        if keep_hash.is_empty() {
            continue;
        }
        let keep_paths: Vec<PathBuf> = hashes
            .get(&keep_hash)
            .map(|v| v.iter().map(|(p, _, _)| p.clone()).collect())
            .unwrap_or_default();
        if !hash_has_artifact(&keep_paths) {
            let older_has_artifact = hashes
                .iter()
                .filter(|(h, _)| *h != &keep_hash)
                .any(|(_, files)| hash_has_artifact(&files.iter().map(|(p, _, _)| p.clone()).collect::<Vec<_>>()));
            if older_has_artifact {
                continue;
            }
        }
        for (hash, files) in &hashes {
            let purge = hash != &keep_hash;
            for (path, _, bytes) in files {
                let Some(rel) = rel_from_repo(repo, path) else {
                    continue;
                };
                out.push(PlanCandidate {
                    path: rel,
                    bytes: *bytes,
                    policy: policy_keep.clone(),
                    kept: !purge,
                });
            }
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

fn scan_incremental_crate(
    repo: &Path,
    crate_dir: &Path,
) -> Result<Vec<PlanCandidate>, String> {
    let mut sessions: Vec<(PathBuf, SystemTime)> = Vec::new();
    for entry in fs::read_dir(crate_dir).map_err(|e| e.to_string())? {
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
    let drop_count = sessions.len().saturating_sub(2);
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

fn scan_incremental(repo: &Path, inc_root: &Path) -> Result<Vec<PlanCandidate>, String> {
    if !inc_root.is_dir() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(inc_root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.path().is_dir() {
            continue;
        }
        out.extend(scan_incremental_crate(repo, &entry.path())?);
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
    fn dep_generation_keeps_rlib_and_rmeta() {
        let repo = std::env::temp_dir().join(format!("ret-target-{}", std::process::id()));
        let _ = fs::remove_dir_all(&repo);
        fs::create_dir_all(repo.join("SddIA")).unwrap();
        fs::write(repo.join("SddIA/Cargo.toml"), "[workspace]\n").unwrap();
        let deps = repo.join("SddIA/target/debug/deps");
        fs::create_dir_all(&deps).unwrap();
        fs::write(deps.join("libfoo-aaaaaaaaaaaaaaaa.rlib"), b"old").unwrap();
        fs::write(deps.join("libfoo-aaaaaaaaaaaaaaaa.rmeta"), b"oldm").unwrap();
        thread::sleep(Duration::from_millis(20));
        fs::write(deps.join("libfoo-bbbbbbbbbbbbbbbb.rlib"), b"new").unwrap();
        fs::write(deps.join("libfoo-bbbbbbbbbbbbbbbb.rmeta"), b"newm").unwrap();
        let c = enumerate(&repo).expect("enum");
        let purge: Vec<_> = c.iter().filter(|x| !x.kept).collect();
        assert_eq!(purge.len(), 2);
        assert!(purge.iter().all(|x| x.path.contains("aaaaaaaa")));
        let kept: Vec<_> = c.iter().filter(|x| x.kept).collect();
        assert_eq!(kept.len(), 2);
        assert!(kept.iter().all(|x| x.path.contains("bbbbbbbb")));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn dep_hold_incomplete_generation() {
        let repo = std::env::temp_dir().join(format!("ret-hold-{}", std::process::id()));
        let _ = fs::remove_dir_all(&repo);
        fs::create_dir_all(repo.join("SddIA")).unwrap();
        fs::write(repo.join("SddIA/Cargo.toml"), "[workspace]\n").unwrap();
        let deps = repo.join("SddIA/target/debug/deps");
        fs::create_dir_all(&deps).unwrap();
        fs::write(deps.join("libbar-cccccccccccccccc.rlib"), b"old").unwrap();
        thread::sleep(Duration::from_millis(20));
        fs::write(deps.join("libbar-dddddddddddddddd.rmeta"), b"newm").unwrap();
        let c = enumerate(&repo).expect("enum");
        assert!(c.iter().filter(|x| !x.kept).next().is_none());
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn dep_logical_name_strips_hash() {
        assert_eq!(
            dep_logical_name("execute_process-deadbeefdeadbeef"),
            "execute_process"
        );
    }

    #[test]
    fn incremental_keeps_two_sessions_per_crate() {
        let repo = std::env::temp_dir().join(format!("ret-inc-{}", std::process::id()));
        let _ = fs::remove_dir_all(&repo);
        fs::create_dir_all(repo.join("SddIA")).unwrap();
        fs::write(repo.join("SddIA/Cargo.toml"), "[workspace]\n").unwrap();
        let crate_dir = repo.join("SddIA/target/debug/incremental/mycrate-abc");
        fs::create_dir_all(crate_dir.join("s-old")).unwrap();
        thread::sleep(Duration::from_millis(15));
        fs::create_dir_all(crate_dir.join("s-mid")).unwrap();
        thread::sleep(Duration::from_millis(15));
        fs::create_dir_all(crate_dir.join("s-new")).unwrap();
        let c = enumerate(&repo).expect("enum");
        let purge: Vec<_> = c
            .iter()
            .filter(|x| !x.kept && x.policy.starts_with("keep-two-incremental"))
            .collect();
        assert_eq!(purge.len(), 1);
        assert!(purge[0].path.contains("s-old"));
        let _ = fs::remove_dir_all(&repo);
    }
}
