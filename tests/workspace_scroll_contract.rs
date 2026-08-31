use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use project_init::agents::{ActivityEvent, ActivityKind};
use project_init::domain::{Requirement, RequirementStatus, SourceType};
use project_init::storage::SqliteStore;
use project_init::tui::{WorkspaceSection, WorkspaceState, render_workspace};
use project_init::workflow::ProjectService;
use ratatui::{Terminal, backend::TestBackend};

/// Creates enough wrapped findings to overflow a compact workbench viewport.
fn findings_workspace(count: usize) -> WorkspaceState {
    let findings = (1..=count)
        .map(|index| serde_json::json!({
            "kind": "confirmed_fact",
            "statement": format!("Finding {index:02} begins {} ends {index:02}", "detail ".repeat(18)),
            "impact": "low"
        }))
        .collect::<Vec<_>>();
    let store = SqliteStore::open_in_memory().expect("database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Scroll test",
            "Explicit scope",
            &serde_json::json!({"findings": findings}).to_string(),
        )
        .expect("project should initialize");
    let snapshot = service
        .inspect_project(project.id())
        .expect("snapshot should load");
    let mut state = WorkspaceState::new(snapshot, Vec::new());
    while state.section() != WorkspaceSection::Findings {
        press(&mut state, KeyCode::Right);
    }
    state
}

/// Sends a real workbench key event without invoking external workflow operations.
fn press(state: &mut WorkspaceState, code: KeyCode) {
    assert!(
        state
            .handle_key(KeyEvent::new(code, KeyModifiers::NONE))
            .expect("navigation should succeed")
            .is_none()
    );
}

/// Captures the actual Ratatui output and refreshes viewport measurements.
fn render(state: &WorkspaceState, width: u16, height: u16) -> String {
    let mut terminal =
        Terminal::new(TestBackend::new(width, height)).expect("terminal should initialize");
    terminal
        .draw(|frame| render_workspace(frame, state))
        .expect("workspace should render");
    terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect()
}

/// Scrolls wrapped rows in both directions and reaches findings below the first viewport.
#[test]
fn findings_arrow_keys_scroll_wrapped_content() {
    let mut state = findings_workspace(12);
    let first = render(&state, 90, 18);
    assert!(first.contains("Finding 01 begins"));
    assert!(!first.contains("ends 12"));
    press(&mut state, KeyCode::Down);
    let second = render(&state, 90, 18);
    assert_ne!(first, second, "Down must change the visible findings");
    press(&mut state, KeyCode::Up);
    assert_eq!(render(&state, 90, 18), first);
    for _ in 0..100 {
        press(&mut state, KeyCode::Down);
        render(&state, 90, 18);
    }
    let last = render(&state, 90, 18);
    assert!(last.contains("ends 12"));
    press(&mut state, KeyCode::Down);
    assert_eq!(
        render(&state, 90, 18),
        last,
        "scrolling must stop at the bottom"
    );
    press(&mut state, KeyCode::Up);
    assert_ne!(
        render(&state, 90, 18),
        last,
        "Up must respond immediately at the bottom"
    );
}

/// Supports page and boundary navigation without losing the offset when switching sections.
#[test]
fn findings_support_page_and_boundary_keys() {
    let mut state = findings_workspace(12);
    let first = render(&state, 90, 18);
    press(&mut state, KeyCode::PageDown);
    let page = render(&state, 90, 18);
    assert_ne!(page, first);
    press(&mut state, KeyCode::PageUp);
    assert_eq!(render(&state, 90, 18), first);
    press(&mut state, KeyCode::End);
    let last = render(&state, 90, 18);
    assert!(last.contains("ends 12"));
    press(&mut state, KeyCode::Right);
    render(&state, 90, 18);
    press(&mut state, KeyCode::Left);
    assert_eq!(render(&state, 90, 18), last);
    press(&mut state, KeyCode::Home);
    assert_eq!(render(&state, 90, 18), first);
    press(&mut state, KeyCode::Up);
    assert_eq!(render(&state, 90, 18), first);
}

/// Clamps an existing offset when a resize or refreshed snapshot removes overflow.
#[test]
fn findings_scroll_clamps_after_resize_and_refresh() {
    let mut state = findings_workspace(12);
    render(&state, 50, 18);
    press(&mut state, KeyCode::End);
    assert!(render(&state, 50, 18).contains("ends 12"));
    let expanded = render(&state, 180, 50);
    assert!(expanded.contains("Finding 01 begins"));
    assert!(expanded.contains("ends 12"));
    render(&state, 50, 18);
    press(&mut state, KeyCode::End);
    let mut snapshot = state.snapshot().clone();
    snapshot.findings.truncate(1);
    state.apply_snapshot(snapshot);
    assert!(render(&state, 90, 18).contains("Finding 01 begins"));
    let mut snapshot = state.snapshot().clone();
    snapshot.findings.clear();
    state.apply_snapshot(snapshot);
    let empty = render(&state, 90, 18);
    press(&mut state, KeyCode::End);
    assert_eq!(render(&state, 90, 18), empty);
}

/// Populates the other read-only panels with distinct wrapped rows for navigation checks.
fn populated_workspace(section: WorkspaceSection) -> WorkspaceState {
    let mut snapshot = findings_workspace(20).snapshot().clone();
    let timestamp = snapshot
        .findings
        .last()
        .expect("findings should exist")
        .updated_at()
        + chrono::Duration::seconds(1);
    snapshot.requirements = (1..=20)
        .map(|index| Requirement {
            id: format!("requirement-{index}"),
            display_id: format!("REQ-{index:03}"),
            project_id: snapshot.project.id().clone(),
            statement: format!(
                "Requirement {index:02} {} req-end-{index:02}",
                "detail ".repeat(18)
            ),
            source_type: SourceType::UserBrief,
            source_reference: "brief".to_owned(),
            priority: "high".to_owned(),
            status: RequirementStatus::Active,
            rationale: None,
            acceptance_criteria: "Visible when scrolled".to_owned(),
            created_at: timestamp,
            updated_at: timestamp,
        })
        .collect();
    let activity = (1..=20)
        .map(|index| ActivityEvent {
            sequence: index,
            kind: ActivityKind::Progress,
            message: format!(
                "Activity {index:02} {} activity-end-{index:02}",
                "detail ".repeat(18)
            ),
            created_at: timestamp,
        })
        .collect();
    let mut state = WorkspaceState::new(snapshot, activity);
    while state.section() != section {
        press(&mut state, KeyCode::Right);
    }
    state
}

/// Makes all read-only sections scroll by row, page, and boundary without overscroll.
#[test]
fn read_only_panels_support_all_scroll_controls() {
    for section in [
        WorkspaceSection::Overview,
        WorkspaceSection::Requirements,
        WorkspaceSection::Activity,
    ] {
        let mut state = populated_workspace(section);
        let first = render(&state, 90, 18);
        press(&mut state, KeyCode::Down);
        let row = render(&state, 90, 18);
        assert_ne!(row, first, "{section:?} must scroll down");
        press(&mut state, KeyCode::Home);
        render(&state, 90, 18);
        press(&mut state, KeyCode::PageDown);
        let page = render(&state, 90, 18);
        assert_ne!(page, first);
        assert_ne!(page, row, "page navigation must advance more than one row");
        press(&mut state, KeyCode::PageUp);
        assert_eq!(render(&state, 90, 18), first);
        press(&mut state, KeyCode::End);
        let last = render(&state, 90, 18);
        assert_ne!(last, first);
        press(&mut state, KeyCode::Down);
        assert_eq!(render(&state, 90, 18), last);
        press(&mut state, KeyCode::Up);
        assert_ne!(render(&state, 90, 18), last);
        press(&mut state, KeyCode::Home);
        assert_eq!(render(&state, 90, 18), first);
    }
}

/// Keeps each panel's offset independent and stops composer input from scrolling content.
#[test]
fn panels_preserve_independent_offsets_and_composer_focus() {
    let mut state = populated_workspace(WorkspaceSection::Findings);
    render(&state, 90, 18);
    press(&mut state, KeyCode::PageDown);
    let findings = render(&state, 90, 18);
    press(&mut state, KeyCode::Right);
    let requirements = render(&state, 90, 18);
    assert!(requirements.contains("Requirement 01"));
    press(&mut state, KeyCode::End);
    let requirements_end = render(&state, 90, 18);
    assert!(requirements_end.contains("req-end-20"));
    press(&mut state, KeyCode::Left);
    assert_eq!(render(&state, 90, 18), findings);
    press(&mut state, KeyCode::Right);
    assert_eq!(render(&state, 90, 18), requirements_end);
    state.focus_composer();
    let composer = render(&state, 90, 18);
    press(&mut state, KeyCode::Home);
    press(&mut state, KeyCode::PageUp);
    assert_eq!(render(&state, 90, 18), composer);
}

/// Exposes the complete overview timeline rather than discarding all but twelve entries.
#[test]
fn overview_scrolling_can_reach_the_complete_timeline() {
    let mut state = populated_workspace(WorkspaceSection::Overview);
    let first = render(&state, 90, 18);
    assert!(
        first.contains("Finding 01 begins"),
        "old timeline entries must remain reachable"
    );
    press(&mut state, KeyCode::End);
    assert!(render(&state, 90, 18).contains("req-end-20"));
}
