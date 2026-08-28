# Implementation Plan

## Overview

Implementation follows vertical, test-first slices. Each slice leaves a runnable or verifiably stronger system, updates the unreleased changelog only for notable behavior, and ends in an atomic commit after the quality commands pass.

## Task 1: Preserve and specify the baseline transition

**Acceptance criteria**

- The exact upstream baseline and its two commits are retained under `baseline/`.
- Baseline behavior, limits, and independent command are documented.
- Product, architecture, schema, and verification contracts are explicit.

**Verification:** inspect Git ancestry, validate Markdown links/paths, and review staged content.

**Files:** `baseline/`, `Docs/*.md`, `CHANGELOG.md`.

## Task 2: Bootstrap the typed core and migrations

**Acceptance criteria**

- Cargo builds a library and `project-init` binary on stable Rust.
- Typed IDs, enums, project/finding/requirement/question records, and lifecycle rules exist.
- SQLite migrations create every authoritative table with foreign keys and indexes.
- An on-disk project survives close/reopen.

**Verification:** red domain/migration/restart tests, then the full milestone quality suite.

**Likely files:** `Cargo.toml`, `src/domain/*`, `src/storage/*`, `migrations/001_initial.sql`, `tests/persistence.rs`.

## Task 3: Ingest and clarify a brief

**Acceptance criteria**

- Structured analysis responses are schema-validated before persistence.
- The deterministic analyzer creates explicit findings and conservative unknowns.
- Priority boundaries order and filter questions predictably.

**Verification:** fixture tests for normal, empty, contradictory, and malformed inputs.

**Likely files:** `src/agents/*`, `src/workflow/analyze.rs`, `src/workflow/clarify.rs`, `fixtures/*`.

## Task 4: Record answers and reconcile history

**Acceptance criteria**

- Answers are first-class immutable records.
- Reconciliation can confirm/reject assumptions, resolve unknowns, create requirements, and invalidate decisions with trace links.
- A failure leaves all authoritative and sequence state unchanged.

**Verification:** permutation and post-error recovery integration tests, including restart.

**Likely files:** `src/domain/answer.rs`, `src/workflow/reconcile.rs`, `src/storage/*`, `tests/reconciliation.rs`.

## Task 5: Add evidence, decisions, traceability, generation, and validation

**Acceptance criteria**

- Evidence is distinct from conclusions; accepted decisions have provenance.
- Documents render from snapshots and include stable IDs.
- Validation persists structured failures, detects contradictions and unsupported decisions, and bounds repair attempts.

**Verification:** fishing fixture generation plus deliberate missing-trace, contradiction, and artifact-corruption cases.

**Likely files:** `src/documents/*`, `src/workflow/decide.rs`, `src/workflow/generate.rs`, `src/workflow/verify.rs`, `tests/generation.rs`.

## Task 6: Deliver the CLI and TUI

**Acceptance criteria**

- All specified commands parse and dispatch with useful errors.
- Overview, clarification, and finding panels render with intentional controls.
- Quit preserves state and terminal cleanup is reliable.

**Verification:** Clap parse tests, Ratatui test-backend snapshots/assertions, and a manual terminal smoke test.

**Likely files:** `src/main.rs`, `src/cli.rs`, `src/tui/*`, `tests/cli.rs`.

## Task 7: Add derived semantic retrieval

**Acceptance criteria**

- Provider-neutral embedding/index traits isolate all LanceDB/Arrow types.
- Content hashes skip unchanged records and failed indexing marks SQLite state stale.
- Search enforces project/status/type filters, reloads SQLite entities, limits context, and records telemetry.
- Relational mode and LanceDB-unavailable behavior remain fully operational.

**Verification:** the 18 semantic retrieval families in `init.md`, primarily with deterministic embeddings, plus feature-gated local LanceDB creation/rebuild.

**Likely files:** `src/retrieval/*`, `tests/retrieval.rs`, `Cargo.toml`.

## Task 8: Complete evaluation and contributor documentation

**Acceptance criteria**

- The fishing fixture, example run, generated package, trajectory example, and known limitations are present.
- One command runs the same case in baseline, relational, and semantic modes while recording—not inventing—metrics.
- README documents setup, use, tests, and data locations.

**Verification:** execute offline example, validate links/artifacts, and dry-run evaluation help without invoking a paid model.

**Likely files:** `README.md`, `examples/*`, `evaluation/*`, `Docs/known-limitations.md`, `Docs/agent-trajectories.md`.

## Checkpoints

After each implementation task:

```text
cargo fmt --check
cargo check --all-features
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Inspect the staged diff, search it for credential-shaped content, update the changelog when behavior is notable, then commit one logical save point.

## Risks and mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| LanceDB/Arrow dependency churn | High | Optional feature, adapter isolation, locked graph, official API verification |
| Broad product scope | High | Vertical slices, deterministic core first, bounded features and commits |
| Silent inference | High | Provenance assigned by application, conservative analyzer, confirmation status |
| Partial reconciliation | High | One SQLite transaction and recovery tests |
| Cross-project retrieval leakage | High | Mandatory project filters plus authoritative reload and tests |
| TUI hard to automate | Medium | Pure state transitions and Ratatui test backend |
| Evaluation overclaim | Medium | Raw metrics and empty report templates; no fabricated results |

## Specification checkpoint

Artifacts: baseline import, baseline assessment, product specification, architecture, schema, and ordered task plan.

Decisions: single package with library/binary targets; SQLite authority; optional LanceDB; deterministic offline default; bounded repair; baseline preserved as history.

Evidence: inspected both upstream commits and every baseline file; verified current Ratatui, Clap, Rusqlite, and LanceDB APIs against official docs.

Unresolved tradeoffs: exact dependency resolution and any LanceDB feature compatibility will be fixed by `Cargo.lock` and compile evidence. No material product choice blocks the test phase.

Next phase: create failing domain and persistence tests. It may add `Cargo.toml`, `src/`, `tests/`, and migrations, but will not yet implement workflow behavior.
