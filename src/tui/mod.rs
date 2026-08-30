//! Ratatui project inspection with safe terminal setup and restoration.

mod workspace;

pub use workspace::{
    QuestionDraft, SnapshotDiff, TimelineEntry, WorkspaceCommand, WorkspaceFocus,
    WorkspaceInputError, WorkspaceSection, WorkspaceState, render_workspace,
};

use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use thiserror::Error;

use tokio::sync::{mpsc, oneshot};

use crate::agents::{
    ActivityEvent, ActivityHistory, ActivityKind, AgentError, AgentExecution, CancellationToken,
    CodexCliClient, CodexCliConfig, DocumentationClient, DocumentationRequest,
    resolve_codex_executable, sanitize_terminal_text,
};
use crate::domain::{ProjectId, ProjectSnapshot};
use crate::storage::SqliteStore;
use crate::workflow::{
    AutoAnswerProgress, ProjectService, WorkflowError, WorkflowRunOutcome, WorkflowRunStop,
    WorkflowRunner, requires_documentation_client,
};

/// Tells the terminal loop whether a command stays local, quits, or starts background execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceDirective {
    /// Keeps the current workbench open without starting a worker.
    Continue,
    /// Closes the workbench without additional mutation.
    Quit,
    /// Starts or resumes one background run and identifies provider-backed presentation.
    Execute {
        /// Shows the full-screen overlay only while Codex generation or repair is required.
        show_overlay: bool,
    },
    /// Starts fixed three-worker automatic clarification with a dedicated progress board.
    AutoAnswer,
}

/// Carries process and storage configuration needed only when the workbench executes Codex.
#[derive(Debug, Clone)]
pub struct WorkspaceRuntimeConfig {
    data_dir: PathBuf,
    codex_override: Option<OsString>,
    timeout: Duration,
    history_capacity: usize,
    activity_capacity: usize,
    disable_skills: bool,
}

impl WorkspaceRuntimeConfig {
    /// Creates skill-free background execution configuration without resolving Codex eagerly.
    pub fn new(
        data_dir: &Path,
        codex_override: Option<OsString>,
        timeout: Duration,
        history_capacity: usize,
        activity_capacity: usize,
    ) -> Self {
        Self {
            data_dir: data_dir.to_path_buf(),
            codex_override,
            timeout,
            history_capacity,
            activity_capacity,
            disable_skills: true,
        }
    }

    /// Selects skill suppression for Codex work started later from this workbench.
    pub fn with_skills_disabled(mut self, disable_skills: bool) -> Self {
        self.disable_skills = disable_skills;
        self
    }

    /// Returns the authoritative database path shared by foreground and worker connections.
    fn database_path(&self) -> PathBuf {
        self.data_dir.join("project-init.sqlite3")
    }

    /// Returns the accepted package location for one project.
    fn output_path(&self, project_id: &ProjectId) -> PathBuf {
        self.data_dir
            .join("projects")
            .join(project_id.as_str())
            .join("Docs")
    }
}

/// Owns channels and cancellation for one active background workbench execution.
struct ActiveWorkspaceExecution {
    cancellation: CancellationToken,
    activity: mpsc::Receiver<ActivityEvent>,
    auto_answer_progress: mpsc::Receiver<AutoAnswerProgress>,
    provider_activity: mpsc::Receiver<bool>,
    result: oneshot::Receiver<Result<WorkflowRunOutcome, String>>,
}

impl Drop for ActiveWorkspaceExecution {
    /// Cancels provider work if terminal execution exits before the worker reports completion.
    fn drop(&mut self) {
        self.cancellation.cancel();
    }
}

/// Decorates the Codex documentation client with exact provider-active UI signals.
struct SignalingDocumentationClient {
    inner: CodexCliClient,
    provider_activity: mpsc::Sender<bool>,
}

#[async_trait::async_trait]
impl DocumentationClient for SignalingDocumentationClient {
    /// Brackets only the external Codex call, leaving local workflow work unobscured.
    async fn execute(
        &self,
        request: DocumentationRequest,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        let _ = self.provider_activity.try_send(true);
        let result = self.inner.execute(request, activity, cancellation).await;
        let _ = self.provider_activity.try_send(false);
        result
    }
}

/// Describes the terminal-visible result of a transient project creation session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreationOutcome {
    /// The background analyzer has not produced a terminal result yet.
    Running,
    /// Analysis validated and the authoritative project committed successfully.
    Completed,
    /// The user cancelled analysis before authoritative persistence.
    Cancelled,
    /// Analysis stopped because an external or validation boundary failed.
    Failed,
}

/// Returns the transient TUI result without allowing presentation code to persist state.
#[derive(Debug)]
pub enum CreationRunResult {
    /// Carries a successful execution to workflow validation and atomic persistence.
    Completed(AgentExecution),
    /// Reports immediate terminal cancellation with no execution result.
    Cancelled,
    /// Carries a sanitized failure after the user closes the error view.
    Failed(String),
}

/// Describes a workbench startup or terminal lifecycle failure that cannot remain in-session.
#[derive(Debug, Error)]
pub enum WorkspaceRunError {
    /// Propagates terminal setup, draw, or event-read failures after safe restoration.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Propagates an initial authoritative snapshot or activity load failure.
    #[error(transparent)]
    Workflow(#[from] WorkflowError),
    /// Reports use outside the Tokio runtime required for online background work.
    #[error("the interactive workbench requires an active Tokio runtime")]
    RuntimeUnavailable,
}

/// Owns bounded, scrollable presentation state for live initial-brief analysis.
#[derive(Debug, Clone)]
pub struct CreationState {
    project_name: String,
    history: ActivityHistory,
    scroll_from_end: usize,
    outcome: CreationOutcome,
    project_id: Option<String>,
    error: Option<String>,
    spinner_index: usize,
}

impl CreationState {
    /// Creates a running transient session without allocating authoritative identity.
    pub fn new(project_name: &str, history_capacity: usize) -> Self {
        Self {
            project_name: project_name.to_owned(),
            history: ActivityHistory::bounded(history_capacity),
            scroll_from_end: 0,
            outcome: CreationOutcome::Running,
            project_id: None,
            error: None,
            spinner_index: 0,
        }
    }

    /// Appends one live event and preserves the user's explicit scroll position.
    pub fn push_activity(&mut self, event: ActivityEvent) {
        self.history.push(event);
    }

    /// Returns retained activity in stable chronological order.
    pub fn visible_activity(&self) -> Vec<ActivityEvent> {
        self.history.events()
    }

    /// Moves the viewport toward older activity without exceeding retained history.
    pub fn scroll_up(&mut self) {
        self.scroll_from_end = self
            .scroll_from_end
            .saturating_add(1)
            .min(self.history.len().saturating_sub(1));
    }

    /// Moves the viewport toward newer activity and resumes follow mode at zero.
    pub fn scroll_down(&mut self) {
        self.scroll_from_end = self.scroll_from_end.saturating_sub(1);
    }

    /// Records immediate user cancellation without assigning a project identity.
    pub fn cancel(&mut self) {
        if self.outcome == CreationOutcome::Running {
            self.outcome = CreationOutcome::Cancelled;
        }
    }

    /// Records the identity returned by a successful atomic project commit.
    pub fn complete(&mut self, project_id: &str) {
        self.outcome = CreationOutcome::Completed;
        self.project_id = Some(project_id.to_owned());
        self.error = None;
    }

    /// Records a sanitized terminal failure without creating a project identity.
    pub fn fail(&mut self, message: &str) {
        self.outcome = CreationOutcome::Failed;
        self.project_id = None;
        self.error = Some(sanitize_terminal_text(message));
    }

    /// Advances the spinner independently of background work completion.
    pub fn tick(&mut self) {
        self.spinner_index = self.spinner_index.wrapping_add(1);
    }

    /// Returns the current terminal outcome.
    pub const fn outcome(&self) -> CreationOutcome {
        self.outcome
    }

    /// Returns the committed identity only after successful completion.
    pub fn project_id(&self) -> Option<&str> {
        self.project_id.as_deref()
    }
}

/// Opens a full-screen project overview until the user presses Q or Escape.
pub fn run(snapshot: &ProjectSnapshot) -> io::Result<()> {
    run_with_activity(snapshot, &[])
}

/// Opens the project overview with its persisted successful-analysis timeline.
pub fn run_with_activity(snapshot: &ProjectSnapshot, activity: &[ActivityEvent]) -> io::Result<()> {
    ratatui::run(|terminal| -> io::Result<()> {
        loop {
            terminal.draw(|frame| render(frame, snapshot, activity))?;
            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                return Ok(());
            }
        }
    })
}

/// Opens the interactive workbench and keeps recoverable workflow failures visible in-session.
pub fn run_workspace(
    service: &mut ProjectService,
    project_id: &ProjectId,
    runtime_config: WorkspaceRuntimeConfig,
) -> Result<(), WorkspaceRunError> {
    run_workspace_with_initial_mode(service, project_id, runtime_config, false)
}

/// Opens the workbench and immediately starts fixed three-worker automatic clarification.
pub fn run_workspace_auto_answer(
    service: &mut ProjectService,
    project_id: &ProjectId,
    runtime_config: WorkspaceRuntimeConfig,
) -> Result<(), WorkspaceRunError> {
    run_workspace_with_initial_mode(service, project_id, runtime_config, true)
}

/// Runs the shared workbench event loop with an optional initial automatic-answer execution.
fn run_workspace_with_initial_mode(
    service: &mut ProjectService,
    project_id: &ProjectId,
    runtime_config: WorkspaceRuntimeConfig,
    initial_auto_answer: bool,
) -> Result<(), WorkspaceRunError> {
    let snapshot = service.inspect_project(project_id)?;
    let activity = service.agent_activity(project_id)?;
    let mut state = WorkspaceState::new(snapshot, activity);
    let output = runtime_config.output_path(project_id);
    refresh_workspace_context(service, &mut state, &output)?;
    let runtime =
        tokio::runtime::Handle::try_current().map_err(|_| WorkspaceRunError::RuntimeUnavailable)?;
    let mut execution = initial_auto_answer.then(|| {
        state.begin_auto_answer_execution();
        spawn_workspace_execution(&runtime, runtime_config.clone(), project_id.clone(), true)
    });
    ratatui::run(|terminal| -> io::Result<()> {
        loop {
            let completed = execution.as_mut().and_then(|active| {
                while let Ok(provider_active) = active.provider_activity.try_recv() {
                    if provider_active {
                        state.begin_execution();
                    } else if state.execution_active() {
                        state.finish_execution("Codex finished. Applying local workflow checks…");
                    }
                }
                while let Ok(event) = active.activity.try_recv() {
                    state.push_execution_activity(event);
                }
                while let Ok(progress) = active.auto_answer_progress.try_recv() {
                    state.push_auto_answer_progress(progress);
                }
                match active.result.try_recv() {
                    Ok(result) => Some(result),
                    Err(oneshot::error::TryRecvError::Closed) => Some(Err(
                        "workflow worker ended without returning a result".to_owned(),
                    )),
                    Err(oneshot::error::TryRecvError::Empty) => None,
                }
            });
            if let Some(result) = completed {
                execution = None;
                if let Err(error) = refresh_workspace_context(service, &mut state, &output) {
                    state.finish_execution(&format!("Error: {error}"));
                } else {
                    match result {
                        Ok(outcome) => state.finish_execution(match outcome.stop {
                            WorkflowRunStop::Boundary => "Workflow paused for user input.",
                            WorkflowRunStop::Complete => {
                                "Workflow complete. The documentation package passed validation."
                            }
                            WorkflowRunStop::Cancelled => {
                                "Workflow cancelled safely and remains resumable."
                            }
                        }),
                        Err(error) => state.finish_execution(&format!("Error: {error}")),
                    }
                }
            }
            terminal.draw(|frame| render_workspace(frame, &state))?;
            let next_event = if execution.is_some() {
                event::poll(Duration::from_millis(80))?
                    .then(event::read)
                    .transpose()?
            } else {
                Some(event::read()?)
            };
            let Some(Event::Key(key)) = next_event else {
                continue;
            };
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if let Some(active) = execution.as_ref() {
                if key.code == KeyCode::Esc {
                    active.cancellation.cancel();
                }
                continue;
            }
            let command = match state.handle_key(key) {
                Ok(command) => command,
                Err(error) => {
                    state.show_error(&error.to_string());
                    continue;
                }
            };
            if let Some(command) = command {
                match apply_interactive_workspace_command(service, &mut state, command, &output) {
                    Ok(WorkspaceDirective::Continue) => {}
                    Ok(WorkspaceDirective::Quit) => return Ok(()),
                    Ok(WorkspaceDirective::Execute { show_overlay }) => {
                        if show_overlay {
                            state.show_notice("Preparing Codex documentation execution…");
                        } else {
                            state.show_notice("Resuming workflow…");
                        }
                        execution = Some(spawn_workspace_execution(
                            &runtime,
                            runtime_config.clone(),
                            project_id.clone(),
                            false,
                        ));
                    }
                    Ok(WorkspaceDirective::AutoAnswer) => {
                        state.begin_auto_answer_execution();
                        execution = Some(spawn_workspace_execution(
                            &runtime,
                            runtime_config.clone(),
                            project_id.clone(),
                            true,
                        ));
                    }
                    Err(error) => state.show_error(&error.to_string()),
                }
            }
        }
    })?;
    Ok(())
}

/// Spawns one shared runner with its own service connection and bounded result channels.
fn spawn_workspace_execution(
    runtime: &tokio::runtime::Handle,
    config: WorkspaceRuntimeConfig,
    project_id: ProjectId,
    auto_answer: bool,
) -> ActiveWorkspaceExecution {
    let (activity_sender, activity_receiver) = mpsc::channel(config.activity_capacity);
    let (auto_progress_sender, auto_progress_receiver) = mpsc::channel(config.activity_capacity);
    let (provider_sender, provider_receiver) = mpsc::channel(4);
    let (result_sender, result_receiver) = oneshot::channel();
    let cancellation = CancellationToken::new();
    let worker_cancellation = cancellation.clone();
    runtime.spawn(async move {
        let result = run_workspace_worker(
            config,
            project_id,
            activity_sender,
            auto_progress_sender,
            provider_sender,
            worker_cancellation,
            auto_answer,
        )
        .await;
        let _ = result_sender.send(result);
    });
    ActiveWorkspaceExecution {
        cancellation,
        activity: activity_receiver,
        auto_answer_progress: auto_progress_receiver,
        provider_activity: provider_receiver,
        result: result_receiver,
    }
}

/// Opens worker-owned dependencies and resolves Codex only for provider-backed steps.
async fn run_workspace_worker(
    config: WorkspaceRuntimeConfig,
    project_id: ProjectId,
    activity: mpsc::Sender<ActivityEvent>,
    auto_answer_progress: mpsc::Sender<AutoAnswerProgress>,
    provider_activity: mpsc::Sender<bool>,
    cancellation: CancellationToken,
    auto_answer: bool,
) -> Result<WorkflowRunOutcome, String> {
    let store = SqliteStore::open(&config.database_path()).map_err(|error| error.to_string())?;
    let mut service = ProjectService::new(store);
    let output = config.output_path(&project_id);
    let status = service
        .workflow_status(&project_id, &output)
        .map_err(|error| error.to_string())?;
    if !auto_answer && !requires_documentation_client(&status) {
        let mut runner = WorkflowRunner::offline(service, &output);
        return runner
            .run_until_pause(&project_id, None, Some(activity), cancellation)
            .await
            .map_err(|error| sanitize_terminal_text(&error.to_string()));
    }
    let executable = match resolve_codex_executable(config.codex_override.clone()) {
        Ok(executable) => executable,
        Err(error) => {
            if service
                .active_run(&project_id)
                .map_err(|active_error| active_error.to_string())?
                .is_some()
            {
                service
                    .pause_run(&project_id, "capability_unavailable")
                    .map_err(|pause_error| pause_error.to_string())?;
            }
            return Err(error.to_string());
        }
    };
    let codex = CodexCliClient::new(CodexCliConfig {
        executable,
        working_directory: config.data_dir.clone(),
        timeout: config.timeout,
        history_capacity: config.history_capacity,
    })
    .with_skills_disabled(config.disable_skills);
    let client = SignalingDocumentationClient {
        inner: codex.clone(),
        provider_activity,
    };
    let mut runner = WorkflowRunner::online(service, &output, &client);
    if auto_answer {
        runner = runner.with_auto_answer_client(Arc::new(codex));
        runner
            .run_until_pause_with_auto_answer_progress(
                &project_id,
                None,
                Some(activity),
                Some(auto_answer_progress),
                cancellation,
            )
            .await
            .map_err(|error| sanitize_terminal_text(&error.to_string()))
    } else {
        runner
            .run_until_pause(&project_id, None, Some(activity), cancellation)
            .await
            .map_err(|error| sanitize_terminal_text(&error.to_string()))
    }
}

/// Applies one typed workbench command and refreshes its disposable snapshot after accepted writes.
pub fn apply_workspace_command(
    service: &mut ProjectService,
    state: &mut WorkspaceState,
    command: WorkspaceCommand,
) -> Result<bool, WorkflowError> {
    let project_id = state.snapshot().project.id().clone();
    let threshold_notice = match &command {
        WorkspaceCommand::SetClarificationThreshold(value) => {
            Some(format!("Consequential threshold updated to {value}"))
        }
        WorkspaceCommand::Answer { .. }
        | WorkspaceCommand::AskQuestion(_)
        | WorkspaceCommand::Resume
        | WorkspaceCommand::AutoAnswer
        | WorkspaceCommand::StartRun(_)
        | WorkspaceCommand::StartAutoAnswer(_)
        | WorkspaceCommand::ApproveDecision { .. }
        | WorkspaceCommand::RejectDecision { .. }
        | WorkspaceCommand::RequestRepair
        | WorkspaceCommand::Quit => None,
    };
    match command {
        WorkspaceCommand::Quit => return Ok(false),
        WorkspaceCommand::Answer {
            question_id,
            answer,
        } => service.answer_question(&question_id, &answer, None)?,
        WorkspaceCommand::AskQuestion(request) => {
            service.ask_question(&project_id, request)?;
        }
        WorkspaceCommand::SetClarificationThreshold(value) => {
            service.set_clarification_threshold(&project_id, value)?;
        }
        WorkspaceCommand::ApproveDecision {
            decision_id,
            reason,
        } => service.approve_decision(&decision_id, &reason)?,
        WorkspaceCommand::RejectDecision {
            decision_id,
            reason,
        } => service.reject_decision(&decision_id, &reason)?,
        WorkspaceCommand::Resume
        | WorkspaceCommand::AutoAnswer
        | WorkspaceCommand::StartRun(_)
        | WorkspaceCommand::StartAutoAnswer(_)
        | WorkspaceCommand::RequestRepair => {
            return Err(WorkflowError::InvalidWorkflowInput(
                "workflow execution commands require the interactive coordinator".to_owned(),
            ));
        }
    }
    let snapshot = service.inspect_project(&project_id)?;
    state.apply_snapshot(snapshot);
    if let Some(notice) = threshold_notice {
        state.show_notice(&notice);
    }
    Ok(true)
}

/// Applies an interactive command or returns a typed background-execution directive.
pub fn apply_interactive_workspace_command(
    service: &mut ProjectService,
    state: &mut WorkspaceState,
    command: WorkspaceCommand,
    output: &std::path::Path,
) -> Result<WorkspaceDirective, WorkflowError> {
    let project_id = state.snapshot().project.id().clone();
    match command {
        WorkspaceCommand::Resume => {
            if service.active_run(&project_id)?.is_some() {
                let status = service.workflow_status(&project_id, output)?;
                Ok(WorkspaceDirective::Execute {
                    show_overlay: requires_documentation_client(&status),
                })
            } else {
                state.open_policy_selection();
                Ok(WorkspaceDirective::Continue)
            }
        }
        WorkspaceCommand::AutoAnswer => {
            if service.active_run(&project_id)?.is_some() {
                Ok(WorkspaceDirective::AutoAnswer)
            } else {
                state.open_auto_answer_policy_selection();
                Ok(WorkspaceDirective::Continue)
            }
        }
        WorkspaceCommand::StartRun(policy) => {
            service.start_run(&project_id, policy)?;
            refresh_workspace_context(service, state, output)?;
            let status = service.workflow_status(&project_id, output)?;
            Ok(WorkspaceDirective::Execute {
                show_overlay: requires_documentation_client(&status),
            })
        }
        WorkspaceCommand::StartAutoAnswer(policy) => {
            service.start_run(&project_id, policy)?;
            refresh_workspace_context(service, state, output)?;
            Ok(WorkspaceDirective::AutoAnswer)
        }
        WorkspaceCommand::RequestRepair => {
            service.request_repair(&project_id, output)?;
            refresh_workspace_context(service, state, output)?;
            state.show_notice("Repair authorized. Enter /resume to run one repair attempt.");
            Ok(WorkspaceDirective::Continue)
        }
        WorkspaceCommand::Quit => Ok(WorkspaceDirective::Quit),
        command => {
            apply_workspace_command(service, state, command)?;
            refresh_workspace_context(service, state, output)?;
            Ok(WorkspaceDirective::Continue)
        }
    }
}

/// Reloads the snapshot, workflow step, and validation context after accepted state changes.
fn refresh_workspace_context(
    service: &ProjectService,
    state: &mut WorkspaceState,
    output: &std::path::Path,
) -> Result<(), WorkflowError> {
    let project_id = state.snapshot().project.id().clone();
    let snapshot = service.inspect_project(&project_id)?;
    state.apply_snapshot(snapshot);
    let status = service.workflow_status_if_started(&project_id, output)?;
    let findings = if status.as_ref().is_some_and(|status| {
        matches!(
            status.step,
            crate::domain::WorkflowStep::RepairRequired { .. }
        )
    }) {
        service.latest_validation_findings(&project_id)?
    } else {
        Vec::new()
    };
    state.set_workflow_context(status, findings);
    Ok(())
}

/// Runs the live creation view while a background agent task owns external execution.
pub fn run_creation(
    project_name: &str,
    mut activity: mpsc::Receiver<ActivityEvent>,
    mut result: oneshot::Receiver<Result<AgentExecution, AgentError>>,
    cancellation: CancellationToken,
    history_capacity: usize,
) -> io::Result<CreationRunResult> {
    ratatui::run(|terminal| {
        let mut state = CreationState::new(project_name, history_capacity);
        loop {
            while let Ok(event) = activity.try_recv() {
                state.push_activity(event);
            }
            let completed = if state.outcome == CreationOutcome::Running {
                match result.try_recv() {
                    Ok(Ok(execution)) => Some(execution),
                    Ok(Err(error)) => {
                        state.fail(&error.to_string());
                        None
                    }
                    Err(oneshot::error::TryRecvError::Closed) => {
                        state.fail("Analysis task ended without returning a result");
                        None
                    }
                    Err(oneshot::error::TryRecvError::Empty) => None,
                }
            } else {
                None
            };
            terminal.draw(|frame| render_creation(frame, &state))?;
            if let Some(execution) = completed {
                return Ok(CreationRunResult::Completed(execution));
            }
            if event::poll(Duration::from_millis(80))?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                match key.code {
                    KeyCode::Up | KeyCode::PageUp => state.scroll_up(),
                    KeyCode::Down | KeyCode::PageDown => state.scroll_down(),
                    KeyCode::Esc if state.outcome == CreationOutcome::Running => {
                        cancellation.cancel();
                        state.cancel();
                        return Ok(CreationRunResult::Cancelled);
                    }
                    KeyCode::Esc | KeyCode::Enter if state.outcome == CreationOutcome::Failed => {
                        return Ok(CreationRunResult::Failed(
                            state
                                .error
                                .clone()
                                .unwrap_or_else(|| "Analysis failed".to_owned()),
                        ));
                    }
                    _ => {}
                }
            }
            state.tick();
        }
    })
}

/// Renders a responsive creation view with live progress, retained history, and controls.
fn render_creation(frame: &mut Frame<'_>, state: &CreationState) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(4),
            Constraint::Length(3),
        ])
        .split(frame.area());
    let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"][state.spinner_index % 10];
    let (label, symbol, color) = match state.outcome {
        CreationOutcome::Running => ("ANALYZING", spinner, Color::Cyan),
        CreationOutcome::Completed => ("COMPLETE", "✓", Color::Green),
        CreationOutcome::Cancelled => ("CANCELLED", "■", Color::Yellow),
        CreationOutcome::Failed => ("FAILED", "!", Color::Red),
    };
    let header = vec![
        Line::from(vec![
            Span::styled(
                format!(" {symbol} {label} "),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(&state.project_name),
        ]),
        Line::from("Initial brief analysis · isolated Codex CLI"),
    ];
    frame.render_widget(
        Paragraph::new(header).block(Block::default().borders(Borders::ALL)),
        areas[0],
    );

    let retained = state.history.events();
    let visible_rows = usize::from(areas[1].height.saturating_sub(2)).max(1);
    // Offset from the newest suffix lets incoming events follow unless the user browses upward.
    let end = retained.len().saturating_sub(state.scroll_from_end);
    let start = end.saturating_sub(visible_rows);
    let items = retained[start..end].iter().map(|event| {
        let (prefix, style) = match event.kind {
            ActivityKind::Lifecycle => ("✓", Style::default().fg(Color::Green)),
            ActivityKind::Progress => ("•", Style::default().fg(Color::Cyan)),
            ActivityKind::Warning => ("!", Style::default().fg(Color::Yellow)),
        };
        ListItem::new(Line::from(vec![
            Span::styled(format!(" {prefix} "), style),
            Span::raw(&event.message),
        ]))
    });
    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(format!(" ACTIVITY · {} retained ", retained.len()))
                .borders(Borders::ALL),
        ),
        areas[1],
    );
    let help = match state.outcome {
        CreationOutcome::Running => "Esc cancel  ·  ↑↓ scroll",
        CreationOutcome::Completed | CreationOutcome::Cancelled | CreationOutcome::Failed => {
            "Enter/Esc close  ·  ↑↓ scroll"
        }
    };
    let detail = state
        .error
        .as_deref()
        .or(state.project_id.as_deref())
        .unwrap_or("Authoritative state is created only after validation succeeds");
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(help, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("  │  {detail}")),
        ]))
        .block(Block::default().borders(Borders::ALL)),
        areas[2],
    );
}

/// Renders overview, clarification, findings, and optional persisted activity panels.
fn render(frame: &mut Frame<'_>, snapshot: &ProjectSnapshot, activity: &[ActivityEvent]) {
    let constraints = if activity.is_empty() {
        vec![
            Constraint::Length(6),
            Constraint::Percentage(42),
            Constraint::Percentage(58),
        ]
    } else {
        vec![
            Constraint::Length(6),
            Constraint::Percentage(28),
            Constraint::Percentage(42),
            Constraint::Percentage(30),
        ]
    };
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.area());
    let overview = format!(
        "Project: {}\nStatus: {:?}\n{} findings · {} requirements · {} questions\nQ/Esc: quit safely",
        snapshot.project.name(),
        snapshot.project.status(),
        snapshot.findings.len(),
        snapshot.requirements.len(),
        snapshot.questions.len()
    );
    frame.render_widget(
        Paragraph::new(overview).block(
            Block::default()
                .title("PROJECT OVERVIEW")
                .borders(Borders::ALL),
        ),
        areas[0],
    );
    let questions = snapshot.questions.iter().map(|question| {
        ListItem::new(format!(
            "{} [{}] {} ({:?})",
            question.display_id,
            question.priority.score(),
            question.prompt,
            question.status
        ))
    });
    frame.render_widget(
        List::new(questions).block(
            Block::default()
                .title("CLARIFICATION")
                .borders(Borders::ALL),
        ),
        areas[1],
    );
    let findings = snapshot.findings.iter().map(|finding| {
        ListItem::new(format!(
            "{} [{:?}] {} ({:?})",
            finding.display_id(),
            finding.kind(),
            finding.statement(),
            finding.status()
        ))
    });
    frame.render_widget(
        List::new(findings).block(Block::default().title("FINDINGS").borders(Borders::ALL)),
        areas[2],
    );
    if !activity.is_empty() {
        frame.render_widget(
            List::new(activity.iter().map(activity_list_item)).block(
                Block::default()
                    .title("ANALYSIS ACTIVITY")
                    .borders(Borders::ALL),
            ),
            areas[3],
        );
    }
}

/// Formats one persisted operational event without provider-specific detail.
fn activity_list_item(event: &ActivityEvent) -> ListItem<'static> {
    let prefix = match event.kind {
        ActivityKind::Lifecycle => "✓",
        ActivityKind::Progress => "•",
        ActivityKind::Warning => "!",
    };
    ListItem::new(format!("{prefix} {}", event.message))
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::Duration;

    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use super::{CreationState, WorkspaceRuntimeConfig, render_creation};
    use crate::agents::{ActivityEvent, ActivityKind};

    /// Renders project identity, live status, activity, and keyboard help in a compact terminal.
    #[test]
    fn creation_view_renders_live_analysis_context() {
        let mut terminal = Terminal::new(TestBackend::new(64, 16))
            .expect("the deterministic terminal should initialize");
        let mut state = CreationState::new("Calm Fishing VR", 200);
        state.push_activity(ActivityEvent::now(
            1,
            ActivityKind::Progress,
            "Analyzing initial brief",
        ));

        terminal
            .draw(|frame| render_creation(frame, &state))
            .expect("the creation state should render");
        let rendered = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>();

        assert!(rendered.contains("Calm Fishing VR"));
        assert!(rendered.contains("ANALYZING"));
        assert!(rendered.contains("Analyzing initial brief"));
        assert!(rendered.contains("Esc cancel"));
    }

    /// Carries skill suppression into workbench-started Codex runs by default.
    #[test]
    fn workspace_runtime_disables_skills_by_default() {
        let config = WorkspaceRuntimeConfig::new(
            Path::new(".project-init"),
            None,
            Duration::from_secs(300),
            200,
            256,
        );

        assert!(config.disable_skills);
    }
}
