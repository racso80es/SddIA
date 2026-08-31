use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryStoreError {
    DimensionMismatch { expected: usize, actual: usize },
    SchemaIncompatible { reason: String },
    StoreCorrupt { reason: String },
    EmbeddingFailed { reason: String },
    Io { reason: String },
}

impl fmt::Display for MemoryStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch { expected, actual } => {
                write!(f, "dimension mismatch: expected {expected}, got {actual}")
            }
            Self::SchemaIncompatible { reason } => write!(f, "schema incompatible: {reason}"),
            Self::StoreCorrupt { reason } => write!(f, "store corrupt: {reason}"),
            Self::EmbeddingFailed { reason } => write!(f, "embedding failed: {reason}"),
            Self::Io { reason } => write!(f, "io: {reason}"),
        }
    }
}

impl std::error::Error for MemoryStoreError {}

impl From<String> for MemoryStoreError {
    fn from(reason: String) -> Self {
        Self::Io { reason }
    }
}

impl From<&str> for MemoryStoreError {
    fn from(reason: &str) -> Self {
        Self::Io {
            reason: reason.to_string(),
        }
    }
}
