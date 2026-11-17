//! Puerto `eda.event_manager` **2.0.0** / `EventUri` (MAN-43; supersede MAN-32/35/40 T0).

mod contract;
mod contract_suite;
mod error;
mod event_manager_factory;
mod event_query;
mod memory_mock;
mod port;
mod runtime_bridge;
mod version_gate;

pub use contract::*;
pub use contract_suite::run_event_manager_contract_suite;
pub use error::*;
pub use event_query::{
    filter_and_page_events, filter_terminal_events, indexed_from_parts, page_deliveries,
};
pub use event_manager_factory::*;
pub use memory_mock::*;
pub use port::*;
pub use runtime_bridge::*;
pub use version_gate::*;
