//! Resolución de `tracker_ref` en sustrato remoto (AUT-35).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteTrackerRefResolution {
    Seal(String),
    Conflict,
}

/// Sin `tracker_ref` en frontmatter → sellar con el identifier leído.
/// Igualdad ASCII case-insensitive → sellar con el identifier canónico (trim).
/// Distinto → conflicto sin escritura.
pub fn resolve_remote_tracker_ref(
    fm_tracker_ref: Option<&str>,
    identifier: &str,
) -> RemoteTrackerRefResolution {
    let id = identifier.trim();
    if id.is_empty() {
        return RemoteTrackerRefResolution::Conflict;
    }
    match fm_tracker_ref.map(str::trim).filter(|s| !s.is_empty()) {
        None => RemoteTrackerRefResolution::Seal(id.to_string()),
        Some(fm_tr) if fm_tr.eq_ignore_ascii_case(id) => RemoteTrackerRefResolution::Seal(id.to_string()),
        Some(_) => RemoteTrackerRefResolution::Conflict,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_fm_seals_identifier() {
        assert_eq!(
            resolve_remote_tracker_ref(None, "AUT-35"),
            RemoteTrackerRefResolution::Seal("AUT-35".into())
        );
    }

    #[test]
    fn empty_fm_seals_identifier() {
        assert_eq!(
            resolve_remote_tracker_ref(Some(""), "AUT-35"),
            RemoteTrackerRefResolution::Seal("AUT-35".into())
        );
        assert_eq!(
            resolve_remote_tracker_ref(Some("   "), "AUT-35"),
            RemoteTrackerRefResolution::Seal("AUT-35".into())
        );
    }

    #[test]
    fn equal_ignore_ascii_case_seals_canonical() {
        assert_eq!(
            resolve_remote_tracker_ref(Some("aut-35"), "AUT-35"),
            RemoteTrackerRefResolution::Seal("AUT-35".into())
        );
    }

    #[test]
    fn divergent_fm_conflicts() {
        assert_eq!(
            resolve_remote_tracker_ref(Some("AUT-44"), "AUT-35"),
            RemoteTrackerRefResolution::Conflict
        );
    }
}
