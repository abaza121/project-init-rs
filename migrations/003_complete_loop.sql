BEGIN IMMEDIATE;

CREATE TABLE workflow_runs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    approval_policy TEXT NOT NULL CHECK (approval_policy IN ('strict', 'consequential', 'autonomous')),
    status TEXT NOT NULL CHECK (status IN ('running', 'paused', 'complete', 'failed')),
    pause_reason TEXT,
    started_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT
);

CREATE UNIQUE INDEX workflow_runs_one_active
    ON workflow_runs(project_id)
    WHERE status IN ('running', 'paused');

CREATE INDEX workflow_runs_project_time
    ON workflow_runs(project_id, started_at);

CREATE TABLE decision_approvals (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    decision_id TEXT NOT NULL REFERENCES decisions(id) ON DELETE CASCADE,
    outcome TEXT NOT NULL CHECK (outcome IN ('approved', 'rejected')),
    reason TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX decision_approvals_decision_time
    ON decision_approvals(project_id, decision_id, created_at);

CREATE TABLE document_revisions (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    document_id TEXT REFERENCES documents(id) ON DELETE SET NULL,
    relative_path TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    source TEXT NOT NULL CHECK (source IN ('generated', 'manual_override')),
    previous_hash TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX document_revisions_path_time
    ON document_revisions(project_id, relative_path, created_at);

PRAGMA user_version = 3;
COMMIT;
