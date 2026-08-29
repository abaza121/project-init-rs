# Specification: Project Init

## Objective

Build a stable-Rust terminal application named `project-init` that transforms an incomplete project brief into a local, resumable, evidence-aware project knowledge base and generates project-initiation documents only from that authoritative state.

The primary user is a developer or product creator who has an idea but not yet an implementation-ready foundation. Success means the user can inspect why an important generated statement exists, correct an assumption before it becomes architecture, resume after exit, and deterministically validate the resulting package.

## Acceptance criteria

1. `project-init new --brief <file>` validates a non-empty brief, runs schema-constrained Codex analysis by default, shows bounded live activity in the TUI, atomically persists successful results, prioritizes consequential questions, and opens the durable overview when attached to a terminal. `--offline` explicitly selects deterministic analysis.
2. `project-init open <project-id>` resumes a keyboard-first contextual workbench without losing history. It can answer open questions, capture structured user questions, adjust the consequential threshold, show affected-entity diffs, explicitly resume online generation, handle decision authority and bounded repair, and advance through the complete workflow without requiring a CLI round trip.
3. `list`, `inspect`, `generate`, `validate`, `export`, `inspect-related`, and retrieval-management commands expose the stored model without requiring a live agent.
4. SQLite is the authoritative source for projects, findings, requirements, questions, answers, evidence, decisions, trace links, documents, validation state, and semantic-index state.
5. Every important statement records provenance; agent inference is never silently promoted to a user requirement.
6. Question ordering is deterministic from impact, uncertainty, and cost of being wrong, with a configurable threshold.
7. Recording an answer and reconciling it is atomic, history-preserving, and can resolve or supersede earlier knowledge without destructive overwrite.
8. Consequential unresolved contradictions transition a project to `NeedsUserInput`.
9. Generated documents are rendered from stored records and include stable IDs where traceability matters.
10. Validation detects missing artifacts, unresolved high-impact uncertainty, missing requirement acceptance criteria, unsupported decisions, broken traces, inconsistent naming, and explicit contradictions. Automatic repair is bounded.
11. Relational mode works with no semantic service. Semantic mode uses LanceDB only as a derived candidate index, resolves every match back through SQLite, supports rebuild/stale detection, and degrades without corrupting authoritative state.
12. Unit and integration tests use deterministic clients and embeddings and require no paid API.
13. The fishing VR fixture demonstrates the end-to-end flow and both evaluation modes without fabricated results.
14. The preserved baseline remains runnable independently.
15. `new --run`, `run`, `step`, and `status` drive one persisted orchestration loop with a run-scoped approval policy, full fixed artifact graph, protected manual overrides, and machine-readable pause reasons.
16. Evidence and decisions have explicit capture and authority commands; validation runs, document hashes, and immutable revisions are persisted rather than inferred from files.
17. First workbench resume requires explicit approval-policy selection with no default. Codex generation and repair remain cancellable and provisional until a complete staging package is adopted.
18. The global `--no-skills` option disables every discovered Codex skill for initial analysis, CLI-driven generation, and workbench resume without persisting configuration changes; incomplete discovery fails before provider work begins.
19. `run --auto-answer`, `new --run --auto-answer`, and workbench `/auto-answer` use a fixed three-worker pool to research different dependency-safe consequential questions, require a separate schema-constrained Codex judge, atomically persist still-applicable cited recommendations as imported research rather than user authority, and continue until another workflow boundary or completion. Attached terminals show actor-keyed TUI progress while redirected automation remains headless.

## Explicit behavior

- Users are authoritative over consequential product decisions.
- Users may explicitly delegate clarification recommendations with `--auto-answer` or `/auto-answer`; delegated answers remain distinguishable from user-authored answers and retain their supporting evidence.
- Findings distinguish confirmed facts, requirements, assumptions, unknowns, constraints, risks, and research questions.
- Workflow state is explicit: `Draft`, `Analyzing`, `AwaitingClarification`, `Researching`, `Planning`, `Generating`, `Validating`, `NeedsUserInput`, or `Complete`.
- Requirement, decision, evidence, answer, and validation identifiers are stable within a project.
- Invalid structured agent output is rejected before any project-state mutation.
- Research evidence and conclusions are stored separately.
- A semantic match is a candidate, never an authoritative relationship or contradiction verdict.

## Policy decisions

- Codex CLI is the default initial-brief analyzer. It runs ephemerally with a read-only sandbox, structured output, bounded activity, immediate cancellation, and a five-minute timeout. The conservative deterministic analyzer remains available through `--offline`; failures never trigger it automatically.
- Low-priority questions remain visible but do not interrupt the user until they meet the configured threshold.
- `new --brief` launches live analysis and the durable workbench only when standard input/output are terminals; non-interactive use completes the same Codex and persistence flow without terminal rendering.
- Workbench plain text answers only the selected open question in the Questions section. `/answer` targets explicitly, `/ask` opens structured capture with visible conservative defaults, and `/threshold` persists a value from 1 through 125.
- `/resume` is the only workbench execution trigger. A first run requires explicit policy selection; answers, approvals, rejections, and `/repair` authorization never continue automatically.
- Relational retrieval is the default because it has fewer operational dependencies. Semantic mode is explicit per project.
- Automatic repair is bounded. Deterministic regeneration handles missing or stale generated artifacts, Codex may assist with document repair inside the configured sandbox, and missing evidence or high-impact user authority always pauses explicitly.
- Codex-assisted documentation runs with `workspace-write` in an isolated staging package and inherits configured tools without dangerous approval or sandbox bypass flags.
- Automatic clarification research is opt-in, incompatible with `--offline`, read-only, citation-required, and fail-closed. It does not automatically approve decisions or authorize repairs.
- Skill suppression is explicit and invocation-scoped. It disables exact discovered `SKILL.md` paths while preserving `AGENTS.md`, tools, sandboxing, approval behavior, and the default skill-enabled path.
- The initial release uses one Cargo package with a library target and binary target. This keeps reusable boundaries without premature multi-crate coordination; a workspace split remains possible if compile ownership or release boundaries emerge.
- LanceDB is an optional Cargo feature. This preserves a useful offline binary and makes unavailability behavior testable, while keeping the adapter production-real when enabled.

## Non-goals

- Automatically proving that the improved workflow outperforms the baseline.
- Fabricating current research, citations, or evaluation results.
- Providing a hosted service, multi-user synchronization, authentication, or cloud storage.
- Replacing explicit trace links with vector similarity.
- Sending the entire project database to every agent call.
- Building an unrestricted agent loop that bypasses configured sandbox, approval, provenance, or validation boundaries.
- Supporting arbitrary legacy database schemas before a first stable release exists.

## Commands

```text
Build:     cargo build --all-features
Format:    cargo fmt --check
Check:     cargo check --all-features
Test:      cargo test --all-features
Lint:      cargo clippy --all-targets --all-features -- -D warnings
Run:       cargo run -- new --brief baseline/project-prompt.txt
Baseline:  baseline\run-codex-docs-pipeline.bat -f baseline\project-prompt.txt
```

## Project structure

```text
src/                  Rust library and CLI binary
src/domain/           Typed knowledge model and lifecycle rules
src/storage/          SQLite migrations and authoritative repositories
src/agents/           Provider-neutral client trait and structured responses
src/workflow/         Analysis, clarification, reconciliation, generation, validation
src/retrieval/        Provider-neutral semantic types, sync, context, LanceDB adapter
src/tui/              Ratatui state, views, and event loop
src/documents/        Database-backed renderers and templates
tests/                Cross-boundary integration tests
migrations/           Ordered, embedded SQLite schema
fixtures/             Deterministic agent and embedding inputs
examples/             Demonstration commands and expected package
evaluation/           Cases, runners, raw outputs, and non-fabricated reports
Docs/                 Product, architecture, schema, baseline, and contributor docs
baseline/             Preserved upstream hackathon implementation
```

## Smallest viable architecture and data flow

```text
CLI/TUI -> workflow services -> domain types -> SQLite repositories
                  |                    |
                  |                    +-> document renderer -> validator
                  +-> agent client (structured JSON)
                  +-> semantic facade -> LanceDB candidate IDs -> SQLite reload
```

SQLite owns accepted state. Workflow services validate complete transitions before opening a transaction. Document rendering reads a consistent project snapshot. Semantic synchronization happens after the SQLite commit; failure marks the derived index stale and returns a warning without rolling back accepted project knowledge.

For a C# programmer: the package is roughly one .NET project exposing both a class-library surface and a console executable. Rust modules resemble namespaces with enforced privacy. Data-bearing enums are closed discriminated unions, not numeric enums. Repository and client traits resemble interfaces, while `Result<T, E>` makes recoverable failures part of every relevant method signature instead of relying on exceptions. Borrowed `&T` values provide temporary access without transferring ownership; stored records own their strings so they can outlive a database row callback.

## Public boundaries and core types

- `domain`: IDs, entities, enum vocabularies, transition validation, and priority calculation; no database, TUI, agent, or LanceDB types.
- `storage::ProjectStore`: authoritative transactional operations and query methods.
- `agents::AgentClient`: provider-neutral structured execution returning validated response DTOs.
- `retrieval::EmbeddingProvider` and `retrieval::SemanticIndex`: asynchronous provider boundaries using domain-neutral semantic records and matches.
- `workflow::ProjectService`: application use cases; callers never coordinate raw repository mutations themselves.
- `documents::PackageRenderer`: produces known artifacts from a stored snapshot.
- `workflow::Validator`: returns structured validation findings and applies only explicitly safe repairs.

## Error handling and atomicity

- Domain validation errors use `thiserror` and describe invalid values or transitions.
- Application boundaries add context with `anyhow` and print concise user-facing errors.
- Database writes use transactions. Duplicate/conflict classification, overflow checks, and referential validation occur before mutation when possible.
- An invalid brief, malformed model response, invalid answer, broken trace endpoint, or unsupported transition leaves authoritative state unchanged.
- Semantic errors never invalidate an already committed SQLite operation; the sync state records `Stale` with a diagnostic.
- Production paths avoid `unwrap`, `expect`, and deliberate panics.

## State invariants

- Every entity belongs to exactly one project, and cross-project trace links are rejected.
- Aggregate counts are queried from authoritative rows rather than separately mutable counters.
- A resolved question retains its original text and all answers.
- Superseded/rejected/resolved records remain queryable historically but are excluded from active retrieval by default.
- Stable display IDs are unique by `(project_id, entity_type, sequence)`.
- Every accepted decision has at least one provenance-bearing inbound trace or is marked `NeedsConfirmation`.
- Research-derived claims cite at least one evidence record.
- Generated document metadata matches content stored on disk at the recorded path and content hash.
- Semantic index records contain locators and derived text only; SQLite remains sufficient to rebuild them.
- Rejected operations do not consume display-ID sequences or partially mutate related rows.

## Evaluator risks

- Sentence heuristics may under-extract unusual briefs; fixture coverage must show conservative behavior rather than false confidence.
- SQLite transaction and restart tests must prove history preservation, not only happy-path reads.
- Stable-ID allocation can race if a future multi-process writer is added; transactions and unique constraints are required now.
- LanceDB and Arrow APIs evolve quickly, so all concrete types stay within `retrieval::lancedb_store` and the dependency is pinned by `Cargo.lock`.
- Vector filtering must always include `project_id`, active status, and requested entity types to prevent leakage or stale context.
- A large cascade of reconciliation changes must be proportional to the affected project and emitted trace work, without global scans.
- TUI terminal restoration must work on normal exit and panic.
- Generation must use a single snapshot or transaction to avoid documents mixing states from different moments.

## Testing strategy

- Unit tests cover typed parsing, lifecycle transitions, priority boundaries, structured-output validation, semantic text, hashing, and deterministic validation rules.
- In-memory SQLite tests cover every migration and transactional invariant.
- Temporary on-disk integration tests cover restart/resume, generation, export, and stale-index recovery.
- Ratatui rendering uses its test backend; workbench key handling, command parsing, queue advancement, timeline projection, snapshot diffs, and recoverable input errors are tested as pure state transitions and deterministic workflow boundaries.
- Semantic tests use deterministic embeddings. Feature-gated LanceDB tests cover local index creation and rebuild without paid APIs.
- CLI tests exercise help, invalid input, the fishing fixture, and non-interactive output.
- Each behavior change begins red and is made green before its commit.

## Code style

All functions, structs, and enums—including private and test items—receive meaningful Rust documentation comments as required by `AGENTS.md`. Types model valid states; recoverable errors use `Result`; callers borrow data unless ownership naturally transfers.

```rust
/// Calculates whether a clarification should interrupt the user.
pub fn requires_attention(&self, threshold: u16) -> bool {
    self.priority_score() >= threshold && self.status.is_open()
}
```

Inline comments are reserved for non-obvious invariants in complex or long functions.

## Dependencies

- `clap` with derive: typed CLI and subcommands.
- `rusqlite` with bundled SQLite: local authoritative persistence without a system SQLite prerequisite.
- `serde` and `serde_json`: schema-valid agent boundaries and export.
- `uuid`: non-guessable internal identifiers independent of display sequences.
- `chrono`: UTC audit timestamps.
- `thiserror` and `anyhow`: typed inner errors plus contextual CLI errors.
- `ratatui` and `crossterm`: intentional cross-platform interactive terminal UI.
- `tracing` and `tracing-subscriber`: inspectable workflow and retrieval metadata without credentials.
- `tokio`: concrete async need for LanceDB and subprocess adapters.
- `async-trait`: object-safe asynchronous agent and semantic interfaces.
- `sha2`: stable content hashes for document and semantic synchronization.
- optional `lancedb` plus compatible Arrow crates: derived local vector storage only.
- `tempfile`: isolated Codex schema/output exchange plus restart and database tests.

No dependency is allowed to leak provider-specific types into the domain model.

## Boundaries

- Always: validate before mutation; run the milestone quality suite; update `CHANGELOG.md` for notable behavior; preserve user changes; keep credentials out of logs.
- Ask first: incompatible persisted-schema changes after release, a live paid model call, publishing, deployment, or destructive removal of project data.
- Never: fabricate evidence or evaluation, decide a consequential ambiguity without explicit user input or `--auto-answer` delegation, treat semantic similarity as truth, commit secrets, or weaken a valid test to ease implementation.

## Compatibility surfaces

- The baseline files and command remain unchanged under `baseline/`.
- The new CLI executable is `project-init` and the SQLite schema is migration-managed.
- JSON exports use explicit version metadata.
- Generated filenames follow the baseline where useful while adding traceability, assumption, open-question, decision, and validation artifacts.

## Open questions resolved for initialization

No unresolved question prevents implementation. The optional-LanceDB decision is reversible, deterministic analysis remains an explicit offline mode, and future model-provider configuration can be added behind the existing trait without changing authoritative data.

## Authoritative API sources

- LanceDB Rust 0.37.x uses `lancedb::connect(uri).execute().await` and Arrow record batches for table creation: <https://docs.rs/lancedb/latest/lancedb/>
- LanceDB connection/table operations are builder-based and asynchronous: <https://docs.rs/lancedb/latest/lancedb/connection/struct.Connection.html>
- Rusqlite 0.40.x exposes transactions, prepared statements, batch execution, and pragma helpers on `Connection`: <https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html>
- Ratatui 0.30.x recommends `ratatui::run` for automatic setup and terminal restoration: <https://docs.rs/ratatui/latest/ratatui/fn.run.html>
- Clap 4.6.x derive requires the `derive` feature and models subcommands with `#[command(subcommand)]`: <https://docs.rs/clap/latest/clap/_derive/>
- Codex non-interactive mode provides JSONL activity, strict output schemas, ephemeral sessions, and read-only sandboxing: <https://learn.chatgpt.com/docs/non-interactive-mode>
- Tokio child processes support explicit kill-and-wait cancellation plus kill-on-drop cleanup: <https://docs.rs/tokio/latest/tokio/process/struct.Child.html>

`Cargo.lock` records the exact compatible dependency graph used by the implementation.
