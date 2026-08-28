use project_init::domain::{FindingKind, ProjectStatus, SourceType};
use project_init::storage::SqliteStore;
use project_init::workflow::{ProjectService, StructuredAnalysis};

/// Rejects malformed structured analysis before any project row is created.
#[test]
fn invalid_structured_analysis_cannot_mutate_authoritative_state() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);

    let result = service.initialize_from_analysis_json(
        "Broken response",
        "A valid source brief.",
        r#"{"findings":[{"kind":"invented","statement":"bad"}]}"#,
    );

    assert!(result.is_err());
    assert_eq!(
        service
            .project_count()
            .expect("projects should be countable"),
        0
    );
}

/// Extracts explicit knowledge and asks about consequential VR ambiguity without inventing a platform.
#[test]
fn deterministic_analysis_persists_unknowns_and_prioritized_questions() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project(
            "Calm Fishing VR",
            "I want a calm VR fishing game with environmental cues for fishing points.",
        )
        .expect("the project should be analyzed");

    let snapshot = service
        .inspect_project(project.id())
        .expect("the project snapshot should load");

    assert_eq!(
        snapshot.project.status(),
        ProjectStatus::AwaitingClarification
    );
    assert!(snapshot.findings.iter().any(|finding| {
        finding.kind() == FindingKind::Unknown
            && finding
                .statement()
                .to_ascii_lowercase()
                .contains("platform")
    }));
    assert!(snapshot.questions.iter().any(|question| {
        question.prompt.to_ascii_lowercase().contains("platform") && question.priority.score() >= 75
    }));
    assert!(!snapshot.findings.iter().any(|finding| {
        finding.statement().contains("Meta Quest")
            && finding.source_type() != SourceType::UserAnswer
    }));
}

/// Stores an answer, resolves its unknown, creates a user-sourced requirement, and records provenance edges atomically.
#[test]
fn answering_a_question_reconciles_history_with_traceability() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project(
            "Calm Fishing VR",
            "I want a calm VR fishing game with environmental cues for fishing points.",
        )
        .expect("the project should be analyzed");
    let question = service
        .inspect_project(project.id())
        .expect("the project snapshot should load")
        .questions
        .into_iter()
        .find(|question| question.prompt.to_ascii_lowercase().contains("platform"))
        .expect("analysis should ask about target platform");

    service
        .answer_question(&question.id, "Meta Quest 3", None)
        .expect("the answer should reconcile atomically");
    let snapshot = service
        .inspect_project(project.id())
        .expect("the reconciled snapshot should load");

    assert!(snapshot.answers.iter().any(|answer| {
        answer.question_id == question.id
            && answer.answer_text == "Meta Quest 3"
            && answer.resolves_question
    }));
    let requirement = snapshot
        .requirements
        .iter()
        .find(|requirement| requirement.statement.contains("Meta Quest 3"))
        .expect("reconciliation should create a platform requirement");
    assert_eq!(requirement.source_type, SourceType::UserAnswer);
    assert!(
        snapshot
            .traces
            .iter()
            .any(|trace| { trace.source_id == requirement.id && trace.target_id == question.id })
    );
    assert!(
        snapshot.findings.iter().any(
            |finding| finding.display_id().starts_with("UNK-") && !finding.status().is_active()
        )
    );
}

/// Requires every workflow-critical JSON response to contain non-empty typed findings.
#[test]
fn structured_analysis_validates_non_empty_statements() {
    let invalid = StructuredAnalysis::from_json(
        r#"{"findings":[{"kind":"confirmed_fact","statement":"   ","impact":"low"}]}"#,
    );

    assert!(invalid.is_err());
}

/// Advances the lifecycle when the final blocking clarification is answered.
#[test]
fn final_answer_advances_the_project_to_planning() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project("Calm Fishing VR", "I want a calm VR fishing game.")
        .expect("the project should initialize");
    let questions = service
        .inspect_project(project.id())
        .expect("questions should load")
        .questions;

    for question in questions {
        service
            .answer_question(&question.id, "Confirmed choice", None)
            .expect("each answer should reconcile");
    }

    assert_eq!(
        service
            .inspect_project(project.id())
            .expect("the final state should load")
            .project
            .status(),
        ProjectStatus::Planning
    );
}
