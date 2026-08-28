BEGIN IMMEDIATE;

CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    brief TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN (
        'draft', 'analyzing', 'awaiting_clarification', 'researching',
        'planning', 'generating', 'validating', 'needs_user_input', 'complete'
    )),
    retrieval_mode TEXT NOT NULL CHECK (retrieval_mode IN ('relational', 'semantic')),
    clarification_threshold INTEGER NOT NULL DEFAULT 27 CHECK (clarification_threshold BETWEEN 1 AND 125),
    repair_limit INTEGER NOT NULL DEFAULT 2 CHECK (repair_limit BETWEEN 0 AND 10),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE entity_sequences (
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    prefix TEXT NOT NULL,
    next_value INTEGER NOT NULL CHECK (next_value > 0),
    PRIMARY KEY (project_id, prefix)
);

CREATE TABLE findings (
    id TEXT PRIMARY KEY,
    display_id TEXT NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    kind TEXT NOT NULL CHECK (kind IN (
        'confirmed_fact', 'requirement', 'assumption', 'unknown',
        'constraint', 'risk', 'research_question'
    )),
    statement TEXT NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN (
        'user_brief', 'user_answer', 'research', 'agent_inference', 'derived', 'system'
    )),
    source_reference TEXT NOT NULL,
    confidence TEXT NOT NULL CHECK (confidence IN ('low', 'medium', 'high')),
    impact TEXT NOT NULL CHECK (impact IN ('low', 'medium', 'high')),
    status TEXT NOT NULL CHECK (status IN ('active', 'confirmed', 'resolved', 'superseded', 'rejected')),
    requires_confirmation INTEGER NOT NULL CHECK (requires_confirmation IN (0, 1)),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (project_id, display_id)
);

CREATE INDEX findings_active_priority
    ON findings(project_id, status, impact, kind);
CREATE INDEX findings_statement_lookup
    ON findings(project_id, kind, statement COLLATE NOCASE);

CREATE TABLE requirements (
    id TEXT PRIMARY KEY,
    display_id TEXT NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    statement TEXT NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN (
        'user_brief', 'user_answer', 'research', 'agent_inference', 'derived', 'system'
    )),
    source_reference TEXT NOT NULL,
    priority TEXT NOT NULL CHECK (priority IN ('low', 'medium', 'high')),
    status TEXT NOT NULL CHECK (status IN ('proposed', 'active', 'satisfied', 'superseded', 'rejected')),
    rationale TEXT,
    acceptance_criteria TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (project_id, display_id)
);

CREATE INDEX requirements_active_priority
    ON requirements(project_id, status, priority);

CREATE TABLE questions (
    id TEXT PRIMARY KEY,
    display_id TEXT NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    finding_id TEXT REFERENCES findings(id),
    prompt TEXT NOT NULL,
    rationale TEXT NOT NULL,
    impact TEXT NOT NULL CHECK (impact IN ('low', 'medium', 'high')),
    uncertainty TEXT NOT NULL CHECK (uncertainty IN ('low', 'medium', 'high')),
    cost_of_being_wrong TEXT NOT NULL CHECK (cost_of_being_wrong IN ('low', 'medium', 'high')),
    priority_score INTEGER NOT NULL CHECK (priority_score BETWEEN 1 AND 125),
    status TEXT NOT NULL CHECK (status IN ('open', 'answered', 'deferred', 'superseded')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (project_id, display_id)
);

CREATE INDEX questions_attention
    ON questions(project_id, status, priority_score DESC);

CREATE TABLE answers (
    id TEXT PRIMARY KEY,
    display_id TEXT NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    question_id TEXT NOT NULL REFERENCES questions(id),
    answer_text TEXT NOT NULL,
    answer_source TEXT NOT NULL CHECK (answer_source IN ('user', 'imported', 'system')),
    resolves_question INTEGER NOT NULL CHECK (resolves_question IN (0, 1)),
    notes TEXT,
    created_at TEXT NOT NULL,
    UNIQUE (project_id, display_id)
);

CREATE TABLE evidence (
    id TEXT PRIMARY KEY,
    display_id TEXT NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    research_question_id TEXT REFERENCES findings(id),
    claim TEXT NOT NULL,
    source TEXT NOT NULL,
    source_title TEXT NOT NULL,
    retrieved_at TEXT NOT NULL,
    reliability TEXT NOT NULL CHECK (reliability IN ('low', 'medium', 'high')),
    notes TEXT,
    status TEXT NOT NULL CHECK (status IN ('active', 'stale', 'rejected', 'superseded')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (project_id, display_id)
);

CREATE TABLE decisions (
    id TEXT PRIMARY KEY,
    display_id TEXT NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    statement TEXT NOT NULL,
    rationale TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('proposed', 'accepted', 'rejected', 'superseded', 'needs_confirmation')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (project_id, display_id)
);

CREATE TABLE trace_links (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    relationship TEXT NOT NULL CHECK (relationship IN (
        'supports', 'derived_from', 'answers', 'implements',
        'constrains', 'contradicts', 'supersedes', 'cites'
    )),
    created_at TEXT NOT NULL,
    UNIQUE (project_id, source_type, source_id, target_type, target_id, relationship)
);

CREATE INDEX trace_links_source
    ON trace_links(project_id, source_type, source_id);
CREATE INDEX trace_links_target
    ON trace_links(project_id, target_type, target_id);

CREATE TABLE documents (
    id TEXT PRIMARY KEY,
    display_id TEXT NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,
    relative_path TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'stale', 'missing')),
    generated_at TEXT NOT NULL,
    UNIQUE (project_id, display_id),
    UNIQUE (project_id, relative_path)
);

CREATE TABLE validation_runs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('running', 'passed', 'failed')),
    repair_pass INTEGER NOT NULL CHECK (repair_pass >= 0),
    high_count INTEGER NOT NULL DEFAULT 0 CHECK (high_count >= 0),
    medium_count INTEGER NOT NULL DEFAULT 0 CHECK (medium_count >= 0),
    low_count INTEGER NOT NULL DEFAULT 0 CHECK (low_count >= 0),
    started_at TEXT NOT NULL,
    completed_at TEXT
);

CREATE TABLE validation_findings (
    id TEXT PRIMARY KEY,
    display_id TEXT NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    validation_run_id TEXT NOT NULL REFERENCES validation_runs(id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    severity TEXT NOT NULL CHECK (severity IN ('low', 'medium', 'high')),
    message TEXT NOT NULL,
    entity_type TEXT,
    entity_id TEXT,
    status TEXT NOT NULL CHECK (status IN ('open', 'repaired', 'accepted', 'superseded')),
    repairable INTEGER NOT NULL CHECK (repairable IN (0, 1)),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (project_id, display_id)
);

CREATE TABLE agent_runs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    operation TEXT NOT NULL,
    provider TEXT NOT NULL,
    model TEXT,
    status TEXT NOT NULL CHECK (status IN ('running', 'succeeded', 'failed', 'invalid_response')),
    response_hash TEXT,
    input_tokens INTEGER CHECK (input_tokens IS NULL OR input_tokens >= 0),
    output_tokens INTEGER CHECK (output_tokens IS NULL OR output_tokens >= 0),
    trajectory_path TEXT,
    started_at TEXT NOT NULL,
    completed_at TEXT
);

CREATE TABLE semantic_index_state (
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    indexed_hash TEXT,
    embedding_model TEXT,
    embedding_version TEXT,
    embedding_dimension INTEGER CHECK (embedding_dimension IS NULL OR embedding_dimension > 0),
    state TEXT NOT NULL CHECK (state IN ('current', 'stale', 'missing', 'disabled')),
    last_attempt_at TEXT,
    last_success_at TEXT,
    error_summary TEXT,
    PRIMARY KEY (project_id, entity_type, entity_id)
);

CREATE TABLE retrieval_events (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    operation TEXT NOT NULL,
    purpose TEXT NOT NULL,
    entity_types TEXT NOT NULL,
    top_k_requested INTEGER NOT NULL CHECK (top_k_requested > 0),
    candidates_returned INTEGER NOT NULL CHECK (candidates_returned >= 0),
    records_selected INTEGER NOT NULL CHECK (records_selected >= 0),
    selected_entity_ids TEXT NOT NULL,
    stale_discarded INTEGER NOT NULL CHECK (stale_discarded >= 0),
    duration_ms INTEGER NOT NULL CHECK (duration_ms >= 0),
    created_at TEXT NOT NULL
);

CREATE INDEX retrieval_events_project_time
    ON retrieval_events(project_id, created_at);
CREATE INDEX agent_runs_project_time
    ON agent_runs(project_id, started_at);

PRAGMA user_version = 1;
COMMIT;
