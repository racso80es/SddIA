//! Contract Test Suite compartida (MemoryMock / Redb / Fs).

use serde_json::json;

use crate::ports::event_manager::{
    AppendTraceRequest, ClaimPendingRequest, DeliveryStatus, EventFamily, EventQueueState,
    GetDltSealRequest, GetEventRequest, ListDeliveriesRequest, ListEventsRequest,
    ListTerminalEventsRequest, ListTracesRequest, MoveToDlqRequest,
    PurgeEventRequest, RecordDeliveryRequest, ReplayDlqRequest, SealDltRequest,
    VerifyInlineEcstRequest, publish_request_domain, publish_request_family,
};
use crate::ports::event_manager::error::EventManagerError;
use crate::ports::event_manager::port::EventManagerPort;

/// Ejecuta la suite de conformidad al 100% sobre `port`.
///
/// Casos: lifecycle publish→claim con aislamiento; ACK idempotente; DLQ+replay; traces.
pub fn run_event_manager_contract_suite(port: &dyn EventManagerPort) {
    lifecycle_publish_claim_isolation(port);
    ack_idempotent(port);
    dlq_and_replay(port);
    traces_integrity(port);
    verify_inline_and_dlt_seal(port);
    o0_containment_integrity(port);
    o1_listing_integrity(port);
}

fn lifecycle_publish_claim_isolation(port: &dyn EventManagerPort) {
    let id = unique("life");
    port.publish_ecst(publish_request_domain(
        id.clone(),
        json!({"k": "v"}),
        Some("t-life".into()),
    ))
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
    port.publish_ecst(publish_request_domain(
        id.clone(),
        json!({"ack": true}),
        None,
    ))
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
        subscriber_id: None,
        attempt: Some(1),
    })
    .expect("ack1");

    port.record_delivery(RecordDeliveryRequest {
        event_id: id.clone(),
        delivery_id,
        status: DeliveryStatus::Ack,
        subscriber_id: None,
        attempt: Some(1),
    })
    .expect("ack2 idempotent");
}

fn dlq_and_replay(port: &dyn EventManagerPort) {
    let id = unique("dlq");
    port.publish_ecst(publish_request_domain(id.clone(), json!({"dlq": 1}), None))
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

fn verify_inline_and_dlt_seal(port: &dyn EventManagerPort) {
    let id = unique("inline");
    let ecst = json!({"event_id": id, "event_type": "T", "k": 1});
    port.publish_ecst(publish_request_domain(id.clone(), ecst.clone(), None))
    .expect("publish inline");
    port.verify_inline_ecst(VerifyInlineEcstRequest {
        event_id: id.clone(),
        ecst: ecst.clone(),
    })
    .expect("verify ok");
    port.seal_dlt(SealDltRequest {
        event_id: id.clone(),
        tx_hash: "0xabc".into(),
        digest: "d".into(),
        scope: "test".into(),
    })
    .expect("seal");
    let seal = port
        .get_dlt_seal(GetDltSealRequest {
            event_id: id.clone(),
        })
        .expect("get seal");
    assert_eq!(seal.tx_hash, "0xabc");
}

fn o0_containment_integrity(port: &dyn EventManagerPort) {
    let tag = unique("o0");
    let abc = format!("abc-{tag}");
    let xabcx = format!("xabcx-{tag}");
    let trace_abc = format!("trace-abc-{tag}");
    let trace_x = format!("xabcx-log-{tag}");

    port.publish_ecst(publish_request_domain(
        abc.clone(),
        json!({"k": "abc"}),
        Some(trace_abc.clone()),
    ))
    .expect("publish abc");
    port.publish_ecst(publish_request_domain(
        xabcx.clone(),
        json!({"k": "xabcx"}),
        Some(trace_x.clone()),
    ))
    .expect("publish xabcx");

    port.append_trace(AppendTraceRequest {
        trace_id: trace_x.clone(),
        frame: json!({"step": 1}),
    })
    .expect("append x trace");

    let dup = port.publish_ecst(publish_request_domain(abc.clone(), json!({"k": "dup"}), None));
    assert!(
        matches!(dup, Err(EventManagerError::AlreadyExists { .. })),
        "duplicate publish must be AlreadyExists"
    );

    port.record_delivery(RecordDeliveryRequest {
        event_id: abc.clone(),
        delivery_id: format!("del-{abc}-sub"),
        status: DeliveryStatus::Nack,
        subscriber_id: Some("sub".into()),
        attempt: Some(1),
    })
    .expect("del abc");
    port.record_delivery(RecordDeliveryRequest {
        event_id: xabcx.clone(),
        delivery_id: format!("del-{xabcx}-sub"),
        status: DeliveryStatus::Nack,
        subscriber_id: Some("sub".into()),
        attempt: Some(1),
    })
    .expect("del xabcx");
    port.record_delivery(RecordDeliveryRequest {
        event_id: abc.clone(),
        delivery_id: format!("del-{abc}-sub2"),
        status: DeliveryStatus::Nack,
        subscriber_id: Some("sub-2".into()),
        attempt: Some(1),
    })
    .expect("del abc sub2");

    let listed_abc = port
        .list_deliveries(ListDeliveriesRequest {
            event_id: Some(abc.clone()),
            subscriber_id: None,
            status: None,
            limit: None,
            cursor: None,
        })
        .expect("list abc");
    assert_eq!(
        listed_abc.deliveries.len(),
        2,
        "abc must have exactly its deliveries"
    );

    let listed_x = port
        .list_deliveries(ListDeliveriesRequest {
            event_id: Some(xabcx.clone()),
            subscriber_id: Some("sub".into()),
            status: None,
            limit: None,
            cursor: None,
        })
        .expect("list xabcx");
    assert_eq!(
        listed_x.deliveries.len(),
        1,
        "xabcx filter by event_id and subscriber must be exact"
    );

    port.purge_event(PurgeEventRequest {
        event_id: abc.clone(),
    })
    .expect("purge abc");

    assert!(
        port
            .get_event(GetEventRequest {
                event_id: xabcx.clone(),
            })
            .is_ok(),
        "xabcx must survive purge of abc"
    );

    let after_abc = port
        .list_deliveries(ListDeliveriesRequest {
            event_id: Some(abc.clone()),
            subscriber_id: None,
            status: None,
            limit: None,
            cursor: None,
        })
        .expect("list abc after purge");
    assert!(after_abc.deliveries.is_empty());

    let after_x = port
        .list_deliveries(ListDeliveriesRequest {
            event_id: Some(xabcx.clone()),
            subscriber_id: None,
            status: None,
            limit: None,
            cursor: None,
        })
        .expect("list xabcx after purge");
    assert_eq!(after_x.deliveries.len(), 1);

    let frames = port
        .list_traces(ListTracesRequest {
            trace_id: trace_x.clone(),
        })
        .expect("list x trace");
    assert!(
        !frames.frames.is_empty(),
        "trace xabcx-log must survive purge of abc"
    );
}

fn o1_listing_integrity(port: &dyn EventManagerPort) {
    let tag = unique("o1");
    let et = "SharedType";
    let dom_id = format!("dom-{tag}");
    let tel_id = format!("tel-{tag}");
    port.publish_ecst(publish_request_family(
        EventFamily::Domain,
        dom_id.clone(),
        json!({"event_type": et, "v": 1}),
        None,
    ))
    .expect("publish domain");
    port.publish_ecst(publish_request_family(
        EventFamily::Telemetry,
        tel_id.clone(),
        json!({"event_type": et, "v": 2}),
        None,
    ))
    .expect("publish telemetry");

    let dom_only = port
        .list_events(ListEventsRequest {
            family: Some("domain".into()),
            event_type: Some(et.into()),
            state: None,
            limit: None,
            cursor: None,
        })
        .expect("list domain");
    assert_eq!(dom_only.events.len(), 1);
    assert_eq!(dom_only.events[0].event_id, dom_id);

    let bad_family = port.list_events(ListEventsRequest {
        family: Some("pending".into()),
        event_type: None,
        state: None,
        limit: None,
        cursor: None,
    });
    assert!(
        matches!(
            bad_family,
            Err(EventManagerError::InvalidRequest(_))
                | Err(EventManagerError::ExternalCapsule(_))
        ),
        "pending is queue state, not family"
    );

    let flow_id = format!("flow-{tag}");
    port.publish_ecst(publish_request_domain(
        flow_id.clone(),
        json!({"event_type": "Flow"}),
        None,
    ))
    .expect("publish flow");
    let batch = port
        .claim_pending(ClaimPendingRequest {
            lease_owner: "o1-owner".into(),
            limit: 100,
            lease_ttl_ms: Some(60_000),
        })
        .expect("claim flow");
    assert!(
        batch.events.iter().any(|e| e.event_id == flow_id),
        "flow event must be claimed"
    );
    let claimed = port
        .list_events(ListEventsRequest {
            family: None,
            event_type: None,
            state: Some(EventQueueState::Claimed),
            limit: None,
            cursor: None,
        })
        .expect("list claimed");
    assert!(claimed.events.iter().any(|e| e.event_id == flow_id));

    port.record_delivery(RecordDeliveryRequest {
        event_id: flow_id.clone(),
        delivery_id: format!("del-{flow_id}"),
        status: DeliveryStatus::Ack,
        subscriber_id: None,
        attempt: Some(1),
    })
    .expect("ack flow");
    let processed = port
        .list_events(ListEventsRequest {
            family: None,
            event_type: None,
            state: Some(EventQueueState::Processed),
            limit: None,
            cursor: None,
        })
        .expect("list processed");
    assert!(processed.events.iter().any(|e| e.event_id == flow_id));

    let dlq_id = format!("dlq-{tag}");
    port.publish_ecst(publish_request_domain(dlq_id.clone(), json!({"d": 1}), None))
        .expect("publish dlq");
    let dlq_claim = port
        .claim_pending(ClaimPendingRequest {
            lease_owner: "o1-dlq".into(),
            limit: 100,
            lease_ttl_ms: Some(60_000),
        })
        .expect("claim dlq");
    assert!(dlq_claim.events.iter().any(|e| e.event_id == dlq_id));
    port.move_to_dlq(MoveToDlqRequest {
        event_id: dlq_id.clone(),
        reason: "o1".into(),
    })
    .expect("to dlq");

    let terminals = port
        .list_terminal_events(ListTerminalEventsRequest {
            family: None,
            limit: None,
            cursor: None,
        })
        .expect("terminals");
    assert!(terminals.events.iter().any(|e| e.event_id == flow_id));
    assert!(terminals.events.iter().any(|e| e.event_id == dlq_id));
    assert!(!terminals.events.iter().any(|e| e.event_id == dom_id));

    let p0 = format!("page-a-{tag}");
    let p1 = format!("page-b-{tag}");
    let p2 = format!("page-c-{tag}");
    for id in [&p0, &p1, &p2] {
        port.publish_ecst(publish_request_domain(
            id.clone(),
            json!({"event_type": "PageMarker", "p": id}),
            None,
        ))
        .expect("page publish");
    }
    let first_page = port
        .list_events(ListEventsRequest {
            family: None,
            event_type: Some("PageMarker".into()),
            state: Some(EventQueueState::Pending),
            limit: Some(1),
            cursor: None,
        })
        .expect("page1");
    assert_eq!(first_page.events.len(), 1);
    let cursor = first_page
        .next_cursor
        .clone()
        .expect("cursor after page1");
    let second_page = port
        .list_events(ListEventsRequest {
            family: None,
            event_type: Some("PageMarker".into()),
            state: Some(EventQueueState::Pending),
            limit: Some(1),
            cursor: Some(cursor),
        })
        .expect("page2");
    assert_eq!(second_page.events.len(), 1);
    let third_cursor = second_page.next_cursor.clone().expect("cursor page2");
    let third_page = port
        .list_events(ListEventsRequest {
            family: None,
            event_type: Some("PageMarker".into()),
            state: Some(EventQueueState::Pending),
            limit: Some(1),
            cursor: Some(third_cursor),
        })
        .expect("page3");
    let ids: Vec<String> = first_page
        .events
        .iter()
        .chain(second_page.events.iter())
        .chain(third_page.events.iter())
        .map(|e| e.event_id.clone())
        .collect();
    assert!(ids.contains(&p0));
    assert!(ids.contains(&p1));
    assert!(ids.contains(&p2));
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
