# Implementation Plan: Contextual Project Workbench

## Overview

Turn `project-init open` into a deterministic contextual workbench through small vertical slices. The plan first establishes project-scoped question and threshold operations, then builds a pure workspace state model, connects it to Ratatui and the existing workflow service, and finishes with compatibility documentation and the full Rust quality suite.

## Architecture Decisions

- Keep SQLite authoritative; the TUI invokes `ProjectService` and never coordinates SQL directly.
- Model terminal input as pure workspace actions and commands before applying workflow mutations.
- Reload one complete `ProjectSnapshot` after every accepted mutation and calculate feedback by comparing the before and after snapshots.
- Persist the clarification threshold in the existing `projects.clarification_threshold` column.
- Project the timeline from existing authoritative records and persisted agent activity; do not add a generic chat or event table.
- Keep question creation explicit and structured. Conservative defaults are medium impact, high uncertainty, and medium cost.
- Adding an open question does not infer a project lifecycle rollback.
- Preserve the `answer` CLI command and redirected JSON behavior.

## Dependency Graph

```text
Project threshold/query contract ----+
                                      +-> pure WorkspaceState/actions
Workflow ask-question contract -------+             |
                                                    v
Timeline/diff/queue projection ----------------> Ratatui rendering
                                                    |
                                                    v
                                      interactive CLI wiring and docs
```

## Task List

### Phase 1: Authoritative Operations

## Task 1: Persist and expose the clarification threshold

**Description:** Extend the project read model and workflow boundary so the workbench can read and adjust the existing project-scoped threshold without bypassing validation or changing the database schema.

**Acceptance criteria:**

- [ ] A loaded project exposes its persisted threshold, including the current default of 27.
- [ ] Updating a valid threshold persists across snapshot reload and database reopen.
- [ ] Values outside 1 through 125 fail without changing the stored threshold or revision.

**Verification:**

- [ ] Targeted domain and persistence contract tests pass.
- [ ] `cargo check` succeeds after the slice.

**Dependencies:** None.

**Files likely touched:** `src/domain/project.rs`, `src/storage/workflow.rs`, `src/workflow/mod.rs`, `tests/persistence_contract.rs`.

**Estimated scope:** Medium.

## Task 2: Add user-authored questions through the workflow boundary

**Description:** Promote the existing storage question insertion capability into an explicit workflow use case suitable for structured `/ask`, preserving project ownership and conservative visible defaults.

**Acceptance criteria:**

- [ ] A structured user question persists its prompt, rationale, impact, uncertainty, cost, and calculated priority.
- [ ] Conservative defaults resolve to medium impact, high uncertainty, and medium cost.
- [ ] Invalid or missing project input leaves authoritative state and display-ID sequences unchanged.

**Verification:**

- [ ] Targeted workflow and persistence tests prove success, rejection, and post-error recovery.
- [ ] Existing answer reconciliation tests remain green.

**Dependencies:** None.

**Files likely touched:** `src/workflow/mod.rs`, `src/storage/workflow.rs`, `src/storage/error.rs`, `tests/workflow_contract.rs`.

**Estimated scope:** Medium.

### Checkpoint: Authoritative Operations

- [ ] Threshold and question operations are reachable only through validated workflow APIs.
- [ ] Reopen and rollback behavior is tested.
- [ ] `cargo fmt --check`, `cargo check`, and targeted tests pass.

### Phase 2: Interactive Clarification Slice

## Task 3: Build the pure workspace transition model

**Description:** Introduce state, focus, section, composer, structured question draft, and command types whose transitions can be tested without a real terminal or database.

**Acceptance criteria:**

- [ ] Navigation and focus transitions remain bounded for empty and populated sections.
- [ ] Plain text answers the selected open question; `/answer`, `/ask`, and threshold actions parse predictably.
- [ ] `Enter` submits, modified Enter inserts a newline when reported, and rejected commands preserve composer content with an error.

**Verification:**

- [ ] Unit and contract tests exercise normal paths, empty state, invalid commands, and recovery.
- [ ] All new Rust functions, structs, and enums have meaningful documentation comments.

**Dependencies:** Tasks 1 and 2.

**Files likely touched:** `src/tui/mod.rs`, `tests/workspace_tui_contract.rs`.

**Estimated scope:** Medium.

## Task 4: Project the queue, timeline, and mutation diff

**Description:** Derive user-visible ordering and feedback from snapshots without introducing a second source of truth.

**Acceptance criteria:**

- [ ] Queue selection chooses the highest-priority consequential open question, then the highest-priority remaining open question.
- [ ] Timeline projection includes persisted activity, questions, answers, findings, and requirements, including equal-time records.
- [ ] Snapshot comparison reports added or status-changed questions, findings, requirements, traces, and project status.

**Verification:**

- [ ] Pure tests cover threshold boundaries, tied priorities/timestamps, no-open-question state, and multi-entity reconciliation diffs.
- [ ] Projection work is bounded by the current project snapshot and does not scan unrelated state.

**Dependencies:** Task 3.

**Files likely touched:** `src/tui/mod.rs`, `tests/workspace_tui_contract.rs`.

**Estimated scope:** Medium.

### Checkpoint: Interactive Clarification

- [ ] The answer and `/ask` flows are proven without a live terminal.
- [ ] Queue and diff invariants remain correct after rejected input.
- [ ] `cargo fmt --check`, `cargo check`, and targeted tests pass.

### Phase 3: Workbench Integration

## Task 5: Render and run the contextual workbench

**Description:** Replace the read-only overview loop with the workspace event loop, responsive panels, composer, structured `/ask` overlay, timeline, and deterministic workflow command application.

**Acceptance criteria:**

- [ ] `open` presents Overview, Questions, Findings, Requirements, and Activity sections with visible focus and keyboard help.
- [ ] Answer, ask, and threshold commands mutate through `ProjectService`, reload the snapshot, show feedback, and preserve usable selection.
- [ ] Quit and recoverable workflow errors restore the terminal safely without losing accepted state.

**Verification:**

- [ ] Ratatui `TestBackend` tests cover compact rendering, composer context, overlay defaults, and affected-entity diff feedback.
- [ ] A deterministic event-loop integration test or manual local terminal smoke test exercises the primary flow.

**Dependencies:** Tasks 1 through 4.

**Files likely touched:** `src/tui/mod.rs`, `src/main.rs`, `tests/workspace_tui_contract.rs`.

**Estimated scope:** Medium.

## Task 6: Update user contracts and compatibility documentation

**Description:** Document the interactive workflow, keyboard controls, known limitations, architecture boundary, and notable unreleased behavior without removing the non-interactive CLI path.

**Acceptance criteria:**

- [ ] README usage describes interactive answering, `/ask`, threshold changes, and CLI compatibility.
- [ ] Specification, architecture, known limitations, and changelog match verified behavior.
- [ ] The approved idea and this plan remain traceable under `Docs/ideas/`.

**Verification:**

- [ ] Documentation examples match actual commands and key bindings.
- [ ] The changelog follows `Docs/CHANGELOG_GUIDELINES.md`.

**Dependencies:** Task 5.

**Files likely touched:** `README.md`, `Docs/specification.md`, `Docs/architecture.md`, `Docs/known-limitations.md`, `CHANGELOG.md`.

**Estimated scope:** Medium.

### Checkpoint: Complete

- [ ] `cargo fmt --check` passes.
- [ ] `cargo check --all-features` passes.
- [ ] `cargo test --all-features` passes.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes.
- [ ] Fresh diff review finds no unresolved correctness, compatibility, comment, or scope issue.

## Risks and Mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Contextual composer submits to an unintended question | High | Show the target in the composer border/help and retain explicit `/answer` targeting. |
| TUI becomes a second business-logic layer | High | Emit typed commands and keep all mutations in `ProjectService`. |
| Question creation silently invents priority | High | Display editable conservative defaults before accepting `/ask`. |
| Snapshot reload loses selection | Medium | Restore by stable entity ID, then advance through the queue deliberately. |
| Timeline implies false causality | Medium | Show all records and treat tie-breaking as rendering-only. |
| Modified Enter is not distinguishable in a terminal | Medium | Test Crossterm event handling and retain an alternate explicit newline binding. |
| Existing uncommitted TUI and agent work is overwritten | High | Patch only current working files, inspect diffs after every slice, and avoid destructive Git operations. |

## Assumptions

- The current uncommitted initial-analysis implementation is in scope and must be preserved.
- The existing SQLite column is the authoritative home for the clarification threshold.
- `ProjectService` remains the application boundary; no new dependency is required.
- User-created questions may be added in any current project lifecycle state without automatically changing that state.
- The automated Rust workflow invocation authorizes progression through this reviewed plan without additional phase gates.

## Open Questions

No question blocks implementation. Terminal-specific modified-Enter behavior is treated as a verification risk with a reversible fallback binding.
