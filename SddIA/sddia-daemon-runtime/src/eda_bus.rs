use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct EdBusPaths {
    pub pending: PathBuf,
    pub processing: PathBuf,
    pub processing_subscribers: PathBuf,
    pub processed: PathBuf,
    pub processed_subscribers: PathBuf,
    pub dead_letter: PathBuf,
    pub dead_letter_subscribers: PathBuf,
    pub subscriptions: PathBuf,
}

fn normalize_rel(path: &str) -> String {
    let p = path.replace('\\', "/");
    if let Some(stripped) = p.strip_prefix("./") {
        stripped.to_string()
    } else {
        p
    }
}

fn bus_defaults_from_root(event_bus: &str) -> EdBusPaths {
    let root = normalize_rel(event_bus.trim_end_matches('/'));
    EdBusPaths {
        pending: PathBuf::from(format!("{root}/pending")),
        processing: PathBuf::from(format!("{root}/processing")),
        processing_subscribers: PathBuf::from(format!("{root}/processing/subscribers")),
        processed: PathBuf::from(format!("{root}/processed")),
        processed_subscribers: PathBuf::from(format!("{root}/processed/subscribers")),
        dead_letter: PathBuf::from(format!("{root}/dead-letter")),
        dead_letter_subscribers: PathBuf::from(format!("{root}/dead-letter/subscribers")),
        subscriptions: PathBuf::from("SddIA/core/event-domain-subscriptions.json"),
    }
}

fn rel_to_abs(repo: &Path, rel: PathBuf) -> PathBuf {
    if rel.is_absolute() {
        rel
    } else {
        repo.join(rel)
    }
}

pub fn load_eda_bus(repo: &Path) -> EdBusPaths {
    let env_bus = env::var("EVENT_BUS_PATH").unwrap_or_default();
    let mut paths = if env_bus.trim().is_empty() {
        bus_defaults_from_root(".events")
    } else {
        bus_defaults_from_root(&env_bus)
    };

    if env_bus.trim().is_empty() {
        if let Ok(cfg) = super::load_cumulo(repo) {
            if let Some(eb) = cfg.get("event_bus").and_then(|v| v.as_str()) {
                if !eb.trim().is_empty() {
                    paths = bus_defaults_from_root(eb);
                }
            }
            if let Some(bus) = cfg.get("eda_bus").and_then(|v| v.as_object()) {
                if let Some(p) = bus.get("pending").and_then(|v| v.as_str()) {
                    paths.pending = PathBuf::from(normalize_rel(p));
                }
                for (key, sub_key) in [
                    ("processing", "processing_subscribers"),
                    ("processed", "processed_subscribers"),
                    ("dead_letter", "dead_letter_subscribers"),
                ] {
                    if let Some(p) = bus.get(key).and_then(|v| v.as_str()) {
                        let norm = normalize_rel(p);
                        match key {
                            "processing" => paths.processing = PathBuf::from(&norm),
                            "processed" => paths.processed = PathBuf::from(&norm),
                            "dead_letter" => paths.dead_letter = PathBuf::from(&norm),
                            _ => {}
                        }
                        paths = set_subscriber_path(
                            paths,
                            sub_key,
                            &format!("{}/subscribers", p.trim_end_matches('/')),
                        );
                    }
                }
                if let Some(subs) = bus.get("subscribers").and_then(|v| v.as_object()) {
                    for (legacy, flat) in [
                        ("processing", "processing_subscribers"),
                        ("processed", "processed_subscribers"),
                        ("dead_letter", "dead_letter_subscribers"),
                    ] {
                        if let Some(p) = subs.get(legacy).and_then(|v| v.as_str()) {
                            paths = set_subscriber_path(paths, flat, p);
                        }
                    }
                }
                if let Some(s) = bus.get("subscriptions").and_then(|v| v.as_str()) {
                    paths.subscriptions = PathBuf::from(normalize_rel(s));
                }
            }
        }
    }

    EdBusPaths {
        pending: rel_to_abs(repo, paths.pending),
        processing: rel_to_abs(repo, paths.processing),
        processing_subscribers: rel_to_abs(repo, paths.processing_subscribers),
        processed: rel_to_abs(repo, paths.processed),
        processed_subscribers: rel_to_abs(repo, paths.processed_subscribers),
        dead_letter: rel_to_abs(repo, paths.dead_letter),
        dead_letter_subscribers: rel_to_abs(repo, paths.dead_letter_subscribers),
        subscriptions: rel_to_abs(repo, paths.subscriptions),
    }
}

fn set_subscriber_path(mut paths: EdBusPaths, key: &str, rel: &str) -> EdBusPaths {
    let p = PathBuf::from(normalize_rel(rel));
    match key {
        "processing_subscribers" => paths.processing_subscribers = p,
        "processed_subscribers" => paths.processed_subscribers = p,
        "dead_letter_subscribers" => paths.dead_letter_subscribers = p,
        _ => {}
    }
    paths
}

pub fn ensure_event_bus_topology(repo: &Path) -> Result<EdBusPaths, String> {
    let bus = load_eda_bus(repo);
    for dir in [
        &bus.pending,
        &bus.processing,
        &bus.processing_subscribers,
        &bus.processed,
        &bus.processed_subscribers,
        &bus.dead_letter,
        &bus.dead_letter_subscribers,
    ] {
        fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    }
    Ok(bus)
}

pub fn header_path(bus: &EdBusPaths, state: &str, event_uuid: &str) -> PathBuf {
    let base = match state {
        "processing" => &bus.processing,
        "processed" => &bus.processed,
        "dead_letter" => &bus.dead_letter,
        _ => &bus.processing,
    };
    base.join(format!("{event_uuid}.json"))
}

pub fn load_registry(bus: &EdBusPaths) -> Result<Value, String> {
    let raw = fs::read_to_string(&bus.subscriptions).map_err(|e| format!("subscriptions: {e}"))?;
    serde_json::from_str(&raw).map_err(|e| format!("subscriptions JSON: {e}"))
}
