//! Atestaciones QA locales (`qa-attestation/1.0`) — HU merge-thermodynamics PBI-04.

use super::persist_pec_correlation_proof::resolve_eda_proofs_dir;
use super::qa_profile::{PROFILE_DOCS_ONLY, PROFILE_FULL};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const SCHEMA: &str = "qa-attestation/1.0";
pub const TTL_SECS: u64 = 86_400;
const NAMESPACE: &str = "qa-attestations";

const BRANCH_PREFIXES: &[&str] = &["feat/", "fix/", "refactor/", "hotfix/"];

#[derive(Debug, Clone)]
pub struct AttestationHit {
    pub execution_id: String,
    pub qa_profile: String,
    pub path: PathBuf,
}

pub fn branch_slug(branch: &str) -> String {
    let name = branch.trim();
    for p in BRANCH_PREFIXES {
        if name.starts_with(p) {
            return name[p.len()..].to_string();
        }
    }
    if let Some((_, rest)) = name.split_once('/') {
        return rest.to_string();
    }
    name.to_string()
}

pub fn attestation_path_for_branch(repo: &Path, branch: &str) -> PathBuf {
    let slug = branch_slug(branch);
    resolve_eda_proofs_dir(repo)
        .join(NAMESPACE)
        .join(format!("{slug}.json"))
}

fn git_output(repo: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| format!("git spawn: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn tree_sha_for_commit(repo: &Path, commit_sha: &str) -> Result<String, String> {
    git_output(repo, &["rev-parse", "--verify", &format!("{commit_sha}^{{tree}}")])
}

pub fn merge_source_tree_sha(repo: &Path) -> Result<String, String> {
    if let Ok(t) = git_output(repo, &["rev-parse", "--verify", "HEAD^2^{tree}"]) {
        if !t.is_empty() {
            return Ok(t);
        }
    }
    tree_sha_for_commit(repo, "HEAD")
}

fn instance_requires_hmac(repo: &Path) -> bool {
    let path = repo.join(".SddIA/instances.json");
    let Ok(text) = fs::read_to_string(&path) else {
        return false;
    };
    let Ok(v) = serde_json::from_str::<Value>(&text) else {
        return false;
    };
    v.as_object()
        .map(|m| {
            m.values().any(|inst| {
                inst.get("profile")
                    .and_then(|p| p.as_str())
                    .map(|s| s.eq_ignore_ascii_case("multiuser"))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

fn parse_issued_at(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

pub fn validate_attestation_value(
    doc: &Value,
    expected_tree_sha: &str,
    delta_class: Option<&str>,
) -> Result<AttestationHit, String> {
    if doc.get("schema").and_then(|v| v.as_str()) != Some(SCHEMA) {
        return Err("schema mismatch".into());
    }
    if doc.get("verdict").and_then(|v| v.as_str()) != Some("aprobado") {
        return Err("verdict not aprobado".into());
    }
    let tree = doc
        .get("tree_sha")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "tree_sha missing".to_string())?;
    if tree != expected_tree_sha {
        return Err("tree_sha mismatch".into());
    }
    let issued = doc
        .get("issued_at")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "issued_at missing".to_string())?;
    let issued_dt = parse_issued_at(issued).ok_or_else(|| "issued_at invalid".to_string())?;
    let ttl = doc
        .get("ttl_secs")
        .and_then(|v| v.as_u64())
        .unwrap_or(TTL_SECS);
    let age = Utc::now().signed_duration_since(issued_dt);
    if age.num_seconds() > ttl as i64 {
        return Err("attestation expired".into());
    }
    let qa_profile = doc
        .get("qa_profile")
        .and_then(|v| v.as_str())
        .unwrap_or(PROFILE_FULL);
    if qa_profile == PROFILE_DOCS_ONLY {
        if delta_class == Some("active") {
            return Err("docs-only blocked with active delta".into());
        }
    } else if qa_profile != PROFILE_FULL {
        return Err("qa_profile not eligible".into());
    }
    let execution_id = doc
        .get("execution_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "execution_id missing".to_string())?;
    Ok(AttestationHit {
        execution_id: execution_id.to_string(),
        qa_profile: qa_profile.to_string(),
        path: PathBuf::new(),
    })
}

pub fn validate_attestation_file(
    path: &Path,
    expected_tree_sha: &str,
    delta_class: Option<&str>,
) -> Result<AttestationHit, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read attestation: {e}"))?;
    let doc: Value =
        serde_json::from_str(&text).map_err(|e| format!("parse attestation: {e}"))?;
    let hit = validate_attestation_value(&doc, expected_tree_sha, delta_class)?;
    Ok(AttestationHit {
        execution_id: hit.execution_id,
        qa_profile: hit.qa_profile,
        path: path.to_path_buf(),
    })
}

pub fn remove_attestation_for_branch(repo: &Path, branch: &str) -> bool {
    let path = attestation_path_for_branch(repo, branch);
    fs::remove_file(&path).is_ok()
}

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn resolve_pr_branch(inputs: &Value, state: &Value) -> Option<String> {
    str_field(inputs, "pr_branch")
        .or_else(|| str_field(inputs, "branch_name"))
        .or_else(|| str_field(state, "pr_branch"))
        .or_else(|| str_field(state, "branch_name"))
}

pub fn try_write_from_ppr(repo: &Path, state: &Value, inputs: &Value) -> Result<PathBuf, String> {
    if state.get("verdict").and_then(|v| v.as_str()) != Some("aprobado") {
        return Err("verdict not aprobado".into());
    }
    let branch = resolve_pr_branch(inputs, state)
        .ok_or_else(|| "pr_branch missing for attestation".to_string())?;
    let head_sha = git_output(repo, &["rev-parse", "--verify", &format!("refs/heads/{branch}")])
        .or_else(|_| git_output(repo, &["rev-parse", "--verify", "HEAD"]))?;
    let tree_sha = tree_sha_for_commit(repo, &head_sha)?;
    let qa_profile = state
        .get("qa_profile")
        .and_then(|v| v.as_str())
        .or_else(|| inputs.get("qa_profile").and_then(|v| v.as_str()))
        .unwrap_or(PROFILE_FULL);
    let correlation_id = str_field(inputs, "correlation_id")
        .or_else(|| str_field(state, "correlation_id"))
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let execution_id = str_field(state, "execution_id")
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let issued_at = Utc::now().to_rfc3339();
    let path = attestation_path_for_branch(repo, &branch);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir attestations: {e}"))?;
    }
    let mut doc = json!({
        "schema": SCHEMA,
        "branch": branch,
        "head_sha": head_sha,
        "tree_sha": tree_sha,
        "verdict": "aprobado",
        "qa_profile": qa_profile,
        "issued_at": issued_at,
        "ttl_secs": TTL_SECS,
        "correlation_id": correlation_id,
        "execution_id": execution_id,
    });
    let _ = instance_requires_hmac(repo);
    let text = serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())?;
    fs::write(&path, text).map_err(|e| format!("write attestation: {e}"))?;
    Ok(path)
}

pub fn validate_for_accept_pr(
    repo: &Path,
    inputs: &Value,
) -> Result<AttestationHit, String> {
    let tree = merge_source_tree_sha(repo)?;
    let path = if let Some(att_ref) = str_field(inputs, "attestation_ref") {
        let p = PathBuf::from(att_ref);
        if p.is_absolute() {
            p
        } else {
            repo.join(p)
        }
    } else if let Some(branch) = str_field(inputs, "source_branch") {
        attestation_path_for_branch(repo, &branch)
    } else {
        return Err("no attestation ref or source_branch".into());
    };
    validate_attestation_file(&path, &tree, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn sample_doc(tree: &str, profile: &str) -> Value {
        json!({
            "schema": SCHEMA,
            "branch": "feat/x",
            "head_sha": "abc",
            "tree_sha": tree,
            "verdict": "aprobado",
            "qa_profile": profile,
            "issued_at": Utc::now().to_rfc3339(),
            "ttl_secs": TTL_SECS,
            "correlation_id": "c",
            "execution_id": "e-1",
        })
    }

    #[test]
    fn branch_slug_strips_feat_prefix() {
        assert_eq!(branch_slug("feat/merge-thermo-04"), "merge-thermo-04");
    }

    #[test]
    fn validate_ok_full_profile() {
        let tree = "deadbeef";
        let doc = sample_doc(tree, PROFILE_FULL);
        let hit = validate_attestation_value(&doc, tree, Some("active")).expect("ok");
        assert_eq!(hit.execution_id, "e-1");
    }

    #[test]
    fn validate_docs_only_blocks_active_delta() {
        let tree = "deadbeef";
        let doc = sample_doc(tree, PROFILE_DOCS_ONLY);
        assert!(validate_attestation_value(&doc, tree, Some("active")).is_err());
        assert!(validate_attestation_value(&doc, tree, Some("passive")).is_ok());
    }

    #[test]
    fn validate_rejects_tree_mismatch() {
        let doc = sample_doc("aaa", PROFILE_FULL);
        assert!(validate_attestation_value(&doc, "bbb", None).is_err());
    }

    #[test]
    fn validate_rejects_expired() {
        let old = (Utc::now() - Duration::from_secs(TTL_SECS + 10)).to_rfc3339();
        let doc = json!({
            "schema": SCHEMA,
            "verdict": "aprobado",
            "tree_sha": "t",
            "qa_profile": PROFILE_FULL,
            "issued_at": old,
            "ttl_secs": TTL_SECS,
            "execution_id": "e",
        });
        assert!(validate_attestation_value(&doc, "t", None).is_err());
    }

    #[test]
    fn validate_rejects_bad_verdict() {
        let doc = sample_doc("t", PROFILE_FULL);
        let mut doc = doc;
        doc["verdict"] = json!("rechazado");
        assert!(validate_attestation_value(&doc, "t", None).is_err());
    }
}
