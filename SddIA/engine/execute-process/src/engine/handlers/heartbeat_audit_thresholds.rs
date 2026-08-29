//! SSOT umbrales `daemon-heartbeat-audit` (overlay instancia sobre Core).

use super::super::daemons::{load_cumulo, state_dir};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeartbeatAuditThresholds {
    pub missed_cycles_threshold: i64,
    pub suspend_skew_seconds: i64,
}

impl Default for HeartbeatAuditThresholds {
    fn default() -> Self {
        Self {
            missed_cycles_threshold: 3,
            suspend_skew_seconds: 120,
        }
    }
}

fn parse_thresholds_value(body: &Value) -> HeartbeatAuditThresholds {
    let mut t = HeartbeatAuditThresholds::default();
    if let Some(v) = body.get("missed_cycles_threshold").and_then(|x| x.as_i64()) {
        if v > 0 {
            t.missed_cycles_threshold = v;
        }
    }
    if let Some(v) = body.get("suspend_skew_seconds").and_then(|x| x.as_i64()) {
        if v > 0 {
            t.suspend_skew_seconds = v;
        }
    }
    t
}

fn core_thresholds_path(repo: &Path) -> Result<std::path::PathBuf, String> {
    let cfg = load_cumulo(repo)?;
    let rel = cfg
        .get("argos")
        .and_then(|a| a.get("heartbeat_audit_thresholds"))
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/daemons/heartbeat-audit.thresholds.json");
    Ok(repo.join(rel.trim().trim_start_matches("./")))
}

fn overlay_thresholds_path(repo: &Path) -> Result<std::path::PathBuf, String> {
    Ok(state_dir(repo)?.join("heartbeat-audit.thresholds.json"))
}

fn read_thresholds_file(path: &Path) -> Option<HeartbeatAuditThresholds> {
    let text = fs::read_to_string(path).ok()?;
    let body: Value = serde_json::from_str(&text).ok()?;
    Some(parse_thresholds_value(&body))
}

/// Core default con overlay opcional en `daemons_instance.state`.
pub fn load_heartbeat_audit_thresholds(repo: &Path) -> HeartbeatAuditThresholds {
    let core = core_thresholds_path(repo)
        .ok()
        .and_then(|p| read_thresholds_file(&p))
        .unwrap_or_default();
    let overlay = overlay_thresholds_path(repo)
        .ok()
        .and_then(|p| read_thresholds_file(&p));
    overlay.unwrap_or(core)
}

pub fn monotonic_ms() -> Option<u64> {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) == 0 {
            Some((ts.tv_sec as u64) * 1000 + (ts.tv_nsec as u64) / 1_000_000)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_repo_layout(repo: &Path, overlay: Option<&str>) {
        fs::create_dir_all(repo.join("SddIA/daemons")).unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"argos":{"heartbeat_audit_thresholds":"SddIA/daemons/heartbeat-audit.thresholds.json"},"daemons_instance":{"state":".SddIA/daemons/state"}}"#,
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/daemons/heartbeat-audit.thresholds.json"),
            r#"{"missed_cycles_threshold":3,"suspend_skew_seconds":120}"#,
        )
        .unwrap();
        let overlay_dir = repo.join(".SddIA/daemons/state");
        fs::create_dir_all(&overlay_dir).unwrap();
        if let Some(body) = overlay {
            let mut f =
                fs::File::create(overlay_dir.join("heartbeat-audit.thresholds.json")).unwrap();
            write!(f, "{body}").unwrap();
        }
    }

    #[test]
    fn thresholds_from_ssot_json() {
        let dir = tempfile::TempDir::new().unwrap();
        write_repo_layout(dir.path(), None);
        fs::write(
            dir.path()
                .join("SddIA/daemons/heartbeat-audit.thresholds.json"),
            r#"{"missed_cycles_threshold":5,"suspend_skew_seconds":90}"#,
        )
        .unwrap();
        let t = load_heartbeat_audit_thresholds(dir.path());
        assert_eq!(t.missed_cycles_threshold, 5);
        assert_eq!(t.suspend_skew_seconds, 90);
    }

    #[test]
    fn overlay_wins_over_core() {
        let dir = tempfile::TempDir::new().unwrap();
        write_repo_layout(
            dir.path(),
            Some(r#"{"missed_cycles_threshold":7,"suspend_skew_seconds":200}"#),
        );
        let t = load_heartbeat_audit_thresholds(dir.path());
        assert_eq!(t.missed_cycles_threshold, 7);
        assert_eq!(t.suspend_skew_seconds, 200);
    }
}
