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

## Agent boundary

`AgentClient` accepts an operation name, validated focused context, and a response schema identifier. The Codex CLI adapter makes the executable and arguments visible in logs, sends no credentials, bounds execution, and rejects non-conforming JSON. Tests use a deterministic fixture client.

The analyzer response is a typed collection of proposed findings and contradictions. Provenance is assigned by the application from the operation context; the model cannot claim that its own inference came directly from the user.

## Retrieval boundary

`SemanticIndex` stores `SemanticRecord` locators, semantic text, vectors, content hashes, status, model/version, and timestamps. Search returns IDs and scores. `ContextBuilder` then reloads authoritative entities from SQLite, removes stale/superseded/duplicate records, ranks, limits, and records retrieval telemetry.

Relational mode uses deterministic queries only. Semantic mode adds candidates but does not change authority or trace semantics. This makes the comparison an ablation of retrieval assistance rather than a comparison of two different persistence models.

## Document and validation boundary

Generation reads a consistent project snapshot and emits known artifacts section by section. Every important rendered claim carries or links a stable source ID. The validator checks both stored relationships and filesystem outputs, persists a validation run and findings, and transitions to `Complete` only with no unresolved high-severity failures.

Safe repairs are mechanical: regenerate a missing index, refresh a stale generated file, or add a deterministically derivable traceability row. Product choices, evidence claims, and contradiction resolutions always return to the user.

## Operational behavior

- Default data root: platform-local application data, overridable with `--data-dir`.
- Per-project generated package: `<data-dir>/projects/<project-id>/Docs` unless exported elsewhere.
- SQLite foreign keys are enabled for every connection.
- Schema migrations are monotonic and transactional.
- Logs are structured and avoid brief contents by default; `--trace-retrieval` exposes IDs, counts, scores, discard reasons, and duration, but never credentials.
- Terminal ownership uses Ratatui's documented `run` helper so normal errors and panics restore terminal state.

## Security and resource posture

- Briefs, model output, and paths are untrusted input.
- Export paths are canonicalized or created beneath an explicit target; generated relative paths cannot escape the target root.
- Agent subprocess arguments are passed without shell interpolation.
- SQLite parameters are bound rather than concatenated.
- Brief, answer, and agent-response sizes are bounded.
- Search `top_k`, repair attempts, and context sizes are configurable with conservative maxima.
- Semantic records and sync state are bounded by authoritative entities; no unbounded duplicate history is created in LanceDB.
