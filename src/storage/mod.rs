//! Authoritative SQLite persistence and migration boundaries.

mod error;
mod r#loop;
mod sqlite;
mod workflow;

pub use error::StorageError;
pub use sqlite::SqliteStore;
pub(crate) use workflow::{AnalyzedFindingInput, ClarificationInput};
