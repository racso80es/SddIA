//! Runtime de agentes V5 (slice B kalma2-full-cycle).
//!
//! Cuando `SDDIA_AGENT_RUNTIME_COMMAND` está definido, las fases solo-`agent:`
//! invocan ese CLI (JSON stdin → última línea JSON stdout) en lugar de
//! marcarse `simulated`.

use crate::core::parser::parse_frontmatter_from_str;
use serde_json::{json, Value};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::Duration;

const ENV_CMD: &str = "SDDIA_AGENT_RUNTIME_COMMAND";
const ENV_RELAY: &str = "SDDIA_AGENT_RELAY_IDE";
const ENV_DEPTH: &str = "SDDIA_AGENT_RUNTIME_DEPTH";
const ENV_TIMEOUT: &str = "SDDIA_AGENT_RUNTIME_TIMEOUT_SECS";
const ENV_TIMEOUT_EXEC: &str = "SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION";
const DEFAULT_TIMEOUT_SECS: u64 = 660;
const KILL_GRACE_MS: u64 = 500;

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn lab_relay_active() -> bool {
    env_truthy(ENV_RELAY)
}

fn agent_runtime_depth() -> u32 {
    std::env::var(ENV_DEPTH)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(0)
}

pub fn is_configured() -> bool {
    if lab_relay_active() {
        return false;
    }
    std::env::var(ENV_CMD)
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
}

fn resolve_timeout_secs(phase_name: &str) -> u64 {
    let phase_l = phase_name.to_lowercase();
    if phase_l.starts_with("ejecuc") {
        if let Ok(v) = std::env::var(ENV_TIMEOUT_EXEC) {
            if let Ok(n) = v.trim().parse::<u64>() {
                return n;
            }
        }
    }
    std::env::var(ENV_TIMEOUT)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(DEFAULT_TIMEOUT_SECS)
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

fn resolve_persist_ref_value(inputs: &Value, state: &Value) -> Value {
    inputs
        .get("persist_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| json!(s))
        .or_else(|| {
            state
                .get("workspace")
                .and_then(|w| w.get("persist_ref"))
                .cloned()
                .filter(|v| v.as_str().map(|s| !s.trim().is_empty()).unwrap_or(false))
        })
        .unwrap_or(Value::Null)
}

fn resolve_execution_id(inputs: &Value, state: &Value) -> Option<String> {
    inputs
        .get("execution_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| {
            state
                .get("execution_id")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })
}

/// Guard L-CONFLICT: artefactos con `execution_id` distinto del ciclo vivo.
pub fn check_persist_execution_id_conflict(
    repo: &Path,
    persist_ref: &str,
    live_id: &str,
) -> Result<(), Vec<String>> {
    let dir = repo.join(persist_ref);
    if !dir.is_dir() {
        return Ok(());
    }
    let mut conflicts = Vec::new();
    let entries = fs::read_dir(&dir).map_err(|e| vec![e.to_string()])?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "_agent_handoff.md" || !name.ends_with(".md") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        let fm = parse_frontmatter_from_str(&text).unwrap_or_default();
        if let Some(existing) = fm.get("execution_id").and_then(|v| v.as_str()) {
            let existing = existing.trim();
            if !existing.is_empty() && existing != live_id {
                conflicts.push(
                    path.strip_prefix(repo)
                        .map(|p| p.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|_| path.display().to_string()),
                );
            }
        }
    }
    if conflicts.is_empty() {
        Ok(())
    } else {
        Err(conflicts)
    }
}

fn agent_names(delegates: &[Value]) -> Vec<String> {
    delegates
        .iter()
        .filter_map(|d| d.as_str())
        .filter_map(|s| s.strip_prefix("agent:"))
        .map(str::to_string)
        .collect()
}

fn truthy_state_flag(state: &Value, key: &str) -> bool {
    match state.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => matches!(s.to_lowercase().as_str(), "true" | "1" | "yes" | "on"),
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        _ => false,
    }
}

/// Copia flags nativos PPR (#125) al payload del CLI agente (L-STATE-FWD / PPR #136 residual).
fn inject_runtime_evidence_from_state(payload: &mut Value, state: &Value) {
    let Some(obj) = payload.as_object_mut() else {
        return;
    };

    let git = truthy_state_flag(state, "git_manager_invoked");
    let formal = truthy_state_flag(state, "formal_execute_process")
        || truthy_state_flag(state, "tech_triage_formal");

    if git {
        obj.insert("git_manager_invoked".into(), json!(true));
    }
    if formal {
        obj.insert("formal_execute_process".into(), json!(true));
        obj.insert("tech_triage_formal".into(), json!(true));
    }
    if let Some(checks) = state.get("tech_checks") {
        obj.insert("tech_checks".into(), checks.clone());
    }

    let mut runtime_evidence = json!({
        "git_manager_invoked": git,
        "formal_execute_process": formal,
        "tech_triage_formal": truthy_state_flag(state, "tech_triage_formal"),
    });
    if let Some(checks) = state.get("tech_checks") {
        if let Some(re) = runtime_evidence.as_object_mut() {
            re.insert("tech_checks".into(), checks.clone());
        }
    }
    if git || formal || state.get("tech_checks").is_some() {
        obj.insert("runtime_evidence".into(), runtime_evidence);
    }
}

/// Ola 1: fricción de agente en fases no-aduana (no F2/F4/F5) no colapsa PPR.
fn finish_agent_entry(mut entry: Value, process_name: &str, phase_name: &str) -> Value {
    if process_name == "pull-request-review"
        && matches!(phase_name, "Triaje documental" | "Cosecha Kaizen")
        && entry.get("status").and_then(|v| v.as_str()) == Some("failed")
    {
        entry["fail_soft"] = json!(true);
    }
    entry
}

fn build_agent_command(
    bin: &str,
    args: &[String],
    repo: &Path,
    depth: u32,
) -> Command {
    let mut cmd = Command::new(bin);
    cmd.args(args)
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env(ENV_DEPTH, (depth + 1).to_string());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                if libc::setpgid(0, 0) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
    }

    cmd
}

#[cfg(unix)]
fn kill_process_group(pid: u32) {
    let pgid = pid as i32;
    unsafe {
        libc::kill(-pgid, libc::SIGTERM);
    }
    thread::sleep(Duration::from_millis(KILL_GRACE_MS));
    unsafe {
        libc::kill(-pgid, libc::SIGKILL);
    }
}

#[cfg(not(unix))]
fn kill_process_group(_pid: u32) {
    // Best-effort en hosts no-Unix: el drop de Child intentará wait.
}

use std::sync::mpsc;

fn wait_child_with_timeout(
    child: std::process::Child,
    timeout: Duration,
) -> Result<Output, &'static str> {
    let pid = child.id();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(child.wait_with_output());
    });
    match rx.recv_timeout(timeout) {
        Ok(Ok(out)) => Ok(out),
        Ok(Err(_)) => Err("wait agent-runtime"),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            kill_process_group(pid);
            Err("agent-runtime-timeout")
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => Err("wait agent-runtime"),
    }
}

/// Invoca el CLI de runtime para una fase de agentes.
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

    if lab_relay_active() {
        eprintln!("agent-runtime: lab-relay activo (bóveda ignorada)");
        entry["status"] = json!("simulated");
        entry["note"] = json!("SDDIA_AGENT_RELAY_IDE=1; relevo IDE");
        return finish_agent_entry(entry, process_name, phase_name);
    }

    if agent_runtime_depth() >= 1 {
        entry["status"] = json!("simulated");
        entry["note"] = json!("reentry-guard: SDDIA_AGENT_RUNTIME_DEPTH>=1");
        return finish_agent_entry(entry, process_name, phase_name);
    }

    let raw = match std::env::var(ENV_CMD) {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            entry["status"] = json!("simulated");
            entry["note"] = json!("agentes IDE; sin SDDIA_AGENT_RUNTIME_COMMAND");
            return finish_agent_entry(entry, process_name, phase_name);
        }
    };

    let parts = match split_command(&raw) {
        Ok(p) => p,
        Err(e) => {
            entry["status"] = json!("failed");
            entry["error"] = json!(e);
            return finish_agent_entry(entry, process_name, phase_name);
        }
    };
    let (bin, args) = match parts.split_first() {
        Some((b, a)) => (b.clone(), a.to_vec()),
        None => {
            entry["status"] = json!("failed");
            entry["error"] = json!("comando runtime vacío");
            return finish_agent_entry(entry, process_name, phase_name);
        }
    };

    let execution_id = resolve_execution_id(inputs, state);
    let persist_ref_val = resolve_persist_ref_value(inputs, state);
    if let (Some(live_id), Some(persist)) = (
        execution_id.as_deref(),
        persist_ref_val.as_str().filter(|s| !s.is_empty()),
    ) {
        if let Err(conflicts) = check_persist_execution_id_conflict(repo, persist, live_id) {
            entry["status"] = json!("failed");
            entry["error"] = json!("persist-execution-id-conflict");
            entry["conflict_paths"] = json!(conflicts);
            return finish_agent_entry(entry, process_name, phase_name);
        }
    }

    let agents = agent_names(delegates);
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
        "persist_ref": persist_ref_val,
        "branch_name": branch_name,
        "execution_id": execution_id,
        "correlation_id": inputs.get("correlation_id"),
        "pbi_ref": inputs.get("pbi_ref"),
        "inputs": inputs,
        "workspace_path": state.get("workspace_path")
            .or_else(|| state.get("workspace").and_then(|w| w.get("workspace_path"))),
        "repo_root": repo.display().to_string(),
    });
    inject_runtime_evidence_from_state(&mut payload, state);
    if let Some(di) = di_binding {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("di_binding".into(), di);
        }
    }

    let depth = agent_runtime_depth();
    let mut child = match build_agent_command(&bin, &args, repo, depth).spawn() {
        Ok(c) => c,
        Err(e) => {
            entry["status"] = json!("failed");
            entry["error"] = json!(format!("spawn agent-runtime: {e}"));
            return finish_agent_entry(entry, process_name, phase_name);
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        let body = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".into());
        if let Err(e) = stdin.write_all(body.as_bytes()) {
            entry["status"] = json!("failed");
            entry["error"] = json!(format!("stdin agent-runtime: {e}"));
            return finish_agent_entry(entry, process_name, phase_name);
        }
    }

    let timeout_secs = resolve_timeout_secs(phase_name);
    let out = match wait_child_with_timeout(child, Duration::from_secs(timeout_secs)) {
        Ok(o) => o,
        Err("agent-runtime-timeout") => {
            entry["status"] = json!("failed");
            entry["error"] = json!("agent-runtime-timeout");
            entry["timeout_secs"] = json!(timeout_secs);
            return finish_agent_entry(entry, process_name, phase_name);
        }
        Err(e) => {
            entry["status"] = json!("failed");
            entry["error"] = json!(e);
            return finish_agent_entry(entry, process_name, phase_name);
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
            format!(
                "agent-runtime sin stdout (exit {})",
                out.status.code().unwrap_or(1)
            )
        } else {
            stderr.trim().to_string()
        });
        return finish_agent_entry(entry, process_name, phase_name);
    }

    let body: Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(e) => {
            entry["status"] = json!("failed");
            entry["error"] = json!(format!("JSON agent-runtime: {e}"));
            entry["raw_stdout"] = json!(line);
            return finish_agent_entry(entry, process_name, phase_name);
        }
    };

    let success = body
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let data = body.get("data").cloned().unwrap_or(json!({}));
    let status = data
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or(if success { "executed" } else { "failed" });

    let normalized = match status {
        "executed" | "awaiting_agents" | "failed" | "blocked" => status,
        "awaiting" => "awaiting_agents",
        _ if success => "executed",
        _ => "failed",
    };

    entry["status"] = json!(normalized);
    if let Some(msg) = data.get("message").and_then(|v| v.as_str()) {
        entry["message"] = json!(msg);
    }
    if let Some(err) = body
        .get("error")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        entry["error"] = json!(err);
    }
    if !out.status.success() && normalized == "executed" {
        entry["status"] = json!("failed");
        entry["error"] = json!(format!(
            "agent-runtime exit {} pese a status executed",
            out.status.code().unwrap_or(1)
        ));
    }
    entry["runtime_response"] = body;
    finish_agent_entry(entry, process_name, phase_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::sync::Mutex;

    static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_TEST_LOCK.lock().unwrap()
    }

    fn clear_agent_env() {
        for key in [
            ENV_CMD,
            ENV_RELAY,
            ENV_DEPTH,
            ENV_TIMEOUT,
            ENV_TIMEOUT_EXEC,
        ] {
            std::env::remove_var(key);
        }
    }

    fn mock_script(dir: &Path, body: &str) -> PathBuf {
        let script = dir.join("mock-agent.sh");
        fs::write(&script, body).unwrap();
        let mut perms = fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script, perms).unwrap();
        script
    }

    #[test]
    fn not_configured_returns_simulated() {
        let _guard = env_lock();
        clear_agent_env();
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
    fn relay_flag_forces_simulated_even_with_command() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-relay-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\necho '{\"success\":true,\"data\":{\"status\":\"executed\"}}'\n",
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        std::env::set_var(ENV_RELAY, "1");
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
        std::env::remove_var(ENV_RELAY);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "simulated");
        assert!(entry["note"]
            .as_str()
            .unwrap_or("")
            .contains("SDDIA_AGENT_RELAY_IDE"));
    }

    #[test]
    fn reentry_guard_skips_spawn() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-depth-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\necho '{\"success\":true,\"data\":{\"status\":\"executed\"}}'\n",
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        std::env::set_var(ENV_DEPTH, "1");
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
        std::env::remove_var(ENV_DEPTH);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "simulated");
        assert!(entry["note"].as_str().unwrap_or("").contains("reentry-guard"));
    }

    #[test]
    fn timeout_kills_hanging_command() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-to-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(&dir, "#!/bin/sh\nsleep 30\n");
        std::env::set_var(ENV_CMD, script.display().to_string());
        std::env::set_var(ENV_TIMEOUT, "1");
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Estabilización de Requisitos",
            &[json!("agent:mayeuta")],
            &json!({}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        std::env::remove_var(ENV_TIMEOUT);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "failed");
        assert_eq!(entry["error"], "agent-runtime-timeout");
    }

    #[test]
    fn configured_cli_marks_executed() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"executed\",\"message\":\"ok\"}}'\n",
        );

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
    fn execution_id_conflict_detected() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-conf-{}", uuid::Uuid::new_v4()));
        let persist = dir.join("docs/features/x");
        fs::create_dir_all(&persist).unwrap();
        fs::write(
            persist.join("plan.md"),
            "---\nexecution_id: deadbeef-dead-beef-dead-beefdeadbeef\n---\n",
        )
        .unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\necho '{\"success\":true,\"data\":{\"status\":\"executed\"}}'\n",
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let live = "80a3ca0d-80c5-4662-ab12-2afe757478c8";
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Diseño de Blueprint",
            &[json!("agent:dedalo")],
            &json!({"persist_ref": "docs/features/x", "execution_id": live}),
            &json!({"execution_id": live}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "failed");
        assert_eq!(entry["error"], "persist-execution-id-conflict");
    }

    #[test]
    fn configured_cli_can_await() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-aw-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"awaiting_agents\"}}'\n",
        );

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
    fn configured_cli_can_block() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-bl-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"blocked\"}}'\n",
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "bug-fix",
            "Diseño del fix",
            &[json!("agent:dedalo")],
            &json!({"persist_ref": "docs/fixes/x"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "blocked", "{entry}");
    }

    #[test]
    fn branch_name_coalesces_from_pr_branch() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-br-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c 'import json,sys; doc=json.load(sys.stdin); assert doc.get("branch_name")=="feat/from-pr", doc; print(json.dumps({"success":True,"data":{"status":"executed","message":"branch-ok"}}))'
"#,
        );

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

    #[test]
    fn payload_includes_execution_id() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-eid-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c 'import json,sys; doc=json.load(sys.stdin); assert doc.get("execution_id")=="live-id", doc; print(json.dumps({"success":True,"data":{"status":"executed"}}))'
"#,
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Ejecución",
            &[json!("agent:tekton")],
            &json!({"execution_id": "live-id"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "executed", "{entry}");
    }

    #[test]
    fn runtime_evidence_forwards_native_state_flags() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-ev-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c '
import json,sys
doc=json.load(sys.stdin)
assert doc.get("git_manager_invoked") is True, doc
assert doc.get("formal_execute_process") is True, doc
assert doc.get("tech_triage_formal") is True, doc
re=doc.get("runtime_evidence") or {}
assert re.get("git_manager_invoked") is True, re
assert re.get("formal_execute_process") is True, re
assert (re.get("tech_checks") or {}).get("TECH_FORMAL_EXECUTE_PROCESS")=="APTO", re
print(json.dumps({"success":True,"data":{"status":"executed","message":"evidence-fwd-ok"}}))
'
"#,
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Verificación",
            &[json!("agent:argos")],
            &json!({"persist_ref": "docs/features/x", "branch_name": "feat/x"}),
            &json!({
                "git_manager_invoked": true,
                "tech_triage_formal": true,
                "tech_checks": {"TECH_FORMAL_EXECUTE_PROCESS": "APTO"}
            }),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "executed", "{entry}");
        assert_eq!(entry["message"], "evidence-fwd-ok");
    }

    #[test]
    fn persist_ref_falls_back_from_workspace_state() {
        let from_ws = resolve_persist_ref_value(
            &json!({"persist_ref": ""}),
            &json!({"workspace": {"persist_ref": "docs/features/from-ws"}}),
        );
        assert_eq!(from_ws, json!("docs/features/from-ws"));
        let from_inputs = resolve_persist_ref_value(
            &json!({"persist_ref": "docs/features/top"}),
            &json!({"workspace": {"persist_ref": "docs/features/from-ws"}}),
        );
        assert_eq!(from_inputs, json!("docs/features/top"));
        let missing = resolve_persist_ref_value(&json!({}), &json!({}));
        assert_eq!(missing, Value::Null);
    }

    #[test]
    fn ppr_doc_triage_agent_failed_is_fail_soft() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-fs-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c 'import json; print(json.dumps({"success":False,"data":{"status":"failed","message":"timeout"}}))'
"#,
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let soft = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Triaje documental",
            &[json!("agent:argos")],
            &json!({"pr_branch": "feat/x"}),
            &json!({}),
            None,
        );
        let hard = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Veredicto y bloqueo",
            &[json!("agent:argos")],
            &json!({"pr_branch": "feat/x"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(soft["status"], "failed");
        assert_eq!(soft["fail_soft"], true);
        assert_eq!(hard["status"], "failed");
        assert!(hard.get("fail_soft").is_none());
    }
}
