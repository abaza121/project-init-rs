//! Authoritative SQLite persistence and migration boundaries.

mod error;
mod sqlite;

pub use error::StorageError;
pub use sqlite::SqliteStore;
