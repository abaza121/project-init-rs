# Implementation Plan: OpenCode CLI Provider

## Overview

Add `--provider opencode` as a peer to the existing Codex and local providers. The new adapter will use the authenticated OpenCode CLI profile, run prompts non-interactively with JSON event output, and preserve Project Init's existing validation, staging, timeout, cancellation, and activity-recording boundaries.

## Architecture Decisions

- Add a dedicated `OpenCodeCliClient` rather than generalizing the Codex adapter. The provider protocols differ enough that a shared abstraction would add unproven complexity.
- Resolve a native OpenCode executable from `OPENCODE_BIN` or `PATH`; retain `CODEX_BIN` semantics exclusively for Codex.
- Invoke `opencode run --format json` without `--auto`. OpenCode's configured profile remains authoritative for model, authentication, agent, plugins, and permissions; Project Init must not automatically approve otherwise-denied work.
- Treat every OpenCode event and final model response as untrusted: bound input sizes, sanitize displayed activity, parse structured JSON defensively, and retain existing response validation before persistence.
- Keep Codex as the default and retain the existing local-provider behavior and persisted provider records.

## Source Basis

- OpenCode documents `opencode run [message..]` as its non-interactive execution command and `--format json` as raw JSON event output: <https://dev.opencode.ai/docs/cli/>.
- OpenCode's CLI source shows JSON streams include completed text parts and terminate when the active session becomes idle: <https://github.com/anomalyco/opencode/blob/dev/packages/opencode/src/cli/cmd/run.ts>.

## Task List

### Phase 1: Provider Contract and CLI Boundary

- [ ] Task 1: Define OpenCode provider selection and executable resolution.
- [ ] Task 2: Implement and test the read-only structured OpenCode execution path.

### Checkpoint: Structured Operations

- [ ] Focused provider and adapter tests pass.
- [ ] `cargo check --all-features` succeeds.

### Phase 2: Full Workflow Parity

- [ ] Task 3: Add documentation execution and workbench integration.
- [ ] Task 4: Document the option and update the changelog.

### Checkpoint: Complete

- [ ] Full formatting, build, test, and lint verification passes.
- [ ] Security review confirms no automatic permission escalation and no unsanitized model/event data crosses a boundary.
- [ ] Manual OpenCode smoke test is attempted with an authenticated, healthy local OpenCode installation.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| OpenCode JSON event shape changes | High | Parse only required event fields, bound each line, and test fixture variants. |
| OpenCode profile lacks a required permission | Medium | Fail closed with sanitized diagnostics; never add `--auto`. |
| Installed CLI is unavailable or unhealthy | Medium | Fail before workflow mutation with `OPENCODE_BIN`/installation guidance. |
| Codex behavior regresses | High | Preserve its adapter and default selection; run existing suite. |

## Open Questions

- The current installed `opencode.exe` failed while preparing its config directory, so authenticated end-to-end verification depends on repairing that local installation. The adapter's unit and integration tests must not require a real account.
