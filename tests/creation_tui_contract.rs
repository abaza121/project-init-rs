use project_init::agents::{ActivityEvent, ActivityKind};
use project_init::tui::{CreationOutcome, CreationState};

/// Appends live activity and keeps the newest event visible until the user scrolls away.
#[test]
fn creation_state_follows_new_activity_by_default() {
    let mut state = CreationState::new("Calm Fishing VR", 2);
    for sequence in 1..=3 {
        state.push_activity(ActivityEvent::now(
            sequence,
            ActivityKind::Progress,
            &format!("step {sequence}"),
        ));
    }

    assert_eq!(state.visible_activity()[0].message, "step 2");
    assert_eq!(state.visible_activity()[1].message, "step 3");
}

/// Cancels a running transient session immediately without inventing a project result.
#[test]
fn escape_transitions_creation_to_cancelled() {
    let mut state = CreationState::new("Calm Fishing VR", 10);

    state.cancel();

    assert_eq!(state.outcome(), CreationOutcome::Cancelled);
    assert!(state.project_id().is_none());
}

/// Preserves a completed project identity so the caller can print or reopen the result.
#[test]
fn successful_creation_records_the_committed_project_identity() {
    let mut state = CreationState::new("Calm Fishing VR", 10);

    state.complete("project-id");

    assert_eq!(state.outcome(), CreationOutcome::Completed);
    assert_eq!(state.project_id(), Some("project-id"));
}
