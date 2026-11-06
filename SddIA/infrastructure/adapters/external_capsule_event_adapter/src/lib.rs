//! Puente NFT-ready: delegación a proceso externo vía JSON por stdin/stdout (contrato v1 request).

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use serde_json::{json, Value};
use sddia_core::ports::event_manager::{
    AppendTraceRequest, ClaimPendingRequest, ClaimPendingResult, EventManagerError, EventManagerPort,
    HandshakeVersionRequest, HandshakeVersionResult, MoveToDlqRequest, PublishEcstRequest,
    PublishEcstResult, RecordDeliveryRequest, ReplayDlqRequest,
};

struct ExternalCapsuleInner {
    exe: PathBuf,
    child: Option<Child>,
}

/// Delegación stateful: un único hijo mantiene estado (mock en el dummy).
pub struct ExternalCapsuleEventAdapter {
    inner: Mutex<ExternalCapsuleInner>,
}

impl ExternalCapsuleEventAdapter {
    pub fn new(exe: impl Into<PathBuf>) -> Self {
        Self {
            inner: Mutex::new(ExternalCapsuleInner {
                exe: exe.into(),
                child: None,
            }),
        }
    }

    fn spawn_if_needed(inner: &mut ExternalCapsuleInner) -> Result<&mut Child, EventManagerError> {
        if inner.child.is_none() {
            let mut cmd = Command::new(&inner.exe);
            cmd.env("SDDIA_BYPASS_THERMO_TOLL", "1")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            inner.child = Some(cmd.spawn().map_err(|e| {
                EventManagerError::ExternalCapsule(format!("spawn: {e}"))
            })?);
        }
        inner
            .child
            .as_mut()
            .ok_or_else(|| EventManagerError::ExternalCapsule("no child".into()))
    }

    fn rpc(inner: &mut ExternalCapsuleInner, operation: &str, payload: Value) -> Result<Value, EventManagerError> {
        let envelope = json!({
            "operation": operation,
            "payload": payload,
        });
        let child = Self::spawn_if_needed(inner)?;
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| EventManagerError::ExternalCapsule("stdin closed".into()))?;
        let line = serde_json::to_string(&envelope).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?;
        writeln!(stdin, "{line}").map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?;
        stdin.flush().map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?;

        let stdout = child
            .stdout
            .as_mut()
            .ok_or_else(|| EventManagerError::ExternalCapsule("stdout closed".into()))?;
        let mut reader = BufReader::new(stdout);
        let mut out_line = String::new();
        reader
            .read_line(&mut out_line)
            .map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?;
        let v: Value = serde_json::from_str(out_line.trim())
            .map_err(|e| EventManagerError::ExternalCapsule(format!("json: {e}")))?;
        if !v.get("success").and_then(|x| x.as_bool()).unwrap_or(false) {
            let msg = v
                .pointer("/error/message")
                .and_then(|x| x.as_str())
                .unwrap_or("capsule error");
            return Err(EventManagerError::ExternalCapsule(msg.to_string()));
        }
        Ok(v)
    }
}

impl EventManagerPort for ExternalCapsuleEventAdapter {
    fn publish_ecst(
        &self,
        req: PublishEcstRequest,
    ) -> Result<PublishEcstResult, EventManagerError> {
        let mut g = self.inner.lock().expect("capsule lock");
        let v = Self::rpc(
            &mut g,
            "publish_ecst",
            serde_json::to_value(&req).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?,
        )?;
        let r = v.get("result").ok_or_else(|| EventManagerError::ExternalCapsule("missing result".into()))?;
        let out: PublishEcstResult = serde_json::from_value(r.clone())
            .map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?;
        Ok(out)
    }

    fn claim_pending(
        &self,
        req: ClaimPendingRequest,
    ) -> Result<ClaimPendingResult, EventManagerError> {
        let mut g = self.inner.lock().expect("capsule lock");
        let v = Self::rpc(
            &mut g,
            "claim_pending",
            serde_json::to_value(&req).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?,
        )?;
        let r = v.get("result").ok_or_else(|| EventManagerError::ExternalCapsule("missing result".into()))?;
        serde_json::from_value(r.clone()).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))
    }

    fn record_delivery(&self, req: RecordDeliveryRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("capsule lock");
        let _ = Self::rpc(
            &mut g,
            "record_delivery",
            serde_json::to_value(&req).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?,
        )?;
        Ok(())
    }

    fn move_to_dlq(&self, req: MoveToDlqRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("capsule lock");
        let _ = Self::rpc(
            &mut g,
            "move_to_dlq",
            serde_json::to_value(&req).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?,
        )?;
        Ok(())
    }

    fn replay_dlq(&self, req: ReplayDlqRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("capsule lock");
        let _ = Self::rpc(
            &mut g,
            "replay_dlq",
            serde_json::to_value(&req).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?,
        )?;
        Ok(())
    }

    fn append_trace(&self, req: AppendTraceRequest) -> Result<(), EventManagerError> {
        let mut g = self.inner.lock().expect("capsule lock");
        let _ = Self::rpc(
            &mut g,
            "append_trace",
            serde_json::to_value(&req).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?,
        )?;
        Ok(())
    }

    fn handshake_version(
        &self,
        req: HandshakeVersionRequest,
    ) -> Result<HandshakeVersionResult, EventManagerError> {
        let mut g = self.inner.lock().expect("capsule lock");
        let v = Self::rpc(
            &mut g,
            "handshake_version",
            serde_json::to_value(&req).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))?,
        )?;
        let r = v.get("result").ok_or_else(|| EventManagerError::ExternalCapsule("missing result".into()))?;
        serde_json::from_value(r.clone()).map_err(|e| EventManagerError::ExternalCapsule(e.to_string()))
    }
}
