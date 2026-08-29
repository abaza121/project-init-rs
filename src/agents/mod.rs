//! Provider-neutral agent execution contracts and the isolated Codex CLI adapter.

mod codex;

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{Notify, mpsc};
use uuid::Uuid;

use crate::domain::EvidenceReliability;

pub use codex::{
    CodexCliClient, CodexCliConfig, decode_codex_jsonl_event, resolve_codex_executable,
};

const MAX_ACTIVITY_MESSAGE_CHARS: usize = 512;

/// Classifies safe operational progress independently of a model provider's event vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityKind {
    /// Marks a stable application or agent lifecycle boundary.
    Lifecycle,
    /// Reports non-sensitive work progress inside the current lifecycle stage.
    Progress,
    /// Surfaces a recoverable problem without exposing raw provider diagnostics.
    Warning,
}

impl ActivityKind {
    /// Returns the checked representation stored in SQLite.
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::Lifecycle => "lifecycle",
            Self::Progress => "progress",
            Self::Warning => "warning",
        }
    }

    /// Parses one checked persisted activity category.
    pub(crate) fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "lifecycle" => Some(Self::Lifecycle),
            "progress" => Some(Self::Progress),
            "warning" => Some(Self::Warning),
            _ => None,
        }
    }
}

/// Represents one sanitized, ordered status entry suitable for display and persistence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub sequence: u32,
    pub kind: ActivityKind,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

impl ActivityEvent {
    /// Creates a timestamped activity event after removing unsafe control characters.
    pub fn now(sequence: u32, kind: ActivityKind, message: &str) -> Self {
        Self {
            sequence,
            kind,
            message: sanitize_terminal_text(message),
            created_at: Utc::now(),
        }
    }
}

/// Retains a bounded suffix of an agent timeline while counting discarded older entries.
#[derive(Debug, Clone)]
pub struct ActivityHistory {
    capacity: usize,
    events: VecDeque<ActivityEvent>,
    dropped: u64,
}

impl ActivityHistory {
    /// Creates a bounded history and rejects a zero-sized buffer.
    pub fn new(capacity: usize) -> Result<Self, AgentError> {
        if capacity == 0 {
            return Err(AgentError::InvalidHistoryCapacity);
        }
        Ok(Self::bounded(capacity))
    }

    /// Creates an infallible history for internal callers by clamping the bound to one.
    pub(crate) fn bounded(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Self {
            capacity,
            events: VecDeque::with_capacity(capacity),
            dropped: 0,
        }
    }

    /// Appends an event and evicts the oldest entry when the bound is full.
    pub fn push(&mut self, event: ActivityEvent) {
        if self.events.len() == self.capacity {
            self.events.pop_front();
            self.dropped += 1;
        }
        self.events.push_back(event);
    }

    /// Returns retained events in their original sequence order.
    pub fn events(&self) -> Vec<ActivityEvent> {
        self.events.iter().cloned().collect()
    }

    /// Returns the number of entries currently retained in the bounded history.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns whether the bounded history currently retains no entries.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns how many older events were removed to preserve the configured bound.
    pub const fn dropped_count(&self) -> u64 {
        self.dropped
    }
}

/// Contains one successful provider execution ready for workflow validation and persistence.
#[derive(Debug, Clone)]
pub struct AgentExecution {
    pub id: String,
    pub provider: String,
    pub model: Option<String>,
    pub response: String,
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub activity: Vec<ActivityEvent>,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

/// Contains validated focused context for one initial-brief analysis operation.
#[derive(Debug, Clone)]
pub struct AnalysisRequest {
    pub project_name: String,
    pub brief: String,
}

impl AnalysisRequest {
    /// Creates a bounded request before any paid or external work begins.
    pub fn new(project_name: &str, brief: &str) -> Result<Self, AgentError> {
        let project_name = project_name.trim();
        let brief = brief.trim();
        if project_name.is_empty() || brief.is_empty() {
            return Err(AgentError::InvalidRequest(
                "project name and brief must not be blank".to_owned(),
            ));
        }
        if project_name.chars().count() > 256 || brief.chars().count() > 65_536 {
            return Err(AgentError::InvalidRequest(
                "project name or brief exceeds the supported size".to_owned(),
            ));
        }
        Ok(Self {
            project_name: project_name.to_owned(),
            brief: brief.to_owned(),
        })
    }
}

/// Contains the bounded project context needed to research one blocking clarification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchRequest {
    snapshot_json: String,
    question_id: String,
    question_prompt: String,
    question_rationale: String,
}

impl ResearchRequest {
    /// Creates a focused research request without granting the provider decision authority.
    pub fn new(
        snapshot_json: String,
        question_id: &str,
        question_prompt: &str,
        question_rationale: &str,
    ) -> Result<Self, AgentError> {
        let question_id = question_id.trim();
        let question_prompt = question_prompt.trim();
        let question_rationale = question_rationale.trim();
        if snapshot_json.trim().is_empty()
            || question_id.is_empty()
            || question_prompt.is_empty()
            || question_rationale.is_empty()
        {
            return Err(AgentError::InvalidRequest(
                "research context and question fields must not be blank".to_owned(),
            ));
        }
        if snapshot_json.len() > 1024 * 1024
            || question_id.chars().count() > 256
            || question_prompt.chars().count() > 16_384
            || question_rationale.chars().count() > 16_384
        {
            return Err(AgentError::InvalidRequest(
                "research context exceeds the supported size".to_owned(),
            ));
        }
        Ok(Self {
            snapshot_json,
            question_id: question_id.to_owned(),
            question_prompt: question_prompt.to_owned(),
            question_rationale: question_rationale.to_owned(),
        })
    }

    /// Returns the serialized authoritative project snapshot supplied as inert context.
    pub fn snapshot_json(&self) -> &str {
        &self.snapshot_json
    }

    /// Returns the stable identity of the blocking question.
    pub fn question_id(&self) -> &str {
        &self.question_id
    }

    /// Returns the user-visible clarification prompt to research.
    pub fn question_prompt(&self) -> &str {
        &self.question_prompt
    }

    /// Returns why an unsupported answer could materially affect the project.
    pub fn question_rationale(&self) -> &str {
        &self.question_rationale
    }
}

/// Describes one externally attributable claim supporting an automatic answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResearchEvidence {
    claim: String,
    source: String,
    source_title: String,
    reliability: EvidenceReliability,
    notes: Option<String>,
}

impl ResearchEvidence {
    /// Creates one validated citation suitable for an automatic recommendation.
    pub fn new(
        claim: &str,
        source: &str,
        source_title: &str,
        reliability: EvidenceReliability,
        notes: Option<&str>,
    ) -> Result<Self, AgentError> {
        let evidence = Self {
            claim: claim.trim().to_owned(),
            source: source.trim().to_owned(),
            source_title: source_title.trim().to_owned(),
            reliability,
            notes: notes.map(str::trim).map(ToOwned::to_owned),
        };
        validate_research_evidence(&evidence)?;
        Ok(evidence)
    }

    /// Returns the externally supported claim.
    pub fn claim(&self) -> &str {
        &self.claim
    }

    /// Returns the direct HTTPS citation.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the human-readable source title.
    pub fn source_title(&self) -> &str {
        &self.source_title
    }

    /// Returns the assessed source reliability.
    pub const fn reliability(&self) -> EvidenceReliability {
        self.reliability
    }

    /// Returns optional limitations or interpretation notes.
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}

/// Contains one provider recommendation and the evidence required to audit it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResearchedAnswer {
    answer_text: String,
    notes: Option<String>,
    evidence: Vec<ResearchEvidence>,
}

impl ResearchedAnswer {
    /// Creates one validated recommendation from already validated cited evidence.
    pub fn new(
        answer_text: &str,
        notes: Option<&str>,
        evidence: Vec<ResearchEvidence>,
    ) -> Result<Self, AgentError> {
        let answer = Self {
            answer_text: answer_text.trim().to_owned(),
            notes: notes.map(str::trim).map(ToOwned::to_owned),
            evidence,
        };
        validate_researched_answer(&answer)?;
        Ok(answer)
    }

    /// Parses and validates an untrusted structured provider response before persistence.
    pub fn from_json(json: &str) -> Result<Self, AgentError> {
        let unchecked: UncheckedResearchedAnswer = serde_json::from_str(json).map_err(|error| {
            AgentError::InvalidResponse(format!("research answer is not valid JSON: {error}"))
        })?;
        Self::from_unchecked(unchecked)
    }

    /// Validates one deserialized recommendation shared by worker and judge response formats.
    fn from_unchecked(unchecked: UncheckedResearchedAnswer) -> Result<Self, AgentError> {
        let evidence = unchecked
            .evidence
            .into_iter()
            .map(|item| {
                ResearchEvidence::new(
                    &item.claim,
                    &item.source,
                    &item.source_title,
                    item.reliability,
                    item.notes.as_deref(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Self::new(&unchecked.answer_text, unchecked.notes.as_deref(), evidence)
    }

    /// Returns the evidence-backed recommendation text.
    pub fn answer_text(&self) -> &str {
        &self.answer_text
    }

    /// Returns provider notes that distinguish evidence from inference.
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Returns the citations supporting this recommendation.
    pub fn evidence(&self) -> &[ResearchEvidence] {
        &self.evidence
    }
}

/// Describes one eligible consequential question supplied to the batch coordinator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResearchQuestionContext {
    question_id: String,
    question_display_id: String,
    prompt: String,
    rationale: String,
}

impl ResearchQuestionContext {
    /// Creates a bounded coordinator question without transferring workflow authority.
    pub fn new(
        question_id: &str,
        question_display_id: &str,
        prompt: &str,
        rationale: &str,
    ) -> Result<Self, AgentError> {
        let context = Self {
            question_id: question_id.trim().to_owned(),
            question_display_id: question_display_id.trim().to_owned(),
            prompt: prompt.trim().to_owned(),
            rationale: rationale.trim().to_owned(),
        };
        validate_research_text("question id", &context.question_id, 256)?;
        validate_research_text("question display id", &context.question_display_id, 256)?;
        validate_research_text("question prompt", &context.prompt, 16_384)?;
        validate_research_text("question rationale", &context.rationale, 16_384)?;
        Ok(context)
    }

    /// Returns the durable question identity used for authoritative reconciliation.
    pub fn question_id(&self) -> &str {
        &self.question_id
    }

    /// Returns the concise display identity shown in progress lanes.
    pub fn question_display_id(&self) -> &str {
        &self.question_display_id
    }

    /// Returns the user-visible clarification prompt.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Returns the explanation of why this uncertainty matters.
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

/// Contains the bounded project and question context used to plan one research batch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResearchPlanRequest {
    snapshot_json: String,
    blocking_question_id: String,
    questions: Vec<ResearchQuestionContext>,
}

impl ResearchPlanRequest {
    /// Creates a coordinator request whose immediate blocker is part of the eligible set.
    pub fn new(
        snapshot_json: String,
        blocking_question_id: &str,
        questions: Vec<ResearchQuestionContext>,
    ) -> Result<Self, AgentError> {
        let blocking_question_id = blocking_question_id.trim().to_owned();
        validate_research_text("planning snapshot", &snapshot_json, 1024 * 1024)?;
        validate_research_text("blocking question id", &blocking_question_id, 256)?;
        if questions.is_empty() || questions.len() > 128 {
            return Err(AgentError::InvalidRequest(
                "research planning requires between one and 128 eligible questions".to_owned(),
            ));
        }
        if !questions
            .iter()
            .any(|question| question.question_id == blocking_question_id)
        {
            return Err(AgentError::InvalidRequest(
                "research planning must include the blocking question".to_owned(),
            ));
        }
        let unique = questions
            .iter()
            .map(|question| question.question_id.as_str())
            .collect::<std::collections::HashSet<_>>();
        if unique.len() != questions.len() {
            return Err(AgentError::InvalidRequest(
                "research planning question identities must be unique".to_owned(),
            ));
        }
        Ok(Self {
            snapshot_json,
            blocking_question_id,
            questions,
        })
    }

    /// Returns the inert serialized authoritative project snapshot.
    pub fn snapshot_json(&self) -> &str {
        &self.snapshot_json
    }

    /// Returns the workflow edge that every valid plan must include.
    pub fn blocking_question_id(&self) -> &str {
        &self.blocking_question_id
    }

    /// Returns all consequential open questions eligible for this plan.
    pub fn questions(&self) -> &[ResearchQuestionContext] {
        &self.questions
    }
}

/// Selects at most three unique questions for one concurrent worker batch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResearchBatchPlan {
    question_ids: Vec<String>,
}

impl ResearchBatchPlan {
    /// Creates a structurally bounded plan before project-specific membership validation.
    pub fn new(question_ids: Vec<String>) -> Result<Self, AgentError> {
        if question_ids.is_empty() || question_ids.len() > 3 {
            return Err(AgentError::InvalidResponse(
                "research plan requires between one and three questions".to_owned(),
            ));
        }
        for question_id in &question_ids {
            validate_research_text("planned question id", question_id, 256)?;
        }
        let unique = question_ids
            .iter()
            .map(String::as_str)
            .collect::<std::collections::HashSet<_>>();
        if unique.len() != question_ids.len() {
            return Err(AgentError::InvalidResponse(
                "research plan question identities must be unique".to_owned(),
            ));
        }
        Ok(Self { question_ids })
    }

    /// Parses an untrusted plan and validates it against eligible and blocking identities.
    pub fn from_json(
        json: &str,
        eligible_question_ids: &[String],
        blocking_question_id: &str,
    ) -> Result<Self, AgentError> {
        let unchecked: Self = serde_json::from_str(json).map_err(|error| {
            AgentError::InvalidResponse(format!("research plan is not valid JSON: {error}"))
        })?;
        let plan = Self::new(unchecked.question_ids)?;
        plan.validate_for(eligible_question_ids, blocking_question_id)?;
        Ok(plan)
    }

    /// Confirms provider-selected identities are eligible and include the current blocker.
    pub fn validate_for(
        &self,
        eligible_question_ids: &[String],
        blocking_question_id: &str,
    ) -> Result<(), AgentError> {
        if !self
            .question_ids
            .iter()
            .any(|question_id| question_id == blocking_question_id)
        {
            return Err(AgentError::InvalidResponse(
                "research plan omitted the blocking question".to_owned(),
            ));
        }
        if self
            .question_ids
            .iter()
            .any(|question_id| !eligible_question_ids.contains(question_id))
        {
            return Err(AgentError::InvalidResponse(
                "research plan selected an ineligible question".to_owned(),
            ));
        }
        Ok(())
    }

    /// Returns selected durable question identities in coordinator order.
    pub fn question_ids(&self) -> &[String] {
        &self.question_ids
    }
}

/// Pairs one worker's validated cited answer with its durable question identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResearchCandidate {
    question_id: String,
    answer: ResearchedAnswer,
}

impl ResearchCandidate {
    /// Creates one provisional worker candidate for later batch judgment.
    pub fn new(question_id: &str, answer: ResearchedAnswer) -> Result<Self, AgentError> {
        validate_research_text("candidate question id", question_id, 256)?;
        Ok(Self {
            question_id: question_id.to_owned(),
            answer,
        })
    }

    /// Returns the durable question identity associated with the candidate.
    pub fn question_id(&self) -> &str {
        &self.question_id
    }

    /// Returns the validated provisional cited answer.
    pub const fn answer(&self) -> &ResearchedAnswer {
        &self.answer
    }
}

/// Contains all successful worker candidates supplied to one project-aware judge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResearchJudgmentRequest {
    snapshot_json: String,
    candidates: Vec<ResearchCandidate>,
}

impl ResearchJudgmentRequest {
    /// Creates a bounded exact candidate set for one judge invocation.
    pub fn new(
        snapshot_json: String,
        candidates: Vec<ResearchCandidate>,
    ) -> Result<Self, AgentError> {
        validate_research_text("judgment snapshot", &snapshot_json, 1024 * 1024)?;
        if candidates.is_empty() || candidates.len() > 3 {
            return Err(AgentError::InvalidRequest(
                "research judgment requires between one and three candidates".to_owned(),
            ));
        }
        let unique = candidates
            .iter()
            .map(|candidate| candidate.question_id.as_str())
            .collect::<std::collections::HashSet<_>>();
        if unique.len() != candidates.len() {
            return Err(AgentError::InvalidRequest(
                "research judgment candidate identities must be unique".to_owned(),
            ));
        }
        Ok(Self {
            snapshot_json,
            candidates,
        })
    }

    /// Returns the inert serialized authoritative project snapshot.
    pub fn snapshot_json(&self) -> &str {
        &self.snapshot_json
    }

    /// Returns every provisional candidate the judge must address exactly once.
    pub fn candidates(&self) -> &[ResearchCandidate] {
        &self.candidates
    }
}

/// Associates one judge-approved cited answer with its authoritative question identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JudgedResearchAnswer {
    question_id: String,
    answer: ResearchedAnswer,
}

impl JudgedResearchAnswer {
    /// Creates one validated judge-approved answer.
    pub fn new(question_id: &str, answer: ResearchedAnswer) -> Result<Self, AgentError> {
        validate_research_text("judged question id", question_id, 256)?;
        Ok(Self {
            question_id: question_id.to_owned(),
            answer,
        })
    }

    /// Returns the durable question identity selected by the judge.
    pub fn question_id(&self) -> &str {
        &self.question_id
    }

    /// Returns the final cited recommendation approved by the judge.
    pub const fn answer(&self) -> &ResearchedAnswer {
        &self.answer
    }
}

/// Contains the exact set of final cited answers returned by one batch judge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JudgedResearchBatch {
    answers: Vec<JudgedResearchAnswer>,
}

impl JudgedResearchBatch {
    /// Creates a bounded batch whose question identities are unique.
    pub fn new(answers: Vec<JudgedResearchAnswer>) -> Result<Self, AgentError> {
        if answers.is_empty() || answers.len() > 3 {
            return Err(AgentError::InvalidResponse(
                "judged research requires between one and three answers".to_owned(),
            ));
        }
        let unique = answers
            .iter()
            .map(|answer| answer.question_id.as_str())
            .collect::<std::collections::HashSet<_>>();
        if unique.len() != answers.len() {
            return Err(AgentError::InvalidResponse(
                "judged answer question identities must be unique".to_owned(),
            ));
        }
        Ok(Self { answers })
    }

    /// Parses untrusted judge JSON and requires exact candidate membership.
    pub fn from_json(json: &str, candidate_ids: &[String]) -> Result<Self, AgentError> {
        let unchecked: UncheckedJudgedResearchBatch =
            serde_json::from_str(json).map_err(|error| {
                AgentError::InvalidResponse(format!("judged research is not valid JSON: {error}"))
            })?;
        let answers = unchecked
            .answers
            .into_iter()
            .map(|item| {
                JudgedResearchAnswer::new(
                    &item.question_id,
                    ResearchedAnswer::from_unchecked(item.answer)?,
                )
            })
            .collect::<Result<Vec<_>, AgentError>>()?;
        let batch = Self::new(answers)?;
        batch.validate_for(candidate_ids)?;
        Ok(batch)
    }

    /// Confirms a constructed batch addresses every candidate identity exactly once.
    pub fn validate_for(&self, candidate_ids: &[String]) -> Result<(), AgentError> {
        let actual = self
            .answers
            .iter()
            .map(|answer| answer.question_id.as_str())
            .collect::<std::collections::HashSet<_>>();
        let expected = candidate_ids
            .iter()
            .map(String::as_str)
            .collect::<std::collections::HashSet<_>>();
        if actual != expected || self.answers.len() != candidate_ids.len() {
            return Err(AgentError::InvalidResponse(
                "judged research must answer every candidate exactly once".to_owned(),
            ));
        }
        Ok(())
    }

    /// Returns final judged answers in provider order.
    pub fn answers(&self) -> &[JudgedResearchAnswer] {
        &self.answers
    }
}

/// Mirrors one untrusted judge answer before nested citation validation.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct UncheckedJudgedResearchAnswer {
    question_id: String,
    #[serde(flatten)]
    answer: UncheckedResearchedAnswer,
}

/// Mirrors one untrusted judge batch before exact membership validation.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct UncheckedJudgedResearchBatch {
    answers: Vec<UncheckedJudgedResearchAnswer>,
}

/// Mirrors one untrusted provider citation before constructor validation.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct UncheckedResearchEvidence {
    claim: String,
    source: String,
    source_title: String,
    reliability: EvidenceReliability,
    notes: Option<String>,
}

/// Mirrors one untrusted provider answer before its fields become publicly usable.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct UncheckedResearchedAnswer {
    answer_text: String,
    notes: Option<String>,
    evidence: Vec<UncheckedResearchEvidence>,
}

/// Validates the bounded text and citation invariants for one researched answer.
fn validate_researched_answer(answer: &ResearchedAnswer) -> Result<(), AgentError> {
    validate_research_text("answer", &answer.answer_text, 16_384)?;
    if let Some(notes) = &answer.notes {
        validate_research_text("answer notes", notes, 16_384)?;
    }
    if answer.evidence.is_empty() {
        return Err(AgentError::InvalidResponse(
            "research answer requires at least one evidence source".to_owned(),
        ));
    }
    if answer.evidence.len() > 16 {
        return Err(AgentError::InvalidResponse(
            "research answer exceeds the evidence source limit".to_owned(),
        ));
    }
    for evidence in &answer.evidence {
        validate_research_evidence(evidence)?;
    }
    Ok(())
}

/// Validates one provider-controlled citation before it can support a decision.
fn validate_research_evidence(evidence: &ResearchEvidence) -> Result<(), AgentError> {
    validate_research_text("evidence claim", &evidence.claim, 16_384)?;
    validate_research_text("evidence source", &evidence.source, 2_048)?;
    validate_research_text("evidence source title", &evidence.source_title, 1_024)?;
    if let Some(notes) = &evidence.notes {
        validate_research_text("evidence notes", notes, 16_384)?;
    }
    if !evidence.source.starts_with("https://") {
        return Err(AgentError::InvalidResponse(
            "research evidence source must be an HTTPS URL".to_owned(),
        ));
    }
    Ok(())
}

/// Checks one provider-controlled field without silently truncating authoritative content.
fn validate_research_text(label: &str, value: &str, maximum: usize) -> Result<(), AgentError> {
    if value.trim().is_empty() {
        return Err(AgentError::InvalidResponse(format!(
            "research {label} must not be blank"
        )));
    }
    if value.chars().count() > maximum {
        return Err(AgentError::InvalidResponse(format!(
            "research {label} exceeds the supported size"
        )));
    }
    Ok(())
}

/// Provides cloneable cancellation without coupling callers to a channel ownership model.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    state: Arc<CancellationState>,
}

/// Stores the atomic cancellation flag and wake-up primitive shared by token clones.
#[derive(Debug, Default)]
struct CancellationState {
    cancelled: AtomicBool,
    notify: Notify,
}

impl CancellationToken {
    /// Creates a token in the running state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancellation once and wakes all current or future waiters.
    pub fn cancel(&self) {
        if !self.state.cancelled.swap(true, Ordering::AcqRel) {
            self.state.notify.notify_waiters();
        }
    }

    /// Returns whether cancellation has already been requested.
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }

    /// Waits until cancellation is requested without missing an earlier signal.
    pub async fn cancelled(&self) {
        loop {
            let notified = self.state.notify.notified();
            if self.is_cancelled() {
                return;
            }
            notified.await;
        }
    }
}

/// Defines the provider-neutral asynchronous boundary used by workflow and the TUI.
#[async_trait]
pub trait AgentClient: Send + Sync {
    /// Executes one focused analysis while streaming safe activity to the caller.
    async fn analyze(
        &self,
        request: AnalysisRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<AgentExecution, AgentError>;

    /// Returns the maximum duration this client allows one analysis to run.
    fn timeout(&self) -> Duration;
}

/// Defines the provider boundary for cited, read-only clarification research.
#[async_trait]
pub trait ResearchClient: Send + Sync {
    /// Researches one blocking question and returns a validated evidence-backed recommendation.
    async fn research(
        &self,
        request: ResearchRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError>;
}

/// Defines coordinator, worker, and judge operations for parallel automatic clarification.
#[async_trait]
pub trait AutoAnswerClient: Send + Sync {
    /// Selects a dependency-safe batch containing the current blocking question.
    async fn plan(
        &self,
        request: ResearchPlanRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchBatchPlan, AgentError>;

    /// Researches one selected question and returns a provisional cited recommendation.
    async fn research(
        &self,
        request: ResearchRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError>;

    /// Reviews all worker candidates and returns an exact final cited batch.
    async fn judge(
        &self,
        request: ResearchJudgmentRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<JudgedResearchBatch, AgentError>;
}

/// Distinguishes initial package generation from an explicitly authorized repair attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentationKind {
    /// Creates every expected artifact from authoritative project state.
    Generate,
    /// Revises a staged copy in response to persisted validation findings.
    Repair,
}

/// Owns the complete provider-neutral input for one staged documentation operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentationRequest {
    pub kind: DocumentationKind,
    pub snapshot_json: String,
    pub required_paths: Vec<String>,
    pub staging: PathBuf,
    pub repair_findings_json: Option<String>,
}

impl DocumentationRequest {
    /// Creates a generation request whose output remains provisional inside staging.
    pub fn generation(
        snapshot_json: String,
        required_paths: Vec<String>,
        staging: PathBuf,
    ) -> Self {
        Self {
            kind: DocumentationKind::Generate,
            snapshot_json,
            required_paths,
            staging,
            repair_findings_json: None,
        }
    }

    /// Creates a repair request containing only serialized, sanitized validation context.
    pub fn repair(
        snapshot_json: String,
        required_paths: Vec<String>,
        staging: PathBuf,
        repair_findings_json: String,
    ) -> Self {
        Self {
            kind: DocumentationKind::Repair,
            snapshot_json,
            required_paths,
            staging,
            repair_findings_json: Some(repair_findings_json),
        }
    }
}

/// Defines the asynchronous provider boundary for staged package generation and repair.
#[async_trait]
pub trait DocumentationClient: Send + Sync {
    /// Executes one request without treating produced files as accepted project state.
    async fn execute(
        &self,
        request: DocumentationRequest,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError>;
}

impl AgentExecution {
    /// Creates a completed Codex execution for deterministic clients and validated adapters.
    pub fn new(response: &str, activity: Vec<ActivityEvent>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            provider: "codex_cli".to_owned(),
            model: None,
            response: response.to_owned(),
            input_tokens: None,
            output_tokens: None,
            activity,
            started_at: now,
            completed_at: now,
        }
    }
}

/// Describes a safe, recoverable failure at the external agent boundary.
#[derive(Debug, Error)]
pub enum AgentError {
    /// Rejects invalid focused context before an external invocation is attempted.
    #[error("invalid Codex request: {0}")]
    InvalidRequest(String),
    /// Rejects malformed, unsupported, or uncited provider output.
    #[error("invalid Codex response: {0}")]
    InvalidResponse(String),
    /// Rejects a history buffer that cannot retain any activity.
    #[error("activity history capacity must be greater than zero")]
    InvalidHistoryCapacity,
    /// Rejects malformed JSONL received from the Codex subprocess.
    #[error("Codex emitted malformed JSONL: {0}")]
    InvalidEvent(String),
    /// Reports a bounded subprocess or response failure without including credentials.
    #[error("Codex execution failed: {0}")]
    Execution(String),
    /// Reports immediate cancellation requested by the terminal user.
    #[error("Codex execution was cancelled")]
    Cancelled,
    /// Reports the configured analysis deadline being reached.
    #[error("Codex analysis timed out")]
    TimedOut,
    /// Reports a documentation operation that stopped emitting observable progress.
    #[error("Codex produced no output before the inactivity timeout")]
    Inactive,
}

/// Removes terminal control characters and bounds text before display or persistence.
pub(crate) fn sanitize_terminal_text(message: &str) -> String {
    message
        .chars()
        .filter(|character| !character.is_control() || matches!(character, ' ' | '\t'))
        .take(MAX_ACTIVITY_MESSAGE_CHARS)
        .collect::<String>()
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{CancellationToken, ResearchedAnswer};

    /// Accepts a bounded cited answer returned by an untrusted research provider.
    #[test]
    fn researched_answer_requires_valid_cited_evidence() {
        let answer = ResearchedAnswer::from_json(
            r#"{
                "answer_text":"Target WebAssembly in modern evergreen browsers.",
                "notes":"The recommendation favors reach over native-only APIs.",
                "evidence":[{
                    "claim":"WebAssembly is supported by modern browser engines.",
                    "source":"https://developer.mozilla.org/en-US/docs/WebAssembly",
                    "source_title":"WebAssembly",
                    "reliability":"high",
                    "notes":null
                }]
            }"#,
        )
        .expect("a cited provider answer should validate");

        assert_eq!(answer.evidence().len(), 1);
        assert_eq!(answer.evidence()[0].source_title(), "WebAssembly");
    }

    /// Rejects unsupported answers rather than converting model opinion into authority.
    #[test]
    fn researched_answer_rejects_missing_evidence() {
        let error = ResearchedAnswer::from_json(
            r#"{"answer_text":"Choose WebAssembly.","notes":null,"evidence":[]}"#,
        )
        .expect_err("automatic answers must include external evidence");

        assert!(error.to_string().contains("at least one evidence source"));
    }

    /// Rejects insecure citations before any external claim reaches project storage.
    #[test]
    fn researched_answer_rejects_non_https_evidence() {
        let error = ResearchedAnswer::from_json(
            r#"{
                "answer_text":"Choose WebAssembly.",
                "notes":null,
                "evidence":[{
                    "claim":"A claim.",
                    "source":"http://example.com/source",
                    "source_title":"Example",
                    "reliability":"low",
                    "notes":null
                }]
            }"#,
        )
        .expect_err("insecure citations must not become authoritative evidence");

        assert!(error.to_string().contains("HTTPS URL"));
    }

    /// Wakes every clone when terminal input requests immediate cancellation.
    #[tokio::test]
    async fn cancellation_token_notifies_waiters() {
        let token = CancellationToken::new();
        let waiter = token.clone();

        token.cancel();

        waiter.cancelled().await;
        assert!(waiter.is_cancelled());
    }
}
