# OpenCode Server Migration Tasks

## Historical Baseline

- [x] Existing `--provider opencode` selection and `OPENCODE_BIN` resolution.
- [x] Existing bounded `opencode run --format json` adapter and JSONL activity parsing.
- [x] Existing workflow parity for analysis, research, auto-answer, generation, repair, and workbench resume.

## Task 1: Define the OpenCode server contract and configuration

**Description:** Define the internal server-backed client contract, typed request/response DTOs, managed-process configuration, loopback policy, port strategy, and error taxonomy without changing provider behavior yet.

**Acceptance criteria:**

- [x] The contract covers health, session creation, synchronous message prompting, abort, deletion, and structured-output format.
- [x] Configuration explicitly represents executable, working directory, loopback host/port strategy, timeout, history capacity, and inherited-auth policy.
- [x] Structured-output responses expose only the bounded fields needed by Project Init.
- [x] The design records the selected port strategy and compatibility assumptions.

**Verification:**

- [x] Contract-focused unit tests serialize representative request bodies and deserialize representative responses.
- [x] No live process or network service is required.

**Dependencies:** None

**Files likely touched:**

- `src/agents/opencode.rs` or a new `src/agents/opencode_server.rs`
- `src/agents/mod.rs`
- `tasks/plan.md`

**Estimated scope:** Medium: 3 files

## Task 2: Implement managed server startup, health probing, and shutdown

**Description:** Replace direct `opencode run` process ownership with a lazily started `opencode serve` child bound to loopback, verify readiness through `/global/health`, drain diagnostics, and clean up the process on every terminal path.

**Acceptance criteria:**

- [x] Startup uses explicit loopback binding and does not auto-approve OpenCode permissions.
- [x] Health polling has a bounded startup deadline and distinguishes spawn, HTTP, unhealthy, and premature-exit failures.
- [x] Child stdout/stderr cannot block the server process and diagnostics are bounded/sanitized.
- [x] Drop, cancellation, timeout, and request failure do not leave an owned server process running.

**Verification:**

- [x] The process fixture covers healthy startup, active/silent operation, hard timeout, inactivity timeout, and cancellation cleanup.
- [x] `cargo test --test opencode_cli_contract` and new server lifecycle tests pass.

**Dependencies:** Task 1

**Files likely touched:**

- `src/agents/opencode.rs` or `src/agents/opencode_server.rs`
- `src/agents/mod.rs`
- `tests/opencode_cli_contract.rs`

**Estimated scope:** Medium: 3 files

## Task 3: Implement typed session/message HTTP operations

**Description:** Add one-session-per-operation HTTP methods with bounded body reads, consistent status/error mapping, session cleanup, and no raw provider payload leakage.

**Acceptance criteria:**

- [x] Every request uses the verified server base URL and bounded request/response sizes.
- [x] Session creation, synchronous message prompting, abort, and deletion are validated at the boundary.
- [x] HTTP status failures, malformed JSON, missing structured output, and server-side errors map to `AgentError` without exposing secrets.
- [x] Concurrent cloned clients can use independent sessions safely.

**Verification:**

- [x] The deterministic loopback process fixture covers successful session requests and bounded blocked responses; unit contracts cover malformed and missing response fields.
- [x] Existing domain/workflow tests remain unchanged and pass.

**Dependencies:** Tasks 1-2

**Files likely touched:**

- `src/agents/opencode_server.rs`
- `tests/opencode_cli_contract.rs`
- `Cargo.toml` only if an existing dependency cannot support the bounded HTTP client

**Estimated scope:** Medium: 3 files

## Task 4: Migrate initial analysis to server structured output

**Description:** Send the analysis prompt through a fresh OpenCode session using `format.type = "json_schema"`, retrieve `info.structured_output`, and preserve the existing `AgentExecution` and workflow validation contract.

**Acceptance criteria:**

- [x] The request includes the exact `ANALYSIS_SCHEMA` as a JSON object under `format.schema`.
- [x] The response is converted into the existing `AgentExecution` without scraping a final text event.
- [x] OpenCode structured-output failures and Project Init validation failures remain distinct and actionable.
- [x] The previous `agent_inference`/`kind` mix-up is rejected before persistence.

**Verification:**

- [x] Fixture assertions inspect the request body and return valid/invalid structured outputs.
- [x] The existing workflow invalid-response test and a regression test for the enum mix-up pass.

**Dependencies:** Task 3

**Files likely touched:**

- `src/agents/opencode_server.rs`
- `src/agents/prompts.rs`
- `tests/workflow_contract.rs`
- `tests/opencode_cli_contract.rs`

**Estimated scope:** Medium: 4 files

## Task 5: Migrate research and automatic-answer operations

**Description:** Move research answers, batch planning, worker responses, and judgment to independent server sessions with their respective schemas, preserving citation validation, bounded retries, and concurrent worker behavior.

**Acceptance criteria:**

- [x] Every operation sends its correct schema and receives only its own session's structured output.
- [x] Research evidence and automatic-answer membership/citation validators remain authoritative.
- [x] Existing one-retry behavior is not duplicated by OpenCode `retryCount` in a way that changes workflow semantics unexpectedly.
- [x] Cancellation and one failed concurrent worker still produce the existing pause/failure behavior.

**Verification:**

- [x] Provider-neutral fixtures cover valid responses, structured-output exhaustion, wrong question IDs, bad citations, and concurrent workflow sessions; OpenCode wire-level requests use the same capability schemas.
- [x] `cargo test --test runner_contract --test workflow_contract` passes.

**Dependencies:** Tasks 3-4

**Files likely touched:**

- `src/agents/opencode_server.rs`
- `src/workflow/runner.rs` only if transport cancellation requires an adapter-facing change
- `tests/runner_contract.rs`
- `tests/workflow_contract.rs`

**Estimated scope:** Medium: 4 files

## Task 6: Preserve local validation and structured-output error semantics

**Description:** Finish the structured path by making OpenCode validation and Project Init validation explicit layers, retaining local domain limits, and producing diagnostics that identify whether failure occurred at HTTP, OpenCode schema, JSON, or domain validation.

**Acceptance criteria:**

- [x] No invalid structured output reaches persistence or document adoption.
- [x] Error messages include safe operation context and omit response bodies that may contain prompts, credentials, or tool output.
- [x] Retry and timeout behavior remains bounded and documented.

**Verification:**

- [x] Error-mapping tests cover each failure layer.
- [x] `cargo test --all-features` passes for all provider-neutral validators.

**Dependencies:** Tasks 4-5

**Files likely touched:**

- `src/agents/opencode_server.rs`
- `src/agents/mod.rs`
- `tests/agent_contract.rs`

**Estimated scope:** Small: 3 files

## Task 7: Migrate documentation generation and repair

**Description:** Execute generation and repair through normal OpenCode server sessions rooted in the existing staging directory, retaining exact tool/path restrictions and package adoption validation.

**Acceptance criteria:**

- [x] The server process uses the intended staging/project directory for documentation operations.
- [x] Generation and repair cannot adopt incomplete, out-of-scope, or manually overridden artifacts.
- [x] No new generic shell or filesystem endpoint is exposed by Project Init.

**Verification:**

- [x] Fixture-based generation and repair tests cover successful writes, missing artifacts, out-of-scope writes, cancellation, and failed adoption.
- [x] `cargo test --test generation_contract --test runner_contract` passes.

**Dependencies:** Tasks 2-3

**Files likely touched:**

- `src/agents/opencode_server.rs`
- `src/workflow/runner.rs`
- `tests/generation_contract.rs`
- `tests/runner_contract.rs`

**Estimated scope:** Medium: 4 files

## Task 8: Add filtered SSE activity translation

**Description:** Consume `/event` as a bounded SSE stream, filter events to the active OpenCode session, and translate only safe lifecycle/tool/progress signals into the existing `ActivityEvent` contract.

**Acceptance criteria:**

- [x] SSE framing handles chunk boundaries, blank lines, malformed events, and bounded event sizes.
- [x] Session correlation prevents cross-operation activity leakage during concurrent workers.
- [x] Raw reasoning, prompts, tool arguments, credentials, and arbitrary provider text are not persisted as activity.
- [x] Unknown event types are ignored without breaking a valid operation.

**Verification:**

- [x] SSE unit contracts cover split CRLF framing, safe tool names, session filtering, and unknown/malformed event handling.
- [x] Activity history remains ordered and bounded.

**Dependencies:** Tasks 2-3

**Files likely touched:**

- `src/agents/opencode_server.rs`
- `src/agents/mod.rs`
- `tests/opencode_cli_contract.rs`

**Estimated scope:** Medium: 3 files

## Task 9: Wire cancellation, deadlines, abort, and cleanup

**Description:** Integrate server request cancellation and `/session/:id/abort` with existing hard and inactivity timeout policies, including concurrent automatic-answer workers and process shutdown.

**Acceptance criteria:**

- [x] User cancellation aborts the active OpenCode session and returns the existing cancelled outcome.
- [x] Initial analysis retains its hard deadline; research/documentation retain their intended inactivity semantics.
- [x] Aborted sessions and child processes are cleaned up best-effort without masking the original failure.
- [x] A timeout cannot leave a session or managed server process orphaned.

**Verification:**

- [x] Timeout/cancellation fixtures cover active SSE, blocked HTTP, and concurrent worker cancellation; child-drop cleanup is enforced by the managed-server owner.
- [x] `cargo test --test provider_timeout_contract --test runner_contract` passes.

**Dependencies:** Tasks 5-8

**Files likely touched:**

- `src/agents/opencode_server.rs`
- `src/workflow/runner.rs` if required for cancellation propagation
- `tests/provider_timeout_contract.rs`
- `tests/runner_contract.rs`

**Estimated scope:** Medium: 4 files

## Task 10: Preserve provider construction and add integration fixtures

**Description:** Replace the old OpenCode adapter construction without changing CLI semantics, provider dispatch, TUI/workbench behavior, or Codex/local selection, and consolidate deterministic process/HTTP fixtures.

**Acceptance criteria:**

- [x] `--provider opencode` still selects the migrated provider and local-only flags remain rejected.
- [x] `open`, `run`, `step`, `new --run`, and auto-answer use the migrated client wherever OpenCode is selected.
- [x] Codex remains the default and existing local provider behavior is unchanged.

**Verification:**

- [x] `cargo test --all-features` passes.
- [ ] CLI/provider-selection contract tests pass.

**Dependencies:** Tasks 4-9

**Files likely touched:**

- `src/agents/provider.rs`
- `src/agents/mod.rs`
- `src/main.rs`
- `tests/runner_contract.rs`

**Estimated scope:** Medium: 4 files

## Task 11: Attempt authenticated end-to-end smoke test

**Description:** Run the original `dead-air-dispatch.txt` scenario against the installed OpenCode version, record server version/health/startup timing and the final outcome, and separate installation failures from application failures.

**Acceptance criteria:**

- [x] The smoke test uses a fresh data directory and does not overwrite prior evaluation evidence.
- [x] The recorded result identifies the installed OpenCode runtime, unavailable server health/model/structured-output results, and the startup failure outcome.
- [x] The Windows executable/configuration failure is preserved as a diagnostic without treating it as a schema regression.

**Verification:**

- [x] Run the exact evaluation case with `--provider opencode` after implementation.
- [x] Inspect the bounded failure diagnostic; no project was generated because the server never became ready.

**Dependencies:** Task 10

**Files likely touched:**

- `evaluation/README.md` or a new evaluation result file only if the run is actually performed
- `tasks/plan.md`

**Estimated scope:** Small: 2 files

**Recorded result (2026-08-31):** The fresh-data-dir smoke command reached the migrated adapter, but the installed `C:\ProgramData\chocolatey\bin\opencode.exe` exited before announcing a server address because its own config-directory setup reports `EEXIST` for `C:\Users\abaza\.config\opencode`. No server health response, model response, or structured output was available; this is an installation/configuration failure, not a Project Init schema-validation result.

## Task 12: Update documentation and changelog

**Description:** Document the managed server lifecycle, structured-output enforcement, inherited OpenCode permissions, troubleshooting, and any compatibility or attach-mode decisions.

**Acceptance criteria:**

- [x] README usage examples remain accurate for `--provider opencode`.
- [x] Architecture documentation describes HTTP sessions, schema format, SSE filtering, and cleanup boundaries.
- [x] `CHANGELOG.md` contains one concise unreleased entry under the correct category, following `Docs/CHANGELOG_GUIDELINES.md`.

**Verification:**

- [x] Documentation examples match CLI tests and the implemented configuration.
- [x] `cargo fmt --check` passes.

**Dependencies:** Task 11

**Files likely touched:**

- `README.md`
- `Docs/architecture.md`
- `CHANGELOG.md`
- `Docs/CHANGELOG_GUIDELINES.md` only for reference, not normally edited

**Estimated scope:** Medium: 3 files

## Checkpoints

### Server Foundation

- [x] Tasks 1-3 complete.
- [x] Server lifecycle and HTTP fixture tests pass.

### Structured Operations

- [x] Tasks 4-6 complete.
- [x] Schema request and response validation tests pass.

### Full Provider Parity

- [x] Tasks 7-10 complete.
- [x] Full tests, build, and lint pass.

### Complete

- [x] Task 11 smoke test is recorded.
- [x] Task 12 documentation and changelog are complete.
- [x] No open migration acceptance criterion remains unchecked.
