# Implementation Plan: Workbench Workflow Resume

## Overview

Add explicit in-workbench workflow resumption as six small vertical slices. The plan first extracts the orchestration contract and proves it with deterministic clients, then makes repair a durable bounded transition, adds pure workbench commands and guided states, and finally connects the background runner and Codex overlay to both interactive entry paths without changing established CLI behavior.

## Architecture Decisions

- Put `WorkflowRunner` in the library so CLI and TUI use one orchestration loop.
- Use an async `DocumentationClient` interface backed by `CodexCliClient` and test fakes; add no dependency.
- Give the background task its own `ProjectService` and SQLite connection while the terminal loop retains its foreground service.
- Require explicit policy selection only when no resumable run exists; never preselect a policy.
- Persist repair authorization and attempt progress using existing workflow-run pause reasons, validation repair passes, and project repair limits.
- Stage generation and repair candidates and use the existing adoption boundary to preserve manual overrides.
- Keep workflow context disposable in the TUI and reload it from SQLite after accepted actions.

## Dependency Graph

```text
DocumentationClient contract ----+
                                 +--> shared WorkflowRunner --> CLI composition
Runner state/error contract ------+             |
                                               v
Repair persistence + transitions ----------> runner repair execution
                                               |
Pure workspace commands + cards ---------------+
                                               |
                                               v
                               background TUI coordinator + overlay
```

## Task 1: Prove the shared runner contract

**Description:** Introduce a provider-neutral documentation client and a library-level runner that preserves existing question, approval, generation, validation, completion, and CLI policy behavior. Begin with runner contract tests and a deterministic fake client.

**Acceptance criteria:**

- [ ] A first run without a policy returns a typed policy-required result without mutation.
- [ ] Existing runs preserve identity and policy while running until the next boundary.
- [ ] Online generation adopts a complete fake package and reaches validation/completion; incomplete staging does not replace accepted files.

**Verification:**

- [ ] New runner tests fail before implementation and pass afterward.
- [ ] Existing `loop_contract` offline tests remain green.
- [ ] `cargo check --all-features` succeeds.

**Dependencies:** None.

**Files likely touched:** `src/agents/mod.rs`, `src/workflow/runner.rs`, `src/workflow/mod.rs`, `tests/runner_contract.rs`.

**Estimated scope:** Medium.

## Task 2: Route Codex generation through the shared boundary

**Description:** Implement the documentation-client interface for `CodexCliClient`, move reusable configuration out of `main.rs`, emit safe lifecycle activity around external work, and replace the command-line private orchestration loop with `WorkflowRunner`.

**Acceptance criteria:**

- [ ] CLI `run` preserves autonomous defaulting, explicit policy conflict behavior, offline operation, and JSON status output.
- [ ] Codex generation keeps `workspace-write`, staging, timeout, cancellation, and expected-path validation behavior.
- [ ] Adapter tests inspect safe arguments and prompts without launching Codex.

**Verification:**

- [ ] `cargo test --all-features agents::codex::tests::documentation` passes.
- [ ] `cargo test --all-features --bin project-init` passes.

**Dependencies:** Task 1.

**Files likely touched:** `src/agents/codex.rs`, `src/agents/mod.rs`, `src/main.rs`, `tests/agent_contract.rs`.

**Estimated scope:** Medium.

### Checkpoint: Shared execution foundation

- [ ] CLI and deterministic tests use the same runner.
- [ ] External output remains provisional until adoption.
- [ ] Targeted tests and `cargo check --all-features` pass.

## Task 3: Make repair an explicit bounded transition

**Description:** Add service and persistence operations for repair authorization, latest validation findings, repair-pass accounting, cancellation/failure pauses, and adoption of a complete repair candidate without overwriting overrides.

**Acceptance criteria:**

- [ ] `/repair` preparation is valid only at a repair boundary and survives database reopen.
- [ ] Each successful repair candidate advances exactly one validation repair pass and the configured limit is enforced at both sides of the boundary.
- [ ] Cancellation, client failure, or incomplete staging consumes no completed repair pass and preserves accepted and manual content.

**Verification:**

- [ ] Targeted workflow and runner tests cover repair limit zero, below-limit, at-limit, reopen, cancellation, and override preservation.
- [ ] Existing generation and persistence contracts remain green.

**Dependencies:** Tasks 1 and 2.

**Files likely touched:** `src/storage/loop.rs`, `src/workflow/mod.rs`, `src/workflow/runner.rs`, `tests/loop_contract.rs`, `tests/runner_contract.rs`.

**Estimated scope:** Medium.

## Task 4: Add pure resume and authority interactions

**Description:** Extend `WorkspaceState` with typed resume, policy selection, approval, rejection, and repair commands plus disposable workflow context and contextual-card rendering. Keep parsing and selector navigation database-free.

**Acceptance criteria:**

- [ ] Exact `/resume`, `/approve`, `/reject`, and `/repair` commands parse without accepting misleading prefixes.
- [ ] First resume opens a selector with no default; Enter cannot submit until a policy is highlighted and Esc cancels without mutation.
- [ ] Question, approval, repair, cancellation/failure, and completion contexts render with discoverable next actions.

**Verification:**

- [ ] Pure workspace and TestBackend tests cover normal, empty, invalid, compact-terminal, and recovery paths.
- [ ] Every new Rust and test item has a meaningful `///` comment.

**Dependencies:** Task 3 for stable workflow context vocabulary.

**Files likely touched:** `src/tui/workspace.rs`, `src/tui/mod.rs`, `tests/workspace_tui_contract.rs`.

**Estimated scope:** Medium.

### Checkpoint: Durable authority UX

- [ ] No command parser or renderer mutates authoritative state.
- [ ] Policy and repair decisions survive reopen only after explicit submission.
- [ ] Targeted TUI, workflow, and runner tests pass.

## Task 5: Run Codex from the responsive workbench overlay

**Description:** Add a workbench runtime configuration and coordinator that opens a separate store in a Tokio task, drives the shared runner, receives bounded activity, acknowledges cancellation, reloads authoritative state, and shows the overlay only during active execution.

**Acceptance criteria:**

- [ ] Interactive `new` and `open` both support `/resume` without exiting the workbench.
- [ ] Only one execution can run at a time; `Esc` cancels and no later worker result adopts a cancelled candidate.
- [ ] Worker completion, pause, and recoverable failure return to refreshed guided workbench context with the composer usable.

**Verification:**

- [ ] Coordinator tests use deterministic channels/fakes and no real terminal or paid process.
- [ ] Existing creation cancellation and terminal restoration tests remain green.
- [ ] A manual local smoke test can reach a policy selector and cancel before external execution.

**Dependencies:** Tasks 1 through 4.

**Files likely touched:** `src/tui/mod.rs`, `src/tui/workspace.rs`, `src/main.rs`, `tests/workspace_tui_contract.rs`, `tests/creation_tui_contract.rs`.

**Estimated scope:** Medium.

## Task 6: Align user-facing contracts and finish verification

**Description:** Update only verified README, architecture, feature specification, known-limitations, and changelog statements, then run the complete repository quality suite and inspect the final diff against the saved specification.

**Acceptance criteria:**

- [ ] Documentation describes required policy selection, `/resume`, authority commands, cancellation, repair limits, and output location accurately.
- [ ] The changelog follows `Docs/CHANGELOG_GUIDELINES.md` and does not overclaim unverified behavior.
- [ ] All requested and existing quality commands pass with no unresolved review finding.

**Verification:**

- [ ] `cargo fmt --check`
- [ ] `cargo check --all-features`
- [ ] `cargo test --all-features`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Fresh `git diff` review confirms unrelated user changes were preserved.

**Dependencies:** Tasks 1 through 5.

**Files likely touched:** `README.md`, `Docs/architecture.md`, `Docs/known-limitations.md`, `CHANGELOG.md`.

**Estimated scope:** Medium.

### Checkpoint: Complete

- [ ] The full flow is test-proven from policy selection to complete or explicit pause.
- [ ] Failed, cancelled, and rejected paths preserve authoritative and filesystem invariants.
- [ ] CLI compatibility and all Rust commenting requirements are satisfied.

## Risks and Mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Blocking terminal loop starves async work | High | Use the existing multi-thread Tokio runtime, short event polling, and channel polling rather than blocking `event::read` during execution. |
| Cancellation races with successful generation | High | Check cancellation before adoption and keep all candidate bytes in temporary staging until that check. |
| Repair becomes an unbounded agent loop | High | Require `/repair` before every attempt and enforce persisted `repair_limit`/`repair_pass` values. |
| Separate SQLite connections conflict | Medium | Keep foreground writes disabled while the worker runs, use short transactions, and reload after worker completion. |
| TUI and CLI semantics drift | High | Share `WorkflowRunner`; leave policy defaulting in CLI composition only. |
| Manual override is changed in staging | High | Let the client see a copy but use the adoption boundary to retain accepted override bytes and hashes. |
| Central-file overlap loses user work | High | Patch narrow integration points, inspect the dirty diff before and after every checkpoint, and avoid wholesale rewrites. |

## Open Questions

No question blocks implementation. The invoked automated Rust workflow supplies approval to continue through the documented checkpoints without additional phase gates.
