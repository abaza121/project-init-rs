use std::collections::{HashMap, HashSet};

use chrono::Utc;
use rusqlite::{OptionalExtension, params};
use uuid::Uuid;

use crate::domain::{
    ApprovalPolicy, Decision, DecisionStatus, DocumentRevision, DocumentRevisionSource, Evidence,
    EvidenceReliability, ProjectId, WorkflowRun, WorkflowRunStatus,
};

use super::sqlite::{allocate_display_id, ensure_project_exists, parse_timestamp};
use super::{SqliteStore, StorageError};

/// Describes one current document row needed for filesystem reconciliation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StoredDocument {
    pub relative_path: String,
    pub content_hash: String,
    pub status: String,
}

/// Contains one checked open finding from the newest persisted validation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StoredValidationFinding {
    pub code: String,
    pub severity: String,
    pub message: String,
    pub repairable: bool,
}

impl SqliteStore {
    /// Starts a run or idempotently returns the active run when its policy matches.
    pub(crate) fn start_workflow_run(
        &mut self,
        project_id: &ProjectId,
        policy: ApprovalPolicy,
    ) -> Result<WorkflowRun, StorageError> {
        if let Some(active) = self.active_workflow_run(project_id)? {
            if active.policy == policy {
                return Ok(active);
            }
            return Err(StorageError::ActiveRunPolicyConflict {
                active: active.policy.as_db_str().to_owned(),
                requested: policy.as_db_str().to_owned(),
            });
        }
        let transaction = self.connection.transaction()?;
        ensure_project_exists(&transaction, project_id)?;
        let now = Utc::now();
        let run = WorkflowRun {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.clone(),
            policy,
            status: WorkflowRunStatus::Running,
            pause_reason: None,
            started_at: now,
            updated_at: now,
            completed_at: None,
        };
        transaction.execute(
            "INSERT INTO workflow_runs \
             (id, project_id, approval_policy, status, pause_reason, started_at, updated_at, completed_at) \
             VALUES (?1, ?2, ?3, ?4, NULL, ?5, ?6, NULL)",
            params![
                run.id,
                project_id.as_str(),
                policy.as_db_str(),
                run.status.as_db_str(),
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;
        transaction.commit()?;
        Ok(run)
    }

    /// Loads the sole resumable run for a project, if one exists.
    pub(crate) fn active_workflow_run(
        &self,
        project_id: &ProjectId,
    ) -> Result<Option<WorkflowRun>, StorageError> {
        self.connection
            .query_row(
                "SELECT id, project_id, approval_policy, status, pause_reason, started_at, updated_at, completed_at \
                 FROM workflow_runs WHERE project_id = ?1 AND status IN ('running', 'paused')",
                [project_id.as_str()],
                read_workflow_run,
            )
            .optional()?
            .map(workflow_run_from_raw)
            .transpose()
    }

    /// Loads the newest workflow run including terminal history.
    pub(crate) fn latest_workflow_run(
        &self,
        project_id: &ProjectId,
    ) -> Result<Option<WorkflowRun>, StorageError> {
        self.connection
            .query_row(
                "SELECT id, project_id, approval_policy, status, pause_reason, started_at, updated_at, completed_at \
                 FROM workflow_runs WHERE project_id = ?1 ORDER BY started_at DESC, rowid DESC LIMIT 1",
                [project_id.as_str()],
                read_workflow_run,
            )
            .optional()?
            .map(workflow_run_from_raw)
            .transpose()
    }

    /// Changes the active run state without replacing its effective policy.
    pub(crate) fn set_workflow_run_status(
        &mut self,
        run_id: &str,
        status: WorkflowRunStatus,
        pause_reason: Option<&str>,
    ) -> Result<(), StorageError> {
        let now = Utc::now().to_rfc3339();
        let completed = status.is_terminal().then_some(now.as_str());
        let changed = self.connection.execute(
            "UPDATE workflow_runs SET status = ?2, pause_reason = ?3, updated_at = ?4, completed_at = ?5 \
             WHERE id = ?1 AND status IN ('running', 'paused')",
            params![run_id, status.as_db_str(), pause_reason, now, completed],
        )?;
        if changed == 0 {
            return Err(StorageError::WorkflowRunNotFound(run_id.to_owned()));
        }
        Ok(())
    }

    /// Returns the latest manual artifact paths that ordinary generation must protect.
    pub(crate) fn manual_override_paths(
        &self,
        project_id: &ProjectId,
    ) -> Result<HashSet<String>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT revision.relative_path FROM document_revisions revision \
             WHERE revision.project_id = ?1 AND revision.source = 'manual_override' \
             AND revision.rowid = (SELECT latest.rowid FROM document_revisions latest \
                 WHERE latest.project_id = revision.project_id AND latest.relative_path = revision.relative_path \
                 ORDER BY latest.created_at DESC, latest.rowid DESC LIMIT 1)",
        )?;
        let rows = statement.query_map([project_id.as_str()], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<HashSet<_>, _>>().map_err(Into::into)
    }

    /// Loads current document hashes and statuses keyed by project-relative path.
    pub(crate) fn stored_documents(
        &self,
        project_id: &ProjectId,
    ) -> Result<HashMap<String, StoredDocument>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT relative_path, content_hash, status FROM documents WHERE project_id = ?1",
        )?;
        let rows = statement.query_map([project_id.as_str()], |row| {
            Ok(StoredDocument {
                relative_path: row.get(0)?,
                content_hash: row.get(1)?,
                status: row.get(2)?,
            })
        })?;
        let mut documents = HashMap::new();
        for row in rows {
            let document = row?;
            documents.insert(document.relative_path.clone(), document);
        }
        Ok(documents)
    }

    /// Marks every current generated view stale after authoritative knowledge changes.
    pub(crate) fn mark_documents_stale(
        &mut self,
        project_id: &ProjectId,
    ) -> Result<(), StorageError> {
        self.connection.execute(
            "UPDATE documents SET status = 'stale' WHERE project_id = ?1",
            [project_id.as_str()],
        )?;
        Ok(())
    }

    /// Marks only the fixed dependency closure stale after an artifact override.
    pub(crate) fn mark_document_paths_stale(
        &mut self,
        project_id: &ProjectId,
        relative_paths: &[String],
    ) -> Result<(), StorageError> {
        let transaction = self.connection.transaction()?;
        for relative_path in relative_paths {
            transaction.execute(
                "UPDATE documents SET status = 'stale' WHERE project_id = ?1 AND relative_path = ?2",
                params![project_id.as_str(), relative_path],
            )?;
        }
        transaction.commit()?;
        Ok(())
    }

    /// Commits generated hashes and immutable revisions while skipping protected overrides.
    pub(crate) fn persist_generated_documents(
        &mut self,
        project_id: &ProjectId,
        documents: &[(String, String)],
        protected: &HashSet<String>,
    ) -> Result<(), StorageError> {
        let transaction = self.connection.transaction()?;
        ensure_project_exists(&transaction, project_id)?;
        for (relative_path, content_hash) in documents {
            if protected.contains(relative_path) {
                continue;
            }
            let stored = transaction
                .query_row(
                    "SELECT id, content_hash FROM documents WHERE project_id = ?1 AND relative_path = ?2",
                    params![project_id.as_str(), relative_path],
                    |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
                )
                .optional()?;
            let now = Utc::now().to_rfc3339();
            let (document_id, previous_hash) = if let Some((id, previous_hash)) = stored {
                transaction.execute(
                    "UPDATE documents SET content_hash = ?3, status = 'current', generated_at = ?4 \
                     WHERE project_id = ?1 AND relative_path = ?2",
                    params![project_id.as_str(), relative_path, content_hash, now],
                )?;
                (id, Some(previous_hash))
            } else {
                let id = Uuid::new_v4().to_string();
                let display_id = allocate_display_id(&transaction, project_id, "DOC")?;
                transaction.execute(
                    "INSERT INTO documents \
                     (id, display_id, project_id, kind, relative_path, content_hash, status, generated_at) \
                     VALUES (?1, ?2, ?3, 'markdown', ?4, ?5, 'current', ?6)",
                    params![id, display_id, project_id.as_str(), relative_path, content_hash, now],
                )?;
                (id, None)
            };
            if previous_hash.as_deref() != Some(content_hash.as_str()) {
                insert_document_revision(
                    &transaction,
                    project_id,
                    Some(&document_id),
                    relative_path,
                    content_hash,
                    DocumentRevisionSource::Generated,
                    previous_hash.as_deref(),
                )?;
            }
        }
        transaction.commit()?;
        Ok(())
    }

    /// Registers current file content as a protected manual revision and stales other artifacts.
    pub(crate) fn persist_document_override(
        &mut self,
        project_id: &ProjectId,
        relative_path: &str,
        content_hash: &str,
    ) -> Result<(), StorageError> {
        let transaction = self.connection.transaction()?;
        ensure_project_exists(&transaction, project_id)?;
        let stored = transaction
            .query_row(
                "SELECT id, content_hash FROM documents WHERE project_id = ?1 AND relative_path = ?2",
                params![project_id.as_str(), relative_path],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?
            .ok_or_else(|| StorageError::DocumentNotFound(relative_path.to_owned()))?;
        let now = Utc::now().to_rfc3339();
        transaction.execute(
            "UPDATE documents SET content_hash = ?3, status = 'current', generated_at = ?4 \
             WHERE project_id = ?1 AND relative_path = ?2",
            params![project_id.as_str(), relative_path, content_hash, now],
        )?;
        insert_document_revision(
            &transaction,
            project_id,
            Some(&stored.0),
            relative_path,
            content_hash,
            DocumentRevisionSource::ManualOverride,
            Some(&stored.1),
        )?;
        transaction.commit()?;
        Ok(())
    }

    /// Releases a manual artifact back to generated control while retaining its history.
    pub(crate) fn release_document_override(
        &mut self,
        project_id: &ProjectId,
        relative_path: &str,
    ) -> Result<(), StorageError> {
        if !self
            .manual_override_paths(project_id)?
            .contains(relative_path)
        {
            return Err(StorageError::DocumentNotFound(relative_path.to_owned()));
        }
        let transaction = self.connection.transaction()?;
        let (document_id, current_hash) = transaction.query_row(
            "SELECT id, content_hash FROM documents WHERE project_id = ?1 AND relative_path = ?2",
            params![project_id.as_str(), relative_path],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )?;
        transaction.execute(
            "UPDATE documents SET status = 'stale' WHERE id = ?1",
            [&document_id],
        )?;
        insert_document_revision(
            &transaction,
            project_id,
            Some(&document_id),
            relative_path,
            &current_hash,
            DocumentRevisionSource::Generated,
            Some(&current_hash),
        )?;
        transaction.commit()?;
        Ok(())
    }

    /// Loads immutable revisions for one project-relative artifact in creation order.
    pub(crate) fn list_document_revisions(
        &self,
        project_id: &ProjectId,
        relative_path: &str,
    ) -> Result<Vec<DocumentRevision>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, project_id, document_id, relative_path, content_hash, source, previous_hash, created_at \
             FROM document_revisions WHERE project_id = ?1 AND relative_path = ?2 ORDER BY created_at, rowid",
        )?;
        let rows = statement.query_map(params![project_id.as_str(), relative_path], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;
        rows.map(|row| {
            let (
                id,
                project_id,
                document_id,
                relative_path,
                content_hash,
                source,
                previous_hash,
                created_at,
            ) = row?;
            Ok(DocumentRevision {
                id,
                project_id: ProjectId::from_stored(project_id),
                document_id,
                relative_path,
                content_hash,
                source: DocumentRevisionSource::from_db_str(&source).ok_or_else(|| {
                    StorageError::CorruptData {
                        field: "document revision source",
                        value: source,
                    }
                })?,
                previous_hash,
                created_at: parse_timestamp(created_at, "document_revisions.created_at")?,
            })
        })
        .collect()
    }

    /// Stores one explicit external claim without promoting it to a conclusion.
    pub(crate) fn insert_evidence(
        &mut self,
        project_id: &ProjectId,
        claim: &str,
        source: &str,
        source_title: &str,
        reliability: EvidenceReliability,
        notes: Option<&str>,
    ) -> Result<Evidence, StorageError> {
        let transaction = self.connection.transaction()?;
        ensure_project_exists(&transaction, project_id)?;
        let evidence = Evidence {
            id: Uuid::new_v4().to_string(),
            display_id: allocate_display_id(&transaction, project_id, "EVD")?,
            project_id: project_id.clone(),
            claim: claim.to_owned(),
            source: source.to_owned(),
            source_title: source_title.to_owned(),
            reliability,
            notes: notes.map(ToOwned::to_owned),
            retrieved_at: Utc::now(),
        };
        transaction.execute(
            "INSERT INTO evidence \
             (id, display_id, project_id, research_question_id, claim, source, source_title, retrieved_at, reliability, notes, status, created_at, updated_at) \
             VALUES (?1, ?2, ?3, NULL, ?4, ?5, ?6, ?7, ?8, ?9, 'active', ?7, ?7)",
            params![
                evidence.id,
                evidence.display_id,
                project_id.as_str(),
                evidence.claim,
                evidence.source,
                evidence.source_title,
                evidence.retrieved_at.to_rfc3339(),
                reliability.as_db_str(),
                evidence.notes,
            ],
        )?;
        transaction.commit()?;
        Ok(evidence)
    }

    /// Lists active evidence in stable display order for rendering and inspection.
    pub(crate) fn list_evidence(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<Evidence>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, display_id, project_id, claim, source, source_title, retrieved_at, reliability, notes \
             FROM evidence WHERE project_id = ?1 AND status = 'active' ORDER BY display_id",
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
            ))
        })?;
        rows.map(|row| {
            let raw = row?;
            Ok(Evidence {
                id: raw.0,
                display_id: raw.1,
                project_id: ProjectId::from_stored(raw.2),
                claim: raw.3,
                source: raw.4,
                source_title: raw.5,
                retrieved_at: parse_timestamp(raw.6, "evidence.retrieved_at")?,
                reliability: EvidenceReliability::from_db_str(&raw.7).ok_or_else(|| {
                    StorageError::CorruptData {
                        field: "evidence reliability",
                        value: raw.7,
                    }
                })?,
                notes: raw.8,
            })
        })
        .collect()
    }

    /// Stores one proposed choice with explicit confirmation classification.
    pub(crate) fn insert_decision(
        &mut self,
        project_id: &ProjectId,
        title: &str,
        statement: &str,
        rationale: &str,
        needs_confirmation: bool,
    ) -> Result<Decision, StorageError> {
        let transaction = self.connection.transaction()?;
        ensure_project_exists(&transaction, project_id)?;
        let now = Utc::now();
        let status = if needs_confirmation {
            DecisionStatus::NeedsConfirmation
        } else {
            DecisionStatus::Proposed
        };
        let decision = Decision {
            id: Uuid::new_v4().to_string(),
            display_id: allocate_display_id(&transaction, project_id, "ADR")?,
            project_id: project_id.clone(),
            title: title.to_owned(),
            statement: statement.to_owned(),
            rationale: rationale.to_owned(),
            status,
            created_at: now,
            updated_at: now,
        };
        transaction.execute(
            "INSERT INTO decisions \
             (id, display_id, project_id, title, statement, rationale, status, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
            params![
                decision.id,
                decision.display_id,
                project_id.as_str(),
                decision.title,
                decision.statement,
                decision.rationale,
                status.as_db_str(),
                now.to_rfc3339(),
            ],
        )?;
        transaction.commit()?;
        Ok(decision)
    }

    /// Lists every retained decision in stable display order.
    pub(crate) fn list_decisions(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<Decision>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT id, display_id, project_id, title, statement, rationale, status, created_at, updated_at \
             FROM decisions WHERE project_id = ?1 ORDER BY display_id",
        )?;
        let rows = statement.query_map([project_id.as_str()], read_decision)?;
        rows.map(|row| row.map_err(StorageError::from).and_then(decision_from_raw))
            .collect()
    }

    /// Returns the first decision that must pause under the effective policy.
    pub(crate) fn pending_approval(
        &self,
        project_id: &ProjectId,
        policy: ApprovalPolicy,
    ) -> Result<Option<Decision>, StorageError> {
        let statuses = match policy {
            ApprovalPolicy::Strict => vec!["needs_confirmation", "proposed"],
            ApprovalPolicy::Consequential | ApprovalPolicy::Autonomous => {
                vec!["needs_confirmation"]
            }
        };
        let mut statement = self.connection.prepare(
            "SELECT id, display_id, project_id, title, statement, rationale, status, created_at, updated_at \
             FROM decisions WHERE project_id = ?1 AND status IN (?2, ?3) ORDER BY display_id LIMIT 1",
        )?;
        statement
            .query_row(
                params![
                    project_id.as_str(),
                    statuses[0],
                    statuses.get(1).copied().unwrap_or(statuses[0])
                ],
                read_decision,
            )
            .optional()?
            .map(decision_from_raw)
            .transpose()
    }

    /// Records explicit authority and applies the corresponding terminal decision state.
    pub(crate) fn resolve_decision(
        &mut self,
        decision_id: &str,
        approved: bool,
        reason: &str,
    ) -> Result<ProjectId, StorageError> {
        let transaction = self.connection.transaction()?;
        let (project_id, status) = transaction
            .query_row(
                "SELECT project_id, status FROM decisions WHERE id = ?1",
                [decision_id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?
            .ok_or_else(|| StorageError::DecisionNotFound(decision_id.to_owned()))?;
        if !matches!(status.as_str(), "proposed" | "needs_confirmation") {
            return Err(StorageError::DecisionNotPending(decision_id.to_owned()));
        }
        let now = Utc::now().to_rfc3339();
        transaction.execute(
            "UPDATE decisions SET status = ?2, updated_at = ?3 WHERE id = ?1",
            params![
                decision_id,
                if approved { "accepted" } else { "rejected" },
                now
            ],
        )?;
        transaction.execute(
            "INSERT INTO decision_approvals (id, project_id, decision_id, outcome, reason, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                Uuid::new_v4().to_string(),
                project_id,
                decision_id,
                if approved { "approved" } else { "rejected" },
                reason,
                now,
            ],
        )?;
        transaction.commit()?;
        Ok(ProjectId::from_stored(project_id))
    }

    /// Returns whether the latest validation passed after the newest document revision.
    pub(crate) fn latest_validation_passed(
        &self,
        project_id: &ProjectId,
    ) -> Result<bool, StorageError> {
        self.connection.query_row(
            "SELECT COALESCE((SELECT status = 'passed' FROM validation_runs run \
             WHERE run.project_id = ?1 AND run.completed_at IS NOT NULL \
             AND run.completed_at >= COALESCE((SELECT MAX(created_at) FROM document_revisions WHERE project_id = ?1), '') \
             ORDER BY run.completed_at DESC, run.rowid DESC LIMIT 1), 0)",
            [project_id.as_str()],
            |row| row.get(0),
        ).map_err(Into::into)
    }

    /// Loads open findings from only the newest completed validation run.
    pub(crate) fn latest_open_validation_findings(
        &self,
        project_id: &ProjectId,
    ) -> Result<Vec<StoredValidationFinding>, StorageError> {
        let mut statement = self.connection.prepare(
            "SELECT finding.code, finding.severity, finding.message, finding.repairable \
             FROM validation_findings finding \
             JOIN validation_runs run ON run.id = finding.validation_run_id \
             WHERE finding.project_id = ?1 AND finding.status = 'open' \
             AND run.id = (SELECT newest.id FROM validation_runs newest \
                           WHERE newest.project_id = ?1 AND newest.completed_at IS NOT NULL \
                           AND newest.completed_at >= COALESCE( \
                               (SELECT MAX(created_at) FROM document_revisions \
                                WHERE project_id = ?1), '') \
                           ORDER BY newest.completed_at DESC, newest.rowid DESC LIMIT 1) \
             ORDER BY finding.display_id",
        )?;
        let rows = statement.query_map([project_id.as_str()], |row| {
            Ok(StoredValidationFinding {
                code: row.get(0)?,
                severity: row.get(1)?,
                message: row.get(2)?,
                repairable: row.get(3)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Returns the repair-pass number recorded by the newest completed validation.
    pub(crate) fn latest_validation_repair_pass(
        &self,
        project_id: &ProjectId,
    ) -> Result<u32, StorageError> {
        let value = self.connection.query_row(
            "SELECT COALESCE((SELECT repair_pass FROM validation_runs \
             WHERE project_id = ?1 AND completed_at IS NOT NULL \
             ORDER BY completed_at DESC, rowid DESC LIMIT 1), 0)",
            [project_id.as_str()],
            |row| row.get::<_, i64>(0),
        )?;
        u32::try_from(value).map_err(|_| StorageError::ValueOutOfRange {
            field: "validation repair pass",
            value: value.unsigned_abs(),
        })
    }

    /// Returns the configured maximum number of completed repair validation passes.
    pub(crate) fn project_repair_limit(&self, project_id: &ProjectId) -> Result<u32, StorageError> {
        let value = self
            .connection
            .query_row(
                "SELECT repair_limit FROM projects WHERE id = ?1",
                [project_id.as_str()],
                |row| row.get::<_, i64>(0),
            )
            .optional()?
            .ok_or_else(|| StorageError::ProjectNotFound(project_id.to_string()))?;
        u32::try_from(value).map_err(|_| StorageError::ValueOutOfRange {
            field: "project repair limit",
            value: value.unsigned_abs(),
        })
    }

    /// Closes the newest open repairable findings after a candidate package is adopted.
    pub(crate) fn mark_latest_validation_findings_repaired(
        &mut self,
        project_id: &ProjectId,
    ) -> Result<(), StorageError> {
        let now = Utc::now().to_rfc3339();
        self.connection.execute(
            "UPDATE validation_findings SET status = 'repaired', updated_at = ?2 \
             WHERE project_id = ?1 AND status = 'open' AND repairable = 1 \
             AND validation_run_id = (SELECT id FROM validation_runs \
                                      WHERE project_id = ?1 AND completed_at IS NOT NULL \
                                      ORDER BY completed_at DESC, rowid DESC LIMIT 1)",
            params![project_id.as_str(), now],
        )?;
        Ok(())
    }

    /// Persists one completed validation and every structured finding atomically.
    pub(crate) fn persist_validation_result(
        &mut self,
        project_id: &ProjectId,
        passed: bool,
        repair_pass: u32,
        findings: &[(String, String, String)],
    ) -> Result<(), StorageError> {
        let transaction = self.connection.transaction()?;
        ensure_project_exists(&transaction, project_id)?;
        let run_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let high_count = findings
            .iter()
            .filter(|finding| finding.1 == "high")
            .count();
        let medium_count = findings
            .iter()
            .filter(|finding| finding.1 == "medium")
            .count();
        let low_count = findings.iter().filter(|finding| finding.1 == "low").count();
        transaction.execute(
            "INSERT INTO validation_runs \
             (id, project_id, status, repair_pass, high_count, medium_count, low_count, started_at, completed_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
            params![
                run_id,
                project_id.as_str(),
                if passed { "passed" } else { "failed" },
                repair_pass,
                i64::try_from(high_count).map_err(|_| StorageError::ValueOutOfRange { field: "validation high count", value: u64::MAX })?,
                i64::try_from(medium_count).map_err(|_| StorageError::ValueOutOfRange { field: "validation medium count", value: u64::MAX })?,
                i64::try_from(low_count).map_err(|_| StorageError::ValueOutOfRange { field: "validation low count", value: u64::MAX })?,
                now,
            ],
        )?;
        for (code, severity, message) in findings {
            let display_id = allocate_display_id(&transaction, project_id, "VAL")?;
            transaction.execute(
                "INSERT INTO validation_findings \
                 (id, display_id, project_id, validation_run_id, code, severity, message, entity_type, entity_id, status, repairable, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL, 'open', ?8, ?9, ?9)",
                params![
                    Uuid::new_v4().to_string(),
                    display_id,
                    project_id.as_str(),
                    run_id,
                    code,
                    severity,
                    message,
                    matches!(code.as_str(), "MISSING_ARTIFACT" | "STALE_ARTIFACT"),
                    now,
                ],
            )?;
        }
        transaction.commit()?;
        Ok(())
    }
}

/// Holds unchecked decision columns until closed vocabularies are validated.
type RawDecision = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

/// Reads raw decision columns from one SQLite row.
fn read_decision(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawDecision> {
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
    ))
}

/// Converts persisted decision text into its checked domain record.
fn decision_from_raw(raw: RawDecision) -> Result<Decision, StorageError> {
    Ok(Decision {
        id: raw.0,
        display_id: raw.1,
        project_id: ProjectId::from_stored(raw.2),
        title: raw.3,
        statement: raw.4,
        rationale: raw.5,
        status: DecisionStatus::from_db_str(&raw.6).ok_or_else(|| StorageError::CorruptData {
            field: "decision status",
            value: raw.6,
        })?,
        created_at: parse_timestamp(raw.7, "decisions.created_at")?,
        updated_at: parse_timestamp(raw.8, "decisions.updated_at")?,
    })
}

/// Holds unchecked workflow-run columns until vocabulary and timestamps are validated.
type RawWorkflowRun = (
    String,
    String,
    String,
    String,
    Option<String>,
    String,
    String,
    Option<String>,
);

/// Reads raw workflow-run columns without accepting checked text yet.
fn read_workflow_run(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawWorkflowRun> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
    ))
}

/// Converts persisted run text into the closed domain vocabulary.
fn workflow_run_from_raw(raw: RawWorkflowRun) -> Result<WorkflowRun, StorageError> {
    Ok(WorkflowRun {
        id: raw.0,
        project_id: ProjectId::from_stored(raw.1),
        policy: ApprovalPolicy::from_db_str(&raw.2).ok_or_else(|| StorageError::CorruptData {
            field: "workflow approval policy",
            value: raw.2,
        })?,
        status: WorkflowRunStatus::from_db_str(&raw.3).ok_or_else(|| {
            StorageError::CorruptData {
                field: "workflow run status",
                value: raw.3,
            }
        })?,
        pause_reason: raw.4,
        started_at: parse_timestamp(raw.5, "workflow_runs.started_at")?,
        updated_at: parse_timestamp(raw.6, "workflow_runs.updated_at")?,
        completed_at: raw
            .7
            .map(|value| parse_timestamp(value, "workflow_runs.completed_at"))
            .transpose()?,
    })
}

/// Inserts one immutable document revision inside the caller's transaction.
#[allow(clippy::too_many_arguments)]
fn insert_document_revision(
    transaction: &rusqlite::Transaction<'_>,
    project_id: &ProjectId,
    document_id: Option<&str>,
    relative_path: &str,
    content_hash: &str,
    source: DocumentRevisionSource,
    previous_hash: Option<&str>,
) -> Result<(), StorageError> {
    transaction.execute(
        "INSERT INTO document_revisions \
         (id, project_id, document_id, relative_path, content_hash, source, previous_hash, created_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            Uuid::new_v4().to_string(),
            project_id.as_str(),
            document_id,
            relative_path,
            content_hash,
            source.as_db_str(),
            previous_hash,
            Utc::now().to_rfc3339(),
        ],
    )?;
    Ok(())
}
