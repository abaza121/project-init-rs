use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::normalize_required_text;
use super::{DomainError, ProjectId};

const MAX_FINDING_CHARS: usize = 8_192;

/// Uniquely identifies a finding independently of its stable display identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FindingId(String);

impl FindingId {
    /// Allocates a new internal finding identity.
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Reconstructs an identity already validated by persistence.
    pub(crate) fn from_stored(value: String) -> Self {
        Self(value)
    }

    /// Exposes the durable database representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Distinguishes the semantic role of extracted project knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// A statement explicitly established by an authoritative source.
    ConfirmedFact,
    /// A user need or constraint on observable project behavior.
    Requirement,
    /// A tentative statement that may require confirmation.
    Assumption,
    /// A missing answer that can influence downstream work.
    Unknown,
    /// A boundary that narrows acceptable solutions.
    Constraint,
    /// A potential negative outcome requiring mitigation or acceptance.
    Risk,
    /// A material uncertainty that may be resolved with external evidence.
    ResearchQuestion,
}

impl FindingKind {
    /// Returns the human-readable stable identifier prefix.
    pub const fn display_prefix(self) -> &'static str {
        match self {
            Self::ConfirmedFact => "FACT",
            Self::Requirement => "REQF",
            Self::Assumption => "ASM",
            Self::Unknown => "UNK",
            Self::Constraint => "CON",
            Self::Risk => "RISK",
            Self::ResearchQuestion => "RQ",
        }
    }

    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::ConfirmedFact => "confirmed_fact",
            Self::Requirement => "requirement",
            Self::Assumption => "assumption",
            Self::Unknown => "unknown",
            Self::Constraint => "constraint",
            Self::Risk => "risk",
            Self::ResearchQuestion => "research_question",
        }
    }

    /// Parses a checked persisted kind.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "confirmed_fact" => Some(Self::ConfirmedFact),
            "requirement" => Some(Self::Requirement),
            "assumption" => Some(Self::Assumption),
            "unknown" => Some(Self::Unknown),
            "constraint" => Some(Self::Constraint),
            "risk" => Some(Self::Risk),
            "research_question" => Some(Self::ResearchQuestion),
            _ => None,
        }
    }
}

/// Records where a project statement originated without allowing source promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    /// Direct language from the submitted project brief.
    UserBrief,
    /// A first-class answer supplied or confirmed by the user.
    UserAnswer,
    /// A claim extracted from separately stored research evidence.
    Research,
    /// A model proposal that has not been promoted to user authority.
    AgentInference,
    /// A deterministic conclusion derived from traced records.
    Derived,
    /// A statement introduced by application policy or validation.
    System,
}

impl SourceType {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::UserBrief => "user_brief",
            Self::UserAnswer => "user_answer",
            Self::Research => "research",
            Self::AgentInference => "agent_inference",
            Self::Derived => "derived",
            Self::System => "system",
        }
    }

    /// Parses a checked persisted source type.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "user_brief" => Some(Self::UserBrief),
            "user_answer" => Some(Self::UserAnswer),
            "research" => Some(Self::Research),
            "agent_inference" => Some(Self::AgentInference),
            "derived" => Some(Self::Derived),
            "system" => Some(Self::System),
            _ => None,
        }
    }
}

/// Expresses how strongly the available provenance supports a statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    /// Evidence is weak, incomplete, or explicitly tentative.
    Low,
    /// Evidence is meaningful but still leaves a material chance of error.
    Medium,
    /// Evidence or direct user language strongly supports the statement.
    High,
}

impl Confidence {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    /// Parses a checked persisted confidence value.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }
}

/// Represents the consequence of getting a finding or question wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Impact {
    /// The choice has a localized or cosmetic consequence.
    Low,
    /// The choice affects a meaningful feature or planning dimension.
    Medium,
    /// The choice can change product scope, architecture, safety, or viability.
    High,
}

impl Impact {
    /// Returns the ordinal used by deterministic priority calculation.
    pub(crate) const fn weight(self) -> u16 {
        match self {
            Self::Low => 1,
            Self::Medium => 3,
            Self::High => 5,
        }
    }

    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    /// Parses a checked persisted impact value.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }
}

/// Describes whether a finding participates in current truth or retained history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingStatus {
    /// The finding participates in current project reasoning.
    Active,
    /// The finding has been explicitly confirmed by user authority.
    Confirmed,
    /// The finding has been answered or otherwise closed.
    Resolved,
    /// A newer traced record replaced the finding.
    Superseded,
    /// User authority or validation rejected the finding.
    Rejected,
}

impl FindingStatus {
    /// Returns whether a finding participates in current project reasoning.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active | Self::Confirmed)
    }

    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Confirmed => "confirmed",
            Self::Resolved => "resolved",
            Self::Superseded => "superseded",
            Self::Rejected => "rejected",
        }
    }

    /// Parses a checked persisted status.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "active" => Some(Self::Active),
            "confirmed" => Some(Self::Confirmed),
            "resolved" => Some(Self::Resolved),
            "superseded" => Some(Self::Superseded),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
}

/// Contains validated finding input before storage assigns identity and sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFinding {
    project_id: ProjectId,
    kind: FindingKind,
    statement: String,
    source_type: SourceType,
    source_reference: String,
    confidence: Confidence,
    impact: Impact,
    requires_confirmation: bool,
}

impl NewFinding {
    /// Creates a finding explicitly sourced from the submitted user brief.
    pub fn from_user_brief(
        project_id: ProjectId,
        kind: FindingKind,
        statement: &str,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            project_id,
            kind,
            statement: normalize_required_text(statement, "finding statement", MAX_FINDING_CHARS)?,
            source_type: SourceType::UserBrief,
            source_reference: "project_brief".to_owned(),
            confidence: Confidence::High,
            impact: Impact::Medium,
            requires_confirmation: false,
        })
    }

    /// Creates a validated finding with explicit application-assigned provenance.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        project_id: ProjectId,
        kind: FindingKind,
        statement: &str,
        source_type: SourceType,
        source_reference: &str,
        confidence: Confidence,
        impact: Impact,
        requires_confirmation: bool,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            project_id,
            kind,
            statement: normalize_required_text(statement, "finding statement", MAX_FINDING_CHARS)?,
            source_type,
            source_reference: normalize_required_text(source_reference, "source reference", 1_024)?,
            confidence,
            impact,
            requires_confirmation,
        })
    }

    /// Returns the project partition that owns the finding.
    pub const fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Returns the semantic role of the finding.
    pub const fn kind(&self) -> FindingKind {
        self.kind
    }

    /// Returns the normalized authoritative statement.
    pub fn statement(&self) -> &str {
        &self.statement
    }
}

/// Represents an auditable project statement with provenance and lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    id: FindingId,
    display_id: String,
    project_id: ProjectId,
    kind: FindingKind,
    statement: String,
    source_type: SourceType,
    source_reference: String,
    confidence: Confidence,
    impact: Impact,
    status: FindingStatus,
    requires_confirmation: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Finding {
    /// Allocates identity and timestamps after storage reserves a stable display ID.
    pub(crate) fn from_new(input: NewFinding, display_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: FindingId::new(),
            display_id,
            project_id: input.project_id,
            kind: input.kind,
            statement: input.statement,
            source_type: input.source_type,
            source_reference: input.source_reference,
            confidence: input.confidence,
            impact: input.impact,
            status: FindingStatus::Active,
            requires_confirmation: input.requires_confirmation,
            created_at: now,
            updated_at: now,
        }
    }

    /// Rehydrates a complete persisted finding.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_stored(
        id: FindingId,
        display_id: String,
        project_id: ProjectId,
        kind: FindingKind,
        statement: String,
        source_type: SourceType,
        source_reference: String,
        confidence: Confidence,
        impact: Impact,
        status: FindingStatus,
        requires_confirmation: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            display_id,
            project_id,
            kind,
            statement,
            source_type,
            source_reference,
            confidence,
            impact,
            status,
            requires_confirmation,
            created_at,
            updated_at,
        }
    }

    /// Returns the internal identity used by relational links.
    pub const fn id(&self) -> &FindingId {
        &self.id
    }

    /// Returns the stable human-readable identifier used in documents.
    pub fn display_id(&self) -> &str {
        &self.display_id
    }

    /// Returns the project partition that owns the finding.
    pub const fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Returns the semantic role of the finding.
    pub const fn kind(&self) -> FindingKind {
        self.kind
    }

    /// Returns the authoritative statement.
    pub fn statement(&self) -> &str {
        &self.statement
    }

    /// Returns the provenance category.
    pub const fn source_type(&self) -> SourceType {
        self.source_type
    }

    /// Returns the provenance locator meaningful within its source category.
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }

    /// Returns how strongly provenance supports this statement.
    pub const fn confidence(&self) -> Confidence {
        self.confidence
    }

    /// Returns the consequence associated with the statement.
    pub const fn impact(&self) -> Impact {
        self.impact
    }

    /// Returns whether the finding is current or historical.
    pub const fn status(&self) -> FindingStatus {
        self.status
    }

    /// Returns whether user authority is still required.
    pub const fn requires_confirmation(&self) -> bool {
        self.requires_confirmation
    }

    /// Returns when the finding was created.
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns when the finding last changed.
    pub const fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
