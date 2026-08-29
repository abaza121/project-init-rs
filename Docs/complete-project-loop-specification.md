# Specification: Complete Project Loop

## Objective

Extend `project-init` so a developer, product creator, or automation can start with an incomplete brief, answer consequential questions, and drive one resumable workflow to a complete, validated project-documentation package. SQLite remains authoritative, manual document edits survive later runs, and Codex may automate research, decisions, generation, and repair only through the caller's configured sandbox and approval boundaries.

## Acceptance Criteria

1. `new --run`, `run --brief`, and `run <project-id>` can enter the same resumable workflow without removing existing commands.
2. Each run records its selected `strict`, `consequential`, or `autonomous` approval policy. Resumption retains that policy.
3. `step --json` executes at most one planned mutation and `status --json` reports the same deterministic next action without mutation.
4. Open consequential questions and policy-required decision approvals pause with stable machine-readable reasons.
5. A fixed artifact graph generates the full baseline package in dependency order and marks dependants stale when an input or manual override changes.
6. Generation never silently overwrites a file whose content differs from its recorded generated hash. The edit is registered as a manual override and downstream artifacts become stale.
7. Document metadata, evidence, decisions, workflow runs, validation runs, validation findings, and revisions survive database reopen.
8. Codex-backed package work uses `codex exec --sandbox workspace-write` in the package workspace, inherits configured tools, and never uses approval or sandbox bypass flags.
9. Offline execution remains deterministic, labels unavailable research honestly, and requires no network or paid API.
10. Every validation code has a deterministic repair, bounded Codex repair, or explicit escalation. Failed repair cannot manufacture completion.
11. `Complete` requires no consequential open question, no required approval, every required artifact current or validly overridden, matching hashes, and a persisted passing validation run.
12. Existing tests and CLI behavior remain compatible.

## Commands

```text
Build:      cargo build --all-features
Format:     cargo fmt --check
Check:      cargo check --all-features
Test:       cargo test --all-features
Lint:       cargo clippy --all-targets --all-features -- -D warnings
Create:     project-init new --brief <file> [--name <name>] [--offline] [--run] [--approval <policy>]
Start:      project-init run --brief <file> [--name <name>] [--offline] [--approval <policy>]
Resume:     project-init run <project-id> [--offline] [--approval <policy>]
One step:   project-init step <project-id> [--offline] --json
Status:     project-init status <project-id> --json
Approve:    project-init approve <decision-id>
Reject:     project-init reject <decision-id> --reason <text>
Override:   project-init override <project-id> <relative-path>
Unmanage:   project-init remove-override <project-id> <relative-path>
```

## Smallest Viable Architecture

```text
CLI/TUI
  -> WorkflowRunner
       -> WorkflowPlanner (pure next-step decision)
       -> ProjectService (validated use cases)
       -> ArtifactRegistry (fixed dependency graph)
       -> PackageRenderer / CodexDocumentationClient
       -> Validator + RepairCatalog
  -> SqliteStore (only authoritative mutation boundary)
```

For a C# programmer, `WorkflowStep` is a closed discriminated union similar to a sealed hierarchy of records. `WorkflowPlanner` is a side-effect-free service: it receives a snapshot and returns one union case. The Rust `Result<T, E>` on the runner makes every recoverable persistence, filesystem, agent, and validation failure explicit instead of relying on exceptions. Borrowed snapshots are temporary read access; persisted records own their data.

## Core Types and Boundaries

- `ApprovalPolicy`: `Strict`, `Consequential`, or `Autonomous`.
- `WorkflowRun`: run identity, project, effective policy, status, pause reason, and timestamps.
- `WorkflowStep`: question, approval, generation, validation, repair, or completion.
- `ArtifactKind` and `ArtifactDefinition`: fixed paths and dependency relationships.
- `DocumentRecord` and `DocumentRevision`: current metadata and immutable generated/manual revision history.
- `Evidence` and `Decision`: first-class stored research and explicit choices.
- `ValidationRun` and `ValidationFindingRecord`: persisted verification history and repair state.
- `CodexDocumentationClient`: optional external executor restricted to the package workspace.

The planner never mutates. The runner executes one planned step through `ProjectService`; the service coordinates complete transactions. The filesystem is provisional until content hashes and metadata commit successfully.

## State Model

- Identity is project-scoped for display IDs and global for internal UUIDs.
- SQLite rows are authoritative. Artifact files are checked against stored hashes before every generation or validation step.
- At most one active or paused workflow run exists per project.
- Starting a run selects a policy; resuming an active run preserves it. A different policy requires an explicit new run after the current run reaches a terminal state.
- A rejected operation consumes no display ID, run counter, or revision and leaves files unchanged where practical.
- A generated artifact becomes `stale` when one of its declared inputs changes.
- A manual override is current only after registration and validation. It may make dependants stale but is never overwritten by ordinary generation.
- A passed validation run is invalidated by later authoritative or artifact changes.

## Artifact Graph

The fixed graph contains `README.md`, `Requirements.md`, `Assumptions.md`, `OpenQuestions.md`, `SWOT.md`, `MissionVision.md`, `VisualIdentity.md`, `BrandPrompt.md`, `TechnicalArchitecture.md`, five numbered research documents, `Traceability.md`, a timestamped session log, and `ValidationReport.md`.

Requirements and stored knowledge are the root inputs. Strategic, identity, architecture, and research artifacts depend on requirements. Traceability and the session log depend on the complete package. Validation depends on every required artifact.

## Codex Execution Policy

The installed Codex CLI currently exposes `read-only`, `workspace-write`, and `danger-full-access` sandbox modes. Documentation generation uses `workspace-write`, an explicit package working directory, ephemeral execution, bounded time, and no shell interpolation. It does not pass `--dangerously-bypass-approvals-and-sandbox` or `--dangerously-bypass-hook-trust`.

Codex output is untrusted until expected paths, bounds, manifest structure, hashes, evidence sources, and validation rules pass. A failed, timed-out, or malformed run leaves authoritative state incomplete and resumable.

## Error Handling and Repair

- Domain and storage errors remain typed with `thiserror`; the CLI adds context with `anyhow`.
- Missing artifacts and stale generated artifacts are deterministically regenerated.
- Structurally repairable document defects may receive bounded Codex-assisted repair.
- Missing evidence, contradictory authoritative answers, and consequential intent escalate to the user.
- Repair attempts are capped by the project's configured limit and recorded in validation history.
- A failed repair remains visible and cannot be marked complete.

## Testing Strategy

- Pure planner tests cover every state transition, approval policy, and precedence rule.
- Artifact graph tests cover paths, dependency closure, invalidation, and traversal order.
- SQLite integration tests cover migration, restart, active-run uniqueness, idempotency, revision history, and post-error recovery.
- Filesystem tests cover generation, hash matching, override preservation, removal, and dependent invalidation.
- CLI parse tests cover all new entry forms and JSON-compatible commands.
- End-to-end tests use offline execution and temporary directories; no paid agent call is required.
- Codex adapter tests inspect arguments and prompts without launching the process.

## Explicit Behavior

- Questions take precedence over approvals, generation, validation, and completion.
- Required approvals take precedence over generation.
- Status is observational and never starts a run or mutates the project.
- Step performs no more than one authoritative transition.
- Run loops over steps until complete, paused, failed, or its bounded repair budget is exhausted.
- Manual overrides take precedence over generated content.
- Existing `generate` remains an explicit deterministic rendering command.

## Policy Decisions

- Consequential policy pauses for decisions marked `needs_confirmation`; strict policy pauses for every proposed or confirmation-needed decision; autonomous policy may accept only supported non-consequential proposals.
- Full configured Codex tools are available only within the sandbox chosen by the application and caller environment.
- The fixed graph is compiled into Rust rather than stored as arbitrary executable configuration.
- Both `new --run` and `run --brief` are supported even though they overlap, because one preserves explicit creation semantics and the other optimizes the single-command experience.

## Implementation Invariants

- A failed or duplicate step does not partially mutate run, project, document, validation, or sequence state.
- Resuming the same run returns the same next step until relevant authoritative state changes.
- Accepted decisions have provenance or remain confirmation-required.
- Research-derived claims retain direct sources and retrieval timestamps.
- File paths are fixed registry values or validated project-relative paths without traversal.
- Manual content is never silently lost.
- Completion always corresponds to the latest authoritative revision and validation run.

## Evaluator Risks

- Crash timing between filesystem writes and metadata commit can produce provisional files; reconciliation must detect rather than trust them.
- A manually edited artifact may coincide with a stale generated hash; revision history must disambiguate ownership.
- Approval policy changes during resume could create non-deterministic behavior.
- A Codex run may succeed without producing every required file or may alter an override.
- Repeated repair could loop indefinitely or repeatedly scan the whole project.
- Research can be unavailable, stale, or unsupported even when document prose appears complete.
- Existing dirty worktree changes overlap central modules and must be preserved.

## Non-Goals

- Arbitrary user-defined workflow graphs or document kinds.
- Parallel workflow-step execution.
- Hosted collaboration, authentication, or concurrent document editing.
- Semantic/vector retrieval.
- Automatic publication, deployment, or external side effects beyond configured Codex tools.
- Bypassing Codex, operating-system, repository, or application approval controls.

## Project Boundaries

- Always: preserve existing commands, validate before mutation, document every Rust item, run the full quality suite, and record notable changes in `CHANGELOG.md`.
- Ask first: incompatible released-schema changes, new paid provider integrations, destructive data repair, publication, or deployment.
- Never: fabricate evidence, silently overwrite manual content, bypass Codex safety controls, hide failed validation, weaken tests, or mutate unrelated user changes.

## Success Criteria

The fishing fixture can be created and driven offline through `run` to either a stable question/approval pause or a complete full package. After answers are supplied, resumption generates all required artifacts, records hashes and validation history, survives database reopen, preserves a manually edited artifact, refreshes its dependants, and reports `complete` consistently through human-readable and JSON status.

## Open Questions Resolved for Implementation

No question blocks implementation. Codex availability and research access are runtime capabilities rather than assumptions: unavailable capabilities produce explicit pauses or unverified offline research instead of fabricated success.
