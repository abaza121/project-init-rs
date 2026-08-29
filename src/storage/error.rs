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
    /// Rejects an external numeric value that SQLite cannot represent losslessly.
    #[error("{field} is outside the supported range: {value}")]
    ValueOutOfRange { field: &'static str, value: u64 },
    /// Rejects changing policy while a project still has an active resumable run.
    #[error("active workflow uses {active} approval, not requested {requested} approval")]
    ActiveRunPolicyConflict { active: String, requested: String },
    /// Indicates that no active run matches the requested durable identity.
    #[error("workflow run does not exist or is already terminal: {0}")]
    WorkflowRunNotFound(String),
    /// Indicates that an artifact path has no registered generated document.
    #[error("document is not registered: {0}")]
    DocumentNotFound(String),
    /// Indicates that no decision matches an approval or rejection command.
    #[error("decision does not exist: {0}")]
    DecisionNotFound(String),
    /// Prevents repeated or conflicting authority records for a closed decision.
    #[error("decision is not awaiting authority: {0}")]
    DecisionNotPending(String),
}
