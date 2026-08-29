use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use project_init::agents::{
    AgentError, AutoAnswerClient, CancellationToken, DocumentationClient, DocumentationKind,
    DocumentationRequest, JudgedResearchAnswer, JudgedResearchBatch, ResearchBatchPlan,
    ResearchClient, ResearchEvidence, ResearchJudgmentRequest, ResearchPlanRequest,
    ResearchRequest, ResearchedAnswer,
};
use project_init::domain::{AnswerSource, ApprovalPolicy, EvidenceReliability, WorkflowStep};
use project_init::storage::SqliteStore;
use project_init::workflow::{
    AskQuestionRequest, AutoAnswerActor, AutoAnswerStage, ProjectService, WorkflowRunStop,
    WorkflowRunner, WorkflowRunnerError,
};
use rusqlite::params;

/// Writes every requested artifact so runner tests exercise adoption without a live provider.
struct CompleteDocumentationClient;

#[async_trait]
impl DocumentationClient for CompleteDocumentationClient {
    /// Produces a complete deterministic candidate package in the requested staging directory.
    async fn execute(
        &self,
        request: DocumentationRequest,
        activity: Option<tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>>,
        _cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        if let Some(activity) = activity {
            let _ = activity.try_send(project_init::agents::ActivityEvent::now(
                2,
                project_init::agents::ActivityKind::Progress,
                "Fixture documentation output",
            ));
        }
        for relative in &request.required_paths {
            let path = request.staging.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| AgentError::Execution(error.to_string()))?;
            }
            fs::write(path, format!("# {relative}\n"))
                .map_err(|error| AgentError::Execution(error.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ResearchClient for CompleteDocumentationClient {
    /// Returns one cited recommendation so the runner can cross a question boundary in tests.
    async fn research(
        &self,
        request: ResearchRequest,
        _activity: tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>,
        _cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        let evidence = ResearchEvidence::new(
            "The cited platform documentation supports this fixture choice.",
            "https://example.com/platform-documentation",
            "Platform documentation",
            EvidenceReliability::High,
            None,
        )?;
        ResearchedAnswer::new(
            &format!("Evidence-backed choice for {}", request.question_id()),
            Some("Fixture research recommendation."),
            vec![evidence],
        )
    }
}

/// Represents research that cannot produce a responsibly cited answer.
struct UnavailableResearchClient;

#[async_trait]
impl ResearchClient for UnavailableResearchClient {
    /// Fails before returning any recommendation or evidence.
    async fn research(
        &self,
        _request: ResearchRequest,
        _activity: tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>,
        _cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        Err(AgentError::InvalidResponse(
            "research was unavailable".to_owned(),
        ))
    }
}

/// Records concurrent worker occupancy and optionally cancels immediately before adoption.
struct ParallelAutoAnswerClient {
    active_workers: AtomicUsize,
    maximum_workers: AtomicUsize,
    research_attempts: AtomicUsize,
    judge_attempts: AtomicUsize,
    fail_first_worker: AtomicBool,
    fail_first_judge: AtomicBool,
    cancel_in_judge: bool,
}

impl ParallelAutoAnswerClient {
    /// Creates a deterministic concurrent client for runner behavior tests.
    const fn new(cancel_in_judge: bool, fail_first_worker: bool, fail_first_judge: bool) -> Self {
        Self {
            active_workers: AtomicUsize::new(0),
            maximum_workers: AtomicUsize::new(0),
            research_attempts: AtomicUsize::new(0),
            judge_attempts: AtomicUsize::new(0),
            fail_first_worker: AtomicBool::new(fail_first_worker),
            fail_first_judge: AtomicBool::new(fail_first_judge),
            cancel_in_judge,
        }
    }

    /// Returns the largest number of overlapping research calls observed by the fake.
    fn maximum_workers(&self) -> usize {
        self.maximum_workers.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl AutoAnswerClient for ParallelAutoAnswerClient {
    /// Selects the immediate blocker and the next two eligible questions in stable order.
    async fn plan(
        &self,
        request: ResearchPlanRequest,
        _activity: tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>,
        _cancellation: CancellationToken,
    ) -> Result<ResearchBatchPlan, AgentError> {
        ResearchBatchPlan::new(
            request
                .questions()
                .iter()
                .take(3)
                .map(|question| question.question_id().to_owned())
                .collect(),
        )
    }

    /// Holds each worker briefly so the test can observe genuine overlap.
    async fn research(
        &self,
        request: ResearchRequest,
        _activity: tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>,
        _cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        self.research_attempts.fetch_add(1, Ordering::SeqCst);
        let active = self.active_workers.fetch_add(1, Ordering::SeqCst) + 1;
        self.maximum_workers.fetch_max(active, Ordering::SeqCst);
        tokio::time::sleep(Duration::from_millis(40)).await;
        self.active_workers.fetch_sub(1, Ordering::SeqCst);
        if self.fail_first_worker.swap(false, Ordering::SeqCst) {
            return Err(AgentError::InvalidResponse(
                "fixture worker requires retry".to_owned(),
            ));
        }
        let evidence = ResearchEvidence::new(
            "The fixture source supports this automatic answer.",
            "https://example.com/parallel-research",
            "Parallel research source",
            EvidenceReliability::High,
            None,
        )?;
        ResearchedAnswer::new(
            &format!("Judged answer for {}", request.question_id()),
            None,
            vec![evidence],
        )
    }

    /// Mirrors every candidate into the exact judged batch after workers have stopped.
    async fn judge(
        &self,
        request: ResearchJudgmentRequest,
        _activity: tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<JudgedResearchBatch, AgentError> {
        self.judge_attempts.fetch_add(1, Ordering::SeqCst);
        if self.active_workers.load(Ordering::SeqCst) != 0 {
            return Err(AgentError::Execution(
                "judge started before workers completed".to_owned(),
            ));
        }
        if self.cancel_in_judge {
            cancellation.cancel();
        }
        if self.fail_first_judge.swap(false, Ordering::SeqCst) {
            return Err(AgentError::InvalidResponse(
                "fixture judge requires retry".to_owned(),
            ));
        }
        let answers = request
            .candidates()
            .iter()
            .map(|candidate| {
                JudgedResearchAnswer::new(candidate.question_id(), candidate.answer().clone())
            })
            .collect::<Result<Vec<_>, _>>()?;
        JudgedResearchBatch::new(answers)
    }
}

/// Forwards documentation-client progress through the runner's bounded activity channel.
#[tokio::test]
async fn online_runner_forwards_documentation_output() {
    let (service, project_id) = initialized_service();
    let output = tempfile::tempdir().expect("the output directory should exist");
    let client = CompleteDocumentationClient;
    let mut runner = WorkflowRunner::online(service, output.path(), &client);
    let (sender, mut receiver) = tokio::sync::mpsc::channel(8);

    runner
        .run_until_pause(
            &project_id,
            Some(ApprovalPolicy::Consequential),
            Some(sender),
            CancellationToken::new(),
        )
        .await
        .expect("the online run should complete");

    let mut messages = Vec::new();
    while let Ok(event) = receiver.try_recv() {
        messages.push(event.message);
    }
    assert!(
        messages
            .iter()
            .any(|message| message == "Fixture documentation output")
    );
}

/// Cancels only after writing a seemingly complete candidate to exercise the adoption race.
struct CancelledDocumentationClient;

#[async_trait]
impl DocumentationClient for CancelledDocumentationClient {
    /// Produces candidate bytes and then requests cancellation before returning success.
    async fn execute(
        &self,
        request: DocumentationRequest,
        _activity: Option<tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        for relative in &request.required_paths {
            let path = request.staging.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| AgentError::Execution(error.to_string()))?;
            }
            fs::write(path, format!("# {relative}\n"))
                .map_err(|error| AgentError::Execution(error.to_string()))?;
        }
        cancellation.cancel();
        Ok(())
    }
}

/// Returns success without producing expected files to prove staging remains untrusted.
struct IncompleteDocumentationClient;

#[async_trait]
impl DocumentationClient for IncompleteDocumentationClient {
    /// Deliberately leaves staging empty while reporting provider success.
    async fn execute(
        &self,
        _request: DocumentationRequest,
        _activity: Option<tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>>,
        _cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        Ok(())
    }
}

/// Rewrites one generated artifact while relying on the runner to stage the current package.
struct RepairDocumentationClient;

#[async_trait]
impl DocumentationClient for RepairDocumentationClient {
    /// Repairs requirements and attempts to replace a protected artifact in provisional staging.
    async fn execute(
        &self,
        request: DocumentationRequest,
        _activity: Option<tokio::sync::mpsc::Sender<project_init::agents::ActivityEvent>>,
        _cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        if request.kind != DocumentationKind::Repair {
            return Err(AgentError::Execution(
                "the fixture expected a repair request".to_owned(),
            ));
        }
        fs::write(
            request.staging.join("Requirements.md"),
            "# Requirements\n\nRepaired candidate.\n",
        )
        .map_err(|error| AgentError::Execution(error.to_string()))?;
        fs::write(
            request.staging.join("MissionVision.md"),
            "# Mission and Vision\n\nUntrusted replacement.\n",
        )
        .map_err(|error| AgentError::Execution(error.to_string()))?;
        Ok(())
    }
}

/// Creates a project whose authoritative brief does not require clarification.
fn initialized_service() -> (ProjectService, project_init::domain::ProjectId) {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Runner project",
            "Build a local planning tool.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The product is a local planning tool.","impact":"medium","source_type":"user_brief"}]}"#,
        )
        .expect("the project should initialize");
    (service, project.id().clone())
}

/// Refuses to create a first run until the caller supplies an explicit approval policy.
#[tokio::test]
async fn first_runner_execution_requires_an_explicit_policy_without_mutation() {
    let (service, project_id) = initialized_service();
    let output = tempfile::tempdir().expect("the output directory should exist");
    let client = CompleteDocumentationClient;
    let mut runner = WorkflowRunner::online(service, output.path(), &client);

    let error = runner
        .run_until_pause(&project_id, None, None, CancellationToken::new())
        .await
        .expect_err("the missing policy must stop execution");

    assert!(matches!(error, WorkflowRunnerError::PolicyRequired));
    assert!(
        runner
            .service()
            .active_run(&project_id)
            .expect("active-run inspection should succeed")
            .is_none()
    );
    assert_eq!(
        fs::read_dir(output.path())
            .expect("the output directory should remain readable")
            .count(),
        0
    );
}

/// Runs provider-backed generation and deterministic validation through one shared loop.
#[tokio::test]
async fn online_runner_generates_validates_and_completes_a_package() {
    let (service, project_id) = initialized_service();
    let output = tempfile::tempdir().expect("the output directory should exist");
    let client = CompleteDocumentationClient;
    let mut runner = WorkflowRunner::online(service, output.path(), &client);

    let outcome = runner
        .run_until_pause(
            &project_id,
            Some(ApprovalPolicy::Consequential),
            None,
            CancellationToken::new(),
        )
        .await
        .expect("the complete candidate should finish the workflow");

    assert_eq!(outcome.stop, WorkflowRunStop::Complete);
    assert_eq!(outcome.status.step, WorkflowStep::Complete);
    assert!(output.path().join("Requirements.md").is_file());
    assert_eq!(
        runner
            .service()
            .active_run(&project_id)
            .expect("active-run inspection should succeed"),
        None,
        "completed runs must not remain active"
    );
}

/// Researches every blocking clarification and continues without returning a user boundary.
#[tokio::test]
async fn research_enabled_runner_answers_questions_and_completes() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project("VR project", "Build a calm VR fishing experience.")
        .expect("the project should initialize with questions");
    let project_id = project.id().clone();
    let output = tempfile::tempdir().expect("the output directory should exist");
    let client = CompleteDocumentationClient;
    let mut runner =
        WorkflowRunner::online(service, output.path(), &client).with_research_client(&client);

    let outcome = runner
        .run_until_pause(
            &project_id,
            Some(ApprovalPolicy::Autonomous),
            None,
            CancellationToken::new(),
        )
        .await
        .expect("research-backed automation should complete");
    let snapshot = runner
        .service()
        .inspect_project(&project_id)
        .expect("the completed project should remain inspectable");

    assert_eq!(outcome.stop, WorkflowRunStop::Complete);
    assert!(!snapshot.answers.is_empty());
    assert!(
        snapshot
            .answers
            .iter()
            .all(|answer| answer.source == AnswerSource::Imported)
    );
    assert_eq!(snapshot.evidence.len(), snapshot.answers.len());
}

/// Researches three distinct questions concurrently before judging and adopting their answers.
#[tokio::test]
async fn parallel_auto_answer_uses_three_workers_before_judging() {
    let (mut service, project_id) = initialized_service();
    for prompt in [
        "Which desktop platform launches first?",
        "Which accessibility standard applies?",
        "Which packaging format should ship?",
    ] {
        service
            .ask_question(&project_id, AskQuestionRequest::conservative(prompt))
            .expect("the consequential question should persist");
    }
    let output = tempfile::tempdir().expect("the output directory should exist");
    let documentation = CompleteDocumentationClient;
    let auto_answer = Arc::new(ParallelAutoAnswerClient::new(false, false, false));
    let mut runner = WorkflowRunner::online(service, output.path(), &documentation)
        .with_auto_answer_client(auto_answer.clone());
    let (progress_sender, mut progress_receiver) = tokio::sync::mpsc::channel(64);

    let outcome = runner
        .run_until_pause_with_auto_answer_progress(
            &project_id,
            Some(ApprovalPolicy::Autonomous),
            None,
            Some(progress_sender),
            CancellationToken::new(),
        )
        .await
        .expect("the parallel batch should complete");
    let snapshot = runner
        .service()
        .inspect_project(&project_id)
        .expect("the completed project should remain inspectable");
    let mut saw_worker = false;
    let mut saw_judge = false;
    while let Ok(progress) = progress_receiver.try_recv() {
        saw_worker |= matches!(progress.actor, AutoAnswerActor::Worker { .. })
            && progress.stage == AutoAnswerStage::Researching;
        saw_judge |= progress.actor == AutoAnswerActor::Judge
            && progress.stage == AutoAnswerStage::Reviewing;
    }

    assert_eq!(outcome.stop, WorkflowRunStop::Complete);
    assert_eq!(auto_answer.maximum_workers(), 3);
    assert_eq!(snapshot.answers.len(), 3);
    assert!(saw_worker);
    assert!(saw_judge);
}

/// Refuses to adopt a completed judged batch when cancellation wins the final race.
#[tokio::test]
async fn parallel_auto_answer_cancellation_before_adoption_persists_nothing() {
    let (mut service, project_id) = initialized_service();
    for prompt in [
        "Which desktop platform launches first?",
        "Which accessibility standard applies?",
    ] {
        service
            .ask_question(&project_id, AskQuestionRequest::conservative(prompt))
            .expect("the consequential question should persist");
    }
    let output = tempfile::tempdir().expect("the output directory should exist");
    let documentation = CompleteDocumentationClient;
    let auto_answer = Arc::new(ParallelAutoAnswerClient::new(true, false, false));
    let mut runner = WorkflowRunner::online(service, output.path(), &documentation)
        .with_auto_answer_client(auto_answer);

    let outcome = runner
        .run_until_pause_with_auto_answer_progress(
            &project_id,
            Some(ApprovalPolicy::Autonomous),
            None,
            None,
            CancellationToken::new(),
        )
        .await
        .expect("the cancellation should be a resumable outcome");
    let snapshot = runner
        .service()
        .inspect_project(&project_id)
        .expect("the cancelled project should remain inspectable");

    assert_eq!(outcome.stop, WorkflowRunStop::Cancelled);
    assert!(snapshot.answers.is_empty());
    assert!(snapshot.evidence.is_empty());
}

/// Retries one failed worker and one invalid judge call exactly once before adoption.
#[tokio::test]
async fn parallel_auto_answer_retries_worker_and_judge_once() {
    let (mut service, project_id) = initialized_service();
    service
        .ask_question(
            &project_id,
            AskQuestionRequest::conservative("Which desktop platform launches first?"),
        )
        .expect("the consequential question should persist");
    let output = tempfile::tempdir().expect("the output directory should exist");
    let documentation = CompleteDocumentationClient;
    let auto_answer = Arc::new(ParallelAutoAnswerClient::new(false, true, true));
    let mut runner = WorkflowRunner::online(service, output.path(), &documentation)
        .with_auto_answer_client(auto_answer.clone());
    let (progress_sender, mut progress_receiver) = tokio::sync::mpsc::channel(64);

    let outcome = runner
        .run_until_pause_with_auto_answer_progress(
            &project_id,
            Some(ApprovalPolicy::Autonomous),
            None,
            Some(progress_sender),
            CancellationToken::new(),
        )
        .await
        .expect("bounded retries should recover the batch");
    let progress = std::iter::from_fn(|| progress_receiver.try_recv().ok()).collect::<Vec<_>>();

    assert_eq!(outcome.stop, WorkflowRunStop::Complete);
    assert_eq!(auto_answer.research_attempts.load(Ordering::SeqCst), 2);
    assert_eq!(auto_answer.judge_attempts.load(Ordering::SeqCst), 2);
    assert!(progress.iter().any(|event| {
        matches!(event.actor, AutoAnswerActor::Worker { .. })
            && event.stage == AutoAnswerStage::Retrying
    }));
    assert!(progress.iter().any(|event| {
        event.actor == AutoAnswerActor::Judge && event.stage == AutoAnswerStage::Retrying
    }));
}

/// Fails closed at the question boundary when cited research is unavailable.
#[tokio::test]
async fn failed_research_pauses_without_persisting_an_answer() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project("VR project", "Build a calm VR fishing experience.")
        .expect("the project should initialize with questions");
    let project_id = project.id().clone();
    let output = tempfile::tempdir().expect("the output directory should exist");
    let documentation = CompleteDocumentationClient;
    let research = UnavailableResearchClient;
    let mut runner = WorkflowRunner::online(service, output.path(), &documentation)
        .with_research_client(&research);

    let error = runner
        .run_until_pause(
            &project_id,
            Some(ApprovalPolicy::Autonomous),
            None,
            CancellationToken::new(),
        )
        .await
        .expect_err("unavailable research must stop the run");
    let snapshot = runner
        .service()
        .inspect_project(&project_id)
        .expect("the paused project should remain inspectable");
    let active = runner
        .service()
        .active_run(&project_id)
        .expect("the paused run should load")
        .expect("the failed run should remain resumable");

    assert!(error.to_string().contains("research was unavailable"));
    assert_eq!(active.pause_reason.as_deref(), Some("research_failed"));
    assert!(snapshot.answers.is_empty());
    assert!(snapshot.evidence.is_empty());
    assert_eq!(
        fs::read_dir(output.path())
            .expect("the output directory should remain readable")
            .count(),
        0
    );
}

/// Stops before adoption when cancellation races with a successful provider response.
#[tokio::test]
async fn cancellation_after_candidate_generation_never_adopts_staging() {
    let (service, project_id) = initialized_service();
    let output = tempfile::tempdir().expect("the output directory should exist");
    let client = CancelledDocumentationClient;
    let mut runner = WorkflowRunner::online(service, output.path(), &client);

    let outcome = runner
        .run_until_pause(
            &project_id,
            Some(ApprovalPolicy::Strict),
            None,
            CancellationToken::new(),
        )
        .await
        .expect("acknowledged cancellation should be a resumable outcome");

    assert_eq!(outcome.stop, WorkflowRunStop::Cancelled);
    assert!(!output.path().join("Requirements.md").exists());
    let active = runner
        .service()
        .active_run(&project_id)
        .expect("the paused run should reload")
        .expect("the cancelled run should remain resumable");
    assert_eq!(active.pause_reason.as_deref(), Some("cancelled_by_user"));
}

/// Rejects an incomplete provider candidate without changing accepted package bytes.
#[tokio::test]
async fn incomplete_online_candidate_is_rejected_and_paused() {
    let (service, project_id) = initialized_service();
    let output = tempfile::tempdir().expect("the output directory should exist");
    let client = IncompleteDocumentationClient;
    let mut runner = WorkflowRunner::online(service, output.path(), &client);

    let error = runner
        .run_until_pause(
            &project_id,
            Some(ApprovalPolicy::Autonomous),
            None,
            CancellationToken::new(),
        )
        .await
        .expect_err("missing candidate artifacts must fail adoption");

    assert!(error.to_string().contains("missing required artifact"));
    assert!(!output.path().join("Requirements.md").exists());
    let active = runner
        .service()
        .active_run(&project_id)
        .expect("the failed run should reload")
        .expect("the failed run should remain resumable");
    assert_eq!(active.pause_reason.as_deref(), Some("generation_failed"));
}

/// Repairs once, preserves a manual override, and records the next validation pass.
#[tokio::test]
async fn authorized_online_repair_is_staged_adopted_and_revalidated() {
    let database_directory = tempfile::tempdir().expect("the database directory should exist");
    let database_path = database_directory.path().join("project-init.sqlite3");
    let store = SqliteStore::open(&database_path).expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Repair runner",
            "Build a local planning tool.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The product is a local planning tool.","impact":"medium","source_type":"user_brief"}]}"#,
        )
        .expect("the project should initialize");
    let project_id = project.id().clone();
    let output = tempfile::tempdir().expect("the output directory should exist");
    service
        .start_run(&project_id, ApprovalPolicy::Autonomous)
        .expect("the first run should start");
    service
        .execute_offline_step(&project_id, output.path())
        .expect("the original package should generate");
    service
        .execute_offline_step(&project_id, output.path())
        .expect("the original package should validate");
    let protected_content = "# Mission and Vision\n\nUser-owned direction.\n";
    fs::write(output.path().join("MissionVision.md"), protected_content)
        .expect("the manual content should be written");
    service
        .register_override(&project_id, output.path(), "MissionVision.md")
        .expect("the manual override should register");
    service
        .start_run(&project_id, ApprovalPolicy::Consequential)
        .expect("the override reconciliation run should start");
    service
        .execute_offline_step(&project_id, output.path())
        .expect("stale dependants should regenerate around the override");
    service
        .execute_offline_step(&project_id, output.path())
        .expect("the protected package should validate");
    service
        .start_run(&project_id, ApprovalPolicy::Consequential)
        .expect("the repair run should start");

    let connection =
        rusqlite::Connection::open(&database_path).expect("the fixture connection should open");
    let now = Utc::now().to_rfc3339();
    connection
        .execute(
            "DELETE FROM validation_runs WHERE project_id = ?1",
            [project_id.as_str()],
        )
        .expect("the previous validation should be removed from the fixture");
    connection
        .execute(
            "INSERT INTO validation_runs \
             (id, project_id, status, repair_pass, high_count, medium_count, low_count, started_at, completed_at) \
             VALUES ('repair-runner-validation', ?1, 'failed', 0, 1, 0, 0, ?2, ?2)",
            params![project_id.as_str(), now],
        )
        .expect("the failed validation should persist");
    connection
        .execute(
            "INSERT INTO validation_findings \
             (id, display_id, project_id, validation_run_id, code, severity, message, entity_type, entity_id, status, repairable, created_at, updated_at) \
             VALUES ('repair-runner-finding', 'VAL-REPAIR', ?1, 'repair-runner-validation', 'STALE_ARTIFACT', 'high', 'Requirements are stale.', 'document', 'Requirements.md', 'open', 1, ?2, ?2)",
            params![project_id.as_str(), now],
        )
        .expect("the repair finding should persist");
    drop(connection);
    service
        .request_repair(&project_id, output.path())
        .expect("the repair should be authorized");

    let client = RepairDocumentationClient;
    let mut runner = WorkflowRunner::online(service, output.path(), &client);
    let outcome = runner
        .run_until_pause(&project_id, None, None, CancellationToken::new())
        .await
        .expect("the authorized repair should complete");

    assert_eq!(outcome.stop, WorkflowRunStop::Complete);
    assert_eq!(
        fs::read_to_string(output.path().join("Requirements.md"))
            .expect("the repaired file should be readable"),
        "# Requirements\n\nRepaired candidate.\n"
    );
    assert_eq!(
        fs::read_to_string(output.path().join("MissionVision.md"))
            .expect("the protected file should be readable"),
        protected_content
    );
    let verification = rusqlite::Connection::open(&database_path)
        .expect("the verification connection should open");
    let repair_pass = verification
        .query_row(
            "SELECT repair_pass FROM validation_runs WHERE project_id = ?1 \
             ORDER BY completed_at DESC, rowid DESC LIMIT 1",
            [project_id.as_str()],
            |row| row.get::<_, u32>(0),
        )
        .expect("the repair validation should persist");
    assert_eq!(repair_pass, 1);
}
