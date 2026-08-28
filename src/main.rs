//! Command-line entry point for Project Init.

use std::fs;
use std::io::{self, IsTerminal};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use project_init::documents::PackageRenderer;
use project_init::storage::SqliteStore;
use project_init::workflow::{ProjectService, validate_snapshot};

/// Parses global storage configuration and one project workflow command.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Directory containing the authoritative database and generated packages.
    #[arg(long, default_value = ".project-init")]
    data_dir: PathBuf,
    /// Project workflow operation to execute.
    #[command(subcommand)]
    command: Command,
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
fn main() -> Result<()> {
    run(Cli::parse())
}

/// Dispatches one command against the configured authoritative database.
fn run(cli: Cli) -> Result<()> {
    fs::create_dir_all(&cli.data_dir)
        .with_context(|| format!("failed to create {}", cli.data_dir.display()))?;
    let store = SqliteStore::open(&cli.data_dir.join("project-init.sqlite3"))?;
    match cli.command {
        Command::New { brief, name } => create_project(store, &brief, name.as_deref()),
        Command::List => list_projects(store),
        Command::Open { project_id } => open_project(store, &project_id),
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
fn create_project(store: SqliteStore, brief_path: &Path, name: Option<&str>) -> Result<()> {
    let brief = fs::read_to_string(brief_path)
        .with_context(|| format!("failed to read {}", brief_path.display()))?;
    let fallback = brief_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Untitled Project");
    let mut service = ProjectService::new(store);
    let project = service.initialize_project(name.unwrap_or(fallback), &brief)?;
    let snapshot = service.inspect_project(project.id())?;
    println!(
        "{}\t{}\t{:?}",
        project.id(),
        project.name(),
        project.status()
    );
    for question in snapshot.questions {
        println!(
            "{}\tpriority={}\t{}",
            question.display_id,
            question.priority.score(),
            question.prompt
        );
    }
    Ok(())
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
fn open_project(store: SqliteStore, project_id: &str) -> Result<()> {
    let snapshot =
        store.project_snapshot(&project_init::domain::ProjectId::from_cli(project_id))?;
    if io::stdout().is_terminal() && io::stdin().is_terminal() {
        project_init::tui::run(&snapshot)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    }
    Ok(())
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
