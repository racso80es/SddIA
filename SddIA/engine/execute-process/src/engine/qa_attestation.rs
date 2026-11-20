//! Atestaciones QA locales (`qa-attestation/1.0`) — HU merge-thermodynamics PBI-04.
//! Aduana pre-fusión accept-pr: emisor PPR + tree_sha + perfil (MAN-45 / KZ-2).

use super::git_porcelain;
use super::persist_pec_correlation_proof::resolve_eda_proofs_dir;
use super::qa_profile::{PROFILE_DOCS_ONLY, PROFILE_FULL};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const SCHEMA: &str = "qa-attestation/1.0";
pub const TTL_SECS: u64 = 86_400;
pub const ISSUER_PROCESS: &str = "pull-request-review";
pub const LAB_SKIP_ENV: &str = "SDDIA_LAB_SKIP_PPR_ATTESTATION_ACCEPT";

pub const FRICTION_REQUIRED: &str = "qa_attestation_required";
pub const FRICTION_TREE_MISMATCH: &str = "qa_attestation_tree_mismatch";
pub const FRICTION_PROFILE_BLOCKED: &str = "qa_attestation_profile_blocked";
pub const FRICTION_ISSUER_INVALID: &str = "qa_attestation_issuer_invalid";
pub const FRICTION_DEDUPE_NOT_APPROVAL: &str = "qa_attestation_dedupe_not_approval";

const NAMESPACE: &str = "qa-attestations";
const OPERATOR_HINT: &str =
    "Ejecutar pull-request-review hasta aprobado; no reescribir JSON a mano ni usar docs-only con delta active";

const BRANCH_PREFIXES: &[&str] = &["feat/", "fix/", "refactor/", "hotfix/"];

#[derive(Debug, Clone)]
pub struct AttestationHit {
    pub execution_id: String,
    pub qa_profile: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttestationFault {
    pub friction_id: String,
    pub message: String,
    pub reasons: Vec<String>,
    pub source_branch: Option<String>,
    pub expected_tree_sha: Option<String>,
}

impl AttestationFault {
    pub fn operator_hint(&self) -> &'static str {
        OPERATOR_HINT
    }

    /// Mensaje corto para Err de accept-pr (friction_id primero).
    pub fn to_error_string(&self) -> String {
        if self.reasons.is_empty() {
            format!("{}: {}", self.friction_id, self.message)
        } else {
            format!(
                "{}: {} ({})",
                self.friction_id,
                self.message,
                self.reasons.join(",")
            )
        }
    }
}

pub fn lab_skip_ppr_attestation() -> bool {
    std::env::var(LAB_SKIP_ENV)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

pub fn branch_slug(branch: &str) -> String {
    let name = branch.trim();
    for p in BRANCH_PREFIXES {
        if let Some(rest) = name.strip_prefix(p) {
            return rest.to_string();
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

fn parse_issued_at(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn map_raw_validation_err(err: &str) -> AttestationFault {
    let e = err.to_ascii_lowercase();
    if e.contains("issuer") {
        return AttestationFault {
            friction_id: FRICTION_ISSUER_INVALID.into(),
            message: "accept-pr exige issuer_process=pull-request-review".into(),
            reasons: vec![err.to_string()],
            source_branch: None,
            expected_tree_sha: None,
        };
    }
    if e.contains("tree_sha mismatch") {
        return AttestationFault {
            friction_id: FRICTION_TREE_MISMATCH.into(),
            message: "tree_sha de atestación diverge del árbol a fusionar".into(),
            reasons: vec![err.to_string()],
            source_branch: None,
            expected_tree_sha: None,
        };
    }
    if e.contains("docs-only") || e.contains("qa_profile not eligible") {
        return AttestationFault {
            friction_id: FRICTION_PROFILE_BLOCKED.into(),
            message: "perfil QA no elegible para el delta (docs-only+active denegado)".into(),
            reasons: vec![err.to_string()],
            source_branch: None,
            expected_tree_sha: None,
        };
    }
    AttestationFault {
        friction_id: FRICTION_REQUIRED.into(),
        message: "accept-pr exige qa-attestation/1.0 aprobado del tree_sha a fusionar".into(),
        reasons: vec![err.to_string()],
        source_branch: None,
        expected_tree_sha: None,
    }
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
    let issuer = doc
        .get("issuer_process")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("");
    if issuer != ISSUER_PROCESS {
        return Err("issuer_process invalid or missing".into());
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

fn yaml_frontmatter_field(text: &str, key: &str) -> Option<String> {
    let mut lines = text.lines();
    if lines.next()? != "---" {
        return None;
    }
    let prefix = format!("{key}:");
    for line in lines {
        if line.trim() == "---" {
            break;
        }
        if let Some(v) = line.strip_prefix(prefix.as_str()) {
            return Some(v.trim().trim_matches('"').to_string());
        }
    }
    None
}

/// PPR full-profile: Argos sella `validacion.md` pero no siempre propaga `state.verdict`.
fn ppr_verdict_aprobado(repo: &Path, state: &Value, inputs: &Value) -> bool {
    if state.get("verdict").and_then(|v| v.as_str()) == Some("aprobado") {
        return true;
    }
    if state.get("argos_verdict").and_then(|v| v.as_str()) == Some("pass") {
        return true;
    }
    let persist = str_field(inputs, "persist_ref").or_else(|| str_field(state, "persist_ref"));
    let Some(persist) = persist else {
        return false;
    };
    let path = repo.join(persist.trim_start_matches("./")).join("validacion.md");
    let Ok(text) = fs::read_to_string(&path) else {
        return false;
    };
    let global = yaml_frontmatter_field(&text, "global").unwrap_or_default();
    if !global.eq_ignore_ascii_case("APTO") {
        return false;
    }
    for key in ["ppr_verdict", "verdict", "approval_status"] {
        if let Some(v) = yaml_frontmatter_field(&text, key) {
            if v.eq_ignore_ascii_case("aprobado") {
                return true;
            }
        }
    }
    false
}

pub fn try_write_from_ppr(repo: &Path, state: &Value, inputs: &Value) -> Result<PathBuf, String> {
    if !ppr_verdict_aprobado(repo, state, inputs) {
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
    let doc = json!({
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
        "issuer_process": ISSUER_PROCESS,
    });
    let text = serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())?;
    fs::write(&path, text).map_err(|e| format!("write attestation: {e}"))?;
    Ok(path)
}

fn persist_validacion_rel(repo: &Path, persist_ref: &str) -> String {
    let trimmed = persist_ref.trim();
    let rel = if Path::new(trimmed).is_absolute() {
        Path::new(trimmed)
            .strip_prefix(repo)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| trimmed.to_string())
    } else {
        trimmed.trim_start_matches("./").to_string()
    };
    format!("{}/validacion.md", rel.trim_end_matches('/'))
}

/// Rutas sucias fuera de `{persist_ref}/validacion.md` (gate pre-PR AUT-38 / AUT-36 CA-2).
pub fn worktree_violations_except_validacion(
    repo: &Path,
    persist_ref: &str,
) -> Result<Vec<String>, String> {
    let allowed = persist_validacion_rel(repo, persist_ref);
    let out = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("git status: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git status failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let mut violations = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let Some(path) = git_porcelain::porcelain_path_from_line(line) else {
            continue;
        };
        if path == allowed {
            continue;
        }
        violations.push(path);
    }
    Ok(violations)
}

pub fn validate_attestation_for_branch_head(
    repo: &Path,
    branch: &str,
) -> Result<AttestationHit, String> {
    let head_sha = git_output(
        repo,
        &[
            "rev-parse",
            "--verify",
            &format!("refs/heads/{branch}"),
        ],
    )
    .or_else(|_| git_output(repo, &["rev-parse", "--verify", "HEAD"]))?;
    let tree_sha = tree_sha_for_commit(repo, &head_sha)?;
    let path = attestation_path_for_branch(repo, branch);
    // DCC pre-PR: fail-closed active si no hay delta explícito (paridad MAN-45).
    let hit = validate_attestation_file(&path, &tree_sha, Some("active"))?;
    Ok(AttestationHit {
        execution_id: hit.execution_id,
        qa_profile: hit.qa_profile,
        path,
    })
}

fn truthy_flag(v: &Value, key: &str) -> bool {
    match v.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => matches!(s.to_lowercase().as_str(), "true" | "1" | "yes" | "on"),
        _ => false,
    }
}

/// Resuelve `delta_class` desde inputs/state; fail-closed → `active` si ambiguo (D-DELTA).
pub fn resolve_delta_class(inputs: &Value, state: Option<&Value>) -> String {
    for src in [Some(inputs), state] {
        let Some(v) = src else {
            continue;
        };
        if let Some(dc) = str_field(v, "delta_class") {
            return dc.to_ascii_lowercase();
        }
    }
    "active".to_string()
}

fn claims_dedupe_only_credit(inputs: &Value, state: Option<&Value>) -> bool {
    let keys = [
        "ppr_status",
        "qa_status",
        "attestation_credit",
        "ppr_credit",
        "skipped_reason",
    ];
    for src in [Some(inputs), state] {
        let Some(v) = src else {
            continue;
        };
        for k in keys {
            if let Some(s) = str_field(v, k) {
                if s.eq_ignore_ascii_case("skipped-ppr-dedupe") {
                    return true;
                }
            }
        }
        if truthy_flag(v, "skipped_ppr_dedupe") {
            return true;
        }
    }
    false
}

fn expected_tree_sha_for_accept(
    repo: &Path,
    inputs: &Value,
) -> Result<(String, Option<String>), String> {
    let branch = str_field(inputs, "source_branch");
    if truthy_flag(inputs, "merge_already_done") {
        let tree = merge_source_tree_sha(repo)?;
        return Ok((tree, branch));
    }
    let Some(ref source) = branch else {
        return Err("no attestation ref or source_branch".into());
    };
    let head_sha = git_output(
        repo,
        &[
            "rev-parse",
            "--verify",
            &format!("refs/heads/{source}"),
        ],
    )
    .or_else(|_| {
        git_output(
            repo,
            &["rev-parse", "--verify", &format!("{source}^{{commit}}")],
        )
    })?;
    let tree = tree_sha_for_commit(repo, &head_sha)?;
    Ok((tree, Some(source.clone())))
}

/// Aduana PPR bloqueante para accept-pr (D-API / D-GATE).
pub fn validate_for_accept_pr(
    repo: &Path,
    inputs: &Value,
) -> Result<AttestationHit, String> {
    validate_for_accept_pr_with_state(repo, inputs, None).map_err(|f| f.to_error_string())
}

pub fn validate_for_accept_pr_with_state(
    repo: &Path,
    inputs: &Value,
    state: Option<&Value>,
) -> Result<AttestationHit, AttestationFault> {
    if lab_skip_ppr_attestation() {
        return Ok(AttestationHit {
            execution_id: "lab-skip".into(),
            qa_profile: PROFILE_FULL.into(),
            path: PathBuf::new(),
        });
    }

    let dedupe_claim = claims_dedupe_only_credit(inputs, state);

    let (expected_tree, source_branch) = match expected_tree_sha_for_accept(repo, inputs) {
        Ok(v) => v,
        Err(e) => {
            let mut fault = if dedupe_claim {
                AttestationFault {
                    friction_id: FRICTION_DEDUPE_NOT_APPROVAL.into(),
                    message: "skipped-ppr-dedupe no habilita fusión".into(),
                    reasons: vec!["dedupe_not_approval".into(), e],
                    source_branch: str_field(inputs, "source_branch"),
                    expected_tree_sha: None,
                }
            } else {
                map_raw_validation_err(&e)
            };
            fault.source_branch = str_field(inputs, "source_branch");
            return Err(fault);
        }
    };

    let path = if let Some(att_ref) = str_field(inputs, "attestation_ref") {
        let p = PathBuf::from(att_ref);
        if p.is_absolute() {
            p
        } else {
            repo.join(p)
        }
    } else if let Some(ref branch) = source_branch {
        attestation_path_for_branch(repo, branch)
    } else {
        let mut fault = if dedupe_claim {
            AttestationFault {
                friction_id: FRICTION_DEDUPE_NOT_APPROVAL.into(),
                message: "skipped-ppr-dedupe no habilita fusión".into(),
                reasons: vec!["dedupe_not_approval".into()],
                source_branch: None,
                expected_tree_sha: Some(expected_tree.clone()),
            }
        } else {
            AttestationFault {
                friction_id: FRICTION_REQUIRED.into(),
                message: "accept-pr exige qa-attestation/1.0 aprobado del tree_sha a fusionar"
                    .into(),
                reasons: vec!["missing_or_unapproved_attestation".into()],
                source_branch: None,
                expected_tree_sha: Some(expected_tree.clone()),
            }
        };
        fault.source_branch = str_field(inputs, "source_branch");
        return Err(fault);
    };

    if !path.is_file() {
        let mut fault = if dedupe_claim {
            AttestationFault {
                friction_id: FRICTION_DEDUPE_NOT_APPROVAL.into(),
                message: "skipped-ppr-dedupe no habilita fusión".into(),
                reasons: vec!["dedupe_not_approval".into(), "attestation_missing".into()],
                source_branch: source_branch.clone(),
                expected_tree_sha: Some(expected_tree),
            }
        } else {
            AttestationFault {
                friction_id: FRICTION_REQUIRED.into(),
                message: "accept-pr exige qa-attestation/1.0 aprobado del tree_sha a fusionar"
                    .into(),
                reasons: vec!["missing_or_unapproved_attestation".into()],
                source_branch: source_branch.clone(),
                expected_tree_sha: Some(expected_tree),
            }
        };
        fault.source_branch = source_branch.or_else(|| str_field(inputs, "source_branch"));
        return Err(fault);
    }

    let delta = resolve_delta_class(inputs, state);
    let delta_arg = Some(delta.as_str());

    match validate_attestation_file(&path, &expected_tree, delta_arg) {
        Ok(hit) => Ok(hit),
        Err(raw) => {
            let mut fault = map_raw_validation_err(&raw);
            if dedupe_claim
                && (fault.friction_id == FRICTION_REQUIRED
                    || raw.to_ascii_lowercase().contains("read attestation")
                    || raw.to_ascii_lowercase().contains("parse attestation"))
            {
                fault.friction_id = FRICTION_DEDUPE_NOT_APPROVAL.into();
                fault.message = "skipped-ppr-dedupe no habilita fusión".into();
                fault.reasons.insert(0, "dedupe_not_approval".into());
            }
            fault.source_branch = source_branch.or_else(|| str_field(inputs, "source_branch"));
            fault.expected_tree_sha = Some(expected_tree);
            Err(fault)
        }
    }
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
            "issuer_process": ISSUER_PROCESS,
        })
    }

    fn write_attestation(repo: &Path, branch: &str, doc: &Value) -> PathBuf {
        let path = attestation_path_for_branch(repo, branch);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, serde_json::to_string_pretty(doc).unwrap()).unwrap();
        path
    }

    fn init_repo_with_branch(branch: &str) -> (tempfile::TempDir, String) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        std::process::Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo)
            .status()
            .expect("git init");
        std::process::Command::new("git")
            .args(["config", "user.email", "t@t"])
            .current_dir(repo)
            .status()
            .ok();
        std::process::Command::new("git")
            .args(["config", "user.name", "t"])
            .current_dir(repo)
            .status()
            .ok();
        fs::write(repo.join("seed.txt"), "seed\n").unwrap();
        std::process::Command::new("git")
            .args(["add", "seed.txt"])
            .current_dir(repo)
            .status()
            .expect("add");
        std::process::Command::new("git")
            .args(["commit", "-qm", "seed"])
            .current_dir(repo)
            .status()
            .expect("commit");
        // Ensure branch name exists
        let _ = std::process::Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(repo)
            .status();
        std::process::Command::new("git")
            .args(["checkout", "-qb", branch])
            .current_dir(repo)
            .status()
            .expect("branch");
        fs::write(repo.join("feat.txt"), "feat\n").unwrap();
        std::process::Command::new("git")
            .args(["add", "feat.txt"])
            .current_dir(repo)
            .status()
            .expect("add feat");
        std::process::Command::new("git")
            .args(["commit", "-qm", "feat"])
            .current_dir(repo)
            .status()
            .expect("commit feat");
        let head = git_output(repo, &["rev-parse", "HEAD"]).expect("head");
        let tree = tree_sha_for_commit(repo, &head).expect("tree");
        (tmp, tree)
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
            "issuer_process": ISSUER_PROCESS,
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

    #[test]
    fn validate_rejects_missing_issuer() {
        let mut doc = sample_doc("t", PROFILE_FULL);
        doc.as_object_mut().unwrap().remove("issuer_process");
        let err = validate_attestation_value(&doc, "t", Some("active")).expect_err("issuer");
        assert!(err.contains("issuer"));
    }

    #[test]
    fn ppr_verdict_from_validacion_frontmatter() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = "docs/features/x";
        fs::create_dir_all(repo.join(persist)).unwrap();
        fs::write(
            repo.join(persist).join("validacion.md"),
            "---\nglobal: APTO\nppr_verdict: aprobado\n---\n\n# v\n",
        )
        .unwrap();
        let state = json!({});
        let inputs = json!({"persist_ref": persist});
        assert!(super::ppr_verdict_aprobado(repo, &state, &inputs));
    }

    #[test]
    fn worktree_allows_only_validacion_in_persist() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        std::process::Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo)
            .status()
            .expect("git init");
        std::process::Command::new("git")
            .args(["config", "user.email", "t@t"])
            .current_dir(repo)
            .status()
            .ok();
        std::process::Command::new("git")
            .args(["config", "user.name", "t"])
            .current_dir(repo)
            .status()
            .ok();
        let persist = repo.join("docs/features/x");
        fs::create_dir_all(&persist).unwrap();
        fs::write(persist.join("validacion.md"), "v\n").unwrap();
        std::process::Command::new("git")
            .args(["add", "docs/features/x/validacion.md"])
            .current_dir(repo)
            .status()
            .expect("git add");
        std::process::Command::new("git")
            .args(["commit", "-qm", "seed"])
            .current_dir(repo)
            .status()
            .expect("git commit");
        fs::write(repo.join("other.txt"), "dirty\n").unwrap();
        let v = worktree_violations_except_validacion(repo, "docs/features/x").expect("status");
        assert_eq!(v, vec!["other.txt".to_string()]);
        fs::remove_file(repo.join("other.txt")).unwrap();
        let v2 = worktree_violations_except_validacion(repo, "docs/features/x").expect("clean");
        assert!(v2.is_empty());
    }

    static LAB_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// KA-PPR-1: sin atestación → aborto con qa_attestation_required.
    #[test]
    fn ka_ppr_1_missing_attestation_aborts() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (tmp, _tree) = init_repo_with_branch("feat/ka-ppr-1");
        let repo = tmp.path();
        let inputs = json!({
            "source_branch": "feat/ka-ppr-1",
            "delta_class": "active"
        });
        let err = validate_for_accept_pr_with_state(repo, &inputs, None).expect_err("abort");
        assert_eq!(err.friction_id, FRICTION_REQUIRED);
    }

    /// KA-PPR-2: docs-only + active, o full sin issuer → aborto.
    #[test]
    fn ka_ppr_2_docs_only_or_forged_full_aborts() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (tmp, tree) = init_repo_with_branch("feat/ka-ppr-2");
        let repo = tmp.path();
        let branch = "feat/ka-ppr-2";

        let docs = sample_doc(&tree, PROFILE_DOCS_ONLY);
        write_attestation(repo, branch, &docs);
        let inputs = json!({
            "source_branch": branch,
            "delta_class": "active"
        });
        let err = validate_for_accept_pr_with_state(repo, &inputs, None).expect_err("docs-only");
        assert_eq!(err.friction_id, FRICTION_PROFILE_BLOCKED);

        let mut forged = sample_doc(&tree, PROFILE_FULL);
        forged.as_object_mut().unwrap().remove("issuer_process");
        write_attestation(repo, branch, &forged);
        let err2 = validate_for_accept_pr_with_state(repo, &inputs, None).expect_err("forged");
        assert_eq!(err2.friction_id, FRICTION_ISSUER_INVALID);
    }

    /// KA-PPR-3: tree_sha divergente → qa_attestation_tree_mismatch.
    #[test]
    fn ka_ppr_3_tree_mismatch_aborts() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (tmp, _tree) = init_repo_with_branch("feat/ka-ppr-3");
        let repo = tmp.path();
        let branch = "feat/ka-ppr-3";
        let doc = sample_doc("deadbeef_diverged", PROFILE_FULL);
        write_attestation(repo, branch, &doc);
        let inputs = json!({
            "source_branch": branch,
            "delta_class": "active"
        });
        let err = validate_for_accept_pr_with_state(repo, &inputs, None).expect_err("mismatch");
        assert_eq!(err.friction_id, FRICTION_TREE_MISMATCH);
    }

    /// KA-PPR-4: solo crédito skipped-ppr-dedupe → no habilita fusión.
    #[test]
    fn ka_ppr_4_dedupe_not_approval() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (tmp, _tree) = init_repo_with_branch("feat/ka-ppr-4");
        let repo = tmp.path();
        let inputs = json!({
            "source_branch": "feat/ka-ppr-4",
            "delta_class": "active",
            "ppr_status": "skipped-ppr-dedupe"
        });
        let err = validate_for_accept_pr_with_state(repo, &inputs, None).expect_err("dedupe");
        assert_eq!(err.friction_id, FRICTION_DEDUPE_NOT_APPROVAL);
    }

    #[test]
    fn ka_ppr_canonical_attestation_passes_gate() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (tmp, tree) = init_repo_with_branch("feat/ka-ppr-ok");
        let repo = tmp.path();
        let branch = "feat/ka-ppr-ok";
        let doc = sample_doc(&tree, PROFILE_FULL);
        write_attestation(repo, branch, &doc);
        let inputs = json!({
            "source_branch": branch,
            "delta_class": "active"
        });
        let hit = validate_for_accept_pr_with_state(repo, &inputs, None).expect("pass");
        assert_eq!(hit.execution_id, "e-1");
    }

    #[test]
    fn lab_skip_bypasses_gate() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (tmp, _tree) = init_repo_with_branch("feat/ka-ppr-lab");
        let repo = tmp.path();
        std::env::set_var(LAB_SKIP_ENV, "1");
        let inputs = json!({"source_branch": "feat/ka-ppr-lab", "delta_class": "active"});
        let r = validate_for_accept_pr_with_state(repo, &inputs, None);
        std::env::remove_var(LAB_SKIP_ENV);
        assert!(r.is_ok());
    }
}
