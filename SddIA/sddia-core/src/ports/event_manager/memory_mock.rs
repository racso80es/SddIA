//! Mock in-memory con fault injection (rol `test`).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::ports::event_manager::contract::{
    AppendTraceRequest, ClaimPendingRequest, ClaimPendingResult, ClaimedEvent, CONTRACT_ID,
    CONTRACT_VERSION, DeliveryStatus, GetEventRequest, GetEventResult, HandshakeVersionRequest,
    HandshakeVersionResult, ListedDlqEvent, ListDlqRequest, ListDlqResult, ListPendingRequest,
    ListPendingResult, ListedPendingEvent, ListTracesRequest, ListTracesResult, MoveToDlqRequest,
    PublishEcstRequest, PublishEcstResult, PurgeEventRequest, RecordDeliveryRequest, ReplayDlqRequest,
};
use crate::ports::event_manager::error::EventManagerError;
use crate::ports::event_manager::port::EventManagerPort;
use crate::ports::event_manager::version_gate::version_gate;

#[derive(Debug, Clone, Default)]
pub struct FaultInjection {
    pub fail_write: bool,
    pub steal_lease: bool,
    pub duplicate_ack_noop: bool,
}

#[derive(Debug, Clone)]
struct StoredEvent {
    event_id: String,
    ecst: Value,
    trace_id: Option<String>,
    lease_owner: Option<String>,
    lease_until: Option<Instant>,
    dlq_reason: Option<String>,
}

#[derive(Debug, Default)]
struct Inner {
    pending: BTreeMap<String, StoredEvent>,
    claimed: BTreeMap<String, StoredEvent>,
    dlq: BTreeMap<String, StoredEvent>,
    deliveries: HashMap<String, DeliveryStatus>,
    traces: HashMap<String, Vec<Value>>,
    faults: FaultInjection,
}

#[derive(Debug, Default)]
pub struct InMemoryEventManagerMock {
    inner: Mutex<Inner>,
}

impl InMemoryEventManagerMock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_faults(faults: FaultInjection) -> Self {
        Self {
            inner: Mutex::new(Inner {
                faults,
                ..Inner::default()
            }),
        }
    }

    pub fn set_faults(&self, faults: FaultInjection) {
        let mut g = self.inner.lock().expect("mock lock");
        g.faults = faults;
    }

    pub fn traces_for(&self, trace_id: &str) -> Vec<Value> {
        let g = self.inner.lock().expect("mock lock");
        g.traces.get(trace_id).cloned().unwrap_or_default()
    }
}

impl EventManagerPort for InMemoryEventManagerMock {
    fn publish_ecst(
        &self,
        req: PublishEcstRequest,
    ) -> Result<PublishEcstResult, EventManagerError> {
        let mut g = self.inner.lock().expect("mock lock");
        if g.faults.fail_write {
            return Err(EventManagerError::InjectedFault("fail_write".into()));
        }
        if g.pending.contains_key(&req.event_id)
            || g.claimed.contains_key(&req.event_id)
            || g.dlq.contains_key(&req.event_id)
        {
            return Err(EventManagerError::InvalidRequest(format!(
                "duplicate event_id {}",
                req.event_id
            )));
        }
        g.pending.insert(
            req.event_id.clone(),
            StoredEvent {
                event_id: req.event_id.clone(),
                ecst: req.ecst,
                trace_id: req.trace_id,
                lease_owner: None,
                lease_until: None,
                dlq_reason: None,
            },
        );
        Ok(PublishEcstResult {
            event_id: req.event_id,
        })
    }

    fn claim_pending(
        &self,
        req: ClaimPendingRequest,
    ) -> Result<ClaimPendingResult, EventManagerError> {
        let mut g = self.inner.lock().expect("mock lock");
        let now = Instant::now();
        let ttl = Duration::from_millis(req.lease_ttl_ms.unwrap_or(30_000));

        // Expire leases → back to pending.
        let expired: Vec<String> = g
            .claimed
            .iter()
            .filter(|(_, e)| e.lease_until.map(|u| u <= now).unwrap_or(true))
            .map(|(id, _)| id.clone())
            .collect();
        for id in expired {
            if let Some(mut ev) = g.claimed.remove(&id) {
                ev.lease_owner = None;
                ev.lease_until = None;
                g.pending.insert(id, ev);
            }
        }

        if g.faults.steal_lease {
            // Force-clear first claimed lease to simulate steal path in tests.
            if let Some(id) = g.claimed.keys().next().cloned() {
                if let Some(mut ev) = g.claimed.remove(&id) {
                    ev.lease_owner = None;
                    ev.lease_until = None;
                    g.pending.insert(id, ev);
                }
            }
        }

        let mut taken = Vec::new();
        let keys: Vec<String> = g.pending.keys().cloned().collect();
        for id in keys {
            if taken.len() as u32 >= req.limit {
                break;
            }
            if let Some(mut ev) = g.pending.remove(&id) {
                ev.lease_owner = Some(req.lease_owner.clone());
                ev.lease_until = Some(now + ttl);
                taken.push(ClaimedEvent {
                    event_id: ev.event_id.clone(),
                    ecst: ev.ecst.clone(),
                    trace_id: ev.trace_id.clone(),
                    lease_owner: req.lease_owner.clone(),
                });
                g.claimed.insert(id, ev);
            }
        }
        Ok(ClaimPendingResult { events: taken })
    }

    fn record_delivery(&self, req: RecordDeliveryRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("mock lock");
        if let Some(prev) = g.deliveries.get(&req.delivery_id) {
            if *prev == DeliveryStatus::Ack && req.status == DeliveryStatus::Ack {
                // Idempotent ACK.
                if g.faults.duplicate_ack_noop || true {
                    return Ok(());
                }
            }
        }
        g.deliveries.insert(req.delivery_id.clone(), req.status);

        match req.status {
            DeliveryStatus::Ack => {
                g.claimed.remove(&req.event_id);
                g.pending.remove(&req.event_id);
            }
            DeliveryStatus::Nack => {
                if let Some(mut ev) = g.claimed.remove(&req.event_id) {
                    ev.lease_owner = None;
                    ev.lease_until = None;
                    g.pending.insert(req.event_id, ev);
                }
            }
        }
        Ok(())
    }

    fn move_to_dlq(&self, req: MoveToDlqRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("mock lock");
        let mut ev = g
            .claimed
            .remove(&req.event_id)
            .or_else(|| g.pending.remove(&req.event_id))
            .ok_or_else(|| EventManagerError::NotFound(req.event_id.clone()))?;
        ev.dlq_reason = Some(req.reason);
        ev.lease_owner = None;
        ev.lease_until = None;
        g.dlq.insert(req.event_id, ev);
        Ok(())
    }

    fn replay_dlq(&self, req: ReplayDlqRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("mock lock");
        let ids: Vec<String> = if req.all.unwrap_or(false) {
            g.dlq.keys().cloned().collect()
        } else if let Some(id) = req.event_id {
            vec![id]
        } else {
            return Err(EventManagerError::InvalidRequest(
                "replay_dlq requires event_id or all=true".into(),
            ));
        };
        for id in ids {
            let mut ev = g
                .dlq
                .remove(&id)
                .ok_or_else(|| EventManagerError::NotFound(id.clone()))?;
            ev.dlq_reason = None;
            g.pending.insert(id, ev);
        }
        Ok(())
    }

    fn append_trace(&self, req: AppendTraceRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("mock lock");
        g.traces
            .entry(req.trace_id)
            .or_default()
            .push(req.frame);
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
        let g = self.inner.lock().expect("mock lock");
        let ev = g
            .pending
            .get(&req.event_id)
            .or_else(|| g.claimed.get(&req.event_id))
            .ok_or_else(|| EventManagerError::NotFound(req.event_id.clone()))?;
        Ok(GetEventResult {
            event_id: ev.event_id.clone(),
            ecst: ev.ecst.clone(),
            trace_id: ev.trace_id.clone(),
        })
    }

    fn list_pending(&self, req: ListPendingRequest) -> Result<ListPendingResult, EventManagerError> {
        let g = self.inner.lock().expect("mock lock");
        let limit = req.limit.unwrap_or(u32::MAX) as usize;
        let mut events = Vec::new();
        for (_, ev) in g.pending.iter().take(limit) {
            events.push(ListedPendingEvent {
                event_id: ev.event_id.clone(),
                ecst: ev.ecst.clone(),
                trace_id: ev.trace_id.clone(),
            });
        }
        Ok(ListPendingResult { events })
    }

    fn list_dlq(&self, req: ListDlqRequest) -> Result<ListDlqResult, EventManagerError> {
        let g = self.inner.lock().expect("mock lock");
        let limit = req.limit.unwrap_or(u32::MAX) as usize;
        let mut events = Vec::new();
        for (id, ev) in g.dlq.iter().take(limit) {
            events.push(ListedDlqEvent {
                event_id: id.clone(),
                reason: ev
                    .dlq_reason
                    .clone()
                    .unwrap_or_else(|| "unknown".into()),
                ecst: ev.ecst.clone(),
            });
        }
        Ok(ListDlqResult { events })
    }

    fn list_traces(&self, req: ListTracesRequest) -> Result<ListTracesResult, EventManagerError> {
        let g = self.inner.lock().expect("mock lock");
        let frames = g.traces.get(&req.trace_id).cloned().unwrap_or_default();
        Ok(ListTracesResult { frames })
    }

    fn purge_event(&self, req: PurgeEventRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("mock lock");
        let trace_id = g
            .pending
            .get(&req.event_id)
            .or_else(|| g.claimed.get(&req.event_id))
            .or_else(|| g.dlq.get(&req.event_id))
            .and_then(|ev| ev.trace_id.clone());
        let removed = g.pending.remove(&req.event_id).is_some()
            || g.claimed.remove(&req.event_id).is_some()
            || g.dlq.remove(&req.event_id).is_some();
        if !removed {
            return Err(EventManagerError::NotFound(req.event_id));
        }
        g.deliveries
            .retain(|k, _| !k.contains(req.event_id.as_str()));
        if let Some(tid) = trace_id {
            g.traces.remove(&tid);
        }
        Ok(())
    }
}

/// Helper para asserts de suite: conjunto de event_ids reclamados.
pub fn claimed_ids(result: &ClaimPendingResult) -> BTreeSet<String> {
    result.events.iter().map(|e| e.event_id.clone()).collect()
}
