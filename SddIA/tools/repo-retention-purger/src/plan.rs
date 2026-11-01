//! Plan JSON, digest y apply acusado.

use crate::jail::{assert_candidate_jail, canonical_repo_root, JailError};
use crate::lock::{acquire_runtime_lock, LockError};
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
}

#[derive(Debug)]
pub enum PlanError {
    Jail(JailError),
    Lock(LockError),
    Io(String),
    DigestMismatch,
    ApplyWithoutSimulate,
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
    let candidates: Vec<PlanCandidate> = Vec::new();
    let plan_digest = digest_for(&candidates, &repo.to_string_lossy(), mode_name);
    Ok(json!({
        "simulated": req.simulate,
        "plan_digest": plan_digest,
        "repo_root": repo.to_string_lossy(),
        "store_mode": mode_name,
        "slices": {
            "target": { "candidates": [], "bytes_reclaimable": 0 },
            "events_fs": { "candidates": [], "bytes_reclaimable": 0 },
            "scratch": { "candidates": [], "bytes_reclaimable": 0 },
            "dist": { "candidates": [], "bytes_reclaimable": 0 },
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
    let _lock = acquire_runtime_lock(&repo).map_err(PlanError::Lock)?;
    let candidates: Vec<PlanCandidate> = plan
        .get("candidates")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let mut purged = 0u64;
    let mut bytes = 0u64;
    for c in &candidates {
        if c.kept {
            continue;
        }
        assert_candidate_jail(&repo, Path::new(&c.path)).map_err(PlanError::Jail)?;
        let p = repo.join(&c.path);
        if p.is_file() {
            let meta = fs::metadata(&p).map_err(|e| PlanError::Io(e.to_string()))?;
            bytes += meta.len();
            fs::remove_file(&p).map_err(|e| PlanError::Io(e.to_string()))?;
            purged += 1;
        }
    }
    Ok(json!({
        "applied": true,
        "plan_digest": plan_digest,
        "purged_files": purged,
        "bytes_reclaimed": bytes,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_produces_stable_digest() {
        let req = PlanRequest {
            simulate: true,
            repo_root: None,
            plan_digest: None,
            older_than_hours: None,
        };
        let p1 = build_plan(&req).expect("plan");
        let p2 = build_plan(&req).expect("plan2");
        assert_eq!(
            p1.get("plan_digest").and_then(|v| v.as_str()),
            p2.get("plan_digest").and_then(|v| v.as_str())
        );
    }

    #[test]
    fn apply_rejects_bad_digest() {
        let req = PlanRequest {
            simulate: true,
            repo_root: None,
            plan_digest: None,
            older_than_hours: None,
        };
        let plan = build_plan(&req).expect("plan");
        let apply_req = PlanRequest {
            simulate: false,
            repo_root: None,
            plan_digest: Some("sha256:dead".into()),
            older_than_hours: None,
        };
        let err = apply_plan(&apply_req, &plan).expect_err("mismatch");
        matches!(err, PlanError::DigestMismatch);
    }
}
