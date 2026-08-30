//! Command-line entry point for Project Init.

use std::fs;
use std::io::{self, IsTerminal};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use project_init::agents::{
    ActivityEvent, ActivityKind, AgentClient, AnalysisRequest, CancellationToken, CodexCliClient,
    CodexCliConfig, resolve_codex_executable,
};
use project_init::documents::PackageRenderer;
use project_init::domain::{ApprovalPolicy, EvidenceReliability, ProjectId, WorkflowStep};
use project_init::storage::SqliteStore;
use project_init::tui::CreationRunResult;
use project_init::workflow::{
    ProjectService, WorkflowRunner, requires_documentation_client, validate_snapshot,
};
use tokio::sync::{mpsc, oneshot};

const ANALYSIS_TIMEOUT: Duration = Duration::from_secs(5 * 60);
const ACTIVITY_HISTORY_CAPACITY: usize = 200;
const ACTIVITY_CHANNEL_CAPACITY: usize = 256;

/// Parses global storage configuration and one project workflow command.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Directory containing the authoritative database and generated packages.
    #[arg(long, default_value = ".project-init")]
    data_dir: PathBuf,
    /// Enable configured Codex skills for this invocation.
    #[arg(long, global = true, conflicts_with = "no_skills")]
    skills: bool,
    /// Retains the former explicit spelling for the now-default skill-free mode.
    #[arg(long, global = true, hide = true, conflicts_with = "skills")]
    no_skills: bool,
    /// Project workflow operation to execute.
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    /// Resolves the invocation policy while preserving the former no-skills spelling.
    const fn skills_disabled(&self) -> bool {
        self.no_skills || !self.skills
    }
}

/// Defines the stable user-facing project workflow commands.
#[derive(Debug, Subcommand)]
enum Command {
    /// Create and conservatively analyze a project brief.
    New {
        #[arg(long)]
        brief: PathBuf,
        #[arg(long)]
        name: Option<String>,
        /// Use the deterministic local analyzer instead of Codex CLI.
        #[arg(long)]
        offline: bool,
        /// Continue through generation and validation after creation.
        #[arg(long)]
        run: bool,
        /// Research and answer blocking questions without opening interactive terminal UI.
        #[arg(long, requires = "run", conflicts_with = "offline")]
        auto_answer: bool,
        /// Selects which proposed decisions pause this run.
        #[arg(long, default_value = "autonomous")]
        approval: ApprovalPolicy,
    },
    /// Create or resume a project and execute until completion or user input is required.
    Run {
        project_id: Option<String>,
        #[arg(long)]
        brief: Option<PathBuf>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        offline: bool,
        /// Research and answer blocking questions without opening interactive terminal UI.
        #[arg(long, conflicts_with = "offline")]
        auto_answer: bool,
        #[arg(long)]
        approval: Option<ApprovalPolicy>,
    },
    /// Execute at most one deterministic workflow mutation.
    Step {
        project_id: String,
        #[arg(long)]
        offline: bool,
        #[arg(long)]
        json: bool,
    },
    /// Report the deterministic next workflow action without mutation.
    Status {
        project_id: String,
        #[arg(long)]
        json: bool,
    },
    /// Preserve a manually edited expected artifact as an authoritative override.
    Override {
        project_id: String,
        relative_path: String,
    },
    /// Return a protected artifact to generated control without deleting history.
    RemoveOverride {
        project_id: String,
        relative_path: String,
    },
    /// Store an externally attributable claim separately from conclusions.
    AddEvidence {
        project_id: String,
        #[arg(long)]
        claim: String,
        #[arg(long)]
        source: String,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "medium")]
        reliability: EvidenceReliability,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Store a proposed project decision for policy evaluation.
    ProposeDecision {
        project_id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        statement: String,
        #[arg(long)]
        rationale: String,
        #[arg(long)]
        needs_confirmation: bool,
    },
    /// Record explicit approval for a proposed decision.
    Approve {
        decision_id: String,
        #[arg(long)]
        reason: String,
    },
    /// Record explicit rejection while preserving decision history.
    Reject {
        decision_id: String,
        #[arg(long)]
        reason: String,
    },
    /// List resumable projects.
    List,
    /// Inspect a project as JSON or in the interactive TUI.
    Open { project_id: String },
    /// Print the authoritative project snapshot as JSON.
    Inspect { project_id: String },
    /// Record and reconcile a first-class answer.
    Answer {
        question_id: String,
        answer: String,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Render the traceable Markdown package.
    Generate {
        project_id: String,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Validate an already rendered package.
    Validate {
        project_id: String,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Render a portable package to an explicit directory.
    Export { project_id: String, output: PathBuf },
}

/// Parses arguments and returns contextual failures without panicking.
#[tokio::main]
async fn main() -> Result<()> {
    run(Cli::parse()).await
}

/// Dispatches one command against the configured authoritative database.
async fn run(cli: Cli) -> Result<()> {
    fs::create_dir_all(&cli.data_dir)
        .with_context(|| format!("failed to create {}", cli.data_dir.display()))?;
    let store = SqliteStore::open(&cli.data_dir.join("project-init.sqlite3"))?;
    let no_skills = cli.skills_disabled();
    match cli.command {
        Command::New {
            brief,
            name,
            offline,
            run,
            auto_answer,
            approval,
        } => {
            let project_id = create_project(
                store,
                &cli.data_dir,
                &brief,
                name.as_deref(),
                offline,
                no_skills,
                !auto_answer,
            )
            .await?;
            if run {
                let reopened = SqliteStore::open(&cli.data_dir.join("project-init.sqlite3"))?;
                drive_workflow(
                    reopened,
                    &cli.data_dir,
                    &project_id,
                    Some(approval),
                    offline,
                    no_skills,
                    auto_answer,
                )
                .await?;
            }
            Ok(())
        }
        Command::Run {
            project_id,
            brief,
            name,
            offline,
            auto_answer,
            approval,
        } => {
            let project_id = match (project_id, brief) {
                (Some(project_id), None) => ProjectId::from_cli(&project_id),
                (None, Some(brief)) => {
                    create_project(
                        store,
                        &cli.data_dir,
                        &brief,
                        name.as_deref(),
                        offline,
                        no_skills,
                        !auto_answer,
                    )
                    .await?
                }
                (Some(_), Some(_)) => {
                    anyhow::bail!("pass either a project id or --brief, not both")
                }
                (None, None) => anyhow::bail!("run requires a project id or --brief"),
            };
            let reopened = SqliteStore::open(&cli.data_dir.join("project-init.sqlite3"))?;
            drive_workflow(
                reopened,
                &cli.data_dir,
                &project_id,
                approval,
                offline,
                no_skills,
                auto_answer,
            )
            .await
        }
        Command::Step {
            project_id,
            offline,
            json,
        } => {
            execute_workflow_step(
                store,
                &cli.data_dir,
                &ProjectId::from_cli(&project_id),
                json,
                offline,
                no_skills,
            )
            .await
        }
        Command::Status { project_id, json } => report_workflow_status(
            store,
            &cli.data_dir,
            &ProjectId::from_cli(&project_id),
            json,
        ),
        Command::Override {
            project_id,
            relative_path,
        } => register_document_override(
            store,
            &cli.data_dir,
            &ProjectId::from_cli(&project_id),
            &relative_path,
        ),
        Command::RemoveOverride {
            project_id,
            relative_path,
        } => remove_document_override(store, &ProjectId::from_cli(&project_id), &relative_path),
        Command::AddEvidence {
            project_id,
            claim,
            source,
            title,
            reliability,
            notes,
        } => add_evidence(
            store,
            &ProjectId::from_cli(&project_id),
            &claim,
            &source,
            &title,
            reliability,
            notes.as_deref(),
        ),
        Command::ProposeDecision {
            project_id,
            title,
            statement,
            rationale,
            needs_confirmation,
        } => propose_decision(
            store,
            &ProjectId::from_cli(&project_id),
            &title,
            &statement,
            &rationale,
            needs_confirmation,
        ),
        Command::Approve {
            decision_id,
            reason,
        } => resolve_decision(store, &decision_id, &reason, true),
        Command::Reject {
            decision_id,
            reason,
        } => resolve_decision(store, &decision_id, &reason, false),
        Command::List => list_projects(store),
        Command::Open { project_id } => open_project(store, &cli.data_dir, &project_id, no_skills),
        Command::Inspect { project_id } => inspect_project(store, &project_id),
        Command::Answer {
            question_id,
            answer,
            notes,
        } => answer_question(store, &question_id, &answer, notes.as_deref()),
        Command::Generate { project_id, output } => {
            render_project(store, &cli.data_dir, &project_id, output.as_deref())
        }
        Command::Validate { project_id, output } => {
            validate_project(store, &cli.data_dir, &project_id, output.as_deref())
        }
        Command::Export { project_id, output } => {
            render_project(store, &cli.data_dir, &project_id, Some(&output))
        }
    }
}

/// Reconciles one user-authoritative answer and prints its stable identifier.
fn answer_question(
    store: SqliteStore,
    question_id: &str,
    answer: &str,
    notes: Option<&str>,
) -> Result<()> {
    let stored = store;
    let mut service = ProjectService::new(stored);
    service.answer_question(question_id, answer, notes)?;
    println!("answer recorded for {question_id}");
    Ok(())
}

/// Creates a project from a UTF-8 brief and prints the durable resume identity.
async fn create_project(
    store: SqliteStore,
    data_dir: &Path,
    brief_path: &Path,
    name: Option<&str>,
    offline: bool,
    no_skills: bool,
    allow_interactive_ui: bool,
) -> Result<ProjectId> {
    let brief = fs::read_to_string(brief_path)
        .with_context(|| format!("failed to read {}", brief_path.display()))?;
    let fallback = brief_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Untitled Project");
    let project_name = name.unwrap_or(fallback);
    if offline {
        return create_offline_project(
            store,
            data_dir,
            project_name,
            &brief,
            no_skills,
            allow_interactive_ui,
        );
    }
    let request = AnalysisRequest::new(project_name, &brief)?;
    let executable = resolve_codex_executable(std::env::var_os("CODEX_BIN"))?;
    let client = CodexCliClient::new(CodexCliConfig {
        executable,
        working_directory: std::env::current_dir()
            .context("failed to resolve working directory")?,
        timeout: ANALYSIS_TIMEOUT,
        history_capacity: ACTIVITY_HISTORY_CAPACITY,
    })
    .with_skills_disabled(no_skills);
    let (activity_sender, activity_receiver) = mpsc::channel(ACTIVITY_CHANNEL_CAPACITY);
    let (result_sender, result_receiver) = oneshot::channel();
    let cancellation = CancellationToken::new();
    let task_cancellation = cancellation.clone();
    let task = tokio::spawn(async move {
        let result = client
            .analyze(request, activity_sender, task_cancellation)
            .await;
        let _ = result_sender.send(result);
    });
    let interactive =
        allow_interactive_ui && io::stdout().is_terminal() && io::stdin().is_terminal();
    let mut execution = if interactive {
        match project_init::tui::run_creation(
            project_name,
            activity_receiver,
            result_receiver,
            cancellation,
            ACTIVITY_HISTORY_CAPACITY,
        )? {
            CreationRunResult::Completed(execution) => execution,
            CreationRunResult::Cancelled => {
                task.await.context("Codex cancellation task failed")?;
                anyhow::bail!("Codex analysis was cancelled");
            }
            CreationRunResult::Failed(message) => {
                task.await.context("Codex analysis task failed")?;
                anyhow::bail!(message);
            }
        }
    } else {
        result_receiver
            .await
            .context("Codex analysis task ended without a result")??
    };
    task.await.context("Codex analysis task failed")?;
    append_commit_activity(&mut execution)?;
    let mut service = ProjectService::new(store);
    let project = service.initialize_from_agent_execution(project_name, &brief, execution)?;
    let project_id = project.id().clone();
    report_created_project(&mut service, data_dir, &project, interactive, no_skills)?;
    Ok(project_id)
}

/// Runs the preserved deterministic analyzer only when explicitly requested.
fn create_offline_project(
    store: SqliteStore,
    data_dir: &Path,
    name: &str,
    brief: &str,
    no_skills: bool,
    allow_interactive_ui: bool,
) -> Result<ProjectId> {
    let mut service = ProjectService::new(store);
    let project = service.initialize_project(name, brief)?;
    let interactive =
        allow_interactive_ui && io::stdout().is_terminal() && io::stdin().is_terminal();
    let project_id = project.id().clone();
    report_created_project(&mut service, data_dir, &project, interactive, no_skills)?;
    Ok(project_id)
}

/// Adds the final pre-commit milestone to the history persisted by the transaction.
fn append_commit_activity(execution: &mut project_init::agents::AgentExecution) -> Result<()> {
    let next = execution
        .activity
        .last()
        .map_or(1, |event| event.sequence.saturating_add(1));
    if execution
        .activity
        .last()
        .is_some_and(|event| next <= event.sequence)
    {
        anyhow::bail!("Codex activity sequence is exhausted");
    }
    execution.activity.push(ActivityEvent::now(
        next,
        ActivityKind::Lifecycle,
        "Committing validated project",
    ));
    Ok(())
}

/// Opens the durable overview interactively and prints a resumable identity afterward.
fn report_created_project(
    service: &mut ProjectService,
    data_dir: &Path,
    project: &project_init::domain::Project,
    interactive: bool,
    no_skills: bool,
) -> Result<()> {
    if interactive {
        project_init::tui::run_workspace(
            service,
            project.id(),
            workspace_runtime_config(data_dir, no_skills),
        )?;
    }
    let snapshot = service.inspect_project(project.id())?;
    print!("{}", format_project_summary(&snapshot));
    Ok(())
}

/// Formats the refreshed project identity, lifecycle, and clarification queue for CLI output.
fn format_project_summary(snapshot: &project_init::domain::ProjectSnapshot) -> String {
    let mut output = format!(
        "{}\t{}\t{:?}\n",
        snapshot.project.id(),
        snapshot.project.name(),
        snapshot.project.status()
    );
    for question in &snapshot.questions {
        output.push_str(&format!(
            "{}\tpriority={}\t{}\n",
            question.display_id,
            question.priority.score(),
            question.prompt
        ));
    }
    output
}

/// Prints every resumable project without loading a TUI.
fn list_projects(store: SqliteStore) -> Result<()> {
    for project in store.list_projects()? {
        println!(
            "{}\t{}\t{:?}",
            project.id(),
            project.name(),
            project.status()
        );
    }
    Ok(())
}

/// Opens the interactive project inspector when a terminal is available.
fn open_project(
    store: SqliteStore,
    data_dir: &Path,
    project_id: &str,
    no_skills: bool,
) -> Result<()> {
    let project_id = project_init::domain::ProjectId::from_cli(project_id);
    if io::stdout().is_terminal() && io::stdin().is_terminal() {
        let mut service = ProjectService::new(store);
        project_init::tui::run_workspace(
            &mut service,
            &project_id,
            workspace_runtime_config(data_dir, no_skills),
        )?;
    } else {
        let snapshot = store.project_snapshot(&project_id)?;
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    }
    Ok(())
}

/// Creates lazy online workbench configuration without requiring Codex until `/resume`.
fn workspace_runtime_config(
    data_dir: &Path,
    no_skills: bool,
) -> project_init::tui::WorkspaceRuntimeConfig {
    project_init::tui::WorkspaceRuntimeConfig::new(
        data_dir,
        std::env::var_os("CODEX_BIN"),
        ANALYSIS_TIMEOUT,
        ACTIVITY_HISTORY_CAPACITY,
        ACTIVITY_CHANNEL_CAPACITY,
    )
    .with_skills_disabled(no_skills)
}

/// Prints the complete authoritative inspection snapshot as formatted JSON.
fn inspect_project(store: SqliteStore, project_id: &str) -> Result<()> {
    let snapshot =
        store.project_snapshot(&project_init::domain::ProjectId::from_cli(project_id))?;
    println!("{}", serde_json::to_string_pretty(&snapshot)?);
    Ok(())
}

/// Renders a package beneath an explicit directory or the project data root.
fn render_project(
    store: SqliteStore,
    data_dir: &Path,
    project_id: &str,
    output: Option<&Path>,
) -> Result<()> {
    let snapshot =
        store.project_snapshot(&project_init::domain::ProjectId::from_cli(project_id))?;
    let default_output = data_dir.join("projects").join(project_id).join("Docs");
    for path in PackageRenderer::render(&snapshot, output.unwrap_or(&default_output))? {
        println!("{}", path.display());
    }
    Ok(())
}

/// Computes and prints deterministic package validation as JSON.
fn validate_project(
    store: SqliteStore,
    data_dir: &Path,
    project_id: &str,
    output: Option<&Path>,
) -> Result<()> {
    let snapshot =
        store.project_snapshot(&project_init::domain::ProjectId::from_cli(project_id))?;
    let default_output = data_dir.join("projects").join(project_id).join("Docs");
    let report = validate_snapshot(&snapshot, output.unwrap_or(&default_output));
    println!("{}", serde_json::to_string_pretty(&report)?);
    if !report.passed {
        anyhow::bail!(
            "validation found {} blocking issue(s)",
            report.findings.len()
        );
    }
    Ok(())
}

/// Runs deterministic steps until completion or an explicit pause boundary.
async fn drive_workflow(
    store: SqliteStore,
    data_dir: &Path,
    project_id: &ProjectId,
    approval: Option<ApprovalPolicy>,
    offline: bool,
    no_skills: bool,
    auto_answer: bool,
) -> Result<()> {
    let mut service = ProjectService::new(store);
    match service.active_run(project_id)? {
        Some(active) => {
            if let Some(requested) = approval
                && requested != active.policy
            {
                service.start_run(project_id, requested)?;
            }
        }
        None => {
            service.start_run(project_id, approval.unwrap_or(ApprovalPolicy::Autonomous))?;
        }
    }
    let output = workflow_output(data_dir, project_id);
    if should_show_auto_answer_tui(
        auto_answer,
        io::stdin().is_terminal(),
        io::stdout().is_terminal(),
    ) {
        project_init::tui::run_workspace_auto_answer(
            &mut service,
            project_id,
            workspace_runtime_config(data_dir, no_skills),
        )?;
        let status = service.workflow_status(project_id, &output)?;
        println!("{}", serde_json::to_string_pretty(&status)?);
        return Ok(());
    }
    let status = service.workflow_status(project_id, &output)?;
    let cancellation = CancellationToken::new();
    let outcome = if offline || (!auto_answer && !requires_documentation_client(&status)) {
        let mut runner = WorkflowRunner::offline(service, &output);
        runner
            .run_until_pause(project_id, None, None, cancellation)
            .await?
    } else {
        let executable = resolve_codex_executable(std::env::var_os("CODEX_BIN"))?;
        let client = CodexCliClient::new(CodexCliConfig {
            executable,
            working_directory: data_dir.to_path_buf(),
            timeout: ANALYSIS_TIMEOUT,
            history_capacity: ACTIVITY_HISTORY_CAPACITY,
        })
        .with_skills_disabled(no_skills);
        let mut runner = WorkflowRunner::online(service, &output, &client);
        if auto_answer {
            runner = runner.with_auto_answer_client(Arc::new(client.clone()));
        }
        runner
            .run_until_pause(project_id, None, None, cancellation)
            .await?
    };
    println!("{}", serde_json::to_string_pretty(&outcome.status)?);
    Ok(())
}

/// Selects the progress TUI only for explicit automatic answering on attached terminals.
const fn should_show_auto_answer_tui(
    auto_answer: bool,
    stdin_is_terminal: bool,
    stdout_is_terminal: bool,
) -> bool {
    auto_answer && stdin_is_terminal && stdout_is_terminal
}

/// Executes one mutation and writes either stable JSON or concise human output.
async fn execute_workflow_step(
    store: SqliteStore,
    data_dir: &Path,
    project_id: &ProjectId,
    json: bool,
    offline: bool,
    no_skills: bool,
) -> Result<()> {
    let mut service = ProjectService::new(store);
    let status = execute_selected_step(
        &mut service,
        project_id,
        &workflow_output(data_dir, project_id),
        offline,
        no_skills,
    )
    .await?;
    print_workflow_status(&status, json)
}

/// Uses Codex for generation when enabled and deterministic operations for every other step.
async fn execute_selected_step(
    service: &mut ProjectService,
    project_id: &ProjectId,
    output: &Path,
    offline: bool,
    no_skills: bool,
) -> Result<project_init::domain::WorkflowStatus> {
    let status = service.workflow_status(project_id, output)?;
    if offline || status.step != WorkflowStep::GeneratePackage {
        return service
            .execute_offline_step(project_id, output)
            .map_err(Into::into);
    }
    let snapshot = service.inspect_project(project_id)?;
    let staging = tempfile::tempdir().context("failed to create documentation staging")?;
    let executable = resolve_codex_executable(std::env::var_os("CODEX_BIN"))?;
    let client = CodexCliClient::new(CodexCliConfig {
        executable,
        working_directory: staging.path().to_path_buf(),
        timeout: ANALYSIS_TIMEOUT,
        history_capacity: ACTIVITY_HISTORY_CAPACITY,
    })
    .with_skills_disabled(no_skills);
    client
        .generate_documentation(
            &serde_json::to_string(&snapshot)?,
            &PackageRenderer::required_relative_paths(&snapshot),
            staging.path(),
            None,
            CancellationToken::new(),
        )
        .await?;
    service
        .adopt_generated_package(project_id, staging.path(), output)
        .map_err(Into::into)
}

/// Reports one observational status without starting or changing a run.
fn report_workflow_status(
    store: SqliteStore,
    data_dir: &Path,
    project_id: &ProjectId,
    json: bool,
) -> Result<()> {
    let service = ProjectService::new(store);
    let status = service.workflow_status(project_id, &workflow_output(data_dir, project_id))?;
    print_workflow_status(&status, json)
}

/// Registers the default-package artifact currently present on disk as an override.
fn register_document_override(
    store: SqliteStore,
    data_dir: &Path,
    project_id: &ProjectId,
    relative_path: &str,
) -> Result<()> {
    let mut service = ProjectService::new(store);
    service.register_override(
        project_id,
        &workflow_output(data_dir, project_id),
        relative_path,
    )?;
    println!("override registered: {relative_path}");
    Ok(())
}

/// Releases a protected artifact while retaining its immutable revision history.
fn remove_document_override(
    store: SqliteStore,
    project_id: &ProjectId,
    relative_path: &str,
) -> Result<()> {
    let mut service = ProjectService::new(store);
    service.remove_override(project_id, relative_path)?;
    println!("override removed: {relative_path}");
    Ok(())
}

/// Persists one command-line evidence record and prints its stable identifier.
#[allow(clippy::too_many_arguments)]
fn add_evidence(
    store: SqliteStore,
    project_id: &ProjectId,
    claim: &str,
    source: &str,
    title: &str,
    reliability: EvidenceReliability,
    notes: Option<&str>,
) -> Result<()> {
    let mut service = ProjectService::new(store);
    let evidence = service.add_evidence(project_id, claim, source, title, reliability, notes)?;
    println!("{}", evidence.display_id);
    Ok(())
}

/// Persists one proposed decision and prints both human and durable identities.
fn propose_decision(
    store: SqliteStore,
    project_id: &ProjectId,
    title: &str,
    statement: &str,
    rationale: &str,
    needs_confirmation: bool,
) -> Result<()> {
    let mut service = ProjectService::new(store);
    let decision =
        service.propose_decision(project_id, title, statement, rationale, needs_confirmation)?;
    println!("{}\t{}", decision.id, decision.display_id);
    Ok(())
}

/// Resolves one decision through an immutable authority record.
fn resolve_decision(
    store: SqliteStore,
    decision_id: &str,
    reason: &str,
    approved: bool,
) -> Result<()> {
    let mut service = ProjectService::new(store);
    if approved {
        service.approve_decision(decision_id, reason)?;
    } else {
        service.reject_decision(decision_id, reason)?;
    }
    println!(
        "decision {}",
        if approved { "approved" } else { "rejected" }
    );
    Ok(())
}

/// Returns the authoritative default package directory for one project.
fn workflow_output(data_dir: &Path, project_id: &ProjectId) -> PathBuf {
    data_dir
        .join("projects")
        .join(project_id.as_str())
        .join("Docs")
}

/// Formats workflow state for either scripts or terminal users.
fn print_workflow_status(status: &project_init::domain::WorkflowStatus, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(status)?);
    } else {
        println!(
            "{}\t{}\t{:?}",
            status.run.id, status.project_status, status.step
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Cli, Command, format_project_summary, should_show_auto_answer_tui};
    use clap::Parser;
    use project_init::domain::ProjectStatus;
    use project_init::storage::SqliteStore;
    use project_init::workflow::ProjectService;

    /// Uses Codex analysis by default when the offline override is absent.
    #[test]
    fn new_defaults_to_codex_analysis() {
        let cli = Cli::try_parse_from(["project-init", "new", "--brief", "brief.md"])
            .expect("the default new command should parse");

        assert!(matches!(cli.command, Command::New { offline: false, .. }));
    }

    /// Selects deterministic analysis only when the user explicitly passes offline.
    #[test]
    fn offline_flag_selects_deterministic_analysis() {
        let cli = Cli::try_parse_from(["project-init", "new", "--brief", "brief.md", "--offline"])
            .expect("the offline new command should parse");

        assert!(matches!(cli.command, Command::New { offline: true, .. }));
    }

    /// Accepts the compatibility spelling for the default skill-free mode globally.
    #[test]
    fn no_skills_remains_a_global_compatibility_option() {
        let before =
            Cli::try_parse_from(["project-init", "--no-skills", "new", "--brief", "brief.md"])
                .expect("the global option should parse before the subcommand");
        let after =
            Cli::try_parse_from(["project-init", "new", "--brief", "brief.md", "--no-skills"])
                .expect("the global option should parse after the subcommand");

        assert!(before.no_skills);
        assert!(after.no_skills);
        assert!(before.skills_disabled());
        assert!(after.skills_disabled());
    }

    /// Suppresses configured Codex skills when no skill option is provided.
    #[test]
    fn skills_are_disabled_by_default() {
        let cli = Cli::try_parse_from(["project-init", "new", "--brief", "brief.md"])
            .expect("the default command should parse");

        assert!(cli.skills_disabled());
    }

    /// Accepts explicit skill enablement on either side of the subcommand boundary.
    #[test]
    fn skills_is_a_global_opt_in() {
        let before =
            Cli::try_parse_from(["project-init", "--skills", "new", "--brief", "brief.md"])
                .expect("the global option should parse before the subcommand");
        let after = Cli::try_parse_from(["project-init", "new", "--brief", "brief.md", "--skills"])
            .expect("the global option should parse after the subcommand");

        assert!(!before.skills_disabled());
        assert!(!after.skills_disabled());
    }

    /// Formats the refreshed authoritative lifecycle status after interactive reconciliation.
    #[test]
    fn project_summary_uses_the_refreshed_snapshot_status() {
        let store = SqliteStore::open_in_memory().expect("the database should open");
        let mut service = ProjectService::new(store);
        let project = service
            .initialize_from_analysis_json(
                "Summary project",
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
            .expect("the final question should reconcile");
        let refreshed = service
            .inspect_project(project.id())
            .expect("the refreshed snapshot should load");

        assert_eq!(refreshed.project.status(), ProjectStatus::Planning);
        assert!(format_project_summary(&refreshed).contains("Planning"));
    }

    /// Parses direct brief execution with an explicit run-scoped approval policy.
    #[test]
    fn run_accepts_a_new_brief_and_policy() {
        let cli = Cli::try_parse_from([
            "project-init",
            "run",
            "--brief",
            "brief.md",
            "--name",
            "Direct run",
            "--approval",
            "consequential",
        ])
        .expect("the direct run command should parse");

        assert!(matches!(
            cli.command,
            Command::Run {
                project_id: None,
                ..
            }
        ));
    }

    /// Enables cited automatic clarification only when the caller opts in explicitly.
    #[test]
    fn run_accepts_opt_in_automatic_research_answers() {
        let cli = Cli::try_parse_from(["project-init", "run", "project-1", "--auto-answer"])
            .expect("the automatic answer option should parse");

        assert!(matches!(
            cli.command,
            Command::Run {
                auto_answer: true,
                ..
            }
        ));
    }

    /// Rejects a mode combination that cannot perform the requested external research.
    #[test]
    fn automatic_research_answers_conflict_with_offline_execution() {
        let error = Cli::try_parse_from([
            "project-init",
            "run",
            "project-1",
            "--auto-answer",
            "--offline",
        ])
        .expect_err("offline execution cannot research answers");

        assert!(error.to_string().contains("cannot be used with"));
    }

    /// Keeps create-and-run automation headless under the same explicit option.
    #[test]
    fn new_run_accepts_opt_in_automatic_research_answers() {
        let cli = Cli::try_parse_from([
            "project-init",
            "new",
            "--brief",
            "brief.md",
            "--run",
            "--auto-answer",
        ])
        .expect("create-and-run should accept automatic answers");

        assert!(matches!(
            cli.command,
            Command::New {
                run: true,
                auto_answer: true,
                ..
            }
        ));
    }

    /// Opens automatic-answer progress only when both terminal streams are interactive.
    #[test]
    fn auto_answer_tui_requires_an_interactive_input_and_output() {
        assert!(should_show_auto_answer_tui(true, true, true));
        assert!(!should_show_auto_answer_tui(true, false, true));
        assert!(!should_show_auto_answer_tui(true, true, false));
        assert!(!should_show_auto_answer_tui(false, true, true));
    }

    /// Preserves explicit creation while allowing it to continue into orchestration.
    #[test]
    fn new_accepts_run_and_approval_options() {
        let cli = Cli::try_parse_from([
            "project-init",
            "new",
            "--brief",
            "brief.md",
            "--run",
            "--approval",
            "strict",
        ])
        .expect("the create-and-run command should parse");

        assert!(matches!(cli.command, Command::New { run: true, .. }));
    }

    /// Parses machine-oriented step and status commands without a terminal dependency.
    #[test]
    fn automation_commands_accept_json_output() {
        let step = Cli::try_parse_from(["project-init", "step", "project-1", "--json"])
            .expect("the step command should parse");
        let status = Cli::try_parse_from(["project-init", "status", "project-1", "--json"])
            .expect("the status command should parse");

        assert!(matches!(step.command, Command::Step { json: true, .. }));
        assert!(matches!(status.command, Command::Status { json: true, .. }));
    }
}
