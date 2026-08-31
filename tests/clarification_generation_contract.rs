use project_init::documents::PackageRenderer;
use project_init::domain::{
    ApprovalPolicy, ProjectId, ProjectStatus, QuestionStatus, WorkflowStep,
};
use project_init::storage::SqliteStore;
use project_init::workflow::ProjectService;

/// Initializes a durable clarification queue with an optional consequential question.
fn clarification_project(
    path: &std::path::Path,
    include_blocker: bool,
) -> (ProjectService, ProjectId) {
    let mut findings = vec![serde_json::json!({
        "kind": "unknown", "statement": "The optional accent color is undecided.",
        "impact": "low", "source_type": "derived"
    })];
    if include_blocker {
        findings.push(serde_json::json!({
            "kind": "unknown", "statement": "The delivery platform is undecided.",
            "impact": "high", "source_type": "derived"
        }));
    }
    let mut service =
        ProjectService::new(SqliteStore::open(path).expect("the database should open"));
    let project = service
        .initialize_from_analysis_json(
            "Clarification lifecycle",
            "Build a local planning tool.",
            &serde_json::json!({"findings": findings}).to_string(),
        )
        .expect("the analysis should initialize");
    assert_eq!(project.status(), ProjectStatus::AwaitingClarification);
    service
        .start_run(project.id(), ApprovalPolicy::Strict)
        .expect("the run should start");
    (service, project.id().clone())
}

/// Exercises both generation boundaries after reopening an eligible clarification project.
fn assert_clarification_generation(include_blocker: bool, change_threshold: bool) {
    for provider_package in [false, true] {
        let directory = tempfile::tempdir().expect("the workspace should exist");
        let database = directory.path().join("project.sqlite3");
        let output = directory.path().join("package");
        let (mut service, project_id) = clarification_project(&database, include_blocker);
        let snapshot = service
            .inspect_project(&project_id)
            .expect("the queue should load");
        let remaining = snapshot
            .questions
            .iter()
            .find(|q| q.priority.score() == 25)
            .expect("the low-impact question should exist")
            .clone();
        if include_blocker {
            let blocker = snapshot
                .questions
                .iter()
                .find(|q| q.priority.score() == 125)
                .expect("the high-impact question should exist");
            service
                .answer_question(&blocker.id, "A local desktop application.", None)
                .expect("the consequential answer should reconcile");
        }
        if change_threshold {
            service
                .set_clarification_threshold(&project_id, 25)
                .expect("the inclusive threshold should persist");
            assert_eq!(
                service
                    .execute_offline_step(&project_id, &output)
                    .expect("the exact threshold should pause")
                    .step,
                WorkflowStep::QuestionRequired {
                    question_id: remaining.id.clone()
                }
            );
            assert!(!output.exists());
            service
                .set_clarification_threshold(&project_id, 26)
                .expect("raising the threshold should release generation");
        }
        drop(service);

        // Reopen the original lifecycle state to cover existing saved projects as well as new runs.
        let mut service =
            ProjectService::new(SqliteStore::open(&database).expect("the database should reopen"));
        service
            .resume_run(&project_id)
            .expect("the run should resume");
        let before = service
            .inspect_project(&project_id)
            .expect("the snapshot should load");
        let status = service
            .workflow_status(&project_id, &output)
            .expect("the status should load");
        assert_eq!(status.project_status, ProjectStatus::AwaitingClarification);
        assert_eq!(status.step, WorkflowStep::GeneratePackage);
        assert_eq!(
            service
                .inspect_project(&project_id)
                .expect("status must be read-only"),
            before
        );

        let generated = if provider_package {
            let staging = directory.path().join("staging");
            PackageRenderer::render(&before, &staging)
                .expect("a complete provisional package should render");
            service.adopt_generated_package(&project_id, &staging, &output)
        } else {
            service.execute_offline_step(&project_id, &output)
        }
        .expect("eligible clarification projects should generate");
        assert_eq!(generated.project_status, ProjectStatus::Validating);
        assert_eq!(generated.step, WorkflowStep::ValidatePackage);
        let after = service
            .inspect_project(&project_id)
            .expect("the generated project should load");
        assert_eq!(after.questions, before.questions);
        assert_eq!(after.answers, before.answers);
        assert_eq!(after.findings, before.findings);
        assert!(
            std::fs::read_to_string(output.join("OpenQuestions.md"))
                .expect("open questions should remain documented")
                .contains(&remaining.prompt)
        );
        let completed = service
            .execute_offline_step(&project_id, &output)
            .expect("the package should validate");
        assert_eq!(completed.project_status, ProjectStatus::Complete);
        assert_eq!(completed.step, WorkflowStep::Complete);
        assert!(completed.latest_validation_passed);
    }
}

/// Generates and validates when analysis produced only questions below the default threshold.
#[test]
fn generation_allows_nonblocking_initial_clarifications() {
    assert_clarification_generation(false, false);
}

/// Continues after the consequential answer while preserving the unanswered optional question.
#[test]
fn generation_allows_remaining_clarifications_after_the_last_blocking_answer() {
    assert_clarification_generation(true, false);
}

/// Resumes a saved project when its threshold moves just above the remaining question's score.
#[test]
fn generation_allows_clarifications_after_raising_the_threshold() {
    assert_clarification_generation(false, true);
}

/// Keeps inclusive question and approval boundaries authoritative for both generation paths.
#[test]
fn generation_cannot_bypass_clarification_or_approval_boundaries() {
    let directory = tempfile::tempdir().expect("the workspace should exist");
    let (mut service, project_id) =
        clarification_project(&directory.path().join("project.sqlite3"), false);
    service
        .set_clarification_threshold(&project_id, 25)
        .expect("the threshold should persist");
    let decision = service
        .propose_decision(
            &project_id,
            "Delivery model",
            "Local desktop.",
            "The brief asks for local operation.",
            false,
        )
        .expect("the decision should persist");
    let before = service
        .inspect_project(&project_id)
        .expect("the project should load");
    let question = &before.questions[0];
    assert_eq!(question.status, QuestionStatus::Open);
    let staging = directory.path().join("staging");
    let output = directory.path().join("package");
    PackageRenderer::render(&before, &staging).expect("the provisional package should render");

    // Reject even a complete staged package while either user boundary still applies.
    for boundary in [
        WorkflowStep::QuestionRequired {
            question_id: question.id.clone(),
        },
        WorkflowStep::ApprovalRequired {
            decision_id: decision.id.clone(),
        },
    ] {
        let snapshot = service
            .inspect_project(&project_id)
            .expect("the snapshot should load");
        assert_eq!(
            service
                .execute_offline_step(&project_id, &output)
                .expect("the run should pause")
                .step,
            boundary
        );
        assert!(
            service
                .adopt_generated_package(&project_id, &staging, &output)
                .is_err()
        );
        assert!(!output.exists());
        assert_eq!(
            service
                .inspect_project(&project_id)
                .expect("the snapshot should remain unchanged"),
            snapshot
        );
        service
            .set_clarification_threshold(&project_id, 26)
            .expect("the threshold should release the question");
    }
    service
        .approve_decision(&decision.id, "Approved by the project owner.")
        .expect("the decision should resolve");
    assert_eq!(
        service
            .adopt_generated_package(&project_id, &staging, &output)
            .expect("generation should recover once both boundaries clear")
            .step,
        WorkflowStep::ValidatePackage
    );
}
