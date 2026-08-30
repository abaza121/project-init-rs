use project_init::agents::{
    ActivityEvent, ActivityHistory, ActivityKind, JudgedResearchBatch, LocalDevice,
    LocalHttpConfig, LocalHttpProvider, LocalRuntimeConfig, ResearchBatchPlan,
    decode_codex_jsonl_event,
};
use std::path::PathBuf;
use std::time::Duration;

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

/// Accepts only a loopback local inference endpoint with coherent bounded deadlines.
#[test]
fn local_http_configuration_is_loopback_only_and_bounded() {
    let config = LocalHttpConfig::new(
        "http://127.0.0.1:1234/v1",
        "default",
        Duration::from_secs(30),
        Duration::from_secs(10 * 60),
        200,
    )
    .expect("a bounded loopback endpoint should be accepted");

    assert_eq!(config.endpoint().as_str(), "http://127.0.0.1:1234/v1/");
    for endpoint in [
        "https://127.0.0.1:1234/v1",
        "http://192.168.1.10:1234/v1",
        "http://example.com/v1",
        "http://127.0.0.1:0/v1",
    ] {
        assert!(
            LocalHttpConfig::new(
                endpoint,
                "default",
                Duration::from_secs(30),
                Duration::from_secs(10 * 60),
                200,
            )
            .is_err(),
            "unsafe local endpoint should be rejected: {endpoint}"
        );
    }
}

/// Builds a hardened CPU container without exposing execution tools or host directories.
#[test]
fn local_runtime_arguments_are_hardened_and_model_scoped() {
    let runtime = LocalRuntimeConfig::new(
        PathBuf::from(r"C:\Models\project-init\gemma-4-12b"),
        "gemma-4-12b-it-qat-q4_0.gguf",
        "ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0",
        LocalDevice::Cpu,
        1234,
    )
    .expect("a pinned CPU runtime should be accepted");
    let arguments = runtime
        .docker_arguments()
        .into_iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let joined = arguments.join(" ");

    assert!(joined.contains("127.0.0.1:1234:1234"));
    assert!(joined.contains("readonly"));
    assert!(joined.contains("--cap-drop ALL"));
    assert!(joined.contains("no-new-privileges"));
    assert!(joined.contains("--read-only"));
    assert!(joined.contains("--enable-search"));
    assert!(joined.contains("--cpu"));
    assert!(!joined.contains("--gpus"));
    assert!(!joined.contains("--agent"));
    assert!(!joined.contains("--enable-code-execution"));
    assert!(!joined.contains("--enable-shell"));
}

/// Rejects floating images and adds only the explicit GPU capability in CUDA mode.
#[test]
fn local_runtime_requires_a_pinned_image_and_explicit_device() {
    assert!(
        LocalRuntimeConfig::new(
            PathBuf::from(r"C:\Models\project-init\gemma-4-12b"),
            "gemma-4-12b-it-qat-q4_0.gguf",
            "ghcr.io/ericlbuehler/mistral.rs:latest",
            LocalDevice::Cpu,
            1234,
        )
        .is_err()
    );
    let runtime = LocalRuntimeConfig::new(
        PathBuf::from(r"C:\Models\project-init\gemma-4-12b"),
        "gemma-4-12b-it-qat-q4_0.gguf",
        "ghcr.io/ericlbuehler/mistral.rs:cuda128-sm89-0.9.0",
        LocalDevice::Cuda,
        1234,
    )
    .expect("a versioned CUDA image should be accepted");
    let arguments = runtime
        .docker_arguments()
        .into_iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(" ");

    assert!(arguments.contains("--gpus all"));
    assert!(arguments.contains("--paged-attn on"));
    assert!(!arguments.contains("--cpu"));
}

/// Rejects a managed runtime whose published port does not match the HTTP endpoint.
#[test]
fn managed_local_runtime_must_match_the_provider_endpoint() {
    let provider = LocalHttpProvider::new(
        LocalHttpConfig::new(
            "http://127.0.0.1:1234/v1",
            "default",
            Duration::from_secs(30),
            Duration::from_secs(10 * 60),
            200,
        )
        .expect("the local endpoint should be valid"),
    )
    .expect("the HTTP client should build");
    let runtime = LocalRuntimeConfig::new(
        PathBuf::from(r"C:\Models\project-init\gemma-4-12b"),
        "gemma-4-12b-it-qat-q4_0.gguf",
        "ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0",
        LocalDevice::Cpu,
        4321,
    )
    .expect("the pinned runtime should be valid");

    assert!(provider.with_runtime(runtime).is_err());
}

/// Rejects IPv6 managed endpoints because the hardened Docker publish address is IPv4 loopback.
#[test]
fn managed_local_runtime_requires_its_exact_publish_host() {
    let provider = LocalHttpProvider::new(
        LocalHttpConfig::new(
            "http://[::1]:1234/v1",
            "default",
            Duration::from_secs(30),
            Duration::from_secs(10 * 60),
            200,
        )
        .expect("numeric IPv6 loopback remains valid for an unmanaged HTTP adapter"),
    )
    .expect("the HTTP client should build");
    let runtime = LocalRuntimeConfig::new(
        PathBuf::from(r"C:\Models\project-init\gemma-4-12b"),
        "gemma-4-12b-it-qat-q4_0.gguf",
        "ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0",
        LocalDevice::Cpu,
        1234,
    )
    .expect("the pinned runtime should be valid");

    assert!(provider.with_runtime(runtime).is_err());
}
