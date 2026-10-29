//! Reciclado post-merge de órganos vivos (MAN-25 KZ-CA4).

use super::capsule_seal;
use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;

pub fn maybe_recycle_after_merge(repo: &Path, merge_commit: &str) -> Result<Value, String> {
    if std::env::var("SDDIA_LAB_SKIP_RUNTIME_RECYCLE")
        .ok()
        .map(|s| matches!(s.trim(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
    {
        return Ok(json!({
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_RUNTIME_RECYCLE",
        }));
    }

    let paths = changed_paths_in_merge(repo, merge_commit)?;
    if !sddia_runtime_parity::merge_touches_live_organs(&paths) {
        return Ok(json!({
            "skipped": true,
            "reason": "merge_sin_organos_vivos",
            "changed_count": paths.len(),
        }));
    }

    let target_dir = repo.join("SddIA/target");
    let build = Command::new("cargo")
        .args([
            "build",
            "--release",
            "-p",
            "execute-process",
            "-p",
            "kalma2-bridge",
        ])
        .current_dir(repo.join("SddIA"))
        .env("CARGO_TARGET_DIR", &target_dir)
        .output()
        .map_err(|e| e.to_string())?;
    if !build.status.success() {
        return Err(format!(
            "runtime-recycle: cargo build falló: {}",
            String::from_utf8_lossy(&build.stderr)
        ));
    }

    let rel_ep = target_dir.join("release/execute-process");
    if !rel_ep.is_file() {
        return Err("runtime-recycle: execute-process release ausente tras build".into());
    }

    let seal = capsule_seal::run_batch(
        repo,
        &json!({
            "profile": "release",
            "write_genome": false,
            "write_witness": true,
            "names": ["execute-process", "kalma2-bridge"],
        }),
    )?;

    let restart = restart_live_daemons();

    Ok(json!({
        "recycled": true,
        "merge_commit": merge_commit,
        "touched_paths_sample": paths.into_iter().take(12).collect::<Vec<_>>(),
        "seal": seal,
        "restart": restart,
    }))
}

fn changed_paths_in_merge(repo: &Path, merge_commit: &str) -> Result<Vec<String>, String> {
    let out = Command::new("git")
        .args([
            "-C",
            &repo.to_string_lossy(),
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            merge_commit,
        ])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "git diff-tree falló: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect())
}

fn restart_live_daemons() -> serde_json::Value {
    let units = ["sddia-kalma2-bridge", "sddia-event-watcher"];
    let mut rows = Vec::new();
    for unit in units {
        let attempt = Command::new("systemctl")
            .args(["--user", "try-restart", &format!("{unit}@")])
            .output();
        rows.push(match attempt {
            Ok(o) => json!({
                "unit": unit,
                "ok": o.status.success(),
                "stderr": String::from_utf8_lossy(&o.stderr).trim(),
            }),
            Err(e) => json!({
                "unit": unit,
                "ok": false,
                "error": e.to_string(),
            }),
        });
    }
    json!({ "units": rows })
}
