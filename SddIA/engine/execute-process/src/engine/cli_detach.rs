//! Fire-and-forget del CLI operador: PEC temprano + spawn huérfano (L-CLI-DETACH-ALLOWLIST).

use super::fractal::{load_fractal_dirs, write_fractal_event};
use super::workspace::{load_paths_config, resolve_workspaces_root};
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use uuid::Uuid;

const DEFAULT_ALLOWLIST: &[&str] = &["pull-request-review"];

pub fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

#[derive(Debug, Clone, PartialEq)]
pub struct DetachPolicy {
    pub foreground: bool,
    pub force_detach: bool,
    pub allowlist: Vec<String>,
}

impl DetachPolicy {
    pub fn from_env() -> Self {
        let mut allowlist: Vec<String> = DEFAULT_ALLOWLIST.iter().map(|s| (*s).to_string()).collect();
        if let Ok(extra) = std::env::var("SDDIA_CLI_DETACH_PROCESSES") {
            for p in extra.split(',') {
                let t = p.trim();
                if !t.is_empty() && !allowlist.iter().any(|x| x == t) {
                    allowlist.push(t.to_string());
                }
            }
        }
        Self {
            foreground: env_truthy("SDDIA_CLI_FOREGROUND"),
            force_detach: env_truthy("SDDIA_CLI_DETACH"),
            allowlist,
        }
    }

    pub fn should_detach(&self, process_name: &str) -> bool {
        if self.foreground {
            return false;
        }
        if self.force_detach {
            return true;
        }
        self.allowlist.iter().any(|p| p == process_name)
    }
}

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn emit_detach_pec(
    repo: &Path,
    process_name: &str,
    execution_id: &str,
    correlation_id: &str,
) -> Result<Value, String> {
    let (_, orch_dir, _, _) = load_fractal_dirs(repo);
    fs::create_dir_all(repo.join(&orch_dir)).ok();
    let orch_id = Uuid::new_v4().to_string();
    let payload = json!({
        "process_name": process_name,
        "status": "success",
        "execution_id": execution_id,
        "correlation_id": correlation_id,
        "cycle_phase": "awaiting_agents",
        "detach": true,
        "emitter_hint": "cli-detach-ack",
    });
    let orch_event = json!({
        "event_id": orch_id,
        "event_type": "Process_Execution_Completed",
        "event_family": "orchestration",
        "timestamp": iso_now(),
        "emitter_agent": "execute-process",
        "payload": payload,
        "delivery_state": {},
    });
    write_fractal_event(repo, &orch_event, &orch_dir)
}

pub fn spawn_orphan(cmd: &mut Command) -> Result<u32, String> {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let child = cmd.spawn().map_err(|e| format!("spawn detach: {e}"))?;
    Ok(child.id())
}

fn child_log_path(repo: &Path, process_name: &str, execution_id: &str) -> Result<PathBuf, String> {
    let cfg = load_paths_config(repo).unwrap_or(json!({}));
    let ws = resolve_workspaces_root(repo, &cfg)
        .join(process_name)
        .join(execution_id);
    fs::create_dir_all(&ws).map_err(|e| e.to_string())?;
    Ok(ws.join("detached.log"))
}

fn child_args() -> Vec<String> {
    std::env::args()
        .skip(1)
        .filter(|a| a != "--detach")
        .collect()
}

/// Si aplica allowlist/flag: deposita PEC, spawnea hijo foreground y devuelve acuse.
pub fn maybe_detach(
    repo: &Path,
    process_name: &str,
    process_inputs: &Value,
) -> Result<Option<OrchestratorEnvelope>, String> {
    if !DetachPolicy::from_env().should_detach(process_name) {
        return Ok(None);
    }
    let execution_id = process_inputs
        .get("execution_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let correlation_id = process_inputs
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| execution_id.clone());

    let seal = emit_detach_pec(repo, process_name, &execution_id, &correlation_id)?;
    let event_id = seal
        .get("event_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let log_path = child_log_path(repo, process_name, &execution_id)?;
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| format!("detached.log: {e}"))?;
    let log_err = log_file.try_clone().map_err(|e| format!("detached.log dup: {e}"))?;

    let self_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut cmd = Command::new(&self_exe);
    cmd.args(child_args())
        .current_dir(repo)
        .env("SDDIA_CLI_FOREGROUND", "1")
        .env("SDDIA_CLI_DETACH", "0")
        .env("SDDIA_DETACHED_EXECUTION_ID", &execution_id)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(log_err));

    let pid = spawn_orphan(&mut cmd)?;
    let log_rel = log_path
        .strip_prefix(repo)
        .unwrap_or(&log_path)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(Some(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "status": "accepted",
            "detached": true,
            "process_name": process_name,
            "execution_id": execution_id,
            "correlation_id": correlation_id,
            "event_id": event_id,
            "pid": pid,
            "log_path": log_rel,
            "orchestration": seal,
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": process_name,
            "phases": [{
                "phase_name": "CLI detach",
                "status": "accepted",
                "handler": "cli-detach",
                "detached": true,
            }],
        })),
        exit_code: 0,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn foreground_wins_over_force_and_allowlist() {
        let p = DetachPolicy {
            foreground: true,
            force_detach: true,
            allowlist: vec!["pull-request-review".into()],
        };
        assert!(!p.should_detach("pull-request-review"));
    }

    #[test]
    fn allowlist_detaches_ppr_not_radamanto() {
        let p = DetachPolicy {
            foreground: false,
            force_detach: false,
            allowlist: vec!["pull-request-review".into()],
        };
        assert!(p.should_detach("pull-request-review"));
        assert!(!p.should_detach("radamanto-batch"));
        assert!(!p.should_detach("feature"));
    }

    #[test]
    fn force_detach_any_process() {
        let p = DetachPolicy {
            foreground: false,
            force_detach: true,
            allowlist: vec![],
        };
        assert!(p.should_detach("feature"));
    }

    #[test]
    fn emit_detach_pec_writes_orchestration_not_instance_customization() {
        let tmp = tempfile::tempdir().expect("tmp");
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_fractal":{"telemetry":"./.events/telemetry","orchestration":"./.events/orchestration","domain":"./.events/domain"},"eda_instance":{"customization":".SddIA/events"}}"#,
        )
        .unwrap();
        let seal = emit_detach_pec(repo, "pull-request-review", "exec-1", "corr-1").expect("pec");
        let path = seal.get("target_path").and_then(|v| v.as_str()).unwrap();
        assert!(path.starts_with(".events/orchestration/"), "{path}");
        assert!(!path.contains(".SddIA/events"));
        let raw = std::fs::read_to_string(repo.join(path)).unwrap();
        let ev: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(ev["event_type"], "Process_Execution_Completed");
        assert_eq!(ev["payload"]["cycle_phase"], "awaiting_agents");
        assert_eq!(ev["payload"]["detach"], true);
        assert_eq!(ev["emitter_agent"], "execute-process");
    }

    #[test]
    fn spawn_orphan_returns_before_child_sleep() {
        let mut cmd = Command::new("sleep");
        cmd.arg("2")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let t0 = Instant::now();
        let pid = spawn_orphan(&mut cmd).expect("spawn");
        let elapsed = t0.elapsed().as_millis();
        assert!(pid > 0);
        assert!(
            elapsed < 500,
            "padre no debe join al hijo: {elapsed} ms"
        );
        let _ = Command::new("kill").arg(pid.to_string()).status();
    }
}
