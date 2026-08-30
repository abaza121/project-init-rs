use chrono::{TimeZone, Utc};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use project_init::agents::{ActivityEvent, ActivityKind};
use project_init::domain::{
    ApprovalPolicy, CostOfBeingWrong, Impact, ProjectId, Uncertainty, WorkflowStep,
};
use project_init::storage::SqliteStore;
use project_init::tui::{
    WorkspaceCommand, WorkspaceDirective, WorkspaceFocus, WorkspaceSection, WorkspaceState,
    apply_interactive_workspace_command, apply_workspace_command, render_workspace,
};
use project_init::workflow::{
    AskQuestionRequest, AutoAnswerActor, AutoAnswerProgress, AutoAnswerStage, ProjectService,
};
use ratatui::Terminal;
use ratatui::backend::TestBackend;

/// Creates a project with no generated questions so each test controls the exact queue.
fn empty_queue_project() -> (ProjectService, ProjectId) {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Workbench project",
            "The project scope is explicit.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The tool is local.","impact":"low"}]}"#,
        )
        .expect("the project should initialize without generated questions");
    (service, project.id().clone())
}

/// Routes plain composer text to the selected open question without requiring a CLI round trip.
#[test]
fn plain_composer_text_answers_the_selected_question() {
    let (mut service, project_id) = empty_queue_project();
    let question = service
        .ask_question(
            &project_id,
            AskQuestionRequest::conservative("Which platform launches first?"),
        )
        .expect("the question should persist");
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());

    state.focus_composer();
    state.set_composer("Meta Quest 3");
    let command = state
        .submit_composer()
        .expect("the contextual answer should parse")
        .expect("submitting should produce a workflow command");

    assert_eq!(
        command,
        WorkspaceCommand::Answer {
            question_id: question.id,
            answer: "Meta Quest 3".to_owned(),
        }
    );
}

/// Keeps invalid contextual input available for correction when no open question is selected.
#[test]
fn context_free_plain_text_is_rejected_without_clearing_the_composer() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());

    state.focus_composer();
    state.set_composer("This text has no authoritative target");

    assert!(state.submit_composer().is_err());
    assert_eq!(state.composer(), "This text has no authoritative target");
}

/// Prevents a question hidden behind another section from remaining an implicit answer target.
#[test]
fn plain_text_cannot_answer_a_question_hidden_by_another_section() {
    let (mut service, project_id) = empty_queue_project();
    service
        .ask_question(
            &project_id,
            AskQuestionRequest::conservative("Which platform launches first?"),
        )
        .expect("the question should persist");
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());

    state
        .handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE))
        .expect("the section should change");
    assert_eq!(state.section(), WorkspaceSection::Findings);
    state.focus_composer();
    state.set_composer("Meta Quest 3");

    assert!(state.submit_composer().is_err());
    assert_eq!(state.composer(), "Meta Quest 3");
}

/// Requires slash command names to match exactly rather than accepting misleading prefixes.
#[test]
fn unsupported_command_prefixes_are_rejected_without_consuming_input() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.focus_composer();

    for invalid in ["/answering Q-001 text", "/threshold-extra 45"] {
        state.set_composer(invalid);
        assert!(state.submit_composer().is_err());
        assert_eq!(state.composer(), invalid);
    }
}

/// Parses explicit automatic answering without treating similarly prefixed text as a command.
#[test]
fn auto_answer_command_activates_only_on_an_exact_name() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.focus_composer();
    state.set_composer("/auto-answer");

    assert_eq!(
        state.submit_composer().expect("the command should parse"),
        Some(WorkspaceCommand::AutoAnswer)
    );

    state.set_composer("/auto-answer-now");
    assert!(state.submit_composer().is_err());
}

/// Opens structured question capture with editable conservative defaults instead of inferring hidden values.
#[test]
fn ask_command_opens_a_structured_draft_with_conservative_defaults() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());

    state.focus_composer();
    state.set_composer("/ask Which storefront should host the release?");
    assert!(
        state
            .submit_composer()
            .expect("the ask command should parse")
            .is_none()
    );

    let draft = state
        .question_draft()
        .expect("structured capture should be visible");
    assert_eq!(draft.prompt(), "Which storefront should host the release?");
    assert_eq!(draft.impact(), Impact::Medium);
    assert_eq!(draft.uncertainty(), Uncertainty::High);
    assert_eq!(draft.cost_of_being_wrong(), CostOfBeingWrong::Medium);
    assert_eq!(state.focus(), WorkspaceFocus::QuestionDraft);
}

/// Distinguishes submission from multiline composition when Crossterm reports modified Enter.
#[test]
fn enter_submits_and_control_enter_inserts_a_newline() {
    let (mut service, project_id) = empty_queue_project();
    service
        .ask_question(
            &project_id,
            AskQuestionRequest::conservative("What is the launch rationale?"),
        )
        .expect("the question should persist");
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.focus_composer();
    state.set_composer("First line");

    let newline = state
        .handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL))
        .expect("modified Enter should be accepted");
    assert!(newline.is_none());
    assert_eq!(state.composer(), "First line\n");

    state.set_composer("First line\nSecond line");
    let submission = state
        .handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
        .expect("plain Enter should submit");
    assert!(matches!(submission, Some(WorkspaceCommand::Answer { .. })));
}

/// Cycles focus across content, composer, and navigation without trapping keyboard users.
#[test]
fn tab_cycles_through_every_primary_workspace_surface() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());

    assert_eq!(state.focus(), WorkspaceFocus::Content);
    state
        .handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE))
        .expect("content should yield focus");
    assert_eq!(state.focus(), WorkspaceFocus::Composer);
    state
        .handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE))
        .expect("composer should yield focus");
    assert_eq!(state.focus(), WorkspaceFocus::Navigation);
    state
        .handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE))
        .expect("navigation should yield focus");
    assert_eq!(state.focus(), WorkspaceFocus::Content);
}

/// Moves the contextual answer target through open questions without exceeding queue bounds.
#[test]
fn question_navigation_is_bounded_and_changes_the_composer_target() {
    let (mut service, project_id) = empty_queue_project();
    let high = service
        .ask_question(
            &project_id,
            AskQuestionRequest::new(
                "Which architecture is authoritative?",
                "The decision affects implementation.",
                Impact::High,
                Uncertainty::High,
                CostOfBeingWrong::High,
            ),
        )
        .expect("the high-priority question should persist");
    let low = service
        .ask_question(
            &project_id,
            AskQuestionRequest::new(
                "Which accent color is preferred?",
                "The decision is cosmetic.",
                Impact::Low,
                Uncertainty::Low,
                CostOfBeingWrong::Low,
            ),
        )
        .expect("the low-priority question should persist");
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());

    assert_eq!(
        state
            .selected_question()
            .expect("a question is selected")
            .id,
        high.id
    );
    state
        .handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE))
        .expect("down should select the next open question");
    assert_eq!(
        state
            .selected_question()
            .expect("a question is selected")
            .id,
        low.id
    );
    state
        .handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE))
        .expect("down at the boundary should be harmless");
    assert_eq!(
        state
            .selected_question()
            .expect("a question is selected")
            .id,
        low.id
    );
    state
        .handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE))
        .expect("up should select the previous open question");
    assert_eq!(
        state
            .selected_question()
            .expect("a question is selected")
            .id,
        high.id
    );
}

/// Rejects an invalid threshold before emitting a command or consuming correctable input.
#[test]
fn invalid_threshold_command_preserves_the_composer_for_correction() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.focus_composer();
    state.set_composer("/threshold 126");

    assert!(state.submit_composer().is_err());
    assert_eq!(state.composer(), "/threshold 126");
    assert_eq!(state.snapshot().project.clarification_threshold(), 27);
}

/// Advances through consequential questions before continuing through lower-priority open work.
#[test]
fn queue_advances_by_threshold_then_continues_through_remaining_questions() {
    let (mut service, project_id) = empty_queue_project();
    let low = service
        .ask_question(
            &project_id,
            AskQuestionRequest::new(
                "Which optional accent color is preferred?",
                "The choice is cosmetic.",
                Impact::Low,
                Uncertainty::Low,
                CostOfBeingWrong::Low,
            ),
        )
        .expect("the low-priority question should persist");
    let high = service
        .ask_question(
            &project_id,
            AskQuestionRequest::new(
                "Which architecture boundary is authoritative?",
                "The choice affects the complete implementation.",
                Impact::High,
                Uncertainty::High,
                CostOfBeingWrong::High,
            ),
        )
        .expect("the high-priority question should persist");
    service
        .set_clarification_threshold(&project_id, 27)
        .expect("the threshold should persist");
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());

    assert_eq!(
        state
            .selected_question()
            .expect("the highest-priority question should be selected")
            .id,
        high.id
    );

    service
        .answer_question(&high.id, "The workflow service is authoritative.", None)
        .expect("the consequential question should reconcile");
    let next = service
        .inspect_project(&project_id)
        .expect("the reconciled snapshot should reload");
    state.apply_snapshot(next);

    assert_eq!(
        state
            .selected_question()
            .expect("the lower-priority queue should continue")
            .id,
        low.id
    );
}

/// Retains every equal-time activity record in the projected authoritative timeline.
#[test]
fn timeline_keeps_equal_time_records_without_deduplication() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let timestamp = Utc
        .with_ymd_and_hms(2026, 8, 29, 12, 0, 0)
        .single()
        .expect("the fixed timestamp should be valid");
    let activity = vec![
        ActivityEvent {
            sequence: 1,
            kind: ActivityKind::Progress,
            message: "First tied event".to_owned(),
            created_at: timestamp,
        },
        ActivityEvent {
            sequence: 2,
            kind: ActivityKind::Progress,
            message: "Second tied event".to_owned(),
            created_at: timestamp,
        },
    ];

    let state = WorkspaceState::new(snapshot, activity);
    let messages = state
        .timeline()
        .iter()
        .map(|entry| entry.message())
        .collect::<Vec<_>>();

    assert!(messages.contains(&"First tied event"));
    assert!(messages.contains(&"Second tied event"));
}

/// Places a question's current status at its authoritative update time rather than its creation time.
#[test]
fn timeline_uses_update_time_for_mutable_record_status() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Timeline project",
            "The launch platform is not chosen.",
            r#"{"findings":[{"kind":"unknown","statement":"The launch platform is not chosen.","impact":"high"}]}"#,
        )
        .expect("the project should initialize with one clarification");
    let question = service
        .inspect_project(project.id())
        .expect("the initial snapshot should load")
        .questions
        .into_iter()
        .next()
        .expect("the question should exist");
    service
        .answer_question(&question.id, "Meta Quest 3", None)
        .expect("the answer should reconcile");
    let snapshot = service
        .inspect_project(project.id())
        .expect("the reconciled snapshot should load");
    let updated_at = snapshot.questions[0].updated_at;
    let state = WorkspaceState::new(snapshot, Vec::new());
    let entry = state
        .timeline()
        .iter()
        .find(|entry| entry.message().starts_with(&question.display_id))
        .expect("the updated question should appear in the timeline");

    assert_eq!(entry.timestamp(), updated_at);
}

/// Reports every authoritative entity family changed by deterministic answer reconciliation.
#[test]
fn snapshot_diff_summarizes_multi_entity_reconciliation() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Diff project",
            "The launch platform is not chosen.",
            r#"{"findings":[{"kind":"unknown","statement":"The launch platform is not chosen.","impact":"high"}]}"#,
        )
        .expect("the project should initialize with exactly one clarification");
    let before = service
        .inspect_project(project.id())
        .expect("the initial snapshot should load");
    let question = before
        .questions
        .first()
        .expect("the fixture should contain an open question")
        .clone();
    let mut state = WorkspaceState::new(before, Vec::new());

    service
        .answer_question(&question.id, "Meta Quest 3", None)
        .expect("the answer should reconcile");
    let after = service
        .inspect_project(project.id())
        .expect("the reconciled snapshot should load");
    let diff = state.apply_snapshot(after);

    assert!(diff.questions_changed() >= 1);
    assert_eq!(diff.answers_added(), 1);
    assert!(diff.findings_changed() >= 1);
    assert!(diff.requirements_added() >= 1);
    assert!(diff.traces_added() >= 1);
    assert!(diff.project_status_changed());
}

/// Renders the project sections, contextual answer target, composer, and keyboard help in a compact terminal.
#[test]
fn workspace_render_exposes_context_and_controls() {
    let (mut service, project_id) = empty_queue_project();
    service
        .ask_question(
            &project_id,
            AskQuestionRequest::conservative("Which platform launches first?"),
        )
        .expect("the question should persist");
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let state = WorkspaceState::new(snapshot, Vec::new());
    let mut terminal = Terminal::new(TestBackend::new(88, 24))
        .expect("the deterministic terminal should initialize");

    terminal
        .draw(|frame| render_workspace(frame, &state))
        .expect("the workbench should render");
    let rendered = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    assert!(rendered.contains("Workbench project"));
    assert!(rendered.contains("QUESTIONS"));
    assert!(rendered.contains("ANSWER Q-001"));
    assert!(rendered.contains("/ask"));
    assert!(rendered.contains("Enter submit"));
    assert_eq!(state.section(), WorkspaceSection::Questions);
}

/// Marks the active structured question field so keyboard edits have visible context.
#[test]
fn question_draft_render_marks_the_active_field() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.focus_composer();
    state.set_composer("/ask Which platform launches first?");
    state
        .submit_composer()
        .expect("structured capture should open");
    let mut terminal = Terminal::new(TestBackend::new(88, 24))
        .expect("the deterministic terminal should initialize");

    terminal
        .draw(|frame| render_workspace(frame, &state))
        .expect("the question overlay should render");
    let rendered = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    assert!(rendered.contains("> Prompt:"));
}

/// Applies typed workbench commands through the workflow boundary and reloads authoritative state.
#[test]
fn workspace_commands_persist_and_reload_the_current_project() {
    let (mut service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the initial snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());

    let ask = WorkspaceCommand::AskQuestion(AskQuestionRequest::conservative(
        "Which release channel should launch first?",
    ));
    assert!(
        apply_workspace_command(&mut service, &mut state, ask)
            .expect("question creation should remain in the workbench")
    );
    let question = state
        .selected_question()
        .expect("the created question should become the current queue item")
        .clone();

    assert!(
        apply_workspace_command(
            &mut service,
            &mut state,
            WorkspaceCommand::SetClarificationThreshold(45),
        )
        .expect("threshold update should remain in the workbench")
    );
    assert_eq!(state.snapshot().project.clarification_threshold(), 45);

    assert!(
        apply_workspace_command(
            &mut service,
            &mut state,
            WorkspaceCommand::Answer {
                question_id: question.id,
                answer: "Stable channel".to_owned(),
            },
        )
        .expect("answer reconciliation should remain in the workbench")
    );
    assert_eq!(state.snapshot().answers.len(), 1);

    assert!(
        !apply_workspace_command(&mut service, &mut state, WorkspaceCommand::Quit)
            .expect("quit should be a mutation-free command")
    );
}

/// Emits an explicit resume command without inferring or defaulting an approval policy.
#[test]
fn resume_command_defers_first_run_policy_selection_to_the_workbench() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.focus_composer();
    state.set_composer("/resume");

    assert_eq!(
        state
            .submit_composer()
            .expect("the resume command should parse"),
        Some(WorkspaceCommand::Resume)
    );
}

/// Requires deliberate navigation before a first-run policy can be submitted.
#[test]
fn policy_selection_has_no_default_and_cannot_submit_until_chosen() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.open_policy_selection();

    assert_eq!(
        state
            .policy_selection()
            .expect("the selector should be visible")
            .selected(),
        None
    );
    assert!(
        state
            .handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
            .is_err(),
        "Enter must not silently choose a policy"
    );
    state
        .handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE))
        .expect("navigation should select the first policy");
    assert_eq!(
        state
            .handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
            .expect("the chosen policy should submit"),
        Some(WorkspaceCommand::StartRun(ApprovalPolicy::Strict))
    );
}

/// Parses decision authority and repair preparation as exact explicit commands.
#[test]
fn authority_commands_require_targets_reasons_and_exact_names() {
    let (mut service, project_id) = empty_queue_project();
    let decision = service
        .propose_decision(
            &project_id,
            "Launch channel",
            "Launch through the stable channel.",
            "The channel affects release risk.",
            true,
        )
        .expect("the decision should persist");
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.focus_composer();

    state.set_composer(&format!("/approve {} Ship it.", decision.display_id));
    assert_eq!(
        state.submit_composer().expect("approval should parse"),
        Some(WorkspaceCommand::ApproveDecision {
            decision_id: decision.id.clone(),
            reason: "Ship it.".to_owned(),
        })
    );

    state.set_composer("/repair");
    assert_eq!(
        state
            .submit_composer()
            .expect("repair preparation should parse"),
        Some(WorkspaceCommand::RequestRepair)
    );

    for invalid in [
        "/resume now",
        "/repair now",
        "/approve",
        "/rejecting D-001 no",
    ] {
        state.set_composer(invalid);
        assert!(state.submit_composer().is_err());
        assert_eq!(state.composer(), invalid);
    }
}

/// Renders the pending decision as a contextual authority card with exact next commands.
#[test]
fn approval_boundary_renders_a_guided_workbench_card() {
    let (mut service, project_id) = empty_queue_project();
    let decision = service
        .propose_decision(
            &project_id,
            "Release policy",
            "Ship through the stable channel.",
            "The channel changes release risk.",
            true,
        )
        .expect("the decision should persist");
    service
        .start_run(&project_id, ApprovalPolicy::Consequential)
        .expect("the run should start");
    let output = tempfile::tempdir().expect("the output directory should exist");
    let status = service
        .workflow_status(&project_id, output.path())
        .expect("the approval boundary should load");
    assert!(matches!(status.step, WorkflowStep::ApprovalRequired { .. }));
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.set_workflow_context(Some(status), Vec::new());
    let mut terminal = Terminal::new(TestBackend::new(92, 28))
        .expect("the deterministic terminal should initialize");

    terminal
        .draw(|frame| render_workspace(frame, &state))
        .expect("the approval card should render");
    let rendered = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    assert!(rendered.contains("APPROVAL REQUIRED"));
    assert!(rendered.contains(&decision.display_id));
    assert!(rendered.contains("/approve"));
    assert!(rendered.contains("/reject"));
    assert!(rendered.contains("/resume"));
}

/// Covers the workbench with cancellable progress only while external execution is active.
#[test]
fn active_execution_renders_the_codex_progress_overlay() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.begin_execution();
    state.push_execution_activity(ActivityEvent::now(
        1,
        ActivityKind::Lifecycle,
        "Generating project documentation",
    ));
    let mut terminal = Terminal::new(TestBackend::new(92, 28))
        .expect("the deterministic terminal should initialize");

    terminal
        .draw(|frame| render_workspace(frame, &state))
        .expect("the execution overlay should render");
    let rendered = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    assert!(rendered.contains("RUNNING WORKFLOW"));
    assert!(rendered.contains("Generating project documentation"));
    assert!(rendered.contains("Esc cancel"));
}

/// Renders actor-keyed auto-answer stages as separate coordinator, worker, and judge lanes.
#[test]
fn auto_answer_execution_renders_structured_progress_lanes() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.begin_auto_answer_execution();
    state.push_auto_answer_progress(AutoAnswerProgress::new(
        2,
        AutoAnswerActor::Worker {
            slot: 1,
            question_display_id: "Q-004".to_owned(),
        },
        AutoAnswerStage::Researching,
        "Comparing desktop deployment options",
        None,
    ));
    state.push_auto_answer_progress(AutoAnswerProgress::new(
        2,
        AutoAnswerActor::Judge,
        AutoAnswerStage::Waiting,
        "Waiting for Worker 1",
        None,
    ));
    let mut terminal = Terminal::new(TestBackend::new(100, 28))
        .expect("the deterministic terminal should initialize");

    terminal
        .draw(|frame| render_workspace(frame, &state))
        .expect("the auto-answer overlay should render");
    let rendered = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    assert!(rendered.contains("AUTO ANSWER"));
    assert!(rendered.contains("Worker 1"));
    assert!(rendered.contains("Q-004"));
    assert!(rendered.contains("Researching"));
    assert!(rendered.contains("Judge"));
    assert!(rendered.contains("Waiting for Worker 1"));
    assert!(!rendered.contains('%'));
}

/// Replaces every prior worker assignment when a newer automatic-answer batch starts.
#[test]
fn auto_answer_execution_renders_only_the_current_batch_lanes() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.begin_auto_answer_execution();
    state.push_auto_answer_progress(AutoAnswerProgress::new(
        1,
        AutoAnswerActor::Coordinator,
        AutoAnswerStage::Accepted,
        "Selected 3 question(s)",
        None,
    ));
    for (slot, question_display_id) in [(1, "Q-001"), (2, "Q-002"), (3, "Q-003")] {
        state.push_auto_answer_progress(AutoAnswerProgress::new(
            1,
            AutoAnswerActor::Worker {
                slot,
                question_display_id: question_display_id.to_owned(),
            },
            AutoAnswerStage::Ready,
            "Cited candidate ready",
            Some(5),
        ));
    }
    state.push_auto_answer_progress(AutoAnswerProgress::new(
        1,
        AutoAnswerActor::Judge,
        AutoAnswerStage::Accepted,
        "Accepted 3 judged answer(s)",
        None,
    ));

    state.push_auto_answer_progress(AutoAnswerProgress::new(
        2,
        AutoAnswerActor::Coordinator,
        AutoAnswerStage::Accepted,
        "Selected 2 question(s)",
        None,
    ));
    for (slot, question_display_id) in [(1, "Q-004"), (2, "Q-005")] {
        state.push_auto_answer_progress(AutoAnswerProgress::new(
            2,
            AutoAnswerActor::Worker {
                slot,
                question_display_id: question_display_id.to_owned(),
            },
            AutoAnswerStage::Researching,
            "Researching current assignment",
            None,
        ));
    }
    state.push_auto_answer_progress(AutoAnswerProgress::new(
        2,
        AutoAnswerActor::Judge,
        AutoAnswerStage::Waiting,
        "Waiting for current workers",
        None,
    ));
    state.push_auto_answer_progress(AutoAnswerProgress::new(
        1,
        AutoAnswerActor::Worker {
            slot: 3,
            question_display_id: "Q-003".to_owned(),
        },
        AutoAnswerStage::Ready,
        "Late stale update",
        Some(5),
    ));

    let mut terminal = Terminal::new(TestBackend::new(100, 28))
        .expect("the deterministic terminal should initialize");
    terminal
        .draw(|frame| render_workspace(frame, &state))
        .expect("the current automatic-answer batch should render");
    let rendered = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    assert!(rendered.contains("BATCH 2"));
    assert!(rendered.contains("Q-004"));
    assert!(rendered.contains("Q-005"));
    assert!(!rendered.contains("Q-001"));
    assert!(!rendered.contains("Q-002"));
    assert!(!rendered.contains("Q-003"));
}

/// Keeps the newest streamed Codex output visible when a long run exceeds the panel height.
#[test]
fn execution_overlay_follows_the_latest_codex_output() {
    let (service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    state.begin_execution();
    for index in 0..40 {
        state.push_execution_activity(ActivityEvent::now(
            index,
            ActivityKind::Progress,
            &format!("Codex output {index}"),
        ));
    }
    let mut terminal = Terminal::new(TestBackend::new(92, 20))
        .expect("the deterministic terminal should initialize");

    terminal
        .draw(|frame| render_workspace(frame, &state))
        .expect("the execution overlay should render");
    let rendered = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();

    assert!(rendered.contains("CODEX OUTPUT"));
    assert!(rendered.contains("Codex output 39"));
    assert!(!rendered.contains("Codex output 0"));
}

/// Opens policy selection without starting a run when first resume has no persisted policy.
#[test]
fn interactive_resume_without_a_run_requires_policy_selection_without_mutation() {
    let (mut service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    let output = tempfile::tempdir().expect("the output directory should exist");

    let directive = apply_interactive_workspace_command(
        &mut service,
        &mut state,
        WorkspaceCommand::Resume,
        output.path(),
    )
    .expect("first resume should open policy selection");

    assert_eq!(directive, WorkspaceDirective::Continue);
    assert!(state.policy_selection().is_some());
    assert!(
        service
            .active_run(&project_id)
            .expect("active-run inspection should succeed")
            .is_none()
    );
}

/// Requires explicit approval policy selection before first-run automatic clarification.
#[test]
fn interactive_auto_answer_without_a_run_requires_policy_selection() {
    let (mut service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    let output = tempfile::tempdir().expect("the output directory should exist");

    let directive = apply_interactive_workspace_command(
        &mut service,
        &mut state,
        WorkspaceCommand::AutoAnswer,
        output.path(),
    )
    .expect("automatic answering should open policy selection");

    assert_eq!(directive, WorkspaceDirective::Continue);
    assert!(state.policy_selection().is_some());
    assert!(
        service
            .active_run(&project_id)
            .expect("active-run inspection should succeed")
            .is_none()
    );
}

/// Persists an explicit policy before requesting one background execution.
#[test]
fn explicit_policy_selection_requests_one_background_run() {
    let (mut service, project_id) = empty_queue_project();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    let output = tempfile::tempdir().expect("the output directory should exist");

    let directive = apply_interactive_workspace_command(
        &mut service,
        &mut state,
        WorkspaceCommand::StartRun(ApprovalPolicy::Consequential),
        output.path(),
    )
    .expect("the policy should request execution");

    assert_eq!(
        directive,
        WorkspaceDirective::Execute { show_overlay: true }
    );
    assert_eq!(
        service
            .active_run(&project_id)
            .expect("foreground inspection should succeed")
            .expect("the selected run should persist")
            .policy,
        ApprovalPolicy::Consequential
    );
}

/// Avoids the Codex overlay when resume can only return an existing question boundary.
#[test]
fn question_boundary_resume_does_not_request_a_codex_overlay() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project("Question project", "Build a calm VR fishing game.")
        .expect("the project should initialize with a question");
    let project_id = project.id().clone();
    let snapshot = service
        .inspect_project(&project_id)
        .expect("the snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    let output = tempfile::tempdir().expect("the output directory should exist");

    let directive = apply_interactive_workspace_command(
        &mut service,
        &mut state,
        WorkspaceCommand::StartRun(ApprovalPolicy::Consequential),
        output.path(),
    )
    .expect("the question boundary should be observable");

    assert_eq!(
        directive,
        WorkspaceDirective::Execute {
            show_overlay: false
        }
    );
}
