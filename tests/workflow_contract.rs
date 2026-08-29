use project_init::agents::{
    ActivityEvent, ActivityKind, AgentExecution, JudgedResearchAnswer, JudgedResearchBatch,
    ResearchEvidence, ResearchedAnswer,
};
use project_init::domain::{
    AnswerSource, CostOfBeingWrong, EvidenceReliability, FindingKind, Impact, ProjectStatus,
    QuestionStatus, SourceType, Uncertainty,
};
use project_init::storage::SqliteStore;
use project_init::workflow::{AskQuestionRequest, ProjectService, StructuredAnalysis};

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

/// Rejects an invalid completed agent run without persisting its project or activity history.
#[test]
fn invalid_agent_execution_cannot_mutate_authoritative_state() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let execution = AgentExecution::new(
        r#"{"findings":[{"kind":"invented","statement":"bad"}]}"#,
        vec![ActivityEvent::now(
            1,
            ActivityKind::Lifecycle,
            "Codex session started",
        )],
    );

    let result = service.initialize_from_agent_execution(
        "Broken response",
        "A valid source brief.",
        execution,
    );

    assert!(result.is_err());
    assert_eq!(
        service
            .project_count()
            .expect("projects should be countable"),
        0
    );
}

/// Rejects duplicate model findings before stable identifiers or project state are allocated.
#[test]
fn duplicate_agent_findings_cannot_mutate_authoritative_state() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let execution = AgentExecution::new(
        r#"{"findings":[{"kind":"requirement","statement":"The tool runs locally.","impact":"medium","source_type":"user_brief"},{"kind":"requirement","statement":"the tool runs locally.","impact":"high","source_type":"agent_inference"}]}"#,
        Vec::new(),
    );

    let result = service.initialize_from_agent_execution(
        "Duplicate response",
        "The tool runs locally.",
        execution,
    );

    assert!(result.is_err());
    assert_eq!(
        service
            .project_count()
            .expect("projects should remain countable"),
        0
    );
}

/// Prevents model-declared provenance from promoting an invented statement to user authority.
#[test]
fn agent_cannot_claim_that_its_inference_came_from_the_brief() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let execution = AgentExecution::new(
        r#"{"findings":[{"kind":"confirmed_fact","statement":"Meta Quest 3 is the target.","impact":"high","source_type":"user_brief","requires_confirmation":false}]}"#,
        Vec::new(),
    );

    let project = service
        .initialize_from_agent_execution("VR game", "Build a calm VR game.", execution)
        .expect("the valid response should persist with corrected provenance");
    let snapshot = service
        .inspect_project(project.id())
        .expect("the project should remain inspectable");
    let invented = snapshot
        .findings
        .iter()
        .find(|finding| finding.statement().contains("Meta Quest 3"))
        .expect("the inference should remain visible for confirmation");

    assert_eq!(invented.source_type(), SourceType::AgentInference);
    assert!(invented.requires_confirmation());
}

/// Commits successful analysis and its bounded activity timeline as one resumable project.
#[test]
fn successful_agent_execution_persists_activity_with_the_project() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let execution = AgentExecution::new(
        r#"{"findings":[{"kind":"requirement","statement":"The tool runs locally.","impact":"medium","source_type":"user_brief"}]}"#,
        vec![
            ActivityEvent::now(1, ActivityKind::Lifecycle, "Codex session started"),
            ActivityEvent::now(2, ActivityKind::Progress, "Analyzing initial brief"),
        ],
    );

    let project = service
        .initialize_from_agent_execution("Local tool", "The tool runs locally.", execution)
        .expect("valid agent analysis should commit");
    let activity = service
        .agent_activity(project.id())
        .expect("the committed activity should remain inspectable");

    assert_eq!(activity.len(), 2);
    assert_eq!(activity[0].sequence, 1);
    assert_eq!(activity[1].message, "Analyzing initial brief");
}

/// Reloads successful analysis activity after every database handle has been dropped.
#[test]
fn successful_agent_activity_survives_database_reopen() {
    let directory = tempfile::tempdir().expect("a temporary directory should be available");
    let database_path = directory.path().join("project-init.sqlite3");
    let project_id = {
        let store = SqliteStore::open(&database_path).expect("the database should open");
        let mut service = ProjectService::new(store);
        let execution = AgentExecution::new(
            r#"{"findings":[{"kind":"requirement","statement":"The tool runs locally.","impact":"medium","source_type":"user_brief"}]}"#,
            vec![ActivityEvent::now(
                1,
                ActivityKind::Lifecycle,
                "Codex session started",
            )],
        );

        service
            .initialize_from_agent_execution("Local tool", "The tool runs locally.", execution)
            .expect("valid analysis should commit")
            .id()
            .clone()
    };

    let store = SqliteStore::open(&database_path).expect("the database should reopen");
    let service = ProjectService::new(store);
    let activity = service
        .agent_activity(&project_id)
        .expect("the timeline should survive restart");

    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].message, "Codex session started");
}

/// Rolls back the complete creation transaction when persisted activity violates ordering identity.
#[test]
fn invalid_activity_history_rolls_back_successful_analysis() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let execution = AgentExecution::new(
        r#"{"findings":[{"kind":"requirement","statement":"The tool runs locally.","impact":"medium","source_type":"user_brief"}]}"#,
        vec![
            ActivityEvent::now(1, ActivityKind::Lifecycle, "Codex session started"),
            ActivityEvent::now(1, ActivityKind::Progress, "Duplicate sequence"),
        ],
    );

    let result =
        service.initialize_from_agent_execution("Local tool", "The tool runs locally.", execution);

    assert!(result.is_err());
    assert_eq!(
        service
            .project_count()
            .expect("projects should be countable after rollback"),
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

/// Reconciles every applicable judged answer and its evidence in one service operation.
#[test]
fn research_batch_reconciles_multiple_answers_with_traceability() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Batch project",
            "Build a local tool.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The tool is local.","impact":"low"}]}"#,
        )
        .expect("the project should initialize");
    let first = service
        .ask_question(
            project.id(),
            AskQuestionRequest::conservative("Which desktop platform launches first?"),
        )
        .expect("the first question should persist");
    let second = service
        .ask_question(
            project.id(),
            AskQuestionRequest::conservative("Which accessibility standard applies?"),
        )
        .expect("the second question should persist");
    let batch = judged_batch(&[&first.id, &second.id]);

    let accepted = service
        .answer_questions_from_research(&batch)
        .expect("the judged batch should reconcile atomically");
    let snapshot = service
        .inspect_project(project.id())
        .expect("the reconciled project should remain inspectable");

    assert_eq!(accepted.len(), 2);
    assert_eq!(snapshot.answers.len(), 2);
    assert_eq!(snapshot.evidence.len(), 2);
    assert!(
        snapshot
            .answers
            .iter()
            .all(|answer| answer.source == AnswerSource::Imported)
    );
}

/// Rolls back earlier applicable answers when a later batch member belongs to no question.
#[test]
fn research_batch_failure_leaves_authoritative_state_unchanged() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Atomic batch",
            "Build a local tool.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The tool is local.","impact":"low"}]}"#,
        )
        .expect("the project should initialize");
    let question = service
        .ask_question(
            project.id(),
            AskQuestionRequest::conservative("Which desktop platform launches first?"),
        )
        .expect("the question should persist");
    let batch = judged_batch(&[&question.id, "missing-question"]);

    assert!(service.answer_questions_from_research(&batch).is_err());
    let snapshot = service
        .inspect_project(project.id())
        .expect("the rejected project should remain inspectable");
    assert!(snapshot.answers.is_empty());
    assert!(snapshot.evidence.is_empty());
    assert_eq!(snapshot.questions[0].status, QuestionStatus::Open);
}

/// Builds a valid judged batch for storage and workflow contract tests.
fn judged_batch(question_ids: &[&str]) -> JudgedResearchBatch {
    let answers = question_ids
        .iter()
        .map(|question_id| {
            let evidence = ResearchEvidence::new(
                "The cited source supports this recommendation.",
                "https://example.com/source",
                "Primary source",
                EvidenceReliability::High,
                None,
            )
            .expect("the evidence should validate");
            let answer = ResearchedAnswer::new("Use the supported option.", None, vec![evidence])
                .expect("the answer should validate");
            JudgedResearchAnswer::new(question_id, answer)
                .expect("the judged answer should validate")
        })
        .collect();
    JudgedResearchBatch::new(answers).expect("the batch should validate")
}

/// Persists automatic research as imported authority with cited evidence and provenance edges.
#[test]
fn researched_answer_reconciles_citations_without_claiming_user_authority() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project(
            "Browser tool",
            "Build a portable VR planning tool, but the target platform is undecided.",
        )
        .expect("the project should initialize");
    let question = service
        .inspect_project(project.id())
        .expect("the project snapshot should load")
        .questions
        .into_iter()
        .next()
        .expect("the project should contain a blocking question");
    let evidence = ResearchEvidence::new(
        "WebAssembly is available in modern browser engines.",
        "https://developer.mozilla.org/en-US/docs/WebAssembly",
        "WebAssembly",
        EvidenceReliability::High,
        None,
    )
    .expect("the cited evidence should validate");
    let researched = ResearchedAnswer::new(
        "Support current evergreen browsers through WebAssembly.",
        Some("Codex recommendation based on cited platform documentation."),
        vec![evidence],
    )
    .expect("the researched answer should validate");

    service
        .answer_question_from_research(&question.id, &researched)
        .expect("the researched answer and evidence should commit atomically");
    let snapshot = service
        .inspect_project(project.id())
        .expect("the researched snapshot should load");
    let answer = snapshot
        .answers
        .iter()
        .find(|answer| answer.question_id == question.id)
        .expect("the researched answer should persist");

    assert_eq!(answer.source, AnswerSource::Imported);
    assert_eq!(snapshot.evidence.len(), 1);
    assert!(snapshot.requirements.iter().any(|requirement| {
        requirement.source_type == SourceType::Research
            && requirement.source_reference == answer.display_id
    }));
    assert!(snapshot.traces.iter().any(|trace| {
        trace.source_id == snapshot.evidence[0].id
            && trace.target_id == answer.id
            && trace.relationship == project_init::domain::TraceRelationship::Supports
    }));
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

/// Creates a user-authored question with visible conservative priority defaults through the workflow boundary.
#[test]
fn conservative_question_capture_persists_structured_defaults() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Question capture",
            "The project scope is explicit.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The project is local.","impact":"low"}]}"#,
        )
        .expect("the project should initialize without generated questions");

    let question = service
        .ask_question(
            project.id(),
            AskQuestionRequest::conservative("Which release channel should we use?"),
        )
        .expect("the structured user question should persist");

    assert_eq!(question.display_id, "Q-001");
    assert_eq!(question.impact, Impact::Medium);
    assert_eq!(question.uncertainty, Uncertainty::High);
    assert_eq!(question.cost_of_being_wrong, CostOfBeingWrong::Medium);
    assert_eq!(question.priority.score(), 45);
    assert_eq!(question.project_id, project.id().clone());
    assert_eq!(
        question.rationale,
        "Captured by the user in the contextual project workbench."
    );
}

/// Rejects malformed question capture before consuming a stable display identifier.
#[test]
fn invalid_question_capture_preserves_sequence_and_allows_recovery() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Question recovery",
            "The project scope is explicit.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The project is local.","impact":"low"}]}"#,
        )
        .expect("the project should initialize without generated questions");

    assert!(
        service
            .ask_question(project.id(), AskQuestionRequest::conservative("   "))
            .is_err()
    );
    let recovered = service
        .ask_question(
            project.id(),
            AskQuestionRequest::conservative("What is the rollout order?"),
        )
        .expect("a valid question should succeed after rejection");

    assert_eq!(recovered.display_id, "Q-001");
    assert_eq!(
        service
            .inspect_project(project.id())
            .expect("the project should remain readable")
            .questions
            .len(),
        1
    );
}
