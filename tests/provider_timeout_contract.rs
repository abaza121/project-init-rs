use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use project_init::agents::{
    AgentClient, AgentError, AnalysisRequest, AutoAnswerClient, CancellationToken, CodexCliClient,
    CodexCliConfig, ConfiguredProvider, OpenCodeCliClient, OpenCodeCliConfig, ResearchCandidate,
    ResearchJudgmentRequest, ResearchPlanRequest, ResearchQuestionContext, ResearchRequest,
    ResearchedAnswer,
};
use tokio::sync::mpsc;

const TIMEOUT: Duration = Duration::from_millis(600);
const ANSWER: &str = r#"{"answer_text":"Use the supported platform.","notes":null,"evidence":[{"claim":"The platform is supported.","source":"https://example.com/source","source_title":"Primary source","reliability":"high","notes":null}]}"#;

/// Compiles a portable offline provider once and keeps its temporary executable alive for all tests.
fn fixture_executable() -> PathBuf {
    static FIXTURE: OnceLock<tempfile::TempDir> = OnceLock::new();
    let directory = FIXTURE.get_or_init(|| {
        let directory = tempfile::tempdir().unwrap();
        let output = std::process::Command::new("rustc")
            .arg("--edition=2024")
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/timeout_provider.rs"
            ))
            .arg("-o")
            .arg(
                directory
                    .path()
                    .join(format!("provider{}", std::env::consts::EXE_SUFFIX)),
            )
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        directory
    });
    directory
        .path()
        .join(format!("provider{}", std::env::consts::EXE_SUFFIX))
}

/// Configures a real subprocess adapter with isolated activity and response fixtures.
fn provider(codex: bool, mode: &str, response: &str) -> (tempfile::TempDir, ConfiguredProvider) {
    let executable = fixture_executable();
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("mode"), mode).unwrap();
    let wire_response = response.to_owned();
    std::fs::write(directory.path().join("response"), wire_response).unwrap();
    let client = if codex {
        ConfiguredProvider::Codex(
            CodexCliClient::new(CodexCliConfig {
                executable,
                working_directory: directory.path().to_owned(),
                timeout: TIMEOUT,
                history_capacity: 4,
            })
            .with_skills_disabled(false),
        )
    } else {
        ConfiguredProvider::OpenCode(OpenCodeCliClient::new(OpenCodeCliConfig {
            executable,
            working_directory: directory.path().to_owned(),
            timeout: TIMEOUT,
            history_capacity: 4,
        }))
    };
    (directory, client)
}

/// Supplies one eligible blocking question to exercise the real coordinator adapter.
fn plan_request() -> ResearchPlanRequest {
    ResearchPlanRequest::new(
        "{}".to_owned(),
        "q1",
        vec![
            ResearchQuestionContext::new(
                "q1",
                "Q-001",
                "Which platform?",
                "It affects architecture.",
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

/// Preserves caller-relative data and staging directories after the server changes its cwd.
#[tokio::test]
async fn opencode_resolves_relative_directories_before_server_routing() {
    use project_init::agents::{DocumentationClient, DocumentationRequest};

    let directory = tempfile::Builder::new()
        .prefix("opencode routing ")
        .tempdir_in(".")
        .unwrap();
    let data_dir = PathBuf::from(".").join(directory.path().file_name().unwrap());
    assert!(data_dir.is_relative());
    let absolute = std::fs::canonicalize(directory.path()).unwrap();
    std::fs::write(directory.path().join("mode"), "directory").unwrap();
    std::fs::write(
        directory.path().join("response"),
        r#"{"question_ids":["q1"]}"#,
    )
    .unwrap();
    std::fs::write(
        directory.path().join("expected-directory"),
        absolute.to_str().unwrap(),
    )
    .unwrap();
    let client = OpenCodeCliClient::new(OpenCodeCliConfig {
        executable: fixture_executable(),
        working_directory: data_dir.clone(),
        timeout: Duration::from_secs(5),
        history_capacity: 4,
    });
    let (activity, _receiver) = mpsc::channel(4);
    let plan = client
        .plan(plan_request(), activity, CancellationToken::new())
        .await;
    assert!(plan.is_ok(), "relative data directory failed: {plan:?}");

    // The shared server must route later documentation to staging, not its own cwd.
    let staging = data_dir.join("staging output");
    std::fs::create_dir(&staging).unwrap();
    let expected_staging = std::fs::canonicalize(&staging).unwrap();
    std::fs::write(
        directory.path().join("expected-directory"),
        expected_staging.to_str().unwrap(),
    )
    .unwrap();
    let result = client
        .execute(
            DocumentationRequest::generation(
                "{}".to_owned(),
                vec!["README.md".to_owned()],
                staging,
            ),
            None,
            CancellationToken::new(),
        )
        .await;
    assert!(
        result.is_ok(),
        "relative staging directory failed: {result:?}"
    );
}

/// Proves that coordinator, worker, and judge survive the old hard limit on either output stream.
#[tokio::test]
async fn auto_answer_stages_survive_active_provider_beyond_timeout() {
    // Leave the tiny UI channel undrained: liveness must not depend on display backpressure.
    for codex in [true, false] {
        for mode in ["stdout", "stderr"] {
            let (_directory, client) = provider(codex, mode, r#"{"question_ids":["q1"]}"#);
            let (activity, _receiver) = mpsc::channel(1);
            let plan = client
                .plan(plan_request(), activity, CancellationToken::new())
                .await;
            assert!(plan.is_ok(), "codex={codex}, mode={mode}, plan={plan:?}");

            let (_directory, client) = provider(codex, mode, ANSWER);
            let (activity, _receiver) = mpsc::channel(1);
            let request =
                ResearchRequest::new("{}".to_owned(), "q1", "Which platform?", "Architecture.")
                    .unwrap()
                    .for_delegated_auto_answer();
            let answer =
                AutoAnswerClient::research(&client, request, activity, CancellationToken::new())
                    .await;
            assert!(
                answer.is_ok(),
                "codex={codex}, mode={mode}, answer={answer:?}"
            );

            let judged = format!(
                r#"{{"answers":[{{"question_id":"q1",{}}}]}}"#,
                &ANSWER[1..ANSWER.len() - 1]
            );
            let (_directory, client) = provider(codex, mode, &judged);
            let (activity, _receiver) = mpsc::channel(1);
            let candidate =
                ResearchCandidate::new("q1", ResearchedAnswer::from_json(ANSWER).unwrap()).unwrap();
            let request = ResearchJudgmentRequest::new("{}".to_owned(), vec![candidate]).unwrap();
            let judgment = client
                .judge(request, activity, CancellationToken::new())
                .await;
            assert!(
                judgment.is_ok(),
                "codex={codex}, mode={mode}, judgment={judgment:?}"
            );
        }
    }
}

/// Retains distinct inactivity failures for silent auto-answer subprocesses.
#[tokio::test]
async fn silent_auto_answer_provider_still_times_out() {
    for codex in [true, false] {
        let (_directory, client) = provider(codex, "silent", r#"{"question_ids":["q1"]}"#);
        let (activity, _receiver) = mpsc::channel(1);
        let result = client
            .plan(plan_request(), activity, CancellationToken::new())
            .await;
        assert!(matches!(result, Err(AgentError::Inactive)), "{result:?}");
    }
}

/// Keeps initial brief analysis bounded even when the subprocess continues emitting activity.
#[tokio::test]
async fn initial_analysis_retains_hard_timeout() {
    for codex in [true, false] {
        let (_directory, client) = provider(codex, "stdout", r#"{"findings":[]}"#);
        let (activity, _receiver) = mpsc::channel(1);
        let result = client
            .analyze(
                AnalysisRequest::new("Test", "Build a tool.").unwrap(),
                activity,
                CancellationToken::new(),
            )
            .await;
        assert!(matches!(result, Err(AgentError::TimedOut)), "{result:?}");
    }
}

/// Cancels live auto-answer work without waiting for the inactivity deadline.
#[tokio::test]
async fn active_auto_answer_remains_cancellable() {
    for codex in [true, false] {
        let (_directory, client) = provider(codex, "stdout", r#"{"question_ids":["q1"]}"#);
        let (activity, mut receiver) = mpsc::channel(4);
        let cancellation = CancellationToken::new();
        let cancel = cancellation.clone();
        let task =
            tokio::spawn(async move { client.plan(plan_request(), activity, cancellation).await });
        receiver.recv().await.unwrap();
        // The initial lifecycle event precedes spawn; wait for actual subprocess output too.
        tokio::time::timeout(TIMEOUT, receiver.recv())
            .await
            .unwrap()
            .unwrap();
        cancel.cancel();
        let result = tokio::time::timeout(TIMEOUT, task).await.unwrap().unwrap();
        assert!(matches!(result, Err(AgentError::Cancelled)), "{result:?}");
    }
}
