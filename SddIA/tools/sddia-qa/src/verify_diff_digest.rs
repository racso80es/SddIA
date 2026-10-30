//! AUT-40: verifica `diff_digest` sellado en `{persist_ref}/validacion.md` (paridad CI con DCC).

use execute_process::engine::diff_seal;
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
    if flag_value(args, "--base").is_some() {
        eprintln!("verify-diff-digest: --base reservado; merge-base se calcula vía target-branch");
    }
    match diff_seal::verify_diff_seal_ci(repo, &head_sha, target_branch) {
        Ok(report) => {
            if json_out {
                println!("{}", serde_json::to_string(&report).unwrap_or_default());
            } else if report
                .get("ok")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                println!("verify-diff-digest: OK");
            } else {
                eprintln!("verify-diff-digest: FAILED");
                if let Some(arr) = report.get("errors").and_then(|v| v.as_array()) {
                    for e in arr {
                        eprintln!("  {}", e);
                    }
                }
            }
            if report.get("ok").and_then(|v| v.as_bool()) == Some(true) {
                0
            } else {
                2
            }
        }
        Err(e) => {
            eprintln!("verify-diff-digest: {e}");
            1
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
        return Err(format!(
            "git {} failed",
            args.join(" ")
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}
