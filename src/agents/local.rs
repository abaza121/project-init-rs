//! Implements the documented mistral.rs OpenAI-compatible HTTP surface.
//! See <https://docs.mistralrs.dev/reference/openai-compatibility/>.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::net::Ipv4Addr;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::time::Instant;
use url::{Host, Url};

use super::prompts::{
    ANALYSIS_SCHEMA, RESEARCH_ANSWER_SCHEMA, RESEARCH_JUDGMENT_SCHEMA, RESEARCH_PLAN_SCHEMA,
    analysis_prompt, documentation_prompt, repair_prompt, research_judgment_prompt,
    research_plan_prompt, research_prompt,
};
use super::runtime::LocalRuntimeConfig;
use super::{
    ActivityEvent, ActivityHistory, ActivityKind, AgentClient, AgentError, AgentExecution,
    AnalysisRequest, AutoAnswerClient, CancellationToken, DocumentationClient, DocumentationKind,
    DocumentationRequest, JudgedResearchBatch, ResearchBatchPlan, ResearchClient,
    ResearchJudgmentRequest, ResearchPlanRequest, ResearchRequest, ResearchedAnswer, TimeoutPolicy,
};

const MAX_DOCUMENT_BYTES: usize = 1024 * 1024;
const MAX_DOCUMENT_PACKAGE_BYTES: u64 = 16 * 1024 * 1024;

/// Holds bounded settings for one loopback OpenAI-compatible inference endpoint.
#[derive(Debug, Clone)]
pub struct LocalHttpConfig {
    endpoint: Url,
    model: String,
    inactivity_timeout: Duration,
    hard_timeout: Duration,
    history_capacity: usize,
}

/// Executes bounded provider capabilities through a loopback OpenAI-compatible HTTP service.
#[derive(Debug, Clone)]
pub struct LocalHttpProvider {
    config: LocalHttpConfig,
    client: reqwest::Client,
    runtime: Option<LocalRuntimeConfig>,
}

/// Groups a structured local operation's schema, tool permission, and timeout policy.
struct StructuredTask<'a> {
    schema_name: &'a str,
    schema: &'a str,
    prompt: &'a str,
    web_search: bool,
    timeout_policy: TimeoutPolicy,
}

impl LocalHttpProvider {
    /// Creates a proxy-free HTTP client so environment proxy settings cannot reroute loopback work.
    pub fn new(config: LocalHttpConfig) -> Result<Self, AgentError> {
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(config.inactivity_timeout())
            .build()
            .map_err(|error| {
                AgentError::Execution(format!("could not build local HTTP client: {error}"))
            })?;
        Ok(Self {
            config,
            client,
            runtime: None,
        })
    }

    /// Attaches a managed Docker runtime whose port must match the configured endpoint.
    pub fn with_runtime(mut self, runtime: LocalRuntimeConfig) -> Result<Self, AgentError> {
        let managed_host = matches!(
            self.config.endpoint().host(),
            Some(Host::Ipv4(address)) if address == Ipv4Addr::LOCALHOST
        );
        if !managed_host {
            return Err(AgentError::InvalidRequest(
                "managed local runtime endpoint must use 127.0.0.1".to_owned(),
            ));
        }
        if self.config.endpoint().port_or_known_default() != Some(runtime.port()) {
            return Err(AgentError::InvalidRequest(
                "local runtime port must match the local HTTP endpoint".to_owned(),
            ));
        }
        self.runtime = Some(runtime);
        Ok(self)
    }

    /// Executes a structured response with bounded startup and operation-specific stream deadlines.
    async fn execute_structured_prompt(
        &self,
        task: StructuredTask<'_>,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<AgentExecution, AgentError> {
        let StructuredTask {
            schema_name,
            schema,
            prompt,
            web_search,
            timeout_policy,
        } = task;
        let schema: Value = serde_json::from_str(schema).map_err(|error| {
            AgentError::Execution(format!("local response schema is invalid: {error}"))
        })?;
        let mut request = serde_json::json!({
            "model": self.config.model(),
            "messages": [{"role": "user", "content": prompt}],
            "response_format": {
                "type": "json_schema",
                "json_schema": {"name": schema_name, "strict": true, "schema": schema}
            },
            "stream": true,
            "stream_options": {"include_usage": true},
            "max_tokens": 8192
        });
        if web_search {
            request["web_search_options"] = serde_json::json!({});
            request["max_tool_rounds"] = serde_json::json!(8);
        }
        let started_at = Utc::now();
        let deadline = Instant::now() + self.config.hard_timeout();
        let operation = schema_name.replace('_', " ");
        let initial = ActivityEvent::now(
            1,
            ActivityKind::Lifecycle,
            &format!("Starting local inference {operation}"),
        );
        let _ = activity.try_send(initial.clone());
        let mut history = ActivityHistory::new(self.config.history_capacity())?;
        history.push(initial);
        let mut sequence = 2_u32;
        self.prepare_runtime(
            &activity,
            &cancellation,
            deadline,
            &mut history,
            &mut sequence,
        )
        .await?;
        // Startup always stays bounded; productive research may outlive that startup deadline.
        let inference_deadline = match timeout_policy {
            TimeoutPolicy::Hard => Some(deadline),
            TimeoutPolicy::Inactivity => None,
        };
        let completion = self
            .execute_chat_request(
                &request,
                &activity,
                &cancellation,
                inference_deadline,
                &mut history,
                &mut sequence,
            )
            .await?;
        if !completion.tool_calls.is_empty() {
            return Err(AgentError::InvalidResponse(
                "structured analysis unexpectedly requested a tool".to_owned(),
            ));
        }
        let final_event = ActivityEvent::now(
            sequence,
            ActivityKind::Lifecycle,
            &format!("Validating local {operation}"),
        );
        let _ = activity.try_send(final_event.clone());
        history.push(final_event);
        Ok(AgentExecution {
            id: uuid::Uuid::new_v4().to_string(),
            provider: "local_http".to_owned(),
            model: Some(self.config.model().to_owned()),
            response: completion.content,
            input_tokens: completion.input_tokens,
            output_tokens: completion.output_tokens,
            activity: history.events(),
            started_at,
            completed_at: Utc::now(),
        })
    }

    /// Bounds silence on every stream and optionally enforces a caller-owned total deadline.
    async fn execute_chat_request(
        &self,
        request: &Value,
        activity: &mpsc::Sender<ActivityEvent>,
        cancellation: &CancellationToken,
        deadline: Option<Instant>,
        history: &mut ActivityHistory,
        sequence: &mut u32,
    ) -> Result<ChatCompletion, AgentError> {
        let url = self
            .config
            .endpoint()
            .join("chat/completions")
            .map_err(|error| AgentError::Execution(format!("invalid local API route: {error}")))?;
        let send = self.client.post(url).json(request).send();
        let response_inactivity = Instant::now() + self.config.inactivity_timeout();
        let response = tokio::select! {
            response = send => response.map_err(|error| map_http_error("request", &error))?,
            () = cancellation.cancelled() => return Err(AgentError::Cancelled),
            () = wait_for_deadline(deadline) => return Err(AgentError::TimedOut),
            () = tokio::time::sleep_until(response_inactivity) => return Err(AgentError::Inactive),
        };
        if !response.status().is_success() {
            return Err(AgentError::Execution(format!(
                "local inference returned HTTP {}",
                response.status()
            )));
        }
        let mut stream = response.bytes_stream();
        let mut decoder = SseDecoder::new(64 * 1024);
        let mut accumulator = CompletionAccumulator::new(1024 * 1024, 64);
        let mut completed = false;
        let mut inactivity = Instant::now() + self.config.inactivity_timeout();
        loop {
            let chunk = tokio::select! {
                chunk = stream.next() => chunk,
                () = cancellation.cancelled() => return Err(AgentError::Cancelled),
                () = wait_for_deadline(deadline) => return Err(AgentError::TimedOut),
                () = tokio::time::sleep_until(inactivity) => return Err(AgentError::Inactive),
            };
            let Some(chunk) = chunk else {
                break;
            };
            let chunk = chunk.map_err(|error| map_http_error("stream", &error))?;
            for data in decoder.push(&chunk)? {
                if data == "[DONE]" {
                    completed = true;
                    break;
                }
                accumulator.ingest(&data)?;
                inactivity = Instant::now() + self.config.inactivity_timeout();
                let event = ActivityEvent::now(
                    *sequence,
                    ActivityKind::Progress,
                    "Local model is responding",
                );
                *sequence = (*sequence).saturating_add(1);
                let _ = activity.try_send(event.clone());
                history.push(event);
            }
            if completed {
                break;
            }
        }
        if !completed {
            return Err(AgentError::InvalidResponse(
                "local inference stream ended without a completion marker".to_owned(),
            ));
        }
        accumulator.finish()
    }

    /// Reuses a healthy managed endpoint or launches Docker and polls until the model is ready.
    async fn prepare_runtime(
        &self,
        activity: &mpsc::Sender<ActivityEvent>,
        cancellation: &CancellationToken,
        deadline: Instant,
        history: &mut ActivityHistory,
        sequence: &mut u32,
    ) -> Result<(), AgentError> {
        let Some(runtime) = &self.runtime else {
            return Ok(());
        };
        if self.probe_ready(cancellation, deadline).await? {
            return Ok(());
        }
        let launching = ActivityEvent::now(
            *sequence,
            ActivityKind::Lifecycle,
            "Launching local inference runtime",
        );
        *sequence = (*sequence).saturating_add(1);
        let _ = activity.try_send(launching.clone());
        history.push(launching);
        let launch_inactivity = Instant::now() + self.config.inactivity_timeout();
        tokio::select! {
            result = runtime.launch(cancellation, deadline) => result?,
            () = cancellation.cancelled() => return Err(AgentError::Cancelled),
            () = tokio::time::sleep_until(deadline) => return Err(AgentError::TimedOut),
            () = tokio::time::sleep_until(launch_inactivity) => return Err(AgentError::Inactive),
        }
        loop {
            if self.probe_ready(cancellation, deadline).await? {
                let ready = ActivityEvent::now(
                    *sequence,
                    ActivityKind::Lifecycle,
                    "Local inference runtime is ready",
                );
                *sequence = (*sequence).saturating_add(1);
                let _ = activity.try_send(ready.clone());
                history.push(ready);
                return Ok(());
            }
            tokio::select! {
                () = tokio::time::sleep(Duration::from_millis(500)) => {},
                () = cancellation.cancelled() => return Err(AgentError::Cancelled),
                () = tokio::time::sleep_until(deadline) => return Err(AgentError::TimedOut),
            }
        }
    }

    /// Checks both the server health route and loaded-model listing under the caller deadline.
    async fn probe_ready(
        &self,
        cancellation: &CancellationToken,
        deadline: Instant,
    ) -> Result<bool, AgentError> {
        let mut health_url = self.config.endpoint().clone();
        health_url.set_path("/health");
        let models_url = self
            .config
            .endpoint()
            .join("models")
            .map_err(|error| AgentError::Execution(format!("invalid models route: {error}")))?;
        let probe = async {
            let health = self.client.get(health_url).send().await.ok()?;
            if !health.status().is_success() {
                return None;
            }
            let models = self.client.get(models_url).send().await.ok()?;
            if !models.status().is_success() {
                return None;
            }
            let models = read_bounded_json(models, 64 * 1024).await.ok()?;
            models
                .get("data")
                .and_then(Value::as_array)
                .is_some_and(|items| {
                    items.iter().any(|item| {
                        item.get("id").and_then(Value::as_str) == Some(self.config.model())
                    })
                })
                .then_some(())
        };
        tokio::select! {
            result = probe => Ok(result.is_some()),
            () = cancellation.cancelled() => Err(AgentError::Cancelled),
            () = tokio::time::sleep_until(deadline) => Err(AgentError::TimedOut),
        }
    }
}

#[async_trait]
impl AgentClient for LocalHttpProvider {
    /// Runs authoritative initial analysis through the configured local HTTP service.
    async fn analyze(
        &self,
        request: AnalysisRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<AgentExecution, AgentError> {
        self.execute_structured_prompt(
            StructuredTask {
                schema_name: "analysis",
                schema: ANALYSIS_SCHEMA,
                prompt: &analysis_prompt(&request.project_name, &request.brief),
                web_search: false,
                timeout_policy: TimeoutPolicy::Hard,
            },
            activity,
            cancellation,
        )
        .await
    }

    /// Returns the absolute hard limit applied to one local analysis operation.
    fn timeout(&self) -> Duration {
        self.config.hard_timeout()
    }
}

#[async_trait]
impl ResearchClient for LocalHttpProvider {
    /// Uses the server's DuckDuckGo search tool and validates the cited recommendation.
    async fn research(
        &self,
        request: ResearchRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        let prompt = format!(
            "Use the configured DuckDuckGo web search capability for current evidence.\n{}",
            research_prompt(&request)
        );
        let execution = self
            .execute_structured_prompt(
                StructuredTask {
                    schema_name: "research_answer",
                    schema: RESEARCH_ANSWER_SCHEMA,
                    prompt: &prompt,
                    web_search: true,
                    timeout_policy: TimeoutPolicy::Inactivity,
                },
                activity,
                cancellation,
            )
            .await?;
        ResearchedAnswer::from_json(&execution.response)
    }
}

#[async_trait]
impl AutoAnswerClient for LocalHttpProvider {
    /// Selects a bounded dependency-safe research batch without granting research tools.
    async fn plan(
        &self,
        request: ResearchPlanRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchBatchPlan, AgentError> {
        let execution = self
            .execute_structured_prompt(
                StructuredTask {
                    schema_name: "research_plan",
                    schema: RESEARCH_PLAN_SCHEMA,
                    prompt: &research_plan_prompt(&request),
                    web_search: false,
                    timeout_policy: TimeoutPolicy::Inactivity,
                },
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

    /// Reuses the cited local research boundary for one automatic-answer worker.
    async fn research(
        &self,
        request: ResearchRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        ResearchClient::research(self, request, activity, cancellation).await
    }

    /// Reviews all provisional candidates without adding new search evidence.
    async fn judge(
        &self,
        request: ResearchJudgmentRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<JudgedResearchBatch, AgentError> {
        let execution = self
            .execute_structured_prompt(
                StructuredTask {
                    schema_name: "research_judgment",
                    schema: RESEARCH_JUDGMENT_SCHEMA,
                    prompt: &research_judgment_prompt(&request),
                    web_search: false,
                    timeout_policy: TimeoutPolicy::Inactivity,
                },
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

impl LocalHttpProvider {
    /// Runs a bounded client-side document tool loop under one absolute operation deadline.
    async fn execute_documentation_request(
        &self,
        request: DocumentationRequest,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        fs::create_dir_all(&request.staging).map_err(|error| {
            AgentError::Execution(format!("could not create documentation staging: {error}"))
        })?;
        let workspace = DocumentWorkspace::new(&request.staging, &request.required_paths)?;
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
        let prompt = format!(
            "Use only the supplied document tools for filesystem work. Use the configured DuckDuckGo search capability for current external evidence.\n{prompt}"
        );
        let (sink_sender, _sink_receiver) = mpsc::channel(1);
        let activity = activity.unwrap_or(sink_sender);
        let initial = ActivityEvent::now(
            1,
            ActivityKind::Lifecycle,
            "Starting local documentation work",
        );
        let _ = activity.try_send(initial.clone());
        let mut history = ActivityHistory::new(self.config.history_capacity())?;
        history.push(initial);
        let mut sequence = 2_u32;
        let deadline = Instant::now() + self.config.hard_timeout();
        self.prepare_runtime(
            &activity,
            &cancellation,
            deadline,
            &mut history,
            &mut sequence,
        )
        .await?;
        let tools = document_tools(&request.required_paths);
        let mut messages = vec![serde_json::json!({"role": "user", "content": prompt})];
        let mut seen_call_ids = BTreeSet::new();
        for _round in 0..32 {
            let request_body = serde_json::json!({
                "model": self.config.model(),
                "messages": messages,
                "tools": tools,
                "tool_choice": "auto",
                "web_search_options": {},
                "max_tool_rounds": 8,
                "stream": true,
                "max_tokens": 8192
            });
            if serde_json::to_vec(&request_body)
                .map_err(|error| {
                    AgentError::Execution(format!("could not encode document request: {error}"))
                })?
                .len()
                > 24 * 1024 * 1024
            {
                return Err(AgentError::InvalidResponse(
                    "local documentation transcript exceeded the supported size".to_owned(),
                ));
            }
            let completion = self
                .execute_chat_request(
                    &request_body,
                    &activity,
                    &cancellation,
                    Some(deadline),
                    &mut history,
                    &mut sequence,
                )
                .await?;
            if completion.tool_calls.is_empty() {
                workspace.validate_complete()?;
                let event = ActivityEvent::now(
                    sequence,
                    ActivityKind::Lifecycle,
                    "Local documentation work completed",
                );
                let _ = activity.try_send(event);
                return Ok(());
            }
            let assistant_calls = completion
                .tool_calls
                .iter()
                .map(|call| {
                    serde_json::json!({
                        "id": call.id,
                        "type": "function",
                        "function": {"name": call.name, "arguments": call.arguments}
                    })
                })
                .collect::<Vec<_>>();
            messages.push(serde_json::json!({
                "role": "assistant",
                "content": completion.content,
                "tool_calls": assistant_calls
            }));
            for call in completion.tool_calls {
                if !seen_call_ids.insert(call.id.clone()) {
                    return Err(AgentError::InvalidResponse(
                        "local inference repeated a document tool call id".to_owned(),
                    ));
                }
                let result = workspace.execute_tool(&call.name, &call.arguments)?;
                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": call.id,
                    "content": result
                }));
            }
        }
        Err(AgentError::InvalidResponse(
            "local documentation exceeded the supported tool rounds".to_owned(),
        ))
    }
}

#[async_trait]
impl DocumentationClient for LocalHttpProvider {
    /// Executes local generation or repair while keeping every write provisional in staging.
    async fn execute(
        &self,
        request: DocumentationRequest,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        self.execute_documentation_request(request, activity, cancellation)
            .await
    }
}

/// Builds strict function schemas whose path vocabulary is the exact workflow allowlist.
fn document_tools(required_paths: &[String]) -> Vec<Value> {
    vec![
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "list_expected_documents",
                "description": "List every exact relative Markdown path that must exist.",
                "strict": true,
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": [],
                    "additionalProperties": false
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "read_document",
                "description": "Read one expected staged document when it exists.",
                "strict": true,
                "parameters": {
                    "type": "object",
                    "properties": {
                        "relative_path": {"type": "string", "enum": required_paths}
                    },
                    "required": ["relative_path"],
                    "additionalProperties": false
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "write_document",
                "description": "Replace one expected staged Markdown document.",
                "strict": true,
                "parameters": {
                    "type": "object",
                    "properties": {
                        "relative_path": {"type": "string", "enum": required_paths},
                        "content": {"type": "string", "maxLength": MAX_DOCUMENT_BYTES}
                    },
                    "required": ["relative_path", "content"],
                    "additionalProperties": false
                }
            }
        }),
    ]
}

impl LocalHttpConfig {
    /// Creates validated transport settings that cannot target a remote HTTP service.
    pub fn new(
        endpoint: &str,
        model: &str,
        inactivity_timeout: Duration,
        hard_timeout: Duration,
        history_capacity: usize,
    ) -> Result<Self, AgentError> {
        let mut endpoint = Url::parse(endpoint).map_err(|error| {
            AgentError::InvalidRequest(format!("invalid local inference endpoint: {error}"))
        })?;
        if endpoint.scheme() != "http" || !is_loopback_host(endpoint.host()) {
            return Err(AgentError::InvalidRequest(
                "local inference endpoint must use HTTP on a numeric loopback address".to_owned(),
            ));
        }
        if endpoint.port() == Some(0) {
            return Err(AgentError::InvalidRequest(
                "local inference endpoint port must be greater than zero".to_owned(),
            ));
        }
        if !endpoint.username().is_empty()
            || endpoint.password().is_some()
            || endpoint.query().is_some()
            || endpoint.fragment().is_some()
        {
            return Err(AgentError::InvalidRequest(
                "local inference endpoint must not contain credentials, a query, or a fragment"
                    .to_owned(),
            ));
        }
        let normalized_path = endpoint.path().trim_end_matches('/');
        if normalized_path != "/v1" {
            return Err(AgentError::InvalidRequest(
                "local inference endpoint path must be /v1".to_owned(),
            ));
        }
        endpoint.set_path("/v1/");
        let model = model.trim();
        if model.is_empty() || model.chars().count() > 256 {
            return Err(AgentError::InvalidRequest(
                "local inference model must be a non-empty bounded identifier".to_owned(),
            ));
        }
        if inactivity_timeout.is_zero()
            || hard_timeout.is_zero()
            || inactivity_timeout > hard_timeout
            || history_capacity == 0
        {
            return Err(AgentError::InvalidRequest(
                "local inference timeouts and history capacity must be positive and coherent"
                    .to_owned(),
            ));
        }
        Ok(Self {
            endpoint,
            model: model.to_owned(),
            inactivity_timeout,
            hard_timeout,
            history_capacity,
        })
    }

    /// Returns the normalized loopback base URL ending in `/v1/`.
    pub const fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Returns the model identifier sent to the compatible HTTP API.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Returns the duration allowed without one valid streaming activity event.
    pub const fn inactivity_timeout(&self) -> Duration {
        self.inactivity_timeout
    }

    /// Returns the startup bound and total limit for initial analysis and documentation.
    pub const fn hard_timeout(&self) -> Duration {
        self.hard_timeout
    }

    /// Returns the maximum number of provider activity entries retained per operation.
    pub const fn history_capacity(&self) -> usize {
        self.history_capacity
    }
}

/// Disables only the total deadline when absent, leaving the caller's inactivity timer active.
async fn wait_for_deadline(deadline: Option<Instant>) {
    match deadline {
        Some(deadline) => tokio::time::sleep_until(deadline).await,
        None => std::future::pending::<()>().await,
    }
}

/// Returns whether one parsed URL host is an explicit IPv4 or IPv6 loopback address.
fn is_loopback_host(host: Option<Host<&str>>) -> bool {
    matches!(host, Some(Host::Ipv4(address)) if address.is_loopback())
        || matches!(host, Some(Host::Ipv6(address)) if address.is_loopback())
}

/// Maps transport read deadlines onto the provider inactivity contract consistently.
fn map_http_error(operation: &str, error: &reqwest::Error) -> AgentError {
    if error.is_timeout() {
        AgentError::Inactive
    } else {
        AgentError::Execution(format!("local inference {operation} failed: {error}"))
    }
}

/// Reads one small JSON probe response without trusting Content-Length or chunk boundaries.
async fn read_bounded_json(
    response: reqwest::Response,
    max_bytes: usize,
) -> Result<Value, AgentError> {
    let mut stream = response.bytes_stream();
    let mut bytes = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| {
            AgentError::Execution(format!("local readiness response failed: {error}"))
        })?;
        if bytes.len().saturating_add(chunk.len()) > max_bytes {
            return Err(AgentError::InvalidResponse(
                "local readiness response exceeded the supported size".to_owned(),
            ));
        }
        bytes.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&bytes).map_err(|error| {
        AgentError::InvalidResponse(format!("local readiness response was invalid: {error}"))
    })
}

/// Incrementally separates bounded UTF-8 server-sent events from arbitrary byte chunks.
#[derive(Debug)]
struct SseDecoder {
    buffer: Vec<u8>,
    max_event_bytes: usize,
}

impl SseDecoder {
    /// Creates a decoder whose retained partial event cannot exceed the supplied bound.
    const fn new(max_event_bytes: usize) -> Self {
        Self {
            buffer: Vec::new(),
            max_event_bytes,
        }
    }

    /// Appends one transport chunk and returns every complete `data` payload it contains.
    fn push(&mut self, chunk: &[u8]) -> Result<Vec<String>, AgentError> {
        self.buffer.extend_from_slice(chunk);
        let mut events = Vec::new();
        while let Some((boundary, separator_length)) = find_sse_boundary(&self.buffer) {
            if boundary > self.max_event_bytes {
                return Err(AgentError::InvalidEvent(
                    "local inference SSE event exceeded the supported size".to_owned(),
                ));
            }
            let frame = self.buffer.drain(..boundary).collect::<Vec<_>>();
            self.buffer.drain(..separator_length);
            if let Some(data) = decode_sse_frame(&frame)? {
                events.push(data);
            }
        }
        if self.buffer.len() > self.max_event_bytes {
            return Err(AgentError::InvalidEvent(
                "local inference SSE event exceeded the supported size".to_owned(),
            ));
        }
        Ok(events)
    }
}

/// Finds the first LF or CRLF blank-line boundary and its separator length.
fn find_sse_boundary(buffer: &[u8]) -> Option<(usize, usize)> {
    let lf = buffer
        .windows(2)
        .position(|window| window == b"\n\n")
        .map(|position| (position, 2));
    let crlf = buffer
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|position| (position, 4));
    match (lf, crlf) {
        (Some(left), Some(right)) => Some(if left.0 <= right.0 { left } else { right }),
        (Some(boundary), None) | (None, Some(boundary)) => Some(boundary),
        (None, None) => None,
    }
}

/// Extracts concatenated SSE `data` lines while ignoring comments and other fields.
fn decode_sse_frame(frame: &[u8]) -> Result<Option<String>, AgentError> {
    let frame = std::str::from_utf8(frame).map_err(|error| {
        AgentError::InvalidEvent(format!("local inference SSE was not UTF-8: {error}"))
    })?;
    let data = frame
        .lines()
        .filter_map(|line| line.strip_prefix("data:"))
        .map(str::trim_start)
        .collect::<Vec<_>>()
        .join("\n");
    Ok((!data.is_empty()).then_some(data))
}

/// Owns one reconstructed function call returned across streamed response fragments.
#[derive(Debug, Default)]
struct PartialToolCall {
    id: String,
    name: String,
    arguments: String,
}

/// Contains one complete assistant response after bounded streaming reconstruction.
#[derive(Debug)]
struct ChatCompletion {
    content: String,
    tool_calls: Vec<CompletedToolCall>,
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
}

/// Contains one validated function call ready for application-owned dispatch.
#[derive(Debug)]
struct CompletedToolCall {
    id: String,
    name: String,
    arguments: String,
}

/// Reconstructs bounded text, usage, and indexed function calls from chat deltas.
#[derive(Debug)]
struct CompletionAccumulator {
    content: String,
    tool_calls: BTreeMap<u64, PartialToolCall>,
    max_content_bytes: usize,
    max_tool_calls: usize,
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
}

impl CompletionAccumulator {
    /// Creates an empty accumulator with explicit response and tool-call limits.
    fn new(max_content_bytes: usize, max_tool_calls: usize) -> Self {
        Self {
            content: String::new(),
            tool_calls: BTreeMap::new(),
            max_content_bytes,
            max_tool_calls,
            input_tokens: None,
            output_tokens: None,
        }
    }

    /// Applies one compatible SSE JSON payload without trusting optional provider fields.
    fn ingest(&mut self, data: &str) -> Result<(), AgentError> {
        let event: Value = serde_json::from_str(data).map_err(|error| {
            AgentError::InvalidEvent(format!("local inference emitted invalid JSON: {error}"))
        })?;
        if let Some(content) = event
            .pointer("/choices/0/delta/content")
            .and_then(Value::as_str)
        {
            self.content.push_str(content);
            if self.content.len() > self.max_content_bytes {
                return Err(AgentError::InvalidResponse(
                    "local inference response exceeded the supported size".to_owned(),
                ));
            }
        }
        if let Some(tool_calls) = event
            .pointer("/choices/0/delta/tool_calls")
            .and_then(Value::as_array)
        {
            for tool_call in tool_calls {
                self.ingest_tool_call(tool_call)?;
            }
        }
        if let Some(usage) = event.get("usage") {
            self.input_tokens = usage.get("prompt_tokens").and_then(Value::as_u64);
            self.output_tokens = usage.get("completion_tokens").and_then(Value::as_u64);
        }
        Ok(())
    }

    /// Applies one indexed function-call fragment and enforces aggregate argument bounds.
    fn ingest_tool_call(&mut self, value: &Value) -> Result<(), AgentError> {
        let index = value.get("index").and_then(Value::as_u64).ok_or_else(|| {
            AgentError::InvalidResponse("local tool call omitted its index".to_owned())
        })?;
        if !self.tool_calls.contains_key(&index) && self.tool_calls.len() == self.max_tool_calls {
            return Err(AgentError::InvalidResponse(
                "local inference returned too many tool calls".to_owned(),
            ));
        }
        let tool_call = self.tool_calls.entry(index).or_default();
        if let Some(id) = value.get("id").and_then(Value::as_str) {
            tool_call.id.push_str(id);
        }
        if let Some(name) = value.pointer("/function/name").and_then(Value::as_str) {
            tool_call.name.push_str(name);
        }
        if let Some(arguments) = value.pointer("/function/arguments").and_then(Value::as_str) {
            tool_call.arguments.push_str(arguments);
        }
        if tool_call.id.len() > 256
            || tool_call.name.len() > 128
            || tool_call.arguments.len() > MAX_DOCUMENT_BYTES + 4096
        {
            return Err(AgentError::InvalidResponse(
                "local tool call exceeded the supported size".to_owned(),
            ));
        }
        Ok(())
    }

    /// Validates all required tool-call fields and returns their stable index order.
    fn finish(self) -> Result<ChatCompletion, AgentError> {
        let tool_calls = self
            .tool_calls
            .into_values()
            .map(|tool_call| {
                if tool_call.id.is_empty() || tool_call.name.is_empty() {
                    return Err(AgentError::InvalidResponse(
                        "local tool call omitted its id or function name".to_owned(),
                    ));
                }
                Ok(CompletedToolCall {
                    id: tool_call.id,
                    name: tool_call.name,
                    arguments: tool_call.arguments,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        if self.content.trim().is_empty() && tool_calls.is_empty() {
            return Err(AgentError::InvalidResponse(
                "local inference returned neither content nor tool calls".to_owned(),
            ));
        }
        Ok(ChatCompletion {
            content: self.content,
            tool_calls,
            input_tokens: self.input_tokens,
            output_tokens: self.output_tokens,
        })
    }
}

/// Provides the model an exact, bounded view of one provisional documentation staging tree.
#[derive(Debug)]
struct DocumentWorkspace {
    root: PathBuf,
    allowed: BTreeSet<String>,
}

impl DocumentWorkspace {
    /// Creates an exact path allowlist and rejects unsafe expected-document vocabulary.
    fn new(root: &Path, required_paths: &[String]) -> Result<Self, AgentError> {
        if required_paths.is_empty() {
            return Err(AgentError::InvalidRequest(
                "documentation requires at least one expected path".to_owned(),
            ));
        }
        let mut allowed = BTreeSet::new();
        for path in required_paths {
            if !is_safe_relative_path(path) || !allowed.insert(path.clone()) {
                return Err(AgentError::InvalidRequest(format!(
                    "unsafe or duplicate documentation path: {path}"
                )));
            }
        }
        Ok(Self {
            root: root.to_path_buf(),
            allowed,
        })
    }

    /// Dispatches one strict application-owned tool without exposing a generic filesystem API.
    fn execute_tool(&self, name: &str, arguments: &str) -> Result<String, AgentError> {
        match name {
            "list_expected_documents" => {
                let _: EmptyToolArguments = parse_tool_arguments(arguments)?;
                serde_json::to_string(&self.allowed).map_err(|error| {
                    AgentError::Execution(format!("could not list expected documents: {error}"))
                })
            }
            "read_document" => {
                let arguments: ReadToolArguments = parse_tool_arguments(arguments)?;
                self.read(&arguments.relative_path)
            }
            "write_document" => {
                let arguments: WriteToolArguments = parse_tool_arguments(arguments)?;
                self.write(&arguments.relative_path, &arguments.content)?;
                Ok("{\"written\":true}".to_owned())
            }
            _ => Err(AgentError::InvalidResponse(format!(
                "local inference requested unsupported tool: {name}"
            ))),
        }
    }

    /// Reads one exact allowlisted regular UTF-8 file with a per-document size bound.
    fn read(&self, relative_path: &str) -> Result<String, AgentError> {
        let path = self.resolve(relative_path)?;
        if !path.exists() {
            return Ok("{\"exists\":false,\"content\":null}".to_owned());
        }
        reject_link(&path)?;
        let metadata = fs::metadata(&path).map_err(|error| {
            AgentError::Execution(format!("could not inspect document: {error}"))
        })?;
        if !metadata.is_file() || metadata.len() > MAX_DOCUMENT_BYTES as u64 {
            return Err(AgentError::InvalidResponse(
                "staged document is not a bounded regular file".to_owned(),
            ));
        }
        let content = fs::read_to_string(&path)
            .map_err(|error| AgentError::Execution(format!("could not read document: {error}")))?;
        serde_json::to_string(&serde_json::json!({"exists": true, "content": content})).map_err(
            |error| AgentError::Execution(format!("could not serialize document: {error}")),
        )
    }

    /// Replaces one allowlisted staged file after checking per-file and aggregate bounds.
    fn write(&self, relative_path: &str, content: &str) -> Result<(), AgentError> {
        if content.len() > MAX_DOCUMENT_BYTES {
            return Err(AgentError::InvalidResponse(
                "generated document exceeded the supported size".to_owned(),
            ));
        }
        let path = self.resolve(relative_path)?;
        self.reject_unsafe_ancestors(&path)?;
        let existing_length = path.metadata().map_or(0, |metadata| metadata.len());
        let package_length = self.package_length()?.saturating_sub(existing_length);
        if package_length.saturating_add(content.len() as u64) > MAX_DOCUMENT_PACKAGE_BYTES {
            return Err(AgentError::InvalidResponse(
                "generated document package exceeded the supported size".to_owned(),
            ));
        }
        let parent = path.parent().ok_or_else(|| {
            AgentError::InvalidRequest("documentation path has no staging parent".to_owned())
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            AgentError::Execution(format!("could not create document directory: {error}"))
        })?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent).map_err(|error| {
            AgentError::Execution(format!("could not stage document write: {error}"))
        })?;
        temporary.write_all(content.as_bytes()).map_err(|error| {
            AgentError::Execution(format!("could not write staged document: {error}"))
        })?;
        temporary.persist(&path).map_err(|error| {
            AgentError::Execution(format!("could not replace staged document: {error}"))
        })?;
        Ok(())
    }

    /// Resolves only exact allowlisted relative paths beneath the staging root.
    fn resolve(&self, relative_path: &str) -> Result<PathBuf, AgentError> {
        if !self.allowed.contains(relative_path) {
            return Err(AgentError::InvalidResponse(format!(
                "local tool requested unlisted document: {relative_path}"
            )));
        }
        Ok(self.root.join(relative_path))
    }

    /// Rejects any existing link or non-directory ancestor before a staged write.
    fn reject_unsafe_ancestors(&self, target: &Path) -> Result<(), AgentError> {
        let relative = target.strip_prefix(&self.root).map_err(|_| {
            AgentError::InvalidRequest("documentation path escaped staging".to_owned())
        })?;
        let mut current = self.root.clone();
        reject_link(&current)?;
        for component in relative.components() {
            current.push(component.as_os_str());
            if current.exists() {
                reject_link(&current)?;
                if current != target && !current.is_dir() {
                    return Err(AgentError::InvalidResponse(
                        "documentation parent is not a directory".to_owned(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Sums only allowlisted regular-file sizes to enforce an aggregate generation bound.
    fn package_length(&self) -> Result<u64, AgentError> {
        self.allowed.iter().try_fold(0_u64, |total, relative| {
            let path = self.root.join(relative);
            if !path.exists() {
                return Ok(total);
            }
            reject_link(&path)?;
            let metadata = fs::metadata(path).map_err(|error| {
                AgentError::Execution(format!("could not inspect staged package: {error}"))
            })?;
            if !metadata.is_file() {
                return Err(AgentError::InvalidResponse(
                    "staged package contains a non-file at an expected path".to_owned(),
                ));
            }
            Ok(total.saturating_add(metadata.len()))
        })
    }

    /// Verifies that every expected path is now a bounded regular file inside staging.
    fn validate_complete(&self) -> Result<(), AgentError> {
        for relative in &self.allowed {
            let path = self.root.join(relative);
            if !path.exists() {
                return Err(AgentError::InvalidResponse(format!(
                    "local documentation omitted expected path: {relative}"
                )));
            }
            reject_link(&path)?;
            let metadata = fs::metadata(&path).map_err(|error| {
                AgentError::Execution(format!("could not inspect generated document: {error}"))
            })?;
            if !metadata.is_file() || metadata.len() > MAX_DOCUMENT_BYTES as u64 {
                return Err(AgentError::InvalidResponse(format!(
                    "local documentation produced an invalid expected path: {relative}"
                )));
            }
            fs::read_to_string(path).map_err(|error| {
                AgentError::InvalidResponse(format!(
                    "local documentation path is not valid UTF-8: {error}"
                ))
            })?;
        }
        if self.package_length()? > MAX_DOCUMENT_PACKAGE_BYTES {
            return Err(AgentError::InvalidResponse(
                "generated document package exceeded the supported size".to_owned(),
            ));
        }
        Ok(())
    }
}

/// Represents the exact empty argument object for listing expected documents.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyToolArguments {}

/// Contains the exact allowlisted path requested by the document read tool.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadToolArguments {
    relative_path: String,
}

/// Contains one exact allowlisted path and bounded replacement body.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct WriteToolArguments {
    relative_path: String,
    content: String,
}

/// Parses one strict tool argument object and reports malformed model output consistently.
fn parse_tool_arguments<T: for<'de> Deserialize<'de>>(arguments: &str) -> Result<T, AgentError> {
    serde_json::from_str(arguments).map_err(|error| {
        AgentError::InvalidResponse(format!("local tool arguments were invalid: {error}"))
    })
}

/// Returns whether one expected path is normalized, relative, and separator-safe.
fn is_safe_relative_path(value: &str) -> bool {
    !value.is_empty()
        && !value.contains('\\')
        && Path::new(value)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

/// Rejects symbolic links and Windows reparse points at a filesystem boundary.
fn reject_link(path: &Path) -> Result<(), AgentError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| AgentError::Execution(format!("could not inspect path: {error}")))?;
    if metadata.file_type().is_symlink() {
        return Err(AgentError::InvalidResponse(
            "documentation paths must not contain links".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc as std_mpsc;
    use std::thread;
    use std::time::Duration;

    use serde_json::{Value, json};
    use tokio::sync::mpsc;

    use super::{
        CompletionAccumulator, DocumentWorkspace, LocalHttpConfig, LocalHttpProvider, SseDecoder,
    };
    use crate::agents::{
        AgentClient, AgentError, AnalysisRequest, AutoAnswerClient, CancellationToken,
        DocumentationClient, DocumentationRequest, LocalDevice, LocalRuntimeConfig,
        ResearchCandidate, ResearchClient, ResearchJudgmentRequest, ResearchPlanRequest,
        ResearchQuestionContext, ResearchRequest, ResearchedAnswer,
    };

    /// Supplies a single eligible blocker for local coordinator timeout tests.
    fn timeout_plan_request() -> ResearchPlanRequest {
        ResearchPlanRequest::new(
            "{}".to_owned(),
            "q1",
            vec![
                ResearchQuestionContext::new("q1", "Q-001", "Which platform?", "Architecture.")
                    .unwrap(),
            ],
        )
        .unwrap()
    }

    /// Builds a local fixture client whose short deadlines expose total-time regressions.
    fn timeout_client(endpoint: &str) -> LocalHttpProvider {
        LocalHttpProvider::new(
            LocalHttpConfig::new(
                &format!("{endpoint}/v1"),
                "default",
                Duration::from_millis(200),
                Duration::from_millis(400),
                4,
            )
            .unwrap(),
        )
        .unwrap()
    }

    /// Keeps all three auto-answer stages alive beyond the analysis limit on valid SSE activity.
    #[tokio::test]
    async fn local_auto_answer_stages_outlive_hard_deadline() {
        let answer = r#"{"answer_text":"Use the supported platform.","notes":null,"evidence":[{"claim":"The platform is supported.","source":"https://example.com/source","source_title":"Primary source","reliability":"high","notes":null}]}"#;
        let judgment = format!(
            r#"{{"answers":[{{"question_id":"q1",{}}}]}}"#,
            &answer[1..answer.len() - 1]
        );
        for (stage, response) in [
            ("plan", r#"{"question_ids":["q1"]}"#),
            ("worker", answer),
            ("judge", judgment.as_str()),
        ] {
            let mut frames =
                vec!["data: {\"choices\":[{\"delta\":{\"content\":\" \"}}]}\n\n".to_owned(); 8];
            let event = json!({"choices":[{"delta":{"content":response}}]});
            frames.push(format!("data: {event}\n\ndata: [DONE]\n\n"));
            let (endpoint, server) = spawn_timed_sse_fixture(frames);
            let client = timeout_client(&endpoint);
            // A full display channel must not prevent stream activity from resetting the timer.
            let (activity, _receiver) = mpsc::channel(1);
            let cancellation = CancellationToken::new();
            let result = match stage {
                "plan" => client
                    .plan(timeout_plan_request(), activity, cancellation)
                    .await
                    .map(|_| ()),
                "worker" => AutoAnswerClient::research(
                    &client,
                    ResearchRequest::new("{}".to_owned(), "q1", "Which platform?", "Architecture.")
                        .unwrap()
                        .for_delegated_auto_answer(),
                    activity,
                    cancellation,
                )
                .await
                .map(|_| ()),
                _ => client
                    .judge(
                        ResearchJudgmentRequest::new(
                            "{}".to_owned(),
                            vec![
                                ResearchCandidate::new(
                                    "q1",
                                    ResearchedAnswer::from_json(answer).unwrap(),
                                )
                                .unwrap(),
                            ],
                        )
                        .unwrap(),
                        activity,
                        cancellation,
                    )
                    .await
                    .map(|_| ()),
            };
            server.join().unwrap();
            assert!(result.is_ok(), "{stage}: {result:?}");
        }
    }

    /// Ignores heartbeat bytes for liveness after a valid event, and bounds complete silence too.
    #[tokio::test]
    async fn local_auto_answer_still_times_out_without_valid_activity() {
        for heartbeat in ["", ": heartbeat\n\n"] {
            let mut frames =
                vec!["data: {\"choices\":[{\"delta\":{\"content\":\" \"}}]}\n\n".to_owned()];
            frames.extend(vec![heartbeat.to_owned(); 8]);
            let (endpoint, server) = spawn_timed_sse_fixture(frames);
            let client = timeout_client(&endpoint);
            let (activity, _receiver) = mpsc::channel(1);
            let result = client
                .plan(timeout_plan_request(), activity, CancellationToken::new())
                .await;
            server.join().unwrap();
            assert!(matches!(result, Err(AgentError::Inactive)), "{result:?}");
        }
    }

    /// Cancels a local coordinator after receiving real stream activity.
    #[tokio::test]
    async fn local_auto_answer_remains_cancellable() {
        let frames =
            vec!["data: {\"choices\":[{\"delta\":{\"content\":\" \"}}]}\n\n".to_owned(); 8];
        let (endpoint, server) = spawn_timed_sse_fixture(frames);
        let client = timeout_client(&endpoint);
        let (activity, mut receiver) = mpsc::channel(4);
        let cancellation = CancellationToken::new();
        let cancel = cancellation.clone();
        let task = tokio::spawn(async move {
            client
                .plan(timeout_plan_request(), activity, cancellation)
                .await
        });
        receiver.recv().await.unwrap();
        receiver.recv().await.unwrap();
        cancel.cancel();
        let result = tokio::time::timeout(Duration::from_millis(200), task)
            .await
            .unwrap()
            .unwrap();
        server.join().unwrap();
        assert!(matches!(result, Err(AgentError::Cancelled)), "{result:?}");
    }

    /// Keeps startup bounded even when an unready service sends bytes throughout the deadline.
    #[tokio::test]
    async fn local_auto_answer_runtime_startup_retains_hard_deadline() {
        let (endpoint, server) = spawn_timed_responses_fixture(vec![
            (
                "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                vec![],
            ),
            (
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n",
                vec![" ".to_owned(); 8],
            ),
        ]);
        let model_directory = tempfile::tempdir().unwrap();
        let runtime = LocalRuntimeConfig::new(
            model_directory.path().to_owned(),
            "gemma-4-12b-it-qat-q4_0.gguf",
            "ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0",
            LocalDevice::Cpu,
            url::Url::parse(&endpoint).unwrap().port().unwrap(),
        )
        .unwrap()
        .with_docker_executable("nonexistent-timeout-test-docker".into())
        .unwrap();
        let client = timeout_client(&endpoint).with_runtime(runtime).unwrap();
        let (activity, _receiver) = mpsc::channel(1);
        let result = client
            .plan(timeout_plan_request(), activity, CancellationToken::new())
            .await;
        server.join().unwrap();
        assert!(matches!(result, Err(AgentError::TimedOut)), "{result:?}");
    }

    /// Supplies complete HTTP headers before sending a timed sequence of SSE fragments.
    fn spawn_timed_sse_fixture(frames: Vec<String>) -> (String, thread::JoinHandle<()>) {
        spawn_timed_responses_fixture(vec![(
            "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n",
            frames,
        )])
    }

    /// Sends a finite sequence of timed HTTP responses and tolerates client cancellation.
    fn spawn_timed_responses_fixture(
        responses: Vec<(&'static str, Vec<String>)>,
    ) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            for (headers, frames) in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let _request = read_http_request(&mut stream);
                stream.write_all(headers.as_bytes()).unwrap();
                for frame in frames {
                    if stream.write_all(frame.as_bytes()).is_err() || stream.flush().is_err() {
                        return;
                    }
                    thread::sleep(Duration::from_millis(80));
                }
            }
        });
        (format!("http://{address}"), server)
    }

    /// Reassembles split SSE frames without accepting an unterminated or oversized event.
    #[test]
    fn transport_decoder_reassembles_bounded_sse_frames() {
        let mut decoder = SseDecoder::new(1024);
        assert!(
            decoder
                .push(b"data: {\"choices\":[{\"delta\":{\"content\":\"hel")
                .expect("the first fragment should be buffered")
                .is_empty()
        );
        let events = decoder
            .push(b"lo\"}}]}\n\ndata: [DONE]\n\n")
            .expect("the completed frames should decode");

        assert_eq!(events.len(), 2);
        assert!(events[0].contains("hello"));
        assert_eq!(events[1], "[DONE]");
        assert!(SseDecoder::new(4).push(b"data: too-large").is_err());
    }

    /// Reconstructs streamed text and fragmented strict tool calls by their stable indices.
    #[test]
    fn transport_accumulator_reconstructs_content_and_tool_calls() {
        let mut accumulator = CompletionAccumulator::new(1024, 4);
        accumulator
            .ingest(
                r#"{"choices":[{"delta":{"content":"ready ","tool_calls":[{"index":0,"id":"call-1","type":"function","function":{"name":"write_document","arguments":"{\"relative_"}}]}}]}"#,
            )
            .expect("the first completion fragment should decode");
        accumulator
            .ingest(
                r#"{"choices":[{"delta":{"content":"now","tool_calls":[{"index":0,"function":{"arguments":"path\":\"A.md\",\"content\":\"ok\"}"}}]}}],"usage":{"prompt_tokens":12,"completion_tokens":7}}"#,
            )
            .expect("the second completion fragment should decode");
        let completion = accumulator
            .finish()
            .expect("the accumulated completion should validate");

        assert_eq!(completion.content, "ready now");
        assert_eq!(completion.tool_calls.len(), 1);
        assert_eq!(completion.tool_calls[0].id, "call-1");
        assert_eq!(completion.tool_calls[0].name, "write_document");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&completion.tool_calls[0].arguments)
                .expect("the reconstructed arguments should be JSON"),
            json!({"relative_path":"A.md","content":"ok"})
        );
        assert_eq!(completion.input_tokens, Some(12));
        assert_eq!(completion.output_tokens, Some(7));
    }

    /// Allows exact staged document tools while rejecting traversal and unlisted paths.
    #[test]
    fn documentation_workspace_is_exactly_allowlisted() {
        let staging = tempfile::tempdir().expect("the staging directory should exist");
        let workspace = DocumentWorkspace::new(
            staging.path(),
            &["A.md".to_owned(), "nested/B.md".to_owned()],
        )
        .expect("safe expected documents should form an allowlist");

        workspace
            .execute_tool(
                "write_document",
                r#"{"relative_path":"nested/B.md","content":"bounded output"}"#,
            )
            .expect("an exact allowlisted document should be written");
        let read = workspace
            .execute_tool("read_document", r#"{"relative_path":"nested/B.md"}"#)
            .expect("an exact allowlisted document should be readable");
        assert!(read.contains("bounded output"));
        for arguments in [
            r#"{"relative_path":"../escape.md","content":"bad"}"#,
            r#"{"relative_path":"C:\\escape.md","content":"bad"}"#,
            r#"{"relative_path":"unlisted.md","content":"bad"}"#,
            r#"{"relative_path":"A.md","content":"ok","extra":true}"#,
        ] {
            assert!(
                workspace.execute_tool("write_document", arguments).is_err(),
                "unsafe tool arguments should fail closed: {arguments}"
            );
        }
        assert!(!staging.path().join("escape.md").exists());
    }

    /// Sends schema-constrained analysis to the loopback API and reconstructs its SSE result.
    #[tokio::test]
    async fn transport_executes_structured_analysis_over_local_http() {
        let analysis = r#"{"findings":[{"kind":"fact","statement":"Build an offline tool.","impact":"high","source":"user_brief"}]}"#;
        let event = json!({
            "choices": [{"delta": {"content": analysis}}],
            "usage": {"prompt_tokens": 9, "completion_tokens": 5}
        });
        let response = format!("data: {event}\n\ndata: [DONE]\n\n");
        let (endpoint, request_receiver, server) = spawn_sse_fixture(response);
        let config = LocalHttpConfig::new(
            &format!("{endpoint}/v1"),
            "default",
            Duration::from_secs(2),
            Duration::from_secs(5),
            20,
        )
        .expect("the fixture endpoint should be valid");
        let client = LocalHttpProvider::new(config).expect("the HTTP client should build");
        let (activity_sender, mut activity_receiver) = mpsc::channel(20);

        let execution = client
            .analyze(
                AnalysisRequest::new("Offline tool", "Build an offline tool.")
                    .expect("the analysis request should be valid"),
                activity_sender,
                CancellationToken::new(),
            )
            .await
            .expect("the fixture response should complete");
        server.join().expect("the fixture server should stop");
        let request = request_receiver
            .recv()
            .expect("the fixture should capture one request");

        assert_eq!(execution.provider, "local_http");
        assert_eq!(execution.model.as_deref(), Some("default"));
        assert_eq!(execution.response, analysis);
        assert_eq!(execution.input_tokens, Some(9));
        assert_eq!(execution.output_tokens, Some(5));
        assert!(request.starts_with("POST /v1/chat/completions HTTP/1.1"));
        assert!(request.contains("\"response_format\""));
        assert!(request.contains("\"json_schema\""));
        assert!(activity_receiver.recv().await.is_some());
    }

    /// Refuses HTTP redirects so a loopback server cannot reroute inference to another origin.
    #[tokio::test]
    async fn transport_does_not_follow_redirects() {
        let (endpoint, server) = spawn_redirect_fixture();
        let client = LocalHttpProvider::new(
            LocalHttpConfig::new(
                &format!("{endpoint}/v1"),
                "default",
                Duration::from_secs(1),
                Duration::from_secs(2),
                20,
            )
            .expect("the fixture endpoint should be valid"),
        )
        .expect("the HTTP client should build");
        let (activity_sender, _activity_receiver) = mpsc::channel(20);

        let error = client
            .analyze(
                AnalysisRequest::new("Redirect", "Do not leave loopback.")
                    .expect("the analysis request should be valid"),
                activity_sender,
                CancellationToken::new(),
            )
            .await
            .expect_err("a redirect must fail at the original response");
        server.join().expect("the redirect fixture should stop");

        assert!(matches!(error, AgentError::Execution(message) if message.contains("HTTP 302")));
    }

    /// Stops a request that produces no response headers before the inactivity limit.
    #[tokio::test]
    async fn transport_applies_inactivity_before_response_headers() {
        let (endpoint, server) = spawn_hanging_fixture(Duration::from_millis(150));
        let client = LocalHttpProvider::new(
            LocalHttpConfig::new(
                &format!("{endpoint}/v1"),
                "default",
                Duration::from_millis(50),
                Duration::from_secs(1),
                20,
            )
            .expect("the fixture endpoint should be valid"),
        )
        .expect("the HTTP client should build");
        let (activity_sender, _activity_receiver) = mpsc::channel(20);

        let error = client
            .analyze(
                AnalysisRequest::new("Silent", "Bound silent local requests.")
                    .expect("the analysis request should be valid"),
                activity_sender,
                CancellationToken::new(),
            )
            .await
            .expect_err("silence must reach the inactivity boundary");
        server.join().expect("the hanging fixture should stop");

        assert!(matches!(error, AgentError::Inactive));
    }

    /// Cancels a silent request immediately without waiting for either timeout boundary.
    #[tokio::test]
    async fn transport_honors_cancellation_before_response_headers() {
        let (endpoint, server) = spawn_hanging_fixture(Duration::from_millis(150));
        let client = LocalHttpProvider::new(
            LocalHttpConfig::new(
                &format!("{endpoint}/v1"),
                "default",
                Duration::from_secs(1),
                Duration::from_secs(2),
                20,
            )
            .expect("the fixture endpoint should be valid"),
        )
        .expect("the HTTP client should build");
        let cancellation = CancellationToken::new();
        let cancellation_trigger = cancellation.clone();
        let cancel_task = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(20)).await;
            cancellation_trigger.cancel();
        });
        let (activity_sender, _activity_receiver) = mpsc::channel(20);

        let error = client
            .analyze(
                AnalysisRequest::new("Cancelled", "Cancel the local request.")
                    .expect("the analysis request should be valid"),
                activity_sender,
                cancellation,
            )
            .await
            .expect_err("cancellation should stop the request");
        cancel_task
            .await
            .expect("the cancellation task should finish");
        server.join().expect("the hanging fixture should stop");

        assert!(matches!(error, AgentError::Cancelled));
    }

    /// Enforces the absolute deadline even while valid events keep resetting inactivity.
    #[tokio::test]
    async fn transport_hard_deadline_cannot_be_extended_by_activity() {
        let (endpoint, server) = spawn_active_fixture();
        let client = LocalHttpProvider::new(
            LocalHttpConfig::new(
                &format!("{endpoint}/v1"),
                "default",
                Duration::from_millis(60),
                Duration::from_millis(120),
                20,
            )
            .expect("the fixture endpoint should be valid"),
        )
        .expect("the HTTP client should build");
        let (activity_sender, _activity_receiver) = mpsc::channel(20);

        let error = client
            .analyze(
                AnalysisRequest::new("Active", "Bound even active local requests.")
                    .expect("the analysis request should be valid"),
                activity_sender,
                CancellationToken::new(),
            )
            .await
            .expect_err("activity must not move the absolute deadline");
        server.join().expect("the active fixture should stop");

        assert!(matches!(error, AgentError::TimedOut));
    }

    /// Reuses a healthy managed endpoint only after its loaded model identity is confirmed.
    #[tokio::test]
    async fn managed_runtime_verifies_health_and_model_before_reuse() {
        let analysis = r#"{"findings":[{"kind":"fact","statement":"Reuse the ready model.","impact":"high","source":"user_brief"}]}"#;
        let event = json!({"choices": [{"delta": {"content": analysis}}]});
        let responses = vec![
            String::new(),
            r#"{"data":[{"id":"default"}]}"#.to_owned(),
            format!("data: {event}\n\ndata: [DONE]\n\n"),
        ];
        let (endpoint, request_receiver, server) = spawn_sse_fixtures(responses);
        let endpoint_url = url::Url::parse(&endpoint).expect("the fixture URL should parse");
        let runtime = LocalRuntimeConfig::new(
            tempfile::tempdir()
                .expect("the model directory should exist")
                .keep(),
            "gemma-4-12b-it-qat-q4_0.gguf",
            "ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0",
            LocalDevice::Cpu,
            endpoint_url
                .port()
                .expect("the fixture URL should have a port"),
        )
        .expect("the managed runtime should validate");
        let client = LocalHttpProvider::new(
            LocalHttpConfig::new(
                &format!("{endpoint}/v1"),
                "default",
                Duration::from_secs(2),
                Duration::from_secs(5),
                20,
            )
            .expect("the fixture endpoint should be valid"),
        )
        .expect("the HTTP client should build")
        .with_runtime(runtime)
        .expect("the runtime port should match");
        let (activity_sender, _activity_receiver) = mpsc::channel(20);

        client
            .analyze(
                AnalysisRequest::new("Ready runtime", "Reuse a verified local model.")
                    .expect("the analysis request should be valid"),
                activity_sender,
                CancellationToken::new(),
            )
            .await
            .expect("the verified healthy endpoint should be reused");
        server.join().expect("the fixture server should stop");

        assert!(
            request_receiver
                .recv()
                .expect("the health request should be captured")
                .starts_with("GET /health HTTP/1.1")
        );
        assert!(
            request_receiver
                .recv()
                .expect("the model request should be captured")
                .starts_with("GET /v1/models HTTP/1.1")
        );
        assert!(
            request_receiver
                .recv()
                .expect("the completion request should be captured")
                .starts_with("POST /v1/chat/completions HTTP/1.1")
        );
    }

    /// Enables DuckDuckGo-backed search only for cited research worker requests.
    #[tokio::test]
    async fn structured_research_requests_web_search_and_validates_evidence() {
        let answer = r#"{"answer_text":"Use Rust 1.88.","notes":null,"evidence":[{"claim":"Rust 1.88 is available.","source":"https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/","source_title":"Announcing Rust 1.88.0","reliability":"high","notes":null}]}"#;
        let event = json!({"choices": [{"delta": {"content": answer}}]});
        let (endpoint, request_receiver, server) =
            spawn_sse_fixture(format!("data: {event}\n\ndata: [DONE]\n\n"));
        let client = LocalHttpProvider::new(
            LocalHttpConfig::new(
                &format!("{endpoint}/v1"),
                "default",
                Duration::from_secs(2),
                Duration::from_secs(5),
                20,
            )
            .expect("the fixture endpoint should be valid"),
        )
        .expect("the HTTP client should build");
        let (activity_sender, _activity_receiver) = mpsc::channel(20);

        let researched = ResearchClient::research(
            &client,
            ResearchRequest::new(
                "{\"project\":\"compiler\"}".to_owned(),
                "question-1",
                "Which stable Rust version is supported?",
                "The manifest must pin a minimum toolchain.",
            )
            .expect("the research request should be valid"),
            activity_sender,
            CancellationToken::new(),
        )
        .await
        .expect("the cited fixture response should validate");
        server.join().expect("the fixture server should stop");
        let request = request_receiver
            .recv()
            .expect("the fixture should capture one request");

        assert_eq!(researched.answer_text(), "Use Rust 1.88.");
        assert!(request.contains("\"web_search_options\""));
        assert!(request.contains("DuckDuckGo"));
    }

    /// Executes model-selected document writes through the exact application allowlist.
    #[tokio::test]
    async fn documentation_loop_dispatches_only_strict_staging_tools() {
        let tool_event = json!({
            "choices": [{"delta": {"tool_calls": [{
                "index": 0,
                "id": "call-1",
                "type": "function",
                "function": {
                    "name": "write_document",
                    "arguments": "{\"relative_path\":\"A.md\",\"content\":\"# A\\nGenerated locally.\\n\"}"
                }
            }]}}]
        });
        let final_event = json!({"choices": [{"delta": {"content": "complete"}}]});
        let responses = vec![
            format!("data: {tool_event}\n\ndata: [DONE]\n\n"),
            format!("data: {final_event}\n\ndata: [DONE]\n\n"),
        ];
        let (endpoint, request_receiver, server) = spawn_sse_fixtures(responses);
        let client = LocalHttpProvider::new(
            LocalHttpConfig::new(
                &format!("{endpoint}/v1"),
                "default",
                Duration::from_secs(2),
                Duration::from_secs(5),
                20,
            )
            .expect("the fixture endpoint should be valid"),
        )
        .expect("the HTTP client should build");
        let staging = tempfile::tempdir().expect("the staging directory should exist");

        DocumentationClient::execute(
            &client,
            DocumentationRequest::generation(
                "{\"project\":\"local docs\"}".to_owned(),
                vec!["A.md".to_owned()],
                staging.path().to_path_buf(),
            ),
            None,
            CancellationToken::new(),
        )
        .await
        .expect("the strict tool loop should complete");
        server.join().expect("the fixture server should stop");
        let first_request = request_receiver
            .recv()
            .expect("the fixture should capture the tool request");
        let second_request = request_receiver
            .recv()
            .expect("the fixture should capture the tool result request");

        assert_eq!(
            fs::read_to_string(staging.path().join("A.md"))
                .expect("the generated document should be readable"),
            "# A\nGenerated locally.\n"
        );
        assert!(first_request.contains("\"list_expected_documents\""));
        assert!(first_request.contains("\"read_document\""));
        assert!(first_request.contains("\"write_document\""));
        assert!(first_request.contains("\"strict\":true"));
        assert!(first_request.contains("\"enum\":[\"A.md\"]"));
        let second_body = request_json(&second_request);
        assert_eq!(
            second_body
                .pointer("/messages/2/tool_call_id")
                .and_then(Value::as_str),
            Some("call-1")
        );
        assert_eq!(
            second_body
                .pointer("/messages/2/content")
                .and_then(Value::as_str),
            Some("{\"written\":true}")
        );
    }

    /// Starts one single-request loopback SSE fixture and returns its captured HTTP request.
    fn spawn_sse_fixture(
        response_body: String,
    ) -> (String, std_mpsc::Receiver<String>, thread::JoinHandle<()>) {
        spawn_sse_fixtures(vec![response_body])
    }

    /// Starts a bounded sequence of loopback SSE responses for a multi-round tool loop.
    fn spawn_sse_fixtures(
        response_bodies: Vec<String>,
    ) -> (String, std_mpsc::Receiver<String>, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("the fixture should bind");
        let address = listener
            .local_addr()
            .expect("the fixture should have an address");
        let (request_sender, request_receiver) = std_mpsc::channel();
        let server = thread::spawn(move || {
            for response_body in response_bodies {
                let (mut stream, _) = listener.accept().expect("the fixture should accept");
                let request = read_http_request(&mut stream);
                request_sender
                    .send(request)
                    .expect("the fixture request should be observed");
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    response_body.len(),
                    response_body
                );
                stream
                    .write_all(response.as_bytes())
                    .expect("the fixture should respond");
            }
        });
        (format!("http://{address}"), request_receiver, server)
    }

    /// Returns one loopback fixture that tries to redirect the client away from its base origin.
    fn spawn_redirect_fixture() -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("the fixture should bind");
        let address = listener
            .local_addr()
            .expect("the fixture should have an address");
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("the fixture should accept");
            let _request = read_http_request(&mut stream);
            stream
                .write_all(
                    b"HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/v1/chat/completions\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .expect("the fixture should send a redirect");
        });
        (format!("http://{address}"), server)
    }

    /// Returns one loopback fixture that accepts a request but emits no response activity.
    fn spawn_hanging_fixture(delay: Duration) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("the fixture should bind");
        let address = listener
            .local_addr()
            .expect("the fixture should have an address");
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("the fixture should accept");
            let _request = read_http_request(&mut stream);
            thread::sleep(delay);
        });
        (format!("http://{address}"), server)
    }

    /// Returns one loopback fixture that emits valid SSE activity without ever completing.
    fn spawn_active_fixture() -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("the fixture should bind");
        let address = listener
            .local_addr()
            .expect("the fixture should have an address");
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("the fixture should accept");
            let _request = read_http_request(&mut stream);
            stream
                .write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n",
                )
                .expect("the fixture should send response headers");
            for _ in 0..10 {
                if stream
                    .write_all(b"data: {\"choices\":[{\"delta\":{\"content\":\"x\"}}]}\n\n")
                    .is_err()
                {
                    break;
                }
                if stream.flush().is_err() {
                    break;
                }
                thread::sleep(Duration::from_millis(25));
            }
        });
        (format!("http://{address}"), server)
    }

    /// Reads one bounded HTTP request including the body declared by Content-Length.
    fn read_http_request(stream: &mut std::net::TcpStream) -> String {
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("the fixture timeout should configure");
        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let count = stream.read(&mut buffer).expect("the request should read");
            if count == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..count]);
            let text = String::from_utf8_lossy(&request);
            if let Some(header_end) = text.find("\r\n\r\n") {
                let content_length = text[..header_end]
                    .lines()
                    .find_map(|line| {
                        line.strip_prefix("content-length:")
                            .or_else(|| line.strip_prefix("Content-Length:"))
                    })
                    .and_then(|value| value.trim().parse::<usize>().ok())
                    .unwrap_or(0);
                if request.len() >= header_end + 4 + content_length {
                    break;
                }
            }
            assert!(
                request.len() <= 2 * 1024 * 1024,
                "fixture request was unbounded"
            );
        }
        String::from_utf8(request).expect("the fixture request should be UTF-8")
    }

    /// Parses the JSON body captured after one fixture request's header boundary.
    fn request_json(request: &str) -> Value {
        let (_, body) = request
            .split_once("\r\n\r\n")
            .expect("the fixture request should contain a header boundary");
        serde_json::from_str(body).expect("the fixture request body should be JSON")
    }
}
