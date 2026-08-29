# Live Codex Brief Analysis

## Objective

Make `project-init new` open a responsive Ratatui creation session that runs one isolated Codex CLI analysis, displays a coding-agent-style rolling activity timeline, and persists the resulting project only after the complete structured response has been validated.

The primary user is a developer or product creator starting from an incomplete brief. Success means they can see that analysis is progressing, cancel immediately with `Esc`, and trust that a failed, timed-out, cancelled, or invalid run leaves no authoritative project or agent-run state.

## User Experience

The default `new` path launches the transient analysis TUI and invokes Codex. Stable application milestones are combined with selected, sanitized `codex exec --json` events. The timeline remains scrollable while analysis runs and is stored with the project after success so it can be inspected after reopening.

`Esc` immediately terminates the Codex subprocess and exits. The default timeout is five minutes. Failure never falls back automatically. Users who intentionally need the deterministic analyzer select `--offline`.

## Commands

```text
Codex analysis: cargo run -- --data-dir .project-init new --brief <file> [--name <name>]
Offline analysis: cargo run -- --data-dir .project-init new --brief <file> [--name <name>] --offline
Build: cargo build --all-features
Format: cargo fmt --check
Check: cargo check --all-features
Test: cargo test --all-features
Lint: cargo clippy --all-targets --all-features -- -D warnings
```

The native Codex executable is resolved from the full path in `CODEX_BIN` when set and otherwise discovered on `PATH`, including the executable bundled inside a Windows npm installation.

## Smallest Viable Architecture

```text
CLI `new`
   -> transient CreationSession (memory only)
   -> AgentClient task
   -> `codex exec --json --output-schema ... --ephemeral --sandbox read-only -`
   -> tolerant JSONL-to-ActivityEvent adapter
   -> Ratatui rolling timeline
   -> validate final StructuredAnalysis
   -> one SQLite transaction:
        project + findings + questions + successful agent run + activity history
```

`AgentClient` is analogous to a C# interface: workflow code depends on the behavior, while `CodexCliClient` supplies the subprocess implementation and deterministic fixtures supply tests. `ActivityEvent` is provider-neutral so the TUI never depends on Codex JSON shapes. `Result<T, E>` carries subprocess, timeout, cancellation, and validation failures explicitly rather than using exceptions.

## Modules and Boundaries

- `agents`: request/result types, provider-neutral client trait, Codex subprocess adapter, JSONL translation, timeout and cancellation.
- `workflow`: offline analysis, structured-response validation, and the atomic successful-initialization use case.
- `storage`: schema migrations and the single transaction that persists successful analysis and activity history.
- `tui`: transient creation state, scrolling, rendering, keyboard handling, and stable milestone presentation.
- `cli`: `--offline` parsing, Codex configuration, runtime setup, and command dispatch.

Provider payloads do not enter `domain`, SQL does not enter `tui`, and brief contents are sent over stdin rather than interpolated into a shell command.

## Core Data Contracts

- `AgentRequest`: operation name, focused prompt, response schema, and five-minute deadline.
- `ActivityEvent`: bounded sequence, stable category, sanitized user-facing message, and timestamp.
- `AgentResult`: validated final response text, optional model/thread metadata, token counts when available, and retained activity events.
- `CreationSession`: in-memory status, start time, spinner state, activity history, scroll position, and terminal outcome.
- `StructuredAnalysis`: schema-validated findings accepted by workflow persistence.

Activity history is bounded during execution to prevent unbounded memory growth. Successful history is durable; failed or cancelled history remains visible until exit but is not persisted because no project exists.

## Error Handling and Invariants

- Spawn failure, missing authentication, non-zero exit, malformed JSONL, timeout, cancellation, missing final output, or invalid structured analysis creates no database rows.
- Unknown well-formed Codex event types are ignored or summarized and do not fail the run.
- Malformed individual event lines become sanitized warning events; the final schema-validated response remains authoritative.
- The Codex child is configured to terminate on task drop and is explicitly killed and awaited on cancellation or timeout.
- Process arguments are passed directly without shell interpolation.
- Output, event count, line length, and persisted message sizes are bounded.
- Only operational summaries are shown or stored; chain-of-thought and raw credentials are never retained.
- Successful initialization commits the project, findings, questions, agent-run audit row, and activity events atomically.
- Existing `open`, `inspect`, `answer`, generation, validation, and deterministic analysis behavior remains compatible.

## Testing Strategy

- Unit tests cover JSONL event translation, unknown and malformed events, history bounds, spinner/session transitions, and structured-response validation.
- Storage tests inject invalid analysis before initialization and verify zero mutation; successful initialization verifies atomic project, agent-run, and activity persistence across reopen.
- Agent integration tests use a deterministic helper executable or fixture client for success, timeout, cancellation, non-zero exit, and invalid final output without a live model call.
- Ratatui test-backend assertions cover the rolling status presentation and scroll behavior.
- CLI parsing tests cover default Codex mode and explicit `--offline` behavior.

## Implementation Tasks

1. Define provider-neutral agent and activity contracts plus tolerant Codex JSONL translation. Verify with parsing and bounded-history tests.
2. Add the migration and atomic storage operation for successful analyzed-project creation and persisted activity history. Verify success, rollback, and reopen behavior.
3. Implement the isolated Codex CLI client with schema output, stdin prompting, five-minute timeout, and immediate cancellation. Verify with deterministic subprocess fixtures.
4. Implement the transient Ratatui creation state and rolling timeline. Verify pure state transitions and test-backend rendering.
5. Wire `new` to Codex by default and retain deterministic analysis behind `--offline`. Verify CLI and end-to-end fixture paths.
6. Update user and architecture documentation, changelog, and run the full quality suite.

## Non-Goals

- Answer-driven re-analysis.
- Codex app-server integration, session resume, or automatic retry.
- Automatic deterministic fallback.
- Multiple analysis passes or an autonomous agent loop.
- Raw reasoning or provider payload persistence.
- Semantic retrieval and LanceDB integration.

## Sources

- Codex non-interactive mode, JSONL events, schema output, ephemeral sessions, and read-only sandbox: <https://learn.chatgpt.com/docs/non-interactive-mode>
- Codex CLI command reference: <https://learn.chatgpt.com/docs/developer-commands?surface=cli>
- Tokio child-process cleanup and cancellation: <https://docs.rs/tokio/latest/tokio/process/struct.Child.html>
- Ratatui asynchronous event handling: <https://ratatui.rs/tutorials/counter-async-app/>

## Confirmed Decisions

- Codex runs during `new`; `--offline` selects deterministic analysis.
- The TUI shows a spinner with a detailed rolling status history.
- The timeout is five minutes.
- Activity history persists only with a successfully committed project.
- `Esc` cancels immediately.
- Failure stops without changing authoritative project state.
- Answer-driven re-analysis is deferred.
