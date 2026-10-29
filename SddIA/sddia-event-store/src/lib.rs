//! Store síncrono del bus. Una write-tx de `redb` por mutación.
//! El llamador inyecta la ruta; este crate no consulta `cumulo.paths.json`.

mod error;
mod gate;
mod model;

use std::collections::HashSet;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use redb::{Database, DatabaseError, ReadableTable, TableDefinition};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub use error::StoreError;
pub use gate::{
    append_progress_monotonic, copy_dlt_seal, delivery_status_from_label, load_dispatched_event,
    is_filesystem_rel, mode_from_name, override_store_mode, path_is_heartbeat_sidechannel,
    record_ecst,
    delivery_terminal_in_store, record_witness, runtime_db_path, store_mode, writes_filesystem,
    writes_store, StoreMode,
};
pub use model::{DeadLetter, DeliveryStatus, DeliveryWitness, DltSeal, Family, StoredEvent};

use error::{backend, table_missing};

const EVENTS: TableDefinition<&str, &[u8]> = TableDefinition::new("events");
const FAMILY_INDEX: TableDefinition<(&str, i64, &str), ()> = TableDefinition::new("family_index");
const SUBSCRIBER_DELIVERY: TableDefinition<(&str, &str), &[u8]> =
    TableDefinition::new("subscriber_delivery");
const DEAD_LETTER_INDEX: TableDefinition<&str, &[u8]> = TableDefinition::new("dead_letter_index");
const DLT_AUDIT_SEALS: TableDefinition<&str, &[u8]> = TableDefinition::new("dlt_audit_seals");
const PROGRESS_TRACES: TableDefinition<(&str, u64), &[u8]> =
    TableDefinition::new("progress_traces");

const DEFAULT_OPEN_BUDGET_MS: u64 = 2000;

fn open_budget_ms() -> u64 {
    match std::env::var("SDDIA_EVENT_STORE_OPEN_BUDGET_MS") {
        Ok(raw) => match raw.parse::<u64>() {
            Ok(n) if n > 0 => n,
            _ => DEFAULT_OPEN_BUDGET_MS,
        },
        Err(_) => DEFAULT_OPEN_BUDGET_MS,
    }
}

fn is_lock_contention(err: &DatabaseError) -> bool {
    matches!(err, DatabaseError::DatabaseAlreadyOpen)
        || err.to_string().contains("Database already open")
}

fn lock_backoff_ms(attempt: u32) -> u64 {
    let base = 5u64 << attempt.min(6);
    let jitter = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| (d.subsec_nanos() as u64) % 16)
        .unwrap_or(0)
        .saturating_add(1);
    base.saturating_add(jitter)
}

pub struct EventStore {
    db: Database,
}

impl EventStore {
    /// Abre el fichero o lo crea si no existe. No crea directorios padre.
    /// Ante `DatabaseAlreadyOpen` reintenta con backoff+jitter hasta
    /// `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` (default 2000). Agotado → `Locked`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = path.as_ref();
        let budget = Duration::from_millis(open_budget_ms());
        let start = Instant::now();
        let mut attempt: u32 = 0;
        let mut last_lock;
        loop {
            match Database::create(path) {
                Ok(db) => return Ok(Self { db }),
                Err(err) if is_lock_contention(&err) => {
                    last_lock = err.to_string();
                    let elapsed = start.elapsed();
                    if elapsed >= budget {
                        return Err(StoreError::Locked(last_lock));
                    }
                    let remaining = budget.saturating_sub(elapsed).as_millis() as u64;
                    let sleep_ms = lock_backoff_ms(attempt).min(remaining);
                    if sleep_ms == 0 {
                        return Err(StoreError::Locked(last_lock));
                    }
                    thread::sleep(Duration::from_millis(sleep_ms));
                    attempt = attempt.saturating_add(1);
                }
                Err(err) => return Err(backend(err)),
            }
        }
    }

    /// Evento, índice de familia y N filas `pending` en la misma write-tx.
    pub fn insert_event(
        &self,
        event: &StoredEvent,
        subscriber_ids: &[&str],
    ) -> Result<(), StoreError> {
        let event_id = required_token(&event.event_id, "event_id")?;
        let event_type = required_token(&event.event_type, "event_type")?;
        let subscribers = normalize_ids(subscriber_ids, "subscriber_id")?;
        let stored = StoredEvent {
            event_id: event_id.clone(),
            family: event.family,
            event_type,
            created_at: event.created_at,
            ecst: event.ecst.clone(),
        };
        let event_bytes = encode(&stored)?;
        let pending = encode(&DeliveryWitness {
            status: DeliveryStatus::Pending,
            witness: serde_json::json!({}),
        })?;
        let family = stored.family.as_str();
        let created_at = stored.created_at;

        self.commit_write(|txn| {
            {
                let mut events = txn.open_table(EVENTS).map_err(backend)?;
                if events.get(event_id.as_str()).map_err(backend)?.is_some() {
                    return Err(StoreError::Conflict(format!(
                        "event_id ya existe: {event_id}"
                    )));
                }
                let _ = events
                    .insert(event_id.as_str(), event_bytes.as_slice())
                    .map_err(backend)?;
            }
            {
                let mut index = txn.open_table(FAMILY_INDEX).map_err(backend)?;
                let _ = index
                    .insert((family, created_at, event_id.as_str()), ())
                    .map_err(backend)?;
            }
            {
                let mut deliveries = txn.open_table(SUBSCRIBER_DELIVERY).map_err(backend)?;
                for subscriber_id in &subscribers {
                    let _ = deliveries
                        .insert(
                            (event_id.as_str(), subscriber_id.as_str()),
                            pending.as_slice(),
                        )
                        .map_err(backend)?;
                }
            }
            Ok(())
        })
    }

    /// Actualiza un solo par `(event_id, subscriber_id)`. No crea suscriptores.
    pub fn set_delivery(
        &self,
        event_id: &str,
        subscriber_id: &str,
        status: DeliveryStatus,
        witness: serde_json::Value,
    ) -> Result<(), StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        let subscriber_id = required_token(subscriber_id, "subscriber_id")?;
        let bytes = encode(&DeliveryWitness { status, witness })?;
        self.commit_write(|txn| {
            let mut deliveries = txn.open_table(SUBSCRIBER_DELIVERY).map_err(backend)?;
            if deliveries
                .get((event_id.as_str(), subscriber_id.as_str()))
                .map_err(backend)?
                .is_none()
            {
                return Err(StoreError::NotFound(format!(
                    "sin testigo para {event_id}/{subscriber_id}"
                )));
            }
            let _ = deliveries
                .insert(
                    (event_id.as_str(), subscriber_id.as_str()),
                    bytes.as_slice(),
                )
                .map_err(backend)?;
            Ok(())
        })
    }

    /// Crea el par o sustituye el testigo. El evento tiene que existir.
    pub fn upsert_delivery(
        &self,
        event_id: &str,
        subscriber_id: &str,
        status: DeliveryStatus,
        witness: serde_json::Value,
    ) -> Result<(), StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        let subscriber_id = required_token(subscriber_id, "subscriber_id")?;
        let bytes = encode(&DeliveryWitness { status, witness })?;
        self.commit_write(|txn| {
            require_event(txn, &event_id)?;
            let mut deliveries = txn.open_table(SUBSCRIBER_DELIVERY).map_err(backend)?;
            let _ = deliveries
                .insert(
                    (event_id.as_str(), subscriber_id.as_str()),
                    bytes.as_slice(),
                )
                .map_err(backend)?;
            Ok(())
        })
    }

    pub fn get_delivery(
        &self,
        event_id: &str,
        subscriber_id: &str,
    ) -> Result<Option<DeliveryWitness>, StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        let subscriber_id = required_token(subscriber_id, "subscriber_id")?;
        self.read_tx(|txn| {
            let table = match txn.open_table(SUBSCRIBER_DELIVERY) {
                Ok(table) => table,
                Err(err) if table_missing(&err) => return Ok(None),
                Err(err) => return Err(backend(err)),
            };
            match table
                .get((event_id.as_str(), subscriber_id.as_str()))
                .map_err(backend)?
            {
                Some(guard) => Ok(Some(decode(guard.value())?)),
                None => Ok(None),
            }
        })
    }

    /// Pares `(event_id, subscriber_id)` en `Pending`. No devuelve el ECST.
    pub fn list_pending_deliveries(&self) -> Result<Vec<(String, String)>, StoreError> {
        self.read_tx(|txn| {
            let table = match txn.open_table(SUBSCRIBER_DELIVERY) {
                Ok(table) => table,
                Err(err) if table_missing(&err) => return Ok(Vec::new()),
                Err(err) => return Err(backend(err)),
            };
            let mut pending = Vec::new();
            let range = table.range::<&(&str, &str)>(..).map_err(backend)?;
            for row in range {
                let (key, value) = row.map_err(backend)?;
                let witness: DeliveryWitness = decode(value.value())?;
                if witness.status == DeliveryStatus::Pending {
                    let (event_id, subscriber_id) = key.value();
                    pending.push((event_id.to_string(), subscriber_id.to_string()));
                }
            }
            Ok(pending)
        })
    }

    /// Todas las filas de `subscriber_delivery`. No incluye el ECST.
    pub fn list_deliveries(&self) -> Result<Vec<(String, String, DeliveryStatus)>, StoreError> {
        self.read_tx(|txn| {
            let table = match txn.open_table(SUBSCRIBER_DELIVERY) {
                Ok(table) => table,
                Err(err) if table_missing(&err) => return Ok(Vec::new()),
                Err(err) => return Err(backend(err)),
            };
            let mut rows = Vec::new();
            let range = table.range::<&(&str, &str)>(..).map_err(backend)?;
            for row in range {
                let (key, value) = row.map_err(backend)?;
                let witness: DeliveryWitness = decode(value.value())?;
                let (event_id, subscriber_id) = key.value();
                rows.push((
                    event_id.to_string(),
                    subscriber_id.to_string(),
                    witness.status,
                ));
            }
            Ok(rows)
        })
    }

    pub fn seal_dlt(&self, event_id: &str, seal: &DltSeal) -> Result<(), StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        let tx_hash = required_token(&seal.tx_hash, "tx_hash")?;
        let digest = required_token(&seal.digest, "digest")?;
        let scope = required_token(&seal.scope, "scope")?;
        let bytes = encode(&DltSeal {
            tx_hash,
            digest,
            scope,
        })?;
        self.commit_write(|txn| {
            require_event(txn, &event_id)?;
            let mut seals = txn.open_table(DLT_AUDIT_SEALS).map_err(backend)?;
            let _ = seals
                .insert(event_id.as_str(), bytes.as_slice())
                .map_err(backend)?;
            Ok(())
        })
    }

    pub fn get_seal(&self, event_id: &str) -> Result<Option<DltSeal>, StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        self.read_tx(|txn| read_row(txn, DLT_AUDIT_SEALS, &event_id))
    }

    /// `seq` lo asigna el llamador. La misma pareja `(correlation_id, seq)` no se pisa.
    pub fn append_progress(
        &self,
        correlation_id: &str,
        seq: u64,
        frame: serde_json::Value,
    ) -> Result<(), StoreError> {
        let correlation_id = required_token(correlation_id, "correlation_id")?;
        let bytes = encode(&frame)?;
        self.commit_write(|txn| {
            let mut traces = txn.open_table(PROGRESS_TRACES).map_err(backend)?;
            if traces
                .get((correlation_id.as_str(), seq))
                .map_err(backend)?
                .is_some()
            {
                return Err(StoreError::Conflict(format!(
                    "progress ya existe: {correlation_id}#{seq}"
                )));
            }
            let _ = traces
                .insert((correlation_id.as_str(), seq), bytes.as_slice())
                .map_err(backend)?;
            Ok(())
        })
    }

    pub fn list_progress(
        &self,
        correlation_id: &str,
    ) -> Result<Vec<(u64, serde_json::Value)>, StoreError> {
        let correlation_id = required_token(correlation_id, "correlation_id")?;
        self.read_tx(|txn| {
            let table = match txn.open_table(PROGRESS_TRACES) {
                Ok(table) => table,
                Err(err) if table_missing(&err) => return Ok(Vec::new()),
                Err(err) => return Err(backend(err)),
            };
            let mut frames = Vec::new();
            let range = table
                .range((correlation_id.as_str(), u64::MIN)..=(correlation_id.as_str(), u64::MAX))
                .map_err(backend)?;
            for row in range {
                let (key, value) = row.map_err(backend)?;
                let (_correlation, seq) = key.value();
                frames.push((seq, decode(value.value())?));
            }
            Ok(frames)
        })
    }

    pub fn list_family(&self, family: Family) -> Result<Vec<StoredEvent>, StoreError> {
        let family_name = family.as_str();
        self.read_tx(|txn| {
            let index = match txn.open_table(FAMILY_INDEX) {
                Ok(table) => table,
                Err(err) if table_missing(&err) => return Ok(Vec::new()),
                Err(err) => return Err(backend(err)),
            };
            let events = match txn.open_table(EVENTS) {
                Ok(table) => table,
                Err(err) if table_missing(&err) => return Ok(Vec::new()),
                Err(err) => return Err(backend(err)),
            };
            let mut listed = Vec::new();
            let range = index
                .range((family_name, i64::MIN, "")..=(family_name, i64::MAX, "\u{10FFFF}"))
                .map_err(backend)?;
            for row in range {
                let (key, _empty) = row.map_err(backend)?;
                let (_family, _created_at, event_id) = key.value();
                let Some(guard) = events.get(event_id).map_err(backend)? else {
                    return Err(StoreError::Backend(format!(
                        "family_index sin evento: {event_id}"
                    )));
                };
                listed.push(decode(guard.value())?);
            }
            Ok(listed)
        })
    }

    pub fn get_event(&self, event_id: &str) -> Result<Option<StoredEvent>, StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        self.read_tx(|txn| read_row(txn, EVENTS, &event_id))
    }

    pub fn dead_letter(
        &self,
        event_id: &str,
        cause: &str,
        failed_subscriber_id: &str,
    ) -> Result<(), StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        let cause = required_token(cause, "cause")?;
        let failed_subscriber_id = required_token(failed_subscriber_id, "failed_subscriber_id")?;
        let bytes = encode(&DeadLetter {
            cause,
            failed_subscriber_id: failed_subscriber_id.clone(),
        })?;
        self.commit_write(|txn| {
            require_event(txn, &event_id)?;
            {
                let deliveries = txn.open_table(SUBSCRIBER_DELIVERY).map_err(backend)?;
                if deliveries
                    .get((event_id.as_str(), failed_subscriber_id.as_str()))
                    .map_err(backend)?
                    .is_none()
                {
                    return Err(StoreError::NotFound(format!(
                        "sin testigo para {event_id}/{failed_subscriber_id}"
                    )));
                }
            }
            let mut index = txn.open_table(DEAD_LETTER_INDEX).map_err(backend)?;
            let _ = index
                .insert(event_id.as_str(), bytes.as_slice())
                .map_err(backend)?;
            Ok(())
        })
    }

    pub fn get_dead_letter(&self, event_id: &str) -> Result<Option<DeadLetter>, StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        self.read_tx(|txn| read_row(txn, DEAD_LETTER_INDEX, &event_id))
    }

    pub fn list_dead_letters(&self) -> Result<Vec<(String, DeadLetter)>, StoreError> {
        self.read_tx(|txn| {
            let table = match txn.open_table(DEAD_LETTER_INDEX) {
                Ok(table) => table,
                Err(err) if table_missing(&err) => return Ok(Vec::new()),
                Err(err) => return Err(backend(err)),
            };
            let mut rows = Vec::new();
            let range = table.range::<&str>(..).map_err(backend)?;
            for row in range {
                let (key, value) = row.map_err(backend)?;
                let letter: DeadLetter = decode(value.value())?;
                rows.push((key.value().to_string(), letter));
            }
            Ok(rows)
        })
    }

    /// Borra `dead_letter_index` y deja el testigo del suscriptor fallido en `pending`.
    pub fn replay(&self, event_id: &str) -> Result<(), StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        self.commit_write(|txn| {
            let letter: DeadLetter = {
                let index = txn.open_table(DEAD_LETTER_INDEX).map_err(backend)?;
                let Some(guard) = index.get(event_id.as_str()).map_err(backend)? else {
                    return Err(StoreError::NotFound(format!("sin dead letter: {event_id}")));
                };
                decode(guard.value())?
            };
            let witness = {
                let deliveries = txn.open_table(SUBSCRIBER_DELIVERY).map_err(backend)?;
                let guard = deliveries
                    .get((event_id.as_str(), letter.failed_subscriber_id.as_str()))
                    .map_err(backend)?;
                let witness = match &guard {
                    Some(row) => {
                        let current: DeliveryWitness = decode(row.value())?;
                        current.witness
                    }
                    None => serde_json::json!({}),
                };
                witness
            };
            let pending = encode(&DeliveryWitness {
                status: DeliveryStatus::Pending,
                witness,
            })?;
            {
                let mut deliveries = txn.open_table(SUBSCRIBER_DELIVERY).map_err(backend)?;
                let _ = deliveries
                    .insert(
                        (event_id.as_str(), letter.failed_subscriber_id.as_str()),
                        pending.as_slice(),
                    )
                    .map_err(backend)?;
            }
            {
                let mut index = txn.open_table(DEAD_LETTER_INDEX).map_err(backend)?;
                let _ = index.remove(event_id.as_str()).map_err(backend)?;
            }
            Ok(())
        })
    }

    /// Quita el evento, sus testigos y su dead-letter. `false` si no había fila.
    pub fn purge_event(&self, event_id: &str) -> Result<bool, StoreError> {
        let event_id = required_token(event_id, "event_id")?;
        let mut removed = false;
        self.commit_write(|txn| {
            if let Ok(mut events) = txn.open_table(EVENTS) {
                if events.remove(event_id.as_str()).map_err(backend)?.is_some() {
                    removed = true;
                }
            }
            if let Ok(mut index) = txn.open_table(FAMILY_INDEX) {
                let keys: Vec<(String, i64, String)> = {
                    let range = index
                        .range::<&(&str, i64, &str)>(..)
                        .map_err(backend)?;
                    let mut found = Vec::new();
                    for row in range {
                        let (key, _) = row.map_err(backend)?;
                        let (family, created_at, id) = key.value();
                        if id == event_id {
                            found.push((family.to_string(), created_at, id.to_string()));
                        }
                    }
                    found
                };
                for (family, created_at, id) in keys {
                    let _ = index
                        .remove((family.as_str(), created_at, id.as_str()))
                        .map_err(backend)?;
                    removed = true;
                }
            }
            if let Ok(mut deliveries) = txn.open_table(SUBSCRIBER_DELIVERY) {
                let keys: Vec<(String, String)> = {
                    let range = deliveries.range::<&(&str, &str)>(..).map_err(backend)?;
                    let mut found = Vec::new();
                    for row in range {
                        let (key, _) = row.map_err(backend)?;
                        let (id, subscriber) = key.value();
                        if id == event_id {
                            found.push((id.to_string(), subscriber.to_string()));
                        }
                    }
                    found
                };
                for (id, subscriber) in keys {
                    let _ = deliveries
                        .remove((id.as_str(), subscriber.as_str()))
                        .map_err(backend)?;
                    removed = true;
                }
            }
            if let Ok(mut letters) = txn.open_table(DEAD_LETTER_INDEX) {
                if letters.remove(event_id.as_str()).map_err(backend)?.is_some() {
                    removed = true;
                }
            }
            Ok(())
        })?;
        Ok(removed)
    }

    fn commit_write(
        &self,
        body: impl FnOnce(&redb::WriteTransaction) -> Result<(), StoreError>,
    ) -> Result<(), StoreError> {
        let txn = self.db.begin_write().map_err(backend)?;
        body(&txn)?;
        txn.commit().map_err(backend)?;
        Ok(())
    }

    fn read_tx<T>(
        &self,
        body: impl FnOnce(&redb::ReadTransaction) -> Result<T, StoreError>,
    ) -> Result<T, StoreError> {
        let txn = self.db.begin_read().map_err(backend)?;
        body(&txn)
    }
}

fn require_event(txn: &redb::WriteTransaction, event_id: &str) -> Result<(), StoreError> {
    let events = match txn.open_table(EVENTS) {
        Ok(table) => table,
        Err(err) if table_missing(&err) => {
            return Err(StoreError::NotFound(format!("sin evento: {event_id}")));
        }
        Err(err) => return Err(backend(err)),
    };
    if events.get(event_id).map_err(backend)?.is_none() {
        return Err(StoreError::NotFound(format!("sin evento: {event_id}")));
    }
    Ok(())
}

fn read_row<T: DeserializeOwned>(
    txn: &redb::ReadTransaction,
    definition: TableDefinition<&str, &[u8]>,
    event_id: &str,
) -> Result<Option<T>, StoreError> {
    let table = match txn.open_table(definition) {
        Ok(table) => table,
        Err(err) if table_missing(&err) => return Ok(None),
        Err(err) => return Err(backend(err)),
    };
    match table.get(event_id).map_err(backend)? {
        Some(guard) => Ok(Some(decode(guard.value())?)),
        None => Ok(None),
    }
}

fn required_token(raw: &str, field: &str) -> Result<String, StoreError> {
    let token = raw.trim();
    if token.is_empty() {
        return Err(StoreError::Invalid(format!("{field} vacío")));
    }
    Ok(token.to_string())
}

fn normalize_ids(ids: &[&str], field: &str) -> Result<Vec<String>, StoreError> {
    let mut seen = HashSet::new();
    let mut out = Vec::with_capacity(ids.len());
    for id in ids {
        let token = required_token(id, field)?;
        if !seen.insert(token.clone()) {
            return Err(StoreError::Invalid(format!("{field} duplicado: {token}")));
        }
        out.push(token);
    }
    Ok(out)
}

fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, StoreError> {
    serde_json::to_vec(value).map_err(|err| StoreError::Backend(err.to_string()))
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, StoreError> {
    serde_json::from_slice(bytes)
        .map_err(|err| StoreError::Backend(format!("valor ilegible: {err}")))
}
