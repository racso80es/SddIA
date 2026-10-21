//! Sensor Linear → `Tracker_Issue_Changed` (HU-B 02). Sin apply markdown.

use crate::eda_bus::{ensure_event_bus_topology, write_json_atomic};
use chrono::{DateTime, Utc};
use sddia_event_store::{record_ecst, runtime_db_path, store_mode, writes_filesystem, writes_store};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const EMITTER: &str = "linear-bridge-watcher";
const STATE_DIR: &str = ".SddIA/state/linear-bridge-watcher";
const OUTBOUND_DIR: &str = ".SddIA/state/tracker-outbound";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueSnap {
    pub state: String,
    pub title: String,
    pub description_len: u64,
    pub priority: i64,
    pub labels: Vec<String>,
    pub parent: Option<String>,
    pub cancelled: bool,
    pub linear_updated_at: Option<String>,
    #[serde(default)]
    pub last_comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatcherState {
    #[serde(default)]
    pub issues: HashMap<String, IssueSnap>,
}

pub fn state_path(repo: &Path, project_slug: &str) -> PathBuf {
    repo.join(STATE_DIR).join(format!("{project_slug}.json"))
}

pub fn load_state(repo: &Path, project_slug: &str) -> WatcherState {
    let path = state_path(repo, project_slug);
    if !path.is_file() {
        return WatcherState {
            issues: HashMap::new(),
        };
    }
    let raw = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&raw).unwrap_or(WatcherState {
        issues: HashMap::new(),
    })
}

pub fn save_state(repo: &Path, project_slug: &str, state: &WatcherState) -> Result<(), String> {
    let path = state_path(repo, project_slug);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_json_atomic(&path, &json!(state))
}

fn label_strings(item: &Value) -> Vec<String> {
    item.get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

pub fn issue_ref_from_item(item: &Value) -> Option<String> {
    item.get("identifier")
        .or_else(|| item.get("issue_ref"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

pub fn snap_from_item(item: &Value) -> Option<IssueSnap> {
    let state = item
        .get("state")
        .and_then(|v| v.as_str())
        .unwrap_or("Backlog")
        .to_string();
    let title = item
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let description_len = item
        .get("description_bytes")
        .and_then(|v| v.as_u64())
        .or_else(|| {
            item.get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.len() as u64)
        })
        .unwrap_or(0);
    let priority = item.get("priority").and_then(|v| v.as_i64()).unwrap_or(0);
    let labels = label_strings(item);
    let parent = item
        .get("parent")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let cancelled = item
        .get("cancelled")
        .and_then(|v| v.as_bool())
        .unwrap_or_else(|| state.eq_ignore_ascii_case("cancelled"));
    let linear_updated_at = item
        .get("updated_at")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let last_comment = item
        .get("last_comment")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    Some(IssueSnap {
        state,
        title,
        description_len,
        priority,
        labels,
        parent,
        cancelled,
        linear_updated_at,
        last_comment,
    })
}

pub fn has_tracked_label(item: &Value, hu_label: &str, pbi_label: &str) -> bool {
    let labels = label_strings(item);
    labels.iter().any(|l| l == hu_label || l == pbi_label)
}

pub fn diff_snapshots(from: &IssueSnap, to: &IssueSnap) -> Vec<Value> {
    let mut changes = Vec::new();
    if from.state != to.state {
        changes.push(json!({
            "field": "state",
            "from": from.state,
            "to": to.state,
        }));
    }
    if from.title != to.title {
        changes.push(json!({
            "field": "title",
            "from": from.title,
            "to": to.title,
        }));
    }
    if from.description_len != to.description_len {
        changes.push(json!({
            "field": "description",
            "from": from.description_len,
            "to": to.description_len,
        }));
    }
    if from.priority != to.priority {
        changes.push(json!({
            "field": "priority",
            "from": from.priority,
            "to": to.priority,
        }));
    }
    if from.labels != to.labels {
        changes.push(json!({
            "field": "labels",
            "from": from.labels,
            "to": to.labels,
        }));
    }
    if from.parent != to.parent {
        changes.push(json!({
            "field": "parent",
            "from": from.parent,
            "to": to.parent,
        }));
    }
    if from.cancelled != to.cancelled {
        changes.push(json!({
            "field": "cancelled",
            "from": from.cancelled,
            "to": to.cancelled,
        }));
    }
    changes
}

pub fn outbound_echo_window_secs() -> i64 {
    std::env::var("SDDIA_TRACKER_OUTBOUND_ECHO_WINDOW_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(300)
}

fn parse_rfc3339(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

pub fn outbound_suppresses(repo: &Path, issue_ref: &str, now: DateTime<Utc>) -> bool {
    let safe = issue_ref
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
        .collect::<String>();
    let path = repo.join(OUTBOUND_DIR).join(format!("{safe}.jsonl"));
    if !path.is_file() {
        return false;
    }
    let window = outbound_echo_window_secs();
    let raw = fs::read_to_string(&path).unwrap_or_default();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let v: Value = match serde_json::from_str(line) {
            Ok(x) => x,
            Err(_) => continue,
        };
        if v.get("issue_ref").and_then(|x| x.as_str()) != Some(issue_ref) {
            continue;
        }
        let Some(at) = v.get("occurred_at").and_then(|x| x.as_str()) else {
            return true;
        };
        if let Some(ts) = parse_rfc3339(at) {
            if (now - ts).num_seconds().abs() <= window {
                return true;
            }
        } else {
            return true;
        }
    }
    false
}

pub fn comment_has_sync_id(snap: &IssueSnap) -> bool {
    snap.last_comment
        .as_deref()
        .map(|c| c.contains("sddia-sync-id"))
        .unwrap_or(false)
}

pub fn should_suppress_echo(repo: &Path, issue_ref: &str, snap: &IssueSnap, now: DateTime<Utc>) -> bool {
    comment_has_sync_id(snap) || outbound_suppresses(repo, issue_ref, now)
}

pub fn compose_tracker_issue_changed(
    project_slug: &str,
    issue_ref: &str,
    changes: Vec<Value>,
    linear_updated_at: Option<&str>,
    source: &str,
) -> Value {
    let event_id = Uuid::new_v4().to_string();
    let occurred_at = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let mut payload = json!({
        "correlation_id": event_id,
        "occurred_at": occurred_at,
        "project_slug": project_slug,
        "issue_ref": issue_ref,
        "changes": changes,
        "source": source,
    });
    if let Some(lu) = linear_updated_at.filter(|s| !s.is_empty()) {
        payload["linear_updated_at"] = json!(lu);
    }
    json!({
        "event_id": event_id,
        "event_type": "Tracker_Issue_Changed",
        "timestamp": occurred_at,
        "emitter_agent": EMITTER,
        "payload": payload,
        "delivery_state": {},
    })
}

pub fn materialize_pending(repo: &Path, bus_event: &Value) -> Result<PathBuf, String> {
    let bus = ensure_event_bus_topology(repo)?;
    let event_id = bus_event
        .get("event_id")
        .and_then(|v| v.as_str())
        .ok_or("event_id ausente")?;
    let mode = store_mode();
    if writes_store(mode) {
        record_ecst(repo, "domain", bus_event).map_err(|err| err.to_string())?;
    }
    if !writes_filesystem(mode) {
        return Ok(runtime_db_path(repo));
    }
    let target = bus.pending.join(format!("{event_id}.json"));
    write_json_atomic(&target, bus_event)?;
    Ok(target)
}

/// Procesa un lote `list_issues` (ítems ya reducidos). Devuelve rutas de eventos materializados.
pub fn assimilate_list_issues(
    repo: &Path,
    project_slug: &str,
    hu_label: &str,
    pbi_label: &str,
    items: &[Value],
    now: DateTime<Utc>,
) -> Result<(Vec<PathBuf>, WatcherState), String> {
    assimilate_tracker_items(repo, project_slug, hu_label, pbi_label, items, now, "poll")
}

pub fn assimilate_tracker_items(
    repo: &Path,
    project_slug: &str,
    hu_label: &str,
    pbi_label: &str,
    items: &[Value],
    now: DateTime<Utc>,
    source: &str,
) -> Result<(Vec<PathBuf>, WatcherState), String> {
    let mut state = load_state(repo, project_slug);
    let mut emitted = Vec::new();
    for item in items {
        if !has_tracked_label(item, hu_label, pbi_label) {
            continue;
        }
        let issue_ref = issue_ref_from_item(item).ok_or("issue sin identifier")?;
        let new_snap = snap_from_item(item).ok_or("snap inválido")?;
        if should_suppress_echo(repo, &issue_ref, &new_snap, now) {
            state.issues.insert(issue_ref, new_snap);
            continue;
        }
        let changes = match state.issues.get(&issue_ref) {
            Some(prev) => diff_snapshots(prev, &new_snap),
            None => Vec::new(),
        };
        state.issues.insert(issue_ref.clone(), new_snap.clone());
        if changes.is_empty() {
            continue;
        }
        let bus_event = compose_tracker_issue_changed(
            project_slug,
            &issue_ref,
            changes,
            new_snap.linear_updated_at.as_deref(),
            source,
        );
        emitted.push(materialize_pending(repo, &bus_event)?);
    }
    save_state(repo, project_slug, &state)?;
    Ok((emitted, state))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tmp_repo() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("linear-bridge-{}", Uuid::new_v4()));
        fs::create_dir_all(dir.join(".events/pending")).unwrap();
        fs::create_dir_all(dir.join(OUTBOUND_DIR)).unwrap();
        dir
    }

    fn item(id: &str, state: &str, labels: &[&str]) -> Value {
        json!({
            "identifier": id,
            "title": "t",
            "state": state,
            "priority": 0,
            "labels": labels,
            "updated_at": "2026-10-04T12:00:00Z",
        })
    }

    #[test]
    fn ac11_state_backlog_to_todo_emits_poll_event() {
        let repo = tmp_repo();
        let slug = "lab";
        let hu = "hu";
        let pbi = "pbi";
        let first = item("OSC-11", "Backlog", &["hu"]);
        let _ = assimilate_list_issues(&repo, slug, hu, pbi, &[first], Utc::now()).unwrap();
        let second = item("OSC-11", "Todo", &["hu"]);
        let (paths, _) =
            assimilate_list_issues(&repo, slug, hu, pbi, &[second], Utc::now()).unwrap();
        assert_eq!(paths.len(), 1);
        let raw = fs::read_to_string(&paths[0]).unwrap();
        let ev: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(ev.get("event_type"), Some(&json!("Tracker_Issue_Changed")));
        assert_eq!(
            ev.pointer("/payload/source"),
            Some(&json!("poll"))
        );
        let changes = ev.pointer("/payload/changes").and_then(|v| v.as_array()).unwrap();
        assert!(changes.iter().any(|c| c.get("field") == Some(&json!("state"))));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn ac11_issue_without_hu_pbi_label_skipped() {
        let repo = tmp_repo();
        let only_other = item("OSC-99", "Todo", &["other"]);
        let (paths, state) =
            assimilate_list_issues(&repo, "lab", "hu", "pbi", &[only_other], Utc::now()).unwrap();
        assert!(paths.is_empty());
        assert!(state.issues.is_empty());
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn ac12_outbound_record_suppresses_event() {
        let repo = tmp_repo();
        let slug = "lab";
        let path = repo.join(OUTBOUND_DIR).join("OSC-12.jsonl");
        let line = json!({
            "event_id": "e1",
            "operation": "update_issue_state",
            "issue_ref": "OSC-12",
            "occurred_at": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            "source_process": "tracker-stamp",
        });
        fs::write(&path, format!("{}\n", line)).unwrap();
        let seed = item("OSC-12", "Backlog", &["pbi"]);
        assimilate_list_issues(&repo, slug, "hu", "pbi", &[seed], Utc::now()).unwrap();
        let changed = item("OSC-12", "Todo", &["pbi"]);
        let (paths, _) =
            assimilate_list_issues(&repo, slug, "hu", "pbi", &[changed], Utc::now()).unwrap();
        assert!(paths.is_empty());
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn ac12_sync_id_in_comment_suppresses() {
        let repo = tmp_repo();
        let slug = "lab";
        let seed = json!({
            "identifier": "OSC-13",
            "title": "t",
            "state": "Backlog",
            "labels": ["hu"],
            "last_comment": "note sddia-sync-id: abc",
        });
        assimilate_list_issues(&repo, slug, "hu", "pbi", &[seed], Utc::now()).unwrap();
        let changed = json!({
            "identifier": "OSC-13",
            "title": "t",
            "state": "Todo",
            "labels": ["hu"],
            "last_comment": "note sddia-sync-id: abc",
        });
        let (paths, _) =
            assimilate_list_issues(&repo, slug, "hu", "pbi", &[changed], Utc::now()).unwrap();
        assert!(paths.is_empty());
        let _ = fs::remove_dir_all(&repo);
    }
}
