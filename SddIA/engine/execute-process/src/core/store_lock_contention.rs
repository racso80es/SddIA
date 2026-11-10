//! Contención de lock Redb / store — síntomas y firma `F-EDA-STORE-LOCK-CONTENTION`.

pub const STORE_LOCK_CONTENTION: &str = "STORE_LOCK_CONTENTION";
pub const F_EDA_STORE_LOCK_CONTENTION: &str = "F-EDA-STORE-LOCK-CONTENTION";

/// Literales compartidos (KA-LOCK-1 / L-SYMPTOM).
pub fn is_store_lock_contention_message(message: &str) -> bool {
    message.contains("Database already open")
        || message.contains("Cannot acquire lock")
        || message.contains(STORE_LOCK_CONTENTION)
        || message.contains("StoreError::Locked")
        || message.contains("DatabaseAlreadyOpen")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_redb_and_refine_literals() {
        assert!(is_store_lock_contention_message(
            "Database already open. Cannot acquire lock."
        ));
        assert!(is_store_lock_contention_message("StoreError::Locked: busy"));
        assert!(!is_store_lock_contention_message("raw_idea requerido"));
    }
}
