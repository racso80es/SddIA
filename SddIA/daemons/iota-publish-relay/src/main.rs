//! Centinela supervisor: lock + Daemon_Heartbeat + spawn/reap del hijo Node (`server.mjs`).
//! Ceguera lógica: no lee genoma ni invoca execute-process. Solo vitalidad + /health.

use sddia_daemon_runtime::{find_repo_root, load_bus_topology, DaemonRuntime};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const DAEMON_NAME: &str = "iota-publish-relay";
const TICK_SECS: u64 = 5;
const GRACE_SECS: u64 = 10;
const RESTART_BACKOFF_SECS: u64 = 2;
const HEALTH_TIMEOUT_MS: u64 = 1500;

struct SupervisorTickAction {
    heartbeat_status: &'static str,
    kill_child: bool,
}

fn in_grace(child_present: bool, elapsed: Option<Duration>, grace: Duration) -> bool {
    child_present && elapsed.map(|e| e < grace).unwrap_or(false)
}

fn decide_supervisor_tick(
    health_ok: bool,
    in_grace: bool,
    child_alive: bool,
) -> SupervisorTickAction {
    SupervisorTickAction {
        heartbeat_status: if health_ok || in_grace {
            "alive"
        } else {
            "degraded"
        },
        kill_child: !health_ok && !in_grace && child_alive,
    }
}

fn resolve_relay_dir(repo: &Path) -> Result<PathBuf, String> {
    if let Ok(p) = env::var("SDDIA_IOTA_RELAY_DIR") {
        let t = p.trim();
        if !t.is_empty() {
            let pb = PathBuf::from(t);
            return Ok(if pb.is_absolute() {
                pb
            } else {
                repo.join(pb)
            });
        }
    }
    let conv = repo.join(".SddIA/services/iota-publish-relay");
    if conv.join("server.mjs").is_file() {
        return Ok(conv);
    }
    Err(format!(
        "hijo Node ausente: set SDDIA_IOTA_RELAY_DIR o cree {}",
        conv.display()
    ))
}

fn health_url() -> String {
    if let Ok(url) = env::var("IOTA_PUBLISH_RELAY_URL") {
        let t = url.trim();
        if !t.is_empty() {
            if let Some(base) = t.strip_suffix("/v1/publish") {
                return format!("{base}/health");
            }
        }
    }
    let host = env::var("IOTA_PUBLISH_RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("IOTA_PUBLISH_RELAY_PORT").unwrap_or_else(|_| "8787".into());
    format!("http://{host}:{port}/health")
}

fn probe_health(url: &str) -> bool {
    let without_scheme = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);
    let (hostport, path) = match without_scheme.split_once('/') {
        Some((hp, rest)) => (hp.to_string(), format!("/{rest}")),
        None => (without_scheme.to_string(), "/health".to_string()),
    };
    let Ok(mut addrs) = hostport.to_socket_addrs() else {
        return false;
    };
    let Some(addr) = addrs.next() else {
        return false;
    };
    let Ok(mut stream) =
        TcpStream::connect_timeout(&addr, Duration::from_millis(HEALTH_TIMEOUT_MS))
    else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(HEALTH_TIMEOUT_MS)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(HEALTH_TIMEOUT_MS)));
    let host_hdr = hostport.split(':').next().unwrap_or("127.0.0.1");
    let req = format!("GET {path} HTTP/1.1\r\nHost: {host_hdr}\r\nConnection: close\r\n\r\n");
    if stream.write_all(req.as_bytes()).is_err() {
        return false;
    }
    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf).unwrap_or(0);
    let resp = String::from_utf8_lossy(&buf[..n]);
    resp.contains("200")
}

fn open_child_log(repo: &Path) -> Result<(Stdio, Stdio), String> {
    let log_dir = repo.join(".SddIA/daemons/logs");
    fs::create_dir_all(&log_dir).map_err(|e| format!("mkdir logs: {e}"))?;
    let log_path = log_dir.join(format!("{DAEMON_NAME}.log"));
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| format!("open log: {e}"))?;
    let file2 = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| format!("open log2: {e}"))?;
    Ok((Stdio::from(file), Stdio::from(file2)))
}

fn spawn_node_child(repo: &Path, relay_dir: &Path) -> Result<Child, String> {
    let server = relay_dir.join("server.mjs");
    if !server.is_file() {
        return Err(format!("server.mjs ausente en {}", relay_dir.display()));
    }
    let node_modules = relay_dir.join("node_modules");
    if !node_modules.is_dir() {
        eprintln!("[{DAEMON_NAME}] npm install en {}", relay_dir.display());
        let st = Command::new("npm")
            .arg("install")
            .arg("--silent")
            .current_dir(relay_dir)
            .status()
            .map_err(|e| format!("npm install: {e}"))?;
        if !st.success() {
            return Err(format!("npm install exit={st}"));
        }
    }
    let (stdout, stderr) = open_child_log(repo)?;
    Command::new("node")
        .arg("server.mjs")
        .current_dir(relay_dir)
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
        .map_err(|e| format!("spawn node: {e}"))
}

fn child_alive(child: &mut Child) -> bool {
    match child.try_wait() {
        Ok(None) => true,
        Ok(Some(status)) => {
            eprintln!("[{DAEMON_NAME}] hijo Node terminó: {status}");
            false
        }
        Err(e) => {
            eprintln!("[{DAEMON_NAME}] try_wait: {e}");
            false
        }
    }
}

fn main() {
    let repo = match find_repo_root() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[{DAEMON_NAME}] {e}");
            std::process::exit(1);
        }
    };
    let top = load_bus_topology(&repo);
    let mut centinela = DaemonRuntime::new(repo.clone(), DAEMON_NAME);
    if let Err(e) = centinela.bootstrap(&top) {
        eprintln!("[{DAEMON_NAME}] bootstrap: {e}");
        std::process::exit(1);
    }

    let relay_dir = match resolve_relay_dir(&repo) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("[{DAEMON_NAME}] {e}");
            centinela.shutdown();
            std::process::exit(1);
        }
    };

    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop_c = Arc::clone(&stop);
        let _ = ctrlc::set_handler(move || {
            stop_c.store(true, Ordering::SeqCst);
        });
    }

    let health = health_url();
    eprintln!(
        "[{DAEMON_NAME}] supervisando {} health={health}",
        relay_dir.display()
    );

    let mut child: Option<Child> = None;
    let mut child_spawned_at: Option<Instant> = None;
    let mut last_restart = Instant::now() - Duration::from_secs(RESTART_BACKOFF_SECS);

    while !stop.load(Ordering::SeqCst) {
        let mut child_alive_now = false;
        if let Some(ref mut c) = child {
            child_alive_now = child_alive(c);
            if !child_alive_now {
                child = None;
                child_spawned_at = None;
            }
        }

        if child.is_none() {
            if last_restart.elapsed() < Duration::from_secs(RESTART_BACKOFF_SECS) {
                thread::sleep(Duration::from_millis(200));
            } else {
                match spawn_node_child(&repo, &relay_dir) {
                    Ok(c) => {
                        eprintln!("[{DAEMON_NAME}] hijo Node pid={}", c.id());
                        child = Some(c);
                        child_spawned_at = Some(Instant::now());
                        child_alive_now = true;
                        last_restart = Instant::now();
                    }
                    Err(e) => {
                        eprintln!("[{DAEMON_NAME}] spawn falló: {e}");
                        last_restart = Instant::now();
                    }
                }
            }
        }

        let health_ok = probe_health(&health);
        let grace = in_grace(
            child.is_some(),
            child_spawned_at.map(|t| t.elapsed()),
            Duration::from_secs(GRACE_SECS),
        );
        let action = decide_supervisor_tick(health_ok, grace, child_alive_now);

        if let Err(e) = centinela.tick_with_status(&top, action.heartbeat_status) {
            eprintln!("[{DAEMON_NAME}] heartbeat: {e}");
        }

        if action.kill_child {
            if let Some(ref mut c) = child {
                eprintln!("[{DAEMON_NAME}] /health falló con hijo vivo; reinicio");
                let _ = c.kill();
                let _ = c.wait();
            }
            child = None;
            child_spawned_at = None;
        }

        thread::sleep(Duration::from_secs(TICK_SECS));
    }

    if let Some(mut c) = child.take() {
        let _ = c.kill();
        let _ = c.wait();
    }
    centinela.shutdown();
    eprintln!("[{DAEMON_NAME}] shutdown limpio");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_url_strips_publish_path() {
        std::env::set_var(
            "IOTA_PUBLISH_RELAY_URL",
            "http://127.0.0.1:8787/v1/publish",
        );
        assert_eq!(health_url(), "http://127.0.0.1:8787/health");
        std::env::remove_var("IOTA_PUBLISH_RELAY_URL");
    }

    #[test]
    fn socket_addr_unused_ok() {
        let _: Option<SocketAddr> = "127.0.0.1:8787".parse().ok();
    }

    #[test]
    fn grace_refused_does_not_kill() {
        let grace = Duration::from_secs(GRACE_SECS);
        let action = decide_supervisor_tick(false, in_grace(true, Some(Duration::ZERO), grace), true);
        assert_eq!(action.heartbeat_status, "alive");
        assert!(!action.kill_child);
    }

    #[test]
    fn post_grace_refused_kills_and_ticks_degraded() {
        let grace = Duration::from_secs(GRACE_SECS);
        let action = decide_supervisor_tick(
            false,
            in_grace(true, Some(Duration::from_secs(GRACE_SECS)), grace),
            true,
        );
        assert_eq!(action.heartbeat_status, "degraded");
        assert!(action.kill_child);
    }

    #[test]
    fn healthy_ticks_alive_no_kill() {
        let grace = Duration::from_secs(GRACE_SECS);
        let action = decide_supervisor_tick(true, in_grace(true, Some(Duration::from_secs(60)), grace), true);
        assert_eq!(action.heartbeat_status, "alive");
        assert!(!action.kill_child);
    }

    #[test]
    fn grace_boundary_eq_is_outside() {
        let grace = Duration::from_secs(GRACE_SECS);
        assert!(!in_grace(true, Some(Duration::from_secs(GRACE_SECS)), grace));
    }

    #[test]
    fn post_grace_no_child_ticks_degraded() {
        let grace = Duration::from_secs(GRACE_SECS);
        let action = decide_supervisor_tick(
            false,
            in_grace(false, None, grace),
            false,
        );
        assert_eq!(action.heartbeat_status, "degraded");
        assert!(!action.kill_child);
    }
}
