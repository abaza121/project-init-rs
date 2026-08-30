# OpenCode CLI Provider Tasks

## Task 1: Define OpenCode provider selection and executable resolution

**Description:** Add the `opencode` provider kind, executable discovery, configuration, and dispatch without changing Codex or local behavior.

**Acceptance criteria:**

- [x] `--provider opencode` parses globally and constructs an OpenCode provider.
- [x] `OPENCODE_BIN` overrides PATH discovery and invalid overrides fail with a clear safe error.
- [x] Local-only options remain rejected unless `--provider local` is selected.

**Verification:**

- [x] Focused CLI/provider tests pass.
- [x] `cargo check --all-features` succeeds.

**Dependencies:** None

**Files likely touched:**

- `src/agents/mod.rs`
- `src/agents/provider.rs`
- `src/main.rs`

**Estimated scope:** Medium: 3 files

## Task 2: Implement and test the read-only structured OpenCode execution path

**Description:** Add a bounded OpenCode CLI adapter that executes analysis and research prompts through JSON events, recovers only the completed assistant text, and preserves cancellation and error behavior.

**Acceptance criteria:**

- [x] Analysis, research, planning, and judgment implement their existing provider traits.
- [x] Malformed, oversized, failed, or incomplete JSON streams fail closed.
- [x] Activity is sanitized and only relevant OpenCode events become timeline entries.

**Verification:**

- [x] Adapter parser and process-argument tests pass.
- [x] Existing agent prompt tests pass.

**Dependencies:** Task 1

**Files likely touched:**

- `src/agents/opencode.rs`
- `src/agents/mod.rs`
- `src/agents/provider.rs`

**Estimated scope:** Medium: 3 files

## Checkpoint: Structured Operations

- [x] Tasks 1-2 complete and focused tests pass.
- [x] `cargo check --all-features` succeeds.

## Task 3: Add documentation execution and workbench integration

**Description:** Use the same OpenCode adapter for provisional generation and repair, including interactive workbench resume and auto-answer flows.

**Acceptance criteria:**

- [x] OpenCode runs documentation work only from the provided staging directory.
- [x] `open` uses the selected OpenCode provider rather than lazily constructing Codex.
- [x] Cancellation and errors leave no adopted partial package.

**Verification:**

- [x] Adapter parser and workspace provider-selection tests pass.
- [x] `cargo test --all-features` passes.

**Dependencies:** Tasks 1-2

**Files likely touched:**

- `src/agents/opencode.rs`
- `src/main.rs`
- `src/tui/mod.rs`

**Estimated scope:** Medium: 3 files

## Task 4: Document the option and update the changelog

**Description:** Add safe usage guidance and a concise user-facing unreleased entry.

**Acceptance criteria:**

- [x] README describes selection, `OPENCODE_BIN`, and user-profile permission responsibility.
- [x] Changelog describes the new provider under `Unreleased` / `Added`.

**Verification:**

- [x] Documentation examples match CLI parsing tests.
- [x] `cargo fmt --check` passes.

**Dependencies:** Tasks 1-3

**Files likely touched:**

- `README.md`
- `CHANGELOG.md`

**Estimated scope:** Small: 2 files

## Checkpoint: Complete

- [x] `cargo fmt --check`, `cargo check --all-features`, `cargo test --all-features`, and `cargo clippy --all-targets --all-features -- -D warnings` pass.
- [x] Staged diff is reviewed for secrets and scope.
