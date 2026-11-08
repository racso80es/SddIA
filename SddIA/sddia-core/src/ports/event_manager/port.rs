//! Trait `EventManagerPort` — operaciones L-OPS 1:1.

use crate::ports::event_manager::contract::{
    AppendTraceRequest, ClaimPendingRequest, ClaimPendingResult, GetDltSealRequest,
    GetDltSealResult, GetEventRequest, GetEventResult, HandshakeVersionRequest,
    HandshakeVersionResult, ListDeliveriesRequest, ListDeliveriesResult, ListDlqRequest,
    ListDlqResult, ListEventsRequest, ListEventsResult, ListPendingRequest, ListPendingResult,
    ListProgressRequest, ListProgressResult, ListedEvent, ListTracesRequest, ListTracesResult,
    MoveToDlqRequest, PublishEcstRequest, PublishEcstResult, PurgeEventRequest,
    RecordDeliveryRequest, ReplayDlqRequest, SealDltRequest, VerifyInlineEcstRequest,
};
use crate::ports::event_manager::error::EventManagerError;

pub trait EventManagerPort: Send + Sync {
    fn publish_ecst(
        &self,
        req: PublishEcstRequest,
    ) -> Result<PublishEcstResult, EventManagerError>;

    fn claim_pending(
        &self,
        req: ClaimPendingRequest,
    ) -> Result<ClaimPendingResult, EventManagerError>;

    fn record_delivery(&self, req: RecordDeliveryRequest) -> Result<(), EventManagerError>;

    fn move_to_dlq(&self, req: MoveToDlqRequest) -> Result<(), EventManagerError>;

    fn replay_dlq(&self, req: ReplayDlqRequest) -> Result<(), EventManagerError>;

    fn append_trace(&self, req: AppendTraceRequest) -> Result<(), EventManagerError>;

    fn handshake_version(
        &self,
        req: HandshakeVersionRequest,
    ) -> Result<HandshakeVersionResult, EventManagerError>;

    fn get_event(&self, req: GetEventRequest) -> Result<GetEventResult, EventManagerError>;

    fn list_pending(
        &self,
        req: ListPendingRequest,
    ) -> Result<ListPendingResult, EventManagerError>;

    fn list_dlq(&self, req: ListDlqRequest) -> Result<ListDlqResult, EventManagerError>;

    fn list_traces(&self, req: ListTracesRequest) -> Result<ListTracesResult, EventManagerError>;

    fn purge_event(&self, req: PurgeEventRequest) -> Result<(), EventManagerError>;

    fn verify_inline_ecst(&self, req: VerifyInlineEcstRequest) -> Result<(), EventManagerError> {
        let row = self.get_event(GetEventRequest {
            event_id: req.event_id,
        })?;
        if row.ecst != req.ecst {
            return Err(EventManagerError::InvalidRequest("ecst_mismatch".into()));
        }
        Ok(())
    }

    fn list_events(&self, req: ListEventsRequest) -> Result<ListEventsResult, EventManagerError> {
        let limit = req.limit.unwrap_or(100);
        let pending = self.list_pending(ListPendingRequest {
            limit: Some(limit),
        })?;
        let mut events = Vec::new();
        for e in pending.events {
            if let Some(et) = req.event_type.as_deref() {
                let matches = e
                    .ecst
                    .get("event_type")
                    .and_then(|v| v.as_str())
                    == Some(et);
                if !matches {
                    continue;
                }
            }
            events.push(ListedEvent {
                event_id: e.event_id,
                ecst: e.ecst,
                trace_id: e.trace_id,
            });
        }
        Ok(ListEventsResult {
            events,
            next_cursor: None,
        })
    }

    fn list_deliveries(
        &self,
        _req: ListDeliveriesRequest,
    ) -> Result<ListDeliveriesResult, EventManagerError> {
        Err(EventManagerError::InvalidRequest(
            "list_deliveries not implemented for this adapter".into(),
        ))
    }

    fn list_progress(
        &self,
        req: ListProgressRequest,
    ) -> Result<ListProgressResult, EventManagerError> {
        let trace_id = req
            .trace_id
            .or(req.correlation_id)
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| {
                EventManagerError::InvalidRequest("trace_id or correlation_id required".into())
            })?;
        let frames = self
            .list_traces(ListTracesRequest { trace_id })?
            .frames;
        Ok(ListProgressResult { frames })
    }

    fn seal_dlt(&self, _req: SealDltRequest) -> Result<(), EventManagerError> {
        Err(EventManagerError::InvalidRequest(
            "seal_dlt not implemented for this adapter".into(),
        ))
    }

    fn get_dlt_seal(
        &self,
        _req: GetDltSealRequest,
    ) -> Result<GetDltSealResult, EventManagerError> {
        Err(EventManagerError::InvalidRequest(
            "get_dlt_seal not implemented for this adapter".into(),
        ))
    }
}
