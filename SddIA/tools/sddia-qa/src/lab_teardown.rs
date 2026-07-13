use execute_process::engine::eda_bus_topology::{list_witnesses, EventBusTopology};
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn keep_tmp() -> bool {
    matches!(
        std::env::var("SDDIA_LAB_KEEP_TMP")
            .unwrap_or_default()
            .to_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

fn remove_index_row(index_path: &Path, name: &str) -> bool {
    let Ok(text) = fs::read_to_string(index_path) else {
        return false;
    };
    let filtered: Vec<_> = text
        .lines()
        .filter(|line| !(line.starts_with('|') && line.contains(name)))
        .collect();
    if filtered.len() == text.lines().count() {
        return false;
    }
    fs::write(index_path, format!("{}\n", filtered.join("\n"))).ok();
    true
}

pub fn cleanup_eda_bus_event(repo: &Path, bus: &EventBusTopology, event_id: &str) {
    for key in ["pending", "processing", "processed", "dead_letter"] {
        let rel = match key {
            "pending" => &bus.pending,
            "processing" => &bus.processing,
            "processed" => &bus.processed,
            "dead_letter" => &bus.dead_letter,
            _ => continue,
        };
        let _ = fs::remove_file(repo.join(rel).join(format!("{event_id}.json")));
    }
    for state_key in [
        "processing_subscribers",
        "processed_subscribers",
        "dead_letter_subscribers",
    ] {
        let paths = list_witnesses(repo, bus, state_key, event_id);
        for p in paths {
            let _ = fs::remove_file(p);
        }
    }
}

pub fn cleanup_lab_entity_forge(
    repo: &Path,
    bus: &EventBusTopology,
    entity_class: &str,
    entity_name: &str,
    event_id: Option<&str>,
) -> serde_json::Value {
    if keep_tmp() {
        return serde_json::json!({"skipped": true});
    }
    let mut cleaned = serde_json::json!({
        "artifact_removed": false,
        "index_row_removed": false,
        "bus_cleaned": false,
    });
    if entity_class == "tool" {
        let artifact = repo.join(".SddIA/tools").join(format!("{entity_name}.md"));
        let index_path = repo.join(".SddIA/tools/index.md");
        if artifact.is_file() {
            let _ = fs::remove_file(&artifact);
            cleaned["artifact_removed"] = true.into();
        }
        cleaned["index_row_removed"] = remove_index_row(&index_path, entity_name).into();
    }
    if let Some(eid) = event_id {
        cleanup_eda_bus_event(repo, bus, eid);
        cleaned["bus_cleaned"] = true.into();
    }
    cleaned
}

pub fn cleanup_orphan_core_eda_e2e_tools(repo: &Path) -> Vec<String> {
    if keep_tmp() {
        return vec![];
    }
    let tools_dir = repo.join("SddIA/tools");
    if !tools_dir.is_dir() {
        return vec![];
    }
    let pattern =
        Regex::new(r"^eda-e2e-(tool|action|process|agent|norm|codex|skill|event)-[0-9a-f]{8}\.md$")
            .expect("regex");
    let mut removed = Vec::new();
    let Ok(rd) = fs::read_dir(&tools_dir) else {
        return removed;
    };
    for entry in rd.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if pattern.is_match(name) {
            let _ = fs::remove_file(&path);
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                remove_index_row(&tools_dir.join("index.md"), stem);
            }
            removed.push(
                path.strip_prefix(repo)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
    removed
}
