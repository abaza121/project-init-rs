# Architecture

## Decision summary

Project Init uses a layered single-package Rust architecture with a library target and a thin CLI target. SQLite is the source of truth. LanceDB is an optional derived semantic index accessed only through a provider-neutral boundary. Workflow services coordinate validated state transitions, agent clients return structured responses, renderers generate documents from database snapshots, and validators produce stored findings.

This deviates from the directory sketch in `init.md` only by keeping one Cargo package instead of immediately creating a workspace. The module boundaries are still explicit, but build and release coordination stay small until separate crates have a demonstrated ownership or compatibility need.

## Component responsibilities

| Component | Responsibility | Must not know about |
| --- | --- | --- |
| `domain` | Typed entities, IDs, controlled vocabularies, transition rules | SQL, terminals, providers, LanceDB |
| `storage` | Migrations, transactions, authoritative queries | TUI widgets, agent prompts, vectors |
| `agents` | Structured request/response schemas and provider adapters | SQL schema, document paths |
| `workflow` | Use cases and state-machine orchestration | Concrete LanceDB/Arrow types |
| `retrieval` | Semantic representation, embeddings, sync, candidate search | Authoritative relationship decisions |
| `documents` | Deterministic package rendering from snapshots | Live agents and mutable global state |
| `tui` | User interaction and safe event-loop lifecycle | SQL details and provider payloads |
| `cli` | Argument parsing, configuration, and command dispatch | Business-rule implementation |

## Lifecycle

```text
Draft
  -> Analyzing
  -> AwaitingClarification
       -> NeedsUserInput (blocking conflict)
       -> Researching
  -> Planning
  -> Generating
  -> Validating
       -> NeedsUserInput (unrepairable high severity)
       -> Generating (bounded safe repair)
       -> Complete
```

Opening an existing project does not infer the next state. The persisted status and open validation/question records determine the available actions.

## Transition atomicity

Each use case prepares and validates a complete transition before committing it in one SQLite transaction. When an answer is accepted, the transaction stores the immutable answer, updates the question status, supersedes or confirms affected findings, creates replacement knowledge if needed, records traces, and sets the next project status. Any error rolls back all of those changes.

Semantic synchronization runs after commit. A failed index update records a stale state and warning; it does not erase or roll back the accepted answer.

## Interactive workbench boundary

`project-init open` composes a `WorkspaceState` from one authoritative `ProjectSnapshot`, persisted workflow context, and analysis activity. Keyboard handling emits a closed `WorkspaceCommand` rather than mutating storage directly. `ProjectService` applies answers, user-authored questions, clarification-threshold changes, decision authority, and repair authorization; the workbench then reloads the complete snapshot, calculates an affected-entity diff, and advances the open clarification queue.

`/resume` delegates to the same `WorkflowRunner` used by CLI `run`. A first workbench run persists only after the user selects an approval policy with no default. Long-running provider work owns a separate `ProjectService` and SQLite connection on the Tokio runtime, streams bounded sanitized activity to a progress overlay, applies the selected provider's liveness limits, and accepts cancellation through a shared token. Foreground mutations remain disabled until the worker returns, after which the workbench reloads SQLite and renders the next question, approval, repair, failure, or completion guidance.

Selection, focus, composer text, structured question drafts, timeline rows, and mutation diffs are disposable presentation state. SQLite remains authoritative. The projected timeline uses record update timestamps for mutable status and a stable rendering-only tie-breaker; equal-time records remain visible and the tie-breaker does not claim causal order.

## Agent boundary

`AgentClient` accepts validated focused context, a bounded activity channel, and a cancellation token. `ConfiguredProvider` is a closed dispatcher over the shipped Codex and local HTTP adapters; workflow code continues to depend on the capability-specific `AgentClient`, `ResearchClient`, `AutoAnswerClient`, and `DocumentationClient` contracts. Adding a future provider therefore changes adapter construction and dispatch rather than project state or workflow policy.

The Codex CLI adapter invokes `codex exec` directly with JSONL events, a strict output schema, an ephemeral session, a read-only sandbox, and a five-minute deadline. Brief text travels through stdin rather than process arguments. The local adapter uses a proxy-free `reqwest` client against a numeric loopback `/v1` endpoint, streams OpenAI-compatible chat completions, applies the same JSON schemas and response validators, resets inactivity only for valid events, and enforces an unresettable ten-minute hard limit. Tests use deterministic subprocess and loopback HTTP fixtures and never require a live model.

`ResearchClient` preserves the original single-question cited-research boundary. Parallel automatic clarification uses the additive `AutoAnswerClient`, whose coordinator selects the current blocker and at most two independent consequential questions, whose workers reuse the same focused research contract, and whose judge must return every provisional candidate exactly once. The runner shares the immutable client through `Arc`, uses a fixed three-task `JoinSet`, retries each failed worker and judge once, and streams separate actor-keyed progress without changing persisted activity records. Every Codex call remains ephemeral and read-only, fetched pages remain untrusted evidence, and malformed, uncited, missing, duplicate, or extra answers fail closed.

Worker candidates and judge output remain provisional. After a final cancellation check, one workflow service call validates question identity and stores every still-open judged answer, its evidence, derived requirement, and provenance links in one SQLite transaction. A missing question or persistence error rolls back the complete batch; an answer already closed by another process is skipped.

Codex JSONL and local SSE are translated into provider-neutral, sanitized activity before reaching the TUI. Unknown compatible fields are ignored, malformed events fail or become bounded warnings at the appropriate adapter boundary, and history is bounded. The analyzer response is a typed collection of proposed findings and contradictions. Provenance is assigned by the application from the brief context; the model cannot claim that its own inference came directly from the user.

Successful project creation, findings, questions, agent-run metadata, and activity history commit in one SQLite transaction. Spawn failure, timeout, cancellation, non-zero exit, or invalid structured output leaves no authoritative state.

## Retrieval boundary

`SemanticIndex` stores `SemanticRecord` locators, semantic text, vectors, content hashes, status, model/version, and timestamps. Search returns IDs and scores. `ContextBuilder` then reloads authoritative entities from SQLite, removes stale/superseded/duplicate records, ranks, limits, and records retrieval telemetry.

Relational mode uses deterministic queries only. Semantic mode adds candidates but does not change authority or trace semantics. This makes the comparison an ablation of retrieval assistance rather than a comparison of two different persistence models.

## Document and validation boundary

Generation reads a consistent project snapshot and emits known artifacts section by section. Every important rendered claim carries or links a stable source ID. The validator checks both stored relationships and filesystem outputs, persists a validation run and findings, and transitions to `Complete` only with no unresolved high-severity failures.

Deterministic repairs regenerate a missing index, refresh a stale generated file, or add a mechanically derivable traceability row. Bounded provider repair may revise provisional documents, but product choices, unsupported evidence claims, and contradiction resolutions always return to the user.

The complete-loop planner is observational: it derives exactly one next step from the project snapshot, active workflow run, approval policy, document hashes, and newest validation. The shared runner applies that step through workflow services for both CLI and TUI callers. Codex documentation generation and explicitly authorized repair occur in a temporary `workspace-write` staging directory. Local documentation keeps the model outside the filesystem boundary: strict `list_expected_documents`, `read_document`, and `write_document` calls accept only exact expected paths, and Project Init performs bounded staging writes. For either provider, only a complete expected package is adopted, and registered or automatically detected manual overrides are never replaced. Repair authorization and validation pass counts are persisted and bounded by the project repair limit.

By default, a question remains a stable user-input boundary. `--auto-answer` explicitly attaches an `AutoAnswerClient` to the runner, while `/auto-answer` activates the same path from the workbench after mandatory first-run policy selection. Attached CLI terminals open the actor-keyed auto-answer board; non-terminal automation remains headless. Planning, worker, judge, or adoption failure pauses the run with `research_failed`; cancellation commits no provisional batch state, and approval and repair boundaries retain their existing semantics.

## Operational behavior

- Default data root: platform-local application data, overridable with `--data-dir`.
- Per-project generated package: `<data-dir>/projects/<project-id>/Docs` unless exported elsewhere.
- SQLite foreign keys are enabled for every connection.
- Schema migrations are monotonic and transactional.
- Logs are structured and avoid brief contents by default; `--trace-retrieval` exposes IDs, counts, scores, discard reasons, and duration, but never credentials.
- Terminal ownership uses Ratatui's documented `run` helper so normal errors and panics restore terminal state.
- Native Codex executable discovery bypasses Windows shell shims and can locate the binary bundled by npm; `CODEX_BIN` accepts a full native executable path, while `--offline` selects the deterministic analyzer without acting as an automatic fallback.
- Explicit `--provider local` requires pinned Gemma and versioned mistral.rs settings, checks `/health` and `/v1/models`, and verifies the GGUF identity before launching a named loopback Docker runtime when needed. It is not selected automatically.

## Security and resource posture

- Briefs, model output, and paths are untrusted input.
- Export paths are canonicalized or created beneath an explicit target; generated relative paths cannot escape the target root.
- Agent subprocess arguments are passed without shell interpolation.
- The local container receives only a read-only model bind mount and Docker-owned cache volume, binds to loopback, drops capabilities, uses a read-only root and bounded tmpfs, and enables search without shell or Python execution.
- Model-directed document paths are exact allowlisted enum values; traversal, links, unknown arguments, excessive tool rounds, oversized SSE/content, and incomplete packages fail closed.
- SQLite parameters are bound rather than concatenated.
- Brief, answer, and agent-response sizes are bounded.
- Search `top_k`, repair attempts, and context sizes are configurable with conservative maxima.
- Semantic records and sync state are bounded by authoritative entities; no unbounded duplicate history is created in LanceDB.
