# Implementation Plan: Complete Project Loop

## Overview

Implement the confirmed brief-to-documents loop as small vertical slices. The dependency order is persisted vocabulary first, then pure planning, then artifact ownership, then execution and CLI composition. Every slice leaves existing commands operational and has a narrow verification command.

## Architecture Decisions

- Derive the next step with a pure planner; perform mutations only through `ProjectService` and `SqliteStore`.
- Compile the first artifact graph into Rust so paths and dependency closure are closed, testable vocabularies.
- Store each run's approval policy for deterministic resumption.
- Preserve manual edits as immutable revisions and never overwrite them during normal generation.
- Permit Codex tools through `workspace-write` without dangerous bypass flags; treat all resulting files and manifests as untrusted proposals.
- Use existing dependencies and stable Rust 1.88; add no crate unless later evidence makes it necessary.

## Dependency Graph

```text
Migration + domain records
        |
        +--> persistence operations --> planner snapshot
        |                                  |
        +--> artifact registry ------------+--> runner
        |                                         |
        +--> revisions + writer ------------------+
        |                                         |
        +--> validation history + repairs --------+
                                                  |
                                      CLI + Codex adapter
```

## Task 1: Persist resumable workflow vocabulary

**Description:** Add the additive migration and domain records for workflow runs, document revisions, evidence, decisions, documents, and validation history, then expose them through the project snapshot.

**Acceptance criteria:**

- [ ] Version-two databases migrate without losing existing rows.
- [ ] Run policy/status and document revision values reject corrupt persisted text.
- [ ] Records survive close and reopen.

**Verification:** `cargo test --all-features --test persistence_contract complete_loop`

**Dependencies:** None.

**Files likely touched:** `migrations/003_complete_loop.sql`, `src/domain/loop.rs`, `src/domain/mod.rs`, `src/storage/sqlite.rs`, `tests/persistence_contract.rs`.

**Estimated scope:** Medium.

## Task 2: Plan one deterministic next step

**Description:** Introduce the fixed artifact registry and pure workflow planner, including question and approval precedence, generation, validation, repair, and completion.

**Acceptance criteria:**

- [ ] Identical snapshots produce identical next steps.
- [ ] Policy modes pause on the correct decision states.
- [ ] Status inspection performs no mutation.

**Verification:** `cargo test --all-features --test loop_contract planner`

**Dependencies:** Task 1.

**Files likely touched:** `src/workflow/loop.rs`, `src/workflow/mod.rs`, `src/documents/artifacts.rs`, `tests/loop_contract.rs`.

**Estimated scope:** Medium.

## Task 3: Start, pause, resume, and finish runs atomically

**Description:** Add storage and service operations that enforce one active run per project, retain the selected policy, and apply one planned lifecycle step idempotently.

**Acceptance criteria:**

- [ ] Repeated resume preserves run identity and policy.
- [ ] A conflicting policy is rejected without mutation.
- [ ] Restarting the database returns the same next step.

**Verification:** `cargo test --all-features --test loop_contract run_`

**Dependencies:** Tasks 1-2.

**Files likely touched:** `src/storage/loop.rs`, `src/storage/mod.rs`, `src/workflow/loop.rs`, `src/workflow/mod.rs`, `tests/loop_contract.rs`.

**Estimated scope:** Medium.

### Checkpoint: Workflow foundation

- [ ] Planner and persistence tests pass.
- [ ] Existing workflow tests remain green.
- [ ] No rejected transition consumes state.

## Task 4: Generate the complete fixed artifact package

**Description:** Expand deterministic rendering to every required artifact and render in graph order using one consistent snapshot.

**Acceptance criteria:**

- [ ] The baseline core set and five research documents exist.
- [ ] Index links resolve and later artifacts reuse stable terminology.
- [ ] Offline research is visibly unverified rather than fabricated.

**Verification:** `cargo test --all-features --test generation_contract complete_package`

**Dependencies:** Task 2.

**Files likely touched:** `src/documents/mod.rs`, `src/documents/artifacts.rs`, `tests/generation_contract.rs`, `README.md`.

**Estimated scope:** Medium.

## Task 5: Track hashes and preserve manual overrides

**Description:** Reconcile files against recorded hashes, register changed files as manual revisions, protect them during generation, and stale their dependency closure.

**Acceptance criteria:**

- [ ] Ordinary regeneration cannot overwrite a manual edit.
- [ ] Override revisions survive reopen and removal is explicit.
- [ ] Only graph dependants become stale.

**Verification:** `cargo test --all-features --test generation_contract override`

**Dependencies:** Tasks 1, 2, and 4.

**Files likely touched:** `src/documents/mod.rs`, `src/storage/loop.rs`, `src/workflow/loop.rs`, `tests/generation_contract.rs`.

**Estimated scope:** Medium.

## Task 6: Persist validation and bounded repair outcomes

**Description:** Extend validation to the full graph, hashes, decisions, evidence, and overrides; persist each run and map every finding to repair or escalation.

**Acceptance criteria:**

- [ ] Missing/stale generated files are repaired within the configured bound.
- [ ] Missing truth or authority pauses instead of being invented.
- [ ] Completion requires the latest persisted validation pass.

**Verification:** `cargo test --all-features --test loop_contract validation`

**Dependencies:** Tasks 3-5.

**Files likely touched:** `src/workflow/loop.rs`, `src/storage/loop.rs`, `src/workflow/mod.rs`, `tests/loop_contract.rs`.

**Estimated scope:** Medium.

### Checkpoint: Authoritative package loop

- [ ] Offline end-to-end workflow completes after required answers.
- [ ] Overrides and validation survive restart.
- [ ] Full existing test suite passes.

## Task 7: Add bounded Codex documentation execution

**Description:** Add a package-scoped `workspace-write` Codex adapter that inherits configured tools, protects overrides, and accepts output only after artifact and manifest validation.

**Acceptance criteria:**

- [ ] Arguments never contain dangerous bypass flags or untrusted brief text.
- [ ] Timeout, non-zero exit, missing artifacts, and malformed manifests remain resumable.
- [ ] Deterministic tests launch no paid process.

**Verification:** `cargo test --all-features agents::codex::tests::documentation`

**Dependencies:** Tasks 4-6.

**Files likely touched:** `src/agents/codex.rs`, `src/agents/mod.rs`, `src/workflow/loop.rs`, `tests/agent_contract.rs`.

**Estimated scope:** Medium.

## Task 8: Compose the human and automation CLI

**Description:** Add run, step, status, approval, and override commands while preserving existing command forms and useful terminal behavior.

**Acceptance criteria:**

- [ ] Both create-and-run forms and existing-project resume parse.
- [ ] JSON status and step results have stable tagged shapes.
- [ ] `run` stops only at complete, pause, failure, or bounded repair exhaustion.

**Verification:** `cargo test --all-features --bin project-init`

**Dependencies:** Tasks 3-7.

**Files likely touched:** `src/main.rs`, `src/workflow/loop.rs`, `README.md`, `tests/loop_contract.rs`.

**Estimated scope:** Medium.

## Task 9: Align contracts and operational documentation

**Description:** Update the product spec, architecture, schema, limitations, README, and unreleased changelog to describe only verified behavior.

**Acceptance criteria:**

- [ ] Commands and package names match executable behavior.
- [ ] Earlier conflicting autonomy and repair policy is explicitly updated.
- [ ] Known limitations contain only genuinely deferred scope.

**Verification:** inspect links and run every documented offline command used by the end-to-end test.

**Dependencies:** Tasks 1-8.

**Files likely touched:** `Docs/*.md`, `README.md`, `CHANGELOG.md`.

**Estimated scope:** Medium.

### Checkpoint: Complete

- [ ] `cargo fmt --check`
- [ ] `cargo check --all-features`
- [ ] `cargo test --all-features`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Fresh diff review has no unresolved correctness, compatibility, or comment findings.

## Risks and Mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Filesystem write succeeds before metadata commit | High | Reconcile hashes on every resume and treat unmatched files as provisional or overridden. |
| Codex overwrites a manual edit | High | Snapshot protected bytes, constrain the workspace, verify hashes, and restore before accepting output. |
| Repair loops indefinitely | High | Persist attempt counts and enforce the existing repair limit. |
| Policy changes during resume | High | Store the effective policy and reject conflicting resume options. |
| Agent prose appears researched without evidence | High | Require stored sources or visibly label offline/unverified content. |
| Central-file overlap with existing user changes | High | Add focused modules and patch integration points rather than replacing files. |
| Full graph becomes a generic engine | Medium | Keep artifact kinds and dependency edges closed Rust enums/constants. |

## Open Questions

No question blocks implementation. Runtime Codex or network unavailability is modeled as an explicit resumable outcome.
