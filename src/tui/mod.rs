//! Ratatui project inspection with safe terminal setup and restoration.

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::domain::ProjectSnapshot;

/// Opens a full-screen project overview until the user presses Q or Escape.
pub fn run(snapshot: &ProjectSnapshot) -> io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, snapshot))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                {
                    return Ok(());
                }
            }
        }
    })
}

/// Renders overview, prioritized clarification, and findings panels.
fn render(frame: &mut Frame<'_>, snapshot: &ProjectSnapshot) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Percentage(42),
            Constraint::Percentage(58),
        ])
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
}
