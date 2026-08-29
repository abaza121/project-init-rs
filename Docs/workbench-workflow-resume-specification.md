# Specification: Workbench Workflow Resume

## Objective

Let a developer or product creator continue a project from clarification through online Codex generation, approval decisions, bounded repair, validation, and completion without leaving the contextual workbench. The same behavior must be available after interactive `new` and `open` flows, and the command-line `run` command must share the orchestration implementation.

Success means `/resume` is the only execution trigger, the first run cannot start until the user explicitly selects an approval policy, long-running Codex work is visible and cancellable, every pause remains recoverable inside the workbench, and no cancelled or failed candidate package replaces accepted files.

## Acceptance Criteria

1. `/resume` emits a typed workbench action and never starts execution during pure input parsing.
2. When no active run exists, `/resume` opens a required selector for `strict`, `consequential`, or `autonomous`; no option is initially selected.
3. When an active or paused run exists, `/resume` preserves its persisted policy and runs until a question, approval, repair, completion, cancellation, or operational failure boundary.
4. CLI `run` and workbench `/resume` use the same library-level workflow runner while preserving existing CLI defaults and output.
5. Online generation runs in a background task and displays bounded, sanitized Codex JSONL output in a focused progress overlay. `Esc` requests cancellation and the overlay remains until the worker acknowledges it.
6. Generation and repair use temporary staging. Missing expected files, cancellation, timeout, or client failure leaves the accepted package and registered overrides unchanged.
7. Approval and rejection commands resolve only the currently pending project decision, require a non-blank reason, reload authoritative state, and do not auto-resume.
8. `/repair` is accepted only for a persisted repair boundary, respects the project's repair limit, and records explicit authorization before `/resume` can execute one Codex repair attempt.
9. A repair candidate starts from the current package, receives the current project snapshot and latest persisted validation findings, protects manual overrides during adoption, and is revalidated before completion.
10. Questions, approvals, repair findings, cancellation, failures, and completion render as recoverable contextual workbench feedback after database reopen.
11. Existing answer, ask, threshold, quit, non-interactive open, initial-analysis, offline runner, and generated-package behavior remain compatible.
12. Every new Rust function, struct, enum, trait, and test item has a behavior-oriented documentation comment.

## Technology and Dependencies

- Stable Rust 1.88 with the repository's Rust 2024 edition.
- Tokio channels, tasks, and cancellation primitives already present in the crate.
- Ratatui and Crossterm for the blocking terminal event loop and progress overlay.
- Rusqlite for authoritative run, repair, validation, and document state.
- The existing `async-trait`, `tempfile`, `serde`, and `thiserror` dependencies.

No new crate or database migration is required. Existing `projects.repair_limit`, `validation_runs.repair_pass`, workflow-run pause reasons, and document revision tables provide the required durable vocabulary.

## Commands

```text
Build:   cargo build --all-features
Format:  cargo fmt --check
Check:   cargo check --all-features
Test:    cargo test --all-features
Lint:    cargo clippy --all-targets --all-features -- -D warnings
Create:  cargo run -- --data-dir .project-init new --brief <file>
Open:    cargo run -- --data-dir .project-init open <project-id>
Resume:  enter /resume in the workbench composer
Approve: enter /approve <decision-id> <reason> in the workbench composer
Reject:  enter /reject <decision-id> <reason> in the workbench composer
Repair:  enter /repair in the workbench composer, then /resume
```

## Project Structure

```text
src/agents/       Provider-neutral documentation client and Codex adapter
src/domain/       Closed workflow, policy, and lifecycle value types
src/storage/      Durable run, validation, repair, and revision operations
src/workflow/     ProjectService plus shared asynchronous WorkflowRunner
src/tui/          Pure workspace commands/state and terminal orchestration
src/main.rs       CLI composition and runtime configuration
tests/            Runner, persistence, workflow, and rendering contracts
Docs/ideas/       Confirmed concept and task breakdown
Docs/             Feature specifications and operational documentation
```

## Smallest Viable Architecture

```text
CLI run ------------------------------+
                                      |
Workbench /resume -> background task -+-> WorkflowRunner
                                              |
                           +------------------+-------------------+
                           |                  |                   |
                    ProjectService    DocumentationClient   Activity channel
                           |                  |                   |
                       SqliteStore       Codex CLI/staging   Progress overlay
                           |
                 snapshot/status reload
                           |
            guided question/approval/repair card
```

For a C# programmer, `DocumentationClient` is an asynchronous interface and `WorkflowRunner` is an application service shared by two presentation layers. `WorkflowStep` and workbench commands are closed discriminated unions similar to sealed record hierarchies; Rust requires every case to be handled. The TUI borrows its current state for rendering, while the background task owns a separate service and SQLite connection so no mutable service reference crosses the terminal loop.

## Core Types and Boundaries

- `DocumentationClient`: provider-neutral async generation and repair interface implemented by `CodexCliClient` and deterministic test fakes.
- `WorkflowRunner`: owns one `ProjectService`, output path, execution mode, cancellation token, and optional activity sender for one bounded run.
- `WorkflowRunOutcome`: distinguishes a user-input boundary, completion, and acknowledged cancellation without treating cancellation as package success.
- `WorkspaceCommand`: adds resume, policy selection, decision authority, and repair authorization to existing pure commands.
- `PolicySelection`: transient selector with no default choice; it is never authoritative until submitted.
- `WorkspaceWorkflowState`: disposable workflow status, validation findings, overlay progress, and guided-card context.
- Persisted workflow run, validation run, validation findings, document records, and revisions remain authoritative.

## State Model and Invariants

### Run transitions

- No run plus `/resume` opens policy selection and does not mutate SQLite.
- Submitting a policy starts exactly one run and then explicitly executes it.
- Existing active or paused runs retain their policy; a competing policy is rejected without mutation.
- Question and approval boundaries stop the runner before external work.
- Repair boundaries stop unless the active run has a persisted `repair_requested` authorization.
- Cancellation changes the run to paused with a safe reason and does not adopt staging.
- Completion requires current document hashes and the latest persisted passing validation.

### Repair transitions

- `/repair` is valid only while the current step is `RepairRequired`.
- Authorization consumes no repair pass by itself.
- A successful staged repair marks the run as awaiting repair validation and advances the next validation pass number.
- Failed or cancelled Codex work does not consume a completed repair pass and requires another explicit `/repair`.
- A failed validation persists its repair pass. Another repair is rejected when that pass reaches `projects.repair_limit`.
- Old validation findings remain auditable; successfully adopted repair findings are marked repaired before the next validation creates any replacement findings.

### Ownership and atomicity

- SQLite is the only authority for project, run, decision, validation, and revision state.
- The accepted package is not modified until a complete staging package passes structural checks.
- Manual overrides are never copied from staging over accepted bytes.
- Rejected commands, invalid IDs, missing policies, and exhausted repair attempts change no authoritative state.
- The TUI reloads snapshot and workflow context after every accepted mutation or worker result.
- At most one background workbench execution exists for one open project.

## Error Handling

- Domain, storage, workflow, agent, and runner failures remain typed below `main.rs`; the CLI may add `anyhow` context.
- A missing Codex executable, spawn failure, inactivity timeout, non-zero exit, incomplete package, or repair failure pauses the run with sanitized feedback. The inactivity deadline resets whenever Codex stdout or stderr advances.
- Terminal input and drawing failures unwind through Ratatui's restoration boundary.
- The event loop remains responsive while waiting for activity or worker completion and ignores duplicate `/resume` while an execution is active.
- Production code does not use `unwrap`, `expect`, or deliberate panics.

## Explicit Behavior and Policy Decisions

- The policy selector has no default. Arrow keys change the highlighted choice; Enter confirms; Esc closes without starting a run.
- `/resume` executes continuously until the next explicit boundary instead of one step at a time.
- Approving, rejecting, answering, or authorizing repair never auto-resumes.
- Workbench execution is online-only in this release; existing CLI offline flags remain supported.
- The full-screen overlay is shown only while the shared runner performs external Codex generation or repair. Fast validation and status transitions return through normal workbench feedback.
- Repair uses Codex only for structurally repairable package findings. Missing authority, unresolved consequential questions, and unsupported decisions remain user boundaries.

## Testing Strategy

- Pure TUI tests cover exact command parsing, policy selection with no default, guided-card rendering, duplicate resume rejection, and recoverable invalid input.
- Runner contract tests use a fake documentation client to cover generation, question/approval boundaries, cancellation, incomplete staging, repair authorization, repair limits, override protection, and completion.
- Persistence/workflow tests cover repair pass accounting, pause reasons, reopen, and rejection without mutation.
- Codex adapter tests inspect generation and repair prompts and arguments without launching a paid process.
- Existing offline loop, creation TUI, workspace, generation, persistence, and CLI tests remain regression coverage.
- The complete format, check, test, and Clippy suite is required.

## Code Style

Every Rust item receives a meaningful documentation comment, including private helpers and test fixtures:

```rust
/// Runs deterministic and provider-backed steps until the next persisted authority boundary.
pub async fn run_until_pause(&mut self, project_id: &ProjectId) -> Result<WorkflowRunOutcome, RunnerError> {
    // implementation
}
```

Inline comments explain only non-obvious staging, cancellation, transaction, or state-transition invariants. Ownership stays simple: the background runner owns its service and candidate workspace; render functions borrow disposable state.

## Boundaries

- **Always:** validate before mutation, require explicit policy and execution authority, stage external output, protect overrides, preserve compatibility, add failing behavioral tests first, and inspect the dirty diff.
- **Ask first:** incompatible released-schema changes, new dependencies, destructive data repair, paid live model execution during tests, publication, deployment, or changes outside the requested project.
- **Never:** default the first workbench policy, auto-resume after authority input, bypass Codex sandbox controls, expose raw prompts or reasoning in the TUI, overwrite manual content, hide failed validation, or weaken existing tests.

## Evaluator Risks

- Blocking terminal input starving Tokio worker progress.
- A cancellation racing with successful process exit and adopting output after the user cancels.
- Reopen losing repair authorization or selecting a different policy.
- Repeated `/resume` spawning concurrent Codex processes.
- Repair-limit off-by-one behavior around zero and the configured maximum.
- A repair candidate omitting an artifact or modifying an override.
- Approval commands targeting a decision from another project.
- A completed run followed by authoritative changes requiring a new explicit policy selection.

## Deliverables

- `Docs/ideas/workbench-workflow-resume.md`
- `Docs/ideas/workbench-workflow-resume-implementation-plan.md`
- This feature specification.
- Shared runner and documentation-client boundaries.
- Persisted bounded repair operations.
- Workbench commands, policy selector, guided cards, background execution, and overlay.
- Behavioral tests and updated operational documentation.

## Open Questions

No product decision blocks implementation. Live Codex availability is a runtime capability; deterministic tests use a fake client and never invoke a paid process.
