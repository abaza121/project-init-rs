use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::DomainError;
use super::error::normalize_required_text;

const MAX_PROJECT_NAME_CHARS: usize = 160;
const MAX_BRIEF_CHARS: usize = 65_536;

/// Uniquely identifies a project independently of its human-readable name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectId(String);

impl ProjectId {
    /// Allocates a new non-guessable project identity.
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Reconstructs an identity that has already been validated by persistence.
    pub(crate) fn from_stored(value: String) -> Self {
        Self(value)
    }

    /// Wraps a CLI identity for an authoritative database lookup.
    pub fn from_cli(value: &str) -> Self {
        Self(value.to_owned())
    }

    /// Exposes the stable database and export representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for ProjectId {
    /// Allocates a new identity for callers that need a default-generated project key.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ProjectId {
    /// Writes the stable textual identifier used by CLI commands.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Selects deterministic relational context or derived semantic assistance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalMode {
    /// Uses only authoritative SQLite relationships and deterministic queries.
    Relational,
    /// Adds derived semantic candidates while retaining SQLite authority.
    Semantic,
}

impl RetrievalMode {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Relational => "relational",
            Self::Semantic => "semantic",
        }
    }

    /// Parses a checked database value without accepting new magic strings.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "relational" => Some(Self::Relational),
            "semantic" => Some(Self::Semantic),
            _ => None,
        }
    }
}

/// Represents the resumable stage of the project-initiation workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    /// The brief exists but analysis has not begun.
    Draft,
    /// Structured findings are being extracted and validated.
    Analyzing,
    /// Consequential questions are ready for user review.
    AwaitingClarification,
    /// Evidence is being gathered for material uncertainties.
    Researching,
    /// Requirements and decisions are being reconciled into a plan.
    Planning,
    /// Documents are being rendered from authoritative state.
    Generating,
    /// The generated package and trace graph are being checked.
    Validating,
    /// A consequential ambiguity or failure requires user authority.
    NeedsUserInput,
    /// Validation has passed with no unresolved high-severity finding.
    Complete,
}

impl ProjectStatus {
    /// Determines whether a requested lifecycle edge preserves required workflow ordering.
    const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Draft, Self::Analyzing)
                | (
                    Self::Analyzing,
                    Self::AwaitingClarification | Self::Planning | Self::NeedsUserInput
                )
                | (
                    Self::AwaitingClarification,
                    Self::Researching | Self::Planning | Self::NeedsUserInput
                )
                | (Self::Researching, Self::Planning | Self::NeedsUserInput)
                | (Self::Planning, Self::Generating | Self::NeedsUserInput)
                | (Self::Generating, Self::Validating)
                | (
                    Self::Validating,
                    Self::Complete | Self::Generating | Self::NeedsUserInput
                )
                | (Self::NeedsUserInput, Self::AwaitingClarification)
        )
    }

    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Analyzing => "analyzing",
            Self::AwaitingClarification => "awaiting_clarification",
            Self::Researching => "researching",
            Self::Planning => "planning",
            Self::Generating => "generating",
            Self::Validating => "validating",
            Self::NeedsUserInput => "needs_user_input",
            Self::Complete => "complete",
        }
    }

    /// Parses a checked database value without accepting unknown lifecycle states.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "draft" => Some(Self::Draft),
            "analyzing" => Some(Self::Analyzing),
            "awaiting_clarification" => Some(Self::AwaitingClarification),
            "researching" => Some(Self::Researching),
            "planning" => Some(Self::Planning),
            "generating" => Some(Self::Generating),
            "validating" => Some(Self::Validating),
            "needs_user_input" => Some(Self::NeedsUserInput),
            "complete" => Some(Self::Complete),
            _ => None,
        }
    }
}

impl fmt::Display for ProjectStatus {
    /// Writes a concise lifecycle name for diagnostics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_db_str())
    }
}

/// Contains validated user input before storage allocates identity and timestamps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewProject {
    name: String,
    brief: String,
    retrieval_mode: RetrievalMode,
}

impl NewProject {
    /// Validates and normalizes the minimum input needed to create a project.
    pub fn new(
        name: &str,
        brief: &str,
        retrieval_mode: RetrievalMode,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            name: normalize_required_text(name, "project name", MAX_PROJECT_NAME_CHARS)?,
            brief: normalize_required_text(brief, "project brief", MAX_BRIEF_CHARS)?,
            retrieval_mode,
        })
    }

    /// Returns the normalized project name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the normalized source brief.
    pub fn brief(&self) -> &str {
        &self.brief
    }

    /// Returns the selected retrieval mode.
    pub const fn retrieval_mode(&self) -> RetrievalMode {
        self.retrieval_mode
    }
}

/// Owns the authoritative project brief and its explicit resumable lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    id: ProjectId,
    name: String,
    brief: String,
    status: ProjectStatus,
    retrieval_mode: RetrievalMode,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Project {
    /// Creates a validated draft project for callers that do not need persistence yet.
    pub fn new(
        name: &str,
        brief: &str,
        retrieval_mode: RetrievalMode,
    ) -> Result<Self, DomainError> {
        Ok(Self::from_new(NewProject::new(
            name,
            brief,
            retrieval_mode,
        )?))
    }

    /// Allocates identity and audit timestamps for validated project input.
    pub(crate) fn from_new(input: NewProject) -> Self {
        let now = Utc::now();
        Self {
            id: ProjectId::new(),
            name: input.name,
            brief: input.brief,
            status: ProjectStatus::Draft,
            retrieval_mode: input.retrieval_mode,
            created_at: now,
            updated_at: now,
        }
    }

    /// Rehydrates a project row whose checked fields were validated by storage.
    pub(crate) fn from_stored(
        id: ProjectId,
        name: String,
        brief: String,
        status: ProjectStatus,
        retrieval_mode: RetrievalMode,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            brief,
            status,
            retrieval_mode,
            created_at,
            updated_at,
        }
    }

    /// Returns the durable project identity.
    pub const fn id(&self) -> &ProjectId {
        &self.id
    }

    /// Returns the user-visible project name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the normalized source brief.
    pub fn brief(&self) -> &str {
        &self.brief
    }

    /// Returns the currently persisted lifecycle state.
    pub const fn status(&self) -> ProjectStatus {
        self.status
    }

    /// Returns whether context selection is relational or semantic-assisted.
    pub const fn retrieval_mode(&self) -> RetrievalMode {
        self.retrieval_mode
    }

    /// Returns when this project was first created.
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns when this project was last changed.
    pub const fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Applies a legal lifecycle transition without changing state on rejection.
    pub fn transition_to(&mut self, next: ProjectStatus) -> Result<(), DomainError> {
        if !self.status.can_transition_to(next) {
            return Err(DomainError::InvalidProjectTransition {
                from: self.status.to_string(),
                to: next.to_string(),
            });
        }
        self.status = next;
        self.updated_at = Utc::now();
        Ok(())
    }
}
