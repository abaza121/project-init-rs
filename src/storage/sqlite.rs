use std::path::Path;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Transaction, params};

use crate::domain::{
    Confidence, Finding, FindingId, FindingKind, FindingStatus, Impact, NewFinding, NewProject,
    Project, ProjectId, ProjectStatus, RetrievalMode, SourceType,
};

use super::StorageError;

const CURRENT_SCHEMA_VERSION: u32 = 1;
const INITIAL_MIGRATION: &str = include_str!("../../migrations/001_initial.sql");

/// Owns one SQLite connection and exposes transactional authoritative operations.
pub struct SqliteStore {
    pub(super) connection: Connection,
}

impl SqliteStore {
    /// Opens or creates an on-disk database and applies supported migrations.
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        Self::initialize(Connection::open(path)?)
    }

    /// Opens an isolated in-memory database with the same migrations as production.
    pub fn open_in_memory() -> Result<Self, StorageError> {
        Self::initialize(Connection::open_in_memory()?)
    }

    /// Enables integrity checks and migrates a newly opened connection.
    fn initialize(connection: Connection) -> Result<Self, StorageError> {
        connection.pragma_update(None, "foreign_keys", "ON")?;
        let found =
            connection.pragma_query_value(None, "user_version", |row| row.get::<_, u32>(0))?;
        if found > CURRENT_SCHEMA_VERSION {
            return Err(StorageError::UnsupportedSchema {
                found,
                supported: CURRENT_SCHEMA_VERSION,
            });
        }
        if found == 0 {
            connection.execute_batch(INITIAL_MIGRATION)?;
        }
        Ok(Self { connection })
    }

    /// Lists application table names for migration verification and diagnostics.
    pub fn table_names(&self) -> Result<Vec<String>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT name FROM sqlite_schema \
             WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
        )?;
        let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Persists a validated draft project as one authoritative row.
    pub fn create_project(&mut self, input: NewProject) -> Result<Project, StorageError> {
        let project = Project::from_new(input);
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT INTO projects \
             (id, name, brief, status, retrieval_mode, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                project.id().as_str(),
                project.name(),
                project.brief(),
                project.status().as_db_str(),
                project.retrieval_mode().as_db_str(),
                project.created_at().to_rfc3339(),
                project.updated_at().to_rfc3339(),
            ],
        )?;
        transaction.commit()?;
        Ok(project)
    }

    /// Loads a project by durable identity without inferring a new lifecycle state.
    pub fn get_project(&self, id: &ProjectId) -> Result<Option<Project>, StorageError> {
        let row = self
            .connection
            .query_row(
                "SELECT id, name, brief, status, retrieval_mode, created_at, updated_at \
                 FROM projects WHERE id = ?1",
                [id.as_str()],
                StoredProjectRow::read,
            )
            .optional()?;
        row.map(StoredProjectRow::into_domain).transpose()
    }

    /// Lists projects in creation order for CLI discovery and resume selection.
    pub fn list_projects(&self) -> Result<Vec<Project>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, name, brief, status, retrieval_mode, created_at, updated_at \
             FROM projects ORDER BY created_at, id",
        )?;
        let rows = statement.query_map([], StoredProjectRow::read)?;
        rows.map(|row| {
            row.map_err(StorageError::from)
                .and_then(StoredProjectRow::into_domain)
        })
        .collect()
    }

    /// Inserts distinct project knowledge and allocates its stable display ID atomically.
    pub fn add_finding(&mut self, input: NewFinding) -> Result<Finding, StorageError> {
        let transaction = self.connection.transaction()?;
        ensure_project_exists(&transaction, input.project_id())?;

        // Duplicate classification precedes sequence allocation so rejected input cannot create gaps.
        let duplicate = transaction.query_row(
            "SELECT EXISTS(SELECT 1 FROM findings \
             WHERE project_id = ?1 AND kind = ?2 AND statement = ?3 COLLATE NOCASE \
             AND status IN ('active', 'confirmed'))",
            params![
                input.project_id().as_str(),
                input.kind().as_db_str(),
                input.statement(),
            ],
            |row| row.get::<_, bool>(0),
        )?;
        if duplicate {
            return Err(StorageError::DuplicateFinding);
        }

        let display_id = allocate_display_id(
            &transaction,
            input.project_id(),
            input.kind().display_prefix(),
        )?;
        let finding = Finding::from_new(input, display_id);
        insert_finding(&transaction, &finding)?;
        transaction.commit()?;
        Ok(finding)
    }

    /// Returns all findings in stable display-ID order for one project partition.
    pub fn list_findings(&self, project_id: &ProjectId) -> Result<Vec<Finding>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, display_id, project_id, kind, statement, source_type, \
             source_reference, confidence, impact, status, requires_confirmation, \
             created_at, updated_at FROM findings WHERE project_id = ?1 ORDER BY display_id",
        )?;
        let rows = statement.query_map([project_id.as_str()], StoredFindingRow::read)?;
        rows.map(|row| {
            row.map_err(StorageError::from)
                .and_then(StoredFindingRow::into_domain)
        })
        .collect()
    }
}

/// Holds raw project columns until checked text and timestamps are validated.
struct StoredProjectRow {
    id: String,
    name: String,
    brief: String,
    status: String,
    retrieval_mode: String,
    created_at: String,
    updated_at: String,
}

impl StoredProjectRow {
    /// Reads raw project columns from the current SQLite row.
    fn read(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            brief: row.get(2)?,
            status: row.get(3)?,
            retrieval_mode: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }

    /// Converts checked persisted values into the infrastructure-free domain type.
    fn into_domain(self) -> Result<Project, StorageError> {
        let status =
            ProjectStatus::from_db_str(&self.status).ok_or_else(|| StorageError::CorruptData {
                field: "project status",
                value: self.status.clone(),
            })?;
        let retrieval_mode = RetrievalMode::from_db_str(&self.retrieval_mode).ok_or_else(|| {
            StorageError::CorruptData {
                field: "retrieval mode",
                value: self.retrieval_mode.clone(),
            }
        })?;
        Ok(Project::from_stored(
            ProjectId::from_stored(self.id),
            self.name,
            self.brief,
            status,
            retrieval_mode,
            parse_timestamp(self.created_at, "projects.created_at")?,
            parse_timestamp(self.updated_at, "projects.updated_at")?,
        ))
    }
}

/// Holds raw finding columns until every checked vocabulary value is validated.
struct StoredFindingRow {
    id: String,
    display_id: String,
    project_id: String,
    kind: String,
    statement: String,
    source_type: String,
    source_reference: String,
    confidence: String,
    impact: String,
    status: String,
    requires_confirmation: bool,
    created_at: String,
    updated_at: String,
}

impl StoredFindingRow {
    /// Reads raw finding columns from the current SQLite row.
    fn read(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            display_id: row.get(1)?,
            project_id: row.get(2)?,
            kind: row.get(3)?,
            statement: row.get(4)?,
            source_type: row.get(5)?,
            source_reference: row.get(6)?,
            confidence: row.get(7)?,
            impact: row.get(8)?,
            status: row.get(9)?,
            requires_confirmation: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    }

    /// Converts checked persisted values into an auditable domain finding.
    fn into_domain(self) -> Result<Finding, StorageError> {
        let kind = parse_checked(&self.kind, "finding kind", FindingKind::from_db_str)?;
        let source_type = parse_checked(&self.source_type, "source type", SourceType::from_db_str)?;
        let confidence = parse_checked(&self.confidence, "confidence", Confidence::from_db_str)?;
        let impact = parse_checked(&self.impact, "impact", Impact::from_db_str)?;
        let status = parse_checked(&self.status, "finding status", FindingStatus::from_db_str)?;
        Ok(Finding::from_stored(
            FindingId::from_stored(self.id),
            self.display_id,
            ProjectId::from_stored(self.project_id),
            kind,
            self.statement,
            source_type,
            self.source_reference,
            confidence,
            impact,
            status,
            self.requires_confirmation,
            parse_timestamp(self.created_at, "findings.created_at")?,
            parse_timestamp(self.updated_at, "findings.updated_at")?,
        ))
    }
}

/// Verifies a finding partition before any sequence or knowledge mutation occurs.
fn ensure_project_exists(
    transaction: &Transaction<'_>,
    project_id: &ProjectId,
) -> Result<(), StorageError> {
    let exists = transaction.query_row(
        "SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?1)",
        [project_id.as_str()],
        |row| row.get::<_, bool>(0),
    )?;
    if exists {
        Ok(())
    } else {
        Err(StorageError::ProjectNotFound(project_id.to_string()))
    }
}

/// Reserves the next project-local display identifier inside the caller's transaction.
pub(super) fn allocate_display_id(
    transaction: &Transaction<'_>,
    project_id: &ProjectId,
    prefix: &str,
) -> Result<String, StorageError> {
    let next = transaction
        .query_row(
            "SELECT next_value FROM entity_sequences WHERE project_id = ?1 AND prefix = ?2",
            params![project_id.as_str(), prefix],
            |row| row.get::<_, i64>(0),
        )
        .optional()?;
    let sequence = next.unwrap_or(1);
    if next.is_some() {
        let following = sequence
            .checked_add(1)
            .ok_or_else(|| StorageError::SequenceExhausted(prefix.to_owned()))?;
        transaction.execute(
            "UPDATE entity_sequences SET next_value = ?3 WHERE project_id = ?1 AND prefix = ?2",
            params![project_id.as_str(), prefix, following],
        )?;
    } else {
        transaction.execute(
            "INSERT INTO entity_sequences (project_id, prefix, next_value) VALUES (?1, ?2, 2)",
            params![project_id.as_str(), prefix],
        )?;
    }
    Ok(format!("{prefix}-{sequence:03}"))
}

/// Inserts a fully identified finding inside the caller's atomic transaction.
fn insert_finding(transaction: &Transaction<'_>, finding: &Finding) -> Result<(), StorageError> {
    transaction.execute(
        "INSERT INTO findings \
         (id, display_id, project_id, kind, statement, source_type, source_reference, \
          confidence, impact, status, requires_confirmation, created_at, updated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            finding.id().as_str(),
            finding.display_id(),
            finding.project_id().as_str(),
            finding.kind().as_db_str(),
            finding.statement(),
            finding.source_type().as_db_str(),
            finding.source_reference(),
            finding.confidence().as_db_str(),
            finding.impact().as_db_str(),
            finding.status().as_db_str(),
            finding.requires_confirmation(),
            finding.created_at().to_rfc3339(),
            finding.updated_at().to_rfc3339(),
        ],
    )?;
    Ok(())
}

/// Parses a checked text vocabulary or reports the exact corrupt field.
fn parse_checked<T>(
    value: &str,
    field: &'static str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, StorageError> {
    parser(value).ok_or_else(|| StorageError::CorruptData {
        field,
        value: value.to_owned(),
    })
}

/// Parses an RFC 3339 timestamp while preserving its UTC audit meaning.
pub(super) fn parse_timestamp(
    value: String,
    field: &'static str,
) -> Result<DateTime<Utc>, StorageError> {
    value
        .parse::<DateTime<Utc>>()
        .map_err(|_| StorageError::InvalidTimestamp { field, value })
}
