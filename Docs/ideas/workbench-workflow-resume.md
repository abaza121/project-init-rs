# Workbench Workflow Resume

## Problem Statement

How might we let project creators continue from clarification through online generation, approvals, repairs, and validation without leaving the workbench?

## Recommended Direction

Extract command-line workflow orchestration into a reusable library-level `WorkflowRunner`. Both the CLI and TUI use this runner so their pause, generation, validation, and completion behavior cannot drift.

The `/resume` command starts or continues execution until the next authority boundary. A first run requires the user to select `strict`, `consequential`, or `autonomous`; no policy is preselected. Existing runs retain their persisted policy.

During Codex generation or repair, the workbench switches to a cancellable progress overlay based on the existing creation screen. Questions, approvals, validation failures, and completion return users to guided contextual cards inside the workbench. Answering, approving, rejecting, or preparing a repair does not silently continue execution: the user invokes `/resume` again.

Repair is an explicit staged Codex operation. It receives the current package and persisted validation findings, writes a complete candidate package into staging, and adopts it only after required artifacts are present. Cancellation or failure leaves the current package untouched, and registered manual overrides remain protected.

## Key Assumptions to Validate

- [ ] A background runner can use a separate SQLite connection without conflicting with workbench reads.
- [ ] Cancellation can leave the workflow paused and resumable without partial authoritative changes.
- [ ] Every workflow pause can be reconstructed from persisted state after restarting the application.
- [ ] Codex repair can use validation findings without overwriting registered manual overrides.

## MVP Scope

- A shared `WorkflowRunner` used by the CLI and TUI.
- `/resume` after both interactive `new` and `open` flows.
- A required approval-policy selector when no workflow run exists.
- Online Codex generation with visible progress, retained activity, and `Esc` cancellation.
- Guided question, approval, rejection, repair, and completion cards.
- `/approve`, `/reject`, and `/repair` workbench commands.
- Explicit `/resume` after every pause.
- Transactional staging and preservation of manual overrides.
- Tests covering resume, cancellation, approval, rejection, repair, completion, and database reopening.

## Not Doing (and Why)

- **Permanent Workflow navigation section** - contextual cards are sufficient for the MVP.
- **Automatic resume after an answer or authority decision** - execution remains explicitly authorized.
- **Concurrent workflows for one project** - SQLite remains authoritative around one active run.
- **Editing generated Markdown inside the TUI** - external edits and override tracking already cover this.
- **Workbench offline-mode selection** - existing CLI offline execution remains available.
- **Unbounded automatic repair loops** - each repair attempt requires explicit user authority.

## Resolved Decisions

- The capability is available from both project creation and project reopening.
- `/resume` is the explicit execution boundary.
- Online Codex generation is required.
- Approvals and repairs remain inside the workbench.
- The CLI and TUI share one orchestration implementation.
- Codex execution uses an overlay only while long-running generation or repair is active.
- A new workflow run requires an explicit approval-policy selection with no default.

## Open Questions

No product decision currently blocks implementation. Repository evidence and tests will determine the smallest compatible runner and repair interfaces.
