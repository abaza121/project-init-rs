use std::fs;

use chrono::Utc;
use project_init::domain::{ApprovalPolicy, ProjectStatus, WorkflowStep};
use project_init::storage::SqliteStore;
use project_init::workflow::ProjectService;
use rusqlite::params;

/// Gives an initialized project a complete deterministic brief without generated questions.
fn initialized_service() -> (ProjectService, project_init::domain::ProjectId) {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Complete loop",
            "Build a local planning tool for independent game developers.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The product is a local planning tool for independent game developers.","impact":"medium","source_type":"user_brief"}]}"#,
        )
        .expect("the project should initialize");
    (service, project.id().clone())
}

/// Persists the selected policy and reuses the same active run on resume.
#[test]
fn run_resume_preserves_identity_and_policy() {
    let (mut service, project_id) = initialized_service();

    let started = service
        .start_run(&project_id, ApprovalPolicy::Consequential)
        .expect("the run should start");
    let resumed = service
        .resume_run(&project_id)
        .expect("the active run should resume");

    assert_eq!(resumed.id, started.id);
    assert_eq!(resumed.policy, ApprovalPolicy::Consequential);
}

/// Rejects a competing active policy without replacing the durable run.
#[test]
fn run_rejects_policy_changes_while_active() {
    let (mut service, project_id) = initialized_service();
    let original = service
        .start_run(&project_id, ApprovalPolicy::Strict)
        .expect("the strict run should start");

    assert!(
        service
            .start_run(&project_id, ApprovalPolicy::Autonomous)
            .is_err()
    );
    assert_eq!(
        service
            .resume_run(&project_id)
            .expect("the original run should remain resumable")
            .id,
        original.id
    );
}

/// Gives consequential questions precedence over every automated package action.
#[test]
fn planner_pauses_for_questions_before_generation() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project("VR project", "Build a calm VR fishing game.")
        .expect("the project should initialize with questions");
    service
        .start_run(project.id(), ApprovalPolicy::Autonomous)
        .expect("the run should start");
    let directory = tempfile::tempdir().expect("an output directory should exist");

    let status = service
        .workflow_status(project.id(), directory.path())
        .expect("status should be observable");

    assert!(matches!(status.step, WorkflowStep::QuestionRequired { .. }));
    assert_eq!(status.project_status, ProjectStatus::AwaitingClarification);
}

/// Executes generation and validation as separate resumable mutations before completion.
#[test]
fn offline_steps_complete_the_full_package() {
    let (mut service, project_id) = initialized_service();
    let directory = tempfile::tempdir().expect("an output directory should exist");
    service
        .start_run(&project_id, ApprovalPolicy::Autonomous)
        .expect("the run should start");

    let before = service
        .workflow_status(&project_id, directory.path())
        .expect("the initial step should be available");
    assert_eq!(before.step, WorkflowStep::GeneratePackage);

    let after_generation = service
        .execute_offline_step(&project_id, directory.path())
        .expect("generation should succeed");
    assert_eq!(after_generation.step, WorkflowStep::ValidatePackage);

    let after_validation = service
        .execute_offline_step(&project_id, directory.path())
        .expect("validation should succeed");
    assert_eq!(after_validation.step, WorkflowStep::Complete);
    assert_eq!(after_validation.project_status, ProjectStatus::Complete);
    assert!(after_validation.latest_validation_passed);
    assert_eq!(
        service
            .workflow_status(&project_id, directory.path())
            .expect("completed status should remain observable")
            .step,
        WorkflowStep::Complete
    );

    service
        .add_evidence(
            &project_id,
            "A primary source changes the delivery constraint.",
            "https://example.com/primary",
            "Primary source",
            project_init::domain::EvidenceReliability::High,
            None,
        )
        .expect("new evidence should persist");
    service
        .start_run(&project_id, ApprovalPolicy::Autonomous)
        .expect("authoritative changes should permit a new run");
    assert_eq!(
        service
            .workflow_status(&project_id, directory.path())
            .expect("evidence should invalidate generated views")
            .step,
        WorkflowStep::GeneratePackage
    );
}

/// Preserves a manually edited artifact while refreshing its generated dependants.
#[test]
fn manual_override_survives_a_resumed_run() {
    let (mut service, project_id) = initialized_service();
    let directory = tempfile::tempdir().expect("an output directory should exist");
    service
        .start_run(&project_id, ApprovalPolicy::Autonomous)
        .expect("the run should start");
    service
        .execute_offline_step(&project_id, directory.path())
        .expect("the package should generate");
    service
        .execute_offline_step(&project_id, directory.path())
        .expect("the package should validate");
    let requirements = directory.path().join("Requirements.md");
    fs::write(
        &requirements,
        "# Requirements\n\nManual authoritative edit.\n",
    )
    .expect("the manual edit should be written");

    service
        .start_run(&project_id, ApprovalPolicy::Autonomous)
        .expect("a new run should start after completion");
    service
        .execute_offline_step(&project_id, directory.path())
        .expect("dependent generation should succeed");

    assert_eq!(
        fs::read_to_string(requirements).expect("the override should remain readable"),
        "# Requirements\n\nManual authoritative edit.\n"
    );
    assert!(
        service
            .document_revisions(&project_id, "Requirements.md")
            .expect("revision history should load")
            .iter()
            .any(|revision| revision.is_manual_override())
    );

    service
        .remove_override(&project_id, "Requirements.md")
        .expect("the override should return to generated control");
    service
        .execute_offline_step(&project_id, directory.path())
        .expect("generation should replace a released override");
    assert_ne!(
        fs::read_to_string(directory.path().join("Requirements.md"))
            .expect("the regenerated requirement file should be readable"),
        "# Requirements\n\nManual authoritative edit.\n"
    );
}

/// Applies strict and consequential approval policies to persisted decision state.
#[test]
fn planner_requires_only_policy_selected_decision_approvals() {
    let (mut strict_service, strict_project) = initialized_service();
    let strict_decision = strict_service
        .propose_decision(
            &strict_project,
            "Delivery model",
            "Ship a local desktop application.",
            "The brief prioritizes local operation.",
            false,
        )
        .expect("the decision should be proposed");
    strict_service
        .start_run(&strict_project, ApprovalPolicy::Strict)
        .expect("the strict run should start");
    let strict_output = tempfile::tempdir().expect("an output directory should exist");
    assert_eq!(
        strict_service
            .workflow_status(&strict_project, strict_output.path())
            .expect("strict status should load")
            .step,
        WorkflowStep::ApprovalRequired {
            decision_id: strict_decision.id.clone()
        }
    );
    strict_service
        .approve_decision(&strict_decision.id, "Approved by the project owner.")
        .expect("the decision should be approved");
    assert!(
        strict_service
            .reject_decision(&strict_decision.id, "A conflicting later response.")
            .is_err(),
        "closed decisions must reject conflicting repeated authority"
    );
    assert_eq!(
        strict_service
            .workflow_status(&strict_project, strict_output.path())
            .expect("approved status should load")
            .step,
        WorkflowStep::GeneratePackage
    );

    let (mut autonomous_service, autonomous_project) = initialized_service();
    let consequential = autonomous_service
        .propose_decision(
            &autonomous_project,
            "Working brand name",
            "Use Northstar as the working name.",
            "Naming affects every identity artifact.",
            true,
        )
        .expect("the consequential decision should be proposed");
    autonomous_service
        .start_run(&autonomous_project, ApprovalPolicy::Autonomous)
        .expect("the autonomous run should start");
    let autonomous_output = tempfile::tempdir().expect("an output directory should exist");
    assert_eq!(
        autonomous_service
            .workflow_status(&autonomous_project, autonomous_output.path())
            .expect("autonomous status should load")
            .step,
        WorkflowStep::ApprovalRequired {
            decision_id: consequential.id
        }
    );
}

/// Builds a durable repair boundary with one persisted structurally repairable finding.
fn repair_boundary(
    repair_limit: u8,
) -> (
    tempfile::TempDir,
    tempfile::TempDir,
    ProjectService,
    project_init::domain::ProjectId,
) {
    let database_directory = tempfile::tempdir().expect("the database directory should exist");
    let database_path = database_directory.path().join("project-init.sqlite3");
    let store = SqliteStore::open(&database_path).expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Repair project",
            "Build a local planning tool.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The product is a local planning tool.","impact":"medium","source_type":"user_brief"}]}"#,
        )
        .expect("the project should initialize");
    let project_id = project.id().clone();
    let output = tempfile::tempdir().expect("the package directory should exist");
    service
        .start_run(&project_id, ApprovalPolicy::Autonomous)
        .expect("the initial run should start");
    service
        .execute_offline_step(&project_id, output.path())
        .expect("the package should generate");
    service
        .execute_offline_step(&project_id, output.path())
        .expect("the initial package should validate");
    service
        .start_run(&project_id, ApprovalPolicy::Consequential)
        .expect("a repair fixture run should start");

    let connection =
        rusqlite::Connection::open(&database_path).expect("the fixture connection should open");
    let now = Utc::now().to_rfc3339();
    connection
        .execute(
            "UPDATE projects SET repair_limit = ?2 WHERE id = ?1",
            params![project_id.as_str(), repair_limit],
        )
        .expect("the repair limit should be configured");
    connection
        .execute(
            "DELETE FROM validation_runs WHERE project_id = ?1",
            [project_id.as_str()],
        )
        .expect("the earlier passing validation should be removed from the fixture");
    connection
        .execute(
            "INSERT INTO validation_runs \
             (id, project_id, status, repair_pass, high_count, medium_count, low_count, started_at, completed_at) \
             VALUES ('repair-validation', ?1, 'failed', 0, 1, 0, 0, ?2, ?2)",
            params![project_id.as_str(), now],
        )
        .expect("the failed validation should persist");
    connection
        .execute(
            "INSERT INTO validation_findings \
             (id, display_id, project_id, validation_run_id, code, severity, message, entity_type, entity_id, status, repairable, created_at, updated_at) \
             VALUES ('repair-finding', 'VAL-REPAIR', ?1, 'repair-validation', 'STALE_ARTIFACT', 'high', 'A generated artifact is stale.', 'document', 'Requirements.md', 'open', 1, ?2, ?2)",
            params![project_id.as_str(), now],
        )
        .expect("the repairable finding should persist");
    drop(connection);

    let status = service
        .workflow_status(&project_id, output.path())
        .expect("the persisted validation should produce a repair boundary");
    assert!(matches!(status.step, WorkflowStep::RepairRequired { .. }));
    (database_directory, output, service, project_id)
}

/// Persists repair authorization and its validation context across database reopen.
#[test]
fn repair_authorization_is_explicit_and_resumable() {
    let (database_directory, output, mut service, project_id) = repair_boundary(2);

    let findings = service
        .request_repair(&project_id, output.path())
        .expect("a repair below the configured limit should be authorized");
    assert!(!findings.is_empty());
    assert_eq!(
        service
            .active_run(&project_id)
            .expect("the active run should load")
            .expect("the run should remain active")
            .pause_reason
            .as_deref(),
        Some("repair_requested")
    );
    drop(service);

    let reopened = ProjectService::new(
        SqliteStore::open(&database_directory.path().join("project-init.sqlite3"))
            .expect("the database should reopen"),
    );
    assert_eq!(
        reopened
            .active_run(&project_id)
            .expect("the reopened run should load")
            .expect("the repair run should remain resumable")
            .pause_reason
            .as_deref(),
        Some("repair_requested")
    );
}

/// Rejects repair authorization at a zero limit without changing the paused run.
#[test]
fn repair_limit_zero_rejects_authorization_without_mutation() {
    let (_database_directory, output, mut service, project_id) = repair_boundary(0);
    let before = service
        .active_run(&project_id)
        .expect("the active run should load")
        .expect("the repair run should exist");

    assert!(service.request_repair(&project_id, output.path()).is_err());
    assert_eq!(
        service
            .active_run(&project_id)
            .expect("the rejected run should reload")
            .expect("the run should remain resumable"),
        before
    );
}

/// Rejects another repair when the newest completed pass equals the configured limit.
#[test]
fn repair_limit_rejects_the_exact_exhausted_pass_boundary() {
    let (database_directory, output, mut service, project_id) = repair_boundary(2);
    let connection =
        rusqlite::Connection::open(database_directory.path().join("project-init.sqlite3"))
            .expect("the fixture connection should open");
    connection
        .execute(
            "UPDATE validation_runs SET repair_pass = 2 WHERE id = 'repair-validation'",
            [],
        )
        .expect("the exhausted pass should persist");
    drop(connection);
    let before = service
        .active_run(&project_id)
        .expect("the run should load")
        .expect("the run should remain active");

    let error = service
        .request_repair(&project_id, output.path())
        .expect_err("the exact repair limit must reject another attempt");

    assert!(error.to_string().contains("repair limit of 2"));
    assert_eq!(
        service
            .active_run(&project_id)
            .expect("the rejected run should reload")
            .expect("the run should remain active"),
        before
    );
}

/// Escalates non-repairable validation findings instead of authorizing Codex changes.
#[test]
fn non_repairable_validation_finding_requires_user_authority() {
    let (database_directory, output, mut service, project_id) = repair_boundary(2);
    let connection =
        rusqlite::Connection::open(database_directory.path().join("project-init.sqlite3"))
            .expect("the fixture connection should open");
    connection
        .execute(
            "UPDATE validation_findings SET repairable = 0 WHERE id = 'repair-finding'",
            [],
        )
        .expect("the non-repairable finding should persist");
    drop(connection);

    let error = service
        .request_repair(&project_id, output.path())
        .expect_err("user authority findings must not start document repair");

    assert!(error.to_string().contains("require user authority"));
}

/// Rejects a mixed finding set because document repair cannot resolve user authority work.
#[test]
fn mixed_repairable_and_authority_findings_do_not_start_codex_repair() {
    let (database_directory, output, mut service, project_id) = repair_boundary(2);
    let connection =
        rusqlite::Connection::open(database_directory.path().join("project-init.sqlite3"))
            .expect("the fixture connection should open");
    let now = Utc::now().to_rfc3339();
    connection
        .execute(
            "INSERT INTO validation_findings \
             (id, display_id, project_id, validation_run_id, code, severity, message, entity_type, entity_id, status, repairable, created_at, updated_at) \
             VALUES ('authority-finding', 'VAL-AUTHORITY', ?1, 'repair-validation', 'UNSUPPORTED_REQUIREMENT', 'high', 'A product choice requires authority.', 'requirement', 'REQ-AUTHORITY', 'open', 0, ?2, ?2)",
            params![project_id.as_str(), now],
        )
        .expect("the authority finding should persist");
    drop(connection);

    let error = service
        .request_repair(&project_id, output.path())
        .expect_err("mixed authority work must block Codex repair");

    assert!(error.to_string().contains("require user authority"));
}

/// Ignores an older failed validation after a newer accepted document revision exists.
#[test]
fn newer_document_revisions_return_old_validation_findings_to_validation() {
    let (_database_directory, output, mut service, project_id) = repair_boundary(2);
    fs::write(
        output.path().join("Requirements.md"),
        "# Requirements\n\nManual authoritative revision.\n",
    )
    .expect("the manual revision should be written");
    service
        .register_override(&project_id, output.path(), "Requirements.md")
        .expect("the manual override should register");

    let after_generation = service
        .execute_offline_step(&project_id, output.path())
        .expect("stale dependants should regenerate");

    assert_eq!(after_generation.step, WorkflowStep::ValidatePackage);
}
