//! Typed project knowledge and lifecycle rules independent of infrastructure.

pub(crate) mod error;
mod finding;
mod r#loop;
mod project;
mod question;
mod records;

pub use error::DomainError;
pub use finding::{
    Confidence, Finding, FindingId, FindingKind, FindingStatus, Impact, NewFinding, SourceType,
};
pub use r#loop::{
    ApprovalPolicy, Decision, DecisionStatus, DocumentRevision, DocumentRevisionSource, Evidence,
    EvidenceReliability, WorkflowRun, WorkflowRunStatus, WorkflowStatus, WorkflowStep,
};
pub use project::{NewProject, Project, ProjectId, ProjectStatus, RetrievalMode};
pub use question::{CostOfBeingWrong, Question, QuestionPriority, QuestionStatus, Uncertainty};
pub use records::{
    Answer, AnswerSource, ProjectSnapshot, Requirement, RequirementStatus, TraceLink,
    TraceRelationship,
};
