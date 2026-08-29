BEGIN IMMEDIATE;

CREATE TABLE agent_activity_events (
    agent_run_id TEXT NOT NULL REFERENCES agent_runs(id) ON DELETE CASCADE,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    sequence INTEGER NOT NULL CHECK (sequence > 0),
    kind TEXT NOT NULL CHECK (kind IN ('lifecycle', 'progress', 'warning')),
    message TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (agent_run_id, sequence)
);

CREATE INDEX agent_activity_project_sequence
    ON agent_activity_events(project_id, agent_run_id, sequence);

PRAGMA user_version = 2;
COMMIT;
