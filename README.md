# Project Init

Project Init is a stable-Rust terminal application that turns an incomplete brief into structured, persistent, traceable project knowledge before generating Markdown documents. SQLite is authoritative; facts, unknowns, questions, answers, requirements, and trace links remain inspectable across restarts.

The original PowerShell/Codex hackathon pipeline is preserved unchanged under [`baseline/`](baseline/) and documented in [`Docs/baseline.md`](Docs/baseline.md).

## Build and verify

```text
cargo build
cargo fmt --check
cargo check --all-features
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Rust 1.88 or newer is supported; the repository lockfile records the tested dependency graph.

## Quick start

```text
cargo run -- --data-dir .project-init new --brief baseline/project-prompt.txt --name "Calm Fishing VR"
cargo run -- --data-dir .project-init --no-skills new --brief baseline/project-prompt.txt --name "Calm Fishing VR"
cargo run -- --data-dir .project-init new --brief baseline/project-prompt.txt --name "Calm Fishing VR" --offline
cargo run -- --data-dir .project-init run --brief baseline/project-prompt.txt --name "Calm Fishing VR" --offline --approval consequential
cargo run -- --data-dir .project-init run --brief baseline/project-prompt.txt --name "Calm Fishing VR" --auto-answer --approval autonomous
cargo run -- --data-dir .project-init run <project-id> --offline
cargo run -- --data-dir .project-init step <project-id> --offline --json
cargo run -- --data-dir .project-init status <project-id> --json
cargo run -- --data-dir .project-init list
cargo run -- --data-dir .project-init inspect <project-id>
cargo run -- --data-dir .project-init answer <question-id> "Meta Quest 3"
cargo run -- --data-dir .project-init open <project-id>
cargo run -- --data-dir .project-init generate <project-id>
cargo run -- --data-dir .project-init validate <project-id>
cargo run -- --data-dir .project-init export <project-id> <output-directory>
```

`new` runs an isolated, schema-constrained Codex CLI analysis by default. When attached to a terminal it shows a spinner and a rolling, scrollable activity timeline; `Esc` cancels immediately. The run times out after five minutes, and failure, cancellation, or invalid structured output creates no project state. The application resolves a native Codex executable from `PATH`, including the binary bundled by an npm installation. Set `CODEX_BIN` to the full native executable path (`codex.exe` on Windows) to override discovery, or pass `--offline` to use the conservative deterministic analyzer explicitly.

Pass the global `--no-skills` option to disable every materialized repository, user, bundled, and installed-plugin Codex skill for that invocation without editing Codex configuration. The option applies to initial analysis, `run`/`step` generation, and workbench `/resume`; it does not disable `AGENTS.md`, sandbox rules, or ordinary tools. If Codex has never materialized its bundled skill inventory, run Codex once before using the option—the pipeline fails closed instead of risking a partially disabled run.

Successful Codex activity is stored atomically with the project and appears in the Ratatui workbench. `open` launches that long-lived workspace when attached to a terminal and emits the project snapshot as JSON when redirected.

The workbench provides Overview, Questions, Findings, Requirements, and Activity sections plus a contextual composer. Use the arrow keys to change section or question, `Tab` to move focus, and `Enter` to answer the selected open question. `Ctrl+Enter` inserts a newline when the terminal reports that key combination. `/answer <question-id> <text>` targets a question explicitly, `/ask [prompt]` opens structured question capture with visible conservative defaults, and `/threshold <1-125>` changes the persisted consequential-question threshold. The standalone `answer` command remains available for scripts and non-interactive use.

Enter `/resume` to continue generation and validation without leaving the workbench. Enter `/auto-answer` to delegate consequential clarification research to a fixed three-worker Codex batch and a separate project-aware judge. A first run opens a mandatory selector for `strict`, `consequential`, or `autonomous`; no policy is selected by default, and later resumes retain the persisted choice. Codex generation and repair use a cancellable progress overlay that streams bounded command, tool, and model output. Automatic answering uses a dedicated board with coordinator, worker, and judge lanes, observable stages, latest sanitized activity, and citation counts rather than invented percentages. Its five-minute inactivity deadline resets whenever Codex emits output, so productive long runs can continue. Questions, `/approve <decision-id> <reason>`, `/reject <decision-id> <reason>`, `/repair`, failures, and completion return to contextual workbench guidance. Accepted answers and authority actions never resume automatically: enter `/resume` or `/auto-answer` again when ready.

`run` records a `strict`, `consequential`, or `autonomous` approval policy and executes deterministic steps until the project is complete or needs an answer, approval, repair, or unavailable capability. Online package generation uses the installed Codex CLI with its configured tools inside an isolated `workspace-write` staging directory; `--offline` produces a deterministic package and labels unavailable external research instead of inventing it.

Pass `--auto-answer` to `run` (or to `new --run`) to delegate clarification questions. A coordinator selects the current blocker and up to two independent consequential questions, three isolated workers research different questions concurrently, and a separate Codex judge checks citations and project-wide consistency before the still-applicable answers enter one SQLite transaction. One failed worker or judge call is retried once. Attached terminals open the same cancellable progress workbench; redirected automation remains headless and prints final workflow status. The option conflicts with `--offline` and fails closed with the run paused if planning, research, judgment, validation, or adoption is unavailable or unsuccessful. Approval and repair boundaries remain governed by the selected workflow policy; `--auto-answer` delegates clarification research only.

Generated packages contain the requirements, assumptions, open questions, SWOT, mission and vision, visual identity, brand prompts, technical architecture, five research lanes, traceability, a timestamped session log, and a validation report. File hashes and validation outcomes are stored in SQLite. If a generated file is edited, the next run automatically preserves it as a manual override and refreshes its dependants; `override` and `remove-override` provide explicit control as well.

Use `add-evidence`, `propose-decision`, `approve`, and `reject` to manage the first-class evidence and decision loop. Validation fails while consequential questions or approvals remain open, required files are missing, or requirements lack provenance and acceptance criteria.

## Architecture

- [`Docs/specification.md`](Docs/specification.md) — product contract and acceptance criteria.
- [`Docs/architecture.md`](Docs/architecture.md) — boundaries and lifecycle.
- [`Docs/database-schema.md`](Docs/database-schema.md) — authoritative schema and invariants.
- [`Docs/implementation-plan.md`](Docs/implementation-plan.md) — incremental milestones.
- [`Docs/known-limitations.md`](Docs/known-limitations.md) — deliberately unfinished scope.

The domain is independent of SQLite, Ratatui, and provider types. Workflow services validate structured analysis before mutation. Initial analysis persistence and answer reconciliation each use one transaction so related authoritative state cannot partially commit.

## Baseline comparison

The same fishing brief is stored at `evaluation/cases/fishing-vr.txt` and `baseline/project-prompt.txt`. See [`evaluation/README.md`](evaluation/README.md) for non-fabricated measurement fields and runner commands.
