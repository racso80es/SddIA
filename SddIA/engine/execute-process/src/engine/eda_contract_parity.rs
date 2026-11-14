//! Oráculo de paridad `eda.event_manager.v1.schema.json` ↔ `EventManagerPort` (sddia-core).
//!
//! El schema es la forma de cable del puerto: cada petición que serializan los tipos de
//! `sddia-core` (`{operation, payload}`, igual que `ExternalCapsuleEventAdapter::rpc`) debe
//! validar; su variante con un campo obligatorio ausente o un valor fuera de contrato debe fallar;
//! y el conjunto de operaciones del schema debe ser exactamente el del trait.

use jsonschema::Validator;
use regex::Regex;
use sddia_core::ports::event_manager::{
    AppendTraceRequest, ClaimPendingRequest, DeliveryStatus, EventFamily, EventQueueState,
    EventUri, GetDltSealRequest, GetEventRequest, HandshakeVersionRequest, ListDeliveriesRequest,
    ListDlqRequest, ListEventsRequest, ListPendingRequest, ListProgressRequest,
    ListTerminalEventsRequest, ListTracesRequest, MoveToDlqRequest, PublishEcstRequest,
    PurgeEventRequest, RecordDeliveryRequest, ReplayDlqRequest, SealDltRequest,
    VerifyInlineEcstRequest,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::PathBuf;

const PORT_SRC: &str = include_str!("../../../../sddia-core/src/ports/event_manager/port.rs");

/// `SDDIA_PARITY_SCHEMA` permite ensayar un parche contra una copia antes de aplicarlo al repo.
fn schema() -> Value {
    let path = std::env::var("SDDIA_PARITY_SCHEMA").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../library/norms/capability-contracts/eda.event_manager.v1.schema.json")
    });
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).expect("schema JSON")
}

fn validator() -> Validator {
    Validator::new(&schema()).expect("el schema debe compilar")
}

fn wire<T: Serialize>(op: &str, req: &T) -> Value {
    json!({"operation": op, "payload": serde_json::to_value(req).expect("serialize")})
}

fn uri() -> EventUri {
    EventUri::new(EventFamily::Domain, "evt-1").expect("uri")
}

/// Peticiones mínimas válidas, construidas con los tipos de `sddia-core` (verdad del cable).
fn valid_requests() -> Vec<(&'static str, Value)> {
    let mut v: Vec<(&'static str, Value)> = vec![
        (
            "publish_ecst",
            wire(
                "publish_ecst",
                &PublishEcstRequest::new(uri(), json!({"event_id": "evt-1"}), Some("t-1".into())),
            ),
        ),
        (
            "publish_ecst",
            wire(
                "publish_ecst",
                &PublishEcstRequest::new(
                    EventUri::new(EventFamily::Telemetry, "evt-2").unwrap(),
                    json!({"event_id": "evt-2"}),
                    None,
                ),
            ),
        ),
        (
            "claim_pending",
            wire(
                "claim_pending",
                &ClaimPendingRequest { lease_owner: "w".into(), limit: 1, lease_ttl_ms: Some(1000) },
            ),
        ),
        (
            "move_to_dlq",
            wire("move_to_dlq", &MoveToDlqRequest { event_id: "e".into(), reason: "r".into() }),
        ),
        ("replay_dlq", wire("replay_dlq", &ReplayDlqRequest { event_id: Some("e".into()), all: None })),
        ("replay_dlq", wire("replay_dlq", &ReplayDlqRequest { event_id: None, all: Some(true) })),
        (
            "append_trace",
            wire("append_trace", &AppendTraceRequest { trace_id: "t".into(), frame: json!({"k": 1}) }),
        ),
        (
            "handshake_version",
            wire("handshake_version", &HandshakeVersionRequest { requested_range: "^2.0.0".into() }),
        ),
        ("get_event", wire("get_event", &GetEventRequest { event_id: "e".into() })),
        ("list_pending", wire("list_pending", &ListPendingRequest { limit: Some(5) })),
        ("list_dlq", wire("list_dlq", &ListDlqRequest { limit: Some(5) })),
        ("list_traces", wire("list_traces", &ListTracesRequest { trace_id: "t".into() })),
        ("purge_event", wire("purge_event", &PurgeEventRequest { event_id: "e".into() })),
        (
            "verify_inline_ecst",
            wire(
                "verify_inline_ecst",
                &VerifyInlineEcstRequest { event_id: "e".into(), ecst: json!({"a": 1}) },
            ),
        ),
        (
            "list_events",
            wire(
                "list_events",
                &ListEventsRequest {
                    family: Some("domain".into()),
                    event_type: Some("Domain_Entity_Created".into()),
                    state: Some(EventQueueState::DeadLetter),
                    limit: Some(10),
                    cursor: Some("c".into()),
                },
            ),
        ),
        // El puerto trata family vacía/blanca como «sin filtro» (parse_family_filter).
        (
            "list_events",
            wire(
                "list_events",
                &ListEventsRequest {
                    family: Some(String::new()),
                    event_type: None,
                    state: None,
                    limit: None,
                    cursor: None,
                },
            ),
        ),
        (
            "list_deliveries",
            wire(
                "list_deliveries",
                &ListDeliveriesRequest {
                    event_id: Some("e".into()),
                    subscriber_id: Some("s".into()),
                    status: Some(DeliveryStatus::Processing),
                    limit: Some(10),
                    cursor: Some("c".into()),
                },
            ),
        ),
        (
            "list_terminal_events",
            wire(
                "list_terminal_events",
                &ListTerminalEventsRequest {
                    family: Some("orchestration".into()),
                    limit: Some(10),
                    cursor: Some("c".into()),
                },
            ),
        ),
        (
            "list_progress",
            wire(
                "list_progress",
                &ListProgressRequest { trace_id: Some("t".into()), correlation_id: None },
            ),
        ),
        (
            "list_progress",
            wire(
                "list_progress",
                &ListProgressRequest { trace_id: None, correlation_id: Some("c".into()) },
            ),
        ),
        (
            "seal_dlt",
            wire(
                "seal_dlt",
                &SealDltRequest {
                    event_id: "e".into(),
                    tx_hash: "0xabc".into(),
                    digest: "d".into(),
                    scope: "core".into(),
                },
            ),
        ),
        ("get_dlt_seal", wire("get_dlt_seal", &GetDltSealRequest { event_id: "e".into() })),
    ];
    // Todos los estados de entrega que acepta el tipo (`ack`, `nack`, `processing`).
    for status in [DeliveryStatus::Ack, DeliveryStatus::Nack, DeliveryStatus::Processing] {
        v.push((
            "record_delivery",
            wire(
                "record_delivery",
                &RecordDeliveryRequest {
                    event_id: "e".into(),
                    delivery_id: "d".into(),
                    status,
                    subscriber_id: Some("s".into()),
                    attempt: Some(1),
                },
            ),
        ));
    }
    v
}

enum Break {
    /// Elimina un campo del payload de la petición válida de la operación.
    Remove(&'static str),
    /// Fija un campo del payload a un valor fuera de contrato.
    Set(&'static str, Value),
    /// Sustituye el payload entero.
    Payload(Value),
}

fn broken(op: &str, how: &Break) -> Value {
    let base = valid_requests()
        .into_iter()
        .find(|(o, _)| *o == op)
        .unwrap_or_else(|| panic!("sin petición válida para {op}"))
        .1;
    let mut req = base;
    let payload = req.get_mut("payload").and_then(|p| p.as_object_mut()).expect("payload objeto");
    match how {
        Break::Remove(k) => {
            assert!(payload.remove(*k).is_some(), "{op}: la petición válida no trae {k}");
        }
        Break::Set(k, v) => {
            payload.insert((*k).to_string(), v.clone());
        }
        Break::Payload(v) => {
            *payload = v.as_object().cloned().expect("objeto");
        }
    }
    req
}

fn invalid_cases() -> Vec<(&'static str, &'static str, Break)> {
    vec![
        ("publish_ecst", "sin event_uri (identidad canónica L-1)", Break::Remove("event_uri")),
        ("publish_ecst", "sin event_id", Break::Remove("event_id")),
        ("publish_ecst", "sin ecst", Break::Remove("ecst")),
        (
            "publish_ecst",
            "familia = estado de cola",
            Break::Set("event_uri", json!({"family": "pending", "event_id": "evt-1"})),
        ),
        (
            "publish_ecst",
            "event_uri con separador en event_id",
            Break::Set("event_uri", json!({"family": "domain", "event_id": "a/b"})),
        ),
        (
            "publish_ecst",
            "event_uri en forma de cadena (EventUri no la deserializa)",
            Break::Set("event_uri", json!("eda://domain/evt-1")),
        ),
        ("claim_pending", "sin lease_owner", Break::Remove("lease_owner")),
        ("claim_pending", "limit 0", Break::Set("limit", json!(0))),
        ("record_delivery", "sin delivery_id", Break::Remove("delivery_id")),
        ("record_delivery", "status desconocido", Break::Set("status", json!("weird"))),
        ("move_to_dlq", "sin reason", Break::Remove("reason")),
        ("replay_dlq", "all no booleano", Break::Set("all", json!("yes"))),
        ("append_trace", "sin frame", Break::Remove("frame")),
        ("handshake_version", "sin requested_range", Break::Remove("requested_range")),
        ("get_event", "sin event_id", Break::Remove("event_id")),
        ("list_pending", "limit 0", Break::Set("limit", json!(0))),
        ("list_dlq", "limit 0", Break::Set("limit", json!(0))),
        ("list_traces", "sin trace_id", Break::Remove("trace_id")),
        ("purge_event", "sin event_id", Break::Remove("event_id")),
        ("verify_inline_ecst", "sin ecst", Break::Remove("ecst")),
        ("verify_inline_ecst", "sin event_id", Break::Remove("event_id")),
        ("list_events", "limit 0", Break::Set("limit", json!(0))),
        ("list_events", "state desconocido", Break::Set("state", json!("bogus"))),
        ("list_events", "family desconocida", Break::Set("family", json!("pending"))),
        ("list_deliveries", "status desconocido", Break::Set("status", json!("bogus"))),
        ("list_deliveries", "limit 0", Break::Set("limit", json!(0))),
        ("list_terminal_events", "limit 0", Break::Set("limit", json!(0))),
        ("list_terminal_events", "family desconocida", Break::Set("family", json!("pending"))),
        ("list_progress", "sin trace_id ni correlation_id", Break::Payload(json!({}))),
        ("seal_dlt", "sin tx_hash", Break::Remove("tx_hash")),
        ("seal_dlt", "sin digest", Break::Remove("digest")),
        ("seal_dlt", "sin scope", Break::Remove("scope")),
        ("get_dlt_seal", "sin event_id", Break::Remove("event_id")),
    ]
}

fn trait_ops() -> BTreeSet<String> {
    let re = Regex::new(r"(?m)^    fn ([a-z_]+)\(").expect("regex");
    re.captures_iter(PORT_SRC).map(|c| c[1].to_string()).collect()
}

fn schema_ops() -> BTreeSet<String> {
    schema()
        .pointer("/$defs/request/properties/operation/enum")
        .and_then(|v| v.as_array())
        .expect("enum de operation")
        .iter()
        .map(|v| v.as_str().expect("string").to_string())
        .collect()
}

#[test]
fn trait_has_the_nineteen_known_operations() {
    assert_eq!(trait_ops().len(), 19, "{:?}", trait_ops());
}

#[test]
fn schema_operation_enum_equals_trait_operations() {
    let t = trait_ops();
    let s = schema_ops();
    let missing: Vec<_> = t.difference(&s).collect();
    let extra: Vec<_> = s.difference(&t).collect();
    assert!(missing.is_empty() && extra.is_empty(), "faltan en schema: {missing:?}; sobran: {extra:?}");
}

#[test]
fn every_trait_operation_has_a_typed_valid_request_and_a_negative_case() {
    let valid: BTreeSet<String> = valid_requests().iter().map(|(o, _)| (*o).to_string()).collect();
    let invalid: BTreeSet<String> = invalid_cases().iter().map(|(o, _, _)| (*o).to_string()).collect();
    let ops = trait_ops();
    assert_eq!(valid, ops, "cobertura de peticiones válidas");
    let uncovered: Vec<_> = ops.difference(&invalid).collect();
    assert!(uncovered.is_empty(), "operaciones sin caso negativo: {uncovered:?}");
}

#[test]
fn valid_typed_requests_validate_against_schema() {
    let v = validator();
    let mut failures = Vec::new();
    for (op, req) in valid_requests() {
        if let Err(e) = v.validate(&req) {
            failures.push(format!("{op}: {e}\n   {req}"));
        }
    }
    assert!(failures.is_empty(), "peticiones válidas rechazadas:\n{}", failures.join("\n"));
}

#[test]
fn broken_requests_are_rejected_by_schema() {
    let v = validator();
    let mut accepted = Vec::new();
    for (op, label, how) in invalid_cases() {
        let req = broken(op, &how);
        if v.is_valid(&req) {
            accepted.push(format!("{op}: {label}"));
        }
    }
    assert!(accepted.is_empty(), "el schema acepta peticiones fuera de contrato:\n{}", accepted.join("\n"));
}

#[test]
fn unknown_operation_is_rejected() {
    let v = validator();
    assert!(!v.is_valid(&json!({"operation": "drop_everything", "payload": {}})));
}
