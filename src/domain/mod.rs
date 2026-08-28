//! Typed project knowledge and lifecycle rules independent of infrastructure.

mod error;
mod finding;
mod project;
mod question;

pub use error::DomainError;
pub use finding::{
    Confidence, Finding, FindingId, FindingKind, FindingStatus, Impact, NewFinding, SourceType,
};
pub use project::{NewProject, Project, ProjectId, ProjectStatus, RetrievalMode};
pub use question::{CostOfBeingWrong, QuestionPriority, Uncertainty};
