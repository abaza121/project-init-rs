# Implementation Plan: Parallel Auto Answer

## Status

Implemented and verified on 2026-08-29. All six tasks and their checkpoints are complete; the quality-improvement threshold remains a separate live evaluation decision.

## Overview

Add a fixed three-worker automatic clarification pipeline with dependency-aware Codex planning, one worker per question, batch judgment, atomic answer adoption, and a structured Ratatui progress board. Preserve the existing single-question research contract and headless automation behavior.

## Architecture Decisions

- Add an `AutoAnswerClient` beside `ResearchClient` so existing callers remain source-compatible and deterministic tests can exercise either mode.
- Share the auto-answer client through `Arc<dyn AutoAnswerClient>`; Tokio's existing `JoinSet` provides bounded concurrency without adding a dependency.
- Use a dedicated structured progress channel instead of adding fields to persisted `ActivityEvent` or parsing display strings.
- Keep all coordinator, worker, and judge output provisional until one service-level batch adoption call.
- Reuse the current Codex structured-prompt executor with new strict JSON schemas and prompts.
- Preserve headless behavior when standard input or output is not a terminal; only interactive invocations open Ratatui.

## Dependency Graph

```text
Provider-neutral contracts
  -> Codex schemas and prompts
  -> concurrent runner orchestration
      -> atomic storage adoption
      -> structured progress events
          -> workbench command/runtime
          -> TUI progress board
              -> terminal-aware CLI entry
```

## Task 1: Define bounded planning, judgment, and progress contracts

**Description:** Add provider-neutral request and response types, strict constructors and parsers, the additive `AutoAnswerClient` trait, and structured auto-answer progress types.

**Acceptance criteria:**

- [x] Plans contain one to three unique eligible question IDs and must include the current blocking question.
- [x] Judged batches contain every candidate question exactly once with a valid cited answer.
- [x] Progress identifies one coordinator, worker slot, or judge without modifying persisted `ActivityEvent`.

**Verification:** `cargo test --test agent_contract`

**Dependencies:** None.

**Files likely touched:** `src/agents/mod.rs`, `src/workflow/runner.rs`, `tests/agent_contract.rs`.

**Estimated scope:** Medium.

## Task 2: Prove and implement atomic batch answer adoption

**Description:** Add a service/storage operation that validates a judged batch, skips questions that are no longer open, and reconciles all applicable answers and evidence in one SQLite transaction.

**Acceptance criteria:**

- [x] Multiple applicable answers, citations, requirements, and traces commit together.
- [x] A failure or invalid project/question combination leaves every authoritative table unchanged.
- [x] Already-closed questions are skipped without blocking other applicable answers.

**Verification:** `cargo test --test workflow_contract research_batch`

**Dependencies:** Task 1.

**Files likely touched:** `src/storage/workflow.rs`, `src/workflow/mod.rs`, `tests/workflow_contract.rs`.

**Estimated scope:** Medium.

## Checkpoint: Contracts and authority

- [x] Targeted contract tests pass.
- [x] No database migration or dependency was added.
- [x] Existing single-answer reconciliation tests still pass.

## Task 3: Add Codex coordinator and judge adapters

**Description:** Define strict schemas and prompts for selecting a dependency-safe batch and judging all worker candidates, then implement the new trait through the existing read-only structured Codex boundary.

**Acceptance criteria:**

- [x] Coordinator and judge use ephemeral read-only Codex invocations with existing sandbox and skill settings.
- [x] Prompts delimit project and candidate JSON as untrusted context.
- [x] Malformed, duplicate, missing, or extra IDs are rejected before workflow adoption.

**Verification:** `cargo test agents::codex::tests`

**Dependencies:** Task 1.

**Files likely touched:** `src/agents/codex.rs`, `src/agents/mod.rs`.

**Estimated scope:** Medium.

## Task 4: Orchestrate three workers, retries, judgment, and cancellation

**Description:** Add the parallel runner path using a fixed three-slot `JoinSet`, provisional worker results, one retry per failed worker, one judge retry, structured progress, and final atomic adoption.

**Acceptance criteria:**

- [x] Up to three different questions execute concurrently and the judge begins only after they finish.
- [x] Retry counts and failure pause reasons match the specification.
- [x] Cancellation before adoption returns a cancelled outcome and persists no batch answers.

**Verification:** `cargo test --test runner_contract parallel_auto_answer`

**Dependencies:** Tasks 1-3.

**Files likely touched:** `src/workflow/runner.rs`, `tests/runner_contract.rs`.

**Estimated scope:** Medium.

## Checkpoint: Core orchestration

- [x] Contract, storage, adapter, and runner tests pass.
- [x] A concurrency probe observes at most three active workers and at least two when multiple questions exist.
- [x] Failure and cancellation leave authority unchanged.

## Task 5: Add workbench activation and progress lanes

**Description:** Add `/auto-answer`, explicit first-run policy selection, background execution mode wiring, bounded lane state, and a dedicated auto-answer overlay.

**Acceptance criteria:**

- [x] `/auto-answer` never silently selects an approval policy.
- [x] Coordinator, three workers, and judge render separate latest-state lanes with sanitized detail.
- [x] `Esc` cancels the entire active auto-answer execution and the TUI stays responsive.

**Verification:** `cargo test --test workspace_tui_contract auto_answer`

**Dependencies:** Task 4.

**Files likely touched:** `src/tui/workspace.rs`, `src/tui/mod.rs`, `tests/workspace_tui_contract.rs`.

**Estimated scope:** Medium.

## Task 6: Route terminal-aware CLI auto-answer into the TUI

**Description:** Start the workbench progress experience for terminal-attached `run --auto-answer` and `new --run --auto-answer`, while retaining the current headless runner for automation.

**Acceptance criteria:**

- [x] Interactive auto-answer starts the TUI execution automatically with the requested approval policy.
- [x] Non-terminal auto-answer does not initialize Ratatui and retains final serialized status output.
- [x] Offline conflicts and existing argument parsing remain unchanged.

**Verification:** `cargo test --bin project-init auto_answer`

**Dependencies:** Task 5.

**Files likely touched:** `src/main.rs`, `src/tui/mod.rs`.

**Estimated scope:** Small.

## Checkpoint: Complete feature

- [x] Targeted tests pass.
- [x] `cargo fmt --check`, `cargo check`, `cargo test`, and strict Clippy pass.
- [x] Fresh diff review finds no missing documentation comments, compatibility regression, or unrelated cleanup.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Concurrent research exceeds provider limits | High | Fixed ceiling of three; fail closed with visible retry state |
| Planner chooses semantically dependent questions | Medium | Require current blocker, supply full question context, and let judge reject inconsistency |
| One stale question invalidates useful work | Medium | Skip no-longer-open identities at transactional adoption |
| Worker activity interleaves unreadably | Medium | Dedicated actor-keyed progress events and latest-state lanes |
| Judge worsens otherwise valid answers | High | Preserve candidates provisionally, validate exact membership, and document blind quality evaluation |
| Dirty worktree causes accidental overwrite | High | Use scoped patches, inspect diffs per task, and avoid branch or commit operations |

## Open Questions

None block implementation. Paid live quality evaluation and its acceptable improvement threshold are explicitly deferred.
