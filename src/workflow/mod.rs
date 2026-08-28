//! Project use cases that coordinate validated domain and persistence operations.

use serde::Deserialize;
use thiserror::Error;

use crate::domain::{
    Confidence, DomainError, FindingKind, Impact, Project, ProjectId, ProjectSnapshot,
    ProjectStatus, RetrievalMode, SourceType, Uncertainty,
};
use crate::storage::{SqliteStore, StorageError};

/// Describes a rejected agent response or a failed authoritative workflow operation.
#[derive(Debug, Error)]
pub enum WorkflowError {
    /// Indicates that workflow-critical JSON did not match the declared schema.
    #[error("structured analysis is invalid: {0}")]
    InvalidStructuredAnalysis(String),
    /// Rejects invalid user input before a project row is created.
    #[error(transparent)]
    Domain(#[from] DomainError),
    /// Propagates a transactional persistence failure with its domain context intact.
    #[error(transparent)]
    Storage(#[from] StorageError),
}

/// Represents one schema-validated statement proposed by an analysis operation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AnalysisFinding {
    pub kind: FindingKind,
    pub statement: String,
    pub impact: Impact,
    #[serde(default = "default_agent_source")]
    pub source_type: SourceType,
    #[serde(default)]
    pub requires_confirmation: bool,
}

/// Contains the complete typed output accepted from an analysis client.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StructuredAnalysis {
    pub findings: Vec<AnalysisFinding>,
}

impl StructuredAnalysis {
    /// Parses JSON and rejects empty or blank findings before persistence can begin.
    pub fn from_json(json: &str) -> Result<Self, WorkflowError> {
        let parsed: Self = serde_json::from_str(json)
            .map_err(|error| WorkflowError::InvalidStructuredAnalysis(error.to_string()))?;
        if parsed.findings.is_empty() {
            return Err(WorkflowError::InvalidStructuredAnalysis(
                "at least one finding is required".to_owned(),
            ));
        }
        if parsed
            .findings
            .iter()
            .any(|finding| finding.statement.trim().is_empty())
        {
            return Err(WorkflowError::InvalidStructuredAnalysis(
                "finding statements must not be blank".to_owned(),
            ));
        }
        Ok(parsed)
    }
}

/// Owns the authoritative store and exposes complete project workflow operations.
pub struct ProjectService {
    store: SqliteStore,
}

impl ProjectService {
    /// Creates a workflow service around an initialized authoritative store.
    pub const fn new(store: SqliteStore) -> Self {
        Self { store }
    }

    /// Creates and analyzes a project with the conservative deterministic offline analyzer.
    pub fn initialize_project(
        &mut self,
        name: &str,
        brief: &str,
    ) -> Result<Project, WorkflowError> {
        let analysis = deterministic_analysis(brief);
        self.initialize_with_analysis(name, brief, analysis)
    }

    /// Validates supplied structured output before creating any authoritative project state.
    pub fn initialize_from_analysis_json(
        &mut self,
        name: &str,
        brief: &str,
        json: &str,
    ) -> Result<Project, WorkflowError> {
        let analysis = StructuredAnalysis::from_json(json)?;
        self.initialize_with_analysis(name, brief, analysis)
    }

    /// Stores a first-class user answer and reconciles its requirement and trace effects.
    pub fn answer_question(
        &mut self,
        question_id: &str,
        answer: &str,
        notes: Option<&str>,
    ) -> Result<(), WorkflowError> {
        self.store.reconcile_answer(question_id, answer, notes)?;
        Ok(())
    }

    /// Loads the consistent read model used by inspection and downstream stages.
    pub fn inspect_project(
        &self,
        project_id: &ProjectId,
    ) -> Result<ProjectSnapshot, WorkflowError> {
        self.store.project_snapshot(project_id).map_err(Into::into)
    }

    /// Counts authoritative projects for validation and diagnostic callers.
    pub fn project_count(&self) -> Result<u64, WorkflowError> {
        self.store.project_count().map_err(Into::into)
    }

    /// Returns ownership of the store for later application composition.
    pub fn into_store(self) -> SqliteStore {
        self.store
    }

    /// Persists one validated analysis and creates only material clarification questions.
    fn initialize_with_analysis(
        &mut self,
        name: &str,
        brief: &str,
        analysis: StructuredAnalysis,
    ) -> Result<Project, WorkflowError> {
        let project = self.store.create_project(crate::domain::NewProject::new(
            name,
            brief,
            RetrievalMode::Relational,
        )?)?;
        self.store
            .transition_project(project.id(), ProjectStatus::Analyzing)?;
        let mut question_count = 0_u32;
        for proposal in analysis.findings {
            let confidence = if proposal.source_type == SourceType::UserBrief {
                Confidence::High
            } else {
                Confidence::Low
            };
            let finding = self.store.add_derived_finding(
                project.id().clone(),
                proposal.kind,
                &proposal.statement,
                proposal.source_type,
                "analysis",
                confidence,
                proposal.impact,
                proposal.requires_confirmation,
            )?;
            if proposal.kind == FindingKind::Unknown {
                let (prompt, rationale) = clarification_copy(&proposal.statement);
                self.store.add_question(
                    project.id(),
                    Some(finding.id().as_str()),
                    &prompt,
                    &rationale,
                    proposal.impact,
                    Uncertainty::High,
                    crate::domain::CostOfBeingWrong::High,
                )?;
                question_count += 1;
            }
        }
        let next = if question_count == 0 {
            ProjectStatus::Planning
        } else {
            ProjectStatus::AwaitingClarification
        };
        self.store.transition_project(project.id(), next)?;
        self.store
            .get_project(project.id())?
            .ok_or_else(|| StorageError::ProjectNotFound(project.id().to_string()).into())
    }
}

/// Returns the provenance default for agent-proposed structured findings.
fn default_agent_source() -> SourceType {
    SourceType::AgentInference
}

/// Produces conservative offline findings and explicit unknowns without domain invention.
fn deterministic_analysis(brief: &str) -> StructuredAnalysis {
    let normalized = brief.trim();
    let lower = normalized.to_ascii_lowercase();
    let mut findings = vec![AnalysisFinding {
        kind: FindingKind::Requirement,
        statement: normalized.to_owned(),
        impact: Impact::Medium,
        source_type: SourceType::UserBrief,
        requires_confirmation: false,
    }];
    if lower.contains("vr")
        && ![
            "meta quest",
            "quest 3",
            "pc vr",
            "steamvr",
            "playstation vr",
        ]
        .iter()
        .any(|platform| lower.contains(platform))
    {
        findings.push(AnalysisFinding {
            kind: FindingKind::Unknown,
            statement: "The primary target VR platform is not specified.".to_owned(),
            impact: Impact::High,
            source_type: SourceType::Derived,
            requires_confirmation: true,
        });
    }
    if lower.contains("vr")
        && !["unity", "unreal", "godot"]
            .iter()
            .any(|engine| lower.contains(engine))
    {
        findings.push(AnalysisFinding {
            kind: FindingKind::Unknown,
            statement: "The required game engine or framework is not specified.".to_owned(),
            impact: Impact::High,
            source_type: SourceType::Derived,
            requires_confirmation: true,
        });
    }
    StructuredAnalysis { findings }
}

/// Converts a typed unknown into concise user-facing clarification copy.
fn clarification_copy(statement: &str) -> (String, String) {
    let lower = statement.to_ascii_lowercase();
    if lower.contains("platform") {
        return (
            "Which VR platform should be the primary target?".to_owned(),
            "Platform choice changes SDKs, input APIs, performance budgets, and distribution."
                .to_owned(),
        );
    }
    if lower.contains("engine") || lower.contains("framework") {
        return (
            "Is a specific engine or framework required?".to_owned(),
            "The engine choice changes implementation constraints, tooling, and deployment."
                .to_owned(),
        );
    }
    (
        format!("Please clarify: {statement}"),
        "This unresolved choice can materially affect later project decisions.".to_owned(),
    )
}
