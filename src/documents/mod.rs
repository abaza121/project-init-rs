//! Deterministic Markdown rendering from an authoritative project snapshot.

use std::collections::HashSet;
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
        Self::render_preserving(snapshot, output, &HashSet::new())
    }

    /// Returns every required project-relative artifact path in dependency order.
    pub fn required_relative_paths(snapshot: &ProjectSnapshot) -> Vec<String> {
        vec![
            "README.md".to_owned(),
            "Requirements.md".to_owned(),
            "Assumptions.md".to_owned(),
            "OpenQuestions.md".to_owned(),
            "SWOT.md".to_owned(),
            "MissionVision.md".to_owned(),
            "VisualIdentity.md".to_owned(),
            "BrandPrompt.md".to_owned(),
            "TechnicalArchitecture.md".to_owned(),
            "Research-01-Audience.md".to_owned(),
            "Research-02-Experience.md".to_owned(),
            "Research-03-Market.md".to_owned(),
            "Research-04-Technology.md".to_owned(),
            "Research-05-Delivery.md".to_owned(),
            "Traceability.md".to_owned(),
            session_log_name(snapshot),
            "ValidationReport.md".to_owned(),
        ]
    }

    /// Returns the transitive fixed-graph dependants invalidated by one artifact change.
    pub fn dependent_relative_paths(snapshot: &ProjectSnapshot, changed: &str) -> Vec<String> {
        let session = session_log_name(snapshot);
        let strategic = [
            "SWOT.md",
            "MissionVision.md",
            "VisualIdentity.md",
            "BrandPrompt.md",
            "TechnicalArchitecture.md",
            "Research-01-Audience.md",
            "Research-02-Experience.md",
            "Research-03-Market.md",
            "Research-04-Technology.md",
            "Research-05-Delivery.md",
            "Traceability.md",
        ];
        let mut dependants = match changed {
            "Requirements.md" => strategic.iter().map(ToString::to_string).collect(),
            "SWOT.md" => ["MissionVision.md", "VisualIdentity.md", "BrandPrompt.md"]
                .iter()
                .map(ToString::to_string)
                .collect(),
            "MissionVision.md" => ["VisualIdentity.md", "BrandPrompt.md"]
                .iter()
                .map(ToString::to_string)
                .collect(),
            "VisualIdentity.md" => vec!["BrandPrompt.md".to_owned()],
            "Assumptions.md" | "OpenQuestions.md" => vec!["Traceability.md".to_owned()],
            _ => Vec::new(),
        };
        if changed != session && changed != "ValidationReport.md" {
            dependants.push(session);
        }
        if changed != "ValidationReport.md" {
            dependants.push("ValidationReport.md".to_owned());
        }
        dependants.push("README.md".to_owned());
        dependants.retain(|relative| relative != changed);
        dependants.sort();
        dependants.dedup();
        dependants
    }

    /// Writes generated artifacts while leaving registered manual paths byte-for-byte intact.
    pub fn render_preserving(
        snapshot: &ProjectSnapshot,
        output: &Path,
        protected: &HashSet<String>,
    ) -> Result<Vec<PathBuf>, DocumentError> {
        fs::create_dir_all(output)?;
        let artifacts = vec![
            ("README.md".to_owned(), render_index(snapshot)),
            ("Requirements.md".to_owned(), render_requirements(snapshot)),
            ("Assumptions.md".to_owned(), render_assumptions(snapshot)),
            ("OpenQuestions.md".to_owned(), render_questions(snapshot)),
            ("SWOT.md".to_owned(), render_swot(snapshot)),
            (
                "MissionVision.md".to_owned(),
                render_mission_vision(snapshot),
            ),
            (
                "VisualIdentity.md".to_owned(),
                render_visual_identity(snapshot),
            ),
            ("BrandPrompt.md".to_owned(), render_brand_prompt(snapshot)),
            (
                "TechnicalArchitecture.md".to_owned(),
                render_technical_architecture(snapshot),
            ),
            (
                "Research-01-Audience.md".to_owned(),
                render_research(snapshot, "Audience", "target users and their primary jobs"),
            ),
            (
                "Research-02-Experience.md".to_owned(),
                render_research(
                    snapshot,
                    "Experience",
                    "the intended user journey and outcomes",
                ),
            ),
            (
                "Research-03-Market.md".to_owned(),
                render_research(snapshot, "Market", "alternatives and differentiation"),
            ),
            (
                "Research-04-Technology.md".to_owned(),
                render_research(
                    snapshot,
                    "Technology",
                    "technical constraints and feasibility",
                ),
            ),
            (
                "Research-05-Delivery.md".to_owned(),
                render_research(
                    snapshot,
                    "Delivery",
                    "testing, rollout, and operational risk",
                ),
            ),
            ("Traceability.md".to_owned(), render_traceability(snapshot)),
            (session_log_name(snapshot), render_session_log(snapshot)),
            (
                "ValidationReport.md".to_owned(),
                render_validation_note(snapshot),
            ),
        ];
        let mut paths = Vec::with_capacity(artifacts.len());
        for (relative, content) in artifacts {
            let path = output.join(&relative);
            if !protected.contains(&relative) {
                fs::write(&path, content)?;
            }
            paths.push(path);
        }
        Ok(paths)
    }
}

/// Renders the package entry point and fixed artifact links.
fn render_index(snapshot: &ProjectSnapshot) -> String {
    format!(
        "# {}\n\n{}\n\n## Project foundation\n\n- [Requirements](Requirements.md)\n- [Assumptions](Assumptions.md)\n- [Open questions](OpenQuestions.md)\n- [SWOT](SWOT.md)\n- [Mission and vision](MissionVision.md)\n- [Visual identity](VisualIdentity.md)\n- [Brand prompts](BrandPrompt.md)\n- [Technical architecture](TechnicalArchitecture.md)\n- [Audience research](Research-01-Audience.md)\n- [Experience research](Research-02-Experience.md)\n- [Market research](Research-03-Market.md)\n- [Technology research](Research-04-Technology.md)\n- [Delivery research](Research-05-Delivery.md)\n- [Traceability](Traceability.md)\n- [Validation report](ValidationReport.md)\n\nStatus: `{:?}`\n",
        snapshot.project.name(),
        snapshot.project.brief(),
        snapshot.project.status()
    )
}

/// Renders a decision-oriented SWOT without inventing external facts.
fn render_swot(snapshot: &ProjectSnapshot) -> String {
    format!(
        "# SWOT\n\n## Strengths\n\n- The project has {} explicit requirement(s) with stored provenance.\n\n## Weaknesses\n\n- {} clarification question(s) remain visible in the authoritative record.\n\n## Opportunities\n\n- Validate the highest-impact assumptions before implementation investment.\n\n## Threats\n\n- Unverified external claims may change; consult the research documents before relying on them.\n",
        snapshot.requirements.len(),
        snapshot
            .questions
            .iter()
            .filter(|question| question.status == QuestionStatus::Open)
            .count()
    )
}

/// Renders a concise mission and vision derived only from the stored brief.
fn render_mission_vision(snapshot: &ProjectSnapshot) -> String {
    format!(
        "# Mission and Vision\n\n## Mission\n\nTurn the intent behind **{}** into a clear, testable project foundation.\n\n## Vision\n\n{}\n\n## Decision principles\n\n- Preserve user authority over consequential choices.\n- Prefer evidence and explicit assumptions over confident invention.\n- Keep scope traceable to recorded requirements.\n",
        snapshot.project.name(),
        snapshot.project.brief()
    )
}

/// Renders an intentionally provisional visual direction grounded in project intent.
fn render_visual_identity(snapshot: &ProjectSnapshot) -> String {
    format!(
        "# Visual Identity\n\n## Experience intent\n\nThe visual system for **{}** should make the product's stated purpose immediately legible.\n\n## Principles\n\n- Clarity before decoration.\n- Accessible contrast and typography.\n- Motion only when it communicates state.\n- Treat palette, typeface, and imagery choices as candidates until approved.\n",
        snapshot.project.name()
    )
}

/// Renders naming and asset prompts while retaining their proposal status.
fn render_brand_prompt(snapshot: &ProjectSnapshot) -> String {
    format!(
        "# Brand Prompt\n\n## Working name\n\n`{}` is the current project name supplied to the workflow. Alternative names remain proposals until explicitly approved.\n\n## Palette prompt\n\nCreate an accessible palette that expresses the mission in `MissionVision.md` and remains usable for text, controls, focus, warning, and success states.\n\n## Logo prompt\n\nCreate a simple, reproducible mark for **{}** that remains recognizable at small sizes and in one color.\n",
        snapshot.project.name(),
        snapshot.project.name()
    )
}

/// Renders a minimal architecture boundary without selecting unsupported infrastructure.
fn render_technical_architecture(snapshot: &ProjectSnapshot) -> String {
    let mut output = "# Technical Architecture\n\n## System boundary\n\nArchitecture must implement the recorded requirements without adding infrastructure unsupported by project constraints.\n\n## Requirement drivers\n\n".to_owned();
    for requirement in &snapshot.requirements {
        output.push_str(&format!(
            "- **{}** {}\n",
            requirement.display_id, requirement.statement
        ));
    }
    if snapshot.requirements.is_empty() {
        output.push_str("- No reconciled implementation requirements are recorded yet.\n");
    }
    output.push_str("\n## Quality strategy\n\n- Validate inputs at system boundaries.\n- Test acceptance criteria from `Requirements.md`.\n- Add security, performance, deployment, and observability choices only when the requirements earn them.\n");
    output
}

/// Renders an offline-safe research lane that distinguishes questions from evidence.
fn render_research(snapshot: &ProjectSnapshot, topic: &str, focus: &str) -> String {
    let mut output = format!(
        "# {topic} Research\n\n## Focus\n\nInvestigate {focus} for **{}**.\n\n## Current evidence\n\n",
        snapshot.project.name()
    );
    if snapshot.evidence.is_empty() {
        output.push_str("No externally retrieved evidence is stored by the deterministic offline renderer. Do not treat this absence as proof. A Codex-enabled run may research this lane with the tools allowed by its configured sandbox and must retain direct sources.\n");
    } else {
        for evidence in &snapshot.evidence {
            output.push_str(&format!(
                "- **{}** {} — [{}]({}) (reliability: `{:?}`, retrieved `{}`)\n",
                evidence.display_id,
                evidence.claim,
                evidence.source_title,
                evidence.source,
                evidence.reliability,
                evidence.retrieved_at.to_rfc3339()
            ));
        }
    }
    output.push_str("\n## Questions\n\n- Which claim in this lane would most change a requirement or decision?\n- What primary source could confirm or reject it?\n- When was that source retrieved, and how reliable is it?\n");
    output
}

/// Derives one stable timestamped log filename from project creation time.
fn session_log_name(snapshot: &ProjectSnapshot) -> String {
    format!(
        "log-{}.md",
        snapshot.project.created_at().format("%Y-%m-%d-%H-%M-%S")
    )
}

/// Renders the actual stored brief, answers, and produced-artifact decision record.
fn render_session_log(snapshot: &ProjectSnapshot) -> String {
    let mut output = format!(
        "# Project Initiation Session\n\n## Notes\n\nProject created at `{}`.\n\n## User Prompt Log\n\n{}\n\n## Planning Decisions Captured During The Session\n\n",
        snapshot.project.created_at().to_rfc3339(),
        snapshot.project.brief()
    );
    for answer in &snapshot.answers {
        output.push_str(&format!(
            "- **{}** {}\n",
            answer.display_id, answer.answer_text
        ));
    }
    if snapshot.answers.is_empty() {
        output.push_str("- No clarification answers have been recorded.\n");
    }
    for decision in &snapshot.decisions {
        output.push_str(&format!(
            "- **{}** {} (`{:?}`)\n",
            decision.display_id, decision.statement, decision.status
        ));
    }
    output.push_str("\n## Resulting Session Artifacts\n\n- See `README.md` for the complete package index.\n\n## Summary\n\nThe package was rendered from the authoritative project snapshot without fabricating external research.\n");
    output
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
