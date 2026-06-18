pub mod core;
pub mod engine;
pub mod envelope;

pub use core::repo::find_repo_root;
pub use core::resolver::{load_process_def, normalize_request, validate_process_inputs};
pub use envelope::{emit, OrchestratorEnvelope};
pub use engine::run_process;
