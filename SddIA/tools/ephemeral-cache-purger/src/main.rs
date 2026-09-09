mod jail;

use jail::{human_size, plan_purge, JailError, PurgePlan};
use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::panic::{self, AssertUnwindSafe};

fn extract_request(root: &Value) -> &Value {
    root.get("request").unwrap_or(root)
}

fn as_bool(v: &Value, key: &str) -> Option<bool> {
    v.get(key).and_then(|x| x.as_bool())
}

fn as_f64(v: &Value, key: &str, default: f64) -> f64 {
    v.get(key)
        .and_then(|x| x.as_f64().or_else(|| x.as_u64().map(|n| n as f64)))
        .unwrap_or(default)
}

fn plan_to_result(plan: &PurgePlan) -> Value {
    json!({
        "simulated": plan.simulated,
        "target_dir": plan.target_dir.to_string_lossy(),
        "bytes_scanned": plan.bytes_scanned,
        "human_size": human_size(plan.bytes_scanned),
        "candidate_targets": plan.candidate_targets.iter().map(|p| p.to_string_lossy().into_owned()).collect::<Vec<_>>(),
        "purged_directories_count": plan.purged_directories_count,
        "purged_files_count": plan.purged_files_count,
        "errors": plan.errors,
    })
}

fn run(root: &Value) -> Result<PurgePlan, JailError> {
    let req = extract_request(root);
    let simulate = as_bool(req, "simulate").ok_or_else(|| {
        JailError::Io("simulate (boolean) requerido".into())
    })?;
    let target = req
        .get("target_dir")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(jail::DEFAULT_TARGET);
    let older = as_f64(req, "older_than_hours", 0.0);
    let purge_root = as_bool(req, "purge_sandbox_root").unwrap_or(false);
    plan_purge(target, simulate, older, purge_root)
}

fn main() {
    let root = read_stdin_json();
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| run(&root)));
    match outcome {
        Ok(Ok(plan)) => {
            let failed_purge = !plan.simulated
                && !plan.candidate_targets.is_empty()
                && plan.purged_directories_count == 0
                && plan.purged_files_count == 0
                && plan
                    .errors
                    .iter()
                    .any(|e| e.code == "IO_PERMISSION_DENIED");
            if failed_purge {
                emit_error(
                    "IO_PERMISSION_DENIED: ningún candidato purgado",
                    1,
                );
            } else {
                emit_success(Some(plan_to_result(&plan)));
            }
        }
        Ok(Err(e)) => {
            let (code, msg) = e.code_and_msg();
            emit_error(&format!("{code}: {msg}"), 1);
        }
        Err(_) => {
            emit_error("panic capturado en ephemeral-cache-purger", 1);
        }
    }
}
