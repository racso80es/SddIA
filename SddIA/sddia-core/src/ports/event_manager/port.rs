//! Trait `EventManagerPort` — operaciones L-OPS 1:1.

use crate::ports::event_manager::contract::{
    AppendTraceRequest, ClaimPendingRequest, ClaimPendingResult, GetEventRequest, GetEventResult,
    HandshakeVersionRequest, HandshakeVersionResult, ListDlqRequest, ListDlqResult,
    ListPendingRequest, ListPendingResult, ListTracesRequest, ListTracesResult, MoveToDlqRequest,
    PublishEcstRequest, PublishEcstResult, PurgeEventRequest, RecordDeliveryRequest,
    ReplayDlqRequest,
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
}
