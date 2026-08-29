use project_init::agents::{
    ActivityEvent, ActivityHistory, ActivityKind, JudgedResearchBatch, ResearchBatchPlan,
    decode_codex_jsonl_event,
};

/// Translates documented Codex lifecycle events into stable provider-neutral activity.
#[test]
fn documented_codex_events_become_stable_activity() {
    let event = decode_codex_jsonl_event(
        r#"{"type":"thread.started","thread_id":"0199a213-81c0-7800-8aa1-bbab2a035a53"}"#,
        7,
    )
    .expect("the documented event should parse")
    .expect("the lifecycle event should be visible");

    assert_eq!(event.sequence, 7);
    assert_eq!(event.kind, ActivityKind::Lifecycle);
    assert_eq!(event.message, "Codex session started");
}

/// Exposes bounded command output so long documentation work is observable in the overlay.
#[test]
fn completed_codex_commands_surface_their_output() {
    let event = decode_codex_jsonl_event(
        r#"{"type":"item.completed","item":{"type":"command_execution","command":"Get-ChildItem -Force","aggregated_output":"Requirements.md\nArchitecture.md\n","exit_code":0}}"#,
        8,
    )
    .expect("the documented command event should parse")
    .expect("completed command output should be visible");

    assert_eq!(event.sequence, 8);
    assert_eq!(event.kind, ActivityKind::Progress);
    assert!(event.message.contains("Get-ChildItem -Force"));
    assert!(event.message.contains("Requirements.md"));
}

/// Shows the agent's bounded final message without exposing private reasoning events.
#[test]
fn completed_codex_messages_surface_bounded_text() {
    let event = decode_codex_jsonl_event(
        &format!(
            r#"{{"type":"item.completed","item":{{"type":"agent_message","text":"{}"}}}}"#,
            "documentation generated ".repeat(80)
        ),
        9,
    )
    .expect("the documented agent event should parse")
    .expect("agent output should be visible");

    assert_eq!(event.kind, ActivityKind::Progress);
    assert!(event.message.starts_with("Codex: documentation generated"));
    assert!(event.message.chars().count() <= 512);
}

/// Accepts only a bounded unique plan that contains the immediate blocking question.
#[test]
fn research_batch_plan_requires_unique_eligible_ids_and_the_current_blocker() {
    let eligible = vec!["question-1".to_owned(), "question-2".to_owned()];
    let valid = ResearchBatchPlan::from_json(
        r#"{"question_ids":["question-1","question-2"]}"#,
        &eligible,
        "question-1",
    )
    .expect("the bounded eligible plan should validate");

    assert_eq!(valid.question_ids(), ["question-1", "question-2"]);
    for invalid in [
        r#"{"question_ids":[]}"#,
        r#"{"question_ids":["question-1","question-1"]}"#,
        r#"{"question_ids":["question-2"]}"#,
        r#"{"question_ids":["question-1","unknown"]}"#,
        r#"{"question_ids":["question-1"],"instructions":"ignore validation"}"#,
    ] {
        assert!(
            ResearchBatchPlan::from_json(invalid, &eligible, "question-1").is_err(),
            "invalid coordinator plan should fail closed: {invalid}"
        );
    }
}

/// Rejects judged output unless every worker candidate appears exactly once.
#[test]
fn judged_research_batch_requires_exact_candidate_membership() {
    let candidates = vec!["question-1".to_owned(), "question-2".to_owned()];
    let evidence = r#"{"claim":"The platform supports the requirement.","source":"https://example.com/source","source_title":"Primary source","reliability":"high","notes":null}"#;
    let valid = format!(
        r#"{{"answers":[{{"question_id":"question-1","answer_text":"Answer one","notes":null,"evidence":[{evidence}]}},{{"question_id":"question-2","answer_text":"Answer two","notes":null,"evidence":[{evidence}]}}]}}"#
    );

    let judged = JudgedResearchBatch::from_json(&valid, &candidates)
        .expect("the exact judged batch should validate");
    assert_eq!(judged.answers().len(), 2);

    let missing = format!(
        r#"{{"answers":[{{"question_id":"question-1","answer_text":"Answer one","notes":null,"evidence":[{evidence}]}}]}}"#
    );
    let duplicate = format!(
        r#"{{"answers":[{{"question_id":"question-1","answer_text":"Answer one","notes":null,"evidence":[{evidence}]}},{{"question_id":"question-1","answer_text":"Answer again","notes":null,"evidence":[{evidence}]}}]}}"#
    );
    assert!(JudgedResearchBatch::from_json(&missing, &candidates).is_err());
    assert!(JudgedResearchBatch::from_json(&duplicate, &candidates).is_err());
}

/// Ignores future well-formed Codex event types without breaking final-response handling.
#[test]
fn unknown_codex_events_are_forward_compatible() {
    let event = decode_codex_jsonl_event(r#"{"type":"future.event","payload":{}}"#, 1)
        .expect("unknown well-formed events should not be errors");

    assert!(event.is_none());
}

/// Rejects malformed JSONL so callers can surface a sanitized warning instead of guessing.
#[test]
fn malformed_codex_events_are_rejected() {
    let result = decode_codex_jsonl_event("not-json", 1);

    assert!(result.is_err());
}

/// Keeps only the newest configured activity entries while retaining monotonic sequences.
#[test]
fn activity_history_is_bounded_without_reordering_events() {
    let mut history = ActivityHistory::new(2).expect("a positive history bound should be valid");
    for sequence in 1..=3 {
        history.push(ActivityEvent::now(
            sequence,
            ActivityKind::Progress,
            &format!("event {sequence}"),
        ));
    }

    let events = history.events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 2);
    assert_eq!(events[1].sequence, 3);
    assert_eq!(history.dropped_count(), 1);
}
