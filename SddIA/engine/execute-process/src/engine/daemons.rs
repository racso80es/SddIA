//! Utilidades compartidas Centinelas (locks, PIDs, índice, bus EDA).

use crate::core::parser::parse_frontmatter;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const SKIP_DAEMONS: &[&str] = &["index", "daemons-contract"];

pub fn load_cumulo(repo: &Path) -> Result<Value, String> {
    let path = repo.join("SddIA/core/cumulo.paths.json");
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

pub fn daemons_dir(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_cumulo(repo)?;
    let rel = cfg
        .get("directories")
        .and_then(|d| d.get("daemons"))
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/daemons");
    Ok(repo.join(rel.trim().trim_start_matches("./")))
}

pub fn status_dir(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_cumulo(repo)?;
    let rel = cfg
        .get("daemons_instance")
        .and_then(|d| d.get("status"))
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/daemons/status");
    Ok(repo.join(rel.trim().trim_start_matches("./")))
}

pub fn state_dir(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_cumulo(repo)?;
    let rel = cfg
        .get("daemons_instance")
        .and_then(|d| d.get("state"))
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/daemons/state");
    Ok(repo.join(rel.trim().trim_start_matches("./")))
}

pub fn lock_path(repo: &Path, daemon_id: &str) -> Result<PathBuf, String> {
    Ok(status_dir(repo)?.join(format!("{daemon_id}.lock")))
}

pub fn list_indexed_daemon_ids(repo: &Path) -> Result<Vec<String>, String> {
    let dir = daemons_dir(repo)?;
    let mut ids = Vec::new();
    if !dir.is_dir() {
        return Ok(ids);
    }
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if SKIP_DAEMONS.contains(&stem) {
            continue;
        }
        ids.push(stem.to_string());
    }
    ids.sort();
    Ok(ids)
}

pub fn pid_alive(pid: i32) -> bool {
    if pid <= 0 {
        return false;
    }
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn read_lock(repo: &Path, daemon_id: &str) -> Option<Value> {
    let path = lock_path(repo, daemon_id).ok()?;
    if !path.is_file() {
        return None;
    }
    let text = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn write_lock(
    repo: &Path,
    daemon_id: &str,
    pid: i32,
    heartbeat_interval_seconds: i32,
) -> Result<PathBuf, String> {
    let status = status_dir(repo)?;
    fs::create_dir_all(&status).map_err(|e| e.to_string())?;
    let path = lock_path(repo, daemon_id)?;
    let payload = json!({
        "daemon_name": daemon_id,
        "pid": pid,
        "started_at": iso_now(),
        "heartbeat_interval_seconds": heartbeat_interval_seconds,
    });
    write_json_atomic(&path, &payload)?;
    Ok(path)
}

pub fn remove_lock(repo: &Path, daemon_id: &str) -> bool {
    if let Ok(path) = lock_path(repo, daemon_id) {
        if path.is_file() {
            let _ = fs::remove_file(&path);
            return true;
        }
    }
    false
}

pub fn write_json_atomic(path: &Path, body: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(body).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| e.to_string())
}

pub fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn parse_iso(value: &str) -> Option<DateTime<Utc>> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    DateTime::parse_from_rfc3339(&trimmed.replace('Z', "+00:00"))
        .ok()
        .map(|d| d.with_timezone(&Utc))
}

pub fn daemon_interval(repo: &Path, daemon_id: &str) -> i32 {
    let md = daemons_dir(repo)
        .ok()
        .map(|d| d.join(format!("{daemon_id}.md")));
    let Some(md) = md.filter(|p| p.is_file()) else {
        return 30;
    };
    let Ok(fm) = parse_frontmatter(&md) else {
        return 30;
    };
    fm.get("execution")
        .and_then(|e| e.get("heartbeat_interval_seconds"))
        .and_then(|v| v.as_i64())
        .map(|n| n.max(5) as i32)
        .unwrap_or(30)
}

pub fn resolve_daemon_uuid(repo: &Path, daemon_id: &str) -> String {
    let md = daemons_dir(repo)
        .ok()
        .map(|d| d.join(format!("{daemon_id}.md")));
    let Some(md) = md.filter(|p| p.is_file()) else {
        return String::new();
    };
    parse_frontmatter(&md)
        .ok()
        .and_then(|fm| fm.get("uuid").and_then(|v| v.as_str()).map(str::to_string))
        .unwrap_or_default()
}

pub struct DaemonSpec {
    pub daemon_id: String,
    pub daemon_uuid: String,
    pub entrypoint: String,
    pub runtime: String,
    pub heartbeat_interval_seconds: i32,
}

pub fn resolve_daemon(repo: &Path, daemon_id: &str) -> Result<DaemonSpec, String> {
    let md = daemons_dir(repo)?.join(format!("{daemon_id}.md"));
    if !md.is_file() {
        return Err(format!("Centinela no definido: {daemon_id}"));
    }
    let fm = parse_frontmatter(&md)?;
    let execution = fm.get("execution").cloned().unwrap_or(YamlValue::Null);
    let entrypoint = execution
        .get("entrypoint")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("execution.entrypoint inválido en {daemon_id}"))?
        .to_string();
    let runtime = execution
        .get("runtime")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("execution.runtime inválido en {daemon_id}"))?
        .to_string();
    let heartbeat = execution
        .get("heartbeat_interval_seconds")
        .and_then(|v| v.as_i64())
        .map(|n| n.max(5) as i32)
        .unwrap_or(30);
    Ok(DaemonSpec {
        daemon_id: daemon_id.to_string(),
        daemon_uuid: fm
            .get("uuid")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        entrypoint,
        runtime,
        heartbeat_interval_seconds: heartbeat,
    })
}

pub fn resolve_daemon_binary(repo: &Path, daemon_id: &str) -> Option<PathBuf> {
    let base = repo.join("SddIA/target");
    for profile in ["release", "debug"] {
        let p = base.join(profile).join(daemon_id);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

pub fn kill_pid(pid: i32, grace_seconds: i32) -> Value {
    let mut signals: Vec<&str> = Vec::new();
    if !pid_alive(pid) {
        return json!({"pid": pid, "alive": false, "signal_sequence": signals, "exit_code": null});
    }
    let _ = Command::new("kill")
        .args(["-TERM", &pid.to_string()])
        .status();
    signals.push("SIGTERM");
    let deadline = Instant::now() + Duration::from_secs(grace_seconds.max(1) as u64);
    while Instant::now() < deadline {
        if !pid_alive(pid) {
            return json!({"pid": pid, "alive": false, "signal_sequence": signals, "exit_code": 0});
        }
        thread::sleep(Duration::from_millis(200));
    }
    if pid_alive(pid) {
        let _ = Command::new("kill")
            .args(["-KILL", &pid.to_string()])
            .status();
        signals.push("SIGKILL");
        thread::sleep(Duration::from_millis(300));
    }
    json!({
        "pid": pid,
        "alive": pid_alive(pid),
        "signal_sequence": signals,
        "exit_code": if pid_alive(pid) { Value::Null } else { json!(0) },
    })
}

pub fn load_eda_pending(repo: &Path) -> Result<String, String> {
    if let Ok(bus) = std::env::var("EVENT_BUS_PATH") {
        if !bus.trim().is_empty() {
            return Ok(format!("{}/pending", bus.trim().trim_end_matches('/')));
        }
    }
    let cfg = load_cumulo(repo)?;
    if let Some(bus) = cfg.get("event_bus").and_then(|v| v.as_str()) {
        if !bus.trim().is_empty() {
            return Ok(format!("{}/pending", bus.trim().trim_end_matches('/')));
        }
    }
    if let Some(pending) = cfg
        .get("eda_bus")
        .and_then(|b| b.get("pending"))
        .and_then(|v| v.as_str())
    {
        if !pending.trim().is_empty() {
            return Ok(pending.trim().trim_start_matches("./").replace('\\', "/"));
        }
    }
    Ok(".events/pending".to_string())
}

pub fn build_process_execution_completed_event(
    event_id: &str,
    asset_id: &str,
    process_name: &str,
    status: &str,
    workspace_path: Option<&str>,
) -> Value {
    let mut payload = json!({
        "asset_id": asset_id,
        "process_name": process_name,
        "status": status,
    });
    if let Some(ws) = workspace_path.filter(|s| !s.trim().is_empty()) {
        payload["workspace_path"] = json!(ws);
    }
    json!({
        "event_id": event_id,
        "event_type": "Process_Execution_Completed",
        "event_family": "orchestration",
        "timestamp": iso_now(),
        "emitter_agent": "execute-process",
        "payload": payload,
        "delivery_state": {},
    })
}

pub fn stamp_delivery_state(event_path: &Path, subscriber_key: &str, status: &str) {
    let Ok(text) = fs::read_to_string(event_path) else {
        return;
    };
    let Ok(mut body) = serde_json::from_str::<Value>(&text) else {
        return;
    };
    let ds = body
        .get("delivery_state")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    let mut new_ds = ds;
    new_ds.insert(subscriber_key.to_string(), json!(status));
    if let Some(obj) = body.as_object_mut() {
        obj.insert("delivery_state".into(), Value::Object(new_ds));
    }
    let _ = write_json_atomic(event_path, &body);
}
