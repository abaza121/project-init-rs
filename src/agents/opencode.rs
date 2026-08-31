use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use futures_util::StreamExt;
use reqwest::{Client, RequestBuilder, Response, redirect::Policy};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::time::Instant;
use url::Url;

use super::prompts::{
    ANALYSIS_SCHEMA, RESEARCH_ANSWER_SCHEMA, RESEARCH_JUDGMENT_SCHEMA, analysis_prompt,
    documentation_prompt, repair_prompt, research_judgment_prompt, research_plan_prompt,
    research_plan_schema, research_prompt,
};
use super::{
    ActivityEvent, ActivityHistory, ActivityKind, AgentClient, AgentError, AgentExecution,
    AnalysisRequest, AutoAnswerClient, CancellationToken, DocumentationClient, DocumentationKind,
    DocumentationRequest, JudgedResearchBatch, ResearchBatchPlan, ResearchClient,
    ResearchJudgmentRequest, ResearchPlanRequest, ResearchRequest, ResearchedAnswer, TimeoutPolicy,
};

const MAX_EVENT_BYTES: usize = 64 * 1024;
const MAX_FINAL_RESPONSE_BYTES: usize = 1024 * 1024;
const MAX_HTTP_BODY_BYTES: usize = 4 * 1024 * 1024;
const MAX_SERVER_LOG_LINE_BYTES: usize = 64 * 1024;
const SERVER_HOSTNAME: &str = "127.0.0.1";
const STRUCTURED_RETRY_COUNT: u64 = 2;

/// Resolves the native OpenCode CLI program used by the managed server process.
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

/// Rejects Windows shell shims because the server adapter requires a native executable.
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

/// Searches PATH for the native OpenCode executable used on Windows.
#[cfg(windows)]
fn resolve_opencode_from_path(path: &OsStr) -> Option<PathBuf> {
    env::split_paths(path)
        .map(|directory| directory.join("opencode.exe"))
        .find(|candidate| candidate.is_file())
}

/// Searches PATH for the native OpenCode executable used on Unix hosts.
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

/// Converts one documented OpenCode CLI JSON event into stable, non-sensitive activity.
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

/// Maps a provider tool event without exposing provider-controlled arguments or output.
fn opencode_tool_message(event: &Value) -> Option<(ActivityKind, String)> {
    event
        .pointer("/part/tool")
        .or_else(|| event.pointer("/properties/part/tool"))
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

/// Holds a hard analysis limit and an inactivity limit for research and documentation.
#[derive(Debug, Clone)]
pub struct OpenCodeCliConfig {
    pub executable: PathBuf,
    pub working_directory: PathBuf,
    pub timeout: Duration,
    pub history_capacity: usize,
}

/// Executes Project Init prompts through one shared managed OpenCode server.
#[derive(Clone)]
pub struct OpenCodeCliClient {
    state: Arc<OpenCodeState>,
}

impl fmt::Debug for OpenCodeCliClient {
    /// Formats only non-sensitive immutable configuration, excluding child and auth state.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenCodeCliClient")
            .field("config", &self.state.config)
            .finish()
    }
}

impl OpenCodeCliClient {
    /// Creates a client and validates the local HTTP transport before any provider call.
    pub fn new(config: OpenCodeCliConfig) -> Self {
        let http = Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("the static OpenCode HTTP client configuration must be valid");
        let auth = env::var("OPENCODE_SERVER_PASSWORD").ok().map(|password| {
            (
                env::var("OPENCODE_SERVER_USERNAME").unwrap_or_else(|_| "opencode".to_owned()),
                password,
            )
        });
        Self {
            state: Arc::new(OpenCodeState {
                config,
                http,
                auth,
                server: Mutex::new(None),
            }),
        }
    }

    /// Returns the immutable process settings used by later OpenCode server operations.
    pub fn config(&self) -> &OpenCodeCliConfig {
        &self.state.config
    }

    /// Returns the managed server command and its loopback-only ephemeral port settings.
    fn command_arguments() -> Vec<OsString> {
        server_command_arguments()
    }

    /// Executes one structured request using OpenCode's native JSON Schema response format.
    async fn execute_structured_prompt(
        &self,
        schema: &str,
        prompt: &str,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
        timeout_policy: TimeoutPolicy,
    ) -> Result<AgentExecution, AgentError> {
        let started_at = Utc::now();
        let (response, history) = self
            .execute_operation(
                OperationSpec {
                    prompt,
                    schema: Some(schema),
                    working_directory: &self.state.config.working_directory,
                    initial_message: "Starting OpenCode server-backed session",
                    timeout_policy,
                },
                activity,
                cancellation,
            )
            .await?;
        let response = response.ok_or_else(|| {
            AgentError::InvalidResponse("OpenCode returned no structured output".to_owned())
        })?;
        Ok(AgentExecution {
            id: uuid::Uuid::new_v4().to_string(),
            provider: "opencode_cli".to_owned(),
            model: None,
            response,
            input_tokens: None,
            output_tokens: None,
            activity: history.events(),
            started_at,
            completed_at: Utc::now(),
        })
    }

    /// Executes generation or repair in the caller-owned staging directory.
    async fn execute_documentation_prompt(
        &self,
        prompt: &str,
        staging: &Path,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        std::fs::create_dir_all(staging)
            .map_err(|error| AgentError::Execution(format!("documentation staging: {error}")))?;
        let (sender, _receiver) = mpsc::channel(self.state.config.history_capacity);
        let activity = activity.unwrap_or(sender);
        self.execute_operation(
            OperationSpec {
                prompt,
                schema: None,
                working_directory: staging,
                initial_message: "OpenCode started documentation work",
                timeout_policy: TimeoutPolicy::Inactivity,
            },
            activity,
            cancellation,
        )
        .await?;
        Ok(())
    }

    /// Runs one session request while enforcing cancellation, timeout, and bounded activity.
    async fn execute_operation(
        &self,
        spec: OperationSpec<'_>,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<(Option<String>, ActivityHistory), AgentError> {
        // OpenCode resolves routing paths from its own cwd, which may already be the data directory.
        let working_directory = std::path::absolute(spec.working_directory).map_err(|error| {
            AgentError::Execution(format!(
                "could not resolve OpenCode working directory: {error}"
            ))
        })?;
        let mut history = ActivityHistory::new(self.state.config.history_capacity)?;
        let mut sequence = 1_u32;
        record_activity(
            &mut history,
            &activity,
            &mut sequence,
            ActivityKind::Lifecycle,
            spec.initial_message,
        );
        let base_url = self.ensure_server(&cancellation).await?;
        record_activity(
            &mut history,
            &activity,
            &mut sequence,
            ActivityKind::Lifecycle,
            "OpenCode server is ready",
        );
        let session_id = tokio::select! {
            result = self.create_session(&base_url, &working_directory) => result?,
            () = cancellation.cancelled() => return Err(AgentError::Cancelled),
            () = tokio::time::sleep(self.state.config.timeout) => return Err(AgentError::TimedOut),
        };
        record_activity(
            &mut history,
            &activity,
            &mut sequence,
            ActivityKind::Lifecycle,
            "OpenCode session created",
        );
        let request_body = match spec.schema {
            Some(schema) => build_structured_message(spec.prompt, schema)?,
            None => build_plain_message(spec.prompt),
        };
        let (event_sender, event_receiver) = mpsc::channel(64);
        let (stop_sender, stop_receiver) = oneshot::channel();
        let event_task = tokio::spawn(self.clone().stream_events(
            base_url.clone(),
            session_id.clone(),
            working_directory.clone(),
            event_sender,
            stop_receiver,
        ));
        let message = self.post_message(&base_url, &session_id, &working_directory, request_body);
        let result = self
            .monitor_message(
                message,
                event_receiver,
                MonitorContext {
                    activity: &activity,
                    history: &mut history,
                    sequence: &mut sequence,
                    cancellation: &cancellation,
                    timeout_policy: spec.timeout_policy,
                },
            )
            .await;
        let _ = stop_sender.send(());
        event_task.abort();
        let _ = event_task.await;
        if result.is_err() {
            self.abort_session(&base_url, &session_id, &working_directory)
                .await;
        }
        self.delete_session(&base_url, &session_id, &working_directory)
            .await;
        let response = result?;
        let response = if spec.schema.is_some() {
            Some(extract_structured_output(&response)?)
        } else {
            None
        };
        record_activity(
            &mut history,
            &activity,
            &mut sequence,
            ActivityKind::Lifecycle,
            "OpenCode session completed",
        );
        Ok((response, history))
    }

    /// Ensures that this client has one healthy shared OpenCode server process.
    async fn ensure_server(&self, cancellation: &CancellationToken) -> Result<Url, AgentError> {
        let mut server = self.state.server.lock().await;
        if let Some(existing) = server.as_mut()
            && existing.is_running()
        {
            return Ok(existing.base_url.clone());
        }
        server.take();
        let managed = self.start_server(cancellation).await?;
        let base_url = managed.base_url.clone();
        *server = Some(managed);
        Ok(base_url)
    }

    /// Starts OpenCode on loopback without enabling automatic permission approval.
    async fn start_server(
        &self,
        cancellation: &CancellationToken,
    ) -> Result<ManagedServer, AgentError> {
        let mut command = Command::new(&self.state.config.executable);
        command
            .args(Self::command_arguments())
            .current_dir(&self.state.config.working_directory)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        let mut child = command.spawn().map_err(|error| {
            AgentError::Execution(format!("could not start OpenCode server: {error}"))
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            AgentError::Execution("OpenCode server stdout was unavailable".to_owned())
        })?;
        let stderr = child.stderr.take().ok_or_else(|| {
            AgentError::Execution("OpenCode server stderr was unavailable".to_owned())
        })?;
        let (url_sender, mut url_receiver) = oneshot::channel();
        let stdout_task = tokio::spawn(read_server_stdout(stdout, url_sender));
        let stderr_task = tokio::spawn(drain_reader(stderr));
        let deadline = Instant::now() + self.state.config.timeout;
        let base_url = loop {
            if let Some(status) = child.try_wait().map_err(|error| {
                AgentError::Execution(format!("could not inspect OpenCode server: {error}"))
            })? {
                stop_process_tasks(&mut child, stdout_task, stderr_task).await;
                return Err(AgentError::Execution(format!(
                    "OpenCode server exited during startup with {status}"
                )));
            }
            if Instant::now() >= deadline {
                stop_process_tasks(&mut child, stdout_task, stderr_task).await;
                return Err(AgentError::TimedOut);
            }
            tokio::select! {
                result = &mut url_receiver => {
                    let url = result.map_err(|_| AgentError::Execution("OpenCode server did not announce a listening address; verify its installation and configuration".to_owned()))??;
                    break url;
                }
                () = cancellation.cancelled() => {
                    stop_process_tasks(&mut child, stdout_task, stderr_task).await;
                    return Err(AgentError::Cancelled);
                }
                () = tokio::time::sleep(Duration::from_millis(100)) => {}
            }
        };
        loop {
            if let Some(status) = child.try_wait().map_err(|error| {
                AgentError::Execution(format!("could not inspect OpenCode server: {error}"))
            })? {
                stop_process_tasks(&mut child, stdout_task, stderr_task).await;
                return Err(AgentError::Execution(format!(
                    "OpenCode server exited before becoming healthy with {status}"
                )));
            }
            if tokio::time::timeout(Duration::from_millis(250), self.check_health(&base_url))
                .await
                .unwrap_or(false)
            {
                return Ok(ManagedServer {
                    child,
                    base_url,
                    stdout_task,
                    stderr_task,
                });
            }
            if Instant::now() >= deadline {
                stop_process_tasks(&mut child, stdout_task, stderr_task).await;
                return Err(AgentError::TimedOut);
            }
            tokio::select! {
                () = cancellation.cancelled() => {
                    stop_process_tasks(&mut child, stdout_task, stderr_task).await;
                    return Err(AgentError::Cancelled);
                }
                () = tokio::time::sleep(Duration::from_millis(100)) => {}
            }
        }
    }

    /// Checks the server health endpoint without accepting a remote or redirected target.
    async fn check_health(&self, base_url: &Url) -> bool {
        let Ok(url) = base_url.join("global/health") else {
            return false;
        };
        let request = self.authorize(self.state.http.get(url));
        let Ok(response) = request.send().await else {
            return false;
        };
        if !response.status().is_success() {
            return false;
        }
        read_bounded_json(response, MAX_HTTP_BODY_BYTES)
            .await
            .ok()
            .and_then(|body| body.get("healthy").and_then(Value::as_bool))
            .unwrap_or(false)
    }

    /// Creates one isolated session in the server's requested project directory.
    async fn create_session(&self, base_url: &Url, directory: &Path) -> Result<String, AgentError> {
        let url = base_url.join("session").map_err(|error| {
            AgentError::Execution(format!("could not address OpenCode session: {error}"))
        })?;
        let response = self
            .request(self.state.http.post(url), Some(directory))
            .json(&serde_json::json!({"title": "Project Init"}))
            .send()
            .await
            .map_err(|error| {
                AgentError::Execution(format!("could not create OpenCode session: {error}"))
            })?;
        let body = read_bounded_json(response, MAX_HTTP_BODY_BYTES).await?;
        let session_id = body.get("id").and_then(Value::as_str).ok_or_else(|| {
            AgentError::InvalidResponse("OpenCode session response omitted its id".to_owned())
        })?;
        validate_session_id(session_id)?;
        Ok(session_id.to_owned())
    }

    /// Builds the synchronous message request for one active OpenCode session.
    fn post_message(
        &self,
        base_url: &Url,
        session_id: &str,
        directory: &Path,
        body: Value,
    ) -> impl Future<Output = Result<Value, AgentError>> + Send + 'static {
        let client = self.clone();
        let base_url = base_url.clone();
        let session_id = session_id.to_owned();
        let directory = directory.to_path_buf();
        async move {
            validate_session_id(&session_id)?;
            let url = base_url
                .join(&format!("session/{session_id}/message"))
                .map_err(|error| {
                    AgentError::Execution(format!("could not address OpenCode message: {error}"))
                })?;
            let response = client
                .request(client.state.http.post(url), Some(&directory))
                .json(&body)
                .send()
                .await
                .map_err(|error| {
                    AgentError::Execution(format!("could not send OpenCode message: {error}"))
                })?;
            read_bounded_json(response, MAX_HTTP_BODY_BYTES).await
        }
    }

    /// Monitors the message response and server event stream under provider-neutral deadlines.
    async fn monitor_message<F>(
        &self,
        message: F,
        mut events: mpsc::Receiver<ServerPulse>,
        context: MonitorContext<'_>,
    ) -> Result<Value, AgentError>
    where
        F: Future<Output = Result<Value, AgentError>>,
    {
        tokio::pin!(message);
        let deadline = tokio::time::sleep(self.state.config.timeout);
        tokio::pin!(deadline);
        let mut events_open = true;
        loop {
            match context.timeout_policy {
                TimeoutPolicy::Hard => {
                    tokio::select! {
                        result = &mut message => return result,
                        () = context.cancellation.cancelled() => return Err(AgentError::Cancelled),
                        pulse = events.recv(), if events_open => {
                            if let Some(pulse) = pulse {
                                record_pulse(
                                    pulse,
                                    context.activity,
                                    context.history,
                                    context.sequence,
                                );
                            } else {
                                events_open = false;
                            }
                        }
                        () = &mut deadline => return Err(AgentError::TimedOut),
                    }
                }
                TimeoutPolicy::Inactivity => {
                    tokio::select! {
                        result = &mut message => return result,
                        () = context.cancellation.cancelled() => return Err(AgentError::Cancelled),
                        pulse = events.recv(), if events_open => {
                            if let Some(pulse) = pulse {
                                record_pulse(
                                    pulse,
                                    context.activity,
                                    context.history,
                                    context.sequence,
                                );
                                deadline
                                    .as_mut()
                                    .reset(Instant::now() + self.state.config.timeout);
                            } else {
                                events_open = false;
                            }
                        }
                        () = &mut deadline => return Err(AgentError::Inactive),
                    }
                }
            }
        }
    }

    /// Streams safe session activity from OpenCode's global event endpoint.
    async fn stream_events(
        self,
        base_url: Url,
        session_id: String,
        directory: PathBuf,
        sender: mpsc::Sender<ServerPulse>,
        mut stop: oneshot::Receiver<()>,
    ) {
        let Ok(url) = base_url.join("event") else {
            return;
        };
        let request = self.request(self.state.http.get(url), Some(&directory));
        let response = tokio::select! {
            result = request.send() => result.ok(),
            _ = &mut stop => None,
        };
        let Some(response) = response else {
            return;
        };
        if !response.status().is_success() {
            return;
        }
        let mut stream = response.bytes_stream();
        let mut decoder = SseDecoder::default();
        while let Some(chunk) = tokio::select! {
            chunk = stream.next() => chunk,
            _ = &mut stop => None,
        } {
            let Ok(chunk) = chunk else {
                return;
            };
            let Ok(frames) = decoder.push(&chunk) else {
                return;
            };
            for frame in frames {
                let Ok(event) = serde_json::from_str::<Value>(&frame) else {
                    continue;
                };
                let pulse = match decode_opencode_server_event(&event, &session_id, 0) {
                    Ok(activity) => ServerPulse {
                        activity: activity.map(|event| (event.kind, event.message)),
                    },
                    Err(_) => continue,
                };
                if sender.send(pulse).await.is_err() {
                    return;
                }
            }
        }
    }

    /// Sends a best-effort abort so a timed-out session stops server-side work promptly.
    async fn abort_session(&self, base_url: &Url, session_id: &str, directory: &Path) {
        let Ok(url) = base_url.join(&format!("session/{session_id}/abort")) else {
            return;
        };
        let request = self.request(self.state.http.post(url), Some(directory));
        let _ = tokio::time::timeout(Duration::from_secs(1), request.send()).await;
    }

    /// Deletes a completed session so server memory does not grow across workflow operations.
    async fn delete_session(&self, base_url: &Url, session_id: &str, directory: &Path) {
        let Ok(url) = base_url.join(&format!("session/{session_id}")) else {
            return;
        };
        let request = self.request(self.state.http.delete(url), Some(directory));
        let _ = tokio::time::timeout(Duration::from_secs(1), request.send()).await;
    }

    /// Adds the directory routing header and inherited server authentication to one request.
    fn request(&self, request: RequestBuilder, directory: Option<&Path>) -> RequestBuilder {
        let request = if let Some(directory) = directory {
            request.header(
                "x-opencode-directory",
                directory.to_string_lossy().to_string(),
            )
        } else {
            request
        };
        self.authorize(request)
    }

    /// Applies inherited OpenCode server credentials without exposing them in diagnostics.
    fn authorize(&self, request: RequestBuilder) -> RequestBuilder {
        match &self.state.auth {
            Some((username, password)) => request.basic_auth(username, Some(password)),
            None => request,
        }
    }
}

#[async_trait]
impl AgentClient for OpenCodeCliClient {
    /// Runs one structured OpenCode analysis through the user's configured server profile.
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
            TimeoutPolicy::Hard,
        )
        .await
    }

    /// Returns the configured hard deadline for one initial analysis.
    fn timeout(&self) -> Duration {
        self.state.config.timeout
    }
}

#[async_trait]
impl ResearchClient for OpenCodeCliClient {
    /// Runs cited read-only research and validates the server's structured response.
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
                TimeoutPolicy::Inactivity,
            )
            .await?;
        ResearchedAnswer::from_json(&execution.response)
    }
}

#[async_trait]
impl AutoAnswerClient for OpenCodeCliClient {
    /// Selects a bounded independent research batch using the checked provider contract.
    async fn plan(
        &self,
        request: ResearchPlanRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchBatchPlan, AgentError> {
        let execution = self
            .execute_structured_prompt(
                &research_plan_schema(&request),
                &research_plan_prompt(&request),
                activity,
                cancellation,
                TimeoutPolicy::Inactivity,
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

    /// Judges the full candidate batch using the checked response contract.
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
                TimeoutPolicy::Inactivity,
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

/// Describes one provider operation before its prompt is sent to a server session.
struct OperationSpec<'a> {
    prompt: &'a str,
    schema: Option<&'a str>,
    working_directory: &'a Path,
    initial_message: &'a str,
    timeout_policy: TimeoutPolicy,
}

/// Borrows the mutable operation state needed while a message is monitored.
struct MonitorContext<'a> {
    activity: &'a mpsc::Sender<ActivityEvent>,
    history: &'a mut ActivityHistory,
    sequence: &'a mut u32,
    cancellation: &'a CancellationToken,
    timeout_policy: TimeoutPolicy,
}

/// Stores the shared HTTP client, inherited authentication, and optional server process.
struct OpenCodeState {
    config: OpenCodeCliConfig,
    http: Client,
    auth: Option<(String, String)>,
    server: Mutex<Option<ManagedServer>>,
}

/// Owns one managed OpenCode child and the tasks draining its output pipes.
struct ManagedServer {
    child: Child,
    base_url: Url,
    stdout_task: tokio::task::JoinHandle<Result<(), AgentError>>,
    stderr_task: tokio::task::JoinHandle<Result<(), AgentError>>,
}

impl ManagedServer {
    /// Reports whether the child still exists without waiting for it or blocking the workflow.
    fn is_running(&mut self) -> bool {
        self.child.try_wait().ok().flatten().is_none()
    }
}

impl Drop for ManagedServer {
    /// Requests child termination and stops pipe drains when the final client clone is dropped.
    fn drop(&mut self) {
        let _ = self.child.start_kill();
        self.stdout_task.abort();
        self.stderr_task.abort();
    }
}

/// Carries one safe activity mapping from the server event stream to the operation monitor.
struct ServerPulse {
    activity: Option<(ActivityKind, String)>,
}

/// Carries one structured event frame decoder's unfinished data between HTTP chunks.
#[derive(Default)]
struct SseDecoder {
    pending: Vec<u8>,
}

impl SseDecoder {
    /// Appends one bounded HTTP chunk and returns complete JSON data frames.
    fn push(&mut self, chunk: &[u8]) -> Result<Vec<String>, AgentError> {
        if chunk.len() > MAX_EVENT_BYTES {
            return Err(AgentError::InvalidEvent(
                "OpenCode SSE event exceeded the supported size".to_owned(),
            ));
        }
        self.pending.extend_from_slice(chunk);
        let mut frames = Vec::new();
        while let Some((boundary, separator_length)) = find_sse_boundary(&self.pending) {
            let frame = self.pending.drain(..boundary).collect::<Vec<_>>();
            self.pending.drain(..separator_length);
            if let Some(data) = decode_sse_frame(&frame)? {
                frames.push(data);
            }
        }
        if self.pending.len() > MAX_EVENT_BYTES {
            return Err(AgentError::InvalidEvent(
                "OpenCode SSE event exceeded the supported size".to_owned(),
            ));
        }
        Ok(frames)
    }
}

/// Finds the blank-line boundary separating two SSE frames.
fn find_sse_boundary(bytes: &[u8]) -> Option<(usize, usize)> {
    if let Some(position) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
        return Some((position, 4));
    }
    bytes
        .windows(2)
        .position(|window| window == b"\n\n")
        .map(|position| (position, 2))
}

/// Extracts concatenated `data:` lines from one SSE frame and ignores comments or event names.
fn decode_sse_frame(frame: &[u8]) -> Result<Option<String>, AgentError> {
    let text = std::str::from_utf8(frame).map_err(|error| {
        AgentError::InvalidEvent(format!("OpenCode SSE was not UTF-8: {error}"))
    })?;
    let mut data = Vec::new();
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("data:") {
            data.push(value.strip_prefix(' ').unwrap_or(value));
        }
    }
    if data.is_empty() {
        return Ok(None);
    }
    let joined = data.join("\n");
    if joined.len() > MAX_EVENT_BYTES {
        return Err(AgentError::InvalidEvent(
            "OpenCode SSE data exceeded the supported size".to_owned(),
        ));
    }
    Ok(Some(joined))
}

/// Maps one OpenCode server event for the active session to safe provider-neutral activity.
fn decode_opencode_server_event(
    event: &Value,
    session_id: &str,
    sequence: u32,
) -> Result<Option<ActivityEvent>, AgentError> {
    let event_session = event
        .pointer("/properties/sessionID")
        .or_else(|| event.pointer("/properties/part/sessionID"))
        .or_else(|| event.pointer("/sessionID"))
        .and_then(Value::as_str);
    if event_session != Some(session_id) {
        return Ok(None);
    }
    let event_type = event
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mapped = match event_type {
        "session.created" => Some((
            ActivityKind::Lifecycle,
            "OpenCode session is active".to_owned(),
        )),
        "session.status" | "message.updated" => Some((
            ActivityKind::Progress,
            "OpenCode reported session progress".to_owned(),
        )),
        "message.part.updated" => opencode_tool_message(event).or_else(|| {
            Some((
                ActivityKind::Progress,
                "OpenCode updated the active response".to_owned(),
            ))
        }),
        "permission.asked" => Some((
            ActivityKind::Warning,
            "OpenCode requested a permission decision".to_owned(),
        )),
        "session.error" | "error" => Some((
            ActivityKind::Warning,
            "OpenCode reported a session error".to_owned(),
        )),
        _ => None,
    };
    Ok(mapped.map(|(kind, message)| ActivityEvent::now(sequence, kind, &message)))
}

/// Builds a native OpenCode JSON Schema message without embedding a prompt-only schema directive.
fn build_structured_message(prompt: &str, schema: &str) -> Result<Value, AgentError> {
    let schema = serde_json::from_str::<Value>(schema).map_err(|error| {
        AgentError::InvalidRequest(format!(
            "OpenCode structured schema is invalid JSON: {error}"
        ))
    })?;
    if !schema.is_object() {
        return Err(AgentError::InvalidRequest(
            "OpenCode structured schema must be a JSON object".to_owned(),
        ));
    }
    Ok(serde_json::json!({
        "parts": [{"type": "text", "text": prompt}],
        "format": {"type": "json_schema", "schema": schema, "retryCount": STRUCTURED_RETRY_COUNT}
    }))
}

/// Builds a normal text message used for staged documentation generation and repair.
fn build_plain_message(prompt: &str) -> Value {
    serde_json::json!({"parts": [{"type": "text", "text": prompt}]})
}

/// Extracts OpenCode's structured payload, accepting the SDK-documented alias, with a size bound.
fn extract_structured_output(response: &Value) -> Result<String, AgentError> {
    let info = response.pointer("/info").unwrap_or(&Value::Null);
    if let Some(name) = info.pointer("/error/name").and_then(Value::as_str) {
        return Err(AgentError::InvalidResponse(format!(
            "OpenCode structured output failed: {name}"
        )));
    }
    let output = info
        .pointer("/structured")
        .or_else(|| info.pointer("/structured_output"))
        .ok_or_else(|| {
            AgentError::InvalidResponse("OpenCode response omitted structured output".to_owned())
        })?;
    let text = serde_json::to_string(output).map_err(|error| {
        AgentError::InvalidResponse(format!("could not serialize structured output: {error}"))
    })?;
    if text.len() > MAX_FINAL_RESPONSE_BYTES {
        return Err(AgentError::InvalidResponse(
            "OpenCode structured output exceeded the supported size".to_owned(),
        ));
    }
    Ok(text)
}

/// Returns the managed server command arguments used by both production code and contract tests.
fn server_command_arguments() -> Vec<OsString> {
    vec![
        OsString::from("serve"),
        OsString::from("--hostname"),
        OsString::from(SERVER_HOSTNAME),
        OsString::from("--port"),
        OsString::from("0"),
    ]
}

/// Adds one bounded event to history and attempts non-blocking delivery to the TUI.
fn record_activity(
    history: &mut ActivityHistory,
    sender: &mpsc::Sender<ActivityEvent>,
    sequence: &mut u32,
    kind: ActivityKind,
    message: &str,
) {
    let event = ActivityEvent::now(*sequence, kind, message);
    *sequence = sequence.saturating_add(1);
    let _ = sender.try_send(event.clone());
    history.push(event);
}

/// Records a server pulse while preserving the operation's local sequence numbering.
fn record_pulse(
    pulse: ServerPulse,
    sender: &mpsc::Sender<ActivityEvent>,
    history: &mut ActivityHistory,
    sequence: &mut u32,
) {
    if let Some((kind, message)) = pulse.activity {
        record_activity(history, sender, sequence, kind, &message);
    }
}

/// Reads the server's listening URL from stdout while continuing to drain later log lines.
async fn read_server_stdout(
    reader: impl AsyncRead + Unpin,
    sender: oneshot::Sender<Result<Url, AgentError>>,
) -> Result<(), AgentError> {
    let mut reader = BufReader::new(reader);
    let mut line = Vec::new();
    let mut sender = Some(sender);
    loop {
        line.clear();
        let count = reader.read_until(b'\n', &mut line).await.map_err(|error| {
            AgentError::Execution(format!("could not read OpenCode server output: {error}"))
        })?;
        if count == 0 {
            if let Some(sender) = sender.take() {
                let _ = sender.send(Err(AgentError::Execution(
                    "OpenCode server did not announce a listening address; verify its installation and configuration".to_owned(),
                )));
            }
            return Ok(());
        }
        if line.len() > MAX_SERVER_LOG_LINE_BYTES {
            continue;
        }
        if let Some(url) = parse_listening_url(&String::from_utf8_lossy(&line))
            && let Some(sender) = sender.take()
        {
            let _ = sender.send(Ok(url));
        }
    }
}

/// Parses and validates the loopback URL emitted by `opencode serve`.
fn parse_listening_url(line: &str) -> Option<Url> {
    line.split_whitespace().find_map(|word| {
        let candidate = word.trim_matches(|character| matches!(character, ',' | '.' | ';'));
        let url = Url::parse(candidate).ok()?;
        if url.scheme() == "http" && url.host_str() == Some(SERVER_HOSTNAME) && url.port().is_some()
        {
            Some(url)
        } else {
            None
        }
    })
}

/// Drains a child pipe so server diagnostics cannot block the managed process.
async fn drain_reader(mut reader: impl AsyncRead + Unpin) -> Result<(), AgentError> {
    let mut buffer = [0_u8; 4_096];
    while reader.read(&mut buffer).await.map_err(|error| {
        AgentError::Execution(format!(
            "could not read OpenCode server diagnostics: {error}"
        ))
    })? != 0
    {}
    Ok(())
}

/// Terminates a failed server startup and joins its output-draining tasks.
async fn stop_process_tasks(
    child: &mut Child,
    stdout_task: tokio::task::JoinHandle<Result<(), AgentError>>,
    stderr_task: tokio::task::JoinHandle<Result<(), AgentError>>,
) {
    let _ = child.kill().await;
    stdout_task.abort();
    stderr_task.abort();
    let _ = stdout_task.await;
    let _ = stderr_task.await;
}

/// Reads a bounded JSON response body and maps status or decoding failures safely.
async fn read_bounded_json(response: Response, maximum: usize) -> Result<Value, AgentError> {
    let status = response.status();
    let mut body = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| {
            AgentError::Execution(format!("could not read OpenCode response: {error}"))
        })?;
        if body.len().saturating_add(chunk.len()) > maximum {
            return Err(AgentError::InvalidResponse(
                "OpenCode response exceeded the supported size".to_owned(),
            ));
        }
        body.extend_from_slice(&chunk);
    }
    if !status.is_success() {
        return Err(AgentError::Execution(format!(
            "OpenCode server returned HTTP {status}"
        )));
    }
    serde_json::from_slice(&body).map_err(|error| {
        AgentError::InvalidResponse(format!("OpenCode returned invalid JSON: {error}"))
    })
}

/// Rejects a session identifier before it is inserted into a URL path.
fn validate_session_id(session_id: &str) -> Result<(), AgentError> {
    if session_id.is_empty()
        || session_id.len() > 256
        || !session_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(AgentError::InvalidResponse(
            "OpenCode returned an invalid session identifier".to_owned(),
        ));
    }
    Ok(())
}

/// Extracts one completed text part for compatibility with legacy OpenCode event tests.
#[cfg(test)]
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

#[cfg(test)]
mod tests {
    use super::{
        SseDecoder, build_structured_message, completed_opencode_text,
        decode_opencode_server_event, extract_structured_output, server_command_arguments,
    };

    /// Requires the managed server invocation to preserve the user's permission profile.
    #[test]
    fn command_arguments_do_not_auto_approve_permissions() {
        let arguments = server_command_arguments()
            .into_iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            arguments,
            ["serve", "--hostname", "127.0.0.1", "--port", "0"]
        );
        assert!(!arguments.iter().any(|argument| argument == "--auto"));
    }

    /// Places a structured-output schema in OpenCode's native message format field.
    #[test]
    fn structured_message_uses_native_json_schema_format() {
        let message = build_structured_message(
            "Return the analysis.",
            r#"{"type":"object","properties":{"answer":{"type":"string"}},"required":["answer"]}"#,
        )
        .expect("the schema should be accepted");

        assert_eq!(message["parts"][0]["type"], "text");
        assert_eq!(message["parts"][0]["text"], "Return the analysis.");
        assert_eq!(message["format"]["type"], "json_schema");
        assert_eq!(message["format"]["schema"]["type"], "object");
        assert_eq!(message["format"]["schema"]["required"][0], "answer");
        assert!(!message.to_string().contains("<response_schema>"));
    }

    /// Reads the structured field returned by OpenCode 1.2.20 and 1.18.25.
    #[test]
    fn structured_response_reads_info_structured() {
        let response = serde_json::json!({
            "info": {"structured": {"answer": "validated"}},
            "parts": []
        });

        assert_eq!(
            extract_structured_output(&response).expect("structured output should be present"),
            r#"{"answer":"validated"}"#
        );
    }

    /// Retains compatibility with the field name documented by the OpenCode SDK.
    #[test]
    fn structured_response_reads_info_structured_output() {
        let response = serde_json::json!({
            "info": {"structured_output": {"answer": "validated"}},
            "parts": []
        });

        assert_eq!(
            extract_structured_output(&response).expect("structured output should be present"),
            r#"{"answer":"validated"}"#
        );
    }

    /// Prefers the server's primary field when both response spellings are present.
    #[test]
    fn structured_response_prefers_primary_field() {
        let response = serde_json::json!({
            "info": {
                "structured": {"answer": "primary"},
                "structured_output": {"answer": "alias"}
            },
            "parts": []
        });

        assert_eq!(
            extract_structured_output(&response).expect("the primary output should be used"),
            r#"{"answer":"primary"}"#
        );
    }

    /// Leaves a present null primary value for domain validation instead of using the alias.
    #[test]
    fn structured_response_does_not_hide_null_primary_with_alias() {
        let response = serde_json::json!({
            "info": {"structured": null, "structured_output": {"answer": "alias"}},
            "parts": []
        });

        assert_eq!(
            extract_structured_output(&response).expect("payload validation belongs to the caller"),
            "null"
        );
    }

    /// Applies the same serialized byte limit to both supported field names.
    #[test]
    fn structured_response_enforces_size_boundary_for_both_fields() {
        for field in ["structured", "structured_output"] {
            for excess in [0, 1] {
                // JSON quotes count toward the byte limit, including for the alias.
                let payload = "a".repeat(super::MAX_FINAL_RESPONSE_BYTES - 2 + excess);
                let response = serde_json::json!({"info": {field: payload}, "parts": []});
                let result = extract_structured_output(&response);
                if excess == 0 {
                    assert_eq!(result.unwrap().len(), super::MAX_FINAL_RESPONSE_BYTES);
                } else {
                    assert!(matches!(
                        result,
                        Err(super::AgentError::InvalidResponse(message))
                            if message.contains("exceeded the supported size")
                    ));
                }
            }
        }
    }

    /// Rejects JSON embedded in a text part rather than bypassing server structured output.
    #[test]
    fn structured_response_rejects_text_only_json() {
        let response = serde_json::json!({
            "info": {},
            "parts": [{"type": "text", "text": r#"{"answer":"unvalidated"}"#}]
        });

        let error = extract_structured_output(&response).expect_err("text must not be a fallback");
        assert!(error.to_string().contains("omitted structured output"));
    }

    /// Rejects a server response that completed without a structured-output payload.
    #[test]
    fn structured_response_rejects_missing_output() {
        let response = serde_json::json!({"info": {}, "parts": []});

        let error = extract_structured_output(&response).expect_err("missing output must fail");
        assert!(error.to_string().contains("structured output"));
    }

    /// Surfaces OpenCode's native structured-output failure without accepting a fallback text.
    #[test]
    fn structured_response_surfaces_server_validation_failure() {
        let response = serde_json::json!({
            "info": {"error": {"name": "StructuredOutputError"}},
            "parts": []
        });

        let error = extract_structured_output(&response).expect_err("server validation must fail");
        assert!(error.to_string().contains("StructuredOutputError"));
    }

    /// Preserves server errors even when both structured payload spellings are also present.
    #[test]
    fn structured_response_prioritizes_server_error_over_payloads() {
        let response = serde_json::json!({
            "info": {
                "error": {"name": "StructuredOutputError"},
                "structured": {"answer": "primary"},
                "structured_output": {"answer": "alias"}
            },
            "parts": []
        });

        let error = extract_structured_output(&response).expect_err("server failure must win");
        assert!(error.to_string().contains("StructuredOutputError"));
    }

    /// Handles split CRLF frames and multiple events in one HTTP chunk without merging them.
    #[test]
    fn sse_decoder_handles_split_crlf_frames() {
        let mut decoder = SseDecoder::default();
        assert!(
            decoder
                .push(b"data: {\"type\":\"one\"}\r\n")
                .expect("a partial frame should be retained")
                .is_empty()
        );

        let frames = decoder
            .push(b"\r\ndata: {\"type\":\"two\"}\r\n\r\n")
            .expect("complete frames should decode");
        assert_eq!(frames, [r#"{"type":"one"}"#, r#"{"type":"two"}"#]);
    }

    /// Filters server events by session and keeps tool arguments out of user-visible activity.
    #[test]
    fn server_event_maps_safe_progress_without_tool_arguments() {
        let event = serde_json::json!({
            "type": "message.part.updated",
            "properties": {
                "part": {
                    "sessionID": "session-1",
                    "type": "tool",
                    "tool": "read",
                    "state": {"input": {"secret": "do not display"}}
                }
            }
        });

        let activity = decode_opencode_server_event(&event, "session-1", 3)
            .expect("the event should decode")
            .expect("the tool event should produce safe activity");
        assert!(activity.message.contains("read"));
        assert!(!activity.message.contains("secret"));
        assert!(
            decode_opencode_server_event(&event, "session-2", 3)
                .expect("a different session should be ignored")
                .is_none()
        );
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
