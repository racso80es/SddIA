//! RedbEventAdapter — persistencia ACID para `eda.event_manager` v1.0.0.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sddia_core::ports::event_manager::{
    AppendTraceRequest, ClaimPendingRequest, ClaimPendingResult, ClaimedEvent, DeliveryStatus,
    DualVersionManifest, EventManagerError, EventManagerPort, GetDltSealRequest, GetDltSealResult,
    GetEventRequest, GetEventResult, HandshakeVersionRequest, HandshakeVersionResult,
    ListedDelivery, ListedDlqEvent, ListDeliveriesRequest, ListDeliveriesResult, ListDlqRequest,
    ListDlqResult, ListPendingRequest, ListPendingResult, ListedPendingEvent, ListTracesRequest,
    ListTracesResult, MoveToDlqRequest, PublishEcstRequest, PublishEcstResult, PurgeEventRequest,
    RecordDeliveryRequest, ReplayDlqRequest, SealDltRequest, CONTRACT_ID, CONTRACT_VERSION,
};
use sddia_core::ports::event_manager::version_gate;

const EVENTS: TableDefinition<&str, &[u8]> = TableDefinition::new("em_events");
const LEASES: TableDefinition<&str, &[u8]> = TableDefinition::new("em_leases");
const DELIVERIES: TableDefinition<&str, &[u8]> = TableDefinition::new("em_deliveries");
const DLQ: TableDefinition<&str, &[u8]> = TableDefinition::new("em_dlq");
const TRACES: TableDefinition<&str, &[u8]> = TableDefinition::new("em_traces");
const META: TableDefinition<&str, &[u8]> = TableDefinition::new("em_meta");

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredEvent {
    event_id: String,
    ecst: Value,
    trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LeaseRec {
    event_id: String,
    owner: String,
    until_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DlqRec {
    event: StoredEvent,
    reason: String,
}

pub struct RedbEventAdapter {
    db: Database,
    /// Reloj monotónico auxiliar para TTL de leases en proceso.
    clock: Mutex<()>,
}

impl RedbEventAdapter {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, EventManagerError> {
        let db = Database::create(path.as_ref()).map_err(|e| {
            EventManagerError::Storage(format!("redb open: {e}"))
        })?;
        let adapter = Self {
            db,
            clock: Mutex::new(()),
        };
        adapter.ensure_schema()?;
        adapter.seal_manifest()?;
        Ok(adapter)
    }

    pub fn manifest(&self) -> DualVersionManifest {
        DualVersionManifest::builtin_v1("1.0.0")
    }

    fn ensure_schema(&self) -> Result<(), EventManagerError> {
        let txn = self
            .db
            .begin_write()
            .map_err(|e| EventManagerError::Storage(e.to_string()))?;
        {
            let _ = txn.open_table(EVENTS).map_err(storage)?;
            let _ = txn.open_table(LEASES).map_err(storage)?;
            let _ = txn.open_table(DELIVERIES).map_err(storage)?;
            let _ = txn.open_table(DLQ).map_err(storage)?;
            let _ = txn.open_table(TRACES).map_err(storage)?;
            let _ = txn.open_table(META).map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    fn seal_manifest(&self) -> Result<(), EventManagerError> {
        let bytes = serde_json::to_vec(&self.manifest())
            .map_err(|e| EventManagerError::Storage(e.to_string()))?;
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let mut meta = txn.open_table(META).map_err(storage)?;
            meta.insert("manifest", bytes.as_slice()).map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }

    fn expire_leases(&self) -> Result<(), EventManagerError> {
        let _guard = self.clock.lock().expect("clock");
        let now = Self::now_ms();
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let mut leases = txn.open_table(LEASES).map_err(storage)?;
            let expired: Vec<String> = {
                let mut out = Vec::new();
                let iter = leases.iter().map_err(storage)?;
                for item in iter {
                    let (k, v) = item.map_err(storage)?;
                    let rec: LeaseRec = serde_json::from_slice(v.value()).map_err(json_err)?;
                    if rec.until_ms <= now {
                        out.push(k.value().to_string());
                    }
                }
                out
            };
            for id in expired {
                let _ = leases.remove(id.as_str()).map_err(storage)?;
            }
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    pub fn load_traces(&self, trace_id: &str) -> Result<Vec<Value>, EventManagerError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let table = txn.open_table(TRACES).map_err(storage)?;
        match table.get(trace_id).map_err(storage)? {
            Some(v) => serde_json::from_slice(v.value()).map_err(json_err),
            None => Ok(Vec::new()),
        }
    }
}

fn storage<E: std::fmt::Display>(e: E) -> EventManagerError {
    EventManagerError::Storage(e.to_string())
}

fn json_err<E: std::fmt::Display>(e: E) -> EventManagerError {
    EventManagerError::Storage(format!("json: {e}"))
}

impl EventManagerPort for RedbEventAdapter {
    fn publish_ecst(
        &self,
        req: PublishEcstRequest,
    ) -> Result<PublishEcstResult, EventManagerError> {
        let stored = StoredEvent {
            event_id: req.event_id.clone(),
            ecst: req.ecst,
            trace_id: req.trace_id,
        };
        let bytes = serde_json::to_vec(&stored).map_err(json_err)?;
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let events = txn.open_table(EVENTS).map_err(storage)?;
            if events.get(req.event_id.as_str()).map_err(storage)?.is_some() {
                return Err(EventManagerError::InvalidRequest(format!(
                    "duplicate {}",
                    req.event_id
                )));
            }
            let dlq = txn.open_table(DLQ).map_err(storage)?;
            if dlq.get(req.event_id.as_str()).map_err(storage)?.is_some() {
                return Err(EventManagerError::InvalidRequest(format!(
                    "duplicate in dlq {}",
                    req.event_id
                )));
            }
            drop(dlq);
            drop(events);
            let mut events = txn.open_table(EVENTS).map_err(storage)?;
            events
                .insert(req.event_id.as_str(), bytes.as_slice())
                .map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(PublishEcstResult {
            event_id: req.event_id,
        })
    }

    fn claim_pending(
        &self,
        req: ClaimPendingRequest,
    ) -> Result<ClaimPendingResult, EventManagerError> {
        self.expire_leases()?;
        let now = Self::now_ms();
        let ttl = req.lease_ttl_ms.unwrap_or(30_000);
        let txn = self.db.begin_write().map_err(storage)?;
        let mut taken = Vec::new();
        {
            let events = txn.open_table(EVENTS).map_err(storage)?;
            let leases = txn.open_table(LEASES).map_err(storage)?;
            let mut candidates = Vec::new();
            for item in events.iter().map_err(storage)? {
                let (k, v) = item.map_err(storage)?;
                let id = k.value().to_string();
                if leases.get(id.as_str()).map_err(storage)?.is_some() {
                    continue;
                }
                let stored: StoredEvent = serde_json::from_slice(v.value()).map_err(json_err)?;
                candidates.push(stored);
                if candidates.len() as u32 >= req.limit {
                    break;
                }
            }
            drop(leases);
            drop(events);
            let mut leases = txn.open_table(LEASES).map_err(storage)?;
            for ev in candidates {
                let lease = LeaseRec {
                    event_id: ev.event_id.clone(),
                    owner: req.lease_owner.clone(),
                    until_ms: now.saturating_add(ttl),
                };
                let lb = serde_json::to_vec(&lease).map_err(json_err)?;
                leases
                    .insert(ev.event_id.as_str(), lb.as_slice())
                    .map_err(storage)?;
                taken.push(ClaimedEvent {
                    event_id: ev.event_id,
                    ecst: ev.ecst,
                    trace_id: ev.trace_id,
                    lease_owner: req.lease_owner.clone(),
                });
            }
        }
        txn.commit().map_err(storage)?;
        let _ = Instant::now() + Duration::from_millis(0);
        Ok(ClaimPendingResult { events: taken })
    }

    fn record_delivery(&self, req: RecordDeliveryRequest) -> Result<(), EventManagerError> {
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let mut deliveries = txn.open_table(DELIVERIES).map_err(storage)?;
            if let Some(prev) = deliveries.get(req.delivery_id.as_str()).map_err(storage)? {
                let prev_status: DeliveryStatus =
                    serde_json::from_slice(prev.value()).map_err(json_err)?;
                if prev_status == DeliveryStatus::Ack && req.status == DeliveryStatus::Ack {
                    return Ok(());
                }
            }
            let bytes = serde_json::to_vec(&req.status).map_err(json_err)?;
            deliveries
                .insert(req.delivery_id.as_str(), bytes.as_slice())
                .map_err(storage)?;
            drop(deliveries);

            match req.status {
                DeliveryStatus::Ack => {
                    let mut leases = txn.open_table(LEASES).map_err(storage)?;
                    let _ = leases.remove(req.event_id.as_str()).map_err(storage)?;
                    drop(leases);
                    let mut events = txn.open_table(EVENTS).map_err(storage)?;
                    let _ = events.remove(req.event_id.as_str()).map_err(storage)?;
                }
                DeliveryStatus::Nack => {
                    let mut leases = txn.open_table(LEASES).map_err(storage)?;
                    let _ = leases.remove(req.event_id.as_str()).map_err(storage)?;
                }
            }
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    fn move_to_dlq(&self, req: MoveToDlqRequest) -> Result<(), EventManagerError> {
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let mut events = txn.open_table(EVENTS).map_err(storage)?;
            let raw = events
                .remove(req.event_id.as_str())
                .map_err(storage)?
                .ok_or_else(|| EventManagerError::NotFound(req.event_id.clone()))?;
            let raw_bytes = raw.value().to_vec();
            drop(raw);
            let event: StoredEvent = serde_json::from_slice(&raw_bytes).map_err(json_err)?;
            drop(events);
            let mut leases = txn.open_table(LEASES).map_err(storage)?;
            let _ = leases.remove(req.event_id.as_str()).map_err(storage)?;
            drop(leases);
            let rec = DlqRec {
                event,
                reason: req.reason,
            };
            let bytes = serde_json::to_vec(&rec).map_err(json_err)?;
            let mut dlq = txn.open_table(DLQ).map_err(storage)?;
            dlq.insert(req.event_id.as_str(), bytes.as_slice())
                .map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    fn replay_dlq(&self, req: ReplayDlqRequest) -> Result<(), EventManagerError> {
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let mut dlq = txn.open_table(DLQ).map_err(storage)?;
            let ids: Vec<String> = if req.all.unwrap_or(false) {
                let mut out = Vec::new();
                for item in dlq.iter().map_err(storage)? {
                    let (k, _) = item.map_err(storage)?;
                    out.push(k.value().to_string());
                }
                out
            } else if let Some(id) = req.event_id.clone() {
                vec![id]
            } else {
                return Err(EventManagerError::InvalidRequest(
                    "replay_dlq requires event_id or all=true".into(),
                ));
            };

            let mut restored = BTreeMap::new();
            for id in ids {
                let raw = dlq
                    .remove(id.as_str())
                    .map_err(storage)?
                    .ok_or_else(|| EventManagerError::NotFound(id.clone()))?;
                let rec: DlqRec = serde_json::from_slice(raw.value()).map_err(json_err)?;
                restored.insert(id, rec.event);
            }
            drop(dlq);
            let mut events = txn.open_table(EVENTS).map_err(storage)?;
            for (id, ev) in restored {
                let bytes = serde_json::to_vec(&ev).map_err(json_err)?;
                events
                    .insert(id.as_str(), bytes.as_slice())
                    .map_err(storage)?;
            }
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    fn append_trace(&self, req: AppendTraceRequest) -> Result<(), EventManagerError> {
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let mut traces = txn.open_table(TRACES).map_err(storage)?;
            let mut frames: Vec<Value> = match traces.get(req.trace_id.as_str()).map_err(storage)? {
                Some(v) => serde_json::from_slice(v.value()).map_err(json_err)?,
                None => Vec::new(),
            };
            frames.push(req.frame);
            let bytes = serde_json::to_vec(&frames).map_err(json_err)?;
            traces
                .insert(req.trace_id.as_str(), bytes.as_slice())
                .map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    fn handshake_version(
        &self,
        req: HandshakeVersionRequest,
    ) -> Result<HandshakeVersionResult, EventManagerError> {
        version_gate(&req.requested_range, CONTRACT_VERSION)?;
        Ok(HandshakeVersionResult {
            contract: CONTRACT_ID.to_string(),
            contract_version: CONTRACT_VERSION.to_string(),
            compatible: true,
        })
    }

    fn get_event(&self, req: GetEventRequest) -> Result<GetEventResult, EventManagerError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let events = txn.open_table(EVENTS).map_err(storage)?;
        let leases = txn.open_table(LEASES).map_err(storage)?;
        if let Some(v) = events.get(req.event_id.as_str()).map_err(storage)? {
            let stored: StoredEvent = serde_json::from_slice(v.value()).map_err(json_err)?;
            return Ok(GetEventResult {
                event_id: stored.event_id,
                ecst: stored.ecst,
                trace_id: stored.trace_id,
            });
        }
        if let Some(v) = txn.open_table(DLQ).map_err(storage)?.get(req.event_id.as_str()).map_err(storage)? {
            let rec: DlqRec = serde_json::from_slice(v.value()).map_err(json_err)?;
            return Ok(GetEventResult {
                event_id: rec.event.event_id,
                ecst: rec.event.ecst,
                trace_id: rec.event.trace_id,
            });
        }
        let _ = leases;
        Err(EventManagerError::NotFound(req.event_id))
    }

    fn list_pending(
        &self,
        req: ListPendingRequest,
    ) -> Result<ListPendingResult, EventManagerError> {
        self.expire_leases()?;
        let limit = req.limit.unwrap_or(u32::MAX) as usize;
        let txn = self.db.begin_read().map_err(storage)?;
        let events = txn.open_table(EVENTS).map_err(storage)?;
        let leases = txn.open_table(LEASES).map_err(storage)?;
        let mut out = Vec::new();
        for item in events.iter().map_err(storage)? {
            if out.len() >= limit {
                break;
            }
            let (k, v) = item.map_err(storage)?;
            let id = k.value();
            if leases.get(id).map_err(storage)?.is_some() {
                continue;
            }
            let stored: StoredEvent = serde_json::from_slice(v.value()).map_err(json_err)?;
            out.push(ListedPendingEvent {
                event_id: stored.event_id,
                ecst: stored.ecst,
                trace_id: stored.trace_id,
            });
        }
        Ok(ListPendingResult { events: out })
    }

    fn list_dlq(&self, req: ListDlqRequest) -> Result<ListDlqResult, EventManagerError> {
        let limit = req.limit.unwrap_or(u32::MAX) as usize;
        let txn = self.db.begin_read().map_err(storage)?;
        let dlq = txn.open_table(DLQ).map_err(storage)?;
        let mut out = Vec::new();
        for item in dlq.iter().map_err(storage)? {
            if out.len() >= limit {
                break;
            }
            let (k, v) = item.map_err(storage)?;
            let rec: DlqRec = serde_json::from_slice(v.value()).map_err(json_err)?;
            out.push(ListedDlqEvent {
                event_id: k.value().to_string(),
                reason: rec.reason,
                ecst: rec.event.ecst,
            });
        }
        Ok(ListDlqResult { events: out })
    }

    fn list_traces(&self, req: ListTracesRequest) -> Result<ListTracesResult, EventManagerError> {
        let frames = self.load_traces(&req.trace_id)?;
        Ok(ListTracesResult { frames })
    }

    fn purge_event(&self, req: PurgeEventRequest) -> Result<(), EventManagerError> {
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let mut events = txn.open_table(EVENTS).map_err(storage)?;
            let mut leases = txn.open_table(LEASES).map_err(storage)?;
            let mut dlq = txn.open_table(DLQ).map_err(storage)?;
            let mut traces = txn.open_table(TRACES).map_err(storage)?;
            let mut deliveries = txn.open_table(DELIVERIES).map_err(storage)?;
            let removed_event = events.remove(req.event_id.as_str()).map_err(storage)?.is_some();
            let removed_dlq = dlq.remove(req.event_id.as_str()).map_err(storage)?.is_some();
            if !removed_event && !removed_dlq {
                return Err(EventManagerError::NotFound(req.event_id.clone()));
            }
            let _ = leases.remove(req.event_id.as_str()).map_err(storage)?;
            let prefix = format!("{}:", req.event_id);
            let delivery_keys: Vec<String> = deliveries
                .iter()
                .map_err(storage)?
                .filter_map(|item| {
                    item.ok().and_then(|(k, _)| {
                        let key = k.value().to_string();
                        if key.starts_with(&prefix) || key.contains(req.event_id.as_str()) {
                            Some(key)
                        } else {
                            None
                        }
                    })
                })
                .collect();
            for key in delivery_keys {
                let _ = deliveries.remove(key.as_str()).map_err(storage)?;
            }
            let trace_keys: Vec<String> = traces
                .iter()
                .map_err(storage)?
                .filter_map(|item| item.ok().map(|(k, _)| k.value().to_string()))
                .filter(|tid| tid.contains(req.event_id.as_str()))
                .collect();
            for key in trace_keys {
                let _ = traces.remove(key.as_str()).map_err(storage)?;
            }
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    fn list_deliveries(
        &self,
        req: ListDeliveriesRequest,
    ) -> Result<ListDeliveriesResult, EventManagerError> {
        let limit = req.limit.unwrap_or(100) as usize;
        let txn = self.db.begin_read().map_err(storage)?;
        let deliveries = txn.open_table(DELIVERIES).map_err(storage)?;
        let mut out = Vec::new();
        for item in deliveries.iter().map_err(storage)? {
            if out.len() >= limit {
                break;
            }
            let (k, v) = item.map_err(storage)?;
            let delivery_id = k.value().to_string();
            let status: DeliveryStatus = serde_json::from_slice(v.value()).map_err(json_err)?;
            if let Some(ref eid) = req.event_id {
                if !delivery_id.contains(eid.as_str()) {
                    continue;
                }
            }
            if let Some(ref sub) = req.subscriber_id {
                if !delivery_id.contains(sub.as_str()) {
                    continue;
                }
            }
            if let Some(expected) = req.status {
                if status != expected {
                    continue;
                }
            }
            let event_id = req
                .event_id
                .clone()
                .or_else(|| delivery_id.split(':').next().map(str::to_string))
                .unwrap_or_else(|| delivery_id.clone());
            out.push(ListedDelivery {
                delivery_id,
                event_id,
                status,
            });
        }
        Ok(ListDeliveriesResult {
            deliveries: out,
            next_cursor: None,
        })
    }

    fn seal_dlt(&self, req: SealDltRequest) -> Result<(), EventManagerError> {
        let record = GetDltSealResult {
            event_id: req.event_id.clone(),
            tx_hash: req.tx_hash,
            digest: req.digest,
            scope: req.scope,
        };
        let key = format!("dlt_seal:{}", req.event_id);
        let bytes = serde_json::to_vec(&record).map_err(json_err)?;
        let txn = self.db.begin_write().map_err(storage)?;
        {
            let mut meta = txn.open_table(META).map_err(storage)?;
            meta.insert(key.as_str(), bytes.as_slice())
                .map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(())
    }

    fn get_dlt_seal(
        &self,
        req: GetDltSealRequest,
    ) -> Result<GetDltSealResult, EventManagerError> {
        let key = format!("dlt_seal:{}", req.event_id);
        let txn = self.db.begin_read().map_err(storage)?;
        let meta = txn.open_table(META).map_err(storage)?;
        let Some(v) = meta.get(key.as_str()).map_err(storage)? else {
            return Err(EventManagerError::NotFound(req.event_id));
        };
        serde_json::from_slice(v.value()).map_err(json_err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sddia_core::ports::event_manager::run_event_manager_contract_suite;
    use tempfile::tempdir;

    #[test]
    fn contract_suite_on_redb_tempdir() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("em.redb");
        {
            let adapter = RedbEventAdapter::open(&path).expect("open");
            run_event_manager_contract_suite(&adapter);
        }
        // reopen integrity for traces written during suite
        let adapter2 = RedbEventAdapter::open(&path).expect("reopen");
        let _ = adapter2.manifest();
    }
}
