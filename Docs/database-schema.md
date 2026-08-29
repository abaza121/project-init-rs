# Database Schema

SQLite is authoritative. LanceDB can be deleted and rebuilt without losing project knowledge.

## Identity strategy

Each stored entity has an internal UUID text primary key and a stable human-readable `display_id` such as `REQ-007` or `ADR-003`. Display sequences are allocated transactionally per project and entity prefix. Foreign keys use internal IDs; exports and documents expose display IDs.

All mutable records include UTC `created_at` and `updated_at` text timestamps. History is retained through status changes and explicit superseding traces rather than destructive overwrite.

## Core tables

### `projects`

Stores name, original brief, lifecycle status, retrieval mode, clarification threshold, repair limit, timestamps, and optimistic revision.

### `entity_sequences`

Stores `(project_id, prefix, next_value)` for transactional stable-ID allocation.

### `findings`

Stores finding kind, statement, provenance source type/reference, confidence, impact, status, whether confirmation is required, and timestamps. Assumptions, unknowns, constraints, risks, confirmed facts, and research questions are typed finding kinds.

### `requirements`

Stores stable requirement statement, provenance, priority, status, rationale, and concrete acceptance criteria. Requirements are separate from the extraction finding so their lifecycle and acceptance criteria remain explicit.

### `questions`

Stores prompt, rationale, impact, uncertainty, cost of being wrong, calculated priority, status, and the unknown/finding that motivated it.

### `answers`

Stores immutable answer text/value, source, whether it resolved the question, notes, and timestamp. Multiple answers may exist so edits remain auditable.

### `evidence`

Stores the claim, research-question link, source URL/reference, source title, retrieval time, reliability assessment, notes, and status. Evidence is not itself a decision.

### `decisions`

Stores title, statement, rationale, and `Proposed`, `Accepted`, `Rejected`, `Superseded`, or `NeedsConfirmation` status.

### `trace_links`

Stores typed source and target entity locators plus a relationship: `Supports`, `DerivedFrom`, `Answers`, `Implements`, `Constrains`, `Contradicts`, `Supersedes`, or `Cites`. A project foreign key and application validation prevent cross-project edges.

### `documents`

Stores logical kind, relative output path, content hash, generation timestamp, and status. Generated content remains on disk; the row proves which authoritative snapshot produced it.

### `validation_runs`

Stores start/completion timestamps, pass/fail state, repair pass, and summary counts.

### `validation_findings`

Stores validation code, severity, message, relevant entity/document locator, status, and whether deterministic repair is available.

### `agent_runs`

Stores operation, provider/model configuration where available, start/completion time, status, response hash, token counts when reported, and sanitized trajectory path. Raw credentials and private environment values are never stored.

### `agent_activity_events`

Stores the bounded, sanitized operational timeline for a successful agent run using a run-local sequence, provider-neutral category, message, and timestamp. Activity inserts share the initial project transaction; failed and cancelled runs therefore leave no project or activity rows.

### `semantic_index_state`

Stores project/entity locator, authoritative content hash, indexed hash, embedding model/version, dimension, state (`Current`, `Stale`, `Missing`, `Disabled`), last attempt, last success, and sanitized error summary.

### `retrieval_events`

Stores operation, purpose, filters, requested `top_k`, candidate/selected counts, selected IDs, discarded-stale count, duration, and timestamp for evaluation.

### `workflow_runs`

Stores one run's project, effective approval policy, running/paused/complete/failed state, stable pause reason, and timestamps. A partial unique index permits at most one active or paused run per project.

### `decision_approvals`

Stores immutable approval or rejection authority, its reason, decision identity, and timestamp separately from the mutable decision status.

### `document_revisions`

Stores immutable generated and manual-override hashes. The latest revision source determines whether ordinary generation must protect a file, while older revisions retain its ownership history.

## Constraints and indexes

- Foreign keys are enabled and cascading deletes are limited to explicit project deletion.
- `display_id` is unique within a project and entity table.
- Questions and findings index `(project_id, status, priority/impact)` for active workflow queries.
- Trace links index both source and target locators.
- Semantic state is unique by `(project_id, entity_type, entity_id)`.
- Retrieval and agent event timestamps and sequences are indexed per project for inspection and evaluation exports.
- Active workflow runs are unique per project, and document revisions are indexed by project, path, and time.
- Enum values use checked text columns so exports remain readable while invalid magic strings are rejected.

## Migration policy

Migrations are embedded, numbered SQL files applied in a transaction, and recorded through SQLite `user_version`. A migration test creates an empty database, applies all migrations, verifies required tables and foreign keys, closes the file, reopens it, and confirms idempotent startup.

Before a stable release, additive schema correction is allowed. After a stable release, incompatible changes require an explicit migration, changelog entry marked breaking when appropriate, and an export/backup path.
