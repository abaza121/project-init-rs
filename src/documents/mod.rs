//! Deterministic Markdown rendering from an authoritative project snapshot.

use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::domain::{FindingKind, ProjectSnapshot, QuestionStatus};

/// Describes a filesystem failure while rendering a fixed, safe artifact path.
#[derive(Debug, Error)]
pub enum DocumentError {
    /// Wraps an operating-system error with its original context.
    #[error("document rendering failed: {0}")]
    Io(#[from] std::io::Error),
}

/// Renders the known project-initiation package without consulting a live model.
pub struct PackageRenderer;

impl PackageRenderer {
    /// Writes the complete deterministic package beneath an explicit output directory.
    pub fn render(
        snapshot: &ProjectSnapshot,
        output: &Path,
    ) -> Result<Vec<PathBuf>, DocumentError> {
        fs::create_dir_all(output)?;
        let artifacts = [
            ("README.md", render_index(snapshot)),
            ("Requirements.md", render_requirements(snapshot)),
            ("Assumptions.md", render_assumptions(snapshot)),
            ("OpenQuestions.md", render_questions(snapshot)),
            ("Traceability.md", render_traceability(snapshot)),
            ("ValidationReport.md", render_validation_note(snapshot)),
        ];
        let mut paths = Vec::with_capacity(artifacts.len());
        for (relative, content) in artifacts {
            let path = output.join(relative);
            fs::write(&path, content)?;
            paths.push(path);
        }
        Ok(paths)
    }
}

/// Renders the package entry point and fixed artifact links.
fn render_index(snapshot: &ProjectSnapshot) -> String {
    format!(
        "# {}\n\n{}\n\n## Project foundation\n\n- [Requirements](Requirements.md)\n- [Assumptions](Assumptions.md)\n- [Open questions](OpenQuestions.md)\n- [Traceability](Traceability.md)\n- [Validation report](ValidationReport.md)\n\nStatus: `{:?}`\n",
        snapshot.project.name(),
        snapshot.project.brief(),
        snapshot.project.status()
    )
}

/// Renders first-class requirements with provenance and acceptance criteria.
fn render_requirements(snapshot: &ProjectSnapshot) -> String {
    let mut output = "# Requirements\n\n".to_owned();
    if snapshot.requirements.is_empty() {
        output.push_str("No reconciled requirements have been recorded yet.\n");
    }
    for requirement in &snapshot.requirements {
        output.push_str(&format!(
            "## {}\n\n{}\n\n- Source: `{:?}` / `{}`\n- Priority: `{}`\n- Status: `{:?}`\n- Acceptance: {}\n\n",
            requirement.display_id,
            requirement.statement,
            requirement.source_type,
            requirement.source_reference,
            requirement.priority,
            requirement.status,
            requirement.acceptance_criteria
        ));
    }
    output
}

/// Renders active and historical assumptions without promoting them to facts.
fn render_assumptions(snapshot: &ProjectSnapshot) -> String {
    let mut output = "# Assumptions\n\n".to_owned();
    for finding in snapshot
        .findings
        .iter()
        .filter(|finding| finding.kind() == FindingKind::Assumption)
    {
        output.push_str(&format!(
            "- **{}** {} (`{:?}`)\n",
            finding.display_id(),
            finding.statement(),
            finding.status()
        ));
    }
    if !output.contains("**") {
        output.push_str("No assumptions are currently recorded.\n");
    }
    output
}

/// Renders every clarification with its priority and durable status.
fn render_questions(snapshot: &ProjectSnapshot) -> String {
    let mut output = "# Open Questions\n\n".to_owned();
    for question in &snapshot.questions {
        output.push_str(&format!(
            "- **{}** {} — score {}, status `{:?}`\n",
            question.display_id,
            question.prompt,
            question.priority.score(),
            question.status
        ));
    }
    if snapshot.questions.is_empty() {
        output.push_str("No clarification questions were generated.\n");
    } else if snapshot
        .questions
        .iter()
        .all(|question| question.status != QuestionStatus::Open)
    {
        output.push_str("\nAll generated clarification questions are resolved or deferred.\n");
    }
    output
}

/// Renders explicit graph edges rather than inferred semantic associations.
fn render_traceability(snapshot: &ProjectSnapshot) -> String {
    let mut output = "# Traceability\n\n".to_owned();
    for trace in &snapshot.traces {
        output.push_str(&format!(
            "- `{}/{}` --`{:?}`--> `{}/{}`\n",
            trace.source_type,
            trace.source_id,
            trace.relationship,
            trace.target_type,
            trace.target_id
        ));
    }
    if snapshot.traces.is_empty() {
        output.push_str("No explicit trace links have been recorded yet.\n");
    }
    output
}

/// Renders a reproducible validation instruction without fabricating a pass result.
fn render_validation_note(snapshot: &ProjectSnapshot) -> String {
    format!(
        "# Validation Report\n\nProject `{}` has {} findings, {} requirements, and {} trace links.\n\nRun `project-init validate {}` to compute the current deterministic result.\n",
        snapshot.project.id(),
        snapshot.findings.len(),
        snapshot.requirements.len(),
        snapshot.traces.len(),
        snapshot.project.id()
    )
}
