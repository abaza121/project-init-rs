# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add fixed three-worker `--auto-answer` and workbench `/auto-answer` research with dependency-aware batching, project-wide Codex judgment, atomic cited-answer adoption, retries, cancellation safety, and actor-keyed TUI progress on interactive terminals.
- Add explicit in-workbench `/resume`, approval, rejection, and bounded repair flows with mandatory first-run policy selection, cancellable Codex progress, and safe return to contextual guidance.
- Add a global `--no-skills` option that disables discovered Codex skills across analysis, generation, and workbench resume without changing persistent Codex configuration.
- Add a repository-local build workflow that verifies formatting, linting, tests, and locked release compilation before packaging checksummed binaries.
- Add repository and portable runners for exercising five end-to-end Codex evaluation cases with per-case logs.

### Fixed

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
