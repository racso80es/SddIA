//! Abstención si cargo/rustc o event-watcher activos (Linux /proc).

use std::fs;
use std::path::{Path, PathBuf};

pub fn target_dir_for_repo(repo: &Path) -> PathBuf {
    repo.join("SddIA/target")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcGate {
    Clear,
    Blocked,
    ProcUnreadable,
}

pub fn cargo_build_gate(repo: &Path) -> ProcGate {
    let target = target_dir_for_repo(repo);
    let target_canon = target.canonicalize().unwrap_or(target);
    let Ok(entries) = fs::read_dir("/proc") else {
        return ProcGate::ProcUnreadable;
    };
    for entry in entries.flatten() {
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        if !name.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let cmdline_path = entry.path().join("cmdline");
        let Ok(raw) = fs::read(&cmdline_path) else {
            continue;
        };
        if raw.is_empty() {
            continue;
        }
        let cmd: String = raw
            .split(|&b| b == 0)
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect::<Vec<_>>()
            .join(" ");
        let lower = cmd.to_ascii_lowercase();
        if !(lower.contains("cargo") || lower.contains("rustc")) {
            continue;
        }
        let env_path = entry.path().join("environ");
        if let Ok(env_raw) = fs::read(env_path) {
            let env = env_raw
                .split(|&b| b == 0)
                .filter_map(|b| std::str::from_utf8(b).ok())
                .find(|s| s.starts_with("CARGO_TARGET_DIR="));
            if let Some(line) = env {
                let dir = line.trim_start_matches("CARGO_TARGET_DIR=");
                let p = PathBuf::from(dir);
                let canon = p.canonicalize().unwrap_or(p);
                if canon == target_canon || canon.starts_with(&target_canon) {
                    return ProcGate::Blocked;
                }
            }
        }
        if lower.contains("sddia/target") || cmd.contains("SddIA/target") {
            return ProcGate::Blocked;
        }
    }
    ProcGate::Clear
}

pub fn cargo_build_active_on_target(repo: &Path) -> bool {
    matches!(cargo_build_gate(repo), ProcGate::Blocked)
}

pub fn event_watcher_gate() -> ProcGate {
    let Ok(entries) = fs::read_dir("/proc") else {
        return ProcGate::ProcUnreadable;
    };
    for entry in entries.flatten() {
        let cmdline_path = entry.path().join("cmdline");
        let Ok(raw) = fs::read(cmdline_path) else {
            continue;
        };
        let blob = String::from_utf8_lossy(&raw);
        if blob.contains("event-watcher") {
            return ProcGate::Blocked;
        }
    }
    ProcGate::Clear
}

pub fn event_watcher_active() -> bool {
    matches!(event_watcher_gate(), ProcGate::Blocked)
}
