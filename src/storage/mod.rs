//! Authoritative SQLite persistence and migration boundaries.

mod error;
mod sqlite;
mod workflow;

pub use error::StorageError;
pub use sqlite::SqliteStore;
