//! Project use cases that coordinate validated domain and persistence operations.

mod runner;

pub use runner::{
    AutoAnswerActor, AutoAnswerProgress, AutoAnswerStage, WorkflowRunOutcome, WorkflowRunStop,
    WorkflowRunner, WorkflowRunnerError, requires_documentation_client,
};

use serde::Deserialize;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use thiserror::Error;

use crate::agents::{ActivityEvent, AgentExecution, JudgedResearchBatch, ResearchedAnswer};
use crate::documents::PackageRenderer;
use crate::domain::{
    ApprovalPolicy, Confidence, CostOfBeingWrong, Decision, DocumentRevision, DomainError,
    Evidence, EvidenceReliability, FindingKind, Impact, NewProject, Project, ProjectId,
    ProjectSnapshot, ProjectStatus, Question, QuestionStatus, RetrievalMode, SourceType,
    Uncertainty, WorkflowRun, WorkflowRunStatus, WorkflowStatus, WorkflowStep,
};
use crate::storage::{AnalyzedFindingInput, ClarificationInput, SqliteStore, StorageError};

const MAX_STRUCTURED_ANALYSIS_FINDINGS: usize = 128;
const MAX_ANALYSIS_STATEMENT_CHARS: usize = 8_192;
const MAX_ARTIFACT_BYTES: u64 = 4 * 1024 * 1024;
pub const WORKBENCH_QUESTION_RATIONALE: &str =
    "Captured by the user in the contextual project workbench.";

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
    /// Reports a generated-artifact filesystem failure without losing workflow state.
    #[error("document operation failed: {0}")]
    Document(String),
    /// Rejects paths outside the fixed artifact registry.
    #[error("invalid project artifact path: {0}")]
    InvalidArtifactPath(String),
    /// Rejects blank workflow records before they consume stable identifiers.
    #[error("invalid workflow input: {0}")]
    InvalidWorkflowInput(String),
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

/// Carries explicit structured input for one user-authored clarification question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AskQuestionRequest {
    prompt: String,
    rationale: String,
    impact: Impact,
    uncertainty: Uncertainty,
    cost: CostOfBeingWrong,
}

impl AskQuestionRequest {
    /// Creates a question request with every priority dimension chosen explicitly.
    pub fn new(
        prompt: &str,
        rationale: &str,
        impact: Impact,
        uncertainty: Uncertainty,
        cost: CostOfBeingWrong,
    ) -> Self {
        Self {
            prompt: prompt.to_owned(),
            rationale: rationale.to_owned(),
            impact,
            uncertainty,
            cost,
        }
    }

    /// Creates a workbench question with visible conservative priority defaults.
    pub fn conservative(prompt: &str) -> Self {
        Self::new(
            prompt,
            WORKBENCH_QUESTION_RATIONALE,
            Impact::Medium,
            Uncertainty::High,
            CostOfBeingWrong::Medium,
        )
    }
}

impl StructuredAnalysis {
    /// Parses JSON, retains the first of matching findings, and rejects unsafe or excessive input.
    pub fn from_json(json: &str) -> Result<Self, WorkflowError> {
        let parsed: Self = serde_json::from_str(json)
            .map_err(|error| WorkflowError::InvalidStructuredAnalysis(error.to_string()))?;
        if parsed.findings.is_empty() {
            return Err(WorkflowError::InvalidStructuredAnalysis(
                "at least one finding is required".to_owned(),
            ));
        }
        if parsed.findings.len() > MAX_STRUCTURED_ANALYSIS_FINDINGS {
            return Err(WorkflowError::InvalidStructuredAnalysis(format!(
                "analysis contains more than {MAX_STRUCTURED_ANALYSIS_FINDINGS} findings"
            )));
        }
        let mut identities = HashSet::with_capacity(parsed.findings.len());
        let mut findings = Vec::with_capacity(parsed.findings.len());
        for finding in parsed.findings {
            let statement = finding.statement.trim();
            if statement.is_empty() {
                return Err(WorkflowError::InvalidStructuredAnalysis(
                    "finding statements must not be blank".to_owned(),
                ));
            }
            if statement.chars().count() > MAX_ANALYSIS_STATEMENT_CHARS {
                return Err(WorkflowError::InvalidStructuredAnalysis(format!(
                    "finding statements must not exceed {MAX_ANALYSIS_STATEMENT_CHARS} characters"
                )));
            }
            let identity = (finding.kind.as_db_str(), statement.to_lowercase());
            if identities.insert(identity) {
                findings.push(finding);
            }
        }
        Ok(Self { findings })
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
        self.initialize_with_analysis(name, brief, analysis, None)
    }

    /// Validates supplied structured output before creating any authoritative project state.
    pub fn initialize_from_analysis_json(
        &mut self,
        name: &str,
        brief: &str,
        json: &str,
    ) -> Result<Project, WorkflowError> {
        let analysis = StructuredAnalysis::from_json(json)?;
        self.initialize_with_analysis(name, brief, analysis, None)
    }

    /// Validates a successful external execution before committing its project and activity.
    pub fn initialize_from_agent_execution(
        &mut self,
        name: &str,
        brief: &str,
        execution: AgentExecution,
    ) -> Result<Project, WorkflowError> {
        let analysis = StructuredAnalysis::from_json(&execution.response)?;
        self.initialize_with_analysis(name, brief, analysis, Some(&execution))
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

    /// Stores one cited provider recommendation without misrepresenting it as user authority.
    pub fn answer_question_from_research(
        &mut self,
        question_id: &str,
        answer: &ResearchedAnswer,
    ) -> Result<(), WorkflowError> {
        self.store.reconcile_researched_answer(
            question_id,
            answer.answer_text(),
            answer.notes(),
            answer.evidence(),
        )?;
        Ok(())
    }

    /// Atomically stores every still-applicable answer from one validated judged batch.
    pub fn answer_questions_from_research(
        &mut self,
        batch: &JudgedResearchBatch,
    ) -> Result<Vec<crate::domain::Answer>, WorkflowError> {
        self.store
            .reconcile_researched_answer_batch(batch)
            .map_err(Into::into)
    }

    /// Stores one explicit user-authored question without inferring a lifecycle transition.
    pub fn ask_question(
        &mut self,
        project_id: &ProjectId,
        request: AskQuestionRequest,
    ) -> Result<Question, WorkflowError> {
        self.store
            .add_question(
                project_id,
                None,
                &request.prompt,
                &request.rationale,
                request.impact,
                request.uncertainty,
                request.cost,
            )
            .map_err(Into::into)
    }

    /// Persists the inclusive priority score used to classify consequential questions.
    pub fn set_clarification_threshold(
        &mut self,
        project_id: &ProjectId,
        value: u16,
    ) -> Result<(), WorkflowError> {
        self.store
            .set_clarification_threshold(project_id, value)
            .map_err(Into::into)
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

    /// Loads the sanitized successful-analysis timeline for inspection or TUI replay.
    pub fn agent_activity(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<ActivityEvent>, WorkflowError> {
        self.store
            .list_agent_activity(project_id)
            .map_err(Into::into)
    }

    /// Starts a new resumable orchestration attempt with an explicit effective policy.
    pub fn start_run(
        &mut self,
        project_id: &ProjectId,
        policy: ApprovalPolicy,
    ) -> Result<WorkflowRun, WorkflowError> {
        self.store
            .start_workflow_run(project_id, policy)
            .map_err(Into::into)
    }

    /// Loads the sole active or paused workflow run without treating absence as an error.
    pub fn active_run(&self, project_id: &ProjectId) -> Result<Option<WorkflowRun>, WorkflowError> {
        self.store
            .active_workflow_run(project_id)
            .map_err(Into::into)
    }

    /// Pauses the current run with a stable sanitized machine-readable reason.
    pub fn pause_run(&mut self, project_id: &ProjectId, reason: &str) -> Result<(), WorkflowError> {
        validate_workflow_text("workflow pause reason", reason)?;
        let run = self.resume_run(project_id)?;
        self.store.set_workflow_run_status(
            &run.id,
            WorkflowRunStatus::Paused,
            Some(reason.trim()),
        )?;
        Ok(())
    }

    /// Stores one explicit evidence claim without treating it as a decision.
    pub fn add_evidence(
        &mut self,
        project_id: &ProjectId,
        claim: &str,
        source: &str,
        source_title: &str,
        reliability: EvidenceReliability,
        notes: Option<&str>,
    ) -> Result<Evidence, WorkflowError> {
        validate_workflow_text("evidence claim", claim)?;
        validate_workflow_text("evidence source", source)?;
        validate_workflow_text("evidence source title", source_title)?;
        let evidence = self.store.insert_evidence(
            project_id,
            claim.trim(),
            source.trim(),
            source_title.trim(),
            reliability,
            notes,
        )?;
        self.store.mark_documents_stale(project_id)?;
        Ok(evidence)
    }

    /// Stores a proposed project decision with explicit consequence classification.
    pub fn propose_decision(
        &mut self,
        project_id: &ProjectId,
        title: &str,
        statement: &str,
        rationale: &str,
        needs_confirmation: bool,
    ) -> Result<Decision, WorkflowError> {
        validate_workflow_text("decision title", title)?;
        validate_workflow_text("decision statement", statement)?;
        validate_workflow_text("decision rationale", rationale)?;
        let decision = self.store.insert_decision(
            project_id,
            title.trim(),
            statement.trim(),
            rationale.trim(),
            needs_confirmation,
        )?;
        self.store.mark_documents_stale(project_id)?;
        Ok(decision)
    }

    /// Records explicit user approval for a proposed decision.
    pub fn approve_decision(
        &mut self,
        decision_id: &str,
        reason: &str,
    ) -> Result<(), WorkflowError> {
        validate_workflow_text("approval reason", reason)?;
        let project_id = self
            .store
            .resolve_decision(decision_id, true, reason.trim())?;
        self.store.mark_documents_stale(&project_id)?;
        Ok(())
    }

    /// Records explicit user rejection while retaining the proposed decision history.
    pub fn reject_decision(
        &mut self,
        decision_id: &str,
        reason: &str,
    ) -> Result<(), WorkflowError> {
        validate_workflow_text("rejection reason", reason)?;
        let project_id = self
            .store
            .resolve_decision(decision_id, false, reason.trim())?;
        self.store.mark_documents_stale(&project_id)?;
        Ok(())
    }

    /// Reloads the sole active or paused run without choosing a new policy.
    pub fn resume_run(&self, project_id: &ProjectId) -> Result<WorkflowRun, WorkflowError> {
        self.store.active_workflow_run(project_id)?.ok_or_else(|| {
            WorkflowError::Storage(StorageError::WorkflowRunNotFound(project_id.to_string()))
        })
    }

    /// Observes the deterministic next action without mutating workflow or filesystem state.
    pub fn workflow_status(
        &self,
        project_id: &ProjectId,
        output: &Path,
    ) -> Result<WorkflowStatus, WorkflowError> {
        let run = self
            .store
            .active_workflow_run(project_id)?
            .or(self.store.latest_workflow_run(project_id)?)
            .ok_or_else(|| {
                WorkflowError::Storage(StorageError::WorkflowRunNotFound(project_id.to_string()))
            })?;
        let snapshot = self.inspect_project(project_id)?;
        let latest_validation_passed = self.store.latest_validation_passed(project_id)?;
        let open_validation_findings = self.store.latest_open_validation_findings(project_id)?;
        let step = if let Some(question) = snapshot.questions.iter().find(|question| {
            question.status == QuestionStatus::Open
                && question
                    .priority
                    .requires_attention(snapshot.project.clarification_threshold())
        }) {
            WorkflowStep::QuestionRequired {
                question_id: question.id.clone(),
            }
        } else if let Some(decision) = self.store.pending_approval(project_id, run.policy)? {
            WorkflowStep::ApprovalRequired {
                decision_id: decision.id,
            }
        } else if !package_is_current(&self.store, &snapshot, output)? {
            WorkflowStep::GeneratePackage
        } else if let Some(finding) = open_validation_findings.first() {
            WorkflowStep::RepairRequired {
                code: finding.code.clone(),
            }
        } else if latest_validation_passed {
            WorkflowStep::Complete
        } else {
            WorkflowStep::ValidatePackage
        };
        Ok(WorkflowStatus {
            run,
            step,
            project_status: snapshot.project.status(),
            latest_validation_passed,
        })
    }

    /// Observes workflow status only when the project has at least one persisted run.
    pub fn workflow_status_if_started(
        &self,
        project_id: &ProjectId,
        output: &Path,
    ) -> Result<Option<WorkflowStatus>, WorkflowError> {
        let exists = self.store.active_workflow_run(project_id)?.is_some()
            || self.store.latest_workflow_run(project_id)?.is_some();
        if exists {
            self.workflow_status(project_id, output).map(Some)
        } else {
            Ok(None)
        }
    }

    /// Executes at most one deterministic local workflow mutation and returns the next action.
    pub fn execute_offline_step(
        &mut self,
        project_id: &ProjectId,
        output: &Path,
    ) -> Result<WorkflowStatus, WorkflowError> {
        let current = self.workflow_status(project_id, output)?;
        match current.step {
            WorkflowStep::QuestionRequired { .. } | WorkflowStep::ApprovalRequired { .. } => {
                self.store.set_workflow_run_status(
                    &current.run.id,
                    WorkflowRunStatus::Paused,
                    Some("user_input_required"),
                )?;
                self.workflow_status(project_id, output)
            }
            WorkflowStep::GeneratePackage => {
                let snapshot = self.inspect_project(project_id)?;
                self.capture_manual_edits(project_id, &snapshot, output)?;
                transition_to_generation(&mut self.store, project_id, snapshot.project.status())?;
                let protected = self.store.manual_override_paths(project_id)?;
                let paths = PackageRenderer::render_preserving(&snapshot, output, &protected)
                    .map_err(|error| WorkflowError::Document(error.to_string()))?;
                let mut documents = Vec::with_capacity(paths.len());
                for path in paths {
                    let relative = path
                        .strip_prefix(output)
                        .map_err(|_| {
                            WorkflowError::InvalidArtifactPath(path.display().to_string())
                        })?
                        .to_string_lossy()
                        .replace('\\', "/");
                    documents.push((relative, hash_file(&path)?));
                }
                self.store
                    .persist_generated_documents(project_id, &documents, &protected)?;
                self.store
                    .transition_project(project_id, ProjectStatus::Validating)?;
                self.store.set_workflow_run_status(
                    &current.run.id,
                    WorkflowRunStatus::Running,
                    None,
                )?;
                self.workflow_status(project_id, output)
            }
            WorkflowStep::ValidatePackage | WorkflowStep::RepairRequired { .. } => {
                self.execute_validation_step(project_id, output, current)
            }
            WorkflowStep::Complete => Ok(current),
        }
    }

    /// Authorizes exactly one future repair attempt at the current persisted repair boundary.
    pub fn request_repair(
        &mut self,
        project_id: &ProjectId,
        output: &Path,
    ) -> Result<Vec<ValidationFinding>, WorkflowError> {
        let status = self.workflow_status(project_id, output)?;
        if !matches!(status.step, WorkflowStep::RepairRequired { .. }) {
            return Err(WorkflowError::InvalidWorkflowInput(
                "repair can be requested only for a persisted repair boundary".to_owned(),
            ));
        }
        let stored = self.store.latest_open_validation_findings(project_id)?;
        if stored.is_empty() || stored.iter().any(|finding| !finding.repairable) {
            return Err(WorkflowError::InvalidWorkflowInput(
                "the current validation findings require user authority rather than document repair"
                    .to_owned(),
            ));
        }
        let repair_pass = self.store.latest_validation_repair_pass(project_id)?;
        let repair_limit = self.store.project_repair_limit(project_id)?;
        if repair_pass >= repair_limit {
            return Err(WorkflowError::InvalidWorkflowInput(format!(
                "repair limit of {repair_limit} has been reached"
            )));
        }
        self.store.set_workflow_run_status(
            &status.run.id,
            WorkflowRunStatus::Paused,
            Some("repair_requested"),
        )?;
        Ok(stored
            .into_iter()
            .map(|finding| ValidationFinding {
                code: finding.code,
                severity: finding.severity,
                message: finding.message,
            })
            .collect())
    }

    /// Loads the latest persisted validation findings for guided workbench feedback.
    pub fn latest_validation_findings(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<ValidationFinding>, WorkflowError> {
        self.store
            .latest_open_validation_findings(project_id)?
            .into_iter()
            .map(|finding| {
                Ok(ValidationFinding {
                    code: finding.code,
                    severity: finding.severity,
                    message: finding.message,
                })
            })
            .collect()
    }

    /// Adopts a complete provisional provider package without overwriting protected overrides.
    pub fn adopt_generated_package(
        &mut self,
        project_id: &ProjectId,
        staging: &Path,
        output: &Path,
    ) -> Result<WorkflowStatus, WorkflowError> {
        let current = self.workflow_status(project_id, output)?;
        let repair = matches!(current.step, WorkflowStep::RepairRequired { .. })
            && current.run.pause_reason.as_deref() == Some("repair_requested");
        if current.step != WorkflowStep::GeneratePackage && !repair {
            return Err(WorkflowError::Document(
                "the current workflow step is not package generation".to_owned(),
            ));
        }
        let snapshot = self.inspect_project(project_id)?;
        for relative in PackageRenderer::required_relative_paths(&snapshot) {
            let staged = staging.join(&relative);
            if !staged.is_file() {
                return Err(WorkflowError::Document(format!(
                    "provider package is missing required artifact: {relative}"
                )));
            }
            let _ = hash_file(&staged)?;
        }
        self.capture_manual_edits(project_id, &snapshot, output)?;
        transition_to_generation(&mut self.store, project_id, snapshot.project.status())?;
        fs::create_dir_all(output).map_err(|error| {
            WorkflowError::Document(format!("could not create {}: {error}", output.display()))
        })?;
        let protected = self.store.manual_override_paths(project_id)?;
        let mut documents = Vec::new();
        for relative in PackageRenderer::required_relative_paths(&snapshot) {
            let staged = staging.join(&relative);
            let destination = output.join(&relative);
            if !protected.contains(&relative) {
                fs::copy(&staged, &destination).map_err(|error| {
                    WorkflowError::Document(format!(
                        "could not adopt {}: {error}",
                        destination.display()
                    ))
                })?;
            }
            documents.push((relative, hash_file(&destination)?));
        }
        self.store
            .persist_generated_documents(project_id, &documents, &protected)?;
        if repair {
            self.store
                .mark_latest_validation_findings_repaired(project_id)?;
        }
        self.store
            .transition_project(project_id, ProjectStatus::Validating)?;
        self.store.set_workflow_run_status(
            &current.run.id,
            WorkflowRunStatus::Running,
            repair.then_some("repair_pending_validation"),
        )?;
        self.workflow_status(project_id, output)
    }

    /// Registers one expected artifact as a user-owned revision without changing its bytes.
    pub fn register_override(
        &mut self,
        project_id: &ProjectId,
        output: &Path,
        relative_path: &str,
    ) -> Result<(), WorkflowError> {
        let snapshot = self.inspect_project(project_id)?;
        if !PackageRenderer::required_relative_paths(&snapshot)
            .iter()
            .any(|expected| expected == relative_path)
        {
            return Err(WorkflowError::InvalidArtifactPath(relative_path.to_owned()));
        }
        self.store.persist_document_override(
            project_id,
            relative_path,
            &hash_file(&output.join(relative_path))?,
        )?;
        self.store.mark_document_paths_stale(
            project_id,
            &PackageRenderer::dependent_relative_paths(&snapshot, relative_path),
        )?;
        Ok(())
    }

    /// Returns one protected artifact to generated control without deleting revision history.
    pub fn remove_override(
        &mut self,
        project_id: &ProjectId,
        relative_path: &str,
    ) -> Result<(), WorkflowError> {
        self.store
            .release_document_override(project_id, relative_path)
            .map_err(Into::into)
    }

    /// Loads immutable generated and manual revisions for one expected artifact.
    pub fn document_revisions(
        &self,
        project_id: &ProjectId,
        relative_path: &str,
    ) -> Result<Vec<DocumentRevision>, WorkflowError> {
        self.store
            .list_document_revisions(project_id, relative_path)
            .map_err(Into::into)
    }

    /// Persists one validation result and applies its legal terminal lifecycle transition.
    fn execute_validation_step(
        &mut self,
        project_id: &ProjectId,
        output: &Path,
        current: WorkflowStatus,
    ) -> Result<WorkflowStatus, WorkflowError> {
        let snapshot = self.inspect_project(project_id)?;
        if !package_is_current(&self.store, &snapshot, output)? {
            return self.workflow_status(project_id, output);
        }
        let report = validate_snapshot(&snapshot, output);
        let findings = report
            .findings
            .iter()
            .map(|finding| {
                (
                    finding.code.clone(),
                    finding.severity.clone(),
                    finding.message.clone(),
                )
            })
            .collect::<Vec<_>>();
        let repair_pass =
            if current.run.pause_reason.as_deref() == Some("repair_pending_validation") {
                self.store
                    .latest_validation_repair_pass(project_id)?
                    .saturating_add(1)
            } else {
                0
            };
        self.store
            .persist_validation_result(project_id, report.passed, repair_pass, &findings)?;
        if report.passed {
            self.store
                .transition_project(project_id, ProjectStatus::Complete)?;
            self.store.set_workflow_run_status(
                &current.run.id,
                WorkflowRunStatus::Complete,
                None,
            )?;
            let mut run = current.run;
            run.status = WorkflowRunStatus::Complete;
            run.pause_reason = None;
            run.completed_at = Some(chrono::Utc::now());
            Ok(WorkflowStatus {
                run,
                step: WorkflowStep::Complete,
                project_status: ProjectStatus::Complete,
                latest_validation_passed: true,
            })
        } else {
            self.store
                .transition_project(project_id, ProjectStatus::NeedsUserInput)?;
            self.store.set_workflow_run_status(
                &current.run.id,
                WorkflowRunStatus::Paused,
                Some("validation_failed"),
            )?;
            let mut run = current.run;
            run.status = WorkflowRunStatus::Paused;
            run.pause_reason = Some("validation_failed".to_owned());
            Ok(WorkflowStatus {
                run,
                step: WorkflowStep::RepairRequired {
                    code: report.findings.first().map_or_else(
                        || "VALIDATION_FAILED".to_owned(),
                        |finding| finding.code.clone(),
                    ),
                },
                project_status: ProjectStatus::NeedsUserInput,
                latest_validation_passed: false,
            })
        }
    }

    /// Converts unregistered hash changes into protected revisions before regeneration begins.
    fn capture_manual_edits(
        &mut self,
        project_id: &ProjectId,
        snapshot: &ProjectSnapshot,
        output: &Path,
    ) -> Result<(), WorkflowError> {
        let stored = self.store.stored_documents(project_id)?;
        let protected = self.store.manual_override_paths(project_id)?;
        for relative in PackageRenderer::required_relative_paths(snapshot) {
            let Some(document) = stored.get(&relative) else {
                continue;
            };
            if document.status != "current" || protected.contains(&relative) {
                continue;
            }
            let path = output.join(&relative);
            if path.is_file() {
                let actual = hash_file(&path)?;
                if actual != document.content_hash {
                    self.store
                        .persist_document_override(project_id, &relative, &actual)?;
                    self.store.mark_document_paths_stale(
                        project_id,
                        &PackageRenderer::dependent_relative_paths(snapshot, &relative),
                    )?;
                }
            }
        }
        Ok(())
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
        execution: Option<&AgentExecution>,
    ) -> Result<Project, WorkflowError> {
        let prepared = prepare_analysis(brief, analysis, execution.is_some());
        self.store
            .create_analyzed_project(
                NewProject::new(name, brief, RetrievalMode::Relational)?,
                prepared,
                execution,
            )
            .map_err(Into::into)
    }
}

/// Returns whether every required artifact is current and matches its recorded content hash.
fn package_is_current(
    store: &SqliteStore,
    snapshot: &ProjectSnapshot,
    output: &Path,
) -> Result<bool, WorkflowError> {
    let stored = store.stored_documents(snapshot.project.id())?;
    for relative in PackageRenderer::required_relative_paths(snapshot) {
        let Some(document) = stored.get(&relative) else {
            return Ok(false);
        };
        if document.status != "current" {
            return Ok(false);
        }
        let path = output.join(&relative);
        if !path.is_file() || hash_file(&path)? != document.content_hash {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Hashes one artifact with the same stable digest stored in document metadata.
fn hash_file(path: &Path) -> Result<String, WorkflowError> {
    let metadata = fs::metadata(path).map_err(|error| {
        WorkflowError::Document(format!("could not inspect {}: {error}", path.display()))
    })?;
    if metadata.len() > MAX_ARTIFACT_BYTES {
        return Err(WorkflowError::Document(format!(
            "artifact exceeds {MAX_ARTIFACT_BYTES} bytes: {}",
            path.display()
        )));
    }
    let bytes = fs::read(path).map_err(|error| {
        WorkflowError::Document(format!("could not read {}: {error}", path.display()))
    })?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

/// Applies only the lifecycle edge needed before a generation retry.
fn transition_to_generation(
    store: &mut SqliteStore,
    project_id: &ProjectId,
    status: ProjectStatus,
) -> Result<(), WorkflowError> {
    match status {
        ProjectStatus::Planning | ProjectStatus::Validating | ProjectStatus::Complete => {
            store.transition_project(project_id, ProjectStatus::Generating)?;
        }
        ProjectStatus::Generating => {}
        _ => {
            return Err(WorkflowError::Document(format!(
                "project cannot generate from {status}"
            )));
        }
    }
    Ok(())
}

/// Rejects blank user or agent workflow record fields before persistence begins.
fn validate_workflow_text(field: &str, value: &str) -> Result<(), WorkflowError> {
    if value.trim().is_empty() {
        return Err(WorkflowError::InvalidWorkflowInput(format!(
            "{field} must not be blank"
        )));
    }
    Ok(())
}

/// Assigns application-controlled provenance and clarification copy before persistence begins.
fn prepare_analysis(
    brief: &str,
    analysis: StructuredAnalysis,
    agent_generated: bool,
) -> Vec<AnalyzedFindingInput> {
    let normalized_brief = brief.to_lowercase();
    analysis
        .findings
        .into_iter()
        .map(|proposal| {
            let source_type = assigned_source(&normalized_brief, &proposal, agent_generated);
            let confidence = if source_type == SourceType::UserBrief {
                Confidence::High
            } else {
                Confidence::Low
            };
            let clarification = (proposal.kind == FindingKind::Unknown).then(|| {
                let (prompt, rationale) = clarification_copy(&proposal.statement);
                ClarificationInput { prompt, rationale }
            });
            AnalyzedFindingInput {
                kind: proposal.kind,
                statement: proposal.statement,
                source_type,
                confidence,
                impact: proposal.impact,
                requires_confirmation: proposal.requires_confirmation
                    || (agent_generated && source_type != SourceType::UserBrief),
                clarification,
            }
        })
        .collect()
}

/// Resolves provenance from application context instead of trusting a model-declared source.
fn assigned_source(
    normalized_brief: &str,
    proposal: &AnalysisFinding,
    agent_generated: bool,
) -> SourceType {
    if !agent_generated {
        return proposal.source_type;
    }
    if proposal.kind == FindingKind::Unknown {
        return SourceType::Derived;
    }
    let statement = proposal.statement.trim().to_lowercase();
    if !statement.is_empty() && normalized_brief.contains(&statement) {
        SourceType::UserBrief
    } else {
        SourceType::AgentInference
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
            "Which platform should be the primary target for this project?".to_owned(),
            "Platform choice can change implementation constraints, performance budgets, testing, and distribution."
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

/// Represents one deterministic package or traceability validation failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationFinding {
    pub code: String,
    pub severity: String,
    pub message: String,
}

/// Summarizes whether a project package can safely be considered complete.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationReport {
    pub passed: bool,
    pub findings: Vec<ValidationFinding>,
}

/// Validates high-impact uncertainty, requirement support, and required artifacts.
pub fn validate_snapshot(snapshot: &ProjectSnapshot, output: &Path) -> ValidationReport {
    let mut findings = Vec::new();
    for question in &snapshot.questions {
        if question.status == crate::domain::QuestionStatus::Open
            && question.priority.requires_attention(75)
        {
            findings.push(ValidationFinding {
                code: "UNRESOLVED_HIGH_QUESTION".to_owned(),
                severity: "high".to_owned(),
                message: format!("{} remains unresolved", question.display_id),
            });
        }
    }
    for requirement in &snapshot.requirements {
        let traced = snapshot
            .traces
            .iter()
            .any(|trace| trace.source_id == requirement.id);
        if requirement.acceptance_criteria.trim().is_empty() || !traced {
            findings.push(ValidationFinding {
                code: "UNSUPPORTED_REQUIREMENT".to_owned(),
                severity: "high".to_owned(),
                message: format!(
                    "{} lacks acceptance criteria or provenance",
                    requirement.display_id
                ),
            });
        }
    }
    for required in PackageRenderer::required_relative_paths(snapshot) {
        if !output.join(&required).is_file() {
            findings.push(ValidationFinding {
                code: "MISSING_ARTIFACT".to_owned(),
                severity: "high".to_owned(),
                message: format!("required artifact is missing: {required}"),
            });
        }
    }
    ValidationReport {
        passed: findings.is_empty(),
        findings,
    }
}

#[cfg(test)]
mod tests {
    /// Keeps platform clarification scoped to the supplied project domain.
    #[test]
    fn clarification_copy_does_not_invent_vr_for_a_browser_project() {
        let (prompt, rationale) =
            super::clarification_copy("The primary target browser platform is not specified.");

        assert!(prompt.contains("platform"));
        assert!(!prompt.to_ascii_lowercase().contains("vr"));
        assert!(!rationale.to_ascii_lowercase().contains("sdk"));
    }
}
