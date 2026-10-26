//! Plan de entregas. El store no lee el catálogo; este crate sí, y solo las rutas fijas del core.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use serde_json::{json, Map, Value};
use sddia_event_store::{
    DeliveryStatus, EventStore, Family, StoreError, runtime_db_path,
};

const CORE_REGISTRY: &str = "SddIA/core/event-domain-subscriptions.json";
const CODEX_REGISTRY: &str =
    "SddIA/library/codexes/codex-software-engineering/subscriptions.json";
const SOFTWARE_SLUG: &str = "codex-software-engineering";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutePlan {
    pub subscribers: Vec<Value>,
    pub dead_letter_no_subscriber: bool,
}

pub fn subscriber_id(subscriber: &Value) -> String {
    let agent = subscriber
        .get("agent")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("unknown");
    for key in ["process", "action", "tool"] {
        if let Some(v) = subscriber.get(key).and_then(|x| x.as_str()) {
            let token = v.trim();
            if !token.is_empty() {
                return format!("{agent}.{token}");
            }
        }
    }
    agent.to_string()
}

pub fn resolve_origin_topology(payload: &Value) -> String {
    payload
        .get("origin_topology")
        .and_then(|v| v.as_str())
        .filter(|s| *s == "core" || *s == "local")
        .unwrap_or("core")
        .to_string()
}

pub fn subscriber_applies_to_topology(subscriber: &Value, origin_topology: &str) -> bool {
    let applies = subscriber
        .get("applies_to_origin_topology")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str())
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["core".to_string()]);
    applies.is_empty() || applies.iter().any(|t| t == origin_topology)
}

pub fn plan_route(
    core: &Value,
    codex: Option<&Value>,
    authority: bool,
    event_type: &str,
) -> RoutePlan {
    let empty = Value::Null;
    let codex = codex.unwrap_or(&empty);
    let in_codex = codex.get(event_type).is_some();
    let in_core = core.get(event_type).is_some();
    if in_codex && !in_core && !authority {
        return RoutePlan {
            subscribers: Vec::new(),
            dead_letter_no_subscriber: true,
        };
    }
    let mut subscribers = Vec::new();
    if let Some(arr) = core.get(event_type).and_then(|v| v.as_array()) {
        subscribers.extend(arr.iter().cloned());
    }
    if authority {
        if let Some(arr) = codex.get(event_type).and_then(|v| v.as_array()) {
            subscribers.extend(arr.iter().cloned());
        }
    }
    RoutePlan {
        subscribers,
        dead_letter_no_subscriber: false,
    }
}

fn software_authority(repo: &Path) -> bool {
    let Ok(text) = fs::read_to_string(repo.join(".SddIA/project.md")) else {
        return false;
    };
    text.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("codex_slug:") && trimmed.contains(SOFTWARE_SLUG)
    })
}

fn load_json(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|err| format!("EDA_DELIVERY_PLAN_UNAVAILABLE: {err}"))?;
    serde_json::from_str(text.trim_start_matches('\u{feff}'))
        .map_err(|err| format!("EDA_DELIVERY_PLAN_UNAVAILABLE: {err}"))
}

fn has_subscriber_keys(event: &Value) -> bool {
    event
        .get("delivery_state")
        .and_then(|v| v.as_object())
        .is_some_and(|map| {
            map.keys().any(|key| {
                let token = key.trim();
                !token.is_empty() && !token.starts_with('_')
            })
        })
}

fn applicable_ids(plan: &RoutePlan, payload: &Value) -> Vec<String> {
    let origin = resolve_origin_topology(payload);
    let mut ids = Vec::new();
    let mut seen = HashSet::new();
    for subscriber in &plan.subscribers {
        if !subscriber_applies_to_topology(subscriber, &origin) {
            continue;
        }
        let id = subscriber_id(subscriber);
        if seen.insert(id.clone()) {
            ids.push(id);
        }
    }
    ids
}

/// Rellena `delivery_state` desde el catálogo si el emisor no trajo suscriptores.
pub fn stamp_published_event(repo: &Path, event: &mut Value) -> Result<(), String> {
    if has_subscriber_keys(event) {
        return Ok(());
    }
    let registry_path = repo.join(CORE_REGISTRY);
    if !registry_path.is_file() {
        mark_no_subscriber(event);
        return Ok(());
    }
    let registry = load_json(&registry_path)?;
    let event_type = event
        .get("event_type")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "EDA_DELIVERY_PLAN_UNAVAILABLE: event_type vacío".to_string())?;
    let authority = software_authority(repo);
    let codex = if authority {
        let path = repo.join(CODEX_REGISTRY);
        if path.is_file() {
            Some(load_json(&path)?)
        } else {
            None
        }
    } else {
        None
    };
    let plan = plan_route(&registry, codex.as_ref(), authority, event_type);
    let payload = event.get("payload").cloned().unwrap_or_else(|| json!({}));
    let ids = if plan.dead_letter_no_subscriber {
        Vec::new()
    } else {
        applicable_ids(&plan, &payload)
    };
    if ids.is_empty() {
        mark_no_subscriber(event);
        return Ok(());
    }
    let mut map = Map::new();
    for id in ids {
        map.insert(id, json!("pending"));
    }
    if let Some(obj) = event.as_object_mut() {
        obj.insert("delivery_state".to_string(), Value::Object(map));
    }
    Ok(())
}

fn mark_no_subscriber(event: &mut Value) {
    if let Some(obj) = event.as_object_mut() {
        obj.insert(
            "delivery_state".to_string(),
            json!({"_disposition": "no_subscriber"}),
        );
    }
}

/// Siembra entregas que el alta dejó vacías y apaga filas cuyo id no está en el catálogo.
pub fn reconcile_unstamped(repo: &Path) -> Result<usize, String> {
    let registry_path = repo.join(CORE_REGISTRY);
    if !registry_path.is_file() {
        return Ok(0);
    }
    let db = runtime_db_path(repo);
    if !db.is_file() {
        return Ok(0);
    }
    let registry = load_json(&registry_path)?;
    let authority = software_authority(repo);
    let codex = if authority {
        let path = repo.join(CODEX_REGISTRY);
        if path.is_file() {
            Some(load_json(&path)?)
        } else {
            None
        }
    } else {
        None
    };
    let store = EventStore::open(&db).map_err(|err| err.to_string())?;
    let events = store
        .list_family(Family::Domain)
        .map_err(|err| err.to_string())?;
    let deliveries = store.list_deliveries().map_err(|err| err.to_string())?;
    let mut by_event: HashMap<String, Vec<(String, DeliveryStatus)>> = HashMap::new();
    for (event_id, subscriber_id, status) in deliveries {
        by_event
            .entry(event_id)
            .or_default()
            .push((subscriber_id, status));
    }
    let mut touched = 0usize;
    for event in events {
        let payload = event
            .ecst
            .get("payload")
            .cloned()
            .unwrap_or_else(|| json!({}));
        let plan = plan_route(&registry, codex.as_ref(), authority, &event.event_type);
        let planned: HashSet<String> = if plan.dead_letter_no_subscriber {
            HashSet::new()
        } else {
            applicable_ids(&plan, &payload).into_iter().collect()
        };
        let existing = by_event.get(&event.event_id).cloned().unwrap_or_default();
        if existing.is_empty() {
            for id in &planned {
                store
                    .upsert_delivery(&event.event_id, id, DeliveryStatus::Pending, json!({}))
                    .map_err(store_err)?;
                touched += 1;
            }
            continue;
        }
        for (subscriber_id, status) in existing {
            if status == DeliveryStatus::Pending && !planned.contains(&subscriber_id) {
                store
                    .set_delivery(
                        &event.event_id,
                        &subscriber_id,
                        DeliveryStatus::Skipped,
                        json!({"reconcile": "orphan"}),
                    )
                    .map_err(store_err)?;
                touched += 1;
            }
        }
    }
    Ok(touched)
}

fn store_err(err: StoreError) -> String {
    err.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subscriber_id_joins_agent_and_process() {
        let sub = json!({"agent": "tekton", "process": "tracker-stamp"});
        assert_eq!(subscriber_id(&sub), "tekton.tracker-stamp");
    }

    #[test]
    fn stamp_uses_catalog_not_emitter_keys() {
        let repo = tempfile_repo();
        write_registry(&repo, r#"{"PBI_Refined":[{"agent":"tekton","process":"tracker-stamp"}]}"#);
        let mut event = json!({
            "event_id": "e1",
            "event_type": "PBI_Refined",
            "payload": {},
            "delivery_state": {}
        });
        stamp_published_event(&repo, &mut event).unwrap();
        assert_eq!(
            event["delivery_state"]["tekton.tracker-stamp"],
            json!("pending")
        );
    }

    #[test]
    fn missing_registry_is_no_subscriber() {
        let path = std::env::temp_dir().join(format!(
            "eda-catalog-missing-{}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        let mut event = json!({
            "event_id": "e3",
            "event_type": "PBI_Refined",
            "delivery_state": {}
        });
        stamp_published_event(&path, &mut event).unwrap();
        assert_eq!(event["delivery_state"]["_disposition"], json!("no_subscriber"));
        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn unknown_event_is_no_subscriber() {
        let repo = tempfile_repo();
        write_registry(&repo, r#"{"PBI_Refined":[{"agent":"tekton","process":"tracker-stamp"}]}"#);
        let mut event = json!({
            "event_id": "e2",
            "event_type": "No_Such_Event",
            "delivery_state": {}
        });
        stamp_published_event(&repo, &mut event).unwrap();
        assert_eq!(event["delivery_state"]["_disposition"], json!("no_subscriber"));
    }

    fn tempfile_repo() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "eda-catalog-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(path.join("SddIA/core")).unwrap();
        path
    }

    fn write_registry(repo: &Path, body: &str) {
        fs::write(repo.join(CORE_REGISTRY), body).unwrap();
    }
}
