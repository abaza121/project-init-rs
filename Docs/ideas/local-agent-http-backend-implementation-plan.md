# Implementation Plan: Local HTTP Analysis Backend

## Overview

Implement the confirmed local provider as additive vertical slices. The dependency order is shared provider configuration and prompt contracts, then safe HTTP transport, then structured analysis/research, then allowlisted document tools, then runtime launch and user-facing composition. Existing workflow policy, validation, staging, and persistence stay unchanged.

## Implementation Status

The automated implementation scope is complete as of 2026-08-30. Formatting, all-feature compilation, 162 repository tests, strict all-target Clippy, and the documented CLI help command pass on the installed stable toolchain. All-feature compilation and the complete test suite also pass on Rust 1.88. The live CPU/CUDA capability smoke remains an operator release gate because automated tests intentionally do not download the multi-gigabyte model or start Docker.

## Architecture Decisions

- Keep `AgentClient`, `ResearchClient`, `AutoAnswerClient`, and `DocumentationClient` as the stable capability boundaries.
- Add a closed `ConfiguredProvider` dispatcher for the providers compiled into this binary; future providers add one variant and their own adapter rather than changing workflow code.
- Use `reqwest` with Rustls plus streaming response bytes; do not embed the full `mistralrs` inference crate or a Docker SDK.
- Treat the local HTTP server, model output, search results, and tool arguments as untrusted external input.
- Give the model exact document tools implemented by Project Init, not a mounted staging directory, shell, Python, or generic filesystem API.
- Reuse shared prompts and JSON schemas between Codex and local adapters so provider selection cannot silently change the product contract.
- Preserve `--offline` as deterministic and explicit. Provider failure never triggers another provider automatically.

## Dependency Graph

```text
Shared prompts + provider configuration
                 |
                 +--> HTTP/SSE transport --> analysis + research adapters
                 |                              |
                 |                              +--> documentation tool loop
                 |
                 +--> runtime config --> Docker argument builder --> readiness launcher
                                                        |
                         CLI + workbench composition ----+
                                                        |
                                  setup/security docs + changelog
```

## Task 1: Define additive provider and shared prompt contracts

**Description:** Move provider-neutral prompts and JSON schemas out of the Codex implementation, add typed provider/runtime configuration, and add a closed dispatcher that implements the four existing client traits without changing those traits.

**Acceptance criteria:**

- [ ] Codex requests remain byte-for-byte equivalent where practical and existing Codex tests pass.
- [ ] `ConfiguredProvider` delegates every capability without exposing provider details to workflow code.
- [ ] Local configuration rejects blank model names, non-loopback endpoints, zero limits, floating images, and incomplete launch settings.
- [ ] Every new Rust item has a meaningful documentation comment.

**Verification:** `cargo test --all-features agents::provider`

**Dependencies:** None.

**Files likely touched:** `src/agents/mod.rs`, `src/agents/codex.rs`, `src/agents/prompts.rs`, `src/agents/provider.rs`, `tests/agent_contract.rs`.

**Estimated scope:** Medium.

## Task 2: Build bounded OpenAI-compatible HTTP transport

**Description:** Add the local HTTP adapter, health/model probes, streaming chat-completion request/response types, SSE accumulation, activity reporting, cancellation, inactivity handling, and an absolute hard deadline.

**Acceptance criteria:**

- [ ] Requests target only the configured loopback base URL and expected API paths.
- [ ] Streaming content and fragmented tool calls reconstruct deterministically within configured bounds.
- [ ] Valid stream activity resets inactivity while the hard limit never resets.
- [ ] Non-success status, oversized bodies/events, malformed JSON/SSE, cancellation, inactivity, and deadline errors are distinct and sanitized.

**Verification:** `cargo test --all-features agents::local::tests::transport`

**Dependencies:** Task 1.

**Files likely touched:** `Cargo.toml`, `Cargo.lock`, `src/agents/local.rs`, `src/agents/mod.rs`, `tests/agent_contract.rs`.

**Estimated scope:** Medium.

## Task 3: Implement structured analysis and cited research

**Description:** Implement `AgentClient`, `ResearchClient`, and `AutoAnswerClient` on the local adapter using shared prompts, JSON-schema response format, DuckDuckGo web search options for worker research, and existing response validators.

**Acceptance criteria:**

- [ ] Analysis produces an `AgentExecution` identified as `local_http` and is accepted only after existing structured analysis validation.
- [ ] Research workers request web search and return only existing validator-accepted direct HTTPS evidence.
- [ ] Planning and judgment do not gain research or filesystem capabilities they do not need.
- [ ] Unknown model fields do not break compatible responses, while missing required response fields fail closed.

**Verification:** `cargo test --all-features agents::local::tests::structured`

**Dependencies:** Tasks 1-2.

**Files likely touched:** `src/agents/local.rs`, `src/agents/mod.rs`, `src/agents/prompts.rs`, `tests/agent_contract.rs`.

**Estimated scope:** Medium.

### Checkpoint: Read-only provider capabilities

- [ ] Codex unit and contract tests remain green.
- [ ] Local fixture tests cover analysis, research, plan, and judgment success/failure.
- [ ] No provider can bypass existing domain validation or persistence transactions.

## Task 4: Add the allowlisted documentation tool loop

**Description:** Add strict schemas and bounded dispatch for `list_expected_documents`, `read_document`, and `write_document`, then implement generation and repair through repeated chat-completion rounds.

**Acceptance criteria:**

- [ ] Only exact `required_paths` are readable or writable.
- [ ] Absolute paths, traversal, alternate separators, links/reparse points, oversized content, extra arguments, duplicate call IDs, excessive rounds, and aggregate overflow fail closed.
- [ ] Writes remain under staging and use safe replacement; partial provider output is never adopted directly.
- [ ] Completion requires every expected regular UTF-8 file to exist before returning success.

**Verification:** `cargo test --all-features agents::local::tests::documentation`

**Dependencies:** Tasks 1-3.

**Files likely touched:** `src/agents/local.rs`, `src/agents/prompts.rs`, `tests/agent_contract.rs`, `tests/generation_contract.rs`.

**Estimated scope:** Large.

## Task 5: Verify or launch the hardened mistral.rs runtime

**Description:** Add a Docker readiness launcher that first checks `/health` and `/v1/models`, then builds and runs a version-pinned CPU or CUDA container with explicit arguments and polls until ready.

**Acceptance criteria:**

- [ ] A healthy compatible endpoint is reused without launching Docker.
- [ ] Docker arguments bind the host port to `127.0.0.1`, mount only the dedicated model directory read-only, drop all capabilities, set `no-new-privileges`, use a read-only root, create a bounded tmpfs, disable UI, enable search only, and never enable shell/Python/agent shorthand.
- [ ] CUDA mode adds `--gpus all` and paged attention; CPU mode adds `--cpu` and no GPU flag.
- [ ] Model filename, byte length, and SHA-256 are verified before launch.
- [ ] Docker errors and readiness timeout are bounded, sanitized, and do not fall back.

**Verification:** `cargo test --all-features agents::runtime`

**Dependencies:** Tasks 1-2.

**Files likely touched:** `src/agents/runtime.rs`, `src/agents/local.rs`, `src/agents/mod.rs`, `tests/agent_contract.rs`.

**Estimated scope:** Medium.

## Task 6: Compose provider selection across CLI and workbench

**Description:** Add global provider/local-runtime options, centralize provider construction, pass provider configuration into the workbench, and use the selected provider for creation, generation/repair, research, and automatic answering.

**Acceptance criteria:**

- [ ] No provider option preserves current Codex behavior.
- [ ] `--provider local` works before or after subcommands and is used consistently by `new`, `run`, `step`, and workbench `/resume` or `/auto-answer`.
- [ ] `--offline` conflicts with explicit local provider selection and continues to perform no network/provider work.
- [ ] Codex-only `--skills` does not alter local runtime behavior and is documented as Codex-specific.
- [ ] Provider startup failure pauses resumable workflows through existing capability-unavailable handling.

**Verification:** `cargo test --all-features --bin project-init && cargo test --all-features workspace_tui_contract`

**Dependencies:** Tasks 3-5.

**Files likely touched:** `src/main.rs`, `src/tui/mod.rs`, `src/agents/provider.rs`, `tests/workspace_tui_contract.rs`, `README.md`.

**Estimated scope:** Large.

### Checkpoint: End-to-end local option

- [ ] Existing default Codex and `--offline` CLI tests pass.
- [ ] Deterministic fixture server completes create, auto-answer, and document-generation paths.
- [ ] Cancellation and every provider failure preserve the current atomic/resumable guarantees.

## Task 7: Document setup, security, operations, and compatibility

**Description:** Add pinned Hugging Face setup, CPU/CUDA Docker selection, tuning and capability-probe instructions, offline/search limitations, troubleshooting, architecture, and changelog entries.

**Acceptance criteria:**

- [ ] Download instructions pin repository revision and verify the exact GGUF checksum.
- [ ] CPU and CUDA examples use versioned image tags and explain compute-capability selection.
- [ ] Documentation distinguishes local inference from deterministic offline mode and notes that research requires network access.
- [ ] Security documentation names every mount, network capability, enabled tool, disabled execution primitive, and validation boundary.
- [ ] `CHANGELOG.md` describes the user-facing addition under `Unreleased` without creating a release.

**Verification:** run each documented non-download command against `--help`; manually inspect links and option names.

**Dependencies:** Tasks 5-6.

**Files likely touched:** `README.md`, `Docs/architecture.md`, `Docs/known-limitations.md`, `Docs/ideas/local-agent-http-backend.md`, `CHANGELOG.md`.

**Estimated scope:** Medium.

## Task 8: Perform full verification and adversarial review

**Description:** Run the full repository quality suite, inspect the fresh diff for correctness/security/compatibility/comment findings, fix all findings, and repeat the suite.

**Acceptance criteria:**

- [ ] Formatting, compilation, all tests, and Clippy pass on stable Rust 1.88.
- [ ] Review finds no path escape, SSRF, unbounded accumulation, credential exposure, shell interpolation, floating dependency/runtime, silent fallback, or authority regression.
- [ ] Every new function, struct, and enum, including tests and private items, has a meaningful `///` comment.
- [ ] Existing untracked evaluation artifacts remain untouched.

**Verification:**

```text
cargo fmt --check
cargo check --all-features
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

**Dependencies:** Tasks 1-7.

**Files likely touched:** only files already changed by Tasks 1-7.

**Estimated scope:** Medium.

### Checkpoint: Complete

- [ ] Full quality suite passes twice: before and after review fixes.
- [ ] Saved specification and implementation plan match executable behavior.
- [ ] No live model download or paid provider call occurs in automated tests.

## Risks and Mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Gemma tool calling or schema adherence is insufficient | High | Capability probe on exact model/runtime; strict schemas; fail closed; keep Codex default. |
| 12 GB VRAM cannot sustain the requested context | High | Treat 32K as desired; run `mistralrs tune`; document verified context per machine; allow CPU offload/CPU mode. |
| Model-directed paths escape staging | Critical | Exact path enum, lexical checks, link/reparse rejection, staging-only application writes, aggregate bounds, existing adoption validator. |
| Built-in agent flags accidentally enable arbitrary execution | Critical | Construct explicit `serve --enable-search`; unit-test absence of `--agent`, `--enable-code-execution`, and `--enable-shell`. |
| Prompt injection arrives through brief or web content | High | Treat embedded content as data, restrict tools, validate structured outputs/citations, retain stakeholder-authority rules. |
| SSE stream remains active forever | High | Reset inactivity only on bounded valid events and enforce an unresettable ten-minute hard limit. |
| Floating model/image changes behavior | High | Pin Hugging Face revision, file checksum, and versioned image tag/digest; reject `latest`. |
| Local runtime is healthy but incompatible | Medium | Probe `/v1/models` and run a structured/tool capability check during setup; surface incompatibility explicitly. |
| Central CLI/TUI files overlap user work | Medium | Patch narrow construction points, keep new behavior in provider modules, and preserve untracked evaluation artifacts. |

## Open Questions

No question blocks implementation. The exact CUDA image is intentionally operator-supplied because it depends on the host's driver and compute capability; the setup documentation explains how to choose and pin it.
