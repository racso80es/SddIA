//! AUT-42: re-sello de validacion.md en push sobre PR OPEN (pre-push).

use execute_process::engine::diff_seal::{self, ResealOutcome};
use std::path::Path;
use std::process::Command;

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let json_out = args.iter().any(|a| a == "--json");
    let Some(persist_ref) = flag_value(args, "--persist-ref") else {
        eprintln!("reseal-diff-validacion requiere --persist-ref");
        return 1;
    };
    let branch = flag_value(args, "--branch").unwrap_or("HEAD");
    let target_branch = flag_value(args, "--target-branch").unwrap_or("main");
    let head_sha = match flag_value(args, "--head") {
        Some(h) => h.to_string(),
        None => match git_rev(repo, &["rev-parse", "HEAD"]) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("{e}");
                return 1;
            }
        },
    };
    match diff_seal::reseal_validacion_for_branch(
        repo,
        &head_sha,
        branch,
        target_branch,
        persist_ref,
    ) {
        Ok(outcome) => {
            let report = match &outcome {
                ResealOutcome::SkippedNoScope => serde_json::json!({
                    "outcome": "skipped_no_scope",
                }),
                ResealOutcome::Unchanged => serde_json::json!({
                    "outcome": "unchanged",
                }),
                ResealOutcome::Committed { diff_digest } => serde_json::json!({
                    "outcome": "committed",
                    "diff_digest": diff_digest,
                }),
            };
            if json_out {
                println!("{}", serde_json::to_string(&report).unwrap_or_default());
            } else {
                println!("reseal-diff-validacion: {}", report["outcome"]);
            }
            0
        }
        Err(e) => {
            eprintln!("reseal-diff-validacion: {e}");
            2
        }
    }
}

fn git_rev(repo: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| format!("git spawn: {e}"))?;
    if !out.status.success() {
        return Err(format!("git {} failed", args.join(" ")));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}
