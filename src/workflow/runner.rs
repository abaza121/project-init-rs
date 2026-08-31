use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::Serialize;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

use crate::agents::{
    ActivityEvent, ActivityKind, AgentError, AutoAnswerClient, CancellationToken,
    DocumentationClient, DocumentationRequest, JudgedResearchBatch, ResearchBatchPlan,
    ResearchCandidate, ResearchClient, ResearchJudgmentRequest, ResearchPlanRequest,
    ResearchQuestionContext, ResearchRequest, sanitize_terminal_text,
};
use crate::documents::PackageRenderer;
use crate::domain::{ApprovalPolicy, ProjectId, QuestionStatus, WorkflowStatus, WorkflowStep};

use super::{ProjectService, WorkflowError};

const MAX_RUNNER_STEPS: usize = 64;
const AUTO_ANSWER_WORKER_COUNT: usize = 3;
const AUTO_ANSWER_ATTEMPTS: usize = 2;

/// Identifies one stable lane in the interactive automatic-answer progress board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutoAnswerActor {
    /// Selects a dependency-safe subset from all eligible consequential questions.
    Coordinator,
    /// Researches one question in a fixed numbered worker slot.
    Worker {
        /// Uses one-based numbering for direct terminal presentation.
        slot: usize,
        /// Shows the stable question display identity without exposing raw content.
        question_display_id: String,
    },
    /// Reviews all provisional worker candidates as one project-aware batch.
    Judge,
}

/// Describes an observable automatic-answer stage without inventing completion percentages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoAnswerStage {
    /// The coordinator is selecting independent eligible questions.
    Planning,
    /// A worker has been selected but has not started provider work.
    Queued,
    /// A worker is gathering and validating cited evidence.
    Researching,
    /// A failed worker or judge operation is making its one allowed retry.
    Retrying,
    /// A worker produced a structurally valid cited candidate.
    Ready,
    /// The judge is blocked until every worker has stopped.
    Waiting,
    /// The judge is checking evidence and project-wide consistency.
    Reviewing,
    /// Valid judged answers are entering the authoritative transaction.
    Committing,
    /// The actor completed its current responsibility successfully.
    Accepted,
    /// The actor stopped because its bounded attempts failed.
    Failed,
    /// A no-longer-applicable question was intentionally not adopted.
    Skipped,
}

impl AutoAnswerStage {
    /// Returns the concise label used by the workbench lane renderer.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Planning => "Planning",
            Self::Queued => "Queued",
            Self::Researching => "Researching",
            Self::Retrying => "Retrying",
            Self::Ready => "Evidence ready",
            Self::Waiting => "Waiting",
            Self::Reviewing => "Reviewing",
            Self::Committing => "Committing",
            Self::Accepted => "Accepted",
            Self::Failed => "Failed",
            Self::Skipped => "Skipped",
        }
    }
}

/// Carries one bounded actor-keyed update from runner orchestration to the TUI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoAnswerProgress {
    pub batch: u32,
    pub actor: AutoAnswerActor,
    pub stage: AutoAnswerStage,
    pub detail: String,
    pub evidence_count: Option<usize>,
}

impl AutoAnswerProgress {
    /// Creates a sanitized progress update suitable for direct terminal rendering.
    pub fn new(
        batch: u32,
        actor: AutoAnswerActor,
        stage: AutoAnswerStage,
        detail: &str,
        evidence_count: Option<usize>,
    ) -> Self {
        Self {
            batch,
            actor,
            stage,
            detail: sanitize_terminal_text(detail),
            evidence_count,
        }
    }
}

/// Reports whether the next planned step is an authorized provider-backed operation.
pub fn requires_documentation_client(status: &WorkflowStatus) -> bool {
    matches!(status.step, WorkflowStep::GeneratePackage)
        || matches!(status.step, WorkflowStep::RepairRequired { .. })
            && status.run.pause_reason.as_deref() == Some("repair_requested")
}

/// Identifies why a shared workflow runner returned control to its caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowRunStop {
    /// The next operation requires a question, decision, or repair authority response.
    Boundary,
    /// The latest authoritative package passed validation and the run is terminal.
    Complete,
    /// A cancellation request was acknowledged before provisional files were adopted.
    Cancelled,
}

/// Returns the final observed workflow status together with its stop classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowRunOutcome {
    pub status: WorkflowStatus,
    pub stop: WorkflowRunStop,
}

/// Describes failures specific to composing multiple workflow steps and a provider client.
#[derive(Debug, Error)]
pub enum WorkflowRunnerError {
    /// Requires callers to choose authority policy before the first run is created.
    #[error("an explicit approval policy is required before starting the workflow")]
    PolicyRequired,
    /// Reports a bounded loop that did not reach a stable stop condition.
    #[error("workflow exceeded the bounded step limit")]
    StepLimitExceeded,
    /// Preserves a typed workflow or persistence failure.
    #[error(transparent)]
    Workflow(#[from] WorkflowError),
    /// Preserves a typed external documentation-client failure.
    #[error(transparent)]
    Agent(#[from] AgentError),
    /// Reports serialization that failed before any external execution began.
    #[error("could not serialize workflow context: {0}")]
    Serialization(String),
}

/// Selects deterministic local generation or one provider-backed staged client.
enum RunnerMode<'client> {
    /// Executes package generation through existing deterministic service operations.
    Offline,
    /// Executes package generation in staging through the supplied external client.
    Online(&'client dyn DocumentationClient),
}

/// Drives one project through deterministic steps until an explicit stable boundary.
pub struct WorkflowRunner<'client> {
    service: ProjectService,
    output: PathBuf,
    mode: RunnerMode<'client>,
    research_client: Option<&'client dyn ResearchClient>,
    auto_answer_client: Option<Arc<dyn AutoAnswerClient>>,
    next_activity_sequence: u32,
    next_auto_answer_batch: u32,
}

impl<'client> WorkflowRunner<'client> {
    /// Creates a runner that delegates provisional package work to an online client.
    pub fn online(
        service: ProjectService,
        output: &Path,
        client: &'client dyn DocumentationClient,
    ) -> Self {
        Self {
            service,
            output: output.to_path_buf(),
            mode: RunnerMode::Online(client),
            research_client: None,
            auto_answer_client: None,
            next_activity_sequence: 1,
            next_auto_answer_batch: 1,
        }
    }

    /// Creates a runner that uses deterministic local generation without a provider.
    pub fn offline(service: ProjectService, output: &Path) -> Self {
        Self {
            service,
            output: output.to_path_buf(),
            mode: RunnerMode::Offline,
            research_client: None,
            auto_answer_client: None,
            next_activity_sequence: 1,
            next_auto_answer_batch: 1,
        }
    }

    /// Enables explicit automatic research without changing the default user-authority boundary.
    pub fn with_research_client(mut self, client: &'client dyn ResearchClient) -> Self {
        self.research_client = Some(client);
        self
    }

    /// Enables fixed three-worker automatic research and batch judgment.
    pub fn with_auto_answer_client(mut self, client: Arc<dyn AutoAnswerClient>) -> Self {
        self.auto_answer_client = Some(client);
        self
    }

    /// Borrows the underlying service for status inspection after execution.
    pub const fn service(&self) -> &ProjectService {
        &self.service
    }

    /// Runs generation and validation until completion or persisted user authority is required.
    pub async fn run_until_pause(
        &mut self,
        project_id: &ProjectId,
        policy: Option<ApprovalPolicy>,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<WorkflowRunOutcome, WorkflowRunnerError> {
        self.run_until_pause_with_auto_answer_progress(
            project_id,
            policy,
            activity,
            None,
            cancellation,
        )
        .await
    }

    /// Runs the shared workflow while optionally streaming structured automatic-answer lanes.
    pub async fn run_until_pause_with_auto_answer_progress(
        &mut self,
        project_id: &ProjectId,
        policy: Option<ApprovalPolicy>,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        auto_answer_progress: Option<mpsc::Sender<AutoAnswerProgress>>,
        cancellation: CancellationToken,
    ) -> Result<WorkflowRunOutcome, WorkflowRunnerError> {
        self.ensure_run(project_id, policy)?;
        for _ in 0..MAX_RUNNER_STEPS {
            if cancellation.is_cancelled() {
                return self.cancelled_outcome(project_id);
            }
            let status = self.service.workflow_status(project_id, &self.output)?;
            match status.step {
                WorkflowStep::QuestionRequired { ref question_id }
                    if self.auto_answer_client.is_some() =>
                {
                    if let Err(error) = self
                        .execute_auto_answer_batch(
                            project_id,
                            question_id,
                            auto_answer_progress.as_ref(),
                            cancellation.clone(),
                        )
                        .await
                    {
                        if matches!(error, WorkflowRunnerError::Agent(AgentError::Cancelled))
                            || cancellation.is_cancelled()
                        {
                            return self.cancelled_outcome(project_id);
                        }
                        self.service.pause_run(project_id, "research_failed")?;
                        return Err(error);
                    }
                }
                WorkflowStep::QuestionRequired { ref question_id }
                    if self.research_client.is_some() =>
                {
                    if let Err(error) = self
                        .execute_research_answer(
                            project_id,
                            question_id,
                            activity.as_ref(),
                            cancellation.clone(),
                        )
                        .await
                    {
                        if matches!(error, WorkflowRunnerError::Agent(AgentError::Cancelled))
                            || cancellation.is_cancelled()
                        {
                            return self.cancelled_outcome(project_id);
                        }
                        self.service.pause_run(project_id, "research_failed")?;
                        return Err(error);
                    }
                }
                WorkflowStep::QuestionRequired { .. } | WorkflowStep::ApprovalRequired { .. } => {
                    self.service.pause_run(project_id, "user_input_required")?;
                    return Ok(WorkflowRunOutcome {
                        status: self.service.workflow_status(project_id, &self.output)?,
                        stop: WorkflowRunStop::Boundary,
                    });
                }
                WorkflowStep::RepairRequired { .. } => {
                    if status.run.pause_reason.as_deref() != Some("repair_requested")
                        || matches!(self.mode, RunnerMode::Offline)
                    {
                        self.service.pause_run(project_id, "user_input_required")?;
                        return Ok(WorkflowRunOutcome {
                            status: self.service.workflow_status(project_id, &self.output)?,
                            stop: WorkflowRunStop::Boundary,
                        });
                    }
                    if let Err(error) = self
                        .execute_online_repair(project_id, activity.as_ref(), cancellation.clone())
                        .await
                    {
                        if matches!(error, WorkflowRunnerError::Agent(AgentError::Cancelled))
                            || cancellation.is_cancelled()
                        {
                            return self.cancelled_outcome(project_id);
                        }
                        self.service.pause_run(project_id, "repair_failed")?;
                        return Err(error);
                    }
                }
                WorkflowStep::GeneratePackage => {
                    if matches!(self.mode, RunnerMode::Offline) {
                        self.service
                            .execute_offline_step(project_id, &self.output)?;
                    } else if let Err(error) = self
                        .execute_online_generation(
                            project_id,
                            activity.as_ref(),
                            cancellation.clone(),
                        )
                        .await
                    {
                        if matches!(error, WorkflowRunnerError::Agent(AgentError::Cancelled))
                            || cancellation.is_cancelled()
                        {
                            return self.cancelled_outcome(project_id);
                        }
                        self.service.pause_run(project_id, "generation_failed")?;
                        return Err(error);
                    }
                }
                WorkflowStep::ValidatePackage => {
                    self.service
                        .execute_offline_step(project_id, &self.output)?;
                }
                WorkflowStep::Complete => {
                    return Ok(WorkflowRunOutcome {
                        status,
                        stop: WorkflowRunStop::Complete,
                    });
                }
            }
        }
        Err(WorkflowRunnerError::StepLimitExceeded)
    }

    /// Starts a first run only with explicit policy and preserves any resumable run policy.
    fn ensure_run(
        &mut self,
        project_id: &ProjectId,
        policy: Option<ApprovalPolicy>,
    ) -> Result<(), WorkflowRunnerError> {
        match self.service.active_run(project_id)? {
            Some(active) => {
                if let Some(requested) = policy
                    && requested != active.policy
                {
                    self.service.start_run(project_id, requested)?;
                }
            }
            None => {
                let policy = policy.ok_or(WorkflowRunnerError::PolicyRequired)?;
                self.service.start_run(project_id, policy)?;
            }
        }
        Ok(())
    }

    /// Produces and adopts one complete generation candidate after a final cancellation check.
    async fn execute_online_generation(
        &mut self,
        project_id: &ProjectId,
        activity: Option<&mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), WorkflowRunnerError> {
        self.emit(
            activity,
            ActivityKind::Lifecycle,
            "Generating project documentation",
        );
        let snapshot = self.service.inspect_project(project_id)?;
        let staging = tempfile::tempdir()
            .map_err(|error| WorkflowError::Document(format!("documentation staging: {error}")))?;
        let request = DocumentationRequest::generation(
            serialize(&snapshot)?,
            PackageRenderer::required_relative_paths(&snapshot),
            staging.path().to_path_buf(),
        );
        let RunnerMode::Online(client) = self.mode else {
            return Ok(());
        };
        client
            .execute(request, activity.cloned(), cancellation.clone())
            .await?;
        if cancellation.is_cancelled() {
            return Err(AgentError::Cancelled.into());
        }
        self.service
            .adopt_generated_package(project_id, staging.path(), &self.output)?;
        self.emit(
            activity,
            ActivityKind::Lifecycle,
            "Documentation generation completed",
        );
        Ok(())
    }

    /// Plans, researches, judges, and atomically adopts one bounded automatic-answer batch.
    async fn execute_auto_answer_batch(
        &mut self,
        project_id: &ProjectId,
        blocking_question_id: &str,
        progress: Option<&mpsc::Sender<AutoAnswerProgress>>,
        cancellation: CancellationToken,
    ) -> Result<(), WorkflowRunnerError> {
        let batch = self.next_auto_answer_batch;
        self.next_auto_answer_batch = self.next_auto_answer_batch.saturating_add(1);
        let snapshot = self.service.inspect_project(project_id)?;
        let eligible = snapshot
            .questions
            .iter()
            .filter(|question| {
                question.status == QuestionStatus::Open
                    && question
                        .priority
                        .requires_attention(snapshot.project.clarification_threshold())
            })
            .cloned()
            .collect::<Vec<_>>();
        let contexts = eligible
            .iter()
            .map(|question| {
                ResearchQuestionContext::new(
                    &question.id,
                    &question.display_id,
                    &question.prompt,
                    &question.rationale,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let snapshot_json = serialize(&snapshot)?;
        let plan_request =
            ResearchPlanRequest::new(snapshot_json.clone(), blocking_question_id, contexts)?;
        let client = self.auto_answer_client.clone().ok_or_else(|| {
            WorkflowError::Document("parallel automatic research client is unavailable".to_owned())
        })?;

        send_auto_progress(
            progress,
            AutoAnswerProgress::new(
                batch,
                AutoAnswerActor::Coordinator,
                AutoAnswerStage::Planning,
                "Selecting independent consequential questions",
                None,
            ),
        )
        .await;
        let plan = execute_plan_call(
            client.clone(),
            plan_request,
            progress.cloned(),
            batch,
            cancellation.clone(),
        )
        .await?;
        let eligible_ids = eligible
            .iter()
            .map(|question| question.id.clone())
            .collect::<Vec<_>>();
        plan.validate_for(&eligible_ids, blocking_question_id)?;
        send_auto_progress(
            progress,
            AutoAnswerProgress::new(
                batch,
                AutoAnswerActor::Coordinator,
                AutoAnswerStage::Accepted,
                &format!("Selected {} question(s)", plan.question_ids().len()),
                None,
            ),
        )
        .await;

        send_auto_progress(
            progress,
            AutoAnswerProgress::new(
                batch,
                AutoAnswerActor::Judge,
                AutoAnswerStage::Waiting,
                "Waiting for all selected workers",
                None,
            ),
        )
        .await;

        // Worker tasks own cloned requests and clients so no workflow state is shared or mutated.
        let mut workers = JoinSet::new();
        for (index, question_id) in plan.question_ids().iter().enumerate() {
            let question = eligible
                .iter()
                .find(|question| &question.id == question_id)
                .ok_or_else(|| {
                    WorkflowError::Document(format!(
                        "planned question disappeared before research: {question_id}"
                    ))
                })?;
            let request = ResearchRequest::new(
                snapshot_json.clone(),
                &question.id,
                &question.prompt,
                &question.rationale,
            )?
            .for_delegated_auto_answer();
            let slot = index + 1;
            let actor = AutoAnswerActor::Worker {
                slot,
                question_display_id: question.display_id.clone(),
            };
            send_auto_progress(
                progress,
                AutoAnswerProgress::new(
                    batch,
                    actor.clone(),
                    AutoAnswerStage::Queued,
                    &question.prompt,
                    None,
                ),
            )
            .await;
            workers.spawn(run_research_worker(
                client.clone(),
                request,
                progress.cloned(),
                batch,
                actor,
                cancellation.clone(),
            ));
        }

        let mut candidates = Vec::with_capacity(AUTO_ANSWER_WORKER_COUNT);
        while let Some(result) = workers.join_next().await {
            match result {
                Ok(Ok(candidate)) => candidates.push(candidate),
                Ok(Err(error)) => {
                    workers.abort_all();
                    while workers.join_next().await.is_some() {}
                    return Err(error.into());
                }
                Err(error) => {
                    workers.abort_all();
                    while workers.join_next().await.is_some() {}
                    return Err(AgentError::Execution(format!(
                        "automatic research worker stopped unexpectedly: {error}"
                    ))
                    .into());
                }
            }
        }
        if cancellation.is_cancelled() {
            return Err(AgentError::Cancelled.into());
        }
        candidates.sort_by_key(|candidate| {
            plan.question_ids()
                .iter()
                .position(|question_id| question_id == candidate.question_id())
                .unwrap_or(usize::MAX)
        });

        send_auto_progress(
            progress,
            AutoAnswerProgress::new(
                batch,
                AutoAnswerActor::Judge,
                AutoAnswerStage::Waiting,
                "All workers finished; preparing batch review",
                None,
            ),
        )
        .await;
        let candidate_ids = candidates
            .iter()
            .map(|candidate| candidate.question_id().to_owned())
            .collect::<Vec<_>>();
        let judgment_request = ResearchJudgmentRequest::new(snapshot_json, candidates)?;
        let judged = execute_judge_with_retry(
            client,
            judgment_request,
            &candidate_ids,
            progress.cloned(),
            batch,
            cancellation.clone(),
        )
        .await?;
        if cancellation.is_cancelled() {
            return Err(AgentError::Cancelled.into());
        }

        send_auto_progress(
            progress,
            AutoAnswerProgress::new(
                batch,
                AutoAnswerActor::Judge,
                AutoAnswerStage::Committing,
                "Committing validated judged answers",
                None,
            ),
        )
        .await;
        let accepted = self.service.answer_questions_from_research(&judged)?;
        for judged_answer in judged.answers() {
            if !accepted
                .iter()
                .any(|answer| answer.question_id == judged_answer.question_id())
            {
                let question = eligible
                    .iter()
                    .find(|question| question.id == judged_answer.question_id());
                send_auto_progress(
                    progress,
                    AutoAnswerProgress::new(
                        batch,
                        AutoAnswerActor::Worker {
                            slot: plan
                                .question_ids()
                                .iter()
                                .position(|id| id == judged_answer.question_id())
                                .map_or(1, |index| index + 1),
                            question_display_id: question.map_or_else(
                                || "unknown".to_owned(),
                                |value| value.display_id.clone(),
                            ),
                        },
                        AutoAnswerStage::Skipped,
                        "Question was no longer open at adoption",
                        None,
                    ),
                )
                .await;
            }
        }
        send_auto_progress(
            progress,
            AutoAnswerProgress::new(
                batch,
                AutoAnswerActor::Judge,
                AutoAnswerStage::Accepted,
                &format!("Accepted {} judged answer(s)", accepted.len()),
                None,
            ),
        )
        .await;
        Ok(())
    }

    /// Researches and atomically reconciles one blocking question before continuing the run.
    async fn execute_research_answer(
        &mut self,
        project_id: &ProjectId,
        question_id: &str,
        activity: Option<&mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), WorkflowRunnerError> {
        self.emit(
            activity,
            ActivityKind::Lifecycle,
            "Researching blocking clarification",
        );
        let snapshot = self.service.inspect_project(project_id)?;
        let question = snapshot
            .questions
            .iter()
            .find(|question| question.id == question_id)
            .ok_or_else(|| {
                WorkflowError::Document(format!(
                    "blocking question disappeared before research: {question_id}"
                ))
            })?;
        let request = ResearchRequest::new(
            serialize(&snapshot)?,
            &question.id,
            &question.prompt,
            &question.rationale,
        )?;
        let client = self.research_client.ok_or_else(|| {
            WorkflowError::Document("automatic research client is unavailable".to_owned())
        })?;
        let (sink, _receiver) = mpsc::channel(1);
        let answer = client
            .research(
                request,
                activity.cloned().unwrap_or(sink),
                cancellation.clone(),
            )
            .await?;
        if cancellation.is_cancelled() {
            return Err(AgentError::Cancelled.into());
        }
        self.service
            .answer_question_from_research(question_id, &answer)?;
        self.emit(
            activity,
            ActivityKind::Lifecycle,
            "Cited research answer recorded",
        );
        Ok(())
    }

    /// Copies the accepted package into staging and adopts one authorized repair candidate.
    async fn execute_online_repair(
        &mut self,
        project_id: &ProjectId,
        activity: Option<&mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), WorkflowRunnerError> {
        self.emit(
            activity,
            ActivityKind::Lifecycle,
            "Repairing project documentation",
        );
        let snapshot = self.service.inspect_project(project_id)?;
        let staging = tempfile::tempdir()
            .map_err(|error| WorkflowError::Document(format!("repair staging: {error}")))?;
        copy_current_package(&snapshot, &self.output, staging.path())?;
        let findings = self.service.latest_validation_findings(project_id)?;
        let request = DocumentationRequest::repair(
            serialize(&snapshot)?,
            PackageRenderer::required_relative_paths(&snapshot),
            staging.path().to_path_buf(),
            serialize(&findings)?,
        );
        let RunnerMode::Online(client) = self.mode else {
            return Ok(());
        };
        client
            .execute(request, activity.cloned(), cancellation.clone())
            .await?;
        if cancellation.is_cancelled() {
            return Err(AgentError::Cancelled.into());
        }
        self.service
            .adopt_generated_package(project_id, staging.path(), &self.output)?;
        self.emit(
            activity,
            ActivityKind::Lifecycle,
            "Documentation repair completed",
        );
        Ok(())
    }

    /// Pauses the active run and returns its current planned step without adopting staging.
    fn cancelled_outcome(
        &mut self,
        project_id: &ProjectId,
    ) -> Result<WorkflowRunOutcome, WorkflowRunnerError> {
        self.service.pause_run(project_id, "cancelled_by_user")?;
        Ok(WorkflowRunOutcome {
            status: self.service.workflow_status(project_id, &self.output)?,
            stop: WorkflowRunStop::Cancelled,
        })
    }

    /// Offers one bounded sanitized activity entry without allowing a slow UI to block work.
    fn emit(
        &mut self,
        activity: Option<&mpsc::Sender<ActivityEvent>>,
        kind: ActivityKind,
        message: &str,
    ) {
        if let Some(sender) = activity {
            let event = ActivityEvent::now(self.next_activity_sequence, kind, message);
            let _ = sender.try_send(event);
        }
        self.next_activity_sequence = self.next_activity_sequence.saturating_add(1);
    }
}

/// Selects a sole blocker locally or asks the coordinator to choose an independent batch.
async fn execute_plan_call(
    client: Arc<dyn AutoAnswerClient>,
    request: ResearchPlanRequest,
    progress: Option<mpsc::Sender<AutoAnswerProgress>>,
    batch: u32,
    cancellation: CancellationToken,
) -> Result<ResearchBatchPlan, AgentError> {
    if cancellation.is_cancelled() {
        return Err(AgentError::Cancelled);
    }
    // The validated request contains its blocker, so a singleton has exactly one legal plan.
    if request.questions().len() == 1 {
        return ResearchBatchPlan::new(vec![request.blocking_question_id().to_owned()]);
    }
    let (activity_sender, activity_receiver) = mpsc::channel(32);
    let forwarding = tokio::spawn(forward_actor_activity(
        activity_receiver,
        progress,
        batch,
        AutoAnswerActor::Coordinator,
        AutoAnswerStage::Planning,
    ));
    let result = client.plan(request, activity_sender, cancellation).await;
    let _ = forwarding.await;
    result
}

/// Researches one question with one retry while retaining ownership of its worker lane.
async fn run_research_worker(
    client: Arc<dyn AutoAnswerClient>,
    mut request: ResearchRequest,
    progress: Option<mpsc::Sender<AutoAnswerProgress>>,
    batch: u32,
    actor: AutoAnswerActor,
    cancellation: CancellationToken,
) -> Result<ResearchCandidate, AgentError> {
    for attempt in 0..AUTO_ANSWER_ATTEMPTS {
        if cancellation.is_cancelled() {
            return Err(AgentError::Cancelled);
        }
        let stage = if attempt == 0 {
            AutoAnswerStage::Researching
        } else {
            AutoAnswerStage::Retrying
        };
        send_auto_progress(
            progress.as_ref(),
            AutoAnswerProgress::new(
                batch,
                actor.clone(),
                stage,
                if attempt == 0 {
                    "Researching cited answer"
                } else {
                    "Retrying cited research"
                },
                None,
            ),
        )
        .await;
        let (activity_sender, activity_receiver) = mpsc::channel(32);
        let forwarding = tokio::spawn(forward_actor_activity(
            activity_receiver,
            progress.clone(),
            batch,
            actor.clone(),
            stage,
        ));
        let result = client
            .research(request.clone(), activity_sender, cancellation.clone())
            .await;
        let _ = forwarding.await;
        match result {
            Ok(answer) => {
                send_auto_progress(
                    progress.as_ref(),
                    AutoAnswerProgress::new(
                        batch,
                        actor,
                        AutoAnswerStage::Ready,
                        "Cited candidate ready",
                        Some(answer.evidence().len()),
                    ),
                )
                .await;
                return ResearchCandidate::new(request.question_id(), answer);
            }
            Err(AgentError::Cancelled) => return Err(AgentError::Cancelled),
            Err(error) if attempt + 1 < AUTO_ANSWER_ATTEMPTS => {
                let feedback = crate::agents::sanitize_terminal_text(&error.to_string());
                send_auto_progress(
                    progress.as_ref(),
                    AutoAnswerProgress::new(
                        batch,
                        actor.clone(),
                        AutoAnswerStage::Retrying,
                        &error.to_string(),
                        None,
                    ),
                )
                .await;
                request = request.with_retry_feedback(&feedback)?;
            }
            Err(error) => {
                send_auto_progress(
                    progress.as_ref(),
                    AutoAnswerProgress::new(
                        batch,
                        actor,
                        AutoAnswerStage::Failed,
                        &error.to_string(),
                        None,
                    ),
                )
                .await;
                return Err(error);
            }
        }
    }
    Err(AgentError::Execution(
        "automatic research exhausted its bounded attempts".to_owned(),
    ))
}

/// Reviews the complete candidate set with one retry and exact membership validation.
async fn execute_judge_with_retry(
    client: Arc<dyn AutoAnswerClient>,
    mut request: ResearchJudgmentRequest,
    candidate_ids: &[String],
    progress: Option<mpsc::Sender<AutoAnswerProgress>>,
    batch: u32,
    cancellation: CancellationToken,
) -> Result<JudgedResearchBatch, AgentError> {
    for attempt in 0..AUTO_ANSWER_ATTEMPTS {
        if cancellation.is_cancelled() {
            return Err(AgentError::Cancelled);
        }
        let stage = if attempt == 0 {
            AutoAnswerStage::Reviewing
        } else {
            AutoAnswerStage::Retrying
        };
        send_auto_progress(
            progress.as_ref(),
            AutoAnswerProgress::new(
                batch,
                AutoAnswerActor::Judge,
                stage,
                if attempt == 0 {
                    "Reviewing evidence and project consistency"
                } else {
                    "Retrying judged batch validation"
                },
                None,
            ),
        )
        .await;
        let (activity_sender, activity_receiver) = mpsc::channel(32);
        let forwarding = tokio::spawn(forward_actor_activity(
            activity_receiver,
            progress.clone(),
            batch,
            AutoAnswerActor::Judge,
            stage,
        ));
        let result = client
            .judge(request.clone(), activity_sender, cancellation.clone())
            .await
            .and_then(|judged| {
                judged.validate_for(candidate_ids)?;
                Ok(judged)
            });
        let _ = forwarding.await;
        match result {
            Ok(judged) => return Ok(judged),
            Err(AgentError::Cancelled) => return Err(AgentError::Cancelled),
            Err(error) if attempt + 1 < AUTO_ANSWER_ATTEMPTS => {
                let feedback = crate::agents::sanitize_terminal_text(&error.to_string());
                send_auto_progress(
                    progress.as_ref(),
                    AutoAnswerProgress::new(
                        batch,
                        AutoAnswerActor::Judge,
                        AutoAnswerStage::Retrying,
                        &error.to_string(),
                        None,
                    ),
                )
                .await;
                request = request.with_retry_feedback(&feedback)?;
            }
            Err(error) => {
                send_auto_progress(
                    progress.as_ref(),
                    AutoAnswerProgress::new(
                        batch,
                        AutoAnswerActor::Judge,
                        AutoAnswerStage::Failed,
                        &error.to_string(),
                        None,
                    ),
                )
                .await;
                return Err(error);
            }
        }
    }
    Err(AgentError::Execution(
        "automatic research judge exhausted its bounded attempts".to_owned(),
    ))
}

/// Maps provider activity into the latest state of one structured progress lane.
async fn forward_actor_activity(
    mut activity: mpsc::Receiver<ActivityEvent>,
    progress: Option<mpsc::Sender<AutoAnswerProgress>>,
    batch: u32,
    actor: AutoAnswerActor,
    stage: AutoAnswerStage,
) {
    while let Some(event) = activity.recv().await {
        send_auto_progress(
            progress.as_ref(),
            AutoAnswerProgress::new(batch, actor.clone(), stage, &event.message, None),
        )
        .await;
    }
}

/// Sends one structured progress update without treating a closed UI as workflow failure.
async fn send_auto_progress(
    progress: Option<&mpsc::Sender<AutoAnswerProgress>>,
    event: AutoAnswerProgress,
) {
    if let Some(progress) = progress {
        let _ = progress.try_send(event);
    }
}

/// Serializes checked workflow context before any provider process is started.
fn serialize<T: Serialize>(value: &T) -> Result<String, WorkflowRunnerError> {
    serde_json::to_string(value)
        .map_err(|error| WorkflowRunnerError::Serialization(error.to_string()))
}

/// Copies existing expected artifacts into repair staging without requiring missing ones.
fn copy_current_package(
    snapshot: &crate::domain::ProjectSnapshot,
    output: &Path,
    staging: &Path,
) -> Result<(), WorkflowRunnerError> {
    for relative in PackageRenderer::required_relative_paths(snapshot) {
        let source = output.join(&relative);
        if !source.is_file() {
            continue;
        }
        let destination = staging.join(&relative);
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                WorkflowError::Document(format!(
                    "could not create repair staging {}: {error}",
                    parent.display()
                ))
            })?;
        }
        std::fs::copy(&source, &destination).map_err(|error| {
            WorkflowError::Document(format!(
                "could not stage {} for repair: {error}",
                source.display()
            ))
        })?;
    }
    Ok(())
}
