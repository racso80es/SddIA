//! Sello de diff en `{persist_ref}/validacion.md` (AUT-36 CA-3 / AUT-39).

use crate::core::parser::{load_frontmatter_yaml, parse_frontmatter, parse_frontmatter_from_str};
use crate::engine::item_lifecycle_frontmatter::seal_markdown;
use crate::engine::qa_attestation::tree_sha_for_commit;
use serde_json::{json, Value as JsonValue};
use serde_yaml::Value as YamlValue;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSeal {
    pub validated_base_sha: String,
    pub validated_head_sha: String,
    pub validated_tree_sha: String,
    pub diff_digest: String,
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

fn persist_rel(repo: &Path, persist_ref: &str) -> String {
    let trimmed = persist_ref.trim();
    if Path::new(trimmed).is_absolute() {
        Path::new(trimmed)
            .strip_prefix(repo)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| trimmed.to_string())
    } else {
        trimmed.trim_start_matches("./").to_string()
    }
}

fn validacion_rel(repo: &Path, persist_ref: &str) -> String {
    format!("{}/validacion.md", persist_rel(repo, persist_ref).trim_end_matches('/'))
}

pub fn compute_diff_seal(
    repo: &Path,
    branch: &str,
    target_branch: &str,
    persist_ref: &str,
) -> Result<DiffSeal, String> {
    let head_sha = git_output(
        repo,
        &[
            "rev-parse",
            "--verify",
            &format!("refs/heads/{branch}"),
        ],
    )
    .or_else(|_| git_output(repo, &["rev-parse", "--verify", "HEAD"]))?;
    compute_diff_seal_at_head(repo, &head_sha, target_branch, persist_ref)
}

pub fn compute_diff_seal_at_head(
    repo: &Path,
    head_sha: &str,
    target_branch: &str,
    persist_ref: &str,
) -> Result<DiffSeal, String> {
    let tree_sha = tree_sha_for_commit(repo, head_sha)?;
    let merge_base = git_output(
        repo,
        &[
            "merge-base",
            &format!("origin/{target_branch}"),
            head_sha,
        ],
    )
    .or_else(|_| git_output(repo, &["merge-base", target_branch, head_sha]))?;
    let digest = recompute_digest_excluding_validacion(repo, &merge_base, head_sha, persist_ref)?;
    Ok(DiffSeal {
        validated_base_sha: merge_base,
        validated_head_sha: head_sha.to_string(),
        validated_tree_sha: tree_sha,
        diff_digest: digest,
    })
}

const SEAL_KEYS: [&str; 4] = [
    "validated_base_sha",
    "validated_head_sha",
    "validated_tree_sha",
    "diff_digest",
];

fn load_frontmatter_at(
    repo: &Path,
    head_sha: &str,
    rel: &str,
) -> Result<std::collections::HashMap<String, YamlValue>, String> {
    let raw = git_output(repo, &["show", &format!("{head_sha}:{rel}")])?;
    parse_frontmatter_from_str(&raw)
}

pub fn read_seal_from_validacion_at(
    repo: &Path,
    head_sha: &str,
    persist_ref: &str,
) -> Result<Option<DiffSeal>, String> {
    let rel = validacion_rel(repo, persist_ref);
    let fm = match load_frontmatter_at(repo, head_sha, &rel) {
        Ok(f) => f,
        Err(_) => return Ok(None),
    };
    let mut any = false;
    let mut values: [String; 4] = std::array::from_fn(|_| String::new());
    for (i, key) in SEAL_KEYS.iter().enumerate() {
        if let Some(v) = fm.get(*key).and_then(|x| x.as_str()) {
            let t = v.trim();
            if !t.is_empty() {
                any = true;
                values[i] = t.to_string();
            }
        }
    }
    if !any {
        return Ok(None);
    }
    Ok(Some(DiffSeal {
        validated_base_sha: values[0].clone(),
        validated_head_sha: values[1].clone(),
        validated_tree_sha: values[2].clone(),
        diff_digest: values[3].clone(),
    }))
}

pub fn read_seal_from_validacion(
    repo: &Path,
    persist_ref: &str,
) -> Result<Option<DiffSeal>, String> {
    let head = git_output(repo, &["rev-parse", "HEAD"])?;
    read_seal_from_validacion_at(repo, &head, persist_ref)
}

pub fn validacion_requires_ci_seal_check_at(
    repo: &Path,
    head_sha: &str,
    persist_ref: &str,
) -> Result<bool, String> {
    let rel = validacion_rel(repo, persist_ref);
    let fm = match load_frontmatter_at(repo, head_sha, &rel) {
        Ok(f) => f,
        Err(_) => return Ok(false),
    };
    if read_seal_from_validacion_at(repo, head_sha, persist_ref)?.is_some() {
        return Ok(true);
    }
    let global_v = fm
        .get("global")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .eq_ignore_ascii_case("APTO");
    Ok(global_v)
}

pub fn validacion_requires_ci_seal_check(
    repo: &Path,
    persist_ref: &str,
) -> Result<bool, String> {
    let head = git_output(repo, &["rev-parse", "HEAD"])?;
    validacion_requires_ci_seal_check_at(repo, &head, persist_ref)
}

pub fn verify_diff_seal_at_head(
    repo: &Path,
    head_sha: &str,
    target_branch: &str,
    persist_ref: &str,
) -> Result<(), String> {
    if !validacion_requires_ci_seal_check_at(repo, head_sha, persist_ref)? {
        return Ok(());
    }
    let path = validacion_rel(repo, persist_ref);
    let fm = load_frontmatter_at(repo, head_sha, &path)?;
    for key in SEAL_KEYS {
        let present = fm
            .get(key)
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .is_some();
        if !present {
            return Err(format!(
                "{path}: falta campo de sello `{key}` (AUT-40 / AUT-36 CA-4)"
            ));
        }
    }
    let sealed = read_seal_from_validacion_at(repo, head_sha, persist_ref)?
        .ok_or_else(|| format!("{path}: sello incompleto"))?;
    let computed_at_tip =
        compute_diff_seal_at_head(repo, head_sha, target_branch, persist_ref)?;
    if sealed.diff_digest != computed_at_tip.diff_digest {
        return Err(format!(
            "{path}: diff_digest no coincide (sellado={}, recalculado={})",
            sealed.diff_digest, computed_at_tip.diff_digest
        ));
    }
    if sealed.validated_base_sha != computed_at_tip.validated_base_sha {
        return Err(format!(
            "{path}: validated_base_sha no coincide (sellado={}, actual={})",
            sealed.validated_base_sha, computed_at_tip.validated_base_sha
        ));
    }
    let anchor = sealed.validated_head_sha.trim();
    if anchor != head_sha {
        let mb = git_output(
            repo,
            &["merge-base", anchor, head_sha],
        )?;
        if mb != anchor {
            return Err(format!(
                "{path}: validated_head_sha {anchor} no es ancestro de HEAD {head_sha}"
            ));
        }
    }
    let tree_at_anchor = tree_sha_for_commit(repo, anchor)?;
    if sealed.validated_tree_sha != tree_at_anchor {
        return Err(format!(
            "{path}: validated_tree_sha no coincide (sellado={}, árbol en {anchor}={tree_at_anchor})",
            sealed.validated_tree_sha
        ));
    }
    Ok(())
}

/// `persist_ref` relativos bajo `docs/{features,fixes,refactors}/` con `validacion.md` en HEAD.
pub fn list_persist_refs_with_validacion(repo: &Path, head_sha: &str) -> Result<Vec<String>, String> {
    let out = git_output(
        repo,
        &[
            "ls-tree",
            "-r",
            "--name-only",
            head_sha,
            "--",
            "docs/features",
            "docs/fixes",
            "docs/refactors",
        ],
    )?;
    let mut refs: Vec<String> = out
        .lines()
        .filter(|l| l.ends_with("/validacion.md"))
        .map(|l| l.trim_end_matches("/validacion.md").to_string())
        .collect();
    refs.sort();
    refs.dedup();
    Ok(refs)
}

pub fn verify_diff_seal_ci(
    repo: &Path,
    head_sha: &str,
    target_branch: &str,
) -> Result<JsonValue, String> {
    let persist_refs = list_persist_refs_with_validacion(repo, head_sha)?;
    let mut checked = Vec::new();
    let mut skipped = Vec::new();
    let mut errors = Vec::new();
    for persist_ref in persist_refs {
        match validacion_requires_ci_seal_check_at(repo, head_sha, &persist_ref) {
            Ok(true) => {
                match verify_diff_seal_at_head(repo, head_sha, target_branch, &persist_ref) {
                    Ok(()) => checked.push(persist_ref),
                    Err(e) => errors.push(json!({"persist_ref": persist_ref, "error": e})),
                }
            }
            Ok(false) => skipped.push(persist_ref),
            Err(e) => errors.push(json!({"persist_ref": persist_ref, "error": e})),
        }
    }
    let ok = errors.is_empty();
    Ok(json!({
        "ok": ok,
        "head_sha": head_sha,
        "target_branch": target_branch,
        "checked": checked,
        "skipped": skipped,
        "errors": errors,
    }))
}

fn resolve_branch_head_sha(repo: &Path, branch: &str) -> Result<String, String> {
    git_output(
        repo,
        &[
            "rev-parse",
            "--verify",
            &format!("refs/heads/{branch}"),
        ],
    )
    .or_else(|_| {
        git_output(
            repo,
            &["rev-parse", "--verify", &format!("origin/{branch}")],
        )
    })
}

fn lab_skip_diff_seal_accept() -> bool {
    for key in ["SDDIA_LAB_SKIP_DIFF_SEAL_ACCEPT", "SDDIA_LAB_SKIP_DIFF_SEAL"] {
        if std::env::var(key)
            .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false)
        {
            return true;
        }
    }
    false
}

/// Gate AUT-41: bloquea merge en `accept-pr` si el sello no cubre el diff actual de la rama origen.
pub fn validate_for_accept_pr_merge(
    repo: &Path,
    source_branch: &str,
    target_branch: &str,
    persist_ref_explicit: Option<&str>,
) -> Result<(), String> {
    if lab_skip_diff_seal_accept() {
        return Ok(());
    }
    let head = resolve_branch_head_sha(repo, source_branch)?;
    let mut to_check: Vec<String> = Vec::new();
    if let Some(p) = persist_ref_explicit {
        let t = p.trim().trim_start_matches("./");
        if !t.is_empty() {
            to_check.push(t.to_string());
        }
    }
    for p in list_persist_refs_with_validacion(repo, &head)? {
        if !to_check.contains(&p) {
            to_check.push(p);
        }
    }
    for persist in to_check {
        if !validacion_requires_ci_seal_check_at(repo, &head, &persist)? {
            continue;
        }
        verify_diff_seal_at_head(repo, &head, target_branch, &persist)?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResealOutcome {
    /// Sin `global: APTO` ni campos de sello previos.
    SkippedNoScope,
    /// Digest ya alineado con HEAD.
    Unchanged,
    /// `validacion.md` sellado y commiteado.
    Committed {
        diff_digest: String,
    },
}

/// AUT-42: actualiza sello en working tree y commitea (pre-push PR OPEN).
pub fn reseal_validacion_for_branch(
    repo: &Path,
    head_sha: &str,
    branch: &str,
    target_branch: &str,
    persist_ref: &str,
) -> Result<ResealOutcome, String> {
    if lab_skip_diff_seal_accept() {
        return Ok(ResealOutcome::SkippedNoScope);
    }
    if !validacion_requires_ci_seal_check_at(repo, head_sha, persist_ref)? {
        return Ok(ResealOutcome::SkippedNoScope);
    }
    validacion_ready_for_seal(repo, persist_ref)?;
    let computed = compute_diff_seal_at_head(repo, head_sha, target_branch, persist_ref)?;
    if let Some(sealed) = read_seal_from_validacion_at(repo, head_sha, persist_ref)? {
        if sealed.diff_digest == computed.diff_digest
            && sealed.validated_base_sha == computed.validated_base_sha
        {
            return Ok(ResealOutcome::Unchanged);
        }
    }
    apply_diff_seal_to_validacion(repo, persist_ref, &computed)?;
    let rel = validacion_rel(repo, persist_ref);
    let add = Command::new("git")
        .args(["add", &rel])
        .current_dir(repo)
        .status()
        .map_err(|e| format!("git add: {e}"))?;
    if !add.success() {
        return Err("git add validacion.md falló".into());
    }
    let msg = format!("pre-push: re-sello diff validacion ({branch})");
    let commit = Command::new("git")
        .args(["commit", "-m", &msg])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("git commit: {e}"))?;
    if !commit.status.success() {
        let stderr = String::from_utf8_lossy(&commit.stderr);
        if stderr.contains("nothing to commit") {
            return Ok(ResealOutcome::Unchanged);
        }
        return Err(format!("git commit re-sello: {stderr}"));
    }
    Ok(ResealOutcome::Committed {
        diff_digest: computed.diff_digest,
    })
}

pub fn validacion_ready_for_seal(repo: &Path, persist_ref: &str) -> Result<(), String> {
    let path = repo.join(validacion_rel(repo, persist_ref));
    if !path.is_file() {
        return Err("validacion.md ausente".into());
    }
    let fm = load_frontmatter_yaml(&path).map_err(|e| e.to_string())?;
    let global_v = fm
        .get("global")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_uppercase();
    if global_v != "APTO" {
        return Err(format!("global={global_v}; se exige APTO"));
    }
    Ok(())
}

pub fn apply_diff_seal_to_validacion(
    repo: &Path,
    persist_ref: &str,
    seal: &DiffSeal,
) -> Result<PathBuf, String> {
    let rel = validacion_rel(repo, persist_ref);
    let path = repo.join(&rel);
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let patch: YamlValue = serde_yaml::from_str(&format!(
        "validated_base_sha: \"{}\"\nvalidated_head_sha: \"{}\"\nvalidated_tree_sha: \"{}\"\ndiff_digest: \"{}\"\n",
        seal.validated_base_sha,
        seal.validated_head_sha,
        seal.validated_tree_sha,
        seal.diff_digest,
    ))
    .map_err(|e| e.to_string())?;
    let sealed = seal_markdown(&raw, &patch)?;
    fs::write(&path, &sealed).map_err(|e| e.to_string())?;
    Ok(path)
}

/// Recalcula digest excluyendo validacion.md (test de no autorreferencia).
pub fn recompute_digest_excluding_validacion(
    repo: &Path,
    base_sha: &str,
    head_sha: &str,
    persist_ref: &str,
) -> Result<String, String> {
    let exclude = validacion_rel(repo, persist_ref);
    let diff_text = git_output(
        repo,
        &[
            "diff",
            &format!("{base_sha}..{head_sha}"),
            "--",
            ".",
            &format!(":(exclude){exclude}"),
        ],
    )?;
    let mut hasher = Sha256::new();
    hasher.update(diff_text.as_bytes());
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

pub fn seal_patch_json(seal: &DiffSeal) -> JsonValue {
    json!({
        "validated_base_sha": seal.validated_base_sha,
        "validated_head_sha": seal.validated_head_sha,
        "validated_tree_sha": seal.validated_tree_sha,
        "diff_digest": seal.diff_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn digest_excludes_validacion_md_from_diff() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo)
            .status()
            .expect("init");
        for key in [("user.email", "t@t"), ("user.name", "t")] {
            Command::new("git")
                .args(["config", key.0, key.1])
                .current_dir(repo)
                .status()
                .ok();
        }
        fs::write(repo.join("code.txt"), "v1\n").unwrap();
        Command::new("git")
            .args(["add", "code.txt"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "base"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["checkout", "-qb", "feat/x"])
            .current_dir(repo)
            .status()
            .unwrap();
        let persist = "docs/features/x";
        fs::create_dir_all(repo.join(persist)).unwrap();
        fs::write(repo.join("code.txt"), "v2\n").unwrap();
        Command::new("git")
            .args(["add", "code.txt"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "code"])
            .current_dir(repo)
            .status()
            .unwrap();
        let base = git_output(repo, &["merge-base", "main", "HEAD"]).unwrap();
        let head = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        let d0 = recompute_digest_excluding_validacion(repo, &base, &head, persist).unwrap();
        fs::write(
            repo.join("docs/features/x/validacion.md"),
            "---\nglobal: APTO\n---\n\n# v\n",
        )
        .unwrap();
        let d1 = recompute_digest_excluding_validacion(repo, &base, &head, persist).unwrap();
        assert_eq!(d0, d1, "solo validacion.md no debe cambiar el digest");
    }

    #[test]
    fn apply_seal_writes_frontmatter_fields() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo)
            .status()
            .expect("init");
        for key in [("user.email", "t@t"), ("user.name", "t")] {
            Command::new("git")
                .args(["config", key.0, key.1])
                .current_dir(repo)
                .status()
                .ok();
        }
        fs::write(repo.join("a.txt"), "1\n").unwrap();
        Command::new("git")
            .args(["add", "a.txt"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "base"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["checkout", "-qb", "feat/y"])
            .current_dir(repo)
            .status()
            .unwrap();
        let persist = "docs/features/y";
        fs::create_dir_all(repo.join(persist)).unwrap();
        fs::write(
            repo.join("docs/features/y/validacion.md"),
            "---\nglobal: APTO\nbranch: feat/y\n---\n\n# ok\n",
        )
        .unwrap();
        fs::write(repo.join("a.txt"), "2\n").unwrap();
        Command::new("git")
            .args(["add", "a.txt"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "change"])
            .current_dir(repo)
            .status()
            .unwrap();
        validacion_ready_for_seal(repo, persist).expect("ready");
        let seal = compute_diff_seal(repo, "feat/y", "main", persist).expect("seal");
        apply_diff_seal_to_validacion(repo, persist, &seal).expect("apply");
        let fm = parse_frontmatter(&repo.join("docs/features/y/validacion.md")).unwrap();
        assert_eq!(
            fm.get("diff_digest").and_then(|v| v.as_str()),
            Some(seal.diff_digest.as_str())
        );
        assert_eq!(
            fm.get("validated_head_sha").and_then(|v| v.as_str()),
            Some(seal.validated_head_sha.as_str())
        );
    }

    #[test]
    fn ci_verify_fails_when_apto_without_seal_fields() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        init_repo_main_feat(repo, "feat/z", "docs/features/z");
        fs::write(
            repo.join("docs/features/z/validacion.md"),
            "---\nglobal: APTO\n---\n\n# ok\n",
        )
        .unwrap();
        Command::new("git")
            .args(["add", "docs/features/z/validacion.md"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "val"])
            .current_dir(repo)
            .status()
            .unwrap();
        let head = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        let err = verify_diff_seal_at_head(repo, &head, "main", "docs/features/z").unwrap_err();
        assert!(err.contains("validated_base_sha"));
    }

    #[test]
    fn ci_verify_fails_on_digest_mismatch() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        init_repo_main_feat(repo, "feat/w", "docs/features/w");
        fs::write(
            repo.join("docs/features/w/validacion.md"),
            "---\nglobal: APTO\n---\n\n# ok\n",
        )
        .unwrap();
        fs::write(repo.join("code.txt"), "v2\n").unwrap();
        Command::new("git")
            .args(["add", "code.txt"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "change"])
            .current_dir(repo)
            .status()
            .unwrap();
        let head = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        let seal = compute_diff_seal_at_head(repo, &head, "main", "docs/features/w").unwrap();
        apply_diff_seal_to_validacion(repo, "docs/features/w", &seal).unwrap();
        Command::new("git")
            .args(["add", "docs/features/w/validacion.md"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "seal"])
            .current_dir(repo)
            .status()
            .unwrap();
        fs::write(repo.join("code.txt"), "v3\n").unwrap();
        Command::new("git")
            .args(["add", "code.txt"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "drift"])
            .current_dir(repo)
            .status()
            .unwrap();
        let head2 = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        let err =
            verify_diff_seal_at_head(repo, &head2, "main", "docs/features/w").unwrap_err();
        assert!(err.contains("diff_digest") || err.contains("validated_head_sha"));
    }

    #[test]
    fn ci_verify_passes_when_seal_matches() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        init_repo_main_feat(repo, "feat/v", "docs/features/v");
        fs::write(
            repo.join("docs/features/v/validacion.md"),
            "---\nglobal: APTO\n---\n\n# ok\n",
        )
        .unwrap();
        Command::new("git")
            .args(["add", "docs/features/v/validacion.md"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "val"])
            .current_dir(repo)
            .status()
            .unwrap();
        let head = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        let seal = compute_diff_seal_at_head(repo, &head, "main", "docs/features/v").unwrap();
        apply_diff_seal_to_validacion(repo, "docs/features/v", &seal).unwrap();
        Command::new("git")
            .args(["add", "docs/features/v/validacion.md"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "seal"])
            .current_dir(repo)
            .status()
            .unwrap();
        let head = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        verify_diff_seal_at_head(repo, &head, "main", "docs/features/v").expect("match");
    }

    #[test]
    fn accept_pr_merge_blocks_when_seal_stale() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        init_repo_main_feat(repo, "feat/ap", "docs/features/ap");
        fs::write(
            repo.join("docs/features/ap/validacion.md"),
            "---\nglobal: APTO\n---\n\n# ok\n",
        )
        .unwrap();
        Command::new("git")
            .args(["add", "docs/features/ap/validacion.md"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "val"])
            .current_dir(repo)
            .status()
            .unwrap();
        let head = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        let seal = compute_diff_seal_at_head(repo, &head, "main", "docs/features/ap").unwrap();
        apply_diff_seal_to_validacion(repo, "docs/features/ap", &seal).unwrap();
        Command::new("git")
            .args(["add", "docs/features/ap/validacion.md"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "seal"])
            .current_dir(repo)
            .status()
            .unwrap();
        fs::write(repo.join("code.txt"), "drift\n").unwrap();
        Command::new("git")
            .args(["add", "code.txt"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "drift"])
            .current_dir(repo)
            .status()
            .unwrap();
        let err = validate_for_accept_pr_merge(repo, "feat/ap", "main", None).unwrap_err();
        assert!(err.contains("diff_digest") || err.contains("validated_head_sha"));
    }

    #[test]
    fn reseal_commits_when_digest_changes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        init_repo_main_feat(repo, "feat/r", "docs/features/r");
        fs::write(
            repo.join("docs/features/r/validacion.md"),
            "---\nglobal: APTO\n---\n\n# ok\n",
        )
        .unwrap();
        Command::new("git")
            .args(["add", "docs/features/r/validacion.md"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "val"])
            .current_dir(repo)
            .status()
            .unwrap();
        let head = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        let seal = compute_diff_seal_at_head(repo, &head, "main", "docs/features/r").unwrap();
        apply_diff_seal_to_validacion(repo, "docs/features/r", &seal).unwrap();
        Command::new("git")
            .args(["add", "docs/features/r/validacion.md"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "seal"])
            .current_dir(repo)
            .status()
            .unwrap();
        fs::write(repo.join("code.txt"), "drift\n").unwrap();
        Command::new("git")
            .args(["add", "code.txt"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "drift"])
            .current_dir(repo)
            .status()
            .unwrap();
        let head2 = git_output(repo, &["rev-parse", "HEAD"]).unwrap();
        match reseal_validacion_for_branch(repo, &head2, "feat/r", "main", "docs/features/r")
            .expect("reseal")
        {
            ResealOutcome::Committed { .. } => {}
            other => panic!("expected committed, got {other:?}"),
        }
        let fm = parse_frontmatter(&repo.join("docs/features/r/validacion.md")).unwrap();
        assert!(fm.get("diff_digest").and_then(|v| v.as_str()).is_some());
    }

    fn init_repo_main_feat(repo: &Path, branch: &str, persist: &str) {
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo)
            .status()
            .expect("init");
        for key in [("user.email", "t@t"), ("user.name", "t")] {
            Command::new("git")
                .args(["config", key.0, key.1])
                .current_dir(repo)
                .status()
                .ok();
        }
        fs::create_dir_all(repo.join(persist)).unwrap();
        fs::write(repo.join("code.txt"), "v1\n").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "base"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["branch", "-M", "main"])
            .current_dir(repo)
            .status()
            .unwrap();
        Command::new("git")
            .args(["checkout", "-qb", branch])
            .current_dir(repo)
            .status()
            .unwrap();
    }
}
