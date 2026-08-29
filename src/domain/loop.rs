use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{ProjectId, ProjectStatus};

/// Classifies the assessed trustworthiness of one external evidence source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceReliability {
    /// The source is weak, indirect, or not independently verified.
    Low,
    /// The source is relevant but has material limitations.
    Medium,
    /// The source is primary or otherwise strongly supports the recorded claim.
    High,
}

impl EvidenceReliability {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    /// Parses a checked persisted reliability value.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }
}

impl FromStr for EvidenceReliability {
    type Err = String;

    /// Parses the stable command-line reliability vocabulary.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err(format!(
                "unsupported reliability: {value}; expected low, medium, or high"
            )),
        }
    }
}

/// Stores one externally attributable claim separately from project conclusions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub display_id: String,
    pub project_id: ProjectId,
    pub claim: String,
    pub source: String,
    pub source_title: String,
    pub reliability: EvidenceReliability,
    pub notes: Option<String>,
    pub retrieved_at: DateTime<Utc>,
}

/// Describes the authority lifecycle of one explicit project choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionStatus {
    /// The choice is available for approval but is not yet authoritative.
    Proposed,
    /// Explicit authority accepted the choice.
    Accepted,
    /// Explicit authority rejected the choice.
    Rejected,
    /// A newer recorded choice replaced this history.
    Superseded,
    /// The choice is consequential and cannot be accepted automatically.
    NeedsConfirmation,
}

impl DecisionStatus {
    /// Returns the checked representation stored in SQLite.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Superseded => "superseded",
            Self::NeedsConfirmation => "needs_confirmation",
        }
    }

    /// Parses a checked persisted decision status.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "proposed" => Some(Self::Proposed),
            "accepted" => Some(Self::Accepted),
            "rejected" => Some(Self::Rejected),
            "superseded" => Some(Self::Superseded),
            "needs_confirmation" => Some(Self::NeedsConfirmation),
            _ => None,
        }
    }
}

/// Records one explicit project choice independently of supporting evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub display_id: String,
    pub project_id: ProjectId,
    pub title: String,
    pub statement: String,
    pub rationale: String,
    pub status: DecisionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Selects which proposed decisions interrupt one workflow run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalPolicy {
    /// Requires explicit authority for every proposed decision.
    Strict,
    /// Requires authority only for decisions marked as consequential.
    Consequential,
    /// Continues through supported low-risk proposals without routine interruption.
    Autonomous,
}

impl ApprovalPolicy {
    /// Returns the checked representation stored with a workflow run.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Consequential => "consequential",
            Self::Autonomous => "autonomous",
        }
    }

    /// Parses one checked persisted policy.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "strict" => Some(Self::Strict),
            "consequential" => Some(Self::Consequential),
            "autonomous" => Some(Self::Autonomous),
            _ => None,
        }
    }
}

impl FromStr for ApprovalPolicy {
    type Err = String;

    /// Parses the stable command-line policy vocabulary.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_db_str(value).ok_or_else(|| {
            format!("unsupported approval policy: {value}; expected strict, consequential, or autonomous")
        })
    }
}

/// Describes whether a persisted orchestration run can continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunStatus {
    /// The run is eligible to execute its next planned step.
    Running,
    /// The run is waiting for user authority or unavailable capability.
    Paused,
    /// The run reached a validated terminal package.
    Complete,
    /// An unrecoverable operational failure ended the run.
    Failed,
}

impl WorkflowRunStatus {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Complete => "complete",
            Self::Failed => "failed",
        }
    }

    /// Parses a checked persisted run status.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "running" => Some(Self::Running),
            "paused" => Some(Self::Paused),
            "complete" => Some(Self::Complete),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }

    /// Returns whether another start operation may create a new run.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Complete | Self::Failed)
    }
}

/// Records the durable identity and effective policy of one orchestration attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowRun {
    pub id: String,
    pub project_id: ProjectId,
    pub policy: ApprovalPolicy,
    pub status: WorkflowRunStatus,
    pub pause_reason: Option<String>,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Identifies the single deterministic action currently required by a workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorkflowStep {
    /// Waits for an immutable answer to a consequential question.
    QuestionRequired { question_id: String },
    /// Waits for explicit authority over a proposed decision.
    ApprovalRequired { decision_id: String },
    /// Produces or refreshes the fixed documentation package.
    GeneratePackage,
    /// Checks every authoritative and filesystem completion invariant.
    ValidatePackage,
    /// Reports a validation failure that needs a repair or escalation action.
    RepairRequired { code: String },
    /// Marks the latest authoritative revision as validated and complete.
    Complete,
}

/// Presents the observational state returned to people and automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStatus {
    pub run: WorkflowRun,
    pub step: WorkflowStep,
    pub project_status: ProjectStatus,
    pub latest_validation_passed: bool,
}

/// Distinguishes generated content from a user-owned artifact revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentRevisionSource {
    /// The application or configured agent produced the content.
    Generated,
    /// A person changed the content and registered it as authoritative.
    ManualOverride,
}

impl DocumentRevisionSource {
    /// Returns the checked database representation.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::ManualOverride => "manual_override",
        }
    }

    /// Parses one checked persisted document source.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "generated" => Some(Self::Generated),
            "manual_override" => Some(Self::ManualOverride),
            _ => None,
        }
    }
}

/// Stores one immutable hash-bearing revision of a generated artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentRevision {
    pub id: String,
    pub project_id: ProjectId,
    pub document_id: Option<String>,
    pub relative_path: String,
    pub content_hash: String,
    pub source: DocumentRevisionSource,
    pub previous_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl DocumentRevision {
    /// Returns whether this revision must be protected from ordinary regeneration.
    pub const fn is_manual_override(&self) -> bool {
        matches!(self.source, DocumentRevisionSource::ManualOverride)
    }
}
