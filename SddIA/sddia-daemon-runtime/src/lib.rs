pub mod eda_bus;
pub mod eda_sweep;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BusTopology {
    pub pending: PathBuf,
    pub dead_letter_subscribers: PathBuf,
    pub telemetry: PathBuf,
    pub orchestration: PathBuf,
    pub domain: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DaemonLock {
    daemon_name: String,
    pid: u32,
    started_at: String,
    heartbeat_interval_seconds: u64,
}

pub struct DaemonRuntime {
    repo: PathBuf,
    daemon_name: String,
    daemon_uuid: String,
    heartbeat_interval: Duration,
    lock_path: PathBuf,
    started_at: Instant,
    started_iso: String,
    last_emit: Instant,
    last_stimulus_at: Option<String>,
    bootstrapped: bool,
}

pub fn find_repo_root() -> Result<PathBuf, String> {
    let mut dir = std::env::current_dir().map_err(|e| format!("current_dir: {e}"))?;
    loop {
        if dir.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    Err("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)".into())
}

pub fn load_cumulo(repo: &Path) -> Result<Value, String> {
    let path = repo.join("SddIA/core/cumulo.paths.json");
    let raw = fs::read_to_string(&path).map_err(|e| format!("cumulo: {e}"))?;
    serde_json::from_str(&raw).map_err(|e| format!("cumulo JSON: {e}"))
}

fn rel_path(repo: &Path, rel: &str) -> PathBuf {
    let trimmed = rel.trim().trim_start_matches("./");
    repo.join(trimmed)
}

pub fn status_dir(repo: &Path) -> PathBuf {
    load_cumulo(repo)
        .ok()
        .and_then(|cfg| {
            cfg.get("daemons_instance")
                .and_then(|d| d.get("status"))
                .and_then(|v| v.as_str())
                .map(|s| rel_path(repo, s))
        })
        .unwrap_or_else(|| repo.join(".SddIA/daemons/status"))
}

pub fn load_bus_topology(repo: &Path) -> BusTopology {
    let defaults = |repo: &Path| BusTopology {
        pending: repo.join(".events/pending"),
        dead_letter_subscribers: repo.join(".events/dead-letter/subscribers"),
        telemetry: repo.join(".events/telemetry"),
        orchestration: repo.join(".events/orchestration"),
        domain: repo.join(".events/domain"),
    };
    let Ok(cfg) = load_cumulo(repo) else {
        return defaults(repo);
    };
    let bus = cfg.get("eda_bus").and_then(|v| v.as_object());
    let fractal = cfg.get("eda_fractal").and_then(|v| v.as_object());
    let mut top = defaults(repo);
    if let Some(bus) = bus {
        if let Some(p) = bus.get("pending").and_then(|v| v.as_str()) {
            top.pending = rel_path(repo, p);
        }
        if let Some(dl) = bus.get("dead_letter").and_then(|v| v.as_str()) {
            top.dead_letter_subscribers = rel_path(repo, &format!("{}/subscribers", dl.trim_end_matches('/')));
        }
    }
    if let Some(fractal) = fractal {
        for (key, dest) in [
            ("telemetry", &mut top.telemetry),
            ("orchestration", &mut top.orchestration),
            ("domain", &mut top.domain),
        ] {
            if let Some(p) = fractal.get(key).and_then(|v| v.as_str()) {
                *dest = rel_path(repo, p);
            }
        }
    }
    top
}

pub fn ensure_fractal_dirs(top: &BusTopology) -> Result<(), String> {
    for dir in [&top.telemetry, &top.orchestration, &top.domain] {
        fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    }
    Ok(())
}

pub fn ensure_bus_dirs(top: &BusTopology) -> Result<(), String> {
    fs::create_dir_all(&top.pending).map_err(|e| format!("mkdir pending: {e}"))?;
    fs::create_dir_all(&top.dead_letter_subscribers)
        .map_err(|e| format!("mkdir dead_letter_subscribers: {e}"))?;
    Ok(())
}

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn write_json_atomic(path: &Path, value: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir: {e}"))?;
    }
    let tmp = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| format!("json: {e}"))?;
    {
        let mut file = fs::File::create(&tmp).map_err(|e| format!("create tmp: {e}"))?;
        file.write_all(&bytes).map_err(|e| format!("write tmp: {e}"))?;
        file.write_all(b"\n").map_err(|e| format!("write nl: {e}"))?;
    }
    fs::rename(&tmp, path).map_err(|e| format!("rename: {e}"))
}

pub fn write_fractal_event(_repo: &Path, top: &BusTopology, event: &Value, family: &str) -> Result<(), String> {
    let dir = match family {
        "telemetry" => &top.telemetry,
        "orchestration" => &top.orchestration,
        "domain" => &top.domain,
        other => return Err(format!("familia inválida: {other}")),
    };
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| "event_id requerido".to_string())?;
    let target = dir.join(format!("{event_id}.json"));
    write_json_atomic(&target, event)
}

pub fn pid_alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    #[cfg(not(target_family = "wasm"))]
    {
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = pid;
        false
    }
}

fn parse_daemon_spec(repo: &Path, daemon_name: &str) -> (String, u64) {
    let md = repo.join("SddIA/daemons").join(format!("{daemon_name}.md"));
    let Ok(raw) = fs::read_to_string(&md) else {
        return (String::new(), 30);
    };
    let mut uuid = String::new();
    let mut interval: u64 = 30;
    let mut in_execution = false;
    for line in raw.lines() {
        let t = line.trim();
        if t == "execution:" {
            in_execution = true;
            continue;
        }
        if in_execution && t.starts_with("heartbeat_interval_seconds:") {
            if let Some(n) = t.split(':').nth(1).and_then(|s| s.trim().parse::<u64>().ok()) {
                interval = n.max(5);
            }
        }
        if in_execution && !t.starts_with(' ') && !t.is_empty() && t.ends_with(':') && t != "execution:" {
            in_execution = false;
        }
        if t.starts_with("uuid:") {
            uuid = t
                .split(':')
                .nth(1)
                .map(|s| s.trim().trim_matches('"'))
                .unwrap_or("")
                .to_string();
        }
    }
    (uuid, interval)
}

impl DaemonRuntime {
    pub fn new(repo: PathBuf, daemon_name: &str) -> Self {
        let (daemon_uuid, interval_secs) = parse_daemon_spec(&repo, daemon_name);
        Self {
            repo,
            daemon_name: daemon_name.to_string(),
            daemon_uuid,
            heartbeat_interval: Duration::from_secs(interval_secs),
            lock_path: PathBuf::new(),
            started_at: Instant::now(),
            started_iso: iso_now(),
            last_emit: Instant::now() - Duration::from_secs(3600),
            last_stimulus_at: None,
            bootstrapped: false,
        }
    }

    pub fn bootstrap(&mut self, top: &BusTopology) -> Result<(), String> {
        if self.bootstrapped {
            return Ok(());
        }
        self.lock_path = status_dir(&self.repo).join(format!("{}.lock", self.daemon_name));
        ensure_fractal_dirs(top)?;
        if let Some(existing) = self.read_lock()? {
            let other = existing.pid;
            if other != std::process::id() && pid_alive(other) {
                return Err(format!(
                    "[{}] lock activo pid={other}; abortando duplicado",
                    self.daemon_name
                ));
            }
        }
        self.write_lock()?;
        self.emit_heartbeat(top, true)?;
        self.bootstrapped = true;
        Ok(())
    }

    pub fn note_stimulus(&mut self) {
        self.last_stimulus_at = Some(iso_now());
    }

    pub fn tick(&mut self, top: &BusTopology) -> Result<(), String> {
        if !self.bootstrapped {
            self.bootstrap(top)?;
        }
        self.emit_heartbeat(top, false)?;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        if self.lock_path.is_file() {
            let _ = fs::remove_file(&self.lock_path);
        }
    }

    fn read_lock(&self) -> Result<Option<DaemonLock>, String> {
        if !self.lock_path.is_file() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&self.lock_path).map_err(|e| format!("read lock: {e}"))?;
        let lock: DaemonLock = serde_json::from_str(&raw).map_err(|e| format!("lock JSON: {e}"))?;
        Ok(Some(lock))
    }

    fn write_lock(&self) -> Result<(), String> {
        let status = status_dir(&self.repo);
        fs::create_dir_all(&status).map_err(|e| format!("mkdir status: {e}"))?;
        let lock = DaemonLock {
            daemon_name: self.daemon_name.clone(),
            pid: std::process::id(),
            started_at: self.started_iso.clone(),
            heartbeat_interval_seconds: self.heartbeat_interval.as_secs(),
        };
        let bytes = serde_json::to_string_pretty(&lock).map_err(|e| format!("lock json: {e}"))?;
        fs::write(&self.lock_path, format!("{bytes}\n")).map_err(|e| format!("write lock: {e}"))
    }

    fn emit_heartbeat(&mut self, top: &BusTopology, force: bool) -> Result<(), String> {
        if !force && self.last_emit.elapsed() < self.heartbeat_interval {
            return Ok(());
        }
        let mut payload = json!({
            "daemon_name": self.daemon_name,
            "daemon_uuid": self.daemon_uuid,
            "pid": std::process::id(),
            "uptime_seconds": self.started_at.elapsed().as_secs(),
            "status": "alive",
        });
        if let Some(stimulus) = &self.last_stimulus_at {
            payload["last_stimulus_at"] = json!(stimulus);
        }
        let event = json!({
            "event_id": Uuid::new_v4().to_string(),
            "event_type": "Daemon_Heartbeat",
            "timestamp": iso_now(),
            "emitter_agent": self.daemon_name,
            "payload": payload,
        });
        write_fractal_event(&self.repo, top, &event, "telemetry")?;
        self.last_emit = Instant::now();
        Ok(())
    }
}

impl Drop for DaemonRuntime {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn lock_excludes_duplicate_pid() {
        let base = std::env::temp_dir().join(format!("sddia-daemon-test-{}", Uuid::new_v4()));
        let repo = base.clone();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"daemons_instance":{"status":".SddIA/daemons/status"},"eda_fractal":{"telemetry":".events/telemetry"}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/daemons")).unwrap();
        fs::write(
            repo.join("SddIA/daemons/test-daemon.md"),
            "uuid: \"00000000-0000-4000-8000-000000000001\"\nexecution:\n  heartbeat_interval_seconds: 5\n",
        )
        .unwrap();
        let top = load_bus_topology(&repo);
        let mut rt = DaemonRuntime::new(repo.clone(), "test-daemon");
        rt.bootstrap(&top).unwrap();
        assert!(rt.lock_path.is_file());
        rt.shutdown();
        thread::sleep(StdDuration::from_millis(20));
        assert!(!rt.lock_path.is_file());
        let _ = fs::remove_dir_all(base);
    }
}
