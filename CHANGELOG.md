# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-08-31T02:18:11+01:00

### Added

- Add `--local-inactivity-timeout-secs` (1–600, default 60) to accommodate slower local models without disabling silence detection.
- Add `--provider opencode` for authenticated OpenCode CLI analysis, research, generation, repair, automatic answers, and workbench resume while retaining Codex as the default provider.
- Add an explicit `--provider local` option for authoritative loopback HTTP analysis, DuckDuckGo-backed cited research, automatic clarification, and allowlisted staged documentation through a managed CPU or NVIDIA CUDA mistral.rs container.
- Add `--local-format plain` (also accepted as `tensor`) for managed local safetensors model directories while retaining pinned GGUF as the default.

### Changed

- Increase the default local startup, initial analysis, and documentation hard timeout from ten to twenty minutes; retain the 60-second inactivity default and activity-based research/auto-answer behavior.
- Improve project analysis and documentation prompts to preserve literal brief provenance, separate research from stakeholder authority, generate unique evaluator-legible requirement, acceptance, decision, assumption, and evidence records, require concrete implementation structure, and prevent generic platform unknowns from becoming VR-specific questions.

### Fixed

- Make Overview, Findings, Requirements, and Activity scrollable with Up/Down, Page Up/Page Down, and Home/End, preserving independent positions through section changes and supporting wrapped text and viewport changes; expose the complete Overview timeline instead of only its latest twelve entries.
- Allow local research and auto-answer planning, workers, and judgment to run without a total inference deadline while valid model activity arrives, preserving cancellation, bounded startup, and separate analysis/documentation limits.
- Keep Codex and OpenCode research and auto-answer planning, workers, and judgment running while provider output remains active, retaining cancellation and five-minute inactivity limits instead of cutting off productive work after five minutes.
- Keep the selected clarification question visible while navigating a queue that exceeds the workbench viewport.
- Prevent auto-answer from persisting `FAIL` responses by requiring evidence-informed provisional decisions and using validation feedback to correct worker and judge retries.
- Use the current mistral.rs `--max-seq-len` runtime option so managed local CUDA and CPU containers reach readiness with supported images.

### Security

- Restrict managed local inference to loopback, pinned model and image identities, hardened model-scoped Docker mounts, bounded streaming/tool loops, and three exact document tools without shell, Python, repository, or home-directory access.

## [0.3.0] - 2026-08-30T00:43:51+01:00

### Added

- Add fixed three-worker `--auto-answer` and workbench `/auto-answer` research with dependency-aware batching, project-wide Codex judgment, atomic cited-answer adoption, retries, cancellation safety, and actor-keyed TUI progress on interactive terminals.
- Add explicit in-workbench `/resume`, approval, rejection, and bounded repair flows with mandatory first-run policy selection, cancellable Codex progress, and safe return to contextual guidance.
- Run Codex analysis, generation, and workbench resume without discovered skills by default, with a global `--skills` opt-in and no persistent configuration changes.
- Add a repository-local build workflow that verifies formatting, linting, tests, and locked release compilation before packaging checksummed binaries.
- Add repository and portable runners for exercising five end-to-end Codex evaluation cases with per-case logs.

### Fixed

- Keep auto-answer research-plan schemas compatible with Codex Structured Outputs by omitting an unsupported array uniqueness constraint.
- Preserve the newest failed validation as a resumable repair boundary instead of allowing an older passing validation to hide it.
- Stream bounded Codex generation output in the workbench and reset its timeout whenever the provider remains active instead of stopping after five minutes of productive work.

## [0.2.0] - 2026-08-29T11:44:27+01:00

### Added

- Add a keyboard-first contextual project workbench for answering clarification questions, capturing structured questions, adjusting the consequential threshold, and inspecting authoritative project history without leaving the TUI.
- Add a resumable `run`/`step`/`status` workflow with run-scoped approval policies, full project-document generation, persisted validation, evidence and decision commands, Codex-assisted research and generation, and automatic preservation of manually edited artifacts.

### Fixed

- Prevent Codex analysis and document generation from failing on Windows when shell shims appear before the native Codex executable on `PATH`.

## [0.1.0] - 2026-08-29

### Added

- Preserve the original hackathon documentation pipeline under `baseline/` with its upstream history and standalone runner.
- Add the product specification, architecture, database schema, baseline assessment, and incremental implementation plan for the Rust application.
- Add the stable-Rust package, typed project and finding model, explicit lifecycle rules, and authoritative SQLite schema with restart-safe project persistence.
- Add conservative structured brief analysis, prioritized clarification questions, first-class answers, transactional reconciliation, user-sourced requirements, and explicit provenance links.
- Add deterministic Markdown package generation, blocking validation for unresolved high-impact questions, a resumable CLI, and a Ratatui project inspector.
- Add isolated Codex CLI analysis during `new`, with schema-validated output, a five-minute timeout, immediate TUI cancellation, rolling activity, atomic persistence, and an explicit `--offline` analyzer.
