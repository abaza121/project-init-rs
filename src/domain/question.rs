use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Impact, ProjectId};

/// Represents how likely available project knowledge is incomplete or wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Uncertainty {
    /// The current statement is strongly supported.
    Low,
    /// Important details remain ambiguous.
    Medium,
    /// The system lacks a reliable answer.
    High,
}

impl Uncertainty {
    /// Returns the ordinal used in the deterministic priority product.
    const fn weight(self) -> u16 {
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

    /// Parses a checked persisted uncertainty value.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }
}

/// Represents the consequence of proceeding with an incorrect answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostOfBeingWrong {
    /// Correction would be localized and inexpensive.
    Low,
    /// Correction would cause meaningful rework.
    Medium,
    /// Correction could invalidate product scope or architecture.
    High,
}

impl CostOfBeingWrong {
    /// Returns the ordinal used in the deterministic priority product.
    const fn weight(self) -> u16 {
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

    /// Parses a checked persisted cost value.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }
}

/// Stores a comparable question score derived from impact, uncertainty, and rework cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QuestionPriority(u16);

impl QuestionPriority {
    /// Calculates the bounded multiplicative priority for a clarification candidate.
    pub const fn new(
        impact: Impact,
        uncertainty: Uncertainty,
        cost_of_being_wrong: CostOfBeingWrong,
    ) -> Self {
        Self(impact.weight() * uncertainty.weight() * cost_of_being_wrong.weight())
    }

    /// Returns the numeric score used for ordering and persistence.
    pub const fn score(self) -> u16 {
        self.0
    }

    /// Determines whether the score meets an inclusive interruption threshold.
    pub const fn requires_attention(self, threshold: u16) -> bool {
        self.0 >= threshold
    }

    /// Rehydrates a score constrained by the database schema.
    pub(crate) const fn from_score(score: u16) -> Self {
        Self(score)
    }
}

/// Describes whether a clarification question still requires attention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionStatus {
    /// The question can be presented to the user.
    Open,
    /// At least one resolving answer has been stored.
    Answered,
    /// The user intentionally postponed the choice.
    Deferred,
    /// A newer question or known answer replaced this prompt.
    Superseded,
}

impl QuestionStatus {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Answered => "answered",
            Self::Deferred => "deferred",
            Self::Superseded => "superseded",
        }
    }

    /// Parses a checked persisted question status.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "open" => Some(Self::Open),
            "answered" => Some(Self::Answered),
            "deferred" => Some(Self::Deferred),
            "superseded" => Some(Self::Superseded),
            _ => None,
        }
    }
}

/// Represents a prioritized clarification tied to an authoritative project unknown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub display_id: String,
    pub project_id: ProjectId,
    pub finding_id: Option<String>,
    pub prompt: String,
    pub rationale: String,
    pub impact: Impact,
    pub uncertainty: Uncertainty,
    pub cost_of_being_wrong: CostOfBeingWrong,
    pub priority: QuestionPriority,
    pub status: QuestionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
