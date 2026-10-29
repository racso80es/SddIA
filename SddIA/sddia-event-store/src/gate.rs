//! Corte `SDDIA_EVENT_STORE`. El crate no lee `cumulo.paths.json`.
//! El llamador pasa el repo; la base vive en `.SddIA/runtime/events.redb`.

use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use crate::{DeliveryStatus, DltSeal, EventStore, Family, StoreError, StoredEvent};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreMode {
    Filesystem,
    Dual,
    Redb,
}

thread_local! {
    static MODE_OVERRIDE: Cell<Option<StoreMode>> = const { Cell::new(None) };
}

pub fn mode_from_name(name: Option<&str>) -> StoreMode {
    match name.map(str::trim).filter(|s| !s.is_empty()) {
        Some("filesystem") => StoreMode::Filesystem,
        Some("dual") => StoreMode::Dual,
        _ => StoreMode::Redb,
    }
}

/// Ruta de directorio o fichero. Un nombre de tabla o `*.redb` no lo es.
pub fn is_filesystem_rel(raw: &str) -> bool {
    let s = raw.trim();
    !s.is_empty()
        && !s.ends_with(".redb")
        && (s.contains('/') || s.contains('\\') || s.starts_with('.'))
}

pub fn store_mode() -> StoreMode {
    MODE_OVERRIDE
        .with(|slot| slot.get())
        .unwrap_or_else(|| mode_from_name(std::env::var("SDDIA_EVENT_STORE").ok().as_deref()))
}

/// Sustituye la variable de entorno en este hilo. `None` vuelve al entorno.
pub fn override_store_mode(mode: Option<StoreMode>) {
    MODE_OVERRIDE.with(|slot| slot.set(mode));
}

pub fn writes_filesystem(mode: StoreMode) -> bool {
    matches!(mode, StoreMode::Filesystem | StoreMode::Dual)
}

pub fn writes_store(mode: StoreMode) -> bool {
    matches!(mode, StoreMode::Dual | StoreMode::Redb)
}

pub fn runtime_db_path(repo: &Path) -> PathBuf {
    repo.join(".SddIA/runtime/events.redb")
}

pub fn path_is_heartbeat_sidechannel(path: &Path) -> bool {
    path.to_string_lossy()
        .replace('\\', "/")
        .contains("/daemons/state/heartbeats/")
}

pub fn delivery_status_from_label(status: &str) -> DeliveryStatus {
    let label = status.trim().to_ascii_lowercase();
    if label == "success" || label == "processed" {
        DeliveryStatus::Success
    } else if label == "failed" || label == "dead-letter" || label == "dead_letter" {
        DeliveryStatus::Failed
    } else if label == "skipped" || label.starts_with("skipped") {
        DeliveryStatus::Skipped
    } else {
        DeliveryStatus::Pending
    }
}

pub fn record_ecst(repo: &Path, family: &str, event: &Value) -> Result<(), StoreError> {
    if !writes_store(store_mode()) {
        return Ok(());
    }
    let family = parse_family(family)?;
    let event_id = event_token(event, "event_id")?;
    let event_type = event
        .get("event_type")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("unknown");
    let entries = delivery_entries(family, event)?;
    let ids: Vec<String> = entries.iter().map(|(id, _)| id.clone()).collect();
    let refs: Vec<&str> = ids.iter().map(String::as_str).collect();
    let stored = StoredEvent {
        event_id: event_id.clone(),
        family,
        event_type: event_type.to_string(),
        created_at: now_millis(),
        ecst: event.clone(),
    };
    let store = open_store(repo)?;
    match store.insert_event(&stored, &refs) {
        Err(StoreError::Conflict(_)) => Ok(()),
        Err(err) => Err(err),
        Ok(()) => {
            for (id, status) in &entries {
                if *status != DeliveryStatus::Pending {
                    store.set_delivery(&event_id, id, *status, serde_json::json!({}))?;
                }
            }
            Ok(())
        }
    }
}

pub fn record_witness(
    repo: &Path,
    event_id: &str,
    subscriber_id: &str,
    status: DeliveryStatus,
    witness: &Value,
) -> Result<(), StoreError> {
    if !writes_store(store_mode()) {
        return Ok(());
    }
    let store = open_store(repo)?;
    store.upsert_delivery(event_id, subscriber_id, status, witness.clone())
}

/// Entrega terminal en `subscriber_delivery` (Success, Failed o Skipped).
pub fn delivery_terminal_in_store(
    repo: &Path,
    event_id: &str,
    subscriber_id: &str,
) -> Result<bool, StoreError> {
    if !writes_store(store_mode()) {
        return Ok(false);
    }
    let store = open_store(repo)?;
    Ok(match store.get_delivery(event_id, subscriber_id)? {
        Some(w) => w.status != DeliveryStatus::Pending,
        None => false,
    })
}

pub fn append_progress_monotonic(
    repo: &Path,
    correlation_id: &str,
    frame: &Value,
) -> Result<u64, StoreError> {
    if !writes_store(store_mode()) {
        return Ok(0);
    }
    let store = open_store(repo)?;
    let seq = next_seq(&store, correlation_id)?;
    match store.append_progress(correlation_id, seq, frame.clone()) {
        Ok(()) => Ok(seq),
        Err(StoreError::Conflict(_)) => {
            let seq = next_seq(&store, correlation_id)?;
            store.append_progress(correlation_id, seq, frame.clone())?;
            Ok(seq)
        }
        Err(err) => Err(err),
    }
}

pub fn copy_dlt_seal(repo: &Path, event_id: &str, tx_hash: &str) -> Result<(), StoreError> {
    if !writes_store(store_mode()) {
        return Ok(());
    }
    let tx_hash = tx_hash.trim();
    if tx_hash.is_empty() {
        return Ok(());
    }
    let store = open_store(repo)?;
    store.seal_dlt(
        event_id,
        &DltSeal {
            tx_hash: tx_hash.to_string(),
            digest: tx_hash.to_string(),
            scope: "dlt-telemetry".to_string(),
        },
    )
}

/// `redb` lee solo el store. `dual` usa la fila si existe y, si no, el fichero.
pub fn load_dispatched_event(
    repo: &Path,
    event_id: &str,
    file_path: &Path,
) -> Result<Option<Value>, StoreError> {
    match store_mode() {
        StoreMode::Filesystem => read_event_file(file_path),
        StoreMode::Redb => {
            let store = open_store(repo)?;
            Ok(store.get_event(event_id)?.map(|event| event.ecst))
        }
        StoreMode::Dual => {
            let store = open_store(repo)?;
            if let Some(event) = store.get_event(event_id)? {
                return Ok(Some(event.ecst));
            }
            read_event_file(file_path)
        }
    }
}

fn open_store(repo: &Path) -> Result<EventStore, StoreError> {
    let path = runtime_db_path(repo);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| StoreError::Backend(err.to_string()))?;
    }
    EventStore::open(path)
}

fn next_seq(store: &EventStore, correlation_id: &str) -> Result<u64, StoreError> {
    let frames = store.list_progress(correlation_id)?;
    Ok(frames
        .last()
        .map(|(seq, _)| seq.saturating_add(1))
        .unwrap_or(0))
}

fn read_event_file(path: &Path) -> Result<Option<Value>, StoreError> {
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(path).map_err(|err| StoreError::Backend(err.to_string()))?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|err| StoreError::Invalid(format!("json de evento ilegible: {err}")))
}

fn parse_family(family: &str) -> Result<Family, StoreError> {
    match family.trim().to_ascii_lowercase().as_str() {
        "telemetry" => Ok(Family::Telemetry),
        "orchestration" => Ok(Family::Orchestration),
        "domain" | "pending" | "processing" | "processed" | "dead-letter" | "dead_letter" => {
            Ok(Family::Domain)
        }
        other => Err(StoreError::Invalid(format!("familia inválida: {other}"))),
    }
}

fn event_token(event: &Value, field: &str) -> Result<String, StoreError> {
    event
        .get(field)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| StoreError::Invalid(format!("{field} vacío")))
}

const DELIVERY_META_KEYS: &[&str] = &["last_batch_anchor_error", "transaction_digest"];

fn delivery_entries(
    family: Family,
    event: &Value,
) -> Result<Vec<(String, DeliveryStatus)>, StoreError> {
    let map = event.get("delivery_state").and_then(|v| v.as_object());
    let disposition = map
        .and_then(|state| state.get("_disposition"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mut entries = Vec::new();
    if let Some(map) = map {
        for (key, value) in map {
            let token = key.trim();
            if token.is_empty() || token.starts_with('_') || DELIVERY_META_KEYS.contains(&token) {
                continue;
            }
            let label = value.as_str().unwrap_or("pending");
            entries.push((token.to_string(), delivery_status_from_label(label)));
        }
    }
    if family == Family::Domain && entries.is_empty() && disposition != "no_subscriber" {
        return Err(StoreError::Invalid("EDA_DELIVERY_PLAN_MISSING".into()));
    }
    Ok(entries)
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis() as i64)
        .unwrap_or(0)
}
