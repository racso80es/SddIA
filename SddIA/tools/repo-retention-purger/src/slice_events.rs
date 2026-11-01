//! Slice B — residuo JSON bajo `.events/` (solo modo `redb`).

use crate::plan::PlanCandidate;
use sddia_event_store::{runtime_db_path, EventStore, StoreMode};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn event_fs_roots(repo: &Path) -> Vec<PathBuf> {
    vec![
        repo.join(".events/pending"),
        repo.join(".events/processed"),
        repo.join(".events/processed/subscribers"),
        repo.join(".events/dead-letter"),
        repo.join(".events/dead-letter/subscribers"),
        repo.join(".events/telemetry"),
        repo.join(".events/domain"),
        repo.join(".events/orchestration"),
        repo.join(".events/progress"),
    ]
}

fn rel_from_repo(repo: &Path, abs: &Path) -> Option<String> {
    abs.strip_prefix(repo)
        .ok()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
}

fn indexed_dead_letters(repo: &Path) -> HashSet<String> {
    let db = runtime_db_path(repo);
    if !db.is_file() {
        return HashSet::new();
    }
    let Ok(store) = EventStore::open(&db) else {
        return HashSet::new();
    };
    store
        .list_dead_letters()
        .map(|rows| rows.into_iter().map(|(id, _)| id).collect())
        .unwrap_or_default()
}

pub fn enumerate(repo: &Path, mode: StoreMode) -> Result<Vec<PlanCandidate>, String> {
    if !matches!(mode, StoreMode::Redb) {
        return Ok(Vec::new());
    }
    let index = indexed_dead_letters(repo);
    let mut out = Vec::new();
    for root in event_fs_roots(repo) {
        if !root.is_dir() {
            continue;
        }
        walk_json(repo, &root, &index, &mut out)?;
    }
    Ok(out)
}

fn walk_json(
    repo: &Path,
    dir: &Path,
    index: &HashSet<String>,
    out: &mut Vec<PlanCandidate>,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_json(repo, &path, index, out)?;
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let Some(rel) = rel_from_repo(repo, &path) else {
            continue;
        };
        let in_index = index.contains(&stem);
        out.push(PlanCandidate {
            path: rel,
            bytes,
            policy: if in_index {
                "dead-letter-index".into()
            } else {
                "events-fs-residual-redb".into()
            },
            kept: in_index,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn filesystem_mode_empty() {
        let repo = std::env::temp_dir().join(format!("ret-ev-fs-{}", std::process::id()));
        let _ = fs::remove_dir_all(&repo);
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        fs::write(repo.join(".events/pending/e1.json"), b"{}").unwrap();
        let c = enumerate(&repo, StoreMode::Filesystem).expect("ok");
        assert!(c.is_empty());
        let _ = fs::remove_dir_all(&repo);
    }
}
