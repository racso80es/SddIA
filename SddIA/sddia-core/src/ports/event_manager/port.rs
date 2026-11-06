//! Trait `EventManagerPort` — operaciones L-OPS 1:1.

use crate::ports::event_manager::contract::{
    AppendTraceRequest, ClaimPendingRequest, ClaimPendingResult, HandshakeVersionRequest,
    HandshakeVersionResult, MoveToDlqRequest, PublishEcstRequest, PublishEcstResult,
    RecordDeliveryRequest, ReplayDlqRequest,
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
}
