use repo_retention_purger::{apply_plan, build_plan, PlanRequest};
use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::Value;
use std::panic::{self, AssertUnwindSafe};

fn extract_request(root: &Value) -> Value {
    root.get("request").cloned().unwrap_or_else(|| root.clone())
}

fn parse_request(v: &Value) -> PlanRequest {
    serde_json::from_value(v.clone()).unwrap_or(PlanRequest {
        simulate: v
            .get("simulate")
            .and_then(|x| x.as_bool())
            .unwrap_or(true),
        repo_root: v
            .get("repo_root")
            .and_then(|x| x.as_str())
            .map(str::to_string),
        plan_digest: v
            .get("plan_digest")
            .and_then(|x| x.as_str())
            .map(str::to_string),
        older_than_hours: v.get("older_than_hours").and_then(|x| x.as_f64()),
    })
}

fn run(root: &Value) -> Result<Value, repo_retention_purger::plan::PlanError> {
    let req = parse_request(&extract_request(root));
    let plan = build_plan(&req)?;
    if req.simulate {
        return Ok(plan);
    }
    apply_plan(&req, &plan)
}

fn main() {
    let root = read_stdin_json();
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| run(&root)));
    match outcome {
        Ok(Ok(result)) => emit_success(Some(result)),
        Ok(Err(e)) => {
            let (code, msg) = e.code_and_msg();
            emit_error(&format!("{code}: {msg}"), 1);
        }
        Err(_) => emit_error("panic capturado en repo-retention-purger", 1),
    }
}
