//! Sello de diff en `{persist_ref}/validacion.md` (AUT-36 CA-3 / AUT-39).

use crate::core::parser::{load_frontmatter_yaml, parse_frontmatter};
use crate::engine::item_lifecycle_frontmatter::seal_markdown;
use crate::engine::qa_attestation::tree_sha_for_commit;
use serde_json::{json, Value};
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
    let tree_sha = tree_sha_for_commit(repo, &head_sha)?;
    let merge_base = git_output(
        repo,
        &[
            "merge-base",
            &format!("origin/{target_branch}"),
            &head_sha,
        ],
    )
    .or_else(|_| {
        git_output(
            repo,
            &["merge-base", target_branch, &head_sha],
        )
    })?;
    let exclude = validacion_rel(repo, persist_ref);
    let diff_text = git_output(
        repo,
        &[
            "diff",
            &format!("{merge_base}..{head_sha}"),
            "--",
            ".",
            &format!(":(exclude){exclude}"),
        ],
    )?;
    let mut hasher = Sha256::new();
    hasher.update(diff_text.as_bytes());
    Ok(DiffSeal {
        validated_base_sha: merge_base,
        validated_head_sha: head_sha,
        validated_tree_sha: tree_sha,
        diff_digest: format!("sha256:{:x}", hasher.finalize()),
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

pub fn seal_patch_json(seal: &DiffSeal) -> Value {
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
}
