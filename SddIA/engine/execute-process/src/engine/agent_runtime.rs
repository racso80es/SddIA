//! Runtime de agentes V5 (slice B kalma2-full-cycle).
//!
//! Cuando `SDDIA_AGENT_RUNTIME_COMMAND` está definido, las fases solo-`agent:`
//! invocan ese CLI (JSON stdin → última línea JSON stdout) en lugar de
//! marcarse `simulated`.

use serde_json::{json, Value};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const ENV_CMD: &str = "SDDIA_AGENT_RUNTIME_COMMAND";

pub fn is_configured() -> bool {
    std::env::var(ENV_CMD)
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
}

fn split_command(raw: &str) -> Result<Vec<String>, String> {
    let mut parts: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quote: Option<char> = None;
    for c in raw.trim().chars() {
        match (c, in_quote) {
            ('"', None) => in_quote = Some('"'),
            ('\'', None) => in_quote = Some('\''),
            ('"', Some('"')) | ('\'', Some('\'')) => in_quote = None,
            (' ', None) if cur.is_empty() => {}
            (' ', None) => {
                parts.push(std::mem::take(&mut cur));
            }
            (_, _) => cur.push(c),
        }
    }
    if !cur.is_empty() {
        parts.push(cur);
    }
    if parts.is_empty() {
        return Err("SDDIA_AGENT_RUNTIME_COMMAND vacío".into());
    }
    Ok(parts)
}

fn agent_names(delegates: &[Value]) -> Vec<String> {
    delegates
        .iter()
        .filter_map(|d| d.as_str())
        .filter_map(|s| s.strip_prefix("agent:"))
        .map(str::to_string)
        .collect()
}

/// Invoca el CLI de runtime para una fase de agentes.
/// Respuesta esperada (última línea JSON):
/// `{ "success": bool, "data": { "status": "executed"|"awaiting_agents"|"failed", "message"?: str } }`
pub fn invoke_agent_phase(
    repo: &Path,
    process_name: &str,
    phase_name: &str,
    delegates: &[Value],
    inputs: &Value,
    state: &Value,
    di_binding: Option<Value>,
) -> Value {
    let mut entry = json!({
        "phase_name": phase_name,
        "delegates_to": delegates,
        "handler": "agent-runtime",
    });
    if let Some(di) = &di_binding {
        entry["di_binding"] = di.clone();
    }

    let raw = match std::env::var(ENV_CMD) {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            entry["status"] = json!("simulated");
            entry["note"] = json!("agentes IDE; sin SDDIA_AGENT_RUNTIME_COMMAND");
            return entry;
        }
    };

    let parts = match split_command(&raw) {
        Ok(p) => p,
        Err(e) => {
            entry["status"] = json!("failed");
            entry["error"] = json!(e);
            return entry;
        }
    };
    let (bin, args) = match parts.split_first() {
        Some((b, a)) => (b.clone(), a.to_vec()),
        None => {
            entry["status"] = json!("failed");
            entry["error"] = json!("comando runtime vacío");
            return entry;
        }
    };

    let agents = agent_names(delegates);
    // G4: PPR inyecta `pr_branch`; runtime Kalma2 lee `branch_name`.
    let branch_name = inputs
        .get("branch_name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| json!(s))
        .or_else(|| {
            inputs
                .get("pr_branch")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| json!(s))
        })
        .unwrap_or(Value::Null);
    let mut payload = json!({
        "operation": "AGENT_PHASE",
        "process_name": process_name,
        "phase_name": phase_name,
        "agents": agents,
        "persist_ref": inputs.get("persist_ref"),
        "branch_name": branch_name,
        "correlation_id": inputs.get("correlation_id"),
        "pbi_ref": inputs.get("pbi_ref"),
        "inputs": inputs,
        "workspace_path": state.get("workspace_path")
            .or_else(|| state.get("workspace").and_then(|w| w.get("workspace_path"))),
        "repo_root": repo.display().to_string(),
    });
    if let Some(di) = di_binding {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("di_binding".into(), di);
        }
    }

    let mut child = match Command::new(&bin)
        .args(&args)
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            entry["status"] = json!("failed");
            entry["error"] = json!(format!("spawn agent-runtime: {e}"));
            return entry;
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        let body = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".into());
        if let Err(e) = stdin.write_all(body.as_bytes()) {
            entry["status"] = json!("failed");
            entry["error"] = json!(format!("stdin agent-runtime: {e}"));
            return entry;
        }
    }

    let out = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            entry["status"] = json!("failed");
            entry["error"] = json!(format!("wait agent-runtime: {e}"));
            return entry;
        }
    };

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let line = stdout
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .last()
        .unwrap_or("");

    if line.is_empty() {
        entry["status"] = json!("failed");
        entry["error"] = json!(if stderr.trim().is_empty() {
            format!("agent-runtime sin stdout (exit {})", out.status.code().unwrap_or(1))
        } else {
            stderr.trim().to_string()
        });
        return entry;
    }

    let body: Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(e) => {
            entry["status"] = json!("failed");
            entry["error"] = json!(format!("JSON agent-runtime: {e}"));
            entry["raw_stdout"] = json!(line);
            return entry;
        }
    };

    let success = body.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
    let data = body.get("data").cloned().unwrap_or(json!({}));
    let status = data
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or(if success { "executed" } else { "failed" });

    let normalized = match status {
        "executed" | "awaiting_agents" | "failed" => status,
        "awaiting" => "awaiting_agents",
        _ if success => "executed",
        _ => "failed",
    };

    entry["status"] = json!(normalized);
    if let Some(msg) = data.get("message").and_then(|v| v.as_str()) {
        entry["message"] = json!(msg);
    }
    if let Some(err) = body.get("error").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        entry["error"] = json!(err);
    }
    if !out.status.success() && normalized == "executed" {
        // CLI non-zero: no promover a executed silencioso
        entry["status"] = json!("failed");
        entry["error"] = json!(format!(
            "agent-runtime exit {} pese a status executed",
            out.status.code().unwrap_or(1)
        ));
    }
    entry["runtime_response"] = body;
    entry
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn not_configured_returns_simulated() {
        std::env::remove_var(ENV_CMD);
        let entry = invoke_agent_phase(
            Path::new("."),
            "bug-fix",
            "Diseño del fix",
            &[json!("agent:dedalo")],
            &json!({}),
            &json!({}),
            None,
        );
        assert_eq!(entry["status"], "simulated");
    }

    #[test]
    fn configured_cli_marks_executed() {
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = dir.join("mock-agent.sh");
        fs::write(
            &script,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"executed\",\"message\":\"ok\"}}'\n",
        )
        .unwrap();
        let mut perms = fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script, perms).unwrap();

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "bug-fix",
            "Diseño del fix",
            &[json!("agent:dedalo")],
            &json!({"persist_ref": "docs/fixes/x", "correlation_id": "corr"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "executed", "{entry}");
        assert_eq!(entry["handler"], "agent-runtime");
    }

    #[test]
    fn configured_cli_can_await() {
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-aw-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = dir.join("mock-agent.sh");
        fs::write(
            &script,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"awaiting_agents\"}}'\n",
        )
        .unwrap();
        let mut perms = fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script, perms).unwrap();

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Ejecución",
            &[json!("agent:tekton")],
            &json!({}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "awaiting_agents");
    }

    #[test]
    fn branch_name_coalesces_from_pr_branch() {
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-br-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = dir.join("mock-agent.sh");
        // Echo stdin last line of interest via capturing branch from stdin JSON is heavy;
        // assert payload construction by re-reading ENV path: we check status executed and
        // that invoke succeeds with only pr_branch set (no panic / null-only path).
        fs::write(
            &script,
            r#"#!/bin/sh
python3 -c 'import json,sys; doc=json.load(sys.stdin); assert doc.get("branch_name")=="feat/from-pr", doc; print(json.dumps({"success":True,"data":{"status":"executed","message":"branch-ok"}}))'
"#,
        )
        .unwrap();
        let mut perms = fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script, perms).unwrap();

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Triaje documental",
            &[json!("agent:argos")],
            &json!({"pr_branch": "feat/from-pr", "persist_ref": "docs/features/x"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "executed", "{entry}");
    }
}
