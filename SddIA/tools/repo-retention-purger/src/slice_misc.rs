//! Slices C (scratch) y D (dist).

use crate::plan::PlanCandidate;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SCRATCH_PREFIXES: &[&str] = &[".tmp/", "tmp/", "docs/tmp/"];
const SUSPICIOUS_BEFORE: u64 = 946_684_800; // 2000-01-01 UTC

fn rel_from_repo(repo: &Path, abs: &Path) -> Option<String> {
    abs.strip_prefix(repo)
        .ok()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
}

fn under_scratch_prefix(rel: &str) -> bool {
    SCRATCH_PREFIXES
        .iter()
        .any(|p| rel.starts_with(p) || rel == p.trim_end_matches('/'))
}

pub fn enumerate_scratch(repo: &Path, older_than_hours: f64) -> Result<Vec<PlanCandidate>, String> {
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs((older_than_hours * 3600.0) as u64))
        .unwrap_or(UNIX_EPOCH);
    let mut out = Vec::new();
    for prefix in SCRATCH_PREFIXES {
        walk_scratch(repo, &repo.join(prefix.trim_end_matches('/')), cutoff, &mut out)?;
    }
    Ok(out)
}

fn walk_scratch(
    repo: &Path,
    dir: &Path,
    cutoff: SystemTime,
    out: &mut Vec<PlanCandidate>,
) -> Result<(), String> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_scratch(repo, &path, cutoff, out)?;
            continue;
        }
        let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
        let Some(rel) = rel_from_repo(repo, &path) else {
            continue;
        };
        if !under_scratch_prefix(&rel) {
            continue;
        }
        let modified = meta.modified().unwrap_or(UNIX_EPOCH);
        let secs = modified
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if secs < SUSPICIOUS_BEFORE {
            out.push(PlanCandidate {
                path: rel,
                bytes: meta.len(),
                policy: "hold-suspicious-mtime".into(),
                kept: true,
            });
            continue;
        }
        if modified >= cutoff {
            continue;
        }
        out.push(PlanCandidate {
            path: rel,
            bytes: meta.len(),
            policy: "scratch-ttl".into(),
            kept: false,
        });
    }
    Ok(())
}

pub fn enumerate_dist(repo: &Path, retention_days: u64) -> Result<Vec<PlanCandidate>, String> {
    let dist = repo.join("dist");
    if !dist.is_dir() {
        return Ok(Vec::new());
    }
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(retention_days * 86400))
        .unwrap_or(UNIX_EPOCH);
    let mut by_name: HashMap<String, Vec<(PathBuf, SystemTime, u64)>> = HashMap::new();
    for entry in fs::read_dir(&dist).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let m = fs::metadata(&path).map_err(|e| e.to_string())?;
        by_name
            .entry(name)
            .or_default()
            .push((path, m.modified().unwrap_or(UNIX_EPOCH), m.len()));
    }
    let mut out = Vec::new();
    for (_name, mut files) in by_name {
        files.sort_by_key(|(_, t, _)| *t);
        if files.is_empty() {
            continue;
        }
        let keep = files.last().expect("non-empty");
        for (path, modified, bytes) in &files {
            let Some(rel) = rel_from_repo(repo, path) else {
                continue;
            };
            if path == &keep.0 {
                out.push(PlanCandidate {
                    path: rel,
                    bytes: *bytes,
                    policy: "dist-keep-newest".into(),
                    kept: true,
                });
            } else if *modified < cutoff {
                out.push(PlanCandidate {
                    path: rel,
                    bytes: *bytes,
                    policy: "dist-age".into(),
                    kept: false,
                });
            }
        }
    }
    Ok(out)
}
