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
cargo run -- --data-dir .project-init --provider opencode new --brief baseline/project-prompt.txt --name "Calm Fishing VR"
cargo run -- --data-dir .project-init --provider local --local-model-dir C:\Models\project-init\gemma-4-12b --local-image ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0 --local-device cpu new --brief baseline/project-prompt.txt --name "Calm Fishing VR"
cargo run -- --data-dir .project-init --provider local --local-format plain --local-model-dir C:\Models\project-init\gemma-4-plain --local-image <versioned-image-or-digest> --local-device cuda new --brief baseline/project-prompt.txt --name "Plain local inference"
cargo run -- --data-dir .project-init --skills new --brief baseline/project-prompt.txt --name "Calm Fishing VR"
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

Pass the global `--provider opencode` to run the same analysis, cited research, automatic-answer, generation, repair, and workbench flows through the authenticated OpenCode CLI profile. Project Init invokes the documented `opencode run --format json` interface, uses the scoped Project Init working or staging directory, and keeps the existing output validation and cancellation boundaries. Set `OPENCODE_BIN` to the full native executable path (`opencode.exe` on Windows) to override `PATH` discovery. Project Init never passes OpenCode's permission auto-approval flag: your OpenCode profile remains responsible for models, agents, plugins, authentication, and tool permissions. See the [OpenCode CLI reference](https://dev.opencode.ai/docs/cli/).

Pass the global `--provider local` plus a model directory, versioned `mistral.rs` image, and `cpu` or `cuda` device to use local HTTP inference as a secondary option. GGUF remains the default and requires the pinned Gemma file; pass `--local-format plain` (or `tensor`) for a native safetensors directory containing `config.json` and at least one `.safetensors` file. Project Init verifies a healthy loopback service or launches a hardened Docker container, then uses the same provider for analysis, cited DuckDuckGo research, automatic answers, generation, repair, and workbench resume. Local document generation receives only three exact allowlisted application tools; it never receives shell, Python, or generic host filesystem access. Each local operation has a 60-second activity-reset inactivity timer and an absolute ten-minute cap. See [`Docs/local-inference-setup.md`](Docs/local-inference-setup.md) for the model setup, CPU/CUDA configuration, tuning, security boundary, and smoke commands. Web research still needs network access, and provider failures never trigger an automatic fallback.

Codex runs without any materialized repository, user, bundled, or installed-plugin skills by default, without editing persistent Codex configuration. This applies to initial analysis, `run`/`step` generation, and workbench `/resume`; it does not disable `AGENTS.md`, sandbox rules, or ordinary tools. Pass the global `--skills` option to restore configured Codex skills for one invocation. The former `--no-skills` spelling remains accepted for compatibility. OpenCode uses its own configured profile and does not interpret either Codex skill option. If Codex has never materialized its bundled skill inventory, run Codex once or pass `--skills`—the skill-free default fails closed instead of risking a partially disabled run.

Successful provider activity is stored atomically with the project and appears in the Ratatui workbench. `open` launches that long-lived workspace when attached to a terminal and emits the project snapshot as JSON when redirected.

The workbench provides Overview, Questions, Findings, Requirements, and Activity sections plus a contextual composer. Use the arrow keys to change section or question, `Tab` to move focus, and `Enter` to answer the selected open question. `Ctrl+Enter` inserts a newline when the terminal reports that key combination. `/answer <question-id> <text>` targets a question explicitly, `/ask [prompt]` opens structured question capture with visible conservative defaults, and `/threshold <1-125>` changes the persisted consequential-question threshold. The standalone `answer` command remains available for scripts and non-interactive use.

Enter `/resume` to continue generation and validation without leaving the workbench. Enter `/auto-answer` to delegate consequential clarification research to a fixed three-worker selected-provider batch and a separate project-aware judge. A first run opens a mandatory selector for `strict`, `consequential`, or `autonomous`; no policy is selected by default, and later resumes retain the persisted choice. Provider generation and repair use a cancellable progress overlay that streams bounded command, tool, and model activity. Automatic answering uses a dedicated board with coordinator, worker, and judge lanes, observable stages, latest sanitized activity, and citation counts rather than invented percentages. Provider-specific inactivity and hard deadlines keep silent or runaway work bounded while allowing productive activity. Questions, `/approve <decision-id> <reason>`, `/reject <decision-id> <reason>`, `/repair`, failures, and completion return to contextual workbench guidance. Accepted answers and authority actions never resume automatically: enter `/resume` or `/auto-answer` again when ready.

`run` records a `strict`, `consequential`, or `autonomous` approval policy and executes deterministic steps until the project is complete or needs an answer, approval, repair, or unavailable capability. Online package generation uses the selected provider while keeping output provisional in an isolated staging directory; `--offline` produces a deterministic package and labels unavailable external research instead of inventing it.

Pass `--auto-answer` to `run` (or to `new --run`) to delegate clarification questions. A coordinator selects the current blocker and up to two independent consequential questions, three isolated workers research different questions concurrently, and a separate selected-provider judge checks citations and project-wide consistency before the still-applicable answers enter one SQLite transaction. One failed worker or judge call is retried once. Attached terminals open the same cancellable progress workbench; redirected automation remains headless and prints final workflow status. The option conflicts with `--offline` and fails closed with the run paused if planning, research, judgment, validation, or adoption is unavailable or unsuccessful. Approval and repair boundaries remain governed by the selected workflow policy; `--auto-answer` delegates clarification research only.

Generated packages contain the requirements, assumptions, open questions, SWOT, mission and vision, visual identity, brand prompts, technical architecture, five research lanes, traceability, a timestamped session log, and a validation report. File hashes and validation outcomes are stored in SQLite. If a generated file is edited, the next run automatically preserves it as a manual override and refreshes its dependants; `override` and `remove-override` provide explicit control as well.

Use `add-evidence`, `propose-decision`, `approve`, and `reject` to manage the first-class evidence and decision loop. Validation fails while consequential questions or approvals remain open, required files are missing, or requirements lack provenance and acceptance criteria.

## Architecture

- [`Docs/specification.md`](Docs/specification.md) — product contract and acceptance criteria.
- [`Docs/architecture.md`](Docs/architecture.md) — boundaries and lifecycle.
- [`Docs/database-schema.md`](Docs/database-schema.md) — authoritative schema and invariants.
- [`Docs/implementation-plan.md`](Docs/implementation-plan.md) — incremental milestones.
- [`Docs/known-limitations.md`](Docs/known-limitations.md) — deliberately unfinished scope.
- [`Docs/local-inference-setup.md`](Docs/local-inference-setup.md) — pinned local model, Docker, CPU/CUDA, and security setup.

The domain is independent of SQLite, Ratatui, and provider types. Workflow services validate structured analysis before mutation. Initial analysis persistence and answer reconciliation each use one transaction so related authoritative state cannot partially commit.

## Baseline comparison

The same fishing brief is stored at `evaluation/cases/fishing-vr.txt` and `baseline/project-prompt.txt`. See [`evaluation/README.md`](evaluation/README.md) for non-fabricated measurement fields and runner commands.
