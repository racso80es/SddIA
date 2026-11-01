//! Plan JSON, digest y apply acusado.

use crate::jail::{assert_candidate_jail, canonical_repo_root, JailError};
use crate::live::{cargo_build_gate, event_watcher_gate, ProcGate};
use crate::lock::{acquire_runtime_lock, LockError};
use crate::slice_events;
use crate::slice_misc;
use crate::slice_target;
use sddia_event_store::{store_mode, StoreMode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCandidate {
    pub path: String,
    pub bytes: u64,
    pub policy: String,
    pub kept: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanRequest {
    pub simulate: bool,
    #[serde(default)]
    pub repo_root: Option<String>,
    #[serde(default)]
    pub plan_digest: Option<String>,
    #[serde(default)]
    pub older_than_hours: Option<f64>,
    #[serde(default)]
    pub skip_live_check: bool,
}

#[derive(Debug)]
pub enum PlanError {
    Jail(JailError),
    Lock(LockError),
    Io(String),
    DigestMismatch,
    ApplyWithoutSimulate,
    CargoActive,
    WatcherActive,
    ProcUnreadable,
}

impl PlanError {
    pub fn code_and_msg(&self) -> (&'static str, String) {
        match self {
            PlanError::Jail(e) => e.code_and_msg(),
            PlanError::Lock(e) => (e.code, e.detail.clone()),
            PlanError::Io(s) => ("IO_ERROR", s.clone()),
            PlanError::DigestMismatch => (
                "PLAN_DIGEST_MISMATCH",
                "plan_digest no coincide con candidatos".into(),
            ),
            PlanError::ApplyWithoutSimulate => (
                "APPLY_REQUIRES_SIMULATE_FIRST",
                "simulate=false exige plan_digest".into(),
            ),
            PlanError::CargoActive => (
                "CARGO_ACTIVE",
                "cargo/rustc activo sobre SddIA/target".into(),
            ),
            PlanError::ProcUnreadable => (
                "PROC_UNREADABLE",
                "/proc no legible; apply abortado (fail-closed)".into(),
            ),
            PlanError::WatcherActive => (
                "WATCHER_ACTIVE",
                "event-watcher activo; abortar slice B".into(),
            ),
        }
    }
}

fn digest_for(candidates: &[PlanCandidate], repo_root: &str, mode: &str) -> String {
    let mut sorted = candidates.to_vec();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));
    let payload = json!({
        "repo_root": repo_root,
        "store_mode": mode,
        "candidates": sorted,
    });
    let bytes = serde_json::to_vec(&payload).unwrap_or_default();
    let hash = Sha256::digest(bytes);
    format!("sha256:{}", hex::encode(hash))
}

fn slice_bytes(candidates: &[PlanCandidate], prefix: &str) -> u64 {
    candidates
        .iter()
        .filter(|c| !c.kept && c.policy.starts_with(prefix) || c.policy.contains(prefix))
        .map(|c| c.bytes)
        .sum()
}

fn target_slice_bytes(candidates: &[PlanCandidate]) -> u64 {
    candidates
        .iter()
        .filter(|c| {
            !c.kept
                && (c.policy.starts_with("keep-latest-dep")
                    || c.policy.starts_with("keep-two-incremental"))
        })
        .map(|c| c.bytes)
        .sum()
}

fn events_slice_bytes(candidates: &[PlanCandidate]) -> u64 {
    candidates
        .iter()
        .filter(|c| !c.kept && c.policy.starts_with("events-fs"))
        .map(|c| c.bytes)
        .sum()
}

pub fn build_plan(req: &PlanRequest) -> Result<Value, PlanError> {
    let start = req
        .repo_root
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let repo = canonical_repo_root(&start).map_err(PlanError::Jail)?;
    let mode = store_mode();
    let mode_name = match mode {
        StoreMode::Redb => "redb",
        StoreMode::Filesystem => "filesystem",
        StoreMode::Dual => "dual",
    };
    let mut candidates = slice_target::enumerate(&repo).map_err(PlanError::Io)?;
    candidates.extend(slice_events::enumerate(&repo, mode).map_err(PlanError::Io)?);
    let ttl = req.older_than_hours.unwrap_or(168.0);
    candidates.extend(
        slice_misc::enumerate_scratch(&repo, ttl).map_err(PlanError::Io)?,
    );
    candidates.extend(slice_misc::enumerate_dist(&repo, 30).map_err(PlanError::Io)?);
    let plan_digest = digest_for(&candidates, &repo.to_string_lossy(), mode_name);
    Ok(json!({
        "simulated": req.simulate,
        "plan_digest": plan_digest,
        "repo_root": repo.to_string_lossy(),
        "store_mode": mode_name,
        "native_debug_bins_present": slice_target::native_debug_bin_exists(&repo),
        "slices": {
            "target": {
                "bytes_reclaimable": target_slice_bytes(&candidates),
            },
            "events_fs": {
                "bytes_reclaimable": events_slice_bytes(&candidates),
            },
            "scratch": {
                "bytes_reclaimable": slice_bytes(&candidates, "scratch"),
            },
            "dist": {
                "bytes_reclaimable": slice_bytes(&candidates, "dist-age"),
            },
        },
        "candidates": candidates,
    }))
}

pub fn apply_plan(req: &PlanRequest, plan: &Value) -> Result<Value, PlanError> {
    if req.simulate {
        return Ok(plan.clone());
    }
    let digest = req
        .plan_digest
        .as_deref()
        .filter(|s| !s.is_empty())
        .ok_or(PlanError::ApplyWithoutSimulate)?;
    let plan_digest = plan
        .get("plan_digest")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if digest != plan_digest {
        return Err(PlanError::DigestMismatch);
    }
    let repo_s = plan
        .get("repo_root")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let repo = PathBuf::from(repo_s);
    if !req.skip_live_check {
        match cargo_build_gate(&repo) {
            ProcGate::Blocked => return Err(PlanError::CargoActive),
            ProcGate::ProcUnreadable => return Err(PlanError::ProcUnreadable),
            ProcGate::Clear => {}
        }
    }
    let candidates: Vec<PlanCandidate> = plan
        .get("candidates")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let purging_events = candidates
        .iter()
        .any(|c| !c.kept && c.policy.starts_with("events-fs"));
    if !req.skip_live_check && purging_events {
        match event_watcher_gate() {
            ProcGate::Blocked => return Err(PlanError::WatcherActive),
            ProcGate::ProcUnreadable => return Err(PlanError::ProcUnreadable),
            ProcGate::Clear => {}
        }
    }
    let _lock = acquire_runtime_lock(&repo).map_err(PlanError::Lock)?;
    let mut purged_files = 0u64;
    let mut purged_dirs = 0u64;
    let mut bytes = 0u64;
    for c in &candidates {
        if c.kept {
            continue;
        }
        assert_candidate_jail(&repo, Path::new(&c.path)).map_err(PlanError::Jail)?;
        let p = repo.join(&c.path);
        if p.is_dir() {
            bytes += dir_size(&p);
            fs::remove_dir_all(&p).map_err(|e| PlanError::Io(e.to_string()))?;
            purged_dirs += 1;
        } else if p.is_file() {
            let meta = fs::metadata(&p).map_err(|e| PlanError::Io(e.to_string()))?;
            bytes += meta.len();
            fs::remove_file(&p).map_err(|e| PlanError::Io(e.to_string()))?;
            purged_files += 1;
        }
    }
    Ok(json!({
        "applied": true,
        "plan_digest": plan_digest,
        "purged_files": purged_files,
        "purged_dirs": purged_dirs,
        "bytes_reclaimed": bytes,
    }))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn simulate_produces_stable_digest() {
        let repo = std::env::temp_dir().join(format!("ret-digest-{}", std::process::id()));
        let _ = fs::remove_dir_all(&repo);
        fs::create_dir_all(repo.join("SddIA")).unwrap();
        fs::write(repo.join("SddIA/Cargo.toml"), "[workspace]\n").unwrap();
        let req = PlanRequest {
            simulate: true,
            repo_root: Some(repo.to_string_lossy().into_owned()),
            plan_digest: None,
            older_than_hours: None,
            skip_live_check: true,
        };
        let p1 = build_plan(&req).expect("plan");
        let p2 = build_plan(&req).expect("plan2");
        assert_eq!(
            p1.get("plan_digest").and_then(|v| v.as_str()),
            p2.get("plan_digest").and_then(|v| v.as_str())
        );
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn apply_rejects_bad_digest() {
        let req = PlanRequest {
            simulate: true,
            repo_root: None,
            plan_digest: None,
            older_than_hours: None,
            skip_live_check: true,
        };
        let plan = build_plan(&req).expect("plan");
        let apply_req = PlanRequest {
            simulate: false,
            repo_root: None,
            plan_digest: Some("sha256:dead".into()),
            older_than_hours: None,
            skip_live_check: true,
        };
        let err = apply_plan(&apply_req, &plan).expect_err("mismatch");
        matches!(err, PlanError::DigestMismatch);
    }

    #[test]
    fn native_bins_flag_in_plan() {
        let repo = std::env::temp_dir().join(format!("ret-native-{}", std::process::id()));
        let _ = fs::remove_dir_all(&repo);
        fs::create_dir_all(repo.join("SddIA")).unwrap();
        fs::write(repo.join("SddIA/Cargo.toml"), "[workspace]\n").unwrap();
        fs::create_dir_all(repo.join("SddIA/target/native-debug-bins")).unwrap();
        fs::write(
            repo.join("SddIA/target/native-debug-bins/execute-process"),
            b"bin",
        )
        .unwrap();
        let req = PlanRequest {
            simulate: true,
            repo_root: Some(repo.to_string_lossy().into_owned()),
            plan_digest: None,
            older_than_hours: None,
            skip_live_check: true,
        };
        let plan = build_plan(&req).expect("plan");
        assert_eq!(
            plan.get("native_debug_bins_present").and_then(|v| v.as_bool()),
            Some(true)
        );
        let _ = fs::remove_dir_all(&repo);
    }
}
