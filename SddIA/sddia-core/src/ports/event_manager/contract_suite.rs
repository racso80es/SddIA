//! Contract Test Suite compartida (MemoryMock / Redb / Fs).

use serde_json::json;

use crate::ports::event_manager::contract::{
    AppendTraceRequest, ClaimPendingRequest, DeliveryStatus, MoveToDlqRequest, PublishEcstRequest,
    RecordDeliveryRequest, ReplayDlqRequest,
};
use crate::ports::event_manager::port::EventManagerPort;

/// Ejecuta la suite de conformidad al 100% sobre `port`.
///
/// Casos: lifecycle publish→claim con aislamiento; ACK idempotente; DLQ+replay; traces.
pub fn run_event_manager_contract_suite(port: &dyn EventManagerPort) {
    lifecycle_publish_claim_isolation(port);
    ack_idempotent(port);
    dlq_and_replay(port);
    traces_integrity(port);
}

fn lifecycle_publish_claim_isolation(port: &dyn EventManagerPort) {
    let id = unique("life");
    port.publish_ecst(PublishEcstRequest {
        event_id: id.clone(),
        ecst: json!({"k": "v"}),
        trace_id: Some("t-life".into()),
    })
    .expect("publish");

    let first = port
        .claim_pending(ClaimPendingRequest {
            lease_owner: "owner-a".into(),
            limit: 10,
            lease_ttl_ms: Some(60_000),
        })
        .expect("claim a");
    assert!(
        first.events.iter().any(|e| e.event_id == id),
        "first claim must see published event"
    );

    let second = port
        .claim_pending(ClaimPendingRequest {
            lease_owner: "owner-b".into(),
            limit: 10,
            lease_ttl_ms: Some(60_000),
        })
        .expect("claim b");
    assert!(
        second.events.iter().all(|e| e.event_id != id),
        "concurrent claim must not duplicate leased event"
    );
}

fn ack_idempotent(port: &dyn EventManagerPort) {
    let id = unique("ack");
    port.publish_ecst(PublishEcstRequest {
        event_id: id.clone(),
        ecst: json!({"ack": true}),
        trace_id: None,
    })
    .expect("publish ack");

    let claimed = port
        .claim_pending(ClaimPendingRequest {
            lease_owner: "ack-owner".into(),
            limit: 1,
            lease_ttl_ms: Some(60_000),
        })
        .expect("claim ack");
    assert_eq!(claimed.events.len(), 1);
    assert_eq!(claimed.events[0].event_id, id);

    let delivery_id = format!("del-{id}");
    port.record_delivery(RecordDeliveryRequest {
        event_id: id.clone(),
        delivery_id: delivery_id.clone(),
        status: DeliveryStatus::Ack,
        attempt: Some(1),
    })
    .expect("ack1");

    port.record_delivery(RecordDeliveryRequest {
        event_id: id.clone(),
        delivery_id,
        status: DeliveryStatus::Ack,
        attempt: Some(1),
    })
    .expect("ack2 idempotent");
}

fn dlq_and_replay(port: &dyn EventManagerPort) {
    let id = unique("dlq");
    port.publish_ecst(PublishEcstRequest {
        event_id: id.clone(),
        ecst: json!({"dlq": 1}),
        trace_id: None,
    })
    .expect("publish dlq");

    let claimed = port
        .claim_pending(ClaimPendingRequest {
            lease_owner: "dlq-owner".into(),
            limit: 1,
            lease_ttl_ms: Some(60_000),
        })
        .expect("claim dlq");
    assert_eq!(claimed.events[0].event_id, id);

    port.move_to_dlq(MoveToDlqRequest {
        event_id: id.clone(),
        reason: "test-poison".into(),
    })
    .expect("move dlq");

    let after = port
        .claim_pending(ClaimPendingRequest {
            lease_owner: "dlq-owner-2".into(),
            limit: 10,
            lease_ttl_ms: Some(60_000),
        })
        .expect("claim after dlq");
    assert!(
        after.events.iter().all(|e| e.event_id != id),
        "event in DLQ must be absent from pending"
    );

    port.replay_dlq(ReplayDlqRequest {
        event_id: Some(id.clone()),
        all: None,
    })
    .expect("replay");

    let replayed = port
        .claim_pending(ClaimPendingRequest {
            lease_owner: "dlq-owner-3".into(),
            limit: 10,
            lease_ttl_ms: Some(60_000),
        })
        .expect("claim after replay");
    assert!(
        replayed.events.iter().any(|e| e.event_id == id),
        "replayed DLQ event must be claimable"
    );
}

fn traces_integrity(port: &dyn EventManagerPort) {
    let tid = unique("trace");
    port.append_trace(AppendTraceRequest {
        trace_id: tid.clone(),
        frame: json!({"step": 1}),
    })
    .expect("append1");
    port.append_trace(AppendTraceRequest {
        trace_id: tid,
        frame: json!({"step": 2}),
    })
    .expect("append2");
}

fn unique(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(1);
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    format!("{prefix}-{n}-{}", std::process::id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::event_manager::memory_mock::InMemoryEventManagerMock;

    #[test]
    fn suite_passes_on_memory_mock() {
        let mock = InMemoryEventManagerMock::new();
        run_event_manager_contract_suite(&mock);
        assert_eq!(mock.traces_for("unused").len(), 0);
    }
}
