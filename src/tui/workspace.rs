//! Pure contextual-workbench state, projections, and deterministic Ratatui rendering.

use std::collections::{HashMap, HashSet};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use thiserror::Error;

use crate::agents::{ActivityEvent, ActivityKind};
use crate::domain::{
    ApprovalPolicy, CostOfBeingWrong, DecisionStatus, Impact, ProjectSnapshot, Question,
    QuestionStatus, Uncertainty, WorkflowStatus, WorkflowStep,
};
use crate::workflow::{
    AskQuestionRequest, AutoAnswerActor, AutoAnswerProgress, ValidationFinding,
    WORKBENCH_QUESTION_RATIONALE,
};

/// Names each durable project view available from the workbench navigation rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceSection {
    /// Summarizes lifecycle state and the projected project timeline.
    Overview,
    /// Lists clarification questions and their deterministic priority.
    Questions,
    /// Lists extracted and user-reconciled project findings.
    Findings,
    /// Lists implementation requirements derived from authoritative state.
    Requirements,
    /// Lists persisted operational analysis activity.
    Activity,
}

impl WorkspaceSection {
    /// Returns every navigation section in stable display order.
    const fn all() -> [Self; 5] {
        [
            Self::Overview,
            Self::Questions,
            Self::Findings,
            Self::Requirements,
            Self::Activity,
        ]
    }

    /// Returns the uppercase label used by navigation and panel titles.
    const fn title(self) -> &'static str {
        match self {
            Self::Overview => "OVERVIEW",
            Self::Questions => "QUESTIONS",
            Self::Findings => "FINDINGS",
            Self::Requirements => "REQUIREMENTS",
            Self::Activity => "ACTIVITY",
        }
    }

    /// Moves to the neighboring section while clamping at the first and last item.
    fn offset(self, delta: isize) -> Self {
        let sections = Self::all();
        let current = sections
            .iter()
            .position(|section| *section == self)
            .unwrap_or(0);
        let next = current.saturating_add_signed(delta).min(sections.len() - 1);
        sections[next]
    }
}

/// Identifies which workbench surface currently receives keyboard input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceFocus {
    /// Keyboard navigation changes the active section.
    Navigation,
    /// Keyboard navigation changes the selected entity in the active section.
    Content,
    /// Text input edits and submits the contextual composer.
    Composer,
    /// Text and choice input edit a structured user-authored question.
    QuestionDraft,
    /// Arrow and confirmation keys choose a mandatory first-run approval policy.
    PolicySelection,
}

/// Represents one validated application operation emitted by pure TUI state handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceCommand {
    /// Reconciles immutable answer text against one open question.
    Answer { question_id: String, answer: String },
    /// Persists an explicitly structured user-authored question.
    AskQuestion(AskQuestionRequest),
    /// Changes the inclusive score used to classify consequential questions.
    SetClarificationThreshold(u16),
    /// Requests continuation of an existing run or first-run policy selection.
    Resume,
    /// Starts fixed three-worker automatic clarification or requests first-run policy selection.
    AutoAnswer,
    /// Starts a first workflow run with the policy explicitly selected by the user.
    StartRun(ApprovalPolicy),
    /// Starts a first workflow run and immediately activates automatic clarification.
    StartAutoAnswer(ApprovalPolicy),
    /// Records explicit acceptance of one pending project decision.
    ApproveDecision { decision_id: String, reason: String },
    /// Records explicit rejection of one pending project decision.
    RejectDecision { decision_id: String, reason: String },
    /// Authorizes one bounded repair attempt before a later explicit resume.
    RequestRepair,
    /// Closes the workbench without mutating authoritative project state.
    Quit,
}

/// Describes correctable composer or structured-capture input failures.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WorkspaceInputError {
    /// Plain text has no selected open question that can authoritatively receive it.
    #[error("select an open question or use an explicit command")]
    NoAnswerTarget,
    /// A slash command is missing required arguments.
    #[error("{0}")]
    IncompleteCommand(String),
    /// A slash-prefixed input does not map to a supported deterministic operation.
    #[error("unsupported command: {0}")]
    UnsupportedCommand(String),
    /// Structured question capture is missing required user-visible text.
    #[error("{0} must not be blank")]
    BlankQuestionField(&'static str),
}

/// Identifies the active field inside structured question capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuestionDraftField {
    /// Edits the user-facing clarification prompt.
    Prompt,
    /// Edits the explanation for why the question matters.
    Rationale,
    /// Selects the scope consequence of an incorrect answer.
    Impact,
    /// Selects how incomplete current project knowledge is.
    Uncertainty,
    /// Selects how expensive later correction would be.
    Cost,
}

/// Holds a transient mandatory approval-policy choice with no implicit default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicySelection {
    selected: Option<ApprovalPolicy>,
    auto_answer: bool,
}

/// Retains bounded display activity while one background workflow execution is active.
#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkspaceExecutionView {
    activity: Vec<ActivityEvent>,
    auto_answer: Option<Vec<AutoAnswerProgress>>,
}

impl WorkspaceExecutionView {
    /// Creates an empty execution view before the worker emits its first milestone.
    const fn new(auto_answer: bool) -> Self {
        Self {
            activity: Vec::new(),
            auto_answer: if auto_answer { Some(Vec::new()) } else { None },
        }
    }

    /// Retains a bounded recent suffix so long external runs cannot grow presentation memory.
    fn push(&mut self, event: ActivityEvent) {
        const MAX_EXECUTION_ACTIVITY: usize = 200;
        if self.activity.len() == MAX_EXECUTION_ACTIVITY {
            self.activity.remove(0);
        }
        self.activity.push(event);
    }

    /// Replaces one current-batch actor lane while retaining a bounded five-lane board.
    fn push_auto_answer(&mut self, progress: AutoAnswerProgress) {
        let Some(lanes) = self.auto_answer.as_mut() else {
            return;
        };
        match lanes.iter().map(|lane| lane.batch).max() {
            Some(batch) if progress.batch < batch => return,
            Some(batch) if progress.batch > batch => lanes.clear(),
            _ => {}
        }
        if let Some(existing) = lanes
            .iter_mut()
            .find(|existing| same_auto_answer_lane(&existing.actor, &progress.actor))
        {
            *existing = progress;
        } else if lanes.len() < 5 {
            lanes.push(progress);
        }
    }
}

/// Matches stable progress lanes without treating a worker's changing question as its identity.
fn same_auto_answer_lane(existing: &AutoAnswerActor, incoming: &AutoAnswerActor) -> bool {
    match (existing, incoming) {
        (AutoAnswerActor::Coordinator, AutoAnswerActor::Coordinator)
        | (AutoAnswerActor::Judge, AutoAnswerActor::Judge) => true,
        (
            AutoAnswerActor::Worker { slot: existing, .. },
            AutoAnswerActor::Worker { slot: incoming, .. },
        ) => existing == incoming,
        _ => false,
    }
}

impl PolicySelection {
    /// Creates an unselected policy prompt that cannot yet be submitted.
    const fn new(auto_answer: bool) -> Self {
        Self {
            selected: None,
            auto_answer,
        }
    }

    /// Returns the explicitly highlighted policy, if the user has navigated to one.
    pub const fn selected(&self) -> Option<ApprovalPolicy> {
        self.selected
    }

    /// Returns whether the selected policy should launch automatic clarification.
    const fn starts_auto_answer(&self) -> bool {
        self.auto_answer
    }

    /// Moves through stable policy order and deliberately selects on first navigation.
    fn move_selection(&mut self, delta: isize) {
        let policies = [
            ApprovalPolicy::Strict,
            ApprovalPolicy::Consequential,
            ApprovalPolicy::Autonomous,
        ];
        let current = self
            .selected
            .and_then(|selected| policies.iter().position(|policy| *policy == selected));
        let next = match (current, delta.is_negative()) {
            (None, false) => 0,
            (None, true) => policies.len() - 1,
            (Some(index), true) => index.saturating_sub(1),
            (Some(index), false) => (index + 1).min(policies.len() - 1),
        };
        self.selected = Some(policies[next]);
    }
}

impl QuestionDraftField {
    /// Cycles focus through every structured question field.
    fn next(self) -> Self {
        match self {
            Self::Prompt => Self::Rationale,
            Self::Rationale => Self::Impact,
            Self::Impact => Self::Uncertainty,
            Self::Uncertainty => Self::Cost,
            Self::Cost => Self::Prompt,
        }
    }
}

/// Holds explicit editable input before a user-authored question becomes authoritative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionDraft {
    prompt: String,
    rationale: String,
    impact: Impact,
    uncertainty: Uncertainty,
    cost: CostOfBeingWrong,
    field: QuestionDraftField,
}

impl QuestionDraft {
    /// Creates structured capture with the approved visible conservative defaults.
    fn conservative(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_owned(),
            rationale: WORKBENCH_QUESTION_RATIONALE.to_owned(),
            impact: Impact::Medium,
            uncertainty: Uncertainty::High,
            cost: CostOfBeingWrong::Medium,
            field: QuestionDraftField::Prompt,
        }
    }

    /// Returns the currently edited user-facing prompt.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Returns why answering this question matters to the project.
    pub fn rationale(&self) -> &str {
        &self.rationale
    }

    /// Returns the selected scope impact.
    pub const fn impact(&self) -> Impact {
        self.impact
    }

    /// Returns the selected knowledge uncertainty.
    pub const fn uncertainty(&self) -> Uncertainty {
        self.uncertainty
    }

    /// Returns the selected rework cost.
    pub const fn cost_of_being_wrong(&self) -> CostOfBeingWrong {
        self.cost
    }

    /// Appends one character to the active textual field.
    fn insert_char(&mut self, value: char) {
        match self.field {
            QuestionDraftField::Prompt => self.prompt.push(value),
            QuestionDraftField::Rationale => self.rationale.push(value),
            QuestionDraftField::Impact
            | QuestionDraftField::Uncertainty
            | QuestionDraftField::Cost => {}
        }
    }

    /// Removes one character from the active textual field.
    fn backspace(&mut self) {
        match self.field {
            QuestionDraftField::Prompt => {
                self.prompt.pop();
            }
            QuestionDraftField::Rationale => {
                self.rationale.pop();
            }
            QuestionDraftField::Impact
            | QuestionDraftField::Uncertainty
            | QuestionDraftField::Cost => {}
        }
    }

    /// Cycles the active categorical value in the requested direction.
    fn cycle_choice(&mut self, reverse: bool) {
        match self.field {
            QuestionDraftField::Impact => self.impact = cycle_impact(self.impact, reverse),
            QuestionDraftField::Uncertainty => {
                self.uncertainty = cycle_uncertainty(self.uncertainty, reverse);
            }
            QuestionDraftField::Cost => self.cost = cycle_cost(self.cost, reverse),
            QuestionDraftField::Prompt | QuestionDraftField::Rationale => {}
        }
    }

    /// Converts complete visible fields into one workflow request without hidden inference.
    fn request(&self) -> Result<AskQuestionRequest, WorkspaceInputError> {
        if self.prompt.trim().is_empty() {
            return Err(WorkspaceInputError::BlankQuestionField("question prompt"));
        }
        if self.rationale.trim().is_empty() {
            return Err(WorkspaceInputError::BlankQuestionField(
                "question rationale",
            ));
        }
        Ok(AskQuestionRequest::new(
            &self.prompt,
            &self.rationale,
            self.impact,
            self.uncertainty,
            self.cost,
        ))
    }
}

/// Represents one non-authoritative row projected from durable project records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineEntry {
    timestamp: chrono::DateTime<chrono::Utc>,
    category: &'static str,
    stable_key: String,
    message: String,
}

impl TimelineEntry {
    /// Returns when the projected authoritative state was created or last changed.
    pub const fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.timestamp
    }

    /// Returns the human-readable projected timeline message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Summarizes entity additions and status changes caused by one accepted mutation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SnapshotDiff {
    questions_changed: usize,
    answers_added: usize,
    findings_changed: usize,
    requirements_added: usize,
    traces_added: usize,
    project_status_changed: bool,
}

impl SnapshotDiff {
    /// Returns how many questions were added or changed status.
    pub const fn questions_changed(&self) -> usize {
        self.questions_changed
    }

    /// Returns how many immutable answers were added.
    pub const fn answers_added(&self) -> usize {
        self.answers_added
    }

    /// Returns how many findings were added or changed status.
    pub const fn findings_changed(&self) -> usize {
        self.findings_changed
    }

    /// Returns how many requirements were added by the mutation.
    pub const fn requirements_added(&self) -> usize {
        self.requirements_added
    }

    /// Returns how many provenance edges were added by the mutation.
    pub const fn traces_added(&self) -> usize {
        self.traces_added
    }

    /// Returns whether the project lifecycle status changed.
    pub const fn project_status_changed(&self) -> bool {
        self.project_status_changed
    }

    /// Formats the affected entity counts for immediate workbench feedback.
    fn summary(&self) -> String {
        format!(
            "Changed: {} questions · {} answers · {} findings · {} requirements · {} traces{}",
            self.questions_changed,
            self.answers_added,
            self.findings_changed,
            self.requirements_added,
            self.traces_added,
            if self.project_status_changed {
                " · project status"
            } else {
                ""
            }
        )
    }
}

/// Owns disposable interaction state and one current authoritative project snapshot.
#[derive(Debug, Clone)]
pub struct WorkspaceState {
    snapshot: ProjectSnapshot,
    activity: Vec<ActivityEvent>,
    timeline: Vec<TimelineEntry>,
    section: WorkspaceSection,
    focus: WorkspaceFocus,
    selected_question: Option<usize>,
    composer: String,
    question_draft: Option<QuestionDraft>,
    policy_selection: Option<PolicySelection>,
    workflow_status: Option<WorkflowStatus>,
    validation_findings: Vec<ValidationFinding>,
    execution: Option<WorkspaceExecutionView>,
    notice: Option<String>,
}

impl WorkspaceState {
    /// Creates a workspace and selects the highest-priority open clarification when available.
    pub fn new(snapshot: ProjectSnapshot, activity: Vec<ActivityEvent>) -> Self {
        let selected_question = next_question_index(&snapshot);
        let section = if selected_question.is_some() {
            WorkspaceSection::Questions
        } else {
            WorkspaceSection::Overview
        };
        let timeline = project_timeline(&snapshot, &activity);
        Self {
            snapshot,
            activity,
            timeline,
            section,
            focus: WorkspaceFocus::Content,
            selected_question,
            composer: String::new(),
            question_draft: None,
            policy_selection: None,
            workflow_status: None,
            validation_findings: Vec::new(),
            execution: None,
            notice: None,
        }
    }

    /// Returns the active navigation section.
    pub const fn section(&self) -> WorkspaceSection {
        self.section
    }

    /// Returns the surface currently receiving keyboard input.
    pub const fn focus(&self) -> WorkspaceFocus {
        self.focus
    }

    /// Returns the current unsubmitted composer text.
    pub fn composer(&self) -> &str {
        &self.composer
    }

    /// Replaces composer text for deterministic input restoration and tests.
    pub fn set_composer(&mut self, value: &str) {
        self.composer = value.to_owned();
    }

    /// Moves keyboard input into the contextual composer.
    pub fn focus_composer(&mut self) {
        self.focus = WorkspaceFocus::Composer;
    }

    /// Returns the currently selected open question, if one remains.
    pub fn selected_question(&self) -> Option<&Question> {
        if self.section != WorkspaceSection::Questions {
            return None;
        }
        self.selected_question
            .and_then(|index| self.snapshot.questions.get(index))
            .filter(|question| question.status == QuestionStatus::Open)
    }

    /// Returns the structured question draft while its overlay is active.
    pub const fn question_draft(&self) -> Option<&QuestionDraft> {
        self.question_draft.as_ref()
    }

    /// Returns the mandatory first-run policy selector while it is visible.
    pub const fn policy_selection(&self) -> Option<&PolicySelection> {
        self.policy_selection.as_ref()
    }

    /// Opens an unselected first-run policy prompt without mutating authoritative state.
    pub fn open_policy_selection(&mut self) {
        self.policy_selection = Some(PolicySelection::new(false));
        self.focus = WorkspaceFocus::PolicySelection;
    }

    /// Opens mandatory policy selection that will launch automatic clarification on submit.
    pub fn open_auto_answer_policy_selection(&mut self) {
        self.policy_selection = Some(PolicySelection::new(true));
        self.focus = WorkspaceFocus::PolicySelection;
    }

    /// Replaces disposable workflow guidance with newly observed authoritative context.
    pub fn set_workflow_context(
        &mut self,
        status: Option<WorkflowStatus>,
        validation_findings: Vec<ValidationFinding>,
    ) {
        self.workflow_status = status;
        self.validation_findings = validation_findings;
    }

    /// Marks the workspace as externally executing and clears activity from an older run.
    pub fn begin_execution(&mut self) {
        self.execution = Some(WorkspaceExecutionView::new(false));
        self.notice = Some("Workflow execution started".to_owned());
    }

    /// Opens the structured automatic-answer board before coordinator work begins.
    pub fn begin_auto_answer_execution(&mut self) {
        self.execution = Some(WorkspaceExecutionView::new(true));
        self.notice = Some("Parallel automatic answering started".to_owned());
    }

    /// Adds one sanitized runner milestone to the active progress overlay.
    pub fn push_execution_activity(&mut self, event: ActivityEvent) {
        if let Some(execution) = self.execution.as_mut() {
            execution.push(event);
        }
    }

    /// Applies one actor-keyed automatic-answer update to the active progress board.
    pub fn push_auto_answer_progress(&mut self, progress: AutoAnswerProgress) {
        if let Some(execution) = self.execution.as_mut() {
            execution.push_auto_answer(progress);
        }
    }

    /// Returns whether the workbench currently owns one background execution.
    pub const fn execution_active(&self) -> bool {
        self.execution.is_some()
    }

    /// Closes the progress overlay and leaves a concise recoverable result notice.
    pub fn finish_execution(&mut self, message: &str) {
        self.execution = None;
        self.notice = Some(message.to_owned());
    }

    /// Returns the complete derived timeline in stable rendering order.
    pub fn timeline(&self) -> &[TimelineEntry] {
        &self.timeline
    }

    /// Parses and consumes valid composer text while preserving correctable failures.
    pub fn submit_composer(&mut self) -> Result<Option<WorkspaceCommand>, WorkspaceInputError> {
        let input = self.composer.trim().to_owned();
        if let Some(prompt) = ask_prompt(&input) {
            self.question_draft = Some(QuestionDraft::conservative(prompt));
            self.composer.clear();
            self.focus = WorkspaceFocus::QuestionDraft;
            return Ok(None);
        }
        let command = match input.split_whitespace().next() {
            Some("/answer") => self.explicit_answer(&input)?,
            Some("/threshold") => threshold_command(&input)?,
            Some("/resume") => exact_command(&input, "/resume", WorkspaceCommand::Resume)?,
            Some("/auto-answer") => {
                exact_command(&input, "/auto-answer", WorkspaceCommand::AutoAnswer)?
            }
            Some("/repair") => exact_command(&input, "/repair", WorkspaceCommand::RequestRepair)?,
            Some("/approve") => self.decision_authority(&input, true)?,
            Some("/reject") => self.decision_authority(&input, false)?,
            Some(command) if command.starts_with('/') => {
                return Err(WorkspaceInputError::UnsupportedCommand(input));
            }
            _ => self.contextual_answer(&input)?,
        };
        self.composer.clear();
        self.notice = None;
        Ok(Some(command))
    }

    /// Applies one keyboard event to the focused surface and emits validated workflow intent.
    pub fn handle_key(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<WorkspaceCommand>, WorkspaceInputError> {
        match self.focus {
            WorkspaceFocus::Composer => self.handle_composer_key(key),
            WorkspaceFocus::QuestionDraft => self.handle_question_draft_key(key),
            WorkspaceFocus::PolicySelection => self.handle_policy_selection_key(key),
            WorkspaceFocus::Navigation | WorkspaceFocus::Content => self.handle_navigation_key(key),
        }
    }

    /// Replaces stale authoritative data, calculates feedback, and advances the open queue.
    pub fn apply_snapshot(&mut self, snapshot: ProjectSnapshot) -> SnapshotDiff {
        let diff = snapshot_diff(&self.snapshot, &snapshot);
        self.snapshot = snapshot;
        self.selected_question = next_question_index(&self.snapshot);
        if self.selected_question.is_some() {
            self.section = WorkspaceSection::Questions;
        }
        self.timeline = project_timeline(&self.snapshot, &self.activity);
        self.notice = Some(diff.summary());
        diff
    }

    /// Stores a recoverable event-loop error for visible correction without exiting.
    pub fn show_error(&mut self, message: &str) {
        self.notice = Some(format!("Error: {message}"));
    }

    /// Stores an informational event-loop notice until the next accepted mutation.
    pub fn show_notice(&mut self, message: &str) {
        self.notice = Some(message.to_owned());
    }

    /// Returns the current authoritative snapshot for workflow targeting and rendering.
    pub const fn snapshot(&self) -> &ProjectSnapshot {
        &self.snapshot
    }

    /// Parses plain text against the selected open question.
    fn contextual_answer(&self, input: &str) -> Result<WorkspaceCommand, WorkspaceInputError> {
        if input.is_empty() {
            return Err(WorkspaceInputError::IncompleteCommand(
                "answer text must not be blank".to_owned(),
            ));
        }
        let question = self
            .selected_question()
            .ok_or(WorkspaceInputError::NoAnswerTarget)?;
        Ok(WorkspaceCommand::Answer {
            question_id: question.id.clone(),
            answer: input.to_owned(),
        })
    }

    /// Parses an explicit display or durable question identity and answer body.
    fn explicit_answer(&self, input: &str) -> Result<WorkspaceCommand, WorkspaceInputError> {
        let mut parts = input.splitn(3, char::is_whitespace);
        let _command = parts.next();
        let target = parts
            .next()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| {
                WorkspaceInputError::IncompleteCommand(
                    "usage: /answer <question-id> <answer>".to_owned(),
                )
            })?;
        let answer = parts
            .next()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                WorkspaceInputError::IncompleteCommand(
                    "usage: /answer <question-id> <answer>".to_owned(),
                )
            })?;
        let question = self
            .snapshot
            .questions
            .iter()
            .find(|question| {
                question.status == QuestionStatus::Open
                    && (question.id == target || question.display_id.eq_ignore_ascii_case(target))
            })
            .ok_or_else(|| {
                WorkspaceInputError::IncompleteCommand(format!("open question not found: {target}"))
            })?;
        Ok(WorkspaceCommand::Answer {
            question_id: question.id.clone(),
            answer: answer.to_owned(),
        })
    }

    /// Parses explicit decision authority against only pending decisions in this project.
    fn decision_authority(
        &self,
        input: &str,
        approved: bool,
    ) -> Result<WorkspaceCommand, WorkspaceInputError> {
        let usage = if approved {
            "/approve <decision-id> <reason>"
        } else {
            "/reject <decision-id> <reason>"
        };
        let mut parts = input.splitn(3, char::is_whitespace);
        let _command = parts.next();
        let target = parts
            .next()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| WorkspaceInputError::IncompleteCommand(format!("usage: {usage}")))?;
        let reason = parts
            .next()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| WorkspaceInputError::IncompleteCommand(format!("usage: {usage}")))?;
        let decision = self
            .snapshot
            .decisions
            .iter()
            .find(|decision| {
                matches!(
                    decision.status,
                    DecisionStatus::Proposed | DecisionStatus::NeedsConfirmation
                ) && (decision.id == target || decision.display_id.eq_ignore_ascii_case(target))
            })
            .ok_or_else(|| {
                WorkspaceInputError::IncompleteCommand(format!(
                    "pending decision not found: {target}"
                ))
            })?;
        if approved {
            Ok(WorkspaceCommand::ApproveDecision {
                decision_id: decision.id.clone(),
                reason: reason.to_owned(),
            })
        } else {
            Ok(WorkspaceCommand::RejectDecision {
                decision_id: decision.id.clone(),
                reason: reason.to_owned(),
            })
        }
    }

    /// Handles editing and submission while the composer owns focus.
    fn handle_composer_key(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<WorkspaceCommand>, WorkspaceInputError> {
        match key.code {
            KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.composer.push('\n');
                Ok(None)
            }
            KeyCode::Enter => self.submit_composer(),
            KeyCode::Char(value) => {
                self.composer.push(value);
                Ok(None)
            }
            KeyCode::Backspace => {
                self.composer.pop();
                Ok(None)
            }
            KeyCode::Esc => {
                self.focus = WorkspaceFocus::Content;
                Ok(None)
            }
            KeyCode::Tab => {
                self.focus = WorkspaceFocus::Navigation;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    /// Handles visible structured question fields without persisting incomplete input.
    fn handle_question_draft_key(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<WorkspaceCommand>, WorkspaceInputError> {
        match key.code {
            KeyCode::Esc => {
                self.question_draft = None;
                self.focus = WorkspaceFocus::Composer;
                Ok(None)
            }
            KeyCode::Tab => {
                if let Some(draft) = self.question_draft.as_mut() {
                    draft.field = draft.field.next();
                }
                Ok(None)
            }
            KeyCode::Left | KeyCode::Right => {
                if let Some(draft) = self.question_draft.as_mut() {
                    draft.cycle_choice(key.code == KeyCode::Left);
                }
                Ok(None)
            }
            KeyCode::Char(value) => {
                if let Some(draft) = self.question_draft.as_mut() {
                    draft.insert_char(value);
                }
                Ok(None)
            }
            KeyCode::Backspace => {
                if let Some(draft) = self.question_draft.as_mut() {
                    draft.backspace();
                }
                Ok(None)
            }
            KeyCode::Enter => {
                let request = self
                    .question_draft
                    .as_ref()
                    .ok_or_else(|| {
                        WorkspaceInputError::IncompleteCommand(
                            "question draft is no longer available".to_owned(),
                        )
                    })?
                    .request()?;
                self.question_draft = None;
                self.focus = WorkspaceFocus::Content;
                Ok(Some(WorkspaceCommand::AskQuestion(request)))
            }
            _ => Ok(None),
        }
    }

    /// Handles mandatory first-run policy navigation without supplying a hidden default.
    fn handle_policy_selection_key(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<WorkspaceCommand>, WorkspaceInputError> {
        match key.code {
            KeyCode::Esc => {
                self.policy_selection = None;
                self.focus = WorkspaceFocus::Content;
                Ok(None)
            }
            KeyCode::Up | KeyCode::Left => {
                if let Some(selection) = self.policy_selection.as_mut() {
                    selection.move_selection(-1);
                }
                Ok(None)
            }
            KeyCode::Down | KeyCode::Right => {
                if let Some(selection) = self.policy_selection.as_mut() {
                    selection.move_selection(1);
                }
                Ok(None)
            }
            KeyCode::Enter => {
                let selection = self.policy_selection.as_ref().ok_or_else(|| {
                    WorkspaceInputError::IncompleteCommand(
                        "select an approval policy before starting the workflow".to_owned(),
                    )
                })?;
                let policy = selection.selected().ok_or_else(|| {
                    WorkspaceInputError::IncompleteCommand(
                        "select an approval policy before starting the workflow".to_owned(),
                    )
                })?;
                let auto_answer = selection.starts_auto_answer();
                self.policy_selection = None;
                self.focus = WorkspaceFocus::Content;
                Ok(Some(if auto_answer {
                    WorkspaceCommand::StartAutoAnswer(policy)
                } else {
                    WorkspaceCommand::StartRun(policy)
                }))
            }
            _ => Ok(None),
        }
    }

    /// Handles section, entity, and focus navigation outside text entry.
    fn handle_navigation_key(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<WorkspaceCommand>, WorkspaceInputError> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Ok(Some(WorkspaceCommand::Quit)),
            KeyCode::Tab => {
                self.focus = match self.focus {
                    WorkspaceFocus::Navigation => WorkspaceFocus::Content,
                    WorkspaceFocus::Content => WorkspaceFocus::Composer,
                    WorkspaceFocus::Composer
                    | WorkspaceFocus::QuestionDraft
                    | WorkspaceFocus::PolicySelection => WorkspaceFocus::Navigation,
                };
                Ok(None)
            }
            KeyCode::Left => {
                self.section = self.section.offset(-1);
                Ok(None)
            }
            KeyCode::Right => {
                self.section = self.section.offset(1);
                Ok(None)
            }
            KeyCode::Up => {
                self.move_question_selection(-1);
                Ok(None)
            }
            KeyCode::Down => {
                self.move_question_selection(1);
                Ok(None)
            }
            KeyCode::Char('/') => {
                self.focus = WorkspaceFocus::Composer;
                self.composer.push('/');
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    /// Moves between open clarification targets while preserving selection at queue boundaries.
    fn move_question_selection(&mut self, delta: isize) {
        if self.section != WorkspaceSection::Questions {
            return;
        }
        let Some(current) = self.selected_question else {
            self.selected_question = next_question_index(&self.snapshot);
            return;
        };
        let next = if delta.is_negative() {
            (0..current)
                .rev()
                .find(|index| self.snapshot.questions[*index].status == QuestionStatus::Open)
        } else {
            ((current + 1)..self.snapshot.questions.len())
                .find(|index| self.snapshot.questions[*index].status == QuestionStatus::Open)
        };
        if let Some(next) = next {
            self.selected_question = Some(next);
        }
    }
}

/// Extracts an optional initial prompt from the exact ask command family.
fn ask_prompt(input: &str) -> Option<&str> {
    if input == "/ask" {
        Some("")
    } else {
        input.strip_prefix("/ask ").map(str::trim)
    }
}

/// Parses a persisted clarification-threshold update from the composer.
fn threshold_command(input: &str) -> Result<WorkspaceCommand, WorkspaceInputError> {
    let mut parts = input.split_whitespace();
    let _command = parts.next();
    let value = parts
        .next()
        .and_then(|value| value.parse::<u16>().ok())
        .filter(|value| (1..=125).contains(value))
        .filter(|_| parts.next().is_none())
        .ok_or_else(|| {
            WorkspaceInputError::IncompleteCommand("usage: /threshold <1-125>".to_owned())
        })?;
    Ok(WorkspaceCommand::SetClarificationThreshold(value))
}

/// Accepts one exact argument-free slash command without misleading suffixes or arguments.
fn exact_command(
    input: &str,
    expected: &'static str,
    command: WorkspaceCommand,
) -> Result<WorkspaceCommand, WorkspaceInputError> {
    if input == expected {
        Ok(command)
    } else {
        Err(WorkspaceInputError::IncompleteCommand(format!(
            "usage: {expected}"
        )))
    }
}

/// Selects the next open question, prioritizing consequential scores before the remaining queue.
fn next_question_index(snapshot: &ProjectSnapshot) -> Option<usize> {
    let threshold = snapshot.project.clarification_threshold();
    snapshot
        .questions
        .iter()
        .enumerate()
        .filter(|(_, question)| question.status == QuestionStatus::Open)
        .max_by_key(|(_, question)| {
            (
                question.priority.score() >= threshold,
                question.priority.score(),
                std::cmp::Reverse(question.display_id.as_str()),
            )
        })
        .map(|(index, _)| index)
}

/// Projects every supported authoritative record into one reproducibly ordered timeline.
fn project_timeline(snapshot: &ProjectSnapshot, activity: &[ActivityEvent]) -> Vec<TimelineEntry> {
    let mut entries = Vec::with_capacity(
        activity.len()
            + snapshot.findings.len()
            + snapshot.questions.len()
            + snapshot.answers.len()
            + snapshot.requirements.len(),
    );
    entries.extend(activity.iter().map(|event| TimelineEntry {
        timestamp: event.created_at,
        category: "activity",
        stable_key: format!("{:010}", event.sequence),
        message: event.message.clone(),
    }));
    entries.extend(snapshot.findings.iter().map(|finding| TimelineEntry {
        timestamp: finding.updated_at(),
        category: "finding",
        stable_key: finding.display_id().to_owned(),
        message: format!(
            "{} {:?}: {}",
            finding.display_id(),
            finding.kind(),
            finding.statement()
        ),
    }));
    entries.extend(snapshot.questions.iter().map(|question| TimelineEntry {
        timestamp: question.updated_at,
        category: "question",
        stable_key: question.display_id.clone(),
        message: format!(
            "{} {:?}: {}",
            question.display_id, question.status, question.prompt
        ),
    }));
    entries.extend(snapshot.answers.iter().map(|answer| TimelineEntry {
        timestamp: answer.created_at,
        category: "answer",
        stable_key: answer.display_id.clone(),
        message: format!("{}: {}", answer.display_id, answer.answer_text),
    }));
    entries.extend(
        snapshot
            .requirements
            .iter()
            .map(|requirement| TimelineEntry {
                timestamp: requirement.updated_at,
                category: "requirement",
                stable_key: requirement.display_id.clone(),
                message: format!("{}: {}", requirement.display_id, requirement.statement),
            }),
    );
    entries.sort_by(|left, right| {
        (left.timestamp, left.category, left.stable_key.as_str()).cmp(&(
            right.timestamp,
            right.category,
            right.stable_key.as_str(),
        ))
    });
    entries
}

/// Compares authoritative entity identities and statuses before replacing the current snapshot.
fn snapshot_diff(before: &ProjectSnapshot, after: &ProjectSnapshot) -> SnapshotDiff {
    let before_questions = before
        .questions
        .iter()
        .map(|question| (question.id.as_str(), question.status))
        .collect::<HashMap<_, _>>();
    let before_findings = before
        .findings
        .iter()
        .map(|finding| (finding.id().as_str(), finding.status()))
        .collect::<HashMap<_, _>>();
    let before_answers = before
        .answers
        .iter()
        .map(|answer| answer.id.as_str())
        .collect::<HashSet<_>>();
    let before_requirements = before
        .requirements
        .iter()
        .map(|requirement| requirement.id.as_str())
        .collect::<HashSet<_>>();
    let before_traces = before
        .traces
        .iter()
        .map(|trace| trace.id.as_str())
        .collect::<HashSet<_>>();
    SnapshotDiff {
        questions_changed: after
            .questions
            .iter()
            .filter(|question| before_questions.get(question.id.as_str()) != Some(&question.status))
            .count(),
        answers_added: after
            .answers
            .iter()
            .filter(|answer| !before_answers.contains(answer.id.as_str()))
            .count(),
        findings_changed: after
            .findings
            .iter()
            .filter(|finding| before_findings.get(finding.id().as_str()) != Some(&finding.status()))
            .count(),
        requirements_added: after
            .requirements
            .iter()
            .filter(|requirement| !before_requirements.contains(requirement.id.as_str()))
            .count(),
        traces_added: after
            .traces
            .iter()
            .filter(|trace| !before_traces.contains(trace.id.as_str()))
            .count(),
        project_status_changed: before.project.status() != after.project.status(),
    }
}

/// Renders the complete contextual project workbench from disposable state.
pub fn render_workspace(frame: &mut Frame<'_>, state: &WorkspaceState) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(frame.area());
    render_header(frame, state, rows[0]);
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(20)])
        .split(rows[1]);
    render_navigation(frame, state, columns[0]);
    render_content(frame, state, columns[1]);
    render_composer(frame, state, rows[2]);
    render_help(frame, rows[3]);
    if state.question_draft.is_some() {
        render_question_draft(frame, state);
    }
    if state.policy_selection.is_some() {
        render_policy_selection(frame, state);
    }
    if state.execution.is_some() {
        render_execution_overlay(frame, state);
    }
}

/// Renders durable identity, lifecycle, threshold, and the latest operation notice.
fn render_header(frame: &mut Frame<'_>, state: &WorkspaceState, area: Rect) {
    let project = &state.snapshot.project;
    let lines = vec![
        Line::from(vec![
            Span::styled(
                project.name(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "  ·  {:?}  ·  consequential ≥ {}",
                project.status(),
                project.clarification_threshold()
            )),
        ]),
        Line::from(
            state
                .notice
                .as_deref()
                .unwrap_or("Authoritative project workspace · deterministic reconciliation"),
        ),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL)),
        area,
    );
}

/// Renders stable project sections and record counts with a visible active marker.
fn render_navigation(frame: &mut Frame<'_>, state: &WorkspaceState, area: Rect) {
    let items = WorkspaceSection::all().into_iter().map(|section| {
        let count = match section {
            WorkspaceSection::Overview => state.timeline.len(),
            WorkspaceSection::Questions => state.snapshot.questions.len(),
            WorkspaceSection::Findings => state.snapshot.findings.len(),
            WorkspaceSection::Requirements => state.snapshot.requirements.len(),
            WorkspaceSection::Activity => state.activity.len(),
        };
        let marker = if section == state.section { ">" } else { " " };
        ListItem::new(format!("{marker} {} ({count})", section.title()))
    });
    let border = focus_style(state.focus == WorkspaceFocus::Navigation);
    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(" PROJECT ")
                .borders(Borders::ALL)
                .border_style(border),
        ),
        area,
    );
}

/// Renders the active section without allowing view logic to mutate its source snapshot.
fn render_content(frame: &mut Frame<'_>, state: &WorkspaceState, area: Rect) {
    let mut lines = workflow_guidance_lines(state);
    if !lines.is_empty() {
        lines.push(Line::from(""));
    }
    let guidance_line_count = lines.len();
    lines.extend(match state.section {
        WorkspaceSection::Overview => overview_lines(state),
        WorkspaceSection::Questions => question_lines(state),
        WorkspaceSection::Findings => state
            .snapshot
            .findings
            .iter()
            .map(|finding| {
                Line::from(format!(
                    "{} [{:?}] {} ({:?})",
                    finding.display_id(),
                    finding.kind(),
                    finding.statement(),
                    finding.status()
                ))
            })
            .collect(),
        WorkspaceSection::Requirements => state
            .snapshot
            .requirements
            .iter()
            .map(|requirement| {
                Line::from(format!(
                    "{} [{}] {}",
                    requirement.display_id, requirement.priority, requirement.statement
                ))
            })
            .collect(),
        WorkspaceSection::Activity => state
            .activity
            .iter()
            .map(|event| Line::from(activity_line(event)))
            .collect(),
    });
    let border = focus_style(state.focus == WorkspaceFocus::Content);
    let scroll = question_scroll_offset(state, area, guidance_line_count);
    frame.render_widget(
        Paragraph::new(lines)
            .scroll((scroll, 0))
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(format!(" {} ", state.section.title()))
                    .borders(Borders::ALL)
                    .border_style(border),
            ),
        area,
    );
}

/// Offsets the question panel only enough to keep its two-line active row in view.
fn question_scroll_offset(state: &WorkspaceState, area: Rect, guidance_line_count: usize) -> u16 {
    if state.section != WorkspaceSection::Questions {
        return 0;
    }
    let Some(selected_question) = state.selected_question else {
        return 0;
    };
    let visible_rows = usize::from(area.height.saturating_sub(2));
    let active_row_start = guidance_line_count.saturating_add(selected_question.saturating_mul(2));
    let maximum_start_row = visible_rows.saturating_sub(2);
    u16::try_from(active_row_start.saturating_sub(maximum_start_row)).unwrap_or(u16::MAX)
}

/// Builds contextual next-action copy from persisted workflow status without mutating it.
fn workflow_guidance_lines(state: &WorkspaceState) -> Vec<Line<'static>> {
    let Some(status) = state.workflow_status.as_ref() else {
        return Vec::new();
    };
    match &status.step {
        WorkflowStep::QuestionRequired { .. } => vec![
            Line::from("QUESTION REQUIRED"),
            Line::from("Answer it manually, or enter /auto-answer for cited parallel research."),
        ],
        WorkflowStep::ApprovalRequired { decision_id } => {
            let Some(decision) = state
                .snapshot
                .decisions
                .iter()
                .find(|decision| decision.id == *decision_id)
            else {
                return vec![Line::from("APPROVAL REQUIRED")];
            };
            vec![
                Line::from("APPROVAL REQUIRED"),
                Line::from(format!("{}: {}", decision.display_id, decision.title)),
                Line::from(decision.statement.clone()),
                Line::from(format!(
                    "/approve {} <reason> or /reject {} <reason>",
                    decision.display_id, decision.display_id
                )),
                Line::from("After deciding, enter /resume."),
            ]
        }
        WorkflowStep::RepairRequired { code } => {
            let mut lines = vec![
                Line::from(format!("REPAIR REQUIRED: {code}")),
                Line::from("Review the validation findings below."),
            ];
            lines.extend(
                state
                    .validation_findings
                    .iter()
                    .map(|finding| Line::from(format!("- {}: {}", finding.code, finding.message))),
            );
            lines.push(Line::from(
                "Enter /repair to authorize one attempt, then /resume.",
            ));
            lines
        }
        WorkflowStep::GeneratePackage | WorkflowStep::ValidatePackage => vec![
            Line::from("WORKFLOW READY"),
            Line::from("Enter /resume to continue the persisted run."),
        ],
        WorkflowStep::Complete => vec![
            Line::from("WORKFLOW COMPLETE"),
            Line::from("The latest documentation package passed validation."),
        ],
    }
}

/// Builds the overview summary and the newest projected authoritative timeline rows.
fn overview_lines(state: &WorkspaceState) -> Vec<Line<'static>> {
    let mut lines = vec![Line::from(format!(
        "{} findings · {} questions · {} answers · {} requirements",
        state.snapshot.findings.len(),
        state.snapshot.questions.len(),
        state.snapshot.answers.len(),
        state.snapshot.requirements.len()
    ))];
    lines.push(Line::from(""));
    lines.push(Line::from("PROJECT TIMELINE"));
    lines.extend(
        state
            .timeline
            .iter()
            .rev()
            .take(12)
            .rev()
            .map(|entry| Line::from(format!("{}  {}", entry.category, entry.message))),
    );
    lines
}

/// Builds question rows with a marker on the current contextual answer target.
fn question_lines(state: &WorkspaceState) -> Vec<Line<'static>> {
    state
        .snapshot
        .questions
        .iter()
        .enumerate()
        .map(|(index, question)| {
            let marker = if Some(index) == state.selected_question {
                ">"
            } else {
                " "
            };
            Line::from(format!(
                "{marker} {} [{}] {} ({:?})\n    {}",
                question.display_id,
                question.priority.score(),
                question.prompt,
                question.status,
                question.rationale
            ))
        })
        .collect()
}

/// Renders contextual input with the exact authoritative target in the border title.
fn render_composer(frame: &mut Frame<'_>, state: &WorkspaceState, area: Rect) {
    let title = state
        .selected_question()
        .map(|question| format!(" ANSWER {} ", question.display_id))
        .unwrap_or_else(|| " COMMAND ".to_owned());
    let border = focus_style(state.focus == WorkspaceFocus::Composer);
    frame.render_widget(
        Paragraph::new(format!("> {}", state.composer)).block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(border),
        ),
        area,
    );
}

/// Renders concise deterministic controls without suggesting unsupported chat behavior.
fn render_help(frame: &mut Frame<'_>, area: Rect) {
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Tab focus · ←→ sections · Enter submit · /ask · /resume · /auto-answer"),
            Line::from("/approve · /reject · /repair · Q/Esc quit"),
        ]),
        area,
    );
}

/// Renders the no-default approval-policy selector used by a first workbench run.
fn render_policy_selection(frame: &mut Frame<'_>, state: &WorkspaceState) {
    let Some(selection) = state.policy_selection.as_ref() else {
        return;
    };
    let area = centered_rect(62, 48, frame.area());
    let policies = [
        (
            ApprovalPolicy::Strict,
            "STRICT",
            "Approve every proposed decision",
        ),
        (
            ApprovalPolicy::Consequential,
            "CONSEQUENTIAL",
            "Approve consequential decisions",
        ),
        (
            ApprovalPolicy::Autonomous,
            "AUTONOMOUS",
            "Pause only when authority is required",
        ),
    ];
    let mut body = vec![
        Line::from("Choose the authority policy for this workflow run."),
        Line::from("No policy is selected by default."),
        Line::from(""),
    ];
    body.extend(policies.into_iter().map(|(policy, label, detail)| {
        let marker = if selection.selected == Some(policy) {
            ">"
        } else {
            " "
        };
        Line::from(format!("{marker} {label}: {detail}"))
    }));
    body.push(Line::from(""));
    body.push(Line::from("Arrow keys select · Enter start · Esc cancel"));
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(body).wrap(Wrap { trim: false }).block(
            Block::default()
                .title(" SELECT APPROVAL POLICY ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        ),
        area,
    );
}

/// Covers the workspace with bounded activity while external provider work is active.
fn render_execution_overlay(frame: &mut Frame<'_>, state: &WorkspaceState) {
    let Some(execution) = state.execution.as_ref() else {
        return;
    };
    if execution.auto_answer.is_some() {
        render_auto_answer_overlay(frame, execution);
        return;
    }
    let area = frame.area();
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(4),
            Constraint::Length(3),
        ])
        .split(area);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(
                "RUNNING WORKFLOW",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from("Provider package work remains provisional until adoption."),
        ])
        .block(Block::default().borders(Borders::ALL)),
        rows[0],
    );
    let visible_rows = usize::from(rows[1].height.saturating_sub(2));
    let items = execution
        .activity
        .iter()
        .rev()
        .take(visible_rows)
        .rev()
        .map(|event| {
            let prefix = match event.kind {
                ActivityKind::Lifecycle => "*",
                ActivityKind::Progress => "-",
                ActivityKind::Warning => "!",
            };
            ListItem::new(format!("{prefix} {}", event.message))
        });
    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(" CODEX OUTPUT ")
                .borders(Borders::ALL),
        ),
        rows[1],
    );
    frame.render_widget(
        Paragraph::new("Esc cancel · wait for cancellation acknowledgement")
            .block(Block::default().borders(Borders::ALL)),
        rows[2],
    );
}

/// Covers the workbench with the latest bounded coordinator, worker, and judge lane states.
fn render_auto_answer_overlay(frame: &mut Frame<'_>, execution: &WorkspaceExecutionView) {
    let area = frame.area();
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(area);
    frame.render_widget(Clear, area);
    let batch = execution
        .auto_answer
        .as_ref()
        .and_then(|lanes| lanes.iter().map(|lane| lane.batch).max())
        .unwrap_or(1);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(
                format!("AUTO ANSWER · BATCH {batch}"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from("Three fixed workers research different questions before one judge review."),
        ])
        .block(Block::default().borders(Borders::ALL)),
        rows[0],
    );
    let mut lanes = execution.auto_answer.clone().unwrap_or_default();
    lanes.sort_by_key(|lane| match lane.actor {
        AutoAnswerActor::Coordinator => 0,
        AutoAnswerActor::Worker { slot, .. } => slot,
        AutoAnswerActor::Judge => 4,
    });
    let items = lanes.into_iter().map(|lane| {
        let actor = match lane.actor {
            AutoAnswerActor::Coordinator => "Coordinator".to_owned(),
            AutoAnswerActor::Worker {
                slot,
                question_display_id,
            } => format!("Worker {slot}  {question_display_id}"),
            AutoAnswerActor::Judge => "Judge".to_owned(),
        };
        let evidence = lane
            .evidence_count
            .map_or_else(String::new, |count| format!(" · {count} cited source(s)"));
        ListItem::new(vec![
            Line::from(format!("{actor}  {}{evidence}", lane.stage.label())),
            Line::from(format!("  {}", lane.detail)),
        ])
    });
    frame.render_widget(
        List::new(items).block(
            Block::default()
                .title(" PARALLEL CODEX PROGRESS ")
                .borders(Borders::ALL),
        ),
        rows[1],
    );
    frame.render_widget(
        Paragraph::new("Esc cancel all work · no provisional answer is authoritative")
            .block(Block::default().borders(Borders::ALL)),
        rows[2],
    );
}

/// Renders the structured ask overlay and its visible conservative defaults.
fn render_question_draft(frame: &mut Frame<'_>, state: &WorkspaceState) {
    let Some(draft) = state.question_draft.as_ref() else {
        return;
    };
    let area = centered_rect(76, 70, frame.area());
    let marker = |field| if draft.field == field { ">" } else { " " };
    let body = vec![
        Line::from(format!(
            "{} Prompt: {}",
            marker(QuestionDraftField::Prompt),
            draft.prompt
        )),
        Line::from(""),
        Line::from(format!(
            "{} Rationale: {}",
            marker(QuestionDraftField::Rationale),
            draft.rationale
        )),
        Line::from(""),
        Line::from(format!(
            "{} Impact: {:?}",
            marker(QuestionDraftField::Impact),
            draft.impact
        )),
        Line::from(format!(
            "{} Uncertainty: {:?}",
            marker(QuestionDraftField::Uncertainty),
            draft.uncertainty
        )),
        Line::from(format!(
            "{} Cost of being wrong: {:?}",
            marker(QuestionDraftField::Cost),
            draft.cost
        )),
        Line::from(""),
        Line::from("Tab next field · ←→ change choice · Enter create · Esc cancel"),
    ];
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(body).wrap(Wrap { trim: false }).block(
            Block::default()
                .title(" ASK A PROJECT QUESTION ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        ),
        area,
    );
}

/// Calculates a centered percentage-based overlay rectangle inside the terminal area.
fn centered_rect(width_percent: u16, height_percent: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_percent) / 2),
            Constraint::Percentage(height_percent),
            Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_percent) / 2),
            Constraint::Percentage(width_percent),
            Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(vertical[1])[1]
}

/// Highlights the border of the currently focused workbench surface.
fn focus_style(active: bool) -> Style {
    if active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    }
}

/// Formats one operational activity row with its stable category marker.
fn activity_line(event: &ActivityEvent) -> String {
    let marker = match event.kind {
        ActivityKind::Lifecycle => "✓",
        ActivityKind::Progress => "•",
        ActivityKind::Warning => "!",
    };
    format!("{marker} {}", event.message)
}

/// Cycles the closed impact vocabulary in either direction.
fn cycle_impact(value: Impact, reverse: bool) -> Impact {
    match (value, reverse) {
        (Impact::Low, false) | (Impact::High, true) => Impact::Medium,
        (Impact::Medium, false) => Impact::High,
        (Impact::Medium, true) => Impact::Low,
        (Impact::High, false) => Impact::Low,
        (Impact::Low, true) => Impact::High,
    }
}

/// Cycles the closed uncertainty vocabulary in either direction.
fn cycle_uncertainty(value: Uncertainty, reverse: bool) -> Uncertainty {
    match (value, reverse) {
        (Uncertainty::Low, false) | (Uncertainty::High, true) => Uncertainty::Medium,
        (Uncertainty::Medium, false) => Uncertainty::High,
        (Uncertainty::Medium, true) => Uncertainty::Low,
        (Uncertainty::High, false) => Uncertainty::Low,
        (Uncertainty::Low, true) => Uncertainty::High,
    }
}

/// Cycles the closed rework-cost vocabulary in either direction.
fn cycle_cost(value: CostOfBeingWrong, reverse: bool) -> CostOfBeingWrong {
    match (value, reverse) {
        (CostOfBeingWrong::Low, false) | (CostOfBeingWrong::High, true) => CostOfBeingWrong::Medium,
        (CostOfBeingWrong::Medium, false) => CostOfBeingWrong::High,
        (CostOfBeingWrong::Medium, true) => CostOfBeingWrong::Low,
        (CostOfBeingWrong::High, false) => CostOfBeingWrong::Low,
        (CostOfBeingWrong::Low, true) => CostOfBeingWrong::High,
    }
}
