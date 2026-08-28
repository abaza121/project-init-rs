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
cargo run -- --data-dir .project-init list
cargo run -- --data-dir .project-init inspect <project-id>
cargo run -- --data-dir .project-init answer <question-id> "Meta Quest 3"
cargo run -- --data-dir .project-init open <project-id>
cargo run -- --data-dir .project-init generate <project-id>
cargo run -- --data-dir .project-init validate <project-id>
cargo run -- --data-dir .project-init export <project-id> <output-directory>
```

`open` launches the Ratatui overview when attached to a terminal and emits JSON when redirected. `new` uses a conservative offline analyzer: it preserves the brief as user-sourced knowledge and creates explicit high-impact unknowns rather than inventing a VR platform or engine.

Generated packages contain `README.md`, `Requirements.md`, `Assumptions.md`, `OpenQuestions.md`, `Traceability.md`, and `ValidationReport.md`. Validation fails while high-impact questions remain open or requirements lack provenance/acceptance criteria.

## Architecture

- [`Docs/specification.md`](Docs/specification.md) — product contract and acceptance criteria.
- [`Docs/architecture.md`](Docs/architecture.md) — boundaries and lifecycle.
- [`Docs/database-schema.md`](Docs/database-schema.md) — authoritative schema and invariants.
- [`Docs/implementation-plan.md`](Docs/implementation-plan.md) — incremental milestones.
- [`Docs/known-limitations.md`](Docs/known-limitations.md) — deliberately unfinished scope.

The domain is independent of SQLite, Ratatui, and provider types. Workflow services validate structured analysis before mutation. Answer reconciliation uses one transaction so stable-ID allocation, question/finding status, requirements, and trace edges cannot partially commit.

## Baseline comparison

The same fishing brief is stored at `evaluation/cases/fishing-vr.txt` and `baseline/project-prompt.txt`. See [`evaluation/README.md`](evaluation/README.md) for non-fabricated measurement fields and runner commands.
