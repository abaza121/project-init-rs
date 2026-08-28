use thiserror::Error;

use crate::domain::DomainError;

/// Describes persistence failures without exposing provider details to domain types.
#[derive(Debug, Error)]
pub enum StorageError {
    /// Wraps a SQLite operation that failed before its transaction could commit.
    #[error("SQLite operation failed: {0}")]
    Sqlite(#[from] rusqlite::Error),
    /// Propagates invalid domain input encountered at the storage boundary.
    #[error(transparent)]
    Domain(#[from] DomainError),
    /// Indicates that persisted checked text no longer maps to a supported domain value.
    #[error("database contains invalid {field}: {value}")]
    CorruptData { field: &'static str, value: String },
    /// Indicates that a persisted timestamp is not valid RFC 3339 UTC text.
    #[error("database contains invalid timestamp in {field}: {value}")]
    InvalidTimestamp { field: &'static str, value: String },
    /// Rejects duplicate active knowledge before sequence allocation or insertion.
    #[error("an equivalent active finding already exists in this project")]
    DuplicateFinding,
    /// Indicates that a mutation referenced a project that does not exist.
    #[error("project does not exist: {0}")]
    ProjectNotFound(String),
    /// Indicates that a clarification answer referenced no persisted question.
    #[error("question does not exist: {0}")]
    QuestionNotFound(String),
    /// Prevents a closed clarification from being reconciled twice by accident.
    #[error("question is not open: {0}")]
    QuestionNotOpen(String),
    /// Prevents stable display identifiers from wrapping or becoming negative.
    #[error("display identifier sequence is exhausted for prefix {0}")]
    SequenceExhausted(String),
    /// Prevents opening a database created by a newer, unsupported schema.
    #[error("database schema version {found} is newer than supported version {supported}")]
    UnsupportedSchema { found: u32, supported: u32 },
}
