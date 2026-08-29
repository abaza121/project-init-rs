# Specification: Contextual Project Workbench

## Objective

Turn `project-init open <project-id>` into a long-lived, keyboard-first workspace for a developer or product creator refining one project. Success means the user can answer a selected question, complete the consequential and remaining clarification queue, create a structured user-authored question, adjust the project's consequential threshold, inspect the authoritative history, and resume later without losing accepted state or provenance.

The workbench feels conversational because it has a persistent composer and chronological projection, but it is not a generic chat client. Every accepted input maps to an explicit workflow operation. The deterministic MVP does not invoke Codex after an answer or infer intent from arbitrary prose.

## Acceptance Criteria

1. `open` renders Overview, Questions, Findings, Requirements, and Activity sections with visible focus, keyboard navigation, a contextual composer, and safe terminal restoration.
2. Plain composer text answers the selected open question. `/answer <question-id> <text>` targets a question explicitly.
3. Answer reconciliation remains one atomic workflow operation. On success, the TUI reloads the snapshot, displays added and changed authoritative entities, and selects the next highest-priority open question.
4. Queue advancement handles all open questions: scores at or above the persisted threshold first, then lower-priority questions.
5. `/ask` opens structured capture with editable medium impact, high uncertainty, and medium cost defaults. Accepted input creates an open question without silently changing the project lifecycle.
6. The clarification threshold accepts values from 1 through 125, persists per project, survives reopen, and rejects invalid values without mutation.
7. The projected timeline includes persisted agent activity, findings, questions, answers, and requirements. Equal-time records are all visible; stable tie-breaking exists only for deterministic rendering.
8. `Enter` submits. `Ctrl+Enter` adds a newline when the terminal reports that distinction; an alternate explicit newline binding may be retained for terminals that do not.
9. Recoverable parsing, validation, storage, and reconciliation errors keep the workbench open, preserve unsubmitted composer content where useful, and do not partially mutate authoritative state.
10. The existing `answer` command, non-interactive `open` JSON output, initial-analysis TUI, stored IDs, and SQLite schema remain compatible.

## Non-Goals

- Agent re-analysis, Codex session resume, or natural-language intent classification.
- Generic persisted chat messages or a new event-store schema.
- Editing, deleting, or silently overwriting immutable answers.
- Mouse-first navigation.
- Document generation, validation, evidence, or decision mutation from the first workbench release.
- Automatic lifecycle rollback when a user creates a question.

## Technology and Dependencies

- Rust 2024 with minimum Rust 1.88.
- Ratatui 0.30.2 for rendering and terminal lifecycle.
- Crossterm 0.29 for keyboard events.
- Rusqlite 0.40.2 with bundled SQLite for authoritative persistence.
- Existing `anyhow`, `thiserror`, `chrono`, `uuid`, and Tokio dependencies where already used.

No new dependency is justified for this feature. Pure Rust state transitions and Ratatui's existing `TestBackend` cover the required behavior.

## Commands

```text
Build:   cargo build --all-features
Format:  cargo fmt --check
Check:   cargo check --all-features
Test:    cargo test --all-features
Lint:    cargo clippy --all-targets --all-features -- -D warnings
Open:    cargo run -- --data-dir .project-init open <project-id>
Answer:  cargo run -- --data-dir .project-init answer <question-id> "<answer>"
```

## Project Structure

```text
src/domain/       Project and clarification value types and invariants
src/storage/      Atomic SQLite reads and mutations
src/workflow/     Application use cases invoked by CLI and TUI
src/tui/          Pure workspace state, projections, rendering, and event loop
src/main.rs       CLI detection and workbench composition
tests/            Cross-boundary workflow, persistence, and TUI contracts
Docs/ideas/       Approved concept and implementation plan
Docs/             Product, architecture, and feature specifications
```

## Smallest Viable Architecture

```text
Crossterm KeyEvent
      |
      v
WorkspaceState::handle_key  -- pure --> WorkspaceCommand
                                      /       |       \
                                     v        v        v
                                  answer     ask    set threshold
                                      \       |       /
                                       ProjectService
                                             |
                                      atomic SqliteStore
                                             |
                                      ProjectSnapshot reload
                                             |
                             queue + timeline + mutation diff
                                             |
                                      Ratatui rendering
```

For a C# programmer, `WorkspaceCommand` is a closed discriminated union representing user intent, similar to a hierarchy of sealed command records but exhaustively matched by the compiler. `ProjectService` is the application-service boundary. `Result<T, E>` exposes recoverable workflow and terminal failures in function signatures rather than using exceptions. `WorkspaceState` owns its current snapshot and strings; render functions borrow them temporarily, so presentation cannot outlive or mutate authoritative data accidentally.

## Core Types and Ownership

- `WorkspaceSection`: closed set of Overview, Questions, Findings, Requirements, and Activity views.
- `WorkspaceFocus`: navigation, content, composer, structured question overlay, or command menu.
- `WorkspaceState`: disposable in-memory selection, focus, composer, overlay, notice, activity, and current snapshot.
- `WorkspaceCommand`: validated answer, ask-question, threshold update, or quit request emitted by pure input handling.
- `QuestionDraft`: prompt, rationale, impact, uncertainty, cost, and active field for structured capture.
- `TimelineEntry`: rendered projection with timestamp, stable rendering key, category, and message; it is derived and never authoritative.
- `SnapshotDiff`: added or status-changed questions, findings, requirements, traces, and project status produced by comparing two snapshots.

## State Model and Invariants

### Identity and ordering

- Stable database IDs and display IDs identify authoritative entities per project.
- Question queue order is descending priority, then stable display ID for deterministic ties.
- Consequential membership is `priority >= project.clarification_threshold`.
- Timeline order is ascending timestamp, then category and stable entity key only to make rendering reproducible. The tie-breaker has no causal meaning.

### Authoritative and derived state

- SQLite owns projects, thresholds, findings, questions, answers, requirements, traces, and persisted agent activity.
- `ProjectSnapshot`, queue positions, timeline entries, selection indexes, composer text, overlays, and diffs are derived or transient.
- After every accepted mutation, the TUI discards the stale snapshot and reloads authoritative state.

### Legal transitions

- An answer is accepted only for an open question and retains immutable history.
- An invalid answer or command changes neither authoritative state nor stable-ID sequences.
- A new question is open and project-scoped. Creation does not infer a lifecycle transition.
- A valid threshold update changes only the project threshold, update timestamp, and revision.
- Quit changes no authoritative state.

### Capacity and complexity

- Composer, answer, prompt, and rationale input reuse bounded domain or storage validation; the TUI must not bypass those limits.
- Snapshot-derived work is proportional to entities in the current project, never all projects.
- No new unbounded message history or secondary index is introduced.
- Selection and scroll offsets saturate at valid boundaries, including empty sections.

## Error Handling

- Parser errors become visible notices and preserve input for correction.
- Invalid question targets, closed questions, invalid thresholds, and empty required fields return typed workflow or storage errors without partial mutation.
- Workflow errors remain recoverable inside the event loop. Terminal I/O setup, draw, or event-read errors unwind through Ratatui's safe restoration boundary.
- Snapshot reload failure after a committed mutation is reported as a terminal error; reopening reloads the already-committed authoritative state.
- Production paths do not use `unwrap`, `expect`, or deliberate panics.

## Policy Decisions

- Plain text has meaning only when an open question is selected; otherwise the UI requests an explicit command.
- `/ask` always shows its structured overlay before persistence, including when initial prompt text follows the command.
- User-authored question rationale defaults to a visible workbench-origin explanation and remains editable.
- Consequential threshold adjustment persists immediately after validation.
- Lower-priority open questions follow the consequential queue automatically.
- Equal-time timeline rows are all rendered and are not collapsed.

## Testing Strategy

- Domain and persistence contract tests cover threshold boundaries, reopen, rejection, revision consistency, and post-error recovery.
- Workflow contract tests cover conservative question defaults, explicit values, project scoping, and answer compatibility.
- Pure TUI tests cover sections, focus, bounded navigation, composer parsing, multiline input, structured overlay transitions, queue ordering, timeline ties, and snapshot diffs.
- Ratatui `TestBackend` tests cover compact rendering, contextual target visibility, default question fields, help, and reconciliation feedback.
- Existing creation, agent, workflow, persistence, generation, and CLI-adjacent tests remain regression coverage.
- The full Cargo format, check, test, and Clippy suite is required before completion.

## Code Style

Every function, struct, and enum, including private and test items, has a meaningful Rust documentation comment:

```rust
/// Chooses the next open question by consequential priority before the remaining queue.
fn next_question_index(snapshot: &ProjectSnapshot, threshold: u16) -> Option<usize> {
    // implementation
}
```

Inline comments are reserved for non-obvious invariants or functions longer than 18 lines. Callers borrow snapshots and strings unless ownership naturally transfers into durable or long-lived state.

## Compatibility Surfaces

- Existing command names, arguments, and redirected output remain valid.
- Existing SQLite databases migrate without a new schema revision for this feature.
- Stable entity ID allocation and immutable answer history remain unchanged.
- Initial Codex analysis activity and creation cancellation remain functional.
- `run_with_activity` may become an internal compatibility wrapper, but callers still receive safe terminal behavior.

## Evaluator Risks

- Empty projects or sections causing index underflow or invalid selection.
- Selected questions becoming answered after a snapshot reload.
- Threshold boundaries at 1, 27, and 125 and rejected neighbors 0 and 126.
- Equal priority and equal timestamp records disappearing or being treated as duplicates.
- Failed `/answer`, `/ask`, or threshold input consuming display IDs or clearing correctable input.
- Repeated answers advancing past consequential questions incorrectly.
- Questions added after a project reaches Planning remaining actionable without corrupting lifecycle state.
- Long answers, prompts, and activity text breaking compact terminal layouts.
- Terminals reporting modified Enter inconsistently.

## Boundaries

- **Always:** validate before mutation, keep SQLite authoritative, preserve immutable history, reload after accepted writes, add behavioral tests first, document every Rust item, inspect the dirty diff, and run the full quality suite.
- **Ask first:** incompatible schema changes, new dependencies, destructive history changes, live paid model calls, publishing, deployment, or removing user data.
- **Never:** infer consequential user intent, store raw chain-of-thought, bypass the workflow boundary from TUI code, weaken existing tests, overwrite unrelated user changes, or claim causal ordering for tied timeline records.

## Deliverables

- `Docs/ideas/contextual-project-workbench.md`
- `Docs/ideas/contextual-project-workbench-implementation-plan.md`
- This feature specification.
- Workflow and persistence support for questions and thresholds.
- Stateful workbench TUI and CLI integration.
- Behavior-focused contract and rendering tests.
- Updated README, architecture, product specification, known limitations, and changelog.

## Open Questions

No material product question remains. Modified-Enter portability is a verification item with an explicit fallback rather than an architecture blocker.
