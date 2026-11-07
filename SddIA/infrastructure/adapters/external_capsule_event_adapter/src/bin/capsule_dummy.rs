//! Dummy: líneas JSON `{operation, payload}` → `InMemoryEventManagerMock` → respuesta una línea.

use std::io::{BufRead, BufWriter, Write};

use serde::Deserialize;
use serde_json::{json, Value};
use sddia_core::ports::event_manager::{
    AppendTraceRequest, ClaimPendingRequest, EventManagerPort, GetEventRequest,
    HandshakeVersionRequest, InMemoryEventManagerMock, ListDlqRequest, ListPendingRequest,
    ListTracesRequest, MoveToDlqRequest, PublishEcstRequest, PurgeEventRequest,
    RecordDeliveryRequest, ReplayDlqRequest,
};

#[derive(Debug, Deserialize)]
struct CapsuleRequest {
    operation: String,
    payload: Value,
}

fn ok_line(result: Option<Value>) -> String {
    serde_json::to_string(&json!({
        "success": true,
        "exitCode": 0,
        "result": result.unwrap_or(json!({})),
    }))
    .expect("serde")
}

fn err_line(msg: &str) -> String {
    serde_json::to_string(&json!({
        "success": false,
        "exitCode": 1,
        "error": { "message": msg },
    }))
    .expect("serde")
}

fn main() {
    let mock = InMemoryEventManagerMock::new();
    let stdin = std::io::stdin();
    let mut out = BufWriter::new(std::io::stdout());
    for line in stdin.lock().lines().map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let req: CapsuleRequest = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(e) => {
                writeln!(out, "{}", err_line(&format!("parse: {e}"))).ok();
                let _ = out.flush();
                continue;
            }
        };

        let out_line: String = match req.operation.as_str() {
            "handshake_version" => match serde_json::from_value::<HandshakeVersionRequest>(req.payload)
            {
                Ok(p) => match mock.handshake_version(p) {
                    Ok(r) => ok_line(Some(serde_json::to_value(r).unwrap())),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "publish_ecst" => match serde_json::from_value::<PublishEcstRequest>(req.payload) {
                Ok(p) => match mock.publish_ecst(p) {
                    Ok(r) => ok_line(Some(serde_json::to_value(r).unwrap())),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "claim_pending" => match serde_json::from_value::<ClaimPendingRequest>(req.payload) {
                Ok(p) => match mock.claim_pending(p) {
                    Ok(r) => ok_line(Some(serde_json::to_value(r).unwrap())),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "record_delivery" => match serde_json::from_value::<RecordDeliveryRequest>(req.payload) {
                Ok(p) => match mock.record_delivery(p) {
                    Ok(()) => ok_line(None),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "move_to_dlq" => match serde_json::from_value::<MoveToDlqRequest>(req.payload) {
                Ok(p) => match mock.move_to_dlq(p) {
                    Ok(()) => ok_line(None),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "replay_dlq" => match serde_json::from_value::<ReplayDlqRequest>(req.payload) {
                Ok(p) => match mock.replay_dlq(p) {
                    Ok(()) => ok_line(None),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "append_trace" => match serde_json::from_value::<AppendTraceRequest>(req.payload) {
                Ok(p) => match mock.append_trace(p) {
                    Ok(()) => ok_line(None),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "get_event" => match serde_json::from_value::<GetEventRequest>(req.payload) {
                Ok(p) => match mock.get_event(p) {
                    Ok(r) => ok_line(Some(serde_json::to_value(r).unwrap())),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "list_pending" => match serde_json::from_value::<ListPendingRequest>(req.payload) {
                Ok(p) => match mock.list_pending(p) {
                    Ok(r) => ok_line(Some(serde_json::to_value(r).unwrap())),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "list_dlq" => match serde_json::from_value::<ListDlqRequest>(req.payload) {
                Ok(p) => match mock.list_dlq(p) {
                    Ok(r) => ok_line(Some(serde_json::to_value(r).unwrap())),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "list_traces" => match serde_json::from_value::<ListTracesRequest>(req.payload) {
                Ok(p) => match mock.list_traces(p) {
                    Ok(r) => ok_line(Some(serde_json::to_value(r).unwrap())),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            "purge_event" => match serde_json::from_value::<PurgeEventRequest>(req.payload) {
                Ok(p) => match mock.purge_event(p) {
                    Ok(()) => ok_line(None),
                    Err(e) => err_line(&e.to_string()),
                },
                Err(e) => err_line(&e.to_string()),
            },
            _ => err_line("unknown operation"),
        };

        writeln!(out, "{out_line}").ok();
        let _ = out.flush();
    }
}
