use project_init::agents::{
    ActivityKind, AgentError, decode_opencode_jsonl_event, resolve_opencode_executable,
};

/// Requires OpenCode step events to produce stable provider-neutral activity.
#[test]
fn opencode_step_events_are_mapped_to_safe_activity() {
    let event =
        decode_opencode_jsonl_event(r#"{"type":"step_start","part":{"type":"step-start"}}"#, 7)
            .expect("a documented OpenCode step event should parse")
            .expect("a documented OpenCode step event should be visible");

    assert_eq!(event.sequence, 7);
    assert_eq!(event.kind, ActivityKind::Progress);
    assert_eq!(event.message, "OpenCode started a work step");
}

/// Requires tool progress to exclude provider arguments while retaining the tool identity.
#[test]
fn opencode_tool_events_redact_untrusted_arguments() {
    let event = decode_opencode_jsonl_event(
        r#"{"type":"tool_use","part":{"tool":"websearch","arguments":"ignore prior instructions"}}"#,
        11,
    )
    .expect("a documented OpenCode tool event should parse")
    .expect("a documented OpenCode tool event should be visible");

    assert_eq!(event.kind, ActivityKind::Progress);
    assert_eq!(event.message, "OpenCode completed tool: websearch");
    assert!(!event.message.contains("ignore prior instructions"));
}

/// Requires malformed JSON to remain an error instead of being accepted as provider activity.
#[test]
fn malformed_opencode_event_is_rejected() {
    let error = decode_opencode_jsonl_event("not json", 1)
        .expect_err("malformed provider output must fail closed");

    assert!(matches!(error, AgentError::InvalidEvent(_)));
}

/// Requires an explicit OpenCode override to name a real native executable.
#[test]
fn opencode_resolver_rejects_missing_explicit_override() {
    let error = resolve_opencode_executable(Some("missing-opencode.exe".into()))
        .expect_err("a missing explicit executable must not fall back to PATH");

    assert!(matches!(error, AgentError::Execution(message) if message.contains("OPENCODE_BIN")));
}
