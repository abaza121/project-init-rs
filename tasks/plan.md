# Implementation Plan: Migrate OpenCode to the Headless Server API

## Overview

Replace Project Init's prompt-only `opencode run --format json` adapter with a managed, loopback-only `opencode serve` client. The migration will preserve the existing `--provider opencode` selection, provider-neutral capability traits, workflow validation, staged document boundary, activity history, cancellation, timeout policy, and atomic persistence. Structured operations will use OpenCode's server `format: { type: "json_schema", ... }` contract so schema validation is performed by OpenCode before Project Init applies its own domain validation.

The existing CLI-provider implementation remains the historical baseline in the completed tasks below. The new work starts after that baseline and must not weaken its safety or compatibility guarantees.

## Current State

- `OpenCodeCliClient` now lazily owns a loopback `opencode serve` child and creates a fresh HTTP session for each operation.
- Structured operations send native JSON Schema requests and read `info.structured_output`; provider-neutral validators remain the second validation boundary.
- Codex already has native `--output-schema` enforcement; OpenCode's documented CLI `--format json` is only an event-stream format selector.
- OpenCode's documented server exposes `/global/health`, session creation, synchronous message prompting, abort, deletion, and an SSE `/event` stream.
- OpenCode's SDK/server API accepts `format.type = "json_schema"`, a JSON Schema object, and optional `retryCount`, returning validated structured output or `StructuredOutputError`.

## Goals and Non-goals

### Goals

- Use `opencode serve` as the OpenCode transport for analysis, research, automatic answers, generation, repair, and workbench resume.
- Enforce the analysis/research schemas through OpenCode's structured-output API.
- Preserve one-command startup for Project Init by managing the server child process automatically.
- Preserve concurrent automatic-answer workers by using independent OpenCode sessions over one server instance.
- Preserve bounded, sanitized activity, cancellation, timeout, error, and cleanup behavior.

### Non-goals

- Do not replace the provider-neutral traits or workflow policy.
- Do not make OpenCode permissions more permissive; never pass `--auto`.
- Do not make OpenCode's server a general remote-control endpoint in the first migration.
- Do not remove Codex, local inference, or the existing `--provider opencode` spelling.

## Architecture Decisions

- Retain the `ConfiguredProvider` capability dispatch and replace only the internal OpenCode transport.
- Introduce a server-backed client with a managed child process bound explicitly to `127.0.0.1`. The process inherits the user's OpenCode configuration, authentication, model, agent, plugin, and permission profile.
- Pass `--port 0` and accept only the loopback URL announced by the child, avoiding a fixed-port collision while keeping the listener private.
- Start the server lazily on the first operation, poll `/global/health` before sending work, and kill it on final client drop or terminal cancellation. Startup and shutdown diagnostics remain bounded and sanitized.
- Use one new OpenCode session per provider operation. This prevents context leakage between analysis, research, generation, repair, and concurrent auto-answer workers.
- For analysis, research, planning, and judgment, send the schema in the server message body's `format` field and read `info.structured_output`. Keep Project Init's local validators as a second boundary; OpenCode validation does not replace domain validation.
- For documentation generation and repair, use ordinary server messages in the staging directory and preserve the existing exact allowlist and adoption validator. Do not expose generic host filesystem operations through a new endpoint.
- Consume the server's SSE event stream only for provider-neutral progress. Filter by the active session identity, bound event sizes, and never persist raw tool arguments, reasoning, prompts, credentials, or arbitrary provider text.
- Keep the current hard deadline for initial analysis and inactivity deadline for research/documentation unless server startup measurements require a separately documented adjustment.
- Use a typed internal request/response contract and one consistent error mapping for HTTP status failures, malformed JSON, structured-output failures, server death, cancellation, and timeout. External response bodies are untrusted.
- Defer an attach-to-existing-server URL/port option until the managed lifecycle works end-to-end. If added later, it must be explicit, loopback-validated, and must not silently connect to an unrelated project directory.

## Source Basis

- OpenCode documents `opencode serve` as a headless HTTP server and publishes its OpenAPI specification at `/doc`: <https://dev.opencode.ai/docs/server/>.
- OpenCode documents health, session, message, abort, delete, and SSE event endpoints in the server API: <https://dev.opencode.ai/docs/server/>.
- OpenCode documents `format.type = "json_schema"`, `schema`, `retryCount`, validated `structured_output`, and `StructuredOutputError` in the SDK: <https://opencode.ai/docs/sdk/>.
- OpenCode documents that `opencode run --format json` emits raw JSON events, not schema-constrained model output: <https://dev.opencode.ai/docs/cli/>.

## Dependency Graph

```text
Server process lifecycle and HTTP contract
    -> health and session client
        -> structured-output operations
        -> staged documentation operations
    -> SSE activity and cancellation
        -> provider trait integration
            -> CLI/TUI/workbench behavior
                -> smoke test and documentation
```

## Task List

### Historical Baseline: Completed OpenCode CLI Provider

- [x] Add provider selection and native executable resolution.
- [x] Add bounded `opencode run --format json` execution and event parsing.
- [x] Integrate analysis, research, automatic answers, documentation, repair, and workbench flows.
- [x] Document the CLI provider and preserve Codex as the default.

### Phase 1: Server Contract and Lifecycle

- [x] Task 1: Define the internal OpenCode server contract and configuration.
- [x] Task 2: Implement managed server startup, health probing, and shutdown.
- [x] Task 3: Implement validated session/message HTTP operations with bounded response handling.

### Checkpoint: Server Foundation

- [x] The server client can start only on loopback and fails clearly when startup or health verification fails.
- [x] Deterministic fixtures cover health, active/silent status, malformed/missing structured fields, and process cleanup paths.
- [x] Existing provider-neutral tests still pass without a live OpenCode account.

### Phase 2: Schema-Enforced Structured Operations

- [x] Task 4: Migrate analysis to server `json_schema` output.
- [x] Task 5: Migrate research, automatic-answer planning, workers, and judgment to server `json_schema` output.
- [x] Task 6: Preserve local validation and map OpenCode structured-output failures to actionable provider errors.

### Checkpoint: Structured Operations

- [x] The request builder places the exact Project Init schema under `format.schema`.
- [x] A response with `kind: "agent_inference"` is rejected by the local validator before persistence, and the prompt identifies the correct `source_type` field.
- [x] Structured-output failure, cancellation, timeout, and invalid `structured_output` are covered.

### Phase 3: Documentation, Activity, and Cancellation

- [x] Task 7: Migrate generation and repair to server sessions rooted in the existing staging directory.
- [x] Task 8: Add filtered SSE activity translation with bounded parsing and safe redaction.
- [x] Task 9: Wire cancellation, hard/inactivity deadlines, abort requests, and child cleanup across concurrent sessions.

### Checkpoint: Full Provider Parity

- [x] Analysis, research, auto-answer, generation, repair, and workbench resume use the server-backed client.
- [x] Failed, cancelled, timed-out, or malformed operations adopt no partial project state or document package.
- [x] Concurrent automatic-answer workers do not share session context or duplicate activity incorrectly.

### Phase 4: Compatibility, Verification, and Documentation

- [x] Task 10: Preserve CLI/provider construction and add deterministic HTTP/process integration fixtures.
- [x] Task 11: Attempt an authenticated end-to-end smoke test with the installed OpenCode version and record version/startup behavior.
- [x] Task 12: Update README, architecture documentation, changelog, and troubleshooting guidance for the server lifecycle.

### Checkpoint: Complete

- [x] `cargo fmt --check` passes.
- [x] `cargo check --all-features` passes.
- [x] `cargo test --all-features` passes.
- [x] `cargo clippy --all-targets --all-features -- -D warnings` passes.
- [x] The original `dead-air-dispatch.txt` reproduction fails with a clear OpenCode server/provider installation diagnostic in the current environment.
- [x] No live credentials, raw prompts, raw tool arguments, or unsanitized provider responses enter activity history or logs.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| OpenCode server API changes between installed versions | High | Probe `/global/health`, inspect `/doc` during smoke testing, keep DTO parsing strict only where needed, and fixture the supported contract. |
| Server startup succeeds before plugins/configuration are usable | High | Treat health as necessary but not sufficient; verify the first controlled session request within the operation deadline and surface bounded diagnostics. |
| Port collision or stale server process | High | Use a dedicated managed lifecycle, explicit loopback binding, startup retries/health verification, and cleanup on every terminal path. Decide port allocation in Task 1 before implementation. |
| SSE event vocabulary changes | Medium | Parse only session identity, event type, and a small allowlist of safe fields; ignore unknown compatible events. |
| Structured output is valid JSON but violates Project Init semantics | High | Retain `StructuredAnalysis`, research, package, and snapshot validators after the server response. |
| Concurrent sessions mix progress | Medium | Generate an operation/session correlation map and filter all activity before forwarding it. |
| Windows `opencode.exe` installation or wrapper failure | Medium | Add a preflight health/startup diagnostic, retain `OPENCODE_BIN`, and keep the fixture suite independent of the local installation. |
| Server inherits unsafe project permissions | High | Bind to loopback, preserve the user's profile without `--auto`, scope working directories, and never expose new generic shell/filesystem calls from Project Init. |

## Open Questions to Resolve Before Implementation

Resolved during implementation:

- Use the current OpenCode ephemeral-port behavior with `--port 0`; accept only the announced `http://127.0.0.1:<port>` URL.
- Inherit `OPENCODE_SERVER_PASSWORD` and optional `OPENCODE_SERVER_USERNAME` from the child environment; do not add a credential CLI option or print credentials.
- Allowlist only session-correlated lifecycle, progress, tool-name, permission, and error events; ignore unknown event types and provider text.
- Share one managed process among clones of a client instance, but create one session per capability call. Defer attach mode.

## Definition of Done

- The OpenCode provider no longer depends on prompt-only schema instructions for structured operations.
- Every externally supplied HTTP response is bounded, parsed, validated, and safely mapped before use.
- All existing provider-neutral workflow and atomic persistence guarantees remain intact.
- The migration is documented as a user-visible integration change and has a recorded smoke-test result.
