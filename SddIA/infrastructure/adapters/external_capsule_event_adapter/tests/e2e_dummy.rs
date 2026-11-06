use std::path::PathBuf;

use sddia_core::ports::event_manager::run_event_manager_contract_suite;
use sddia_infrastructure_external_capsule_event::ExternalCapsuleEventAdapter;

#[test]
fn e2e_suite_on_dummy_capsule() {
    let exe = std::env::var_os("CARGO_BIN_EXE_eda-event-manager-capsule-dummy")
        .map(PathBuf::from)
        .expect("CARGO_BIN_EXE_eda-event-manager-capsule-dummy");
    let adapter = ExternalCapsuleEventAdapter::new(exe);
    run_event_manager_contract_suite(&adapter);
}
