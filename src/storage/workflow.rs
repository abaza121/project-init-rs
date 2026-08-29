use chrono::Utc;
use rusqlite::{OptionalExtension, params};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::agents::{
    ActivityEvent, ActivityKind, AgentExecution, JudgedResearchBatch, ResearchEvidence,
};
use crate::domain::error::normalize_required_text;
use crate::domain::{
    Answer, AnswerSource, Confidence, CostOfBeingWrong, Finding, FindingKind, Impact, NewFinding,
    NewProject, Project, ProjectId, ProjectSnapshot, ProjectStatus, Question, QuestionPriority,
    QuestionStatus, Requirement, RequirementStatus, SourceType, TraceLink, TraceRelationship,
    Uncertainty,
};

use super::sqlite::{
    allocate_display_id, ensure_project_exists, insert_finding, insert_project, parse_timestamp,
};
use super::{SqliteStore, StorageError};

/// Carries one validated analysis proposal into the atomic persistence boundary.
pub(crate) struct AnalyzedFindingInput {
    pub kind: FindingKind,
    pub statement: String,
    pub source_type: SourceType,
    pub confidence: Confidence,
    pub impact: Impact,
    pub requires_confirmation: bool,
    pub clarification: Option<ClarificationInput>,
}

/// Carries user-facing question copy associated with one proposed unknown.
pub(crate) struct ClarificationInput {
    pub prompt: String,
    pub rationale: String,
}

impl SqliteStore {
    /// Commits a validated analysis and optional successful agent history in one transaction.
    pub(crate) fn create_analyzed_project(
        &mut self,
        input: NewProject,
        findings: Vec<AnalyzedFindingInput>,
        execution: Option<&AgentExecution>,
    ) -> Result<Project, StorageError> {
        let mut project = Project::from_new(input);
        project.transition_to(ProjectStatus::Analyzing)?;
        let next = if findings
            .iter()
            .any(|finding| finding.clarification.is_some())
        {
            ProjectStatus::AwaitingClarification
        } else {
            ProjectStatus::Planning
        };
        project.transition_to(next)?;
        let token_counts = execution.map(agent_token_counts).transpose()?;
        let transaction = self.connection.transaction()?;

        // All stable IDs and operational history below are invisible until the final commit.
        insert_project(&transaction, &project)?;
        for input in findings {
            let new_finding = NewFinding::new(
                project.id().clone(),
                input.kind,
                &input.statement,
                input.source_type,
                "analysis",
                input.confidence,
                input.impact,
                input.requires_confirmation,
            )?;
            let display_id =
                allocate_display_id(&transaction, project.id(), input.kind.display_prefix())?;
            let finding = Finding::from_new(new_finding, display_id);
            insert_finding(&transaction, &finding)?;
            if let Some(clarification) = input.clarification {
                let question = build_question(
                    project.id(),
                    Some(finding.id().as_str()),
                    &clarification.prompt,
                    &clarification.rationale,
                    input.impact,
                    Uncertainty::High,
                    CostOfBeingWrong::High,
                    &transaction,
                )?;
                insert_question(&transaction, &question)?;
            }
        }
        if let Some(execution) = execution {
            insert_agent_execution(&transaction, project.id(), execution, token_counts)?;
        }
        transaction.commit()?;
        Ok(project)
    }

    /// Counts projects without loading their briefs into memory.
    pub fn project_count(&self) -> Result<u64, StorageError> {
        let count = self
            .connection
            .query_row("SELECT COUNT(*) FROM projects", [], |row| {
                row.get::<_, i64>(0)
            })?;
        u64::try_from(count).map_err(|_| StorageError::CorruptData {
            field: "project count",
            value: count.to_string(),
        })
    }

    /// Applies and persists one legal lifecycle transition atomically.
    pub fn transition_project(
        &mut self,
        project_id: &ProjectId,
        next: ProjectStatus,
    ) -> Result<(), StorageError> {
        let mut project = self
            .get_project(project_id)?
            .ok_or_else(|| StorageError::ProjectNotFound(project_id.to_string()))?;
        project.transition_to(next)?;
        self.connection.execute(
            "UPDATE projects SET status = ?2, updated_at = ?3, revision = revision + 1 WHERE id = ?1",
            params![project_id.as_str(), next.as_db_str(), project.updated_at().to_rfc3339()],
        )?;
        Ok(())
    }

    /// Validates and persists the project-specific consequential-question threshold atomically.
    pub fn set_clarification_threshold(
        &mut self,
        project_id: &ProjectId,
        value: u16,
    ) -> Result<(), StorageError> {
        let mut project = self
            .get_project(project_id)?
            .ok_or_else(|| StorageError::ProjectNotFound(project_id.to_string()))?;
        project.set_clarification_threshold(value)?;
        self.connection.execute(
            "UPDATE projects SET clarification_threshold = ?2, updated_at = ?3, \
             revision = revision + 1 WHERE id = ?1",
            params![
                project_id.as_str(),
                project.clarification_threshold(),
                project.updated_at().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    /// Persists a validated derived finding without changing its declared provenance.
    #[allow(clippy::too_many_arguments)]
    pub fn add_derived_finding(
        &mut self,
        project_id: ProjectId,
        kind: FindingKind,
        statement: &str,
        source_type: SourceType,
        source_reference: &str,
        confidence: Confidence,
        impact: Impact,
        requires_confirmation: bool,
    ) -> Result<crate::domain::Finding, StorageError> {
        self.add_finding(NewFinding::new(
            project_id,
            kind,
            statement,
            source_type,
            source_reference,
            confidence,
            impact,
            requires_confirmation,
        )?)
    }

    /// Stores a prioritized clarification tied to an optional motivating finding.
    #[allow(clippy::too_many_arguments)]
    pub fn add_question(
        &mut self,
        project_id: &ProjectId,
        finding_id: Option<&str>,
        prompt: &str,
        rationale: &str,
        impact: Impact,
        uncertainty: Uncertainty,
        cost: CostOfBeingWrong,
    ) -> Result<Question, StorageError> {
        let transaction = self.connection.transaction()?;
        ensure_project_exists(&transaction, project_id)?;
        let question = build_question(
            project_id,
            finding_id,
            prompt,
            rationale,
            impact,
            uncertainty,
            cost,
            &transaction,
        )?;
        insert_question(&transaction, &question)?;
        transaction.commit()?;
        Ok(question)
    }

    /// Lists persisted successful-analysis activity in chronological run and sequence order.
    pub fn list_agent_activity(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<ActivityEvent>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT event.sequence, event.kind, event.message, event.created_at \
             FROM agent_activity_events AS event \
             JOIN agent_runs AS run ON run.id = event.agent_run_id \
             WHERE event.project_id = ?1 \
             ORDER BY run.started_at, run.id, event.sequence",
        )?;
        let rows = statement.query_map([project_id.as_str()], |row| {
            Ok((
                row.get::<_, u32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;
        rows.map(|row| {
            let (sequence, kind, message, created_at) = row?;
            let kind =
                ActivityKind::from_db_str(&kind).ok_or_else(|| StorageError::CorruptData {
                    field: "agent activity kind",
                    value: kind,
                })?;
            Ok(ActivityEvent {
                sequence,
                kind,
                message,
                created_at: parse_timestamp(created_at, "agent_activity_events.created_at")?,
            })
        })
        .collect()
    }

    /// Stores a user answer and all deterministic reconciliation effects in one transaction.
    pub fn reconcile_answer(
        &mut self,
        question_id: &str,
        answer_text: &str,
        notes: Option<&str>,
    ) -> Result<Answer, StorageError> {
        self.reconcile_answer_with_evidence(
            question_id,
            answer_text,
            notes,
            AnswerSource::User,
            &[],
        )
    }

    /// Stores one researched answer and all cited evidence in the same transaction.
    pub fn reconcile_researched_answer(
        &mut self,
        question_id: &str,
        answer_text: &str,
        notes: Option<&str>,
        evidence: &[ResearchEvidence],
    ) -> Result<Answer, StorageError> {
        self.reconcile_answer_with_evidence(
            question_id,
            answer_text,
            notes,
            AnswerSource::Imported,
            evidence,
        )
    }

    /// Stores every still-open judged answer in one transaction after validating batch identity.
    pub fn reconcile_researched_answer_batch(
        &mut self,
        batch: &JudgedResearchBatch,
    ) -> Result<Vec<Answer>, StorageError> {
        let transaction = self.connection.transaction()?;
        let mut applicable = Vec::new();
        let mut project_id: Option<ProjectId> = None;
        for judged in batch.answers() {
            let question = load_question_by_id(&transaction, judged.question_id())?
                .ok_or_else(|| StorageError::QuestionNotFound(judged.question_id().to_owned()))?;
            if let Some(expected) = &project_id {
                if expected != &question.project_id {
                    return Err(StorageError::ProjectNotFound(format!(
                        "judged batch spans projects {} and {}",
                        expected, question.project_id
                    )));
                }
            } else {
                project_id = Some(question.project_id.clone());
            }
            if question.status == QuestionStatus::Open {
                applicable.push((question, judged.answer()));
            }
        }

        let now = Utc::now();
        let mut accepted = Vec::with_capacity(applicable.len());
        for (question, researched) in applicable {
            accepted.push(reconcile_answer_in_transaction(
                &transaction,
                &question,
                researched.answer_text(),
                researched.notes(),
                AnswerSource::Imported,
                researched.evidence(),
                now,
            )?);
        }
        if let Some(project_id) = project_id {
            update_project_after_answers(&transaction, &project_id, now)?;
        }
        transaction.commit()?;
        Ok(accepted)
    }

    /// Applies common answer reconciliation while preserving source and evidence provenance.
    fn reconcile_answer_with_evidence(
        &mut self,
        question_id: &str,
        answer_text: &str,
        notes: Option<&str>,
        source: AnswerSource,
        evidence: &[ResearchEvidence],
    ) -> Result<Answer, StorageError> {
        let transaction = self.connection.transaction()?;
        let question = load_question_by_id(&transaction, question_id)?
            .ok_or_else(|| StorageError::QuestionNotFound(question_id.to_owned()))?;
        if question.status != QuestionStatus::Open {
            return Err(StorageError::QuestionNotOpen(question_id.to_owned()));
        }

        let now = Utc::now();
        let answer = reconcile_answer_in_transaction(
            &transaction,
            &question,
            answer_text,
            notes,
            source,
            evidence,
            now,
        )?;
        update_project_after_answers(&transaction, &question.project_id, now)?;
        transaction.commit()?;
        Ok(answer)
    }

    /// Loads a consistent inspection model for workflow, rendering, and validation callers.
    pub fn project_snapshot(
        &self,
        project_id: &ProjectId,
    ) -> Result<ProjectSnapshot, StorageError> {
        let project = self
            .get_project(project_id)?
            .ok_or_else(|| StorageError::ProjectNotFound(project_id.to_string()))?;
        Ok(ProjectSnapshot {
            project,
            findings: self.list_findings(project_id)?,
            questions: self.list_questions(project_id)?,
            answers: self.list_answers(project_id)?,
            requirements: self.list_requirements(project_id)?,
            evidence: self.list_evidence(project_id)?,
            decisions: self.list_decisions(project_id)?,
            traces: self.list_traces(project_id)?,
        })
    }

    /// Lists clarification questions in descending priority and stable-ID order.
    pub fn list_questions(&self, project_id: &ProjectId) -> Result<Vec<Question>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, display_id, project_id, finding_id, prompt, rationale, impact, uncertainty, \
             cost_of_being_wrong, priority_score, status, created_at, updated_at \
             FROM questions WHERE project_id = ?1 ORDER BY priority_score DESC, display_id",
        )?;
        let rows = statement.query_map([project_id.as_str()], read_question)?;
        rows.map(|row| row.map_err(StorageError::from).and_then(question_from_raw))
            .collect()
    }

    /// Lists immutable answers in their project-local stable order.
    pub fn list_answers(&self, project_id: &ProjectId) -> Result<Vec<Answer>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, display_id, project_id, question_id, answer_text, answer_source, \
             resolves_question, notes, created_at FROM answers WHERE project_id = ?1 ORDER BY display_id",
        )?;
        let rows = statement.query_map([project_id.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, bool>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, String>(8)?,
            ))
        })?;
        rows.map(|row| {
            let (
                id,
                display_id,
                project_id,
                question_id,
                answer_text,
                source,
                resolves,
                notes,
                created,
            ) = row?;
            Ok(Answer {
                id,
                display_id,
                project_id: ProjectId::from_stored(project_id),
                question_id,
                answer_text,
                source: parse_value(&source, "answer source", AnswerSource::from_db_str)?,
                resolves_question: resolves,
                notes,
                created_at: parse_timestamp(created, "answers.created_at")?,
            })
        })
        .collect()
    }

    /// Lists first-class requirements in their stable project order.
    pub fn list_requirements(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<Requirement>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, display_id, project_id, statement, source_type, source_reference, priority, \
             status, rationale, acceptance_criteria, created_at, updated_at \
             FROM requirements WHERE project_id = ?1 ORDER BY display_id",
        )?;
        let rows = statement.query_map([project_id.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
            ))
        })?;
        rows.map(|row| {
            let (
                id,
                display_id,
                project_id,
                statement,
                source,
                source_reference,
                priority,
                status,
                rationale,
                acceptance,
                created,
                updated,
            ) = row?;
            Ok(Requirement {
                id,
                display_id,
                project_id: ProjectId::from_stored(project_id),
                statement,
                source_type: parse_value(&source, "source type", SourceType::from_db_str)?,
                source_reference,
                priority,
                status: parse_value(
                    &status,
                    "requirement status",
                    RequirementStatus::from_db_str,
                )?,
                rationale,
                acceptance_criteria: acceptance,
                created_at: parse_timestamp(created, "requirements.created_at")?,
                updated_at: parse_timestamp(updated, "requirements.updated_at")?,
            })
        })
        .collect()
    }

    /// Lists explicit graph edges without inferring relationships from similarity.
    pub fn list_traces(&self, project_id: &ProjectId) -> Result<Vec<TraceLink>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, project_id, source_type, source_id, target_type, target_id, relationship, created_at \
             FROM trace_links WHERE project_id = ?1 ORDER BY created_at, id",
        )?;
        let rows = statement.query_map([project_id.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;
        rows.map(|row| {
            let (id, project_id, source_type, source_id, target_type, target_id, relation, created) = row?;
            Ok(TraceLink {
                id,
                project_id: ProjectId::from_stored(project_id),
                source_type,
                source_id,
                target_type,
                target_id,
                relationship: parse_value(
                    &relation,
                    "trace relationship",
                    TraceRelationship::from_db_str,
                )?,
                created_at: parse_timestamp(created, "trace_links.created_at")?,
            })
        })
        .collect()
    }
}

/// Contains raw question values returned by SQLite before checked parsing.
type RawQuestion = (
    String,
    String,
    String,
    Option<String>,
    String,
    String,
    String,
    String,
    String,
    u16,
    String,
    String,
    String,
);

/// Reads a raw question row without interpreting checked text in the SQLite callback.
fn read_question(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawQuestion> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
        row.get(10)?,
        row.get(11)?,
        row.get(12)?,
    ))
}

/// Converts checked question vocabulary and timestamps into a domain record.
fn question_from_raw(raw: RawQuestion) -> Result<Question, StorageError> {
    let (
        id,
        display_id,
        project_id,
        finding_id,
        prompt,
        rationale,
        impact,
        uncertainty,
        cost,
        score,
        status,
        created,
        updated,
    ) = raw;
    Ok(Question {
        id,
        display_id,
        project_id: ProjectId::from_stored(project_id),
        finding_id,
        prompt,
        rationale,
        impact: parse_value(&impact, "impact", Impact::from_db_str)?,
        uncertainty: parse_value(&uncertainty, "uncertainty", Uncertainty::from_db_str)?,
        cost_of_being_wrong: parse_value(
            &cost,
            "cost of being wrong",
            CostOfBeingWrong::from_db_str,
        )?,
        priority: QuestionPriority::from_score(score),
        status: parse_value(&status, "question status", QuestionStatus::from_db_str)?,
        created_at: parse_timestamp(created, "questions.created_at")?,
        updated_at: parse_timestamp(updated, "questions.updated_at")?,
    })
}

/// Builds one validated clarification record while sharing the caller's display-ID transaction.
#[allow(clippy::too_many_arguments)]
fn build_question(
    project_id: &ProjectId,
    finding_id: Option<&str>,
    prompt: &str,
    rationale: &str,
    impact: Impact,
    uncertainty: Uncertainty,
    cost: CostOfBeingWrong,
    transaction: &rusqlite::Transaction<'_>,
) -> Result<Question, StorageError> {
    let now = Utc::now();
    Ok(Question {
        id: Uuid::new_v4().to_string(),
        display_id: allocate_display_id(transaction, project_id, "Q")?,
        project_id: project_id.clone(),
        finding_id: finding_id.map(str::to_owned),
        prompt: normalize_required_text(prompt, "question prompt", 8_192)?,
        rationale: normalize_required_text(rationale, "question rationale", 8_192)?,
        impact,
        uncertainty,
        cost_of_being_wrong: cost,
        priority: QuestionPriority::new(impact, uncertainty, cost),
        status: QuestionStatus::Open,
        created_at: now,
        updated_at: now,
    })
}

/// Inserts one fully identified clarification inside an existing transaction.
fn insert_question(
    transaction: &rusqlite::Transaction<'_>,
    question: &Question,
) -> Result<(), StorageError> {
    transaction.execute(
        "INSERT INTO questions (id, display_id, project_id, finding_id, prompt, rationale, \
         impact, uncertainty, cost_of_being_wrong, priority_score, status, created_at, updated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            question.id,
            question.display_id,
            question.project_id.as_str(),
            question.finding_id,
            question.prompt,
            question.rationale,
            question.impact.as_db_str(),
            question.uncertainty.as_db_str(),
            question.cost_of_being_wrong.as_db_str(),
            question.priority.score(),
            question.status.as_db_str(),
            question.created_at.to_rfc3339(),
            question.updated_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

/// Converts optional provider token counts before any transaction can mutate state.
fn agent_token_counts(
    execution: &AgentExecution,
) -> Result<(Option<i64>, Option<i64>), StorageError> {
    Ok((
        checked_token_count(execution.input_tokens, "agent input token count")?,
        checked_token_count(execution.output_tokens, "agent output token count")?,
    ))
}

/// Converts one optional token count without lossy integer narrowing.
fn checked_token_count(
    value: Option<u64>,
    field: &'static str,
) -> Result<Option<i64>, StorageError> {
    value
        .map(|count| {
            i64::try_from(count).map_err(|_| StorageError::ValueOutOfRange {
                field,
                value: count,
            })
        })
        .transpose()
}

/// Inserts one successful run and its sanitized activity entries atomically with the project.
fn insert_agent_execution(
    transaction: &rusqlite::Transaction<'_>,
    project_id: &ProjectId,
    execution: &AgentExecution,
    token_counts: Option<(Option<i64>, Option<i64>)>,
) -> Result<(), StorageError> {
    let (input_tokens, output_tokens) = token_counts.unwrap_or((None, None));
    let response_hash = format!("{:x}", Sha256::digest(execution.response.as_bytes()));
    transaction.execute(
        "INSERT INTO agent_runs (id, project_id, operation, provider, model, status, \
         response_hash, input_tokens, output_tokens, started_at, completed_at) \
         VALUES (?1, ?2, 'initial_brief_analysis', ?3, ?4, 'succeeded', ?5, ?6, ?7, ?8, ?9)",
        params![
            execution.id,
            project_id.as_str(),
            execution.provider,
            execution.model,
            response_hash,
            input_tokens,
            output_tokens,
            execution.started_at.to_rfc3339(),
            execution.completed_at.to_rfc3339(),
        ],
    )?;
    for event in &execution.activity {
        transaction.execute(
            "INSERT INTO agent_activity_events \
             (agent_run_id, project_id, sequence, kind, message, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                execution.id,
                project_id.as_str(),
                event.sequence,
                event.kind.as_db_str(),
                event.message,
                event.created_at.to_rfc3339(),
            ],
        )?;
    }
    Ok(())
}

/// Loads one question inside an existing reconciliation transaction.
fn load_question_by_id(
    transaction: &rusqlite::Transaction<'_>,
    question_id: &str,
) -> Result<Option<Question>, StorageError> {
    transaction
        .query_row(
            "SELECT id, display_id, project_id, finding_id, prompt, rationale, impact, uncertainty, \
             cost_of_being_wrong, priority_score, status, created_at, updated_at FROM questions WHERE id = ?1",
            [question_id],
            read_question,
        )
        .optional()?
        .map(question_from_raw)
        .transpose()
}

/// Applies one validated answer inside a caller-owned transaction without committing it.
fn reconcile_answer_in_transaction(
    transaction: &rusqlite::Transaction<'_>,
    question: &Question,
    answer_text: &str,
    notes: Option<&str>,
    source: AnswerSource,
    evidence: &[ResearchEvidence],
    now: chrono::DateTime<Utc>,
) -> Result<Answer, StorageError> {
    let answer_text = normalize_required_text(answer_text, "answer", 16_384)?;
    let notes = notes
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    let answer = Answer {
        id: Uuid::new_v4().to_string(),
        display_id: allocate_display_id(transaction, &question.project_id, "ANS")?,
        project_id: question.project_id.clone(),
        question_id: question.id.clone(),
        answer_text,
        source,
        resolves_question: true,
        notes,
        created_at: now,
    };
    insert_answer(transaction, &answer)?;
    for research_evidence in evidence {
        let evidence = insert_researched_evidence(transaction, question, research_evidence, now)?;
        insert_trace(
            transaction,
            &question.project_id,
            "evidence",
            &evidence.id,
            "answer",
            &answer.id,
            TraceRelationship::Supports,
            now,
        )?;
    }
    transaction.execute(
        "UPDATE questions SET status = 'answered', updated_at = ?2 WHERE id = ?1",
        params![question.id, now.to_rfc3339()],
    )?;
    if let Some(finding_id) = &question.finding_id {
        transaction.execute(
            "UPDATE findings SET status = 'resolved', updated_at = ?2 WHERE id = ?1",
            params![finding_id, now.to_rfc3339()],
        )?;
    }
    let requirement = reconciled_requirement(transaction, question, &answer, now)?;
    insert_trace(
        transaction,
        &question.project_id,
        "requirement",
        &requirement.id,
        "question",
        &question.id,
        TraceRelationship::DerivedFrom,
        now,
    )?;
    insert_trace(
        transaction,
        &question.project_id,
        "answer",
        &answer.id,
        "question",
        &question.id,
        TraceRelationship::Answers,
        now,
    )?;
    Ok(answer)
}

/// Advances a clarified project only after its transaction contains no remaining open questions.
fn update_project_after_answers(
    transaction: &rusqlite::Transaction<'_>,
    project_id: &ProjectId,
    now: chrono::DateTime<Utc>,
) -> Result<(), StorageError> {
    let open_questions = transaction.query_row(
        "SELECT COUNT(*) FROM questions WHERE project_id = ?1 AND status = 'open'",
        [project_id.as_str()],
        |row| row.get::<_, i64>(0),
    )?;
    if open_questions == 0 {
        transaction.execute(
            "UPDATE projects SET status = 'planning', updated_at = ?2, revision = revision + 1 \
             WHERE id = ?1 AND status = 'awaiting_clarification'",
            params![project_id.as_str(), now.to_rfc3339()],
        )?;
    }
    Ok(())
}

/// Inserts the immutable answer portion of a reconciliation transaction.
fn insert_answer(
    transaction: &rusqlite::Transaction<'_>,
    answer: &Answer,
) -> Result<(), StorageError> {
    transaction.execute(
        "INSERT INTO answers (id, display_id, project_id, question_id, answer_text, answer_source, \
         resolves_question, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            answer.id,
            answer.display_id,
            answer.project_id.as_str(),
            answer.question_id,
            answer.answer_text,
            answer.source.as_db_str(),
            answer.resolves_question,
            answer.notes,
            answer.created_at.to_rfc3339()
        ],
    )?;
    Ok(())
}

/// Creates and inserts an active requirement deterministically derived from a user answer.
fn reconciled_requirement(
    transaction: &rusqlite::Transaction<'_>,
    question: &Question,
    answer: &Answer,
    now: chrono::DateTime<Utc>,
) -> Result<Requirement, StorageError> {
    let subject = if question.prompt.to_ascii_lowercase().contains("platform") {
        "primary target platform"
    } else {
        "clarified project choice"
    };
    let requirement = Requirement {
        id: Uuid::new_v4().to_string(),
        display_id: allocate_display_id(transaction, &question.project_id, "REQ")?,
        project_id: question.project_id.clone(),
        statement: format!("The {subject} must be {}.", answer.answer_text),
        source_type: match answer.source {
            AnswerSource::User => SourceType::UserAnswer,
            AnswerSource::Imported => SourceType::Research,
            AnswerSource::System => SourceType::System,
        },
        source_reference: answer.display_id.clone(),
        priority: question.impact.as_db_str().to_owned(),
        status: RequirementStatus::Active,
        rationale: Some(question.rationale.clone()),
        acceptance_criteria: format!(
            "Project plans and generated documents identify {} as the {subject}.",
            answer.answer_text
        ),
        created_at: now,
        updated_at: now,
    };
    transaction.execute(
        "INSERT INTO requirements (id, display_id, project_id, statement, source_type, source_reference, \
         priority, status, rationale, acceptance_criteria, created_at, updated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            requirement.id,
            requirement.display_id,
            requirement.project_id.as_str(),
            requirement.statement,
            requirement.source_type.as_db_str(),
            requirement.source_reference,
            requirement.priority,
            requirement.status.as_db_str(),
            requirement.rationale,
            requirement.acceptance_criteria,
            requirement.created_at.to_rfc3339(),
            requirement.updated_at.to_rfc3339()
        ],
    )?;
    Ok(requirement)
}

/// Inserts one cited research record tied to its originating unknown and answer.
fn insert_researched_evidence(
    transaction: &rusqlite::Transaction<'_>,
    question: &Question,
    input: &ResearchEvidence,
    now: chrono::DateTime<Utc>,
) -> Result<crate::domain::Evidence, StorageError> {
    let evidence = crate::domain::Evidence {
        id: Uuid::new_v4().to_string(),
        display_id: allocate_display_id(transaction, &question.project_id, "EVD")?,
        project_id: question.project_id.clone(),
        claim: input.claim().to_owned(),
        source: input.source().to_owned(),
        source_title: input.source_title().to_owned(),
        reliability: input.reliability(),
        notes: input.notes().map(ToOwned::to_owned),
        retrieved_at: now,
    };
    transaction.execute(
        "INSERT INTO evidence \
         (id, display_id, project_id, research_question_id, claim, source, source_title, retrieved_at, reliability, notes, status, created_at, updated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'active', ?8, ?8)",
        params![
            evidence.id,
            evidence.display_id,
            evidence.project_id.as_str(),
            question.finding_id,
            evidence.claim,
            evidence.source,
            evidence.source_title,
            evidence.retrieved_at.to_rfc3339(),
            evidence.reliability.as_db_str(),
            evidence.notes,
        ],
    )?;
    Ok(evidence)
}

/// Inserts one explicit graph edge inside the caller's transaction.
#[allow(clippy::too_many_arguments)]
fn insert_trace(
    transaction: &rusqlite::Transaction<'_>,
    project_id: &ProjectId,
    source_type: &str,
    source_id: &str,
    target_type: &str,
    target_id: &str,
    relationship: TraceRelationship,
    now: chrono::DateTime<Utc>,
) -> Result<(), StorageError> {
    transaction.execute(
        "INSERT INTO trace_links (id, project_id, source_type, source_id, target_type, target_id, relationship, created_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            Uuid::new_v4().to_string(),
            project_id.as_str(),
            source_type,
            source_id,
            target_type,
            target_id,
            relationship.as_db_str(),
            now.to_rfc3339()
        ],
    )?;
    Ok(())
}

/// Parses checked text or reports the exact corrupt field and value.
fn parse_value<T>(
    value: &str,
    field: &'static str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, StorageError> {
    parser(value).ok_or_else(|| StorageError::CorruptData {
        field,
        value: value.to_owned(),
    })
}
