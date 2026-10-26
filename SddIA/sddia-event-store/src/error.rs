use std::fmt;

#[derive(Debug)]
pub enum StoreError {
    Invalid(String),
    NotFound(String),
    Conflict(String),
    /// Contención de lock de `redb` tras agotar el presupuesto de reintento.
    Locked(String),
    Backend(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::Invalid(message)
            | StoreError::NotFound(message)
            | StoreError::Conflict(message)
            | StoreError::Locked(message)
            | StoreError::Backend(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for StoreError {}

pub(crate) fn backend(err: impl fmt::Display) -> StoreError {
    StoreError::Backend(err.to_string())
}

pub(crate) fn table_missing(err: &redb::TableError) -> bool {
    matches!(err, redb::TableError::TableDoesNotExist(_))
}
