//! Puerto aditivo `eda.event_manager` v1.0.0 (PBI-EDA-001 / MAN-32).

mod contract;
mod contract_suite;
mod error;
mod event_manager_factory;
mod memory_mock;
mod port;
mod version_gate;

pub use contract::*;
pub use contract_suite::run_event_manager_contract_suite;
pub use error::*;
pub use event_manager_factory::*;
pub use memory_mock::*;
pub use port::*;
pub use version_gate::*;
