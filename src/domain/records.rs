use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Decision, Evidence, Finding, Project, ProjectId, Question, SourceType};

/// Records who supplied a first-class answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerSource {
    /// The answer was entered or confirmed by the user.
    User,
    /// The answer was imported from an explicit external record.
    Imported,
    /// The application produced a mechanical answer with no product judgment.
    System,
}

impl AnswerSource {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Imported => "imported",
            Self::System => "system",
        }
    }

    /// Parses a checked persisted answer source.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "user" => Some(Self::User),
            "imported" => Some(Self::Imported),
            "system" => Some(Self::System),
            _ => None,
        }
    }
}

/// Stores an immutable response to one clarification question.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Answer {
    pub id: String,
    pub display_id: String,
    pub project_id: ProjectId,
    pub question_id: String,
    pub answer_text: String,
    pub source: AnswerSource,
    pub resolves_question: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Describes the lifecycle of an implementation requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementStatus {
    /// The requirement is suggested but still needs authority.
    Proposed,
    /// The requirement participates in current project scope.
    Active,
    /// Evidence or implementation has met the acceptance criteria.
    Satisfied,
    /// A newer traced requirement replaced it.
    Superseded,
    /// User authority rejected it.
    Rejected,
}

impl RequirementStatus {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Active => "active",
            Self::Satisfied => "satisfied",
            Self::Superseded => "superseded",
            Self::Rejected => "rejected",
        }
    }

    /// Parses a checked persisted requirement status.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "proposed" => Some(Self::Proposed),
            "active" => Some(Self::Active),
            "satisfied" => Some(Self::Satisfied),
            "superseded" => Some(Self::Superseded),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
}

/// Represents an implementation-ready need with provenance and acceptance criteria.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub display_id: String,
    pub project_id: ProjectId,
    pub statement: String,
    pub source_type: SourceType,
    pub source_reference: String,
    pub priority: String,
    pub status: RequirementStatus,
    pub rationale: Option<String>,
    pub acceptance_criteria: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines the auditable meaning of an edge in the project knowledge graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceRelationship {
    /// The source increases support for the target.
    Supports,
    /// The source was produced from the target.
    DerivedFrom,
    /// The source directly answers the target question.
    Answers,
    /// The source implements the target requirement or decision.
    Implements,
    /// The source restricts valid interpretations of the target.
    Constrains,
    /// The source is explicitly incompatible with the target.
    Contradicts,
    /// The source replaces the target while retaining history.
    Supersedes,
    /// The source explicitly cites the target evidence.
    Cites,
}

impl TraceRelationship {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Supports => "supports",
            Self::DerivedFrom => "derived_from",
            Self::Answers => "answers",
            Self::Implements => "implements",
            Self::Constrains => "constrains",
            Self::Contradicts => "contradicts",
            Self::Supersedes => "supersedes",
            Self::Cites => "cites",
        }
    }

    /// Parses a checked persisted trace relationship.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "supports" => Some(Self::Supports),
            "derived_from" => Some(Self::DerivedFrom),
            "answers" => Some(Self::Answers),
            "implements" => Some(Self::Implements),
            "constrains" => Some(Self::Constrains),
            "contradicts" => Some(Self::Contradicts),
            "supersedes" => Some(Self::Supersedes),
            "cites" => Some(Self::Cites),
            _ => None,
        }
    }
}

/// Stores one explicit, project-scoped relationship between authoritative entities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceLink {
    pub id: String,
    pub project_id: ProjectId,
    pub source_type: String,
    pub source_id: String,
    pub target_type: String,
    pub target_id: String,
    pub relationship: TraceRelationship,
    pub created_at: DateTime<Utc>,
}

/// Provides a consistent read model for inspection, rendering, and validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectSnapshot {
    pub project: Project,
    pub findings: Vec<Finding>,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
    pub requirements: Vec<Requirement>,
    pub evidence: Vec<Evidence>,
    pub decisions: Vec<Decision>,
    pub traces: Vec<TraceLink>,
}
