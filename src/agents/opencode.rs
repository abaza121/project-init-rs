use std::env;
use std::ffi::{OsStr, OsString};
use std::future::Future;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;
use tokio::sync::mpsc;

use super::prompts::{
    ANALYSIS_SCHEMA, RESEARCH_ANSWER_SCHEMA, RESEARCH_JUDGMENT_SCHEMA, RESEARCH_PLAN_SCHEMA,
    analysis_prompt, documentation_prompt, repair_prompt, research_judgment_prompt,
    research_plan_prompt, research_prompt,
};
use super::{
    ActivityEvent, ActivityHistory, ActivityKind, AgentClient, AgentError, AgentExecution,
    AnalysisRequest, AutoAnswerClient, CancellationToken, DocumentationClient, DocumentationKind,
    DocumentationRequest, JudgedResearchBatch, ResearchBatchPlan, ResearchClient,
    ResearchJudgmentRequest, ResearchPlanRequest, ResearchRequest, ResearchedAnswer,
};

const MAX_JSONL_LINE_BYTES: usize = 64 * 1024;
const MAX_FINAL_RESPONSE_BYTES: usize = 1024 * 1024;

/// Resolves the native OpenCode CLI program used by child processes on the current host.
pub fn resolve_opencode_executable(configured: Option<OsString>) -> Result<PathBuf, AgentError> {
    if let Some(configured) = configured {
        let path = PathBuf::from(configured);
        if path.is_file() {
            return validate_native_executable(path);
        }
        return Err(AgentError::Execution(format!(
            "OPENCODE_BIN does not point to a file: {}",
            path.display()
        )));
    }
    let path = env::var_os("PATH").ok_or_else(missing_opencode_error)?;
    resolve_opencode_from_path(&path).ok_or_else(missing_opencode_error)
}

/// Rejects Windows shell shims because the subprocess adapter requires a native executable.
fn validate_native_executable(path: PathBuf) -> Result<PathBuf, AgentError> {
    #[cfg(windows)]
    if path
        .extension()
        .and_then(OsStr::to_str)
        .is_none_or(|extension| !extension.eq_ignore_ascii_case("exe"))
    {
        return Err(AgentError::Execution(format!(
            "OPENCODE_BIN must point to opencode.exe on Windows, not a shell shim: {}",
            path.display()
        )));
    }
    Ok(path)
}

/// Searches PATH for the native OpenCode executable used on the current platform.
#[cfg(windows)]
fn resolve_opencode_from_path(path: &OsStr) -> Option<PathBuf> {
    env::split_paths(path)
        .map(|directory| directory.join("opencode.exe"))
        .find(|candidate| candidate.is_file())
}

/// Searches PATH for the extensionless OpenCode executable used on Unix hosts.
#[cfg(not(windows))]
fn resolve_opencode_from_path(path: &OsStr) -> Option<PathBuf> {
    env::split_paths(path)
        .map(|directory| directory.join("opencode"))
        .find(|candidate| candidate.is_file())
}

/// Creates a stable setup error that explains how to select the required OpenCode program.
fn missing_opencode_error() -> AgentError {
    AgentError::Execution(
        "could not locate the OpenCode CLI executable; install OpenCode or set OPENCODE_BIN to its full path"
            .to_owned(),
    )
}

/// Converts one documented OpenCode JSON event into stable, non-sensitive activity.
pub fn decode_opencode_jsonl_event(
    line: &str,
    sequence: u32,
) -> Result<Option<ActivityEvent>, AgentError> {
    let event: Value =
        serde_json::from_str(line).map_err(|error| AgentError::InvalidEvent(error.to_string()))?;
    let Some(event_type) = event.get("type").and_then(Value::as_str) else {
        return Ok(None);
    };
    let mapped = match event_type {
        "step_start" => Some((
            ActivityKind::Progress,
            "OpenCode started a work step".to_owned(),
        )),
        "step_finish" => Some((
            ActivityKind::Progress,
            "OpenCode completed a work step".to_owned(),
        )),
        "tool_use" => opencode_tool_message(&event),
        "error" => Some((
            ActivityKind::Warning,
            "OpenCode reported an error".to_owned(),
        )),
        _ => None,
    };
    Ok(mapped.map(|(kind, message)| ActivityEvent::now(sequence, kind, &message)))
}

/// Maps tool completion without exposing provider-controlled arguments or output.
fn opencode_tool_message(event: &Value) -> Option<(ActivityKind, String)> {
    event
        .pointer("/part/tool")
        .and_then(Value::as_str)
        .filter(|tool| !tool.is_empty())
        .map(|tool| {
            (
                ActivityKind::Progress,
                format!(
                    "OpenCode completed tool: {}",
                    super::sanitize_terminal_text(tool)
                ),
            )
        })
}

/// Holds the process limits and working-directory policy for OpenCode invocations.
#[derive(Debug, Clone)]
pub struct OpenCodeCliConfig {
    pub executable: PathBuf,
    pub working_directory: PathBuf,
    pub timeout: Duration,
    pub history_capacity: usize,
}

/// Executes Project Init prompts through the authenticated OpenCode CLI profile.
#[derive(Debug, Clone)]
pub struct OpenCodeCliClient {
    config: OpenCodeCliConfig,
}

impl OpenCodeCliClient {
    /// Creates a client that preserves the user's OpenCode authentication and permission profile.
    pub const fn new(config: OpenCodeCliConfig) -> Self {
        Self { config }
    }

    /// Returns the immutable process settings used by later OpenCode invocations.
    pub const fn config(&self) -> &OpenCodeCliConfig {
        &self.config
    }

    /// Builds the documented non-interactive OpenCode command without auto-approving permissions.
    fn command_arguments(prompt: &str) -> Vec<OsString> {
        vec![
            OsString::from("run"),
            OsString::from("--format"),
            OsString::from("json"),
            OsString::from(prompt),
        ]
    }

    /// Adds a schema contract because OpenCode's documented run command has no output-schema flag.
    fn structured_prompt(prompt: &str, schema: &str) -> String {
        format!(
            "{prompt}\n\
             Return exactly one JSON object and no Markdown fences, explanation, or surrounding text.\n\
             The response must validate against this JSON Schema; every schema field is required exactly as specified.\n\
             <response_schema>\n{schema}\n</response_schema>\n"
        )
    }

    /// Executes one structured request and rejects absent or oversized final assistant text.
    async fn execute_structured_prompt(
        &self,
        schema: &str,
        prompt: &str,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<AgentExecution, AgentError> {
        let started_at = Utc::now();
        let run = self
            .execute_prompt(
                &Self::structured_prompt(prompt, schema),
                &self.config.working_directory,
                activity,
                cancellation,
                "Starting isolated OpenCode",
                TimeoutPolicy::Hard,
            )
            .await?;
        let response = run.response.ok_or_else(|| {
            AgentError::InvalidResponse("OpenCode returned no completed assistant text".to_owned())
        })?;
        Ok(AgentExecution {
            id: uuid::Uuid::new_v4().to_string(),
            provider: "opencode_cli".to_owned(),
            model: None,
            response,
            input_tokens: None,
            output_tokens: None,
            activity: run.history.events(),
            started_at,
            completed_at: Utc::now(),
        })
    }

    /// Executes generation or repair in the caller-owned staging directory.
    async fn execute_documentation_prompt(
        &self,
        prompt: &str,
        staging: &std::path::Path,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        std::fs::create_dir_all(staging)
            .map_err(|error| AgentError::Execution(format!("documentation staging: {error}")))?;
        let (sender, _receiver) = mpsc::channel(self.config.history_capacity);
        let activity = activity.unwrap_or(sender);
        self.execute_prompt(
            prompt,
            staging,
            activity,
            cancellation,
            "OpenCode started documentation work",
            TimeoutPolicy::Inactivity,
        )
        .await?;
        Ok(())
    }

    /// Runs one bounded OpenCode process and retains only the final completed text response.
    async fn execute_prompt(
        &self,
        prompt: &str,
        working_directory: &std::path::Path,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
        initial_message: &str,
        timeout_policy: TimeoutPolicy,
    ) -> Result<OpenCodeRun, AgentError> {
        let initial = ActivityEvent::now(1, ActivityKind::Lifecycle, initial_message);
        let _ = activity.try_send(initial.clone());
        let mut command = Command::new(&self.config.executable);
        command
            .args(Self::command_arguments(prompt))
            .current_dir(working_directory)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        let mut child = command
            .spawn()
            .map_err(|error| AgentError::Execution(format!("could not start OpenCode: {error}")))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| AgentError::Execution("OpenCode stdout was unavailable".to_owned()))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| AgentError::Execution("OpenCode stderr was unavailable".to_owned()))?;
        let (progress_sender, progress_receiver) = mpsc::channel(1);
        let activity_task = tokio::spawn(read_opencode_activity(
            stdout,
            activity,
            self.config.history_capacity,
            initial,
            progress_sender.clone(),
        ));
        let stderr_task = tokio::spawn(drain_reader(stderr, progress_sender));
        let outcome = match timeout_policy {
            TimeoutPolicy::Hard => {
                wait_with_deadline(child.wait(), cancellation, self.config.timeout).await
            }
            TimeoutPolicy::Inactivity => match wait_with_inactivity(
                child.wait(),
                progress_receiver,
                cancellation,
                self.config.timeout,
            )
            .await
            {
                InactivityOutcome::Completed(status) => OpenCodeWait::Exited(status),
                InactivityOutcome::Cancelled => OpenCodeWait::Cancelled,
                InactivityOutcome::Inactive => OpenCodeWait::Inactive,
            },
        };
        match outcome {
            OpenCodeWait::Exited(Ok(status)) if status.success() => {
                let read = activity_task.await.map_err(|error| {
                    AgentError::Execution(format!("OpenCode activity reader stopped: {error}"))
                })??;
                stderr_task.await.map_err(|error| {
                    AgentError::Execution(format!("OpenCode diagnostics reader stopped: {error}"))
                })??;
                if read.reported_error {
                    return Err(AgentError::Execution(
                        "OpenCode reported a failed session".to_owned(),
                    ));
                }
                Ok(OpenCodeRun {
                    history: read.history,
                    response: read.response,
                })
            }
            OpenCodeWait::Exited(Ok(status)) => {
                stop_reader_tasks(activity_task, stderr_task).await;
                Err(AgentError::Execution(format!(
                    "OpenCode exited unsuccessfully with {status}"
                )))
            }
            OpenCodeWait::Exited(Err(error)) => {
                let _ = child.kill().await;
                stop_reader_tasks(activity_task, stderr_task).await;
                Err(AgentError::Execution(format!(
                    "could not wait for OpenCode: {error}"
                )))
            }
            OpenCodeWait::Cancelled => {
                let _ = child.kill().await;
                stop_reader_tasks(activity_task, stderr_task).await;
                Err(AgentError::Cancelled)
            }
            OpenCodeWait::TimedOut => {
                let _ = child.kill().await;
                stop_reader_tasks(activity_task, stderr_task).await;
                Err(AgentError::TimedOut)
            }
            OpenCodeWait::Inactive => {
                let _ = child.kill().await;
                stop_reader_tasks(activity_task, stderr_task).await;
                Err(AgentError::Inactive)
            }
        }
    }
}

#[async_trait]
impl AgentClient for OpenCodeCliClient {
    /// Runs one structured OpenCode analysis through the user-selected CLI profile.
    async fn analyze(
        &self,
        request: AnalysisRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<AgentExecution, AgentError> {
        self.execute_structured_prompt(
            ANALYSIS_SCHEMA,
            &analysis_prompt(&request.project_name, &request.brief),
            activity,
            cancellation,
        )
        .await
    }

    /// Returns the configured hard deadline for one OpenCode operation.
    fn timeout(&self) -> Duration {
        self.config.timeout
    }
}

#[async_trait]
impl ResearchClient for OpenCodeCliClient {
    /// Runs cited read-only research and validates the final structured provider response.
    async fn research(
        &self,
        request: ResearchRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        let execution = self
            .execute_structured_prompt(
                RESEARCH_ANSWER_SCHEMA,
                &research_prompt(&request),
                activity,
                cancellation,
            )
            .await?;
        ResearchedAnswer::from_json(&execution.response)
    }
}

#[async_trait]
impl AutoAnswerClient for OpenCodeCliClient {
    /// Selects a bounded independent research batch using the existing provider-neutral contract.
    async fn plan(
        &self,
        request: ResearchPlanRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchBatchPlan, AgentError> {
        let execution = self
            .execute_structured_prompt(
                RESEARCH_PLAN_SCHEMA,
                &research_plan_prompt(&request),
                activity,
                cancellation,
            )
            .await?;
        let eligible = request
            .questions()
            .iter()
            .map(|question| question.question_id().to_owned())
            .collect::<Vec<_>>();
        ResearchBatchPlan::from_json(
            &execution.response,
            &eligible,
            request.blocking_question_id(),
        )
    }

    /// Reuses the cited research path for one automatically selected question.
    async fn research(
        &self,
        request: ResearchRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        ResearchClient::research(self, request, activity, cancellation).await
    }

    /// Judges the full candidate batch using the existing checked response contract.
    async fn judge(
        &self,
        request: ResearchJudgmentRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<JudgedResearchBatch, AgentError> {
        let execution = self
            .execute_structured_prompt(
                RESEARCH_JUDGMENT_SCHEMA,
                &research_judgment_prompt(&request),
                activity,
                cancellation,
            )
            .await?;
        let candidate_ids = request
            .candidates()
            .iter()
            .map(|candidate| candidate.question_id().to_owned())
            .collect::<Vec<_>>();
        JudgedResearchBatch::from_json(&execution.response, &candidate_ids)
    }
}

#[async_trait]
impl DocumentationClient for OpenCodeCliClient {
    /// Executes staged generation or repair without widening the current working directory.
    async fn execute(
        &self,
        request: DocumentationRequest,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        let prompt = match request.kind {
            DocumentationKind::Generate => {
                documentation_prompt(&request.snapshot_json, &request.required_paths)
            }
            DocumentationKind::Repair => {
                let findings = request.repair_findings_json.as_deref().ok_or_else(|| {
                    AgentError::InvalidRequest(
                        "documentation repair requires validation findings".to_owned(),
                    )
                })?;
                repair_prompt(&request.snapshot_json, &request.required_paths, findings)
            }
        };
        self.execute_documentation_prompt(&prompt, &request.staging, activity, cancellation)
            .await
    }
}

/// Carries the bounded output recovered from one successful OpenCode child process.
struct OpenCodeRun {
    history: ActivityHistory,
    response: Option<String>,
}

/// Carries line-parser state after OpenCode closes its JSON event stream.
struct OpenCodeActivityRead {
    history: ActivityHistory,
    response: Option<String>,
    reported_error: bool,
}

/// Selects the deadline semantics that match the existing Project Init operation type.
#[derive(Debug, Clone, Copy)]
enum TimeoutPolicy {
    /// Enforces an absolute execution limit for structured analysis and research operations.
    Hard,
    /// Extends documentation execution while OpenCode continues to emit bounded activity.
    Inactivity,
}

/// Carries the complete child-process outcome used to map deadline failures precisely.
enum OpenCodeWait {
    /// Carries the operating-system exit status of the child process.
    Exited(std::io::Result<std::process::ExitStatus>),
    /// Reports explicit terminal cancellation.
    Cancelled,
    /// Reports exhaustion of an absolute operation deadline.
    TimedOut,
    /// Reports no progress during an inactivity-bounded documentation operation.
    Inactive,
}

/// Reads OpenCode JSON events with a strict line bound and records only supported final text.
async fn read_opencode_activity(
    mut reader: impl AsyncRead + Unpin,
    sender: mpsc::Sender<ActivityEvent>,
    capacity: usize,
    initial: ActivityEvent,
    progress: mpsc::Sender<()>,
) -> Result<OpenCodeActivityRead, AgentError> {
    let mut history = ActivityHistory::new(capacity)?;
    history.push(initial);
    let mut sequence = 2_u32;
    let mut pending = Vec::new();
    let mut chunk = [0_u8; 4_096];
    let mut response = None;
    let mut reported_error = false;
    loop {
        let count = reader.read(&mut chunk).await.map_err(|error| {
            AgentError::Execution(format!("could not read OpenCode events: {error}"))
        })?;
        if count == 0 {
            break;
        }
        let _ = progress.try_send(());
        for byte in &chunk[..count] {
            if *byte == b'\n' {
                if !pending.is_empty() {
                    process_opencode_event_line(
                        &pending,
                        &mut history,
                        &sender,
                        &mut sequence,
                        &mut response,
                        &mut reported_error,
                    )?;
                }
                pending.clear();
            } else {
                if pending.len() == MAX_JSONL_LINE_BYTES {
                    return Err(AgentError::InvalidEvent(
                        "OpenCode event exceeded the supported line size".to_owned(),
                    ));
                }
                pending.push(*byte);
            }
        }
    }
    if !pending.is_empty() {
        process_opencode_event_line(
            &pending,
            &mut history,
            &sender,
            &mut sequence,
            &mut response,
            &mut reported_error,
        )?;
    }
    Ok(OpenCodeActivityRead {
        history,
        response,
        reported_error,
    })
}

/// Parses one JSON event, retaining only completed assistant text and safe progress metadata.
fn process_opencode_event_line(
    line: &[u8],
    history: &mut ActivityHistory,
    sender: &mpsc::Sender<ActivityEvent>,
    sequence: &mut u32,
    response: &mut Option<String>,
    reported_error: &mut bool,
) -> Result<(), AgentError> {
    let text = std::str::from_utf8(line).map_err(|error| {
        AgentError::InvalidEvent(format!("OpenCode event was not UTF-8: {error}"))
    })?;
    let event: Value = serde_json::from_str(text).map_err(|error| {
        AgentError::InvalidEvent(format!("OpenCode event was invalid JSON: {error}"))
    })?;
    if event.get("type").and_then(Value::as_str) == Some("error") {
        *reported_error = true;
    }
    if let Some(text) = completed_opencode_text(&event)? {
        *response = Some(text);
    }
    if let Some(event) = decode_opencode_jsonl_event(text, *sequence)? {
        let _ = sender.try_send(event.clone());
        history.push(event);
        *sequence += 1;
    }
    Ok(())
}

/// Extracts one completed text part and rejects responses that exceed the persisted-output bound.
fn completed_opencode_text(event: &Value) -> Result<Option<String>, AgentError> {
    if event.get("type").and_then(Value::as_str) != Some("text")
        || event.pointer("/part/type").and_then(Value::as_str) != Some("text")
        || event.pointer("/part/time/end").is_none_or(Value::is_null)
    {
        return Ok(None);
    }
    let text = event
        .pointer("/part/text")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            AgentError::InvalidEvent("OpenCode completed text event omitted text".to_owned())
        })?;
    if text.len() > MAX_FINAL_RESPONSE_BYTES {
        return Err(AgentError::InvalidResponse(
            "OpenCode final response exceeded the supported size".to_owned(),
        ));
    }
    Ok(Some(text.to_owned()))
}

/// Drains diagnostics so an OpenCode child cannot block on a full stderr pipe.
async fn drain_reader(
    mut reader: impl AsyncRead + Unpin,
    progress: mpsc::Sender<()>,
) -> Result<(), AgentError> {
    let mut buffer = [0_u8; 4_096];
    loop {
        let count = reader.read(&mut buffer).await.map_err(|error| {
            AgentError::Execution(format!("could not read OpenCode diagnostics: {error}"))
        })?;
        if count == 0 {
            return Ok(());
        }
        let _ = progress.try_send(());
    }
}

/// Stops pipe readers after cancellation or child-process failure.
async fn stop_reader_tasks(
    activity_task: tokio::task::JoinHandle<Result<OpenCodeActivityRead, AgentError>>,
    stderr_task: tokio::task::JoinHandle<Result<(), AgentError>>,
) {
    activity_task.abort();
    stderr_task.abort();
    let _ = activity_task.await;
    let _ = stderr_task.await;
}

/// Classifies completion, cancellation, and inactivity for one child-process wait.
enum InactivityOutcome<T> {
    /// Carries the completed child-process result.
    Completed(T),
    /// Reports a user cancellation while the child process remained active.
    Cancelled,
    /// Reports that no provider output arrived before the configured deadline.
    Inactive,
}

/// Waits for completion under an absolute deadline without allowing provider activity to extend it.
async fn wait_with_deadline<F>(
    completion: F,
    cancellation: CancellationToken,
    timeout: Duration,
) -> OpenCodeWait
where
    F: Future<Output = std::io::Result<std::process::ExitStatus>>,
{
    tokio::select! {
        status = completion => OpenCodeWait::Exited(status),
        () = cancellation.cancelled() => OpenCodeWait::Cancelled,
        () = tokio::time::sleep(timeout) => OpenCodeWait::TimedOut,
    }
}

/// Waits for completion while resetting the inactivity deadline after each provider pulse.
async fn wait_with_inactivity<F>(
    completion: F,
    mut activity: mpsc::Receiver<()>,
    cancellation: CancellationToken,
    inactivity_timeout: Duration,
) -> InactivityOutcome<F::Output>
where
    F: Future,
{
    tokio::pin!(completion);
    let deadline = tokio::time::sleep(inactivity_timeout);
    tokio::pin!(deadline);
    let mut activity_open = true;
    loop {
        tokio::select! {
            result = &mut completion => return InactivityOutcome::Completed(result),
            () = cancellation.cancelled() => return InactivityOutcome::Cancelled,
            pulse = activity.recv(), if activity_open => {
                if pulse.is_some() {
                    deadline.as_mut().reset(tokio::time::Instant::now() + inactivity_timeout);
                } else {
                    activity_open = false;
                }
            }
            () = &mut deadline => return InactivityOutcome::Inactive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OpenCodeCliClient, completed_opencode_text};

    /// Requires the non-interactive invocation to preserve the user's permission profile.
    #[test]
    fn command_arguments_do_not_auto_approve_permissions() {
        let arguments = OpenCodeCliClient::command_arguments("return JSON")
            .into_iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        assert_eq!(arguments, ["run", "--format", "json", "return JSON"]);
        assert!(!arguments.iter().any(|argument| argument == "--auto"));
    }

    /// Requires only a completed text event to become a persisted structured response.
    #[test]
    fn completed_text_requires_a_finalized_text_part() {
        let event = serde_json::json!({
            "type": "text",
            "part": {"type": "text", "text": "{\"findings\":[]}", "time": {"end": 1}},
        });

        assert_eq!(
            completed_opencode_text(&event).expect("the completed text event should parse"),
            Some("{\"findings\":[]}".to_owned())
        );
    }

    /// Rejects a text event that has not reached OpenCode's completed-part state.
    #[test]
    fn incomplete_text_is_not_treated_as_a_final_response() {
        let event = serde_json::json!({
            "type": "text",
            "part": {"type": "text", "text": "partial", "time": {}},
        });

        assert_eq!(
            completed_opencode_text(&event).expect("an incomplete text event should be ignored"),
            None
        );
    }
}
