# Contextual Project Workbench

## Problem Statement

How might we turn `project-init open` into a long-lived, keyboard-first project workspace where users can resolve existing uncertainty and capture newly discovered questions without leaving the TUI or invoking an agent after every action?

## Recommended Direction

Replace the read-only overview with a contextual workbench containing section navigation, a main detail or timeline area, and a persistent actionable composer. The selected entity determines what plain-text input means; explicit slash commands support actions outside that context.

Every mutation maps to an authoritative workflow operation. Answer submission uses existing atomic reconciliation, reloads the project snapshot, displays a diff of the affected entities, and advances through consequential questions before continuing through the remaining open queue. `/ask` opens structured question capture rather than treating arbitrary text as if it had been intelligently interpreted.

The timeline is projected from authoritative analysis activity, questions, answers, findings, and requirements. It resembles a conversation while preserving the product's structured, traceable model.

## Success Definition

A user can:

1. Select and answer one question without leaving the TUI.
2. Work through every consequential open question in one session.
3. Capture a newly discovered question through structured `/ask`.
4. Exit and reopen the project without losing accepted actions, queue settings, or provenance.

## Key Assumptions to Validate

- [ ] Contextual composition is understandable: users can predict what submitting plain text will do before pressing Enter.
- [ ] Deterministic feedback feels responsive enough when the accepted answer and affected entity diff appear immediately after reconciliation.
- [ ] Structured question capture is not too burdensome when prompt, rationale, impact, uncertainty, and cost are shown in a compact overlay.
- [ ] Users value a projected timeline without generic chat messages.
- [ ] Snapshot reloads preserve section, scroll, and useful selection with acceptable latency.
- [ ] Priority-based advancement matches user expectations when consequential questions precede lower-priority open questions.

## MVP Scope

The MVP performs one job well: resolve and extend the project's clarification queue interactively.

Included:

- Stateful workspace logic separated from terminal rendering.
- Sections for Overview, Questions, Findings, Requirements, and Activity.
- Keyboard navigation and visible focus.
- Detail rendering for the selected entity.
- A persistent composer with contextual prompt text.
- Plain-text answering when an open question is selected.
- `/answer <question-id> <text>` for explicit answer targeting.
- `/ask` structured capture with visible conservative defaults: medium impact, high uncertainty, and medium cost.
- A project-specific consequential-question threshold that can be adjusted in the workspace.
- `Enter` to submit and `Ctrl+Enter` to add a newline.
- Atomic workflow operations followed by snapshot reload.
- Automatic progression through consequential questions and then the remaining open queue.
- A timeline projected from authoritative records; equal-time records are all shown and any stable tie-breaker has no semantic meaning.
- A post-answer diff covering affected questions, findings, requirements, traces, and project status.
- Inline success and recoverable error feedback.
- Ratatui test-backend coverage for state transitions and rendering.
- Preservation of the existing non-interactive `answer` command.

The smallest data flow is:

```text
TUI input
  -> parsed WorkspaceAction
  -> ProjectService operation
  -> atomic SQLite transition
  -> refreshed ProjectSnapshot
  -> projected timeline and affected-entity diff
  -> render
```

## Not Doing (and Why)

- Agent re-analysis after answers - explicitly outside the deterministic MVP.
- Arbitrary persisted chat messages - they have no authoritative semantic meaning yet.
- Natural-language intent inference - it would make composer behavior unpredictable without a model.
- Generic event-store migration - existing authoritative records can support the first projected timeline.
- Editing or deleting accepted answers - immutable history requires a separate correction or supersession design.
- Mouse support - keyboard interaction establishes the core workflow first.
- Document generation and validation controls - useful later but unrelated to proving interactive clarification.
- Every entity-management operation - the first release focuses mutations on questions, answers, and the clarification threshold.
- Codex session resume - it introduces provider lifecycle and agent-state concerns prematurely.

## Confirmed Policy Decisions

- `/ask` uses editable conservative defaults rather than inventing hidden priority values.
- Adjusting the consequential threshold persists the setting per project.
- Adding a question does not infer or automatically rewind the project lifecycle; the open question remains visible and actionable regardless of the current stage.
- `Enter` submits and `Ctrl+Enter` inserts a newline.
- Equal-time timeline records are all shown; stable rendering order does not assert causality.
- Answer feedback shows a diff of affected authoritative entities.
- Queue advancement continues automatically after consequential questions are exhausted.

## Open Questions

No unresolved product question blocks the MVP. Terminal support for distinguishing modified Enter keys must be verified on the supported Crossterm backends; if a terminal cannot report `Ctrl+Enter`, the composer must retain an alternate newline binding without changing submission semantics.
