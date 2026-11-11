//! Gate SemVer: rango exigido vs `implements_contract.contract_version`.

use semver::{Version, VersionReq};

use crate::ports::event_manager::contract::REQUIRED_CONTRACT_RANGE;
use crate::ports::event_manager::error::EventManagerError;

/// Valida que `implementor_version` satisfaga `required_range` (p.ej. `^2.0.0`).
/// Rechazo MAJOR → `EventManagerError::ContractVersionMismatch` / `F-DI-CONTRACT-VERSION-MISMATCH`.
pub fn version_gate(
    required_range: &str,
    implementor_version: &str,
) -> Result<(), EventManagerError> {
    let req = VersionReq::parse(required_range).map_err(|e| {
        EventManagerError::InvalidRequest(format!("invalid required_range '{required_range}': {e}"))
    })?;
    let ver = Version::parse(implementor_version).map_err(|e| {
        EventManagerError::InvalidRequest(format!(
            "invalid implementor_version '{implementor_version}': {e}"
        ))
    })?;

    if req.matches(&ver) {
        Ok(())
    } else {
        Err(EventManagerError::version_mismatch(format!(
            "implementor {implementor_version} does not satisfy {required_range}"
        )))
    }
}

/// Gate canónico del host contra `REQUIRED_CONTRACT_RANGE` (^2.0.0).
pub fn gate_host_v1(implementor_version: &str) -> Result<(), EventManagerError> {
    version_gate(REQUIRED_CONTRACT_RANGE, implementor_version)
}

/// Alias explícito v2.
pub fn gate_host_v2(implementor_version: &str) -> Result<(), EventManagerError> {
    gate_host_v1(implementor_version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::event_manager::error::F_DI_CONTRACT_VERSION_MISMATCH;

    #[test]
    fn accepts_minor_and_patch_under_caret_2() {
        assert!(version_gate("^2.0.0", "2.0.0").is_ok());
        assert!(version_gate("^2.0.0", "2.0.1").is_ok());
        assert!(version_gate("^2.0.0", "2.1.0").is_ok());
        assert!(gate_host_v1("2.0.0").is_ok());
        assert!(gate_host_v2("2.0.0").is_ok());
    }

    #[test]
    fn rejects_v1_consumers() {
        let err = version_gate("^2.0.0", "1.2.0").unwrap_err();
        match err {
            EventManagerError::ContractVersionMismatch { seed, .. } => {
                assert_eq!(seed, F_DI_CONTRACT_VERSION_MISMATCH);
            }
            other => panic!("expected ContractVersionMismatch, got {other:?}"),
        }
    }
}
